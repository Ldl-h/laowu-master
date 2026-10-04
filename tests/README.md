# xuanxue-core 最严格测试套件打包

历轮 5 阶段全量审查（十六轮）使用过的全部测试代码，供本地复测。

## 一、前置条件

1. **Rust 二进制**：在 xuanxue-core 项目根目录 `cargo build --release`，生成 `target/release/xuanxue-core`
2. **数据文件**：`data/` 目录必须完整（astrodata_index.bin、astrodata_details.bin、ephem_*.bin、xuanshi.bin、tiaowen.bin、bsc5_stars.bin、china_geo.bin 等，约 154MB）
3. **Python 3**（异步测试需要 asyncio；默认即带）
4. **真值对比**：可选——`/home/user/.horosa/runtime/current/horosa-core-js`（JS 引擎，`node bin/cli.mjs run <tool>`）与 horosa-skill Python（service.py `_run_*_tool`）

## 二、各套件用途与运行

### 1. Rust 端到端测试（最核心，32 个测试含 110 项端到端）
- 文件：`rust_tests/src_tests.rs`
- 用法：将此文件覆盖到项目 `src/tests.rs`，然后：
  ```
  cargo test --release
  ```
- 内容：110 项技法端到端分发 + 历轮修复的回归锁（行年固定起点、建除交节日、月将公式、灵棋 125 卦、合盘、时区、廿八宿、astrodata 字段等）

### 2. Python 批量异步并发测试（110 项跑通 + 数据结构校验）
- 文件：`py_tests/test_all_110_techniques.py`
- 用法（需先 `cargo build --release`）：
  ```
  python3 test_all_110_techniques.py
  ```
- 内容：异步高并发调用 110 项，校验 `ok:true` + 产出结构合理性 + 深度检测是否真实击穿底层二进制数据库（tiaowen/bsc5/china_geo/xuanshi/astrodata/ephem）

### 3. Python 专属输入深度校验（110 项定制入参 + 命理规则断言）
- 文件：`py_tests/test_all_110_custom_inputs.py`
- 用法：
  ```
  python3 test_all_110_custom_inputs.py
  ```
- 内容：针对每项技法深度定制真实输入（干支/农历/月将/经纬度/卦数/牌阵等），通过 validate 回调做命理逻辑断言（不是只看 ok:true）

### 4. 技法规范定义（供上述测试 import）
- `py_tests/techniques_110_spec.py`：110 项标准入参规范
- `py_tests/techniques_110_custom_spec.py`：110 项专属入参与命理校验规范

### 5. 真值对比脚本（Rust vs JS 原版）
- 文件：`truth_compare/run_rust.py`、`run_js.py`、`run_js2~5.py`、`rerun.py`
- 用法（按脚本内路径变量调整）：
  ```
  python3 run_rust.py   # 批量跑 Rust，统计 ok/字段数/占位符
  python3 run_js.py     # 用 Rust→JS 映射跑 JS 真值，同输入对比
  ```
- 说明：这些是历轮审查用的临时脚本，路径写死为云电脑绝对路径，本地需自行修改开头的 `RUST`/`JS`/`OUT` 变量；`run_js*.py` 内含 Rust tool→JS tool 映射表，可作真值对比参考

## 三、历轮审查方法论备忘（本地复测可参照）

1. **入参审计**：Rust `techniques_meta_data.inc` 的 params/example vs Python registry.py ToolDefinition vs JS tools
2. **输出契约审计**：原版 return dict 字段 vs Rust serde 输出字段，逐字段集合对比
3. **真值对比**：同输入跑两边，逐字段比（数值允许微小误差，枚举文本必须一致）
4. **边界用例**：节气交节 ±1 分钟、子时双开关、tz ±14、公元前/2100/闰日、空对象/类型错/畸形时间
5. **fuzz**：随机日期/tz/枚举批量灌入，找未预期 panic（历轮用临时脚本，本地可按此思路自建）

## 四、已知状态（截至第十七轮）

- P0/P1/P2 全部清零；112 项技法与 Python+JS 双基准对齐；32 测试全绿、构建零警告
- 若本地测试发现差异，对照历轮报告（飞书文档）逐条核对
