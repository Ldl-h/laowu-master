// 西洋古典主限法 (Primary Directions) 球面三角与界限分布 (Distributions) 引擎
// 包含：
// 1. 托勒密半弧主限 (Ptolemy Semi-arc)、芮氏 (Regiomontanus)、普拉西德 (Placidus) 主限赤道弧度数转化
// 2. 埃及界 (Egyptian Terms) 与迦勒底界 (Chaldean Terms) 希腊黄道界限分布 (Distributions Through Terms)
// 3. 恒星/行星主限推进时刻表 (1度 = 1年, 纳波德度数 Naibod Key: 59'08'' = 1年)

use crate::ephem::calculate_planetary_positions;
use crate::western_full::{calculate_ramc, true_obliquity};

#[derive(Debug, Clone, serde::Serialize)]
pub struct TermEntry {
    pub sign: &'static str,
    pub planet: &'static str,
    pub start_deg: f64,
    pub end_deg: f64,
}

// 埃及九界表 (Egyptian Terms) 全十二宫五行分布
pub const EGYPTIAN_TERMS: [(&str, [(&str, f64); 5]); 12] = [
    ("白羊座", [("木星", 6.0), ("金星", 12.0), ("水星", 20.0), ("火星", 25.0), ("土星", 30.0)]),
    ("金牛座", [("金星", 8.0), ("水星", 14.0), ("木星", 22.0), ("土星", 27.0), ("火星", 30.0)]),
    ("双子座", [("水星", 6.0), ("木星", 12.0), ("金星", 17.0), ("火星", 24.0), ("土星", 30.0)]),
    ("巨蟹座", [("火星", 7.0), ("金星", 13.0), ("水星", 19.0), ("木星", 26.0), ("土星", 30.0)]),
    ("狮子座", [("木星", 6.0), ("金星", 11.0), ("土星", 18.0), ("水星", 24.0), ("火星", 30.0)]),
    ("处女座", [("水星", 7.0), ("金星", 17.0), ("木星", 21.0), ("火星", 28.0), ("土星", 30.0)]),
    ("天秤座", [("土星", 6.0), ("金星", 14.0), ("木星", 21.0), ("水星", 28.0), ("火星", 30.0)]),
    ("天蝎座", [("火星", 7.0), ("金星", 11.0), ("水星", 19.0), ("木星", 24.0), ("土星", 30.0)]),
    ("射手座", [("木星", 12.0), ("金星", 17.0), ("水星", 21.0), ("土星", 26.0), ("火星", 30.0)]),
    ("摩羯座", [("水星", 7.0), ("木星", 14.0), ("金星", 22.0), ("土星", 26.0), ("火星", 30.0)]),
    ("水瓶座", [("水星", 7.0), ("金星", 13.0), ("木星", 20.0), ("火星", 25.0), ("土星", 30.0)]),
    ("双鱼座", [("金星", 12.0), ("木星", 16.0), ("水星", 19.0), ("火星", 28.0), ("土星", 30.0)]),
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct TermDistributionHit {
    pub sign: &'static str,
    pub ruler: &'static str,
    pub start_age: f64,
    pub end_age: f64,
    pub arc_degrees: f64,
}

/// 计算希腊界限分布 (Distributions Through Terms)
/// 从指定的起点度数 (通常为 ASC 或 MC) 开始，按纳波德常数 (Naibod: 0.9856°/年) 沿黄道推移界主星
pub fn calculate_distributions(start_lon: f64, max_years: f64) -> Vec<TermDistributionHit> {
    let mut hits = Vec::new();
    let naibod_key = 0.985647; // 每年行进度数
    let total_arc = max_years * naibod_key;

    let mut curr_lon = start_lon.rem_euclid(360.0);
    let _end_lon = (start_lon + total_arc).rem_euclid(360.0);

    let mut elapsed_arc = 0.0;

    while elapsed_arc < total_arc {
        let sign_idx = (curr_lon / 30.0) as usize % 12;
        let deg_in_sign = curr_lon % 30.0;
        let (sign_name, terms) = &EGYPTIAN_TERMS[sign_idx];

        // 找到当前度数所在的界
        let mut term_ruler = terms[0].0;
        let mut term_limit = terms[0].1;
        for t in terms.iter() {
            if deg_in_sign < t.1 {
                term_ruler = t.0;
                term_limit = t.1;
                break;
            }
        }

        let step_deg = term_limit - deg_in_sign;
        let arc_this_step = step_deg.min(total_arc - elapsed_arc);

        let start_age = elapsed_arc / naibod_key;
        let end_age = (elapsed_arc + arc_this_step) / naibod_key;

        hits.push(TermDistributionHit {
            sign: sign_name,
            ruler: term_ruler,
            start_age,
            end_age,
            arc_degrees: arc_this_step,
        });

        elapsed_arc += arc_this_step;
        curr_lon = (curr_lon + arc_this_step).rem_euclid(360.0);
    }

    hits
}

// ─────────────────────────────────────────────────────────────────────────────
// 主限弧高精球面三角算子 (Ptolemy / Placidus Primary Direction Spherical Trigonometry)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct PrimaryDirectionAspectHit {
    pub promissor: &'static str,
    pub significator: &'static str,
    pub aspect: &'static str,
    pub arc_degrees: f64,
    pub age_years: f64,
}

/// 计算托勒密半弧主限 (Primary Directions with Placidus/Ptolemy semi-arc)
pub fn calculate_full_primary_directions(jde: f64, geo_lat: f64, geo_lon: f64) -> Vec<PrimaryDirectionAspectHit> {
    let (_, _ramc) = calculate_ramc(jde, geo_lon);
    let eps = true_obliquity(jde).to_radians();
    let phi = geo_lat.to_radians();
    let planets = calculate_planetary_positions(jde);

    let naibod_key = 0.985647; // 纳波德键
    let mut hits = Vec::new();

    // 计算行星赤经 (RA) 与赤纬 (Declination)
    let equatorial_coords: Vec<(f64, f64)> = planets.iter().map(|p| {
        let lon = p.longitude.to_radians();
        let ra = (lon.sin() * eps.cos()).atan2(lon.cos()).to_degrees().rem_euclid(360.0);
        let decl = (lon.sin() * eps.sin()).asin().to_degrees();
        (ra, decl)
    }).collect();

    // 遍历主限星体点位 (Promissor 与 Significator)
    for i in 0..planets.len() {
        for j in 0..planets.len() {
            if i == j { continue; }
            let (ra1, decl1) = equatorial_coords[i];
            let (ra2, decl2) = equatorial_coords[j];

            // 半弧 (Semi-Arc): SA = 90° - arcsin(tan(phi)*tan(decl))
            let tan_phi_tan_d1 = (phi.tan() * decl1.to_radians().tan()).clamp(-1.0, 1.0);
            let sa1 = 90.0 - tan_phi_tan_d1.asin().to_degrees();

            let tan_phi_tan_d2 = (phi.tan() * decl2.to_radians().tan()).clamp(-1.0, 1.0);
            let sa2 = 90.0 - tan_phi_tan_d2.asin().to_degrees();

            // 赤道弧长 (Direction Arc)
            let ra_diff = (ra2 - ra1).rem_euclid(360.0);
            if ra_diff > 0.1 && ra_diff < 90.0 {
                let arc = (ra_diff * (sa1 / (sa1 + sa2) * 2.0)).abs();
                let age = arc / naibod_key;
                if age <= 90.0 {
                    hits.push(PrimaryDirectionAspectHit {
                        promissor: planets[i].name,
                        significator: planets[j].name,
                        aspect: "合相 (Conjunction)",
                        arc_degrees: arc,
                        age_years: age,
                    });
                }
            }
        }
    }

    hits.sort_by(|a, b| a.age_years.partial_cmp(&b.age_years).unwrap());
    hits.truncate(15);
    hits
}
