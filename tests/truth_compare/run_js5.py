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

with open(os.path.join(OUT, "js_final.json")) as f:
    results = json.load(f)

# liureng: time must be full ganzhi. 丙日子时 = 戊子时. yue=巳.
liureng_chart = {
    "nongli": {
        "dayGanZi": "丙辰",
        "time": "戊子",  # 丙日起戊子
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
data = j.get("data",{})
print("liureng", ok_of(j), "layout:", "present" if data.get("layout") else "null")
print("  panStyleName:", data.get("panStyleName"))
if data.get("layout"):
    lay = data["layout"]
    print("  layout keys:", list(lay.keys())[:20])
    # Print heaven plate / earth plate
    for k in ["up","down","tian","di","yue","timezi","gui"]:
        if k in lay:
            print(f"  {k}:", lay[k])
sc = data.get("sanChuan")
if sc:
    print("  sanChuan:", json.dumps(sc, ensure_ascii=False)[:500])
ke = data.get("ke")
if ke:
    print("  ke:", json.dumps(ke, ensure_ascii=False)[:500])

# liuyao: time must be full ganzhi too. Use bazi_local nongli which has timeGanZi.
bl = results["bazi_local"]["out"]["data"]["bazi"]
nongli = bl.get("nongli", {})
# nongli should have timeGanZi
print("  nongli.timeGanZi:", nongli.get("timeGanZi"), "dayGanZi:", nongli.get("dayGanZi"))
liuyao_payload = {
    "date":"1998-10-24","time":"08:30:00","zone":"+08:00",
    "nongli": nongli,
}
j, err = run_js("liuyao", liuyao_payload)
results["liuyao"] = {"payload": liuyao_payload, "ok": ok_of(j), "out": j, "err_tail": err[-300:]}
print("liuyao", ok_of(j), "time_cast_failed" if j.get("data",{}).get("time_cast_failed") else "casted")
if not j.get("data",{}).get("time_cast_failed"):
    cg = j.get("data",{}).get("currentGua")
    print("  currentGua:", json.dumps(cg, ensure_ascii=False)[:400])

with open(os.path.join(OUT, "js_final.json"), "w") as f:
    json.dump(results, f, ensure_ascii=False, indent=2)
print("SAVED")
