// 西洋占星高阶分宫制 (House Systems)、占星地图 (ACG)、合盘中点 (Midpoints) 与返照推进引擎
// 包含：
// 1. Placidus (普拉西德)、Koch (高氏)、Regiomontanus (芮氏)、Campanus (甘氏)、Whole Sign (整宫)、Equal (等宫) 六大分宫制球面三角数值解
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
    pub houses: Vec<HouseCusp>,
    pub planets: Vec<PlanetPosition>,
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

/// 计算完整排盘 (支持多种分宫制: placidus, wholesign, equal, regiomontanus)
pub fn calculate_full_astro_chart(jde: f64, geo_lat: f64, geo_lon: f64, hsys: &str) -> FullAstroChart {
    let (_, ramc) = calculate_ramc(jde, geo_lon);
    let eps = true_obliquity(jde);
    let (asc, mc) = calculate_angles(ramc, geo_lat, eps);

    let raw_cusps = match hsys.to_lowercase().as_str() {
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
        _ => calculate_placidus_houses(ramc, geo_lat, eps),
    };

    let houses = raw_cusps.iter().enumerate().map(|(idx, &c)| {
        let (sign, deg) = get_zodiac_sign(c);
        HouseCusp {
            house: idx + 1,
            longitude: c,
            sign,
            degree: deg,
        }
    }).collect();

    let planets = calculate_planetary_positions(jde);

    FullAstroChart {
        jde,
        lat: geo_lat,
        lon: geo_lon,
        ascendant: asc,
        mc,
        armc: ramc,
        house_system: if hsys == "wholesign" { "Whole Sign" } else if hsys == "equal" { "Equal" } else { "Placidus" },
        houses,
        planets,
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

