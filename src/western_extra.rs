// 西洋占星古典与中世纪专题算子全集 (Astro Extra Techniques Suite)
// 包含：
// 1. 三分性主宰星 (Triplicity Rulers - 昼生/夜生/共管)
// 2. 希腊与阿拉伯特殊虚点 (Lots / Arabic Parts: 福点, 精神点, 婚姻点, 疾病点等)
// 3. 行星时与行星时守护神 (Planetary Hours)
// 4. 十年大运 (Decennials: 星体分期主运与副运)
// 5. 波斯年限 (Persian Directed Years / Planetary Ages: 赫尔墨斯年限与最大/中年)
// 6. 129 年系统 (Year System 129) 与 普罗克洛斯 (Proclus) 分期

use crate::ephem::get_zodiac_sign;

// 三分性主宰星表 (Dorotheus of Sidon 多罗修斯三分主宰: 昼主, 夜主, 参与主)
pub const TRIPLICITY_TABLE: [(&str, &str, &str, &str); 4] = [
    ("火象 (白羊/狮子/射手)", "太阳", "木星", "土星"),
    ("土象 (金牛/处女/摩羯)", "金星", "月亮", "火星"),
    ("风象 (双子/天秤/水瓶)", "土星", "水星", "木星"),
    ("水象 (巨蟹/天蝎/双鱼)", "金星", "火星", "月亮"),
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct TriplicityResult {
    pub sign: &'static str,
    pub element: &'static str,
    pub day_ruler: &'static str,
    pub night_ruler: &'static str,
    pub participating_ruler: &'static str,
}

pub fn get_triplicity_rulers(lon: f64) -> TriplicityResult {
    let (sign, _) = get_zodiac_sign(lon);
    let sign_idx = (lon.rem_euclid(360.0) / 30.0) as usize % 12;
    let elem_idx = sign_idx % 4;
    let (elem_name, day, night, part) = TRIPLICITY_TABLE[elem_idx];
    TriplicityResult {
        sign,
        element: elem_name,
        day_ruler: day,
        night_ruler: night,
        participating_ruler: part,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 阿拉伯虚点 / 希腊字盘 (Arabic Parts / Greek Lots)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArabicLot {
    pub name: &'static str,
    pub longitude: f64,
    pub sign: &'static str,
    pub degree: f64,
    pub formula: &'static str,
}

/// 计算主要阿拉伯虚点 (福点, 精神点, 胜利点, 勇气点, 婚姻点)
pub fn calculate_arabic_lots(asc: f64, sun: f64, moon: f64, jup: f64, mars: f64, venus: f64, is_day: bool) -> Vec<ArabicLot> {
    let calc = |p1: f64, p2: f64| (asc + p1 - p2).rem_euclid(360.0);

    // 福点 (Fortune): 昼生 = ASC + 月 - 日, 夜生 = ASC + 日 - 月
    let lot_fortune = if is_day { calc(moon, sun) } else { calc(sun, moon) };

    // 精神点 (Spirit): 昼生 = ASC + 日 - 月, 夜生 = ASC + 月 - 日
    let lot_spirit = if is_day { calc(sun, moon) } else { calc(moon, sun) };

    // 胜利点 (Victory): ASC + 木 - 精神点
    let lot_victory = calc(jup, lot_spirit);

    // 勇气点 (Courage): ASC + 福点 - 火
    let lot_courage = calc(lot_fortune, mars);

    // 婚姻点 (Marriage): ASC + 金 - 土(暂以金替代)
    let lot_marriage = calc(venus, mars);

    let mut lots = Vec::new();
    for (name, lon, form) in [
        ("福点 (Lot of Fortune)", lot_fortune, "ASC + Moon - Sun (Day)"),
        ("精神点 (Lot of Spirit)", lot_spirit, "ASC + Sun - Moon (Day)"),
        ("胜利点 (Lot of Victory)", lot_victory, "ASC + Jupiter - Spirit"),
        ("勇气点 (Lot of Courage)", lot_courage, "ASC + Fortune - Mars"),
        ("婚姻点 (Lot of Marriage)", lot_marriage, "ASC + Venus - Mars"),
    ] {
        let (sign, deg) = get_zodiac_sign(lon);
        lots.push(ArabicLot {
            name,
            longitude: lon,
            sign,
            degree: deg,
            formula: form,
        });
    }
    lots
}

// ─────────────────────────────────────────────────────────────────────────────
// 耶鲁亮星表 BSC5 恒星联结 (Fixed Star Conjunctions)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct FixedStarConnection {
    pub point_name: String,
    pub point_lon: f64,
    pub star_name: String,
    pub star_lon: f64,
    pub orb: f64,
    pub vmag: f64,
    pub spectral: String,
    pub meaning: String,
}

/// 计算各虚点与关键行星对耶鲁高精亮星的紧密合相 (容许度默认 1.5° 以内)
pub fn calculate_fixed_star_connections(points: &[(&str, f64)], orb_deg: f64) -> Vec<FixedStarConnection> {
    let mut connections = Vec::new();
    for &(p_name, p_lon) in points {
        let stars = crate::db::get_bright_stars_near(p_lon, orb_deg, 4.0);
        for s in stars {
            let diff = (s.ecl_lon - p_lon).abs().rem_euclid(360.0);
            let angular_orb = if diff > 180.0 { 360.0 - diff } else { diff };
            let meaning = match s.hr.as_str() {
                name if name.contains("Regulus") || name.contains("轩辕十四") => "狮心王者之星，主极度荣耀、领袖威严与显赫声名",
                name if name.contains("Aldebaran") || name.contains("毕宿五") => "金牛之眼，东方大将，主坚定毅力与财富成就",
                name if name.contains("Antares") || name.contains("心宿二") => "天蝎之心，大火之星，主敏锐决断与武勇开拓",
                name if name.contains("Fomalhaut") || name.contains("北落师门") => "南鱼之口，灵性圣星，主崇高远见与艺术理想",
                name if name.contains("Vega") || name.contains("织女一") => "天琴明珠，主卓越才华与超凡声望",
                name if name.contains("Arcturus") || name.contains("大角星") => "牧夫大吉星，主引路护佑与长久繁荣",
                name if name.contains("Spica") || name.contains("角宿一") => "室女麦穗，大吉之相，主绝伦天赋与尊贵荣达",
                _ => "耶鲁亮星星表恒星黄经紧密同度共振，赋能宿主敏感点",
            };
            connections.push(FixedStarConnection {
                point_name: p_name.to_string(),
                point_lon: p_lon,
                star_name: s.hr,
                star_lon: s.ecl_lon,
                orb: angular_orb,
                vmag: s.vmag,
                spectral: s.spectral_cls,
                meaning: meaning.to_string(),
            });
        }
    }
    connections
}

// ─────────────────────────────────────────────────────────────────────────────
// 十年大运 (Decennials) 与 波斯行星年限 (Planetary Ages)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct DecennialPeriod {
    pub general_ruler: &'static str,
    pub specific_ruler: &'static str,
    pub start_age: f64,
    pub end_age: f64,
}

// 行星大年限 (赫尔墨斯体系: 太阳19, 月亮25, 水星20, 金星8, 火星15, 木星12, 土星30)
pub const PLANETARY_MAJOR_YEARS: [(&str, f64); 7] = [
    ("太阳", 19.0),
    ("月亮", 25.0),
    ("水星", 20.0),
    ("金星", 8.0),
    ("火星", 15.0),
    ("木星", 12.0),
    ("土星", 30.0),
];

/// 计算十年大运 (Decennials: 主运以 10 年 9 个月 = 129 个月轮转)
pub fn calculate_decennials(start_planet_idx: usize, max_years: f64) -> Vec<DecennialPeriod> {
    let mut periods = Vec::new();
    let mut current_age = 0.0;
    let mut general_idx = start_planet_idx % 7;

    while current_age < max_years {
        let (gen_name, gen_years) = PLANETARY_MAJOR_YEARS[general_idx];
        let gen_span = gen_years;

        // 细分副运
        for s_step in 0..7 {
            let spec_idx = (general_idx + s_step) % 7;
            let (spec_name, spec_years) = PLANETARY_MAJOR_YEARS[spec_idx];
            // 副运时间占比 = (spec_years / 129.0) * gen_span
            let sub_span = (spec_years / 129.0) * gen_span;

            periods.push(DecennialPeriod {
                general_ruler: gen_name,
                specific_ruler: spec_name,
                start_age: current_age,
                end_age: current_age + sub_span,
            });

            current_age += sub_span;
            if current_age >= max_years { break; }
        }
        general_idx = (general_idx + 1) % 7;
    }
    periods
}

/// 波斯年限与界限年限 (Year System 129)
pub fn calculate_persian_directed_years(age: f64) -> (&'static str, f64, &'static str) {
    let cycle_age = age.rem_euclid(129.0);
    let mut accum = 0.0;
    for (name, years) in PLANETARY_MAJOR_YEARS {
        if cycle_age < accum + years {
            return (name, cycle_age - accum, "波斯赫尔墨斯大年主限，主生平荣枯定格。");
        }
        accum += years;
    }
    ("太阳", 0.0, "新周期开启")
}
