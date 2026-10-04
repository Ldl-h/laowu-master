import json, subprocess, os

JS = "/home/user/.horosa/runtime/current/horosa-core-js"
OUT = "/home/user/.doubao/agent_mode/workspace/.sessions/38445268222044418/agents/o_000cHCROvLg/work"

# Mapping: Rust tool -> list of JS tools to try
MAPPING = {
    "bazi": ["bazi_local", "bazi_geju", "bazi_period"],
    "ziwei": ["ziwei_birth", "ziwei_extras"],
    "qimen": ["qimen"],
    "taiyi": ["taiyi"],
    "jinkou": ["jinkou"],
    "liureng": ["liureng"],
    "sanshiunited": ["sanshiunited"],
    "liuyao": ["liuyao"],
    "heluo": ["heluo"],
    "canping": ["canping"],
    "tieban": ["tieban_framework"],
    "xiaoliuren": ["xiaoliuren"],
    "yizhangjing": ["yizhangjing"],
    "lingqi": ["lingqi"],
    "tarot": ["tarot"],
    "guice": ["guice"],
    "tongshefa": ["tongshefa"],
    "yanqin": ["yanqin_yanfa"],
    "suzhan": ["suzhan", "zhengchuan"],
}

with open("/tmp/eastern_techs.json") as f:
    techs = json.load(f)

def try_run(js_tool, payload):
    try:
        p = subprocess.run(
            ["node", "bin/cli.mjs", "run", js_tool],
            input=json.dumps(payload, ensure_ascii=False),
            capture_output=True, text=True, cwd=JS, timeout=60
        )
        return p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired:
        return -1, "", "TIMEOUT"

results = {}
for t in techs:
    rust_tool = t['tool']
    if rust_tool not in MAPPING:
        continue
    ex = json.loads(t['example'])
    js_tools = MAPPING[rust_tool]
    rust_results_for_tool = {}
    for jt in js_tools:
        # Try 1: direct payload
        rc1, out1, err1 = try_run(jt, ex)
        try:
            j1 = json.loads(out1)
            ok1 = j1.get('data', {}).get('ok', False) if isinstance(j1.get('data'), dict) else j1.get('ok')
        except:
            ok1 = False; j1 = None
        # Try 2: {params: ex}
        rc2, out2, err2 = try_run(jt, {"params": ex})
        try:
            j2 = json.loads(out2)
            ok2 = j2.get('data', {}).get('ok', False) if isinstance(j2.get('data'), dict) else j2.get('ok')
        except:
            ok2 = False; j2 = None
        # Pick best
        if ok1 and (not ok2):
            chosen = "direct"; payload = ex; parsed = j1; rc = rc1; stderr = err1
        elif ok2 and (not ok1):
            chosen = "params"; payload = {"params": ex}; parsed = j2; rc = rc2; stderr = err2
        elif ok1 and ok2:
            # both ok, prefer direct
            chosen = "direct"; payload = ex; parsed = j1; rc = rc1; stderr = err1
        else:
            chosen = "FAILED"; payload = ex; parsed = j1 or j2; rc = rc1; stderr = err1
        rust_results_for_tool[jt] = {
            "input_shape": chosen,
            "payload_used": payload,
            "rc": rc,
            "stderr_tail": (stderr or "")[-500:],
            "ok": ok1 if chosen=="direct" else (ok2 if chosen=="params" else False),
            "output": parsed,
            "raw_stdout_tail": (out1 if chosen=="direct" else out2)[-1500:] if chosen!="FAILED" else out1[-1500:],
        }
        print(f"rust={rust_tool:15s} js={jt:20s} shape={chosen:8s} ok={rust_results_for_tool[jt]['ok']}")
    results[rust_tool] = rust_results_for_tool

with open(os.path.join(OUT, "js_results.json"), "w") as f:
    json.dump(results, f, ensure_ascii=False, indent=2)
print("DONE")
