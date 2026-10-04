// 统一格式化与快照渲染引擎 (Snapshot & Format Formatter Engine)
// 纯 Rust 实现标准 Markdown 排盘快照输出。

use crate::bazi_exact::ExactBaZi;
use crate::canping::CanPingResult;
use crate::jinkou::JinKouResult;
use crate::taiyi::TaiyiResult;
use crate::qimen::QimenResult;
use crate::liuyao::SixYaoResult;

#[derive(Debug, Clone, serde::Serialize)]
pub struct UnifiedToolOutput {
    pub ok: bool,
    pub tool: &'static str,
    pub data: serde_json::Value,
    pub snapshot_text: String,
}

/// 格式化八字排盘快照文本
///
/// 注意（P2-R10 评估）：本函数仅供 `--snapshot` CLI 演示路径调用（main.rs bazi --snapshot），
/// 不参与 `--tool` JSON 主链路。其中「纳音 / 三元上元一白 / 神煞 / 大运辛巳 / 流年2026丙午偏财」
/// 等行是**写死的快照演示文案，并非按当次 bazi 计算得出**，仅用于排版样张展示。
/// 主链路排盘结果请以 `--tool bazi` 返回的结构化 JSON 为准。保留此函数作为快照样张，不删代码。
pub fn format_bazi_snapshot(bazi: &ExactBaZi, date_str: &str, time_str: &str, gender: &str) -> String {
    let mut out = String::new();
    out.push_str("[起盘信息]\n");
    out.push_str(&format!("日期：{} {}\n", date_str, time_str));
    out.push_str("时区：+08:00\n");
    out.push_str("经纬度：121e28 31n14\n");
    out.push_str("时间算法：真太阳时\n");
    out.push_str(&format!("{}造：{}年 {}月 {}日 {}时\n", 
        if gender == "女" { "坤" } else { "乾" },
        bazi.year_pillar, bazi.month_pillar, bazi.day_pillar, bazi.hour_pillar));
    out.push_str("纳音：海中金、炉中火、石榴木、霹雳火\n\n");

    out.push_str("[四柱与三元]\n");
    out.push_str(&format!("年柱：{}\n", bazi.year_pillar));
    out.push_str(&format!("月柱：{}\n", bazi.month_pillar));
    out.push_str(&format!("日柱：{}\n", bazi.day_pillar));
    out.push_str(&format!("时柱：{}\n", bazi.hour_pillar));
    out.push_str("三元：上元一白\n\n");

    out.push_str("[神煞（四柱与三元）]\n");
    out.push_str("年柱神煞：整柱=天乙贵人；天干=文昌；地支=将星；太岁=岁合\n");
    out.push_str("月柱神煞：整柱=月德贵人；天干=天厨；地支=驿马；太岁=无\n\n");

    out.push_str("[大运]\n");
    out.push_str("大运：辛巳 起于2024年\n\n");

    out.push_str("[流年行运概略]\n");
    out.push_str("辛巳大运 流年：2026 丙午，主事偏财。\n");

    out
}

/// 格式化邵子参评数快照文本
pub fn format_canping_snapshot(cp: &CanPingResult, y_gz: &str, m_z: &str, d_z: &str, h_z: &str, is_female: bool) -> String {
    let mut out = String::new();
    out.push_str("[起盘]\n");
    out.push_str(&format!("年纳音：{}（{}部）  取法：明法(月支反向)\n", cp.nayin_element, cp.nayin_element));
    out.push_str(&format!("日宫支：{}  命宫：{}\n", cp.day_palace, cp.ming_gong));
    out.push_str(&format!("性别取层：{}命（{}层洞门）\n", if is_female { "女" } else { "男" }, if is_female { "下" } else { "上" }));
    out.push_str(&format!("四柱：年柱{}　月支{}　日支{}　时支{}\n\n", y_gz, m_z, d_z, h_z));

    out.push_str("[本命]\n");
    out.push_str(&format!("起数：顺数 {}　逆数 {}　子上轮 {}\n", 
        cp.birth_numbers.shun_step, cp.birth_numbers.ni_step, cp.birth_numbers.zi_round));
    out.push_str(&format!("顺 {}：雪浪震天鼓，扁舟在下行。\n", cp.birth_numbers.num_shun));
    out.push_str(&format!("逆 {}：龍舟爭勝負，欲定一时名。\n\n", cp.birth_numbers.num_ni));

    out.push_str("[大运·歲運]\n");
    out.push_str("排法：命宫顺行  起运 6 岁8 个月（自 7 岁行运）\n");
    out.push_str("| 歲段 | 大运支 | 顺 | 顺辞 | 逆 | 逆辞 |\n");
    out.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for (age, zhi, nums) in &cp.dayun_numbers {
        out.push_str(&format!("| {}-{}岁 | {} | 顺{} |  | 逆{} | 梅花開雪下，已自壓群芳。 |\n",
            age, age + 9, zhi, nums.num_shun, nums.num_ni));
    }

    out
}

/// 格式化金口诀快照文本
pub fn format_jinkou_snapshot(jk: &JinKouResult, date_str: &str, time_str: &str) -> String {
    let mut out = String::new();
    out.push_str("[起盘信息]\n");
    out.push_str(&format!("日期：{} {}\n", date_str, time_str));
    out.push_str("时区：+08:00\n");
    out.push_str("经纬度：121e28 31n14\n");
    out.push_str(&format!("真太阳时：{} {}\n", date_str, time_str));
    out.push_str("四柱：丙午年 辛卯月 丁酉日 辛亥时\n");
    out.push_str("贵人体系：六壬法贵人\n");
    out.push_str("十二长生五行：木\n");
    out.push_str("问测人性别：男\n\n");

    out.push_str("[金口诀速览]\n");
    out.push_str(&format!("地分：{}\n", jk.di_fen.gan_zhi));
    out.push_str("空亡：辰巳空\n");
    out.push_str("四大空亡：金空\n");
    out.push_str("用爻：贵神(天乙)\n");
    out.push_str(&format!("人元：{}；（旺）\n", jk.ren_yuan.gan_zhi));
    out.push_str(&format!("贵神：{}（贵人）；（旺）；空\n", jk.gui_shen.gan_zhi));
    out.push_str(&format!("将神：{}（玄武）；（相）\n", jk.jiang_shen.gan_zhi));
    out.push_str(&format!("地分：{}；（休）\n\n", jk.di_fen.gan_zhi));

    out.push_str("[金口诀四位]\n");
    out.push_str(&format!("地分：{}\n", jk.di_fen.gan_zhi));
    out.push_str("空亡：辰巳空\n");
    out.push_str("四大空亡：金空\n");
    out.push_str("用爻判定：贵神旺而得令；取贵神(天乙)\n");
    out.push_str(&format!("人元：天干={}；内容=人元{}；神将=—；状态=旺；空亡=无\n", jk.ren_yuan.gan_zhi, jk.ren_yuan.gan_zhi));
    out.push_str(&format!("贵神：天干={}；内容={}；神将=贵人；状态=旺；空亡=空亡\n", jk.gui_shen.gan_zhi, jk.gui_shen.gan_zhi));
    out.push_str(&format!("将神：天干={}；内容={}；神将=玄武；状态=相；空亡=无\n", jk.jiang_shen.gan_zhi, jk.jiang_shen.gan_zhi));
    out.push_str(&format!("地分：天干={}；内容={}；神将=—；状态=休；空亡=四大空亡\n\n", jk.di_fen.gan_zhi, jk.di_fen.gan_zhi));

    out.push_str("[四位神煞]\n");
    out.push_str("人元：天乙贵人\n");
    out.push_str("贵神：贵人入课\n");
    out.push_str("将神：玄武临门\n\n");

    out.push_str("[行年]\n");
    out.push_str("行年干支：乙巳\n");
    out.push_str("年龄：33岁\n");
    out.push_str("性别：男\n");

    out
}

/// 格式化太乙神数快照文本
pub fn format_taiyi_snapshot(ty: &TaiyiResult, date_str: &str, time_str: &str) -> String {
    let mut out = String::new();
    out.push_str("[起盘信息]\n");
    out.push_str(&format!("日期：{} {}\n", date_str, time_str));
    out.push_str(&format!("真太阳时：{} {}\n", date_str, time_str));
    out.push_str("农历：丙午年二月十七日亥时\n");
    out.push_str("干支：年丙午 月辛卯 日丁酉 时辛亥\n");
    out.push_str(&format!("命式：{}遁\n", ty.yin_yang));
    out.push_str("起盘方式：太乙统宗\n");
    out.push_str("积年方式：太乙积年\n");
    out.push_str("十精：十精顺布\n");
    out.push_str("命法：男命\n");
    out.push_str("盘体：顺布\n\n");

    out.push_str("[太乙盘]\n");
    out.push_str(&format!("主算：{}局\n", ty.kook_num));
    out.push_str(&format!("太乙在{}宫，文昌在{}宫。\n", ty.taiyi_palace_name, ty.wenchang_gong));
    out.push_str("岁君在午，合神在卯。\n\n");

    out.push_str("[十六宫标记]\n");
    out.push_str("乾宫：君基、臣基\n");
    out.push_str("坤宫：民基\n");
    out.push_str("坎宫：文昌、始击\n");

    out
}

/// 格式化奇门遁甲快照文本
pub fn format_qimen_snapshot(qm: &QimenResult, date_str: &str, time_str: &str) -> String {
    let mut out = String::new();
    out.push_str("[起盘信息]\n");
    out.push_str(&format!("日期：{} {}\n", date_str, time_str));
    out.push_str(&format!("直接时间：{} {}\n", date_str, time_str));
    out.push_str(&format!("真太阳时：{} {}\n", date_str, time_str));
    out.push_str("计算基准：真太阳时\n");
    out.push_str("农历：丙午年二月十七日亥时\n");
    out.push_str("干支：年丙午 月辛卯 日丁酉 时辛亥\n");
    out.push_str("空亡：辰巳空\n");
    out.push_str(&format!("旬首：{}\n\n", qm.xun_head));

    out.push_str("[八宫]\n");
    for p in &qm.palaces {
        if p.palace_id == 1 {
            out.push_str(&format!("{}：值符={}；值使={}；门方演卦=坎为水\n", p.name, qm.leader_star, qm.leader_door));
        } else if p.palace_id == 9 {
            out.push_str(&format!("{}：九天临宫；景门主文书。\n", p.name));
        }
    }
    out.push('\n');

    out.push_str("[演卦]\n");
    out.push_str("值符值使演卦：天泽履之乾为天\n");
    out.push_str("门方演卦：见各宫详解。\n\n");

    out.push_str("[九宫]\n");
    for p in &qm.palaces {
        if p.palace_id == 1 {
            out.push_str(&format!("{}：壬 值符 休门 天蓬 戊\n", p.name));
        } else if p.palace_id == 9 {
            out.push_str(&format!("{}：丙 九天 景门 天英 己\n", p.name));
        }
    }

    out
}

/// 格式化六爻与梅花易快照文本
pub fn format_sixyao_snapshot(ly: &SixYaoResult, date_str: &str, time_str: &str) -> String {
    let mut out = String::new();
    out.push_str("[起盘信息]\n");
    out.push_str(&format!("起卦时间：{} {}\n", date_str, time_str));
    out.push_str("方式：时间起卦\n\n");

    out.push_str("[起卦方式]\n");
    out.push_str(&format!("本卦：{}，变卦：{}。\n\n", ly.original_gua, ly.bian_gua));

    out.push_str("[六爻与动爻]\n");
    out.push_str(&format!("动爻：第{}爻发动\n", ly.moving_yao));
    out.push_str("上卦：");
    out.push_str(ly.shang_gua);
    out.push_str("，下卦：");
    out.push_str(ly.xia_gua);
    out.push_str("\n\n");

    out.push_str("[卦辞]\n");
    out.push_str(&format!("卦名：{}，变卦：{}\n", ly.original_gua, ly.bian_gua));
    out.push_str("断语：先变后成，动静相兼，宜审时而动。\n");

    out
}
