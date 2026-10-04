# 🧙‍♂️ 老吴大师（哈基仙版）

<p align="center">
  <img src="assets/laowu_master.jpg" alt="老吴大师（哈基仙版）" width="480"/>
</p>

> **极速、脱水、纯 Rust 实现的离线天文占星与东方玄学算力引擎**  
> 零 Java / Node.js / Python 运行时依赖，零 SQLite 动态链接，单二进制交付。

---

## 📜 许可证与上游衍生声明 (License & Attribution)

本项目基于 **[GNU Affero General Public License v3.0 (AGPL-3.0)](LICENSE)** 协议开源。

### 🌟 溯源说明 (Upstream Attribution)
- 本项目脱胎于上游 **`horosa-skill`** 项目。
- **重构工作**：本项目使用纯 **Rust** 对原项目中的排盘逻辑、行星摄动计算、三式九宗门、六爻纳甲、七政四余及各大占卜算法进行了彻底的底座重构与性能重写。
- **数据极限脱水**：将原分散的多源数据、典籍与星历全面提取，采用自定义结构化 Magic Header 与二进制压缩打包规范，脱水烘焙入 `data/` 目录二值化资产中，支持微秒级零拷贝内存映射（Mmap）访问。
- 根据 **AGPL-3.0 Section 4 / Section 5** 的传染性与保留声明要求：
  1. 本项目完整继承并遵循 **GNU AGPL-3.0** 开源许可证。
  2. 任何修改、分发或通过网络服务器（Web API / SaaS 等方式）向用户提供服务的衍生作品，亦必须遵循 AGPL-3.0 协议并开源全部对应源码。
  3. 显著保留上游项目版权声明与相关历史说明。

---

## 🏛️ 顶级架构与目录总览

```text
xuanxue-core/
├── Cargo.toml                  # Rust 依赖配置 (serde, memmap2, chrono, vsop87, lzma-rs 等)
├── Cargo.lock                  # 依赖版本锁定表
├── LICENSE                     # GNU AGPL-3.0 许可证
├── README.md                   # 项目说明与开发者指南
├── TECHNIQUES_110_SPEC.txt     # 110 项占卜技法完整规约与入参清单
├── assets/                     # 🎨 形象与视觉资产目录 (含老吴大师哈基仙版形象)
├── data/                       # 🗄️ 极限脱水后的二进制数据资产目录 (总计 ~130 MB)
│   ├── tiaowen.bin             # 18套古籍神数、条文、塔罗牌阵与卡巴拉
│   ├── ephem_planets_core.bin  # 现代核心600年高精行星切片
│   ├── ephem_moon_core.bin     # 现代核心600年月球物理切片
│   ├── ephem_planets_full.bin  # 全史6000年行星物理星历
│   ├── ephem_moon_full.bin     # 全史6000年月球高精物理星历
│   ├── ephem_asteroids_full.bin# 18颗主要小行星星历
│   ├── xuanshi.bin             # 二十四史历代天象志二值化数据库
│   ├── astrodata_index.bin     # 全球近6万知名名流极速搜索索引
│   ├── astrodata_details.bin   # 4万名流详细生平、维基传记与Rodden评级 (ADTS)
│   ├── bsc5_stars.bin          # 耶鲁亮星星表第5版 (8404颗恒星)
│   └── china_geo.bin           # 全国各省市区县高精经纬度与真太阳时基准
├── src/                        # 🦀 纯 Rust 算法源码实现 (54 个源文件)
│   ├── main.rs                 # 命令行交互与子命令引导
│   ├── dispatcher.rs           # 统一多路调度中心与严格防静默入参校验
│   ├── formatter.rs            # 统一快照文本排版格式化器
│   ├── db.rs                   # 通用二值化数据库驱动 (Mmap 零拷贝)
│   ├── dispatch/               # 领域分发子模块 (东方、西洋、民间杂占)
│   └── ...                     # 110 项技法专项算法实现文件
├── tests/                      # 🧪 110项技法大规模异步并发自动化测试套件
└── xuanxue-core.exe            # 🚀 预编译发布版单体可执行文件 (~1.9 MB)
```

---

## 🗄️ 数据资产架构 (`data/`)

所有数据文件均采用预编译头部（Magic Header）+ 32 字节结构体目录（Catalog）的二值化规范，支持 Rust 微秒级指针偏移与按需解压，日常内存常驻低于 3 MB：

| 文件名称 | 物理大小 | 内部数据结构与技术细节 | 服务技法范围 |
| :--- | :---: | :--- | :--- |
| **`tiaowen.bin`** | **1.14 MB** | 包含 18 套结构化典籍字典：铁板神数 (12000条)、邵子神数 (6144条)、蠢子神数 (4575条)、北极神数 (2341条)、南极神数 (246条)、皇极观物演义、心易发微、太玄诗法 (81首)、分经数、河洛理数 (64卦)、参评数、达摩一掌经、Sabian 360°符号，以及**塔罗 71 套经典牌阵、78 张牌深度象征图解与卡巴拉 32 阶路径表**。 | 14 大神数、河洛理数、邵子参评、塔罗抽牌与牌阵 |
| **`ephem_planets_core.bin`** | **408 KB** | 现代核心 600 年 (1800~2400 CE) 太阳与各大行星的高精切比雪夫多项式物理切片 (`sepl_00.se1`)。 | 现代所有排盘：八字秒级交节、奇门遁甲、大六壬、太乙神数、西占十大行星 |
| **`ephem_moon_core.bin`** | **1.16 MB** | 现代核心 600 年 (1800~2400 CE) 月球高精度物理摄动切片 (`semo_00.se1`)。 | 现代高精占星、月亮返照、七政四余、月相交点 |
| **`ephem_planets_full.bin`** | **20.76 MB** | 全史 6000 年 (BC 3000 ~ AD 3000) 太阳与各大行星真实物理星历 (`sepl*.se1` 共 50 卷)。 | 古代/远未来全史命盘与历史天文考据 |
| **`ephem_moon_full.bin`** | **61.23 MB** | 全史 6000 年 (BC 3000 ~ AD 3000) 月球高精摄动与真交点物理星历 (`semo*.se1` 共 50 卷)。 | 极远古/远未来高精占星与日月食推演 |
| **`ephem_asteroids_full.bin`** | **9.67 MB** | 全史 6000 年凯龙星、谷神星、智神星、婚神星、灶神星等 18 颗主要小行星物理星历 (`seas*.se1` 共 50 卷)。 | 小行星占星、婚恋合盘、深入推运 |
| **`xuanshi.bin`** | **13.47 MB** | 涵盖二十四史历代天象志、古代交食记录、客星彗星二值化历史天象库。 | 历史天象推演、古籍天象验证 |
| **`astrodata_index.bin`** | **2.20 MB** | 全球近 6 万名知名历史人物与名流极速检索索引表 (Magic: `ADTX`)。 | 名人案例库秒级查询、八字占星验证 |
| **`astrodata_details.bin`** | **19.92 MB** | 4 万名流详细生平、维基百科传记摘要、Rodden AA 评级与多重标签 (Magic: `ADTS`)。 | 案例库传记故事、学术研究详实考证 |
| **`bsc5_stars.bin`** | **112 KB** | 耶鲁亮星星表第 5 版：全天 8,404 颗恒星的精密赤经、赤纬、视星等与光谱型。 | 恒星占星、三垣二十八宿天象仪 |
| **`china_geo.bin`** | **87 KB** | 全国所有省、市、区、县级行政区精准经纬度基准表。 | 输入地名自动秒定经纬度与真太阳时差 |

---

## 🔮 110 项占卜技法分类全景

引擎支持 9 大门类、共计 110 项独立算力工具（全量专属入参及详细规范见 [`TECHNIQUES_110_SPEC.txt`](TECHNIQUES_110_SPEC.txt)）：

1. **东方术数与正统三式 (9项)**：
   - `bazi` (四柱八字平黄经交节排盘), `bazi_inverse` (干支逆推公历), `ziwei` (紫微斗数安星十二宫), `qimen` (奇门遁甲转盘时家局), `taiyi` (太乙神数主客算), `jinkou` (大六壬金口诀四位立式), `liureng` (大六壬九宗门三传四课), `liureng_runyear` (六壬行年神煞), `sanshiunited` (三式合一枢纽盘)。
2. **西洋古典与现代占星 (19项)**：
   - `chart` (本命盘 Natal Chart / 萨比恩度数), `transit` (行运盘), `synastry` (比较合盘), `composite_chart` (组合中点盘), `davison` (戴维森时空盘), `chart12` (十二分盘), `chart13` (十三分盘), `relocation` (重置盘), `acg` (占星制图天球投影), `solarreturn` (太阳返照), `lunarreturn` (月亮返照), `lunationphase` (月相周期), `prenatalsyzygy` (产前朔望), `triplicityrulers` (三分性主星), `keypoints` (阿拉伯点与恒星相交), `horary` (卜卦占星), `election` (择日占星评分), `babylon` (巴比伦恒星黄道), `germany` (汉堡学派中点)。
3. **七政四余与吠陀印占 (6项)**：
   - `guolao` (琴堂果老星宗), `qizhengkin` (洞微飞星), `vedic` (印度吠陀 D1/D9), `feigong` (吠陀飞宫互溶), `india_rectify` (印占生时校正), `relative` (六亲衍生分盘)。
4. **高阶星限与推运 (24项)**：
   - `zr` (黄道释放法), `balbillus` (巴比卢斯太阳主限), `decennials` (十年限法), `firdaria` (法达星限), `distributions` (界主限推运), `persiandirected` (波斯大年), `planetaryages` (行星生命周期), `yearsystem129` (瓦伦斯129年限), `profection` (小限法), `draconic` (龙首交点灵魂盘), `solararc` (太阳弧), `harmonic` (高阶谐波), `pd` (主限法半弧直升), `mundane` (世俗春分入宫盘), `ephemeris` (星历自检), `planet_cycles` (外行星周期), `returntimeline` (连续返照线), `extrareturns` (火金木等扩展返照), `agepoint` (胡伯年龄点), `planetaryarc` (行星弧长), `jaynesprog` (简恩次限推进), `vedicprog` (维姆绍塔里大限), `givenyear` (目标流年快照), `hellen_chart` (希腊化整宫制)。
5. **易经卦象与数术 (7项)**：
   - `liuyao` (纳甲六爻梅花起卦), `gua_desc` (六十四卦彖辞详解), `gua_meiyi` (梅花体用生克), `heluo` (河洛理数天地数原典), `canping` (邵子参评数大运), `shaozi` (邵子神数6144条文), `tieban` (铁板神数一万二千数)。
6. **十大神数典籍 (11项)**：
   - `shenshu` / `nanji` (南极神数), `beiji` (北极神数), `taixuan` (太玄八十一首), `wangji` (心易发微皇极经世), `cetian` (策天神数), `chunzi` (蠢子神数4575数), `fendjing` (分经数), `jingjue` (精决数), `shenyishu` (神易数), `wuzhao` (五兆数)。
7. **通胜择吉与历法 (12项)**：
   - `tongshu` (老黄历建除十二神), `bazizeri` (八字择吉扫描), `qimenzeri` (奇门吉门扫描), `taiyizeri` (太乙择吉), `ziweizeri` (紫微择日), `liurengzeri` (六壬择日), `sanshizeri` (三式合一择日), `qizhengzeri` (七政择吉), `indiazeri` (吠陀择日), `huanglizeri` (黄历择日), `zeri` (通用多维扫描), `qizhengelection` (七政天机择吉)。
8. **民间杂占与西方神秘学 (11项)**：
   - `xiaoliuren` (小六壬三传), `yizhangjing` (达摩一掌经轮转), `lingqi` (汉代灵棋经), `tarot` (78张塔罗牌洗牌与牌阵), `geomancy` (天文地占十六卦), `guice` (鬼谷子两头钳), `tongshefa` (古通社法), `yanqin` (演禽神数翻禽倒将), `xianqin` (六十花甲禽星), `suzhan` (宿占正传), `otherbu` (太公金钱课诸法)。
9. **天星小成图与知识库 (11项)**：
   - `tianxing` (五星聚舍交角), `tianxingzeri` (天星外步吉时), `xiaochengtu` (霍斐然小成图天地盘), `xuanshi` (正史天象志检索), `astrodata` (6万名流案例检索), `export_registry` (格式注册表), `knowledge_registry` (学术典籍索引), `nongli_time` (真太阳时折算), `jieqi_birth` (节气秒级交节), `jieqi_year` (全年节气时刻表), `calendar_month` (万年历公农历矩阵)。

---

## 📥 离线数据包下载与安装 (Data Assets Setup)

为了保持代码仓库的极度精简与极速拉取，底层 ~130 MB 的预编译二值化数据库资产托管在 **[GitHub Releases (v0.2.0)](https://github.com/Ldl-h/laowu-master/releases)** 中：

1. 从 Release 页面下载 **[`data.tar.gz`](https://github.com/Ldl-h/laowu-master/releases/download/v0.2.0/data.tar.gz)**。
2. 解压至项目根目录，确保生成 `data/` 文件夹：
   ```bash
   # Linux / macOS / Git Bash
   tar -xzvf data.tar.gz

   # Windows PowerShell
   tar -xzvf data.tar.gz
   ```
3. 也可以直接下载预编译好的发布单体程序 **[`xuanxue-core.exe`](https://github.com/Ldl-h/laowu-master/releases/download/v0.2.0/xuanxue-core.exe)** 配合 `data/` 即可直接运行。

---

## 🚀 极速上手与 CLI 指南

### 1. 自省与规范检索
```bash
# 查询引擎中全部 110 项技法清单及其必填参数与描述
./xuanxue-core.exe list

# 查询单项工具的专属入参规范及示例 payload
./xuanxue-core.exe spec qimen
./xuanxue-core.exe spec bazi
```

### 2. 标准 JSON 占卜调用
```bash
# 1. 高精八字排盘
./xuanxue-core.exe --tool bazi --input '{"date": "1998-10-24", "time": "08:30:00", "gender": 1, "after23_new_day": true, "late_zi_use_next_day": true}'

# 2. 奇门遁甲转盘局
./xuanxue-core.exe --tool qimen --input '{"year": 2026, "month": 6, "day": 21, "hour": 11, "minute": 30, "second": 0, "pai_pan_type": 1}'

# 3. 西洋本命占星盘 (附带萨比恩象征)
./xuanxue-core.exe --tool chart --input '{"date": "1995-05-18", "time": "15:45:00", "lat": 39.9042, "lon": 116.4074, "hsys": "placidus"}'

# 4. 塔罗三张牌时间流抽牌
./xuanxue-core.exe --tool tarot --input '{"spread": "three", "seed": 20261002}'

# 5. 城市自动经纬度匹配与真太阳时差解算
./xuanxue-core.exe --tool nongli_time --input '{"date": "1998-10-24", "time": "08:30:00", "city": "成都"}'
```

---

## 🧪 严格测试与验证

本项目内置专属的高并发端到端自动化测试套件：

```bash
# 执行 Rust 原生 32 项深度单元与端到端集成测试
cargo test

# 执行 110 项专属用例异步并发全量命理断言验收 (16 并发，平均耗时 ~0.8 秒，100% 通过)
python tests/py_tests/test_all_110_custom_inputs.py
```
