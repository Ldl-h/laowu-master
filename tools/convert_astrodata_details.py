#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
convert_astrodata_details.py — 一次性离线转换：ACAS/SQLite → astrodata_details.bin

数据来源（原项目 Horosa-Web astrostudyui，与 Rust 项目 data/astrodata_cases.bin 同源）：
  - Astro-Databank 人物表 59,199 人（name / born_display / sun / moon / asc / ...）
  - 维基百科传记摘要 40,401 条（wiki_summary，CC BY-SA 4.0）
  - 分类表 383,327 条（person_title → category）

许可说明：人物出生数据来自 Astro-Databank（Astrodienst AG），维基百科摘要采用
CC BY-SA 4.0。本转换与原 astrostudyui 前端使用同一数据源，输出 details.bin 仅供
Rust 运行时只读查询，不重新分发原始数据库。

用法：
  python3 tools/convert_astrodata_details.py
      # 默认读取 data/astrodata_cases.bin（ACAS 20字节头 + XZ）
  python3 tools/convert_astrodata_details.py --sqlite /path/to/astrodata-aa.sqlite
      # 直接读取解压后的 sqlite 文件
输出：
  data/astrodata_details.bin
    格式（与 ADTX 同构，便于复用 flate2 ZlibDecoder）：
      [4B magic "ADTS"][4B uncomp_len u32LE][4B comp_len u32LE][comp_len zlib 流]
    zlib 解压后为一个 JSON object：
      { "<person.name>": {
          "wiki_summary": "...",
          "wiki_url": "...",
          "born_display": "...",
          "born_zh": "...",
          "summary_zh": "...",
          "categories": ["1879 births", "Birthplace Ulm, GER", ...]
        }, ... }
    Rust 侧按 name 精确匹配补充索引命中的完整字段。
"""

import argparse
import io
import json
import os
import sqlite3
import sys
import zlib


def open_sqlite_from_acas(path: str):
    """读取 ACAS 容器（20字节头 + XZ 流），解压得到 SQLite。
    返回 (connection, tmpfile_path_or_None)。"""
    with open(path, "rb") as f:
        head = f.read(20)
        if head[:4] != b"ACAS":
            raise SystemExit(f"错误: {path} 不是 ACAS 容器 (magic={head[:4]!r})")
        payload = f.read()
    raw = zlib_decompress_xz(payload)
    # sqlite3 不能直接打开内存 bytes，写入临时文件
    import tempfile
    tf = tempfile.NamedTemporaryFile(suffix=".sqlite", delete=False)
    tf.write(raw)
    tf.close()
    con = sqlite3.connect(tf.name)
    con.row_factory = sqlite3.Row
    return con, tf.name


def zlib_decompress_xz(data: bytes) -> bytes:
    import lzma
    return lzma.decompress(data)


def build_details(con: sqlite3.Connection) -> dict:
    cur = con.cursor()
    # 1) 人物核心字段（仅取有 wiki_summary 的，其余人索引已覆盖精简字段）
    cur.execute(
        """
        SELECT name, title, born_display, born_zh, wiki_summary, wiki_url, summary_zh,
               rodden, lat, lon, gpsLat, gpsLon, zone, tz_abbr, adb_url,
               gender, has_time, birth_year, collector, data_source, pos, pos_zh,
               birth_chart, time_accuracy
        FROM person
        WHERE wiki_summary IS NOT NULL AND wiki_summary != ''
        """
    )
    persons = cur.fetchall()

    # 2) 按 title 聚合分类（category 表用 person_title = "Last, First"）
    title_categories: dict = {}
    cur.execute("SELECT person_title, category FROM category")
    for pt, cat in cur.fetchall():
        title_categories.setdefault(pt, []).append(cat)

    details: dict = {}
    for p in persons:
        title = p["title"]
        cats = title_categories.get(title, [])
        # 分类去重保序，限制条数控制体积
        seen = set()
        cats_uniq = []
        for c in cats:
            if c not in seen:
                seen.add(c)
                cats_uniq.append(c)
        cats_uniq = cats_uniq[:12]  # 每人最多 12 条分类

        def split_bc(bc):
            if not bc:
                return ("", "")
            parts = str(bc).strip().split(" ", 1)
            return (parts[0], parts[1] if len(parts) > 1 else "")
        birth_date, birth_time = split_bc(p["birth_chart"])

        details[p["name"]] = {
            "wiki_summary": p["wiki_summary"],
            "wiki_url": p["wiki_url"] or "",
            "born_display": p["born_display"] or "",
            "born_zh": p["born_zh"] or "",
            "summary_zh": p["summary_zh"] or "",
            "categories": cats_uniq,
            # R14 扩展字段
            "title": title or "",
            "rodden": p["rodden"] or "",
            "lat": p["lat"] or "",
            "lon": p["lon"] or "",
            "gps_lat": p["gpsLat"],
            "gps_lon": p["gpsLon"],
            "zone": p["zone"] or "",
            "tz_abbr": p["tz_abbr"] or "",
            "adb_url": p["adb_url"] or "",
            "gender": p["gender"],
            "has_time": p["has_time"],
            "birth_year": p["birth_year"],
            "birth_date": birth_date,
            "birth_time": birth_time,
            "collector": p["collector"] or "",
            "data_source": p["data_source"] or "",
            "pos": p["pos"] or "",
            "pos_zh": p["pos_zh"] or "",
            "time_accuracy": p["time_accuracy"] or "",
        }
    return details


def write_details_bin(details: dict, out_path: str) -> None:
    payload = json.dumps(details, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
    comp = zlib.compress(payload, 6)
    with open(out_path, "wb") as f:
        f.write(b"ADTS")
        f.write(len(payload).to_bytes(4, "little"))
        f.write(len(comp).to_bytes(4, "little"))
        f.write(comp)
    print(f"[OK] 写出 {out_path}")
    print(f"     记录数: {len(details)}")
    print(f"     未压缩: {len(payload)/1e6:.2f} MB")
    print(f"     zlib 压缩后: {len(comp)/1e6:.2f} MB")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--acas", default="data/astrodata_cases.bin", help="ACAS 容器路径")
    ap.add_argument("--sqlite", default=None, help="直接使用已解压的 sqlite 文件")
    ap.add_argument("--out", default="data/astrodata_details.bin")
    args = ap.parse_args()

    if args.sqlite:
        con = sqlite3.connect(args.sqlite)
        con.row_factory = sqlite3.Row
        tmp = None
    else:
        if not os.path.exists(args.acas):
            raise SystemExit(f"错误: 找不到 {args.acas}")
        print(f"[..] 解压 ACAS: {args.acas}")
        con, tmp = open_sqlite_from_acas(args.acas)

    try:
        print("[..] 读取 person / category 表 ...")
        details = build_details(con)
        write_details_bin(details, args.out)
    finally:
        con.close()
        if tmp and os.path.exists(tmp):
            os.unlink(tmp)


if __name__ == "__main__":
    main()
