import json, subprocess, os, sys, time

RUST = "/home/user/Doubao/chats/38445268222044418/repos/xuanxue-core"
OUT = "/home/user/.doubao/agent_mode/workspace/.sessions/38445268222044418/agents/o_000cHCROvLg/work"

with open("/tmp/eastern_techs.json") as f:
    techs = json.load(f)

PLACEHOLDERS = ["TODO", "未实现", "placeholder", "Placeholder", "todo", "待补充", "待实现", "FIXME", "todo!"]

def count_fields(obj, depth=0):
    """Count leaf-ish fields for depth metric"""
    if isinstance(obj, dict):
        return sum(count_fields(v, depth+1) for v in obj.values())
    elif isinstance(obj, list):
        if len(obj) == 0:
            return 0
        return sum(count_fields(v, depth+1) for v in obj)
    else:
        return 1

def find_empty(obj, path="$"):
    """Find empty dict/list"""
    empties = []
    if isinstance(obj, dict):
        if len(obj) == 0:
            empties.append(path)
        for k,v in obj.items():
            empties.extend(find_empty(v, f"{path}.{k}"))
    elif isinstance(obj, list):
        if len(obj) == 0:
            empties.append(path+"[]")
        for i,v in enumerate(obj):
            empties.extend(find_empty(v, f"{path}[{i}]"))
    return empties

results = []
for t in techs:
    tid = t['id']; tool = t['tool']; ex = t['example']
    rec = {'id': tid, 'tool': tool, 'name': t['name'], 'category': t['category'], 'example': ex}
    try:
        p = subprocess.run(
            ["./target/release/xuanxue-core", "--tool", tool],
            input=ex, capture_output=True, text=True, cwd=RUST, timeout=60
        )
        rec['returncode'] = p.returncode
        rec['stderr'] = p.stderr[-2000:] if p.stderr else ""
        out = p.stdout.strip()
        rec['stdout_raw'] = out
        try:
            j = json.loads(out)
            rec['parse_ok'] = True
            rec['ok'] = j.get('ok', None)
            data = j.get('data', None)
            rec['data_type'] = type(data).__name__
            if isinstance(data, dict):
                rec['top_keys'] = list(data.keys())
                rec['top_field_count'] = len(data.keys())
                rec['leaf_count'] = count_fields(data)
                rec['empty_paths'] = find_empty(data)
            elif isinstance(data, list):
                rec['top_field_count'] = len(data)
                rec['leaf_count'] = count_fields(data)
                rec['empty_paths'] = find_empty(data)
            else:
                rec['top_field_count'] = 1
                rec['leaf_count'] = 1
                rec['empty_paths'] = []
            # placeholder check
            flat = json.dumps(data, ensure_ascii=False) if data is not None else ""
            rec['placeholders'] = [ph for ph in PLACEHOLDERS if ph in flat]
            rec['data_preview'] = flat[:600]
        except Exception as e:
            rec['parse_ok'] = False
            rec['parse_err'] = str(e)
    except subprocess.TimeoutExpired:
        rec['timeout'] = True
    except Exception as e:
        rec['exception'] = str(e)
    results.append(rec)
    print(f"id={tid:3d} tool={tool:20s} rc={rec.get('returncode')} ok={rec.get('ok')} top_fields={rec.get('top_field_count')} leaf={rec.get('leaf_count')} empties={len(rec.get('empty_paths',[]))} ph={rec.get('placeholders')}")

with open(os.path.join(OUT, "rust_results.json"), "w") as f:
    json.dump(results, f, ensure_ascii=False, indent=2)
print("DONE")
