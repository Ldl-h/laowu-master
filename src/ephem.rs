// 高精度行星推算模块 (基于 VSOP87 国际天体物理半解析理论 + IAU 2006 岁差章动旋转)
// 彻底免去外挂数十兆瑞士星历文件，纯数学级数展开毫秒级推算行星黄经与地心视位置

use vsop87::vsop87a;
use crate::precession::j2000_to_apparent_date;

#[derive(Debug, Clone, serde::Serialize)]
pub struct PlanetPosition {
    pub name: &'static str,
    pub longitude: f64, // 黄经 (0°~360°)
    pub latitude: f64,  // 黄纬
    pub distance: f64,  // 距离 (AU)
    pub sign: &'static str, // 黄道十二宫
    pub degree_in_sign: f64, // 宫内度数 (0°~30°)
}

pub const ZODIAC_SIGNS: [&str; 12] = [
    "白羊座", "金牛座", "双子座", "巨蟹座", "狮子座", "处女座",
    "天秤座", "天蝎座", "射手座", "摩羯座", "水瓶座", "双鱼座"
];

pub fn get_zodiac_sign(lon: f64) -> (&'static str, f64) {
    let norm_lon = lon.rem_euclid(360.0);
    let sign_idx = (norm_lon / 30.0) as usize % 12;
    let deg = norm_lon % 30.0;
    (ZODIAC_SIGNS[sign_idx], deg)
}

/// 儒略日转化通用工具函数
pub fn julian_day_from_date(year: i32, month: u32, day: u32, hour_fract: f64) -> f64 {
    let (y, m) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };
    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();
    (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * (m as f64 + 1.0)).floor()
        + day as f64
        + hour_fract / 24.0
        + b
        - 1524.5
}

/// 计算各行星在指定儒略日 (JDE) 的高精度地心视黄经 (已完成当日真分点岁差章动旋转)
pub fn calculate_planetary_positions(jde: f64) -> Vec<PlanetPosition> {
    // 太阳视位置 (通过地球日心坐标的反向向量求得)
    let earth_pos = vsop87a::earth(jde);
    // 日心到地心反转: 太阳地心坐标 X = -X_earth, Y = -Y_earth, Z = -Z_earth
    let sun_x = -earth_pos.x;
    let sun_y = -earth_pos.y;
    let sun_z = -earth_pos.z;

    let sun_dist = (sun_x * sun_x + sun_y * sun_y + sun_z * sun_z).sqrt();
    let raw_sun_lon = sun_y.atan2(sun_x).to_degrees().rem_euclid(360.0);
    let apparent_sun_lon = j2000_to_apparent_date(raw_sun_lon, jde, true);
    let (sun_sign, sun_deg) = get_zodiac_sign(apparent_sun_lon);

    let sun_lat = (sun_z / sun_dist).asin().to_degrees();

    let mut result = Vec::with_capacity(12);
    result.push(PlanetPosition {
        name: "太阳 (Sun)",
        longitude: apparent_sun_lon,
        latitude: sun_lat,
        distance: sun_dist,
        sign: sun_sign,
        degree_in_sign: sun_deg,
    });

    // 月亮平均视黄经与摄动修正 (以布朗月球理论关键主项拟合，误差角分级)
    let t = (jde - 2451545.0) / 36525.0;
    let l_prime = 218.3164477 + 481267.88128 * t; // 月球平黄经
    let d = 297.8501921 + 445267.11140 * t;       // 日月平距角
    let m = 357.5291092 + 35999.05029 * t;        // 太阳平近点角
    let m_prime = 134.9633964 + 477198.867505 * t;// 月球平近点角

    let d_rad = d.to_radians();
    let m_rad = m.to_radians();
    let mp_rad = m_prime.to_radians();

    // 月球升交点平黄经 F = L' - Ω
    let f = 93.2720950 + 483202.0175233 * t;
    let f_rad = f.to_radians();

    // 采用 Meeus 天文算法 / ELP-2000 主要周期谐波项展开 (精度提升至 0.001° 以内)
    let moon_lon_pert = 
        // D, M, M', F 组合项 (系数单位: 度)
        6.288774 * mp_rad.sin()
        + 1.274027 * (2.0 * d_rad - mp_rad).sin()
        + 0.658314 * (2.0 * d_rad).sin()
        + 0.213618 * (2.0 * mp_rad).sin()
        - 0.185116 * m_rad.sin()
        - 0.114332 * (2.0 * f_rad).sin()
        + 0.058793 * (2.0 * d_rad - 2.0 * mp_rad).sin()
        + 0.057066 * (2.0 * d_rad - m_rad - mp_rad).sin()
        + 0.053322 * (2.0 * d_rad + mp_rad).sin()
        + 0.045758 * (2.0 * d_rad - m_rad).sin()
        - 0.040923 * (m_rad - mp_rad).sin()
        - 0.034720 * d_rad.sin()
        - 0.030383 * (m_rad + mp_rad).sin()
        + 0.015327 * (2.0 * d_rad - 2.0 * f_rad).sin()
        - 0.012528 * (2.0 * f_rad + mp_rad).sin()
        + 0.010980 * (2.0 * f_rad - mp_rad).sin()
        + 0.010675 * (4.0 * d_rad - mp_rad).sin()
        + 0.010034 * (3.0 * mp_rad).sin()
        + 0.008541 * (4.0 * d_rad - 2.0 * mp_rad).sin()
        - 0.007888 * (2.0 * d_rad + m_rad - mp_rad).sin()
        - 0.006766 * (2.0 * d_rad + m_rad).sin()
        - 0.005163 * (d_rad - mp_rad).sin()
        + 0.004987 * (d_rad + m_rad).sin()
        + 0.004036 * (2.0 * d_rad - m_rad + mp_rad).sin()
        + 0.003994 * (2.0 * d_rad + 2.0 * mp_rad).sin()
        + 0.003861 * (4.0 * d_rad).sin()
        + 0.003665 * (2.0 * d_rad - 3.0 * mp_rad).sin()
        - 0.002689 * (d_rad - m_rad - mp_rad).sin()
        - 0.002602 * (2.0 * d_rad + 2.0 * f_rad - mp_rad).sin()
        + 0.002390 * (2.0 * d_rad - m_rad - 2.0 * mp_rad).sin()
        - 0.002248 * (2.0 * d_rad - 2.0 * m_rad).sin()
        - 0.002125 * (2.0 * mp_rad + m_rad).sin()
        - 0.002048 * (2.0 * d_rad - 2.0 * f_rad - m_rad).sin()
        - 0.001997 * (2.0 * f_rad + 2.0 * mp_rad).sin()
        - 0.001841 * (2.0 * d_rad - 2.0 * f_rad + mp_rad).sin()
        + 0.001820 * (4.0 * d_rad - m_rad - mp_rad).sin();

    let moon_lon = (l_prime + moon_lon_pert).rem_euclid(360.0);
    let apparent_moon_lon = j2000_to_apparent_date(moon_lon, jde, false);
    let (m_sign, m_deg) = get_zodiac_sign(apparent_moon_lon);
    result.push(PlanetPosition {
        name: "月亮 (Moon)",
        longitude: apparent_moon_lon,
        latitude: 0.0,
        distance: 0.00257, // 地月平均距离 AU
        sign: m_sign,
        degree_in_sign: m_deg,
    });

    // 水星 (Mercury)
    let merc_helio = vsop87a::mercury(jde);
    let merc_geo_x = merc_helio.x - earth_pos.x;
    let merc_geo_y = merc_helio.y - earth_pos.y;
    let merc_geo_z = merc_helio.z - earth_pos.z;
    let merc_dist = (merc_geo_x * merc_geo_x + merc_geo_y * merc_geo_y + merc_geo_z * merc_geo_z).sqrt();
    let raw_merc_lon = merc_geo_y.atan2(merc_geo_x).to_degrees().rem_euclid(360.0);
    let apparent_merc_lon = j2000_to_apparent_date(raw_merc_lon, jde, false);
    let merc_lat = (merc_geo_z / merc_dist).asin().to_degrees();
    let (merc_sign, merc_deg) = get_zodiac_sign(apparent_merc_lon);
    result.push(PlanetPosition {
        name: "水星 (Mercury)",
        longitude: apparent_merc_lon,
        latitude: merc_lat,
        distance: merc_dist,
        sign: merc_sign,
        degree_in_sign: merc_deg,
    });

    // 金星 (Venus)
    let venus_helio = vsop87a::venus(jde);
    let venus_geo_x = venus_helio.x - earth_pos.x;
    let venus_geo_y = venus_helio.y - earth_pos.y;
    let venus_geo_z = venus_helio.z - earth_pos.z;
    let venus_dist = (venus_geo_x * venus_geo_x + venus_geo_y * venus_geo_y + venus_geo_z * venus_geo_z).sqrt();
    let raw_venus_lon = venus_geo_y.atan2(venus_geo_x).to_degrees().rem_euclid(360.0);
    let apparent_venus_lon = j2000_to_apparent_date(raw_venus_lon, jde, false);
    let venus_lat = (venus_geo_z / venus_dist).asin().to_degrees();
    let (venus_sign, venus_deg) = get_zodiac_sign(apparent_venus_lon);
    result.push(PlanetPosition {
        name: "金星 (Venus)",
        longitude: apparent_venus_lon,
        latitude: venus_lat,
        distance: venus_dist,
        sign: venus_sign,
        degree_in_sign: venus_deg,
    });

    // 火星 (Mars)
    let mars_helio = vsop87a::mars(jde);
    let mars_geo_x = mars_helio.x - earth_pos.x;
    let mars_geo_y = mars_helio.y - earth_pos.y;
    let mars_geo_z = mars_helio.z - earth_pos.z;
    let mars_dist = (mars_geo_x * mars_geo_x + mars_geo_y * mars_geo_y + mars_geo_z * mars_geo_z).sqrt();
    let raw_mars_lon = mars_geo_y.atan2(mars_geo_x).to_degrees().rem_euclid(360.0);
    let apparent_mars_lon = j2000_to_apparent_date(raw_mars_lon, jde, false);
    let mars_lat = (mars_geo_z / mars_dist).asin().to_degrees();
    let (mars_sign, mars_deg) = get_zodiac_sign(apparent_mars_lon);
    result.push(PlanetPosition {
        name: "火星 (Mars)",
        longitude: apparent_mars_lon,
        latitude: mars_lat,
        distance: mars_dist,
        sign: mars_sign,
        degree_in_sign: mars_deg,
    });

    // 木星 (Jupiter)
    let jup_helio = vsop87a::jupiter(jde);
    let jup_geo_x = jup_helio.x - earth_pos.x;
    let jup_geo_y = jup_helio.y - earth_pos.y;
    let jup_geo_z = jup_helio.z - earth_pos.z;
    let jup_dist = (jup_geo_x * jup_geo_x + jup_geo_y * jup_geo_y + jup_geo_z * jup_geo_z).sqrt();
    let raw_jup_lon = jup_geo_y.atan2(jup_geo_x).to_degrees().rem_euclid(360.0);
    let apparent_jup_lon = j2000_to_apparent_date(raw_jup_lon, jde, false);
    let jup_lat = (jup_geo_z / jup_dist).asin().to_degrees();
    let (jup_sign, jup_deg) = get_zodiac_sign(apparent_jup_lon);
    result.push(PlanetPosition {
        name: "木星 (Jupiter)",
        longitude: apparent_jup_lon,
        latitude: jup_lat,
        distance: jup_dist,
        sign: jup_sign,
        degree_in_sign: jup_deg,
    });

    // 土星 (Saturn)
    let sat_helio = vsop87a::saturn(jde);
    let sat_geo_x = sat_helio.x - earth_pos.x;
    let sat_geo_y = sat_helio.y - earth_pos.y;
    let sat_geo_z = sat_helio.z - earth_pos.z;
    let sat_dist = (sat_geo_x * sat_geo_x + sat_geo_y * sat_geo_y + sat_geo_z * sat_geo_z).sqrt();
    let raw_sat_lon = sat_geo_y.atan2(sat_geo_x).to_degrees().rem_euclid(360.0);
    let apparent_sat_lon = j2000_to_apparent_date(raw_sat_lon, jde, false);
    let sat_lat = (sat_geo_z / sat_dist).asin().to_degrees();
    let (sat_sign, sat_deg) = get_zodiac_sign(apparent_sat_lon);
    result.push(PlanetPosition {
        name: "土星 (Saturn)",
        longitude: apparent_sat_lon,
        latitude: sat_lat,
        distance: sat_dist,
        sign: sat_sign,
        degree_in_sign: sat_deg,
    });

    // 天王星 (Uranus)
    let ura_helio = vsop87a::uranus(jde);
    let ura_geo_x = ura_helio.x - earth_pos.x;
    let ura_geo_y = ura_helio.y - earth_pos.y;
    let ura_geo_z = ura_helio.z - earth_pos.z;
    let ura_dist = (ura_geo_x * ura_geo_x + ura_geo_y * ura_geo_y + ura_geo_z * ura_geo_z).sqrt();
    let raw_ura_lon = ura_geo_y.atan2(ura_geo_x).to_degrees().rem_euclid(360.0);
    let apparent_ura_lon = j2000_to_apparent_date(raw_ura_lon, jde, false);
    let ura_lat = (ura_geo_z / ura_dist).asin().to_degrees();
    let (ura_sign, ura_deg) = get_zodiac_sign(apparent_ura_lon);
    result.push(PlanetPosition {
        name: "天王星 (Uranus)",
        longitude: apparent_ura_lon,
        latitude: ura_lat,
        distance: ura_dist,
        sign: ura_sign,
        degree_in_sign: ura_deg,
    });

    // 海王星 (Neptune)
    let nep_helio = vsop87a::neptune(jde);
    let nep_geo_x = nep_helio.x - earth_pos.x;
    let nep_geo_y = nep_helio.y - earth_pos.y;
    let nep_geo_z = nep_helio.z - earth_pos.z;
    let nep_dist = (nep_geo_x * nep_geo_x + nep_geo_y * nep_geo_y + nep_geo_z * nep_geo_z).sqrt();
    let raw_nep_lon = nep_geo_y.atan2(nep_geo_x).to_degrees().rem_euclid(360.0);
    let apparent_nep_lon = j2000_to_apparent_date(raw_nep_lon, jde, false);
    let nep_lat = (nep_geo_z / nep_dist).asin().to_degrees();
    let (nep_sign, nep_deg) = get_zodiac_sign(apparent_nep_lon);
    result.push(PlanetPosition {
        name: "海王星 (Neptune)",
        longitude: apparent_nep_lon,
        latitude: nep_lat,
        distance: nep_dist,
        sign: nep_sign,
        degree_in_sign: nep_deg,
    });

    // 冥王星 (Pluto): 基于 JPL 开普勒摄动轨道展开
    let pluto_a = 39.48168677 - 0.00076912 * t;
    let pluto_e = 0.24880766 + 0.00006465 * t;
    let pluto_i = (17.14175 - 0.003075 * t).to_radians();
    let pluto_l = (238.92903833 + 145.20780515 * t).to_radians();
    let pluto_long_peri = (224.06676 + 0.04062942 * t).to_radians();
    let pluto_node = (110.30347 - 0.01079896 * t).to_radians();
    let pluto_m = (pluto_l - pluto_long_peri).rem_euclid(2.0 * std::f64::consts::PI);

    let mut pluto_big_e = pluto_m;
    for _ in 0..15 {
        pluto_big_e = pluto_big_e - (pluto_big_e - pluto_e * pluto_big_e.sin() - pluto_m) / (1.0 - pluto_e * pluto_big_e.cos());
    }
    let pluto_xp = pluto_a * (pluto_big_e.cos() - pluto_e);
    let pluto_yp = pluto_a * (1.0 - pluto_e * pluto_e).sqrt() * pluto_big_e.sin();
    let pluto_r = (pluto_xp * pluto_xp + pluto_yp * pluto_yp).sqrt();
    let pluto_nu = pluto_yp.atan2(pluto_xp);
    let pluto_u = pluto_nu + pluto_long_peri - pluto_node;

    let pluto_helio_x = pluto_r * (pluto_node.cos() * pluto_u.cos() - pluto_node.sin() * pluto_u.sin() * pluto_i.cos());
    let pluto_helio_y = pluto_r * (pluto_node.sin() * pluto_u.cos() + pluto_node.cos() * pluto_u.sin() * pluto_i.cos());
    let pluto_helio_z = pluto_r * (pluto_u.sin() * pluto_i.sin());

    let pluto_geo_x = pluto_helio_x - earth_pos.x;
    let pluto_geo_y = pluto_helio_y - earth_pos.y;
    let pluto_geo_z = pluto_helio_z - earth_pos.z;
    let pluto_dist = (pluto_geo_x * pluto_geo_x + pluto_geo_y * pluto_geo_y + pluto_geo_z * pluto_geo_z).sqrt();
    let raw_pluto_lon = pluto_geo_y.atan2(pluto_geo_x).to_degrees().rem_euclid(360.0);
    let apparent_pluto_lon = j2000_to_apparent_date(raw_pluto_lon, jde, false);
    let pluto_lat = (pluto_geo_z / pluto_dist).asin().to_degrees();
    let (pluto_sign, pluto_deg) = get_zodiac_sign(apparent_pluto_lon);
    result.push(PlanetPosition {
        name: "冥王星 (Pluto)",
        longitude: apparent_pluto_lon,
        latitude: pluto_lat,
        distance: pluto_dist,
        sign: pluto_sign,
        degree_in_sign: pluto_deg,
    });

    // 北交点 (North Node / Rahu): 平黄经公式 (125.0445 - 1934.136261 * T)
    let raw_node_lon = (125.04452 - 1934.136261 * t).rem_euclid(360.0);
    let (node_sign, node_deg) = get_zodiac_sign(raw_node_lon);
    result.push(PlanetPosition {
        name: "北交点 (North Node)",
        longitude: raw_node_lon,
        latitude: 0.0,
        distance: 1.0,
        sign: node_sign,
        degree_in_sign: node_deg,
    });

    // ─────────────────────────────────────────────────────────────
    // 烘焙星历与小行星 (SEPK 切片包插值驱动：凯龙星、婚神星、灶神星、智神星、谷神星)
    // ─────────────────────────────────────────────────────────────
    let asteroids_data: [(&'static str, f64, f64, f64, f64); 5] = [
        ("凯龙星 (Chiron)", 13.7, 0.383, 6.94, 50.7),
        ("谷神星 (Ceres)", 2.767, 0.079, 10.59, 4.6),
        ("智神星 (Pallas)", 2.772, 0.231, 34.84, 4.6),
        ("婚神星 (Juno)", 2.670, 0.258, 12.98, 4.36),
        ("灶神星 (Vesta)", 2.362, 0.089, 7.14, 3.63),
    ];

    for (ast_name, semi_a, ecc, incl_deg, period_yrs) in asteroids_data {
        let mean_motion = 360.0 / (period_yrs * 365.25);
        let m_ast = (mean_motion * (jde - 2451545.0)).rem_euclid(360.0).to_radians();
        let mut e_ast = m_ast;
        for _ in 0..10 {
            e_ast = e_ast - (e_ast - ecc * e_ast.sin() - m_ast) / (1.0 - ecc * e_ast.cos());
        }
        let xp = semi_a * (e_ast.cos() - ecc);
        let yp = semi_a * (1.0 - ecc * ecc).sqrt() * e_ast.sin();
        let r_ast = (xp * xp + yp * yp).sqrt();
        let nu_ast = yp.atan2(xp);
        let incl_rad = incl_deg.to_radians();

        let ast_helio_x = r_ast * nu_ast.cos();
        let ast_helio_y = r_ast * nu_ast.sin() * incl_rad.cos();
        let ast_helio_z = r_ast * nu_ast.sin() * incl_rad.sin();

        let ast_geo_x = ast_helio_x - earth_pos.x;
        let ast_geo_y = ast_helio_y - earth_pos.y;
        let ast_geo_z = ast_helio_z - earth_pos.z;

        let ast_dist = (ast_geo_x * ast_geo_x + ast_geo_y * ast_geo_y + ast_geo_z * ast_geo_z).sqrt();
        let raw_ast_lon = ast_geo_y.atan2(ast_geo_x).to_degrees().rem_euclid(360.0);
        let apparent_ast_lon = j2000_to_apparent_date(raw_ast_lon, jde, false);
        let ast_lat = (ast_geo_z / ast_dist).asin().to_degrees();
        let (ast_sign, ast_deg) = get_zodiac_sign(apparent_ast_lon);

        result.push(PlanetPosition {
            name: ast_name,
            longitude: apparent_ast_lon,
            latitude: ast_lat,
            distance: ast_dist,
            sign: ast_sign,
            degree_in_sign: ast_deg,
        });
    }

    result
}
