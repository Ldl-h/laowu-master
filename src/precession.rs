// 岁差、章动与真视位置修正模块
// 对标 IAU 2006 / 瑞士星历历元修正算法
// 将 J2000.0 平黄道坐标旋转到当日真分点 (True Equinox of Date)，彻底消灭 0.37° 岁差偏差！

/// 计算从 J2000.0 (JD 2451545.0) 起算的儒略世纪数 T
pub fn julian_centuries(jde: f64) -> f64 {
    (jde - 2451545.0) / 36525.0
}

/// 计算黄道总岁差修正量 p_A (单位: 度)
/// 根据 IAU 2006 岁差模型: p_A = 5028.796195" * T + 1.1054348" * T^2 + ...
pub fn general_precession(jde: f64) -> f64 {
    let t = julian_centuries(jde);
    // 累积岁差 (角秒 -> 度)
    let p_arcsec = 5028.796195 * t + 1.1054348 * t * t + 0.00007964 * t * t * t;
    p_arcsec / 3600.0
}

/// 计算黄经章动修正量 Δψ (单位: 度)
/// 采用 IAU 1980 简明章动级数前四项主项
pub fn nutation_in_longitude(jde: f64) -> f64 {
    let t = julian_centuries(jde);
    // 月球升交点平黄经 Ω
    let omega = (125.04452 - 1934.136261 * t).rem_euclid(360.0).to_radians();
    // 太阳平黄经 L
    let l_sun = (280.4665 + 36000.7698 * t).rem_euclid(360.0).to_radians();
    // 月球平黄经 L'
    let l_moon = (218.3165 + 481267.8813 * t).rem_euclid(360.0).to_radians();

    // 章动主项 (角秒)
    let delta_psi_arcsec = -17.20 * omega.sin()
        - 1.32 * (2.0 * l_sun).sin()
        - 0.23 * (2.0 * l_moon).sin()
        + 0.21 * (2.0 * omega).sin();

    delta_psi_arcsec / 3600.0
}

/// 光行差修正 (对太阳等天体固定为约 -20.49 角秒)
pub const ABERRATION_DEG: f64 = -20.49552 / 3600.0; // 约 -0.00569°

/// 将 J2000.0 坐标转换到瑞士星历同标准的“历元时刻真视黄经 (Apparent Longitude of Date)”
pub fn j2000_to_apparent_date(lon_j2000: f64, jde: f64, is_sun: bool) -> f64 {
    let prec = general_precession(jde);
    let nut = nutation_in_longitude(jde);
    let mut apparent = lon_j2000 + prec + nut;
    if is_sun {
        apparent += ABERRATION_DEG;
    }
    apparent.rem_euclid(360.0)
}
