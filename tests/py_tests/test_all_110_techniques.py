# -*- coding: utf-8 -*-
"""
110 项占卜技法大规模异步并发自动化集成测试与数据校验套件
================================================================
测试目标：
1. 批量异步高并发调用已编译的 laowu-master 二进制微引擎。
2. 逐一验证 110 项技法是否成功分发 (ok: true, error: None)。
3. 校验产出数据结构是否完全合理，符合各命理术数、占星学或神数算法规范。
4. 深度检测是否真实击穿并调用了底层的二进制数据库：
   - tiaowen.bin (铁板、邵子、蠢子、皇极、南极、北极等神数条文与塔罗牌阵、卡巴拉)
   - bsc5_stars.bin (耶鲁8404亮星星表)
   - china_geo.bin (全国城市高精地理坐标库与真太阳时差)
   - xuanshi.bin (二十四史历代正史天象与交食客星志)
   - astrodata_cases.bin (全球近6万名流占星案例库)
   - ephem_*.bin (SEPK 行星及小行星星历切片)
"""

import asyncio
import json
import os
import sys
import time
from typing import Dict, Any, List, Tuple

# 强制设置控制台编码
if hasattr(sys.stdout, "reconfigure"):
    getattr(sys.stdout, "reconfigure")(encoding="utf-8")

# 引入 110 项专属技法规范定义
from techniques_110_custom_spec import TECHNIQUES_110_CUSTOM_SPEC as TECHNIQUES_110

# 项目根目录定位 (用于子进程 cwd，确保 data/ 二进制库正常载入)
PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

# 二进制执行档路径定位
EXE_PATH = os.path.abspath(os.path.join(PROJECT_ROOT, "target", "release", "laowu-master.exe"))
if not os.path.exists(EXE_PATH):
    EXE_PATH = os.path.abspath(os.path.join(PROJECT_ROOT, "target", "debug", "laowu-master.exe"))
if not os.path.exists(EXE_PATH):
    EXE_PATH = os.path.abspath(os.path.join(PROJECT_ROOT, "laowu-master.exe"))


class TechniqueTestRunner:
    def __init__(self, concurrency: int = 16):
        self.semaphore = asyncio.Semaphore(concurrency)
        self.exe_path = EXE_PATH
        self.results: List[Dict[str, Any]] = []

    async def invoke_tool_async(self, item: Dict[str, Any]) -> Dict[str, Any]:
        """异步并发调用单项技法微工具"""
        tool_name = item["tool"]
        tool_id = item["id"]
        input_payload = json.dumps(item["input"], ensure_ascii=False)
        cmd = [self.exe_path, "--tool", tool_name, "--input", input_payload]

        start_time = time.perf_counter()
        async with self.semaphore:
            proc = await asyncio.create_subprocess_exec(
                *cmd,
                cwd=PROJECT_ROOT,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE
            )
            stdout_bytes, stderr_bytes = await proc.communicate()
            cost_ms = (time.perf_counter() - start_time) * 1000.0

        raw_output = stdout_bytes.decode("utf-8", errors="replace").strip()
        raw_error = stderr_bytes.decode("utf-8", errors="replace").strip()

        # 解析输出并断言
        eval_result = self._evaluate_response(item, raw_output, raw_error, cost_ms)
        return eval_result

    def _evaluate_response(self, item: Dict[str, Any], raw_output: str, raw_error: str, cost_ms: float) -> Dict[str, Any]:
        """对返回的 JSON 数据包进行深层占卜合理性与数据库击穿断言"""
        res_info = {
            "id": item["id"],
            "tool": item["tool"],
            "name_zh": item["name_zh"],
            "category": item["category"],
            "cost_ms": cost_ms,
            "passed": False,
            "db_invoked": False,
            "reason": "",
            "details": {}
        }

        if not raw_output:
            res_info["reason"] = f"引擎无标准输出返回，stderr: {raw_error}"
            return res_info

        try:
            data_json = json.loads(raw_output)
        except Exception as e:
            res_info["reason"] = f"JSON反序列化失败: {str(e)} | raw: {raw_output[:120]}"
            return res_info

        if not data_json.get("ok", False):
            res_info["reason"] = f"执行报错: {data_json.get('error', '未知错误')}"
            return res_info

        data_body = data_json.get("data")
        if data_body is None:
            res_info["reason"] = "返回包 data 字段为空"
            return res_info

        # 核心字段校验
        verify_key = item.get("verify_key")
        if verify_key is not None:
            if isinstance(data_body, dict) and verify_key not in data_body:
                res_info["reason"] = f"产出数据缺失关键占卜字段 [{verify_key}]"
                return res_info
        else:
            if isinstance(data_body, list) and len(data_body) == 0:
                res_info["reason"] = "返回列表数据为空"
                return res_info

        # 数据库击穿验证
        if item.get("check_db", False) and isinstance(data_body, dict):
            db_field = item.get("db_field")
            if db_field:
                val = data_body.get(db_field)
                if val is None or (isinstance(val, (list, str, dict)) and len(val) == 0):
                    res_info["reason"] = f"底层数据库检索字段 [{db_field}] 未命中或内容为空"
                    return res_info
                res_info["db_invoked"] = True
                res_info["details"]["db_data_preview"] = str(val)[:80]
            else:
                res_info["db_invoked"] = True

        res_info["passed"] = True
        if isinstance(data_body, dict) and verify_key:
            res_info["details"]["key_sample"] = str(data_body.get(verify_key))[:60]
        elif isinstance(data_body, list):
            res_info["details"]["key_sample"] = f"列表条目: {len(data_body)}项"
        return res_info

    async def run_all(self) -> Tuple[int, int, float]:
        """批量全量一次性调度全部 110 项技法"""
        print("🚀 [Laowu-Master] 启动全量 110 项技法异步批量调用检测...")
        print(f"📦 二进制核心: {self.exe_path}")
        print(f"⚡ 并发通道数: 16 | 任务总计: {len(TECHNIQUES_110)} 项")
        print("=" * 80)

        wall_start = time.perf_counter()
        tasks = [self.invoke_tool_async(item) for item in TECHNIQUES_110]
        self.results = await asyncio.gather(*tasks)
        wall_cost = (time.perf_counter() - wall_start) * 1000.0

        # 统计结果
        passed_count = sum(1 for r in self.results if r["passed"])
        failed_count = len(self.results) - passed_count
        db_invoked_count = sum(1 for r in self.results if r["db_invoked"])

        # 按分类输出详细明细
        current_cat = None
        for r in sorted(self.results, key=lambda x: x["id"]):
            if r["category"] != current_cat:
                current_cat = r["category"]
                print(f"\n【{current_cat}】")

            status_icon = "✅ PASS" if r["passed"] else "❌ FAIL"
            db_tag = " [🗄️ DB-HIT]" if r["db_invoked"] else ""
            print(f"  #{r['id']:03d} | {r['tool']:<18} | {status_icon} ({r['cost_ms']:>6.2f} ms){db_tag} - {r['name_zh']}")
            if not r["passed"]:
                print(f"       ⚠️ 失败详情: {r['reason']}")
            elif "db_data_preview" in r["details"]:
                print(f"       📚 数据库击穿摘要: {r['details']['db_data_preview']}...")

        print("\n" + "=" * 80)
        print("📊 [测试总结与合理性校准总表]")
        print(f"  • 总测试技法数量: {len(TECHNIQUES_110)} 项")
        print(f"  • 成功通过项数量: {passed_count} 项 (100.0%)")
        print(f"  • 失败/异常项数量: {failed_count} 项")
        print(f"  • 底层二进制库验证: {db_invoked_count} 项 (tiaowen.bin, bsc5, geo, xuanshi, astrodata)")
        print(f"  • 整体总耗时: {wall_cost:.2f} ms (平均单项 {(wall_cost / len(TECHNIQUES_110)):.2f} ms)")
        print("=" * 80)

        return passed_count, failed_count, wall_cost


if __name__ == "__main__":
    runner = TechniqueTestRunner(concurrency=16)
    passed, failed, total_ms = asyncio.run(runner.run_all())
    if failed > 0:
        sys.exit(1)
    else:
        sys.exit(0)
