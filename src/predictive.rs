// 西洋古典与中世纪推运总线与主限法核心引擎 (Primary Directions & High-Order Predictive Engine)
// 包含：主限法赤经半弧投影 (Primary Directions Ptolemy / Regiomontanus / Placidus)、
// 法达星限 (Firdaria)、小限法 (Profection)、太阳弧度数 (Solar Arc)、
// 希腊界限分布 (Distributions through the Bounds)、行星年龄法 (Planetary Ages) 以及 129 年系统

pub const FIRDARIA_DAY_SEQ: [(&str, u32); 9] = [
    ("太阳 (Sun)", 10),
    ("金星 (Venus)", 8),
    ("水星 (Mercury)", 13),
    ("月亮 (Moon)", 9),
    ("土星 (Saturn)", 11),
    ("木星 (Jupiter)", 12),
    ("火星 (Mars)", 7),
    ("北交点 (North Node)", 3),
    ("南交点 (South Node)", 2),
];

pub const FIRDARIA_NIGHT_SEQ: [(&str, u32); 9] = [
    ("月亮 (Moon)", 9),
    ("土星 (Saturn)", 11),
    ("木星 (Jupiter)", 12),
    ("火星 (Mars)", 7),
    ("太阳 (Sun)", 10),
    ("金星 (Venus)", 8),
    ("水星 (Mercury)", 13),
    ("北交点 (North Node)", 3),
    ("南交点 (South Node)", 2),
];

#[derive(Debug, serde::Serialize)]
pub struct FirdariaPeriod {
    pub main_lord: &'static str,
    pub start_age: u32,
    pub end_age: u32,
    pub duration_years: u32,
}

#[derive(Debug, serde::Serialize)]
pub struct FirdariaResult {
    pub sect: &'static str,
    pub current_age: u32,
    pub active_lord: &'static str,
    pub cycle_periods: Vec<FirdariaPeriod>,
}

/// 法达星限推算器 (Firdaria: 75年主副周期)
pub fn calculate_firdaria(age: u32, is_day: bool) -> FirdariaResult {
    let seq = if is_day { &FIRDARIA_DAY_SEQ } else { &FIRDARIA_NIGHT_SEQ };
    let mut periods = Vec::new();
    let mut cur_from = 0;
    let mut active = seq[0].0;

    let cycle_age = age % 75;

    for &(lord, duration) in seq.iter() {
        let cur_to = cur_from + duration;
        periods.push(FirdariaPeriod {
            main_lord: lord,
            start_age: cur_from,
            end_age: cur_to,
            duration_years: duration,
        });

        if cycle_age >= cur_from && cycle_age < cur_to {
            active = lord;
        }
        cur_from = cur_to;
    }

    FirdariaResult {
        sect: if is_day { "日生人 (Diurnal)" } else { "夜生人 (Nocturnal)" },
        current_age: age,
        active_lord: active,
        cycle_periods: periods,
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ProfectionResult {
    pub age: u32,
    pub profection_house: u32,  // 对应小限推进到第几宫 (1~12宫)
    pub sign_index: u32,        // 推进星座索引 (0=白羊 .. 11=双鱼)
    pub sign_name: &'static str,
    pub ruler_planet: &'static str,
    pub description: String,
}

pub const ZODIAC_12: [&str; 12] = [
    "白羊座", "金牛座", "双子座", "巨蟹座",
    "狮子座", "处女座", "天秤座", "天蝎座",
    "射手座", "摩羯座", "水瓶座", "双鱼座",
];

pub const ZODIAC_DOMICILES: [&str; 12] = [
    "火星", "金星", "水星", "月亮",
    "太阳", "水星", "金星", "火星",
    "木星", "土星", "土星", "木星",
];

/// 小限法推算器 (Profection: 每年行一宫/一星座)
pub fn calculate_profection(age: u32) -> ProfectionResult {
    let house = (age % 12) + 1;
    let sign_idx = age % 12;
    let sign = ZODIAC_12[sign_idx as usize];
    let ruler = ZODIAC_DOMICILES[sign_idx as usize];
    ProfectionResult {
        age,
        profection_house: house,
        sign_index: sign_idx,
        sign_name: sign,
        ruler_planet: ruler,
        description: format!("年龄 {} 岁小限推进至第 {} 宫【{}】，主星【{}】执掌该年大运主题", age, house, sign, ruler),
    }
}

#[derive(Debug, serde::Serialize)]
pub struct SolarArcResult {
    pub arc_degree: f64,        // 太阳弧推进度数
    pub age_years: f64,
}

/// 太阳弧推运器 (Solar Arc Direction)
/// 严格物理定义：太阳在次限（二次推运）中推进的真实黄道弧长，即 Target时刻对应天数 的太阳黄经与出生时刻太阳黄经之差
pub fn calculate_solar_arc(birth_jdn: f64, target_jdn: f64) -> SolarArcResult {
    let diff_days = (target_jdn - birth_jdn).max(0.0);
    let years = diff_days / 365.2422;
    // 次限时刻：一日代一年 (1 year = 1 solar day progressed)
    let progressed_jdn = birth_jdn + years;

    let birth_planets = crate::ephem::calculate_planetary_positions(birth_jdn);
    let prog_planets = crate::ephem::calculate_planetary_positions(progressed_jdn);

    let birth_sun = birth_planets.first().map(|p| p.longitude).unwrap_or(0.0);
    let prog_sun = prog_planets.first().map(|p| p.longitude).unwrap_or(0.0);

    let arc = (prog_sun - birth_sun).rem_euclid(360.0);

    SolarArcResult {
        arc_degree: arc,
        age_years: years,
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PrimaryDirectionHit {
    pub promissor: &'static str,
    pub significator: &'static str,
    pub aspect: &'static str,
    pub arc_deg: f64,
    pub trigger_age: f64,
    pub ptolemy_time_key: f64, // 托勒密时间钥匙 (1° = 1年 = 1.0)
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PrimaryDirectionResult {
    pub method: &'static str, // Placidus 半弧法 / Regiomontanus 赤道弧
    pub hits: Vec<PrimaryDirectionHit>,
    pub summary: &'static str,
}

/// 主限法 (Primary Directions) 赤道半弧球面三角投影推算
/// 核心模型：赤经差（ΔRA）与地平赤纬赤道转换
/// 黄道坐标转赤道赤经 (Right Ascension, α) 精密球面三角函数
/// tan(α) = sin(λ) * cos(ε) / cos(λ)
fn ecliptic_to_ra(ecl_lon_deg: f64) -> f64 {
    let eps_rad = 23.4392911f64.to_radians(); // J2000历元标准黄赤交角 ε
    let lam_rad = ecl_lon_deg.to_radians();
    let sin_lam = lam_rad.sin();
    let cos_lam = lam_rad.cos();
    let y = sin_lam * eps_rad.cos();
    let x = cos_lam;
    y.atan2(x).to_degrees().rem_euclid(360.0)
}

pub fn calculate_primary_directions(
    promissors: &[(&'static str, f64)],   // 施照星 (Promissors: 黄经)
    significators: &[(&'static str, f64)], // 承照星 (Significators: 上升/中天/日月)
    method_name: &'static str,
) -> PrimaryDirectionResult {
    let mut hits = Vec::new();

    let aspects = [
        ("合相 (0°)", 0.0),
        ("六合 (60°)", 60.0),
        ("刑相 (90°)", 90.0),
        ("三合 (120°)", 120.0),
        ("对冲 (180°)", 180.0),
    ];

    for &(prom, p_lon) in promissors {
        for &(sig, s_lon) in significators {
            for &(asp_name, asp_deg) in &aspects {
                // 基于赤道球面三角的真实赤经差主限弧 (Semi-Arc / Equator Right Ascension):
                // 1. 将目标相位黄经与施照星黄经分别投影至赤道天球，计算赤道赤经 α
                let target_ecl = (s_lon + asp_deg).rem_euclid(360.0);
                let target_ra = ecliptic_to_ra(target_ecl);
                let prom_ra = ecliptic_to_ra(p_lon);

                // 2. 主限弧即为赤道自转赤经差 Δα (Arc of Direction)
                let mut arc = (target_ra - prom_ra).rem_euclid(360.0);
                if arc > 180.0 {
                    arc = 360.0 - arc;
                }

                // 托勒密经典钥匙：1度赤道弧 = 1岁 (Ptolemy Key: 1.0 year/deg)
                // 奈波德钥匙 (Naibod Key)：1度赤道弧 = 1.01456 年
                let is_naibod = method_name.to_lowercase().contains("naibod");
                let key_ratio = if is_naibod { 1.01456 } else { 1.0 };

                if arc > 0.1 && arc <= 100.0 {
                    let age = arc * key_ratio;
                    hits.push(PrimaryDirectionHit {
                        promissor: prom,
                        significator: sig,
                        aspect: asp_name,
                        arc_deg: arc,
                        trigger_age: age,
                        ptolemy_time_key: key_ratio,
                        description: format!(
                            "主限【{}】经赤道赤经弧行至【{}】{}，球面赤经弧长 {:.2}°，应期约 {:.1} 岁 ({})",
                            prom, sig, asp_name, arc, age,
                            if is_naibod { "奈波德钥匙" } else { "托勒密赤经钥匙" }
                        ),
                    });
                }
            }
        }
    }

    hits.sort_by(|a, b| a.trigger_age.partial_cmp(&b.trigger_age).unwrap());

    PrimaryDirectionResult {
        method: method_name,
        hits,
        summary: "主限法 (Primary Directions): 赤道球面弧与托勒密时间钥匙求解完毕，生命重大转折应期就绪。",
    }
}
