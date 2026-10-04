import json, subprocess, os
RUST = "/home/user/Doubao/chats/38445268222044418/repos/xuanxue-core"
OUT = "/home/user/.doubao/agent_mode/workspace/.sessions/38445268222044418/agents/o_000cHCROvLg/work"
with open("/tmp/eastern_techs.json") as f:
    techs = json.load(f)
results = []
for t in techs:
    try:
        p = subprocess.run(["./target/release/xuanxue-core","--tool",t['tool']],
            input=t['example'], capture_output=True, text=True, cwd=RUST, timeout=60)
        j = json.loads(p.stdout)
        rec = {'id':t['id'],'tool':t['tool'],'ok':j.get('ok'),'rc':p.returncode}
        if j.get('ok'):
            d = j['data']
            rec['top_fields'] = len(d) if isinstance(d,dict) else (len(d) if isinstance(d,list) else 1)
            rec['preview'] = json.dumps(d, ensure_ascii=False)[:300]
        else:
            rec['error'] = j.get('error','')[:200]
        results.append(rec)
        flag = "✓" if rec['ok'] else "✗"
        print(f"{flag} id={t['id']:3d} {t['tool']:18s} ok={rec['ok']} fields={rec.get('top_fields','-')}")
    except Exception as e:
        print(f"✗ id={t['id']:3d} {t['tool']:18s} EXC {e}")
        results.append({'id':t['id'],'tool':t['tool'],'ok':False,'error':str(e)})
with open(os.path.join(OUT,"rerun_results.json"),"w") as f:
    json.dump(results,f,ensure_ascii=False,indent=2)
