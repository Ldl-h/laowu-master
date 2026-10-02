// 巴尔比卢斯（Balbillus 129 年古典推运系统）核心推算引擎
// 包含：七星小年、擢升度角距削减、主限星旋转、多层级递归子限划分

#[derive(Debug, Clone, serde::Serialize)]
pub struct BalbillusPeriod {
    pub level: usize,
    pub planet: &'static str,
    pub start_age: f64,
    pub end_age: f64,
    pub duration_years: f64,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BalbillusResult {
    pub total_cycle_years: f64,
    pub start_planet: &'static str,
    pub mode: &'static str,
    pub main_periods: Vec<BalbillusPeriod>,
}

pub const SEVEN_PLANETS: [&str; 7] = ["太阳", "月亮", "土星", "木星", "火星", "金星", "水星"];

// 七星 Balbillus 小年 (Σ = 129)
pub const BALBILLUS_YEARS: [(&str, f64); 7] = [
    ("太阳", 19.0),
    ("月亮", 25.0),
    ("土星", 30.0),
    ("木星", 12.0),
    ("火星", 15.0),
    ("金星", 8.0),
    ("水星", 20.0),
];

// 七星传统擢升度 (0~360°绝对黄经)
// 日 白羊19°(19) / 月 金牛3°(33) / 土 天秤21°(201) / 木 巨蟹15°(105) / 火 摩羯28°(298) / 金 双鱼27°(357) / 水 处女15°(165)
pub const BALBILLUS_EXALT: [(&str, f64); 7] = [
    ("太阳", 19.0),
    ("月亮", 33.0),
    ("土星", 201.0),
    ("木星", 105.0),
    ("火星", 298.0),
    ("金星", 357.0),
    ("水星", 165.0),
];

fn norm360(deg: f64) -> f64 {
    deg.rem_euclid(360.0)
}

/// 计算本命星黄经离其擢升度的角距
pub fn balbillus_distance(lon: f64, exalt: f64, is_forward: bool) -> f64 {
    let raw = norm360(lon - exalt);
    if is_forward {
        raw
    } else {
        raw.min(360.0 - raw)
    }
}

/// 纯数学推算 Balbillus 推运主限期
pub fn calculate_balbillus(
    planet_longitudes: &[(&str, f64)],
    start_planet: &str,
    is_forward_mode: bool,
    max_age: f64,
) -> BalbillusResult {
    let mut main_periods = Vec::new();

    // 旋转星曜顺序，使 start_planet 处于首位
    let start_idx = SEVEN_PLANETS.iter().position(|&p| p == start_planet).unwrap_or(0);
    let mut rotated_planets = Vec::new();
    for i in 0..7 {
        rotated_planets.push(SEVEN_PLANETS[(start_idx + i) % 7]);
    }

    let mut current_age = 0.0;
    for p_name in rotated_planets {
        if current_age >= max_age {
            break;
        }

        let base_years = BALBILLUS_YEARS.iter().find(|(name, _)| *name == p_name).map(|(_, y)| *y).unwrap_or(15.0);
        let exalt_deg = BALBILLUS_EXALT.iter().find(|(name, _)| *name == p_name).map(|(_, d)| *d).unwrap_or(0.0);
        let natal_lon = planet_longitudes.iter().find(|(name, _)| *name == p_name).map(|(_, l)| *l).unwrap_or(exalt_deg);

        // 削减公式：duration = base_years * (1.0 - d / 360.0)
        let d = balbillus_distance(natal_lon, exalt_deg, is_forward_mode);
        let factor = (1.0 - (d / 360.0)).clamp(0.1, 1.0);
        let duration = base_years * factor;

        let end_age = (current_age + duration).min(max_age);
        main_periods.push(BalbillusPeriod {
            level: 1,
            planet: p_name,
            start_age: current_age,
            end_age,
            duration_years: duration,
            description: format!("主限 {} 掌限：原始小年 {:.1} 年，经旺距削减系数 {:.3} 折算后为 {:.2} 年", p_name, base_years, factor, duration),
        });

        current_age = end_age;
    }

    BalbillusResult {
        total_cycle_years: 129.0,
        start_planet: if SEVEN_PLANETS.contains(&start_planet) {
            SEVEN_PLANETS[start_idx]
        } else {
            "太阳"
        },
        mode: if is_forward_mode { "顺黄道距 (forward)" } else { "最近角距 (nearest)" },
        main_periods,
    }
}
