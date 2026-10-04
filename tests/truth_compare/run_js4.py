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
            return json.loads(p.stdout), p.stderr
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
        if "error" in d and isinstance(d.get("error"), dict): return False
    return True

with open(os.path.join(OUT, "js_results2.json")) as f:
    results = json.load(f)

# Get bazi_local nongli
bl = results["bazi_local"]["out"]["data"]["bazi"]
nongli = bl.get("nongli", {})
fc = bl.get("fourColumns", {})

# liureng: build chart with nongli from bazi_local. Use a 丙辰日 scenario.
# Rust example: yue_jiang=6 (巳将), zhan_shi=0 (子时), day_gan=丙, day_zhi=4 (辰).
# We need a date that is 丙辰日. Let's find one near 2026.
# Actually, let's just construct a chart object directly with the nongli fields liureng needs.
# dayGanZi = 丙辰, time = 子 (子时)
liureng_chart = {
    "nongli": {
        "dayGanZi": "丙辰",
        "time": "子",
        "yearGanZi": "丙午",
        "monthGanZi": "癸巳",
    },
    "isDiurnal": True,
}
liureng_payload = {
    "chart": liureng_chart,
    "date": "2026-06-21", "time": "00:00:00",
    "yue": "巳",
    "zone": "+08:00",
}
j, err = run_js("liureng", liureng_payload)
results["liureng"] = {"payload": liureng_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("liureng", ok_of(j))
data = j.get("data",{})
print("  layout:", "present" if data.get("layout") else "null")
print("  panStyleName:", data.get("panStyleName"))
sc = data.get("sanChuan")
if sc:
    print("  sanChuan keys:", list(sc.keys())[:10] if isinstance(sc,dict) else type(sc))

# liuyao: pass nongli from bazi_local
liuyao_payload = {
    "date":"1998-10-24","time":"08:30:00","zone":"+08:00",
    "nongli": nongli,
}
j, err = run_js("liuyao", liuyao_payload)
results["liuyao"] = {"payload": liuyao_payload, "ok": ok_of(j) and not j.get("data",{}).get("time_cast_failed"), "out": j, "err_tail": err[-300:]}
print("liuyao", ok_of(j), "time_cast_failed" if j.get("data",{}).get("time_cast_failed") else "casted")
if not j.get("data",{}).get("time_cast_failed"):
    print("  currentGua:", j.get("data",{}).get("currentGua"))

# bazi_geju with proper fourColumns
geju_payload = {"fourColumns": fc, "birth": {"date":"1998-10-24","time":"08:30:00","gender":1}}
j, err = run_js("bazi_geju", geju_payload)
results["bazi_geju"] = {"payload": geju_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("bazi_geju", ok_of(j))

# tieban
tieban_payload = {
    "pillars": [
        {"key":"year","ganzhi":"戊寅"},
        {"key":"month","ganzhi":"壬戌"},
        {"key":"day","ganzhi":"甲辰"},
        {"key":"hour","ganzhi":"戊辰"},
    ],
    "birthYear": 1998, "gender": 1
}
j, err = run_js("tieban_framework", tieban_payload)
results["tieban_framework"] = {"payload": tieban_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("tieban_framework", ok_of(j))

# guice with lunar info
guice_payload = {"yearZhi":"寅","lunarMonth":9,"lunarDay":5,"hourZhi":"辰","qiguaFa":"time"}
j, err = run_js("guice", guice_payload)
results["guice"] = {"payload": guice_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("guice", ok_of(j))

with open(os.path.join(OUT, "js_final.json"), "w") as f:
    json.dump(results, f, ensure_ascii=False, indent=2)
print("SAVED")
