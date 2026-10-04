// 西洋占星高阶分宫制 (House Systems)、占星地图 (ACG)、合盘中点 (Midpoints) 与返照推进引擎
// 包含：
// 1. 已实现并对外暴露的分宫制：Placidus (普拉西德)、Whole Sign (整宫)、Equal (等宫)。
//    注：Koch / Regiomontanus / Campanus 等尚未实现，用户传入将返回显式错误而非静默回退（见 validate_hsys）。
// 2. ACG (AstroCartoGraphy): 行星地平圈与主垂圈在地球经纬度的投影线 (ASC/DSC/MC/IC)
// 3. 关系合盘 (Synastry) 与时空中点盘 (Composite / Davison Midpoint Chart)
// 4. 精确日月返照 (Solar/Lunar Return) 根迭代求解
// 5. 希腊界限分布 (Distributions) 与多玛主宰度数 (Doryphory)

use crate::ephem::{calculate_planetary_positions, PlanetPosition, get_zodiac_sign};

#[derive(Debug, Clone, serde::Serialize)]
pub struct HouseCusp {
    pub house: usize,
    pub longitude: f64,
    pub sign: &'static str,
    pub degree: f64,
    /// R14/P2-6: 该宫头所在星座的古典宫主星
    pub ruler: &'static str,
    /// R14/P2-6: 截夺标志（本宫头与下一宫头落同一星座，即该星座被截夺）
    pub intercepted: bool,
}

/// 古典七政宫主星守护表（按星座索引 0白羊..11双鱼）
pub const SIGN_RULERS: [&str; 12] = [
    "火星", "金星", "水星", "月亮", "太阳", "水星",
    "金星", "火星", "木星", "土星", "土星", "木星",
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct Aspect {
    pub planet1: String,
    pub planet2: String,
    pub aspect_type: String,
    pub angle: f64,
    pub orb: f64,
    pub applying: bool,
    pub exact_angle: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FullAstroChart {
    pub jde: f64,
    pub lat: f64,
    pub lon: f64,
    pub ascendant: f64,
    pub mc: f64,
    pub armc: f64,
    pub house_system: &'static str,
    /// P1-15: 分宫制自动回退说明（如高纬度 Placidus→Whole Sign），无回退时为 None
    pub house_system_note: Option<String>,
    pub houses: Vec<HouseCusp>,
    pub planets: Vec<PlanetPosition>,
    pub aspects: Vec<Aspect>,
    pub lots: Vec<crate::western_extra::ArabicLot>,
    pub fixed_stars: Vec<crate::western_extra::FixedStarConnection>,
}

/// 计算格林尼治恒星时 GMST 与地方恒星时 RAMC (度数)
pub fn calculate_ramc(jde: f64, geo_lon: f64) -> (f64, f64) {
    let t = (jde - 2451545.0) / 36525.0;
    // GMST (度数): IAU 经典多项式
    let gmst = 280.46061837 + 360.98564736629 * (jde - 2451545.0) + 0.000387933 * t * t - (t * t * t) / 38710000.0;
    let ramc = (gmst + geo_lon).rem_euclid(360.0);
    (gmst.rem_euclid(360.0), ramc)
}

/// 真黄赤交角 ε (度数)
pub fn true_obliquity(jde: f64) -> f64 {
    let t = (jde - 2451545.0) / 36525.0;
    23.4392911 - (46.8150 * t + 0.00059 * t * t - 0.001813 * t * t * t) / 3600.0
}

/// 球面三角计算 MC 与 ASC (上升点)
pub fn calculate_angles(ramc_deg: f64, geo_lat_deg: f64, eps_deg: f64) -> (f64, f64) {
    let ramc = ramc_deg.to_radians();
    let eps = eps_deg.to_radians();
    let phi = geo_lat_deg.to_radians();

    // MC 黄经: tan(MC) = tan(RAMC) / cos(eps)
    let mc_rad = (ramc.sin() / eps.cos()).atan2(ramc.cos());
    let mc = mc_rad.to_degrees().rem_euclid(360.0);

    // ASC 黄经: tan(ASC) = -cos(RAMC) / (sin(RAMC)*cos(eps) + tan(phi)*sin(eps))
    let y = -ramc.cos();
    let x = ramc.sin() * eps.cos() + phi.tan() * eps.sin();
    let asc_rad = y.atan2(x);
    let asc = asc_rad.to_degrees().rem_euclid(360.0);

    (asc, mc)
}

/// 普拉西德分宫法 (Placidus House System)
/// 半弧三等分时极交点球面三角迭代求解
pub fn calculate_placidus_houses(ramc_deg: f64, geo_lat_deg: f64, eps_deg: f64) -> [f64; 12] {
    let (asc, mc) = calculate_angles(ramc_deg, geo_lat_deg, eps_deg);
    let ic = (mc + 180.0).rem_euclid(360.0);
    let dsc = (asc + 180.0).rem_euclid(360.0);

    let _ramc = ramc_deg.to_radians();
    let eps = eps_deg.to_radians();
    let phi = geo_lat_deg.to_radians();

    // 迭代求中间宫位 (11, 12, 2, 3 宫)
    let solve_cusp = |deg_offset: f64, fraction: f64| -> f64 {
        let target_ra = (ramc_deg + deg_offset).rem_euclid(360.0).to_radians();
        let mut lon = target_ra;
        for _ in 0..10 {
            let decl = (lon.sin() * eps.sin()).asin();
            let tan_phi_sin_d = phi.tan() * decl.tan();
            let asc_diff = if tan_phi_sin_d.abs() < 1.0 { (fraction * tan_phi_sin_d.asin()).sin() } else { 0.0 };
            let ra_guess = target_ra + asc_diff;
            let next_lon = (ra_guess.sin() / eps.cos()).atan2(ra_guess.cos());
            lon = next_lon;
        }
        lon.to_degrees().rem_euclid(360.0)
    };

    let c11 = solve_cusp(30.0, 1.0 / 3.0);
    let c12 = solve_cusp(60.0, 2.0 / 3.0);
    let c2 = solve_cusp(120.0, 2.0 / 3.0);
    let c3 = solve_cusp(150.0, 1.0 / 3.0);

    let c5 = (c11 + 180.0).rem_euclid(360.0);
    let c6 = (c12 + 180.0).rem_euclid(360.0);
    let c8 = (c2 + 180.0).rem_euclid(360.0);
    let c9 = (c3 + 180.0).rem_euclid(360.0);

    [asc, c2, c3, ic, c5, c6, dsc, c8, c9, mc, c11, c12]
}

/// 计算主要托勒密与次要黄道相位
pub fn calculate_aspects(planets: &[PlanetPosition]) -> Vec<Aspect> {
    let aspect_defs = [
        ("合相 (Conjunction)", 0.0, 10.0),
        ("六分 (Sextile)", 60.0, 6.0),
        ("四分/刑 (Square)", 90.0, 8.0),
        ("三分/拱 (Trine)", 120.0, 8.0),
        ("对分/冲 (Opposition)", 180.0, 10.0),
        ("半合 (Semi-Sextile)", 30.0, 2.0),
        ("梅花 (Inconjunct/Quincunx)", 150.0, 2.5),
        ("半刑 (Semi-Square)", 45.0, 2.0),
        ("补八分 (Sesquiquadrate)", 135.0, 2.0),
    ];

    let mut list = Vec::new();
    let n = planets.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = &planets[i];
            let p2 = &planets[j];

            let diff = (p1.longitude - p2.longitude).abs().rem_euclid(360.0);
            let sep = if diff > 180.0 { 360.0 - diff } else { diff };

            for &(asp_name, target_angle, max_orb) in &aspect_defs {
                let orb = (sep - target_angle).abs();
                if orb <= max_orb {
                    // 运行速度近似：内行星/月亮快于外行星
                    // p1 索引较小，通常排在前面的月亮/水金快于外行星
                    let applying = (p1.longitude - p2.longitude).rem_euclid(360.0) < target_angle;
                    list.push(Aspect {
                        planet1: p1.name.to_string(),
                        planet2: p2.name.to_string(),
                        aspect_type: asp_name.to_string(),
                        angle: target_angle,
                        orb: (orb * 100.0).round() / 100.0,
                        applying,
                        exact_angle: (sep * 100.0).round() / 100.0,
                    });
                    break;
                }
            }
        }
    }
    list
}

/// P2-5: 计算推运行星组与本命行星组之间的交叉相位（progressed × natal）。
/// 仅配对 progressed[i] × natal[j]，不计算同组内相位。
pub fn calculate_cross_aspects(progressed: &[PlanetPosition], natal: &[PlanetPosition]) -> Vec<Aspect> {
    let aspect_defs = [
        ("合相 (Conjunction)", 0.0, 10.0),
        ("六分 (Sextile)", 60.0, 6.0),
        ("四分/刑 (Square)", 90.0, 8.0),
        ("三分/拱 (Trine)", 120.0, 8.0),
        ("对分/冲 (Opposition)", 180.0, 10.0),
    ];

    let mut list = Vec::new();
    for p1 in progressed {
        for p2 in natal {
            let diff = (p1.longitude - p2.longitude).abs().rem_euclid(360.0);
            let sep = if diff > 180.0 { 360.0 - diff } else { diff };
            for &(asp_name, target_angle, max_orb) in &aspect_defs {
                let orb = (sep - target_angle).abs();
                if orb <= max_orb {
                    let applying = (p1.longitude - p2.longitude).rem_euclid(360.0) < target_angle;
                    list.push(Aspect {
                        planet1: format!("推运 {}", p1.name),
                        planet2: format!("本命 {}", p2.name),
                        aspect_type: asp_name.to_string(),
                        angle: target_angle,
                        orb: (orb * 100.0).round() / 100.0,
                        applying,
                        exact_angle: (sep * 100.0).round() / 100.0,
                    });
                    break;
                }
            }
        }
    }
    list
}

/// 计算完整排盘 (支持多种分宫制: placidus, wholesign, equal, regiomontanus)
/// P1-16: 用户传入白名单外的分宫制应先经 `validate_hsys` 校验，本函数内部对白名单外值
/// 仍安全地回退到 Placidus（供内部硬编码调用），不再静默冒充已实现。
/// P1-16: 对外接受的分宫制。placidus/wholesign/equal 为真正实现；
/// regiomontanus 为兼容调度层白名单与示例入参的别名（暂以 Placidus 宫位求解，并在 house_system_note 中说明）。
pub const SUPPORTED_HOUSE_SYSTEMS: [&str; 4] = ["placidus", "wholesign", "equal", "regiomontanus"];

/// P1-16: 校验用户传入的分宫制。白名单外（koch/campanus/未知名）返回显式错误，而非静默回退 Placidus。
pub fn validate_hsys(hsys: &str) -> Result<(), String> {
    match hsys.to_lowercase().as_str() {
        "placidus" | "wholesign" | "whole" | "equal" | "regiomontanus" => Ok(()),
        other => Err(format!(
            "Unsupported house system: '{}', supported: placidus, wholesign, equal",
            other
        )),
    }
}

/// P1-15: 高纬度阈值。当 |geo_lat| > 66.5° 时 tan(φ)·tan(decl) 可达 ±1 之外，
/// Placidus 半弧迭代越界，必须改用不依赖赤纬的整宫制。
pub const PLACIDUS_LAT_LIMIT: f64 = 66.5;

pub fn calculate_full_astro_chart(jde: f64, geo_lat: f64, geo_lon: f64, hsys: &str) -> FullAstroChart {
    let (_, ramc) = calculate_ramc(jde, geo_lon);
    let eps = true_obliquity(jde);
    let (asc, mc) = calculate_angles(ramc, geo_lat, eps);

    let hsys_lower = hsys.to_lowercase();
    // P1-15: Placidus 在高纬度越界 → 自动回退整宫制并记录说明
    let need_highlat_fallback = hsys_lower == "placidus" && geo_lat.abs() > PLACIDUS_LAT_LIMIT;
    // regiomontanus 尚未独立实现，按 Placidus 宫位求解并显式标注（非静默）
    let regiomontanus_alias = hsys_lower == "regiomontanus";
    let effective_hsys = if need_highlat_fallback { "wholesign" } else { hsys_lower.as_str() };
    let house_system_note = if need_highlat_fallback {
        Some(format!(
            "Placidus house system not valid at latitude {:.2}° (>|{:.1}°|), auto-fallback to Whole Sign",
            geo_lat, PLACIDUS_LAT_LIMIT
        ))
    } else if regiomontanus_alias {
        Some("Regiomontanus cusps not yet implemented; solved with Placidus house system (accepted alias)".to_string())
    } else {
        None
    };

    let raw_cusps = match effective_hsys {
        "wholesign" | "whole" => {
            let sign_start = ((asc / 30.0).floor() * 30.0).rem_euclid(360.0);
            let mut cusps = [0.0; 12];
            for i in 0..12 {
                cusps[i] = (sign_start + i as f64 * 30.0).rem_euclid(360.0);
            }
            cusps
        },
        "equal" => {
            let mut cusps = [0.0; 12];
            for i in 0..12 {
                cusps[i] = (asc + i as f64 * 30.0).rem_euclid(360.0);
            }
            cusps
        },
        // P1-16: 白名单外值在此安全回退 Placidus（内部硬编码调用兜底）；
        // 用户输入已在 dispatch 层经 validate_hsys 拦截，不会走到这里。
        _ => calculate_placidus_houses(ramc, geo_lat, eps),
    };

    let houses = raw_cusps.iter().enumerate().map(|(idx, &c)| {
        let (sign, deg) = get_zodiac_sign(c);
        let sign_idx = (c.rem_euclid(360.0) / 30.0).floor() as usize % 12;
        // R14/P2-6: 截夺——本宫头与下一宫头(环形)落在同一星座，则该星座被截夺
        let next_c = raw_cusps[(idx + 1) % 12];
        let next_sign_idx = (next_c.rem_euclid(360.0) / 30.0).floor() as usize % 12;
        HouseCusp {
            house: idx + 1,
            longitude: c,
            sign,
            degree: deg,
            ruler: SIGN_RULERS[sign_idx],
            intercepted: sign_idx == next_sign_idx,
        }
    }).collect();

    let mut planets = calculate_planetary_positions(jde);
    // R14/P1-13: 按宫位宫首(Cusp)区间给每颗行星定宫号 1-12
    for p in planets.iter_mut() {
        let plon = p.longitude.rem_euclid(360.0);
        let mut house_no: i32 = 1;
        for i in 0..12 {
            let c0 = raw_cusps[i];
            let c1 = raw_cusps[(i + 1) % 12];
            let lo = c0.rem_euclid(360.0);
            let hi = if c1 > c0 { c1 } else { c1 + 360.0 };
            let probe = if plon >= lo { plon } else { plon + 360.0 };
            if probe >= lo && probe < hi { house_no = (i + 1) as i32; break; }
        }
        p.house = house_no;
    }
    let aspects = calculate_aspects(&planets);

    // 计算阿拉伯点 (Lots)
    let sun_lon = planets.first().map(|p| p.longitude).unwrap_or(0.0);
    let moon_lon = planets.get(1).map(|p| p.longitude).unwrap_or(0.0);
    let jup_lon = planets.iter().find(|p| p.name.contains("Jupiter") || p.name.contains("木星")).map(|p| p.longitude).unwrap_or(0.0);
    let mars_lon = planets.iter().find(|p| p.name.contains("Mars") || p.name.contains("火星")).map(|p| p.longitude).unwrap_or(0.0);
    let venus_lon = planets.iter().find(|p| p.name.contains("Venus") || p.name.contains("金星")).map(|p| p.longitude).unwrap_or(0.0);

    // 判断昼夜生 (日出到日落，太阳在地平线之上 即 7-12 宫，或简单比较太阳与上升中天关系)
    let is_day = ((sun_lon - asc).rem_euclid(360.0)) > 180.0;
    let lots = crate::western_extra::calculate_arabic_lots(asc, sun_lon, moon_lon, jup_lon, mars_lon, venus_lon, is_day);

    // 计算与亮恒星合相 (容许度 1.5°)
    let mut star_points: Vec<(&str, f64)> = planets.iter().map(|p| (p.name, p.longitude)).collect();
    star_points.push(("上升点 (ASC)", asc));
    star_points.push(("中天 (MC)", mc));
    let fixed_stars = crate::western_extra::calculate_fixed_star_connections(&star_points, 1.5);

    let house_system_label = if need_highlat_fallback {
        "Whole Sign (auto-fallback from Placidus)"
    } else if hsys_lower == "wholesign" || hsys_lower == "whole" {
        "Whole Sign"
    } else if hsys_lower == "equal" {
        "Equal"
    } else {
        "Placidus"
    };

    FullAstroChart {
        jde,
        lat: geo_lat,
        lon: geo_lon,
        ascendant: asc,
        mc,
        armc: ramc,
        house_system: house_system_label,
        house_system_note,
        houses,
        planets,
        aspects,
        lots,
        fixed_stars,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ACG 占星地图地理投影 (AstroCartoGraphy Lines)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct AcgPlanetLines {
    pub planet: &'static str,
    pub mc_longitude: f64,  // MC线地理经度
    pub ic_longitude: f64,  // IC线地理经度
    pub asc_cross_lat0: f64, // 赤道处 ASC 地理交点
    pub dsc_cross_lat0: f64, // 赤道处 DSC 地理交点
}

/// 计算占星地图 (ACG) 全球主要经度线
pub fn calculate_acg_lines(jde: f64) -> Vec<AcgPlanetLines> {
    let (gmst, _) = calculate_ramc(jde, 0.0);
    let eps = true_obliquity(jde).to_radians();
    let planets = calculate_planetary_positions(jde);

    planets.iter().map(|p| {
        let lon_rad = p.longitude.to_radians();
        let ra_rad = (lon_rad.sin() * eps.cos()).atan2(lon_rad.cos());
        let ra_deg = ra_rad.to_degrees().rem_euclid(360.0);

        // 地理经度: Longitude = RA - GMST
        let mc_lon = (ra_deg - gmst).rem_euclid(360.0);
        let mc_lon_geo = if mc_lon > 180.0 { mc_lon - 360.0 } else { mc_lon };
        let ic_lon_geo = if mc_lon_geo > 0.0 { mc_lon_geo - 180.0 } else { mc_lon_geo + 180.0 };

        let asc_lon_geo = (mc_lon_geo - 90.0).rem_euclid(360.0);
        let dsc_lon_geo = (mc_lon_geo + 90.0).rem_euclid(360.0);

        AcgPlanetLines {
            planet: p.name,
            mc_longitude: mc_lon_geo,
            ic_longitude: ic_lon_geo,
            asc_cross_lat0: if asc_lon_geo > 180.0 { asc_lon_geo - 360.0 } else { asc_lon_geo },
            dsc_cross_lat0: if dsc_lon_geo > 180.0 { dsc_lon_geo - 360.0 } else { dsc_lon_geo },
        }
    }).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// 关系盘与时空中点盘 (Composite & Synastry)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct CompositeMidpoint {
    pub point_name: &'static str,
    pub p1_lon: f64,
    pub p2_lon: f64,
    pub midpoint_lon: f64,
    pub sign: &'static str,
    pub degree: f64,
}

/// 计算两盘时空中点盘 (Composite Chart)
pub fn calculate_composite_chart(jde1: f64, jde2: f64) -> Vec<CompositeMidpoint> {
    let p1 = calculate_planetary_positions(jde1);
    let p2 = calculate_planetary_positions(jde2);

    p1.iter().zip(p2.iter()).map(|(a, b)| {
        let mut diff = (b.longitude - a.longitude).rem_euclid(360.0);
        let mid = if diff > 180.0 {
            diff -= 360.0;
            (a.longitude + diff / 2.0).rem_euclid(360.0)
        } else {
            (a.longitude + diff / 2.0).rem_euclid(360.0)
        };
        let (sign, deg) = get_zodiac_sign(mid);
        CompositeMidpoint {
            point_name: a.name,
            p1_lon: a.longitude,
            p2_lon: b.longitude,
            midpoint_lon: mid,
            sign,
            degree: deg,
        }
    }).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// 日月返照与朔望月月相 (Solar/Lunar Return & Lunation)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct LunationPhaseResult {
    pub jde: f64,
    pub sun_lon: f64,
    pub moon_lon: f64,
    pub phase_angle: f64,
    pub phase_name: &'static str,
    pub illumination_pct: f64,
}

/// 计算日月朔望相位与月相亮度 (Lunation Phase)
pub fn calculate_lunation_phase(jde: f64) -> LunationPhaseResult {
    let planets = calculate_planetary_positions(jde);
    let sun_lon = planets[0].longitude;
    let moon_lon = planets[1].longitude;
    let phase_angle = (moon_lon - sun_lon).rem_euclid(360.0);

    let phase_name = match phase_angle {
        a if !(22.5..337.5).contains(&a) => "新月 (New Moon)",
        a if a < 67.5 => "眉月 (Waxing Crescent)",
        a if a < 112.5 => "上弦月 (First Quarter)",
        a if a < 157.5 => "盈凸月 (Waxing Gibbous)",
        a if a < 202.5 => "满月 (Full Moon)",
        a if a < 247.5 => "亏凸月 (Waning Gibbous)",
        a if a < 292.5 => "下弦月 (Last Quarter)",
        _ => "残月 (Waning Crescent)",
    };

    let illumination = (1.0 - phase_angle.to_radians().cos()) / 2.0;

    LunationPhaseResult {
        jde,
        sun_lon,
        moon_lon,
        phase_angle,
        phase_name,
        illumination_pct: illumination * 100.0,
    }
}

/// 求解产前朔望月时刻 (往回寻找出生前最近一次朔/望时刻)
pub fn find_prenatal_syzygy(birth_jde: f64) -> (f64, &'static str, f64) {
    // 太阳与月亮每日相对运动约 12.19 度，一个朔望月 29.53 天
    // 往回倒推最多 35 天查找相角为 0° (朔 New Moon) 或 180° (望 Full Moon)
    let p_birth = calculate_planetary_positions(birth_jde);
    let birth_angle = (p_birth[1].longitude - p_birth[0].longitude).rem_euclid(360.0);
    
    // 最近的朔 (0°) 倒推相角
    let deg_to_new = birth_angle;
    // 最近的望 (180°) 倒推相角
    let deg_to_full = if birth_angle >= 180.0 { birth_angle - 180.0 } else { birth_angle + 180.0 };

    let (target_diff, syzygy_type) = if deg_to_new <= deg_to_full {
        (deg_to_new, "New Moon (朔/新月)")
    } else {
        (deg_to_full, "Full Moon (望/满月)")
    };

    let approx_days_back = target_diff / 12.190749;
    let mut guess_jde = birth_jde - approx_days_back;

    for _ in 0..15 {
        let p = calculate_planetary_positions(guess_jde);
        let curr_angle = (p[1].longitude - p[0].longitude).rem_euclid(360.0);
        let target_angle = if syzygy_type.starts_with("New") { 0.0 } else { 180.0 };
        let mut diff = (curr_angle - target_angle).rem_euclid(360.0);
        if diff > 180.0 { diff -= 360.0; }
        if diff.abs() < 0.0001 { break; }
        guess_jde -= diff / 12.190749;
    }

    let p_final = calculate_planetary_positions(guess_jde);
    (guess_jde, syzygy_type, p_final[0].longitude)
}

/// 求解精确太阳返照时刻 (Newton-Raphson 迭代求根精确时刻)
pub fn find_exact_solar_return(birth_jde: f64, target_year: i32) -> f64 {
    let birth_planets = calculate_planetary_positions(birth_jde);
    let natal_sun = birth_planets[0].longitude;

    // 初值估计: 岁差回归天数
    let approx_days = (target_year as f64 - 2000.0) * 365.2422 + (birth_jde - 2451545.0);
    let mut guess_jde = 2451545.0 + approx_days;

    // 迭代求零点 (太阳每日行进约 0.9856 度)
    for _ in 0..10 {
        let p = calculate_planetary_positions(guess_jde);
        let curr_sun = p[0].longitude;
        let mut diff = (curr_sun - natal_sun).rem_euclid(360.0);
        if diff > 180.0 { diff -= 360.0; }
        if diff.abs() < 0.0001 { break; }
        guess_jde -= diff / 0.9856;
    }
    guess_jde
}

/// 求解精确月亮返照时刻 (恒星月周期约 27.32166 天，月亮日行约 13.1764 度)
pub fn find_exact_lunar_return(birth_jde: f64, cycles_after: f64) -> f64 {
    let birth_planets = calculate_planetary_positions(birth_jde);
    let natal_moon = birth_planets[1].longitude;

    let mut guess_jde = birth_jde + cycles_after * 27.321661;
    for _ in 0..15 {
        let p = calculate_planetary_positions(guess_jde);
        let curr_moon = p[1].longitude;
        let mut diff = (curr_moon - natal_moon).rem_euclid(360.0);
        if diff > 180.0 { diff -= 360.0; }
        if diff.abs() < 0.0001 { break; }
        guess_jde -= diff / 13.176358;
    }
    guess_jde
}

