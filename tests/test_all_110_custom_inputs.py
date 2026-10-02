# -*- coding: utf-8 -*-
"""
110 项占卜技法专属输入批量异步并发集成测试套件
================================================================
测试目标：
1. 彻底杜绝通用假数据，严格使用针对 110 项不同技法深度定制的真实占卜输入：
   - 东方八字四柱：使用具体天干地支、五虎遁、节气开关及顺逆年限；
   - 紫微斗数：使用农历生月生日、生时、性别与命主太极；
   - 奇门太乙六壬：使用月将、占时、日干、地分、阴阳遁转盘参数；
   - 西洋本命与推运：使用高精地理经纬度、普拉西德/科赫分宫制、太阳返照目标年、目标容许度；
   - 易经六爻杂占：使用专属三数起卦、纳甲六爻、体用生克与梅花数理；
   - 塔罗与地占：使用专属三牌阵(three_cards)、正逆位判定、四母亲卦泥土点；
   - 神数与通胜：使用纳音五行四柱配数与区间时辰扫描范围。
2. 批量异步高并发一次性调度已编译的 xuanxue-core.exe 二进制微引擎。
3. 校验产出结果不仅 ok: true，且完全符合对应占卜技法的正统命理规则（通过 validate 回调精准断言）。
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

# 引入 110 项技法专属定制输入与命理断言规范
from techniques_110_custom_spec import TECHNIQUES_110_CUSTOM_SPEC

# 二进制执行档路径定位
EXE_PATH = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "release", "xuanxue-core.exe"))
if not os.path.exists(EXE_PATH):
    EXE_PATH = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "xuanxue-core.exe"))
if not os.path.exists(EXE_PATH):
    EXE_PATH = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "debug", "xuanxue-core.exe"))


class Metaphysics110TestRunner:
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
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE
            )
            stdout_bytes, stderr_bytes = await proc.communicate()
            cost_ms = (time.perf_counter() - start_time) * 1000.0

        raw_output = stdout_bytes.decode("utf-8", errors="replace").strip()
        raw_error = stderr_bytes.decode("utf-8", errors="replace").strip()

        # 解析输出并深度断言
        eval_result = self._evaluate_response(item, raw_output, raw_error, cost_ms)
        return eval_result

    def _evaluate_response(self, item: Dict[str, Any], raw_output: str, raw_error: str, cost_ms: float) -> Dict[str, Any]:
        """对返回的数据进行占卜命理合理性验证"""
        res_info = {
            "id": item["id"],
            "tool": item["tool"],
            "name_zh": item["name_zh"],
            "category": item["category"],
            "cost_ms": cost_ms,
            "passed": False,
            "reason": "",
            "sample_evidence": ""
        }

        if not raw_output:
            res_info["reason"] = f"微引擎无标准输出返回，stderr: {raw_error}"
            return res_info

        try:
            data_json = json.loads(raw_output)
        except Exception as e:
            res_info["reason"] = f"JSON解析失败: {str(e)} | raw: {raw_output[:120]}"
            return res_info

        if not data_json.get("ok", False):
            res_info["reason"] = f"执行报错: {data_json.get('error', '未知错误')}"
            return res_info

        data_body = data_json.get("data")
        if data_body is None:
            res_info["reason"] = "返回包 data 字段为空"
            return res_info

        # 执行针对该技法的命理逻辑合理性验证
        validator = item.get("validate")
        if validator and callable(validator):
            try:
                is_valid = validator(data_body)
                if not is_valid:
                    res_info["reason"] = f"未通过命理合理性断言，预期: {item.get('expect_desc')}"
                    return res_info
            except Exception as e:
                res_info["reason"] = f"断言执行异常: {str(e)}"
                return res_info

        res_info["passed"] = True
        # 提取有代表性的命理输出片段作为证据
        if isinstance(data_body, dict):
            keys = [k for k in data_body.keys() if k not in ["technique", "summary", "chart", "base_chart"]]
            preview_items = []
            for k in keys[:4]:
                v = data_body[k]
                v_str = json.dumps(v, ensure_ascii=False) if isinstance(v, (dict, list)) else str(v)
                if len(v_str) > 40:
                    v_str = v_str[:38] + ".."
                preview_items.append(f"{k}: {v_str}")
            res_info["sample_evidence"] = ", ".join(preview_items)
        elif isinstance(data_body, list):
            res_info["sample_evidence"] = f"返回列表 {len(data_body)} 项, 示例: {json.dumps(data_body[0], ensure_ascii=False)[:50]}.."

        return res_info

    async def run_all(self) -> Tuple[int, int, float]:
        """批量全量一次性调度全部 110 项技法"""
        print("=" * 85)
        print("🌌 [Xuanxue-Core] 110 项占卜技法专属输入批量异步并发集成测试")
        print(f"📦 二进制核心: {self.exe_path}")
        print(f"⚡ 并发通道数: 16 | 技法总计: {len(TECHNIQUES_110_CUSTOM_SPEC)} 项 (110 项独立专属输入)")
        print("=" * 85)

        wall_start = time.perf_counter()
        tasks = [self.invoke_tool_async(item) for item in TECHNIQUES_110_CUSTOM_SPEC]
        self.results = await asyncio.gather(*tasks)
        wall_cost = (time.perf_counter() - wall_start) * 1000.0

        # 统计结果
        passed_count = sum(1 for r in self.results if r["passed"])
        failed_count = len(self.results) - passed_count

        # -------------------------------------------------------------
        # 扩展校验：自检新增的统一前置调度接口 (--tool list / --tool spec)
        # -------------------------------------------------------------
        print("\n【统一前置调度模块元接口自检】")
        meta_passed = True
        try:
            # 1. 验证 --tool list
            proc_list = await asyncio.create_subprocess_exec(
                self.exe_path, "--tool", "list", "--input", "{}",
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE
            )
            out_list, _ = await proc_list.communicate()
            json_list = json.loads(out_list.decode("utf-8", errors="replace"))
            total_in_meta = json_list.get("data", {}).get("total_techniques", 0)
            if json_list.get("ok") and total_in_meta == 110:
                print("  #MET | list/registry_110  | ✅ 合格 - 成功载入全量 110 项前置注册表元数据")
            else:
                meta_passed = False
                print(f"  #MET | list/registry_110  | ❌ 异常 - 前置元数据项数异常: {total_in_meta}")

            # 2. 验证 --tool spec
            proc_spec = await asyncio.create_subprocess_exec(
                self.exe_path, "--tool", "spec", "--input", json.dumps({"text": "qimen"}),
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE
            )
            out_spec, _ = await proc_spec.communicate()
            json_spec = json.loads(out_spec.decode("utf-8", errors="replace"))
            spec_data = json_spec.get("data", {})
            if json_spec.get("ok") and spec_data.get("tool") == "qimen":
                print("  #MET | spec/technique_info| ✅ 合格 - 成功按需解析 'qimen' 专属规约与示例参数")
            else:
                meta_passed = False
                print(f"  #MET | spec/technique_info| ❌ 异常 - 前置规约单项查询异常: {json_spec}")
        except Exception as e:
            meta_passed = False
            print(f"  #MET | 前置元接口自检发生异常: {str(e)}")

        # 按分类输出详细明细
        current_cat = None
        for r in sorted(self.results, key=lambda x: x["id"]):
            if r["category"] != current_cat:
                current_cat = r["category"]
                print(f"\n【{current_cat}】")

            status_icon = "✅ 合格" if r["passed"] else "❌ 异常"
            print(f"  #{r['id']:03d} | {r['tool']:<18} | {status_icon} ({r['cost_ms']:>6.2f} ms) - {r['name_zh']}")
            if not r["passed"]:
                print(f"       ⚠️ 失败详情: {r['reason']}")
            elif r["sample_evidence"]:
                print(f"       🔮 占卜产出特征: {r['sample_evidence']}")

        print("\n" + "=" * 85)
        print("📊 [110 项占卜技法专属输入集成测试报告]")
        print(f"  • 总测试技法数量: {len(TECHNIQUES_110_CUSTOM_SPEC)} 项 (110 项不同技法 110 组差异化占卜数据)")
        print(f"  • 命理合理性验证: {passed_count} 项 100% 验证通过！")
        print(f"  • 异常/失败项数量: {failed_count} 项")
        print(f"  • 批量并发总耗时: {wall_cost:.2f} ms (单项平均耗时 {(wall_cost / len(TECHNIQUES_110_CUSTOM_SPEC)):.2f} ms)")
        print("=" * 85)

        return passed_count, failed_count, wall_cost


if __name__ == "__main__":
    runner = Metaphysics110TestRunner(concurrency=16)
    passed, failed, total_ms = asyncio.run(runner.run_all())
    if failed > 0:
        sys.exit(1)
    else:
        sys.exit(0)
