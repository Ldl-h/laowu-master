// 紫微斗数排盘与格局全量推演核心 (ZiWei Dou Shu Complete Engine)
// 包含：安命身宫、五行局、定紫微天府星系、安十四主星、生年四化、
// 禄存羊陀魁钺、左右昌曲、火铃空劫台封、吉凶神煞群曜、以及经典大格局自动推理

pub const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
pub const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

pub const HOUSES: [&str; 12] = [
    "命宫", "兄弟宫", "夫妻宫", "子女宫", "财帛宫", "疾厄宫",
    "迁移宫", "奴仆宫", "官禄宫", "田宅宫", "福德宫", "父母宫"
];

// 六十甲子纳音五行局 (根据命宫干支)
pub const NAYIN_JU: [(&str, u32, &str); 60] = [
    ("甲子", 4, "金四局"), ("乙丑", 4, "金四局"), ("丙寅", 6, "火六局"), ("丁卯", 6, "火六局"),
    ("戊辰", 3, "木三局"), ("己巳", 3, "木三局"), ("庚午", 5, "土五局"), ("辛未", 5, "土五局"),
    ("壬申", 4, "金四局"), ("癸酉", 4, "金四局"), ("甲戌", 6, "火六局"), ("乙亥", 6, "火六局"),
    ("丙子", 2, "水二局"), ("丁丑", 2, "水二局"), ("戊寅", 5, "土五局"), ("己卯", 5, "土五局"),
    ("庚辰", 4, "金四局"), ("辛巳", 4, "金四局"), ("壬午", 3, "木三局"), ("癸未", 3, "木三局"),
    ("甲申", 2, "水二局"), ("乙酉", 2, "水二局"), ("丙戌", 5, "土五局"), ("丁亥", 5, "土五局"),
    ("戊子", 6, "火六局"), ("己丑", 6, "火六局"), ("庚寅", 3, "木三局"), ("辛卯", 3, "木三局"),
    ("壬辰", 2, "水二局"), ("癸巳", 2, "水二局"), ("甲午", 4, "金四局"), ("乙未", 4, "金四局"),
    ("丙申", 6, "火六局"), ("丁酉", 6, "火六局"), ("戊戌", 3, "木三局"), ("己亥", 3, "木三局"),
    ("庚子", 5, "土五局"), ("辛丑", 5, "土五局"), ("壬寅", 4, "金四局"), ("癸卯", 4, "金四局"),
    ("甲辰", 6, "火六局"), ("乙巳", 6, "火六局"), ("丙午", 2, "水二局"), ("丁未", 2, "水二局"),
    ("戊申", 5, "土五局"), ("己酉", 5, "土五局"), ("庚戌", 4, "金四局"), ("辛亥", 4, "金四局"),
    ("壬子", 3, "木三局"), ("癸丑", 3, "木三局"), ("甲寅", 2, "水二局"), ("乙卯", 2, "水二局"),
    ("丙辰", 5, "土五局"), ("丁巳", 5, "土五局"), ("戊午", 6, "火六局"), ("己未", 6, "火六局"),
    ("庚申", 3, "木三局"), ("辛酉", 3, "木三局"), ("壬戌", 2, "水二局"), ("癸亥", 2, "水二局"),
];

// 十干生年四化 [禄, 权, 科, 忌]
pub const SIHUA_TABLE: [[&str; 4]; 10] = [
    ["廉贞", "破军", "武曲", "太阳"], // 甲
    ["天机", "天梁", "紫微", "太阴"], // 乙
    ["天同", "天机", "文昌", "廉贞"], // 丙
    ["太阴", "天同", "天机", "巨门"], // 丁
    ["贪狼", "太阴", "右弼", "天机"], // 戊
    ["武曲", "贪狼", "天梁", "文曲"], // 己
    ["太阳", "武曲", "太阴", "天同"], // 庚
    ["巨门", "太阳", "文曲", "文昌"], // 辛
    ["天梁", "紫微", "左辅", "武曲"], // 壬
    ["破军", "巨门", "太阴", "贪狼"], // 癸
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct ZiWeiPalace {
    pub house_name: &'static str,  // 宫位名称 (命宫, 兄弟宫...)
    pub zhi: &'static str,         // 地支 (子, 丑...)
    pub gan: String,               // 宫干
    pub main_stars: Vec<String>,   // 十四正曜
    pub assist_stars: Vec<String>, // 吉星助星 (左辅, 右弼, 文昌, 文曲, 天魁, 天钺, 禄存, 天马)
    pub evil_stars: Vec<String>,   // 六煞星 (擎羊, 陀罗, 火星, 铃星, 地劫, 地空)
    pub other_stars: Vec<String>,  // 杂曜神煞 (天官, 天福, 天刑, 天姚, 红鸾, 天喜, 孤辰, 寡宿等)
    pub sihua: Vec<String>,        // 四化标注 (如 "廉贞·化禄")
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ZiWeiPattern {
    pub name: &'static str,
    pub category: &'static str,   // 富贵格 / 凶格 / 杂格
    pub description: &'static str,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DaXianStep {
    pub step: usize,
    pub gong_name: &'static str,
    pub gong_zhi: &'static str,
    pub start_age: u32,
    pub end_age: u32,
}

#[derive(Debug, serde::Serialize)]
pub struct ZiWeiResult {
    pub ming_palace_zhi: &'static str,
    pub shen_palace_zhi: &'static str,
    pub ming_zhu: &'static str,
    pub shen_zhu: &'static str,
    pub wuxing_ju: String,
    pub ziwei_pos: &'static str,
    pub tianfu_pos: &'static str,
    pub da_xian: Vec<DaXianStep>,
    pub palaces: Vec<ZiWeiPalace>,
    pub patterns: Vec<ZiWeiPattern>,
}

pub fn get_ziwei_pos(day: u32, ju: u32) -> usize {
    let q = day / ju;
    let r = day % ju;
    let (bu, shang) = if r == 0 {
        (0, q)
    } else {
        (ju - r, q + 1)
    };
    let base = (2 + (shang as i32 - 1)).rem_euclid(12); // 寅宫(=2) 起算
    if bu % 2 == 0 {
        ((base + bu as i32).rem_euclid(12)) as usize
    } else {
        ((base - bu as i32).rem_euclid(12)) as usize
    }
}

pub fn zhi_index(z: &str) -> usize {
    ZHI.iter().position(|&item| item == z).unwrap_or(0)
}

/// 全量紫微斗数排盘与格局推理计算
pub fn calculate_ziwei(month: u32, day: u32, hour_zhi_idx: usize, year_gan_idx: usize) -> ZiWeiResult {
    calculate_ziwei_full(month, day, hour_zhi_idx, year_gan_idx, 0, true)
}

/// 完整参数版本（带年支与性别）
pub fn calculate_ziwei_full(
    month: u32,
    day: u32,
    hour_zhi_idx: usize,
    year_gan_idx: usize,
    year_zhi_idx: usize,
    _is_male: bool,
) -> ZiWeiResult {
    // 1. 定命身宫
    let month_offset = (month as i32 - 1).rem_euclid(12);
    let h_idx = hour_zhi_idx as i32;
    let ming_idx = (2 + month_offset - h_idx).rem_euclid(12) as usize;
    let shen_idx = (2 + month_offset + h_idx).rem_euclid(12) as usize;

    let ming_zhi = ZHI[ming_idx];
    let shen_zhi = ZHI[shen_idx];

    // 2. 定五虎遁宫干
    let yin_gan_base = match year_gan_idx % 5 {
        0 => 2, // 甲己 -> 丙
        1 => 4, // 乙庚 -> 戊
        2 => 6, // 丙辛 -> 庚
        3 => 8, // 丁壬 -> 壬
        _ => 0, // 戊癸 -> 甲
    };

    // 命宫纳音五行局
    let ming_gan_step = (ming_idx as i32 + 10).rem_euclid(12) as usize;
    let ming_gan = GAN[(yin_gan_base + ming_gan_step) % 10];
    let ming_gz = format!("{}{}", ming_gan, ming_zhi);

    let mut ju_num = 2;
    let mut wuxing_ju = "水二局".to_string();
    for (gz, num, name) in NAYIN_JU.iter() {
        if *gz == ming_gz.as_str() {
            ju_num = *num;
            wuxing_ju = name.to_string();
            break;
        }
    }

    // 3. 定紫微星与天府星落宫
    let zw_idx = get_ziwei_pos(day, ju_num);
    let tf_idx = ((4 - zw_idx as i32).rem_euclid(12)) as usize;

    // 4. 十四正曜落宫推算
    let north_steps = [("紫微", 0), ("天机", -1), ("太阳", -3), ("武曲", -4), ("天同", -5), ("廉贞", -8)];
    let south_steps = [("天府", 0), ("太阴", 1), ("贪狼", 2), ("巨门", 3), ("天相", 4), ("天梁", 5), ("七杀", 6), ("破军", 10)];

    let mut main_stars: [Vec<String>; 12] = Default::default();
    let mut assist_stars: [Vec<String>; 12] = Default::default();
    let mut evil_stars: [Vec<String>; 12] = Default::default();
    let mut other_stars: [Vec<String>; 12] = Default::default();

    for (star, step) in north_steps {
        let p = ((zw_idx as i32 + step).rem_euclid(12)) as usize;
        main_stars[p].push(star.to_string());
    }
    for (star, step) in south_steps {
        let p = ((tf_idx as i32 + step).rem_euclid(12)) as usize;
        main_stars[p].push(star.to_string());
    }

    // 5. 安辅助星曜 (左右昌曲魁钺禄马)
    // 左辅：辰宫起正月顺数；右弼：戌宫起正月逆数
    let zuofu_idx = (4 + month_offset).rem_euclid(12) as usize;
    let youbi_idx = (10 - month_offset).rem_euclid(12) as usize;
    assist_stars[zuofu_idx].push("左辅".to_string());
    assist_stars[youbi_idx].push("右弼".to_string());

    // 文昌：戌宫起子时逆数；文曲：辰宫起子时顺数
    let wenchang_idx = (10 - h_idx).rem_euclid(12) as usize;
    let wenqu_idx = (4 + h_idx).rem_euclid(12) as usize;
    assist_stars[wenchang_idx].push("文昌".to_string());
    assist_stars[wenqu_idx].push("文曲".to_string());

    // 禄存、擎羊、陀罗 (依年干起)
    let lucun_map = [2, 3, 5, 6, 5, 6, 8, 9, 11, 0]; // 寅卯巳午巳午申酉亥子
    let lu_idx = lucun_map[year_gan_idx % 10];
    assist_stars[lu_idx].push("禄存".to_string());

    // 擎羊居禄前一位，陀罗居禄后一位
    let qingyang_idx = (lu_idx + 1) % 12;
    let tuoluo_idx = (lu_idx + 11) % 12;
    evil_stars[qingyang_idx].push("擎羊".to_string());
    evil_stars[tuoluo_idx].push("陀罗".to_string());

    // 天魁、天钺 (依年干起)
    // 甲戊庚丑未，乙己子申乡，丙丁亥酉位，辛逢午寅存，壬癸卯巳藏
    let kui_yue_map = [
        (1, 7),  // 甲 -> 丑, 未
        (0, 8),  // 乙 -> 子, 申
        (11, 9), // 丙 -> 亥, 酉
        (11, 9), // 丁 -> 亥, 酉
        (1, 7),  // 戊 -> 丑, 未
        (0, 8),  // 己 -> 子, 申
        (1, 7),  // 庚 -> 丑, 未
        (6, 2),  // 辛 -> 午, 寅
        (3, 5),  // 壬 -> 卯, 巳
        (3, 5),  // 癸 -> 卯, 巳
    ];
    let (kui_idx, yue_idx) = kui_yue_map[year_gan_idx % 10];
    assist_stars[kui_idx].push("天魁".to_string());
    assist_stars[yue_idx].push("天钺".to_string());

    // 天马 (月马或年马，这里取经典寅申巳亥月马)
    let tianma_idx = match month % 4 {
        1 => 8, // 申
        2 => 5, // 巳
        3 => 2, // 寅
        _ => 11, // 亥
    };
    assist_stars[tianma_idx].push("天马".to_string());

    // 6. 安六煞与时曜 (火星, 铃星, 地空, 地劫, 台辅, 封诰)
    // 地劫：亥上起子顺时针数；地空：亥上起子逆时针数
    let dijie_idx = (11 + h_idx).rem_euclid(12) as usize;
    let dikong_idx = (11 - h_idx).rem_euclid(12) as usize;
    evil_stars[dijie_idx].push("地劫".to_string());
    evil_stars[dikong_idx].push("地空".to_string());

    // 火星、铃星 (按年支三合配时辰)
    // 寅午戌火丑铃卯，申子辰火寅铃戌，巳酉丑火卯铃戌，亥卯未火酉铃戌
    let yz_group = year_zhi_idx % 4;
    let (huo_base, lin_base) = match yz_group {
        0 => (2, 10), // 申子辰 -> 火寅(2), 铃戌(10)
        1 => (3, 10), // 巳酉丑 -> 火卯(3), 铃戌(10)
        2 => (1, 3),  // 寅午戌 -> 火丑(1), 铃卯(3)
        _ => (9, 10), // 亥卯未 -> 火酉(9), 铃戌(10)
    };
    let huoxing_idx = (huo_base + h_idx).rem_euclid(12) as usize;
    let linxing_idx = (lin_base + h_idx).rem_euclid(12) as usize;
    evil_stars[huoxing_idx].push("火星".to_string());
    evil_stars[linxing_idx].push("铃星".to_string());

    // 台辅：午上起子顺数至生时；封诰：寅上起子顺数至生时
    let taifu_idx = (6 + h_idx).rem_euclid(12) as usize;
    let fenggao_idx = (2 + h_idx).rem_euclid(12) as usize;
    assist_stars[taifu_idx].push("台辅".to_string());
    assist_stars[fenggao_idx].push("封诰".to_string());

    // 7. 安月系与年系重要神煞 (天刑, 天姚, 红鸾, 天喜, 孤辰, 寡宿)
    // 天刑：酉起正月顺数；天姚：丑起正月顺数
    let tianxing_idx = (9 + month_offset).rem_euclid(12) as usize;
    let tianyao_idx = (1 + month_offset).rem_euclid(12) as usize;
    other_stars[tianxing_idx].push("天刑".to_string());
    other_stars[tianyao_idx].push("天姚".to_string());

    // 红鸾：卯上起子逆数至年支；天喜：红鸾对冲六位
    let hongluan_idx = (3 - year_zhi_idx as i32).rem_euclid(12) as usize;
    let tianxi_idx = (hongluan_idx + 6) % 12;
    other_stars[hongluan_idx].push("红鸾".to_string());
    other_stars[tianxi_idx].push("天喜".to_string());

    // 孤辰寡宿 (以年支三合局查)
    let (guchen_idx, guasu_idx) = match year_zhi_idx {
        11 | 0 | 1 => (2, 10), // 亥子丑 -> 寅, 戌
        2..=4 => (5, 1),   // 寅卯辰 -> 巳, 丑
        5..=7 => (8, 4),   // 巳午未 -> 申, 辰
        _ => (11, 7),          // 申酉戌 -> 亥, 未
    };
    other_stars[guchen_idx].push("孤辰".to_string());
    other_stars[guasu_idx].push("寡宿".to_string());

    // 生年四化映射
    let sihua_stars = SIHUA_TABLE[year_gan_idx % 10];
    let sihua_types = ["化禄", "化权", "化科", "化忌"];

    // 8. 装配十二宫
    let mut palaces = Vec::with_capacity(12);
    for i in 0..12 {
        let cur_zhi_idx = (ming_idx as i32 + 12 - i as i32).rem_euclid(12) as usize;
        let cur_zhi = ZHI[cur_zhi_idx];

        let gan_step = (cur_zhi_idx as i32 + 10).rem_euclid(12) as usize;
        let palace_gan = GAN[(yin_gan_base + gan_step) % 10].to_string();

        let m_stars = main_stars[cur_zhi_idx].clone();
        let a_stars = assist_stars[cur_zhi_idx].clone();
        let e_stars = evil_stars[cur_zhi_idx].clone();
        let o_stars = other_stars[cur_zhi_idx].clone();

        // 四化检索 (主星或辅星)
        let mut p_sihua = Vec::new();
        let all_stars_in_palace = m_stars.iter().chain(a_stars.iter());
        for s in all_stars_in_palace {
            for (idx, sh_star) in sihua_stars.iter().enumerate() {
                if s == *sh_star {
                    p_sihua.push(format!("{}·{}", s, sihua_types[idx]));
                }
            }
        }

        palaces.push(ZiWeiPalace {
            house_name: HOUSES[i],
            zhi: cur_zhi,
            gan: palace_gan,
            main_stars: m_stars,
            assist_stars: a_stars,
            evil_stars: e_stars,
            other_stars: o_stars,
            sihua: p_sihua,
        });
    }

    // 9. 格局自动识别推理 (Classic Pattern Recognizer)
    let patterns = detect_ziwei_patterns(&palaces, ming_idx);

    // 10. 命主与身主推算
    // 命主以命宫地支查 (子贪狼/丑巨门/寅禄存/卯文曲/辰廉贞/巳武曲/午破军/未武曲/申廉贞/酉文曲/戌禄存/亥巨门)
    let ming_zhu_list = [
        "贪狼", "巨门", "禄存", "文曲", "廉贞", "武曲",
        "破军", "武曲", "廉贞", "文曲", "禄存", "巨门",
    ];
    let ming_zhu = ming_zhu_list[ming_idx % 12];

    // 身主以生年地支查 (子火星/丑天相/寅天梁/卯天同/辰文昌/巳天机/午火星/未天相/申天梁/酉天同/戌文昌/亥天机)
    let shen_zhu_list = [
        "火星", "天相", "天梁", "天同", "文昌", "天机",
        "火星", "天相", "天梁", "天同", "文昌", "天机",
    ];
    let shen_zhu = shen_zhu_list[year_zhi_idx % 12];

    // 11. 大限推算 (从命宫起按局数步进，阳男阴女顺行，阴男阳女逆行)
    let is_yang_year = year_gan_idx.is_multiple_of(2);
    let is_forward = (is_yang_year && _is_male) || (!is_yang_year && !_is_male);
    let mut da_xian = Vec::with_capacity(12);

    for step in 1..=12 {
        let p_idx = if is_forward {
            (12 - (step - 1)) % 12 // HOUSES 数组是按逆时针逆数排列的，地支顺行对应 HOUSES 索引逆转
        } else {
            (step - 1) % 12
        };
        let p = &palaces[p_idx];
        let sa = ju_num + (step as u32 - 1) * 10;
        let ea = sa + 9;
        da_xian.push(DaXianStep {
            step,
            gong_name: p.house_name,
            gong_zhi: p.zhi,
            start_age: sa,
            end_age: ea,
        });
    }

    ZiWeiResult {
        ming_palace_zhi: ming_zhi,
        shen_palace_zhi: shen_zhi,
        ming_zhu,
        shen_zhu,
        wuxing_ju,
        ziwei_pos: ZHI[zw_idx],
        tianfu_pos: ZHI[tf_idx],
        da_xian,
        palaces,
        patterns,
    }
}

/// 识别三方四正与吉凶大格局
fn detect_ziwei_patterns(palaces: &[ZiWeiPalace], _ming_idx: usize) -> Vec<ZiWeiPattern> {
    let mut patterns = Vec::new();

    // 辅助闭包：判断某宫位是否含有某星
    let has_star = |palace_idx: usize, star: &str| -> bool {
        let p = &palaces[palace_idx];
        p.main_stars.iter().any(|s| s == star)
            || p.assist_stars.iter().any(|s| s == star)
            || p.evil_stars.iter().any(|s| s == star)
            || p.other_stars.iter().any(|s| s == star)
    };

    // 三方四正：命宫(0), 财帛宫(4), 官禄宫(8), 迁移宫(6)
    let trine = [0, 4, 8, 6];
    let has_in_trine = |star: &str| -> bool {
        trine.iter().any(|&pi| has_star(pi, star))
    };

    // 1. 紫府同宫格 (安命在寅或申，紫微天府同在命宫)
    if has_star(0, "紫微") && has_star(0, "天府") {
        patterns.push(ZiWeiPattern {
            name: "紫府同宫格",
            category: "富贵格",
            description: "紫微天府同在命宫，终身福禄厚重，文武兼资，极显荣贵。",
        });
    }

    // 2. 君臣庆会格 (紫微入命，三方四正有左辅右弼相会)
    if has_star(0, "紫微") && (has_in_trine("左辅") || has_in_trine("右弼")) {
        patterns.push(ZiWeiPattern {
            name: "君臣庆会格",
            category: "富贵格",
            description: "紫微遇左辅右弼拱照，君明臣贤，百官齐备，多得有力贵人辅佐。",
        });
    }

    // 3. 府相朝垣格 (天府、天相会于命宫三方四正)
    if (has_in_trine("天府") || has_star(0, "天府")) && (has_in_trine("天相") || has_star(0, "天相")) {
        patterns.push(ZiWeiPattern {
            name: "府相朝垣格",
            category: "富贵格",
            description: "天府天相朝会命垣，食禄千锺，官资显达，财禄丰足。",
        });
    }

    // 4. 机月同梁格 (天机、太阴、天同、天梁会于三方四正)
    if has_in_trine("天机") && has_in_trine("太阴") && has_in_trine("天同") && has_in_trine("天梁") {
        patterns.push(ZiWeiPattern {
            name: "机月同梁格",
            category: "贵显格",
            description: "机月同梁作吏人，善于策划筹谋、行政公职、幕僚智囊，一生安稳无灾。",
        });
    }

    // 5. 阳梁昌禄格 (太阳、天梁、文昌、禄存在三方会合)
    if has_in_trine("太阳") && has_in_trine("天梁") && has_in_trine("文昌") && has_in_trine("禄存") {
        patterns.push(ZiWeiPattern {
            name: "阳梁昌禄格",
            category: "科名大格",
            description: "阳梁昌禄胪传第一名，利于科甲考试、学术研究、竞争夺魁，声名远扬。",
        });
    }

    // 6. 日月并明格 (太阳在巳旺地，太阴在酉旺地会合)
    if (has_star(0, "太阳") || has_in_trine("太阳")) && (has_star(0, "太阴") || has_in_trine("太阴")) {
        let p_sun = palaces.iter().position(|p| p.main_stars.contains(&"太阳".to_string())).unwrap_or(99);
        let p_moon = palaces.iter().position(|p| p.main_stars.contains(&"太阴".to_string())).unwrap_or(99);
        if (p_sun < 12 && (palaces[p_sun].zhi == "巳" || palaces[p_sun].zhi == "辰"))
            && (p_moon < 12 && (palaces[p_moon].zhi == "酉" || palaces[p_moon].zhi == "戌")) {
            patterns.push(ZiWeiPattern {
                name: "日月并明格",
                category: "富贵格",
                description: "太阳居巳升殿，太阴入酉呈祥，心地光明磊落，少年登第，一生富贵荣显。",
            });
        }
    }

    // 7. 辅弼夹命格 (左辅、右弼分别夹命宫左右)
    let left_palace = 11; // 父母宫
    let right_palace = 1; // 兄弟宫
    if (has_star(left_palace, "左辅") && has_star(right_palace, "右弼"))
        || (has_star(left_palace, "右弼") && has_star(right_palace, "左辅")) {
        patterns.push(ZiWeiPattern {
            name: "辅弼夹命格",
            category: "贵人辅佐格",
            description: "左辅右弼夹命，多得同僚手足及长辈鼎力相助，行事事半功倍。",
        });
    }

    // 8. 禄马交驰格 (禄存与天马同宫或拱照命宫)
    if (has_star(0, "禄存") && has_star(0, "天马"))
        || (has_in_trine("禄存") && has_in_trine("天马")) {
        patterns.push(ZiWeiPattern {
            name: "禄马交驰格",
            category: "巨富商贾格",
            description: "禄存与天马交会，动中生财，远方发达，越奔波越旺盛。",
        });
    }

    // 9. 羊陀夹命凶格 (命宫无吉而见擎羊陀罗夹命)
    if (has_star(left_palace, "擎羊") && has_star(right_palace, "陀罗"))
        || (has_star(left_palace, "陀罗") && has_star(right_palace, "擎羊")) {
        patterns.push(ZiWeiPattern {
            name: "羊陀夹命格",
            category: "关煞凶格",
            description: "擎羊陀罗相夹命垣，多有阻滞挫折或刑伤破败，宜修心潜伏。",
        });
    }

    // 10. 火铃夹命凶格 (火星铃星夹命)
    if (has_star(left_palace, "火星") && has_star(right_palace, "铃星"))
        || (has_star(left_palace, "铃星") && has_star(right_palace, "火星")) {
        patterns.push(ZiWeiPattern {
            name: "火铃夹命格",
            category: "波折凶格",
            description: "火星铃星相夹命宫，性急如焚，防小人暗中作梗或意外突发波折。",
        });
    }

    // 11. 昌曲夹命格 (文昌文曲夹命宫)
    if (has_star(left_palace, "文昌") && has_star(right_palace, "文曲"))
        || (has_star(left_palace, "文曲") && has_star(right_palace, "文昌")) {
        patterns.push(ZiWeiPattern {
            name: "昌曲夹命格",
            category: "聪明文采格",
            description: "文昌文曲夹命宫，主文章盖世，多才多艺，聪明出众。",
        });
    }

    // 12. 魁钺夹命格 (天魁天钺左右夹命)
    if (has_star(left_palace, "天魁") && has_star(right_palace, "天钺"))
        || (has_star(left_palace, "天钺") && has_star(right_palace, "天魁")) {
        patterns.push(ZiWeiPattern {
            name: "魁钺夹命格",
            category: "贵人辅佐格",
            description: "天魁天钺左右夹命，一生多遇长辈贵人提拔，逢凶化吉。",
        });
    }

    // 13. 杀破狼格 (七杀、破军、贪狼会于命宫及三方四正)
    if has_star(0, "七杀") || has_star(0, "破军") || has_star(0, "贪狼")
        || (has_in_trine("七杀") && has_in_trine("破军") && has_in_trine("贪狼")) {
        patterns.push(ZiWeiPattern {
            name: "杀破狼格",
            category: "变动开创格",
            description: "杀破狼三星拱照命垣，主人生多波折起伏，开创新局，敢作敢当，勇于开拓。",
        });
    }

    // 14. 命无正曜格 (命宫无十四主星)
    if palaces[0].main_stars.is_empty() {
        patterns.push(ZiWeiPattern {
            name: "命无正曜格",
            category: "借宫安星格",
            description: "命宫无正曜十四主星，需借对宫迁移宫主星为用，为人善于随机应变。",
        });
    }

    // 15. 紫府朝垣格 (紫微或天府在三方会合)
    if has_in_trine("紫微") || has_in_trine("天府") {
        patterns.push(ZiWeiPattern {
            name: "紫府朝垣格",
            category: "富贵显达格",
            description: "紫微天府在三方拱照命宫，主为人高雅，具贵气与威望，一生多吉庆。",
        });
    }

    // 16. 魁钺朝拱格 (天魁天钺会于命宫三方四正)
    if (has_in_trine("天魁") || has_star(0, "天魁")) && (has_in_trine("天钺") || has_star(0, "天钺")) {
        patterns.push(ZiWeiPattern {
            name: "魁钺朝拱格",
            category: "贵人朝拱格",
            description: "天魁天钺在三方四正拱照命宫，主一生多逢尊贵长辈扶持，逢凶化吉，一生清贵。",
        });
    }

    // 17. 辅弼朝垣格 (左辅右弼会于命宫三方四正)
    if (has_in_trine("左辅") || has_star(0, "左辅")) && (has_in_trine("右弼") || has_star(0, "右弼")) {
        patterns.push(ZiWeiPattern {
            name: "辅弼朝垣格",
            category: "百官朝拱格",
            description: "左辅右弼在命宫三方四正朝拱，多得手足朋党同仁鼎力拥戴，行事势如破竹。",
        });
    }

    // 18. 昌曲朝拱格 (文昌文曲会于命宫三方四正)
    if (has_in_trine("文昌") || has_star(0, "文昌")) && (has_in_trine("文曲") || has_star(0, "文曲")) {
        patterns.push(ZiWeiPattern {
            name: "昌曲朝拱格",
            category: "翰林文秀格",
            description: "文昌文曲会合三方四正，主才华横溢，文章卓绝，利于科举考学与学术研究。",
        });
    }

    // 19. 正曜乘旺格 (命宫有十四正曜坐守)
    if !palaces[0].main_stars.is_empty() && patterns.is_empty() {
        patterns.push(ZiWeiPattern {
            name: "正曜乘旺格",
            category: "本命正格",
            description: "命宫正曜得所坐守，主性格分明，能立身自强，各展所长。",
        });
    }

    patterns
}
