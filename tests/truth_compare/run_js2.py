import json, subprocess, os

JS = "/home/user/.horosa/runtime/current/horosa-core-js"
OUT = "/home/user/.doubao/agent_mode/workspace/.sessions/38445268222044418/agents/o_000cHCROvLg/work"

def run_js(tool, payload):
    try:
        p = subprocess.run(
            ["node", "bin/cli.mjs", "run", tool],
            input=json.dumps(payload, ensure_ascii=False),
            capture_output=True, text=True, cwd=JS, timeout=90
        )
        try:
            j = json.loads(p.stdout)
            return j, p.stderr
        except:
            return {"_parse_error": True, "_raw": p.stdout[-2000:], "ok": False}, p.stderr
    except subprocess.TimeoutExpired:
        return {"_timeout": True}, ""

def ok_of(j):
    if not isinstance(j, dict): return False
    if j.get("ok") is False: return False
    d = j.get("data")
    if isinstance(d, dict):
        if d.get("ok") is False: return False
        # liureng returns data.layout/ke/sanChuan directly
        if "error" in d and isinstance(d.get("error"), dict): return False
    return True

# Build payloads per tool. Use the Rust example as the source of truth for the *moment*.
results = {}

# 1. bazi_local already works with {params: ex}
ex_bazi = json.loads('{"date": "1998-10-24", "time": "08:30:00", "gender": 1, "after23_new_day": true, "late_zi_use_next_day": true}')
j, err = run_js("bazi_local", {"params": ex_bazi})
results["bazi_local"] = {"payload": {"params": ex_bazi}, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("bazi_local", ok_of(j))

# bazi_period
j, err = run_js("bazi_period", {"params": ex_bazi})
results["bazi_period"] = {"payload": {"params": ex_bazi}, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("bazi_period", ok_of(j))

# bazi_geju: needs four pillars with stem+stemInBranch. Build from bazi_local output.
# First run bazi_local to get pillars
bl = results["bazi_local"]["out"]
fc = bl.get("data",{}).get("bazi",{}).get("fourColumns",{})
pillars_geju = {}
for k in ["year","month","day","time"]:
    col = fc.get(k, {})
    pillars_geju[k] = {
        "stem": col.get("gan") or col.get("stem"),
        "stemInBranch": col.get("zhi") or col.get("branch"),
    }
# Try geju with params
j, err = run_js("bazi_geju", {"params": ex_bazi, "fourPillars": pillars_geju})
results["bazi_geju"] = {"payload": {"params": ex_bazi, "fourPillars": pillars_geju}, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("bazi_geju", ok_of(j), str(j.get("data",{}))[:200])

# 3. ziwei_extras already works direct. But ziwei_birth needs java_result.
ex_ziwei = json.loads('{"is_lunar": true, "lunar_month": 9, "lunar_day": 5, "hour": 8, "gender": 1}')
j, err = run_js("ziwei_extras", ex_ziwei)
results["ziwei_extras"] = {"payload": ex_ziwei, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("ziwei_extras", ok_of(j))

# 4. qimen: Rust ex = {year:2026, month:6, day:21, hour:11, minute:30, second:0, pai_pan_type:1}
qimen_payload = {
    "date": "2026-06-21", "time": "11:30:00",
    "options": {"paiPanType": 1},
    "zone": "+08:00",
}
j, err = run_js("qimen", qimen_payload)
results["qimen"] = {"payload": qimen_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("qimen", ok_of(j), str(j.get("data",{}).get("error",""))[:200] if not ok_of(j) else list(j.get("data",{}).keys())[:10])

# 5. taiyi: Rust ex = {year:2026, month:2, day:4, hour:15, style:3}
taiyi_payload = {"date": "2026-02-04", "time": "15:00:00", "zone": "+08:00"}
j, err = run_js("taiyi", taiyi_payload)
results["taiyi"] = {"payload": taiyi_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("taiyi", ok_of(j), str(j.get("data",{}).get("error",""))[:200] if not ok_of(j) else list(j.get("data",{}).keys())[:10])

# 7. liureng: Rust ex = {yue_jiang:6, zhan_shi:0, day_gan:"丙", day_zhi:4}
# Need a date. Use 2026-06-21 00:00 (子时). day_gan 丙, day_zhi 4 = 辰? zhi index 0=子...4=辰.
# Let's find a date that is 丙辰日. Actually Rust input is abstract. Use a reasonable date.
# yue_jiang 6 = 未将? Let's just pass date/time and yue as branch char.
# yue_jiang index: 0=登明(亥),1=河魁(戌),2=从魁(酉),3=传送(申),4=小吉(未),5=胜光(午),6=太乙(巳),7=天罡(辰),8=太冲(卯),9=功曹(寅),10=大吉(丑),11=神后(子)
# Actually standard: 月将 = 太阳过宫. Let's use 2026-06-21 子时.
liureng_payload = {
    "date": "2026-06-21", "time": "00:00:00",
    "yue": "巳",  # yue_jiang=6 → 太乙巳
    "zone": "+08:00",
}
j, err = run_js("liureng", liureng_payload)
results["liureng"] = {"payload": liureng_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("liureng", ok_of(j), str(j.get("data",{}).get("error",""))[:200] if not ok_of(j) else list(j.get("data",{}).keys())[:10])

# 59. liuyao: Rust ex = {numbers:[1,5,3]}. JS liuyao uses time-cast by default.
# Pass a date/time. Use 2026-06-21 11:30.
liuyao_payload = {"date": "2026-06-21", "time": "11:30:00", "zone": "+08:00"}
j, err = run_js("liuyao", liuyao_payload)
results["liuyao"] = {"payload": liuyao_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("liuyao", ok_of(j), "time_cast_failed" if j.get("data",{}).get("time_cast_failed") else "ok")

# 62. heluo: Rust ex = {date, time, gender}
heluo_payload = {"date": "1998-10-24", "time": "08:30:00", "gender": 1}
j, err = run_js("heluo", heluo_payload)
results["heluo"] = {"payload": heluo_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("heluo", ok_of(j))

# 63. canping
canping_payload = {"date": "1998-10-24", "time": "08:30:00"}
j, err = run_js("canping", canping_payload)
results["canping"] = {"payload": canping_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("canping", ok_of(j))

# 65. tieban_framework: needs pillars. Build from bazi_local.
four_pillars = []
for k, label in [("year","年"),("month","月"),("day","日"),("time","时")]:
    col = fc.get(k, {})
    gz = (col.get("gan") or "") + (col.get("zhi") or "")
    four_pillars.append({"key": label, "ganzhi": gz})
tieban_payload = {"pillars": four_pillars, "birthYear": 1998, "gender": 1}
j, err = run_js("tieban_framework", tieban_payload)
results["tieban_framework"] = {"payload": tieban_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("tieban_framework", ok_of(j), str(j.get("data",{}).get("error",""))[:200] if not ok_of(j) else list(j.get("data",{}).keys())[:10])

# 89. xiaoliuren: Rust ex = {month:8, day:15, hour:6}
xiaoliu_payload = {"nums": [8, 15, 6]}
j, err = run_js("xiaoliuren", xiaoliu_payload)
results["xiaoliuren"] = {"payload": xiaoliu_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("xiaoliuren", ok_of(j))

# 90. yizhangjing: Rust ex = {month:5, day:10, hour:8} (lunar). Need a solar date.
# Use 1998-10-24 08:30 as a stand-in (the bazi moment). Actually Rust yizhangjing uses lunar.
# Let's just pass a solar date/time.
yizhang_payload = {"date": "1998-10-24", "time": "08:30:00", "gender": 1}
j, err = run_js("yizhangjing", yizhang_payload)
results["yizhangjing"] = {"payload": yizhang_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("yizhangjing", ok_of(j))

# 91. lingqi: already works direct with {numbers:[2,1,1]}
lingqi_payload = {"numbers": [2,1,1]}
j, err = run_js("lingqi", lingqi_payload)
results["lingqi"] = {"payload": lingqi_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("lingqi", ok_of(j))

# 92. tarot: already works
tarot_payload = {"spread": "three", "seed": 20261002}
j, err = run_js("tarot", tarot_payload)
results["tarot"] = {"payload": tarot_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("tarot", ok_of(j))

# 94. guice: Rust ex = {year_gan:"戊", hour_gan:"癸"}. JS needs yearZhi/hourZhi etc.
# 戊 year = 寅 (1998戊寅), 癸 hour = 卯? 癸时支 depends. Use 1998戊寅年 卯时.
guice_payload = {"yearZhi": "寅", "hourZhi": "卯", "qiguaFa": "time"}
j, err = run_js("guice", guice_payload)
results["guice"] = {"payload": guice_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("guice", ok_of(j))

# 95. tongshefa: Rust ex = {numbers:[2,1,2,1]}. JS needs taiyin/taiyang/shaoyang/shaoyin (卦名).
# numbers 1=阳 2=阴? Build 4 trigrams.
# Actually tongshefa payload: {taiyin, taiyang, shaoyang, shaoyin} each a hexagram/trigram name.
# Let's try with the numbers as-is first, then with trigram names.
j, err = run_js("tongshefa", {"taiyin": "巽", "taiyang": "坎", "shaoyang": "离", "shaoyin": "坤"})
results["tongshefa"] = {"payload": {"taiyin": "巽", "taiyang": "坎", "shaoyang": "离", "shaoyin": "坤"}, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("tongshefa", ok_of(j))

# 96. yanqin_yanfa: needs year/month/day. Rust ex = {year_gz:"戊寅", hour_gz:"戊辰"}
yanqin_payload = {"year": 1998, "month": 10, "day": 24, "hour": 8}
j, err = run_js("yanqin_yanfa", yanqin_payload)
results["yanqin_yanfa"] = {"payload": yanqin_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("yanqin_yanfa", ok_of(j), str(j.get("data",{}).get("error",""))[:200] if not ok_of(j) else list(j.get("data",{}).keys())[:10])

# 98. suzhan: already works direct
suzhan_payload = {"month": 8, "day": 15}
j, err = run_js("suzhan", suzhan_payload)
results["suzhan"] = {"payload": suzhan_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("suzhan", ok_of(j))

with open(os.path.join(OUT, "js_results2.json"), "w") as f:
    json.dump(results, f, ensure_ascii=False, indent=2)
print("DONE")
