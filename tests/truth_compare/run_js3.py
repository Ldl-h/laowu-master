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

# bazi_geju: pass the fourColumns directly from bazi_local
bl = results["bazi_local"]["out"]["data"]["bazi"]
fc = bl["fourColumns"]
geju_payload = {"fourColumns": fc, "birth": {"date":"1998-10-24","time":"08:30:00","gender":1}}
j, err = run_js("bazi_geju", geju_payload)
results["bazi_geju"] = {"payload": geju_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("bazi_geju", ok_of(j), str(j.get("data",{}).get("reason",""))[:100] if not ok_of(j) else "OK")

# tieban: pillars with English keys
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
print("tieban_framework", ok_of(j), str(j.get("data",{}).get("error",{}).get("message",""))[:100] if not ok_of(j) else "OK")

# guice: try with lunarMonth/lunarDay for time法
# 1998-10-24 = lunar 9th month 5th day. yearZhi=寅, hourZhi=辰 (8-9am = 辰时)
guice_payload = {"yearZhi":"寅","lunarMonth":9,"lunarDay":5,"hourZhi":"辰","qiguaFa":"time"}
j, err = run_js("guice", guice_payload)
results["guice"] = {"payload": guice_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("guice", ok_of(j), str(j.get("data",{}).get("reason",""))[:100] if not ok_of(j) else "OK")

# liuyao: try with nongli pre-computed. Actually time_cast needs nongli. Let's pass date/time and see.
# The buildTimeGua uses nongli. Let's try passing nongli from bazi_local.
nongli = bl.get("nongli", {})
liuyao_payload = {"date":"1998-10-24","time":"08:30:00","zone":"+08:00","nongli": nongli}
j, err = run_js("liuyao", liuyao_payload)
results["liuyao"] = {"payload": liuyao_payload, "ok": ok_of(j) and not j.get("data",{}).get("time_cast_failed"), "out": j, "err_tail": err[-300:]}
print("liuyao", ok_of(j), "time_cast_failed" if j.get("data",{}).get("time_cast_failed") else "casted")

# liureng: already works. But the date I used (2026-06-21) may not match 丙辰日.
# Let me check what day ganzhi JS produced.
lr = results["liureng"]["out"]
print("liureng panStyleName:", lr.get("data",{}).get("panStyleName"))
print("liureng layout keys:", list(lr.get("data",{}).get("layout",{}).keys())[:15])

with open(os.path.join(OUT, "js_results3.json"), "w") as f:
    json.dump(results, f, ensure_ascii=False, indent=2)
print("DONE")
