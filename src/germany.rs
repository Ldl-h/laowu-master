// 汉堡占星学派与乌拉尼亚系统核心引擎 (Hamburg / Uranian Astrology & Midpoints)
// 包含：八颗海外假想星体 (TNP 8 Trans-Neptunian Planets)、行星中点树 (Midpoint Trees)、
// 45°/90° 模数轴向敏感点分析与硬相位交点

#[derive(Debug, Clone, serde::Serialize)]
pub struct UranianBody {
    pub name: &'static str,
    pub name_en: &'static str,
    pub longitude: f64,
    pub period_years: f64,
    pub description: &'static str,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MidpointItem {
    pub body_a: &'static str,
    pub body_b: &'static str,
    pub midpoint_lon: f64,
    pub mod_90: f64, // 90° 展开盘刻度
    pub mod_45: f64, // 45° 展开盘刻度
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct UranianResult {
    pub tnps: Vec<UranianBody>,
    pub midpoints: Vec<MidpointItem>,
    pub summary: &'static str,
}

// 汉堡学派 8 颗海王外虚星 (TNP) 经典历元根数与平均日行度数推算 (Epoch J2000 = JDE 2451545.0)
// J2000 参考黄经与平均年运动
pub const TNP_PROFILES: [(&str, &str, f64, f64, f64, &str); 8] = [
    ("丘比特", "Cupido", 250.5, 262.5, 360.0 / 262.5, "主婚姻、家庭、社交圈、艺术团体及合伙关系"),
    ("哈迪斯", "Hades", 89.2, 360.6, 360.0 / 360.6, "主古老、贫困、深层心理、医学、玄秘与沉滞"),
    ("宙斯", "Zeus", 112.4, 455.6, 360.0 / 455.6, "主创造力、野心、目标、领导意志与火能爆发"),
    ("克洛诺斯", "Kronos", 40.8, 521.8, 360.0 / 521.8, "主权威、名誉、政府高官、至尊独立与父亲"),
    ("阿波罗", "Apollon", 14.3, 576.0, 360.0 / 576.0, "主广博、扩张、远见、智慧、财富与倍增"),
    ("阿德墨托斯", "Admetos", 26.1, 624.0, 360.0 / 624.0, "主根基、深度、坚守、障碍、沉思与凝结"),
    ("武尔坎努斯", "Vulcanus", 131.7, 663.0, 360.0 / 663.0, "主巨大力量、命运定势、强悍活力与不可抗力"),
    ("波塞冬", "Poseidon", 208.9, 740.0, 360.0 / 740.0, "主精神、启示、灵性、哲学真理与灵感光芒"),
];

/// 纯数学推算汉堡学派 TNP 虚星与中点轴心系统
pub fn calculate_uranian(jde: f64, natal_planets: &[(&'static str, f64)]) -> UranianResult {
    let t_years = (jde - 2451545.0) / 365.25;

    let mut tnps = Vec::with_capacity(8);
    let mut all_bodies: Vec<(&'static str, f64)> = natal_planets.to_vec();

    for (cn, en, base_lon, period, annual_motion, desc) in TNP_PROFILES {
        let lon = (base_lon + t_years * annual_motion).rem_euclid(360.0);
        tnps.push(UranianBody {
            name: cn,
            name_en: en,
            longitude: lon,
            period_years: period,
            description: desc,
        });
        all_bodies.push((cn, lon));
    }

    // 组合计算所有两两星体的中点轴向坐标 (Midpoint A/B)
    let mut midpoints = Vec::new();
    let n = all_bodies.len();
    for i in 0..n {
        for j in i + 1..n {
            let (name_a, lon_a) = all_bodies[i];
            let (name_b, lon_b) = all_bodies[j];

            // 劣弧中点公式
            let mut diff = (lon_b - lon_a).rem_euclid(360.0);
            if diff > 180.0 {
                diff -= 360.0;
            }
            let mid = (lon_a + diff / 2.0).rem_euclid(360.0);

            midpoints.push(MidpointItem {
                body_a: name_a,
                body_b: name_b,
                midpoint_lon: mid,
                mod_90: mid.rem_euclid(90.0),
                mod_45: mid.rem_euclid(45.0),
            });
        }
    }

    UranianResult {
        tnps,
        midpoints,
        summary: "汉堡学派(Uranian): 海外八星定位完备，中点树与45°/90°模数对称轴系统构建完毕。",
    }
}
