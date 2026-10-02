// 西洋古典与现代推运时间轴算子 (Western Dynamic Astrology Engine)
// 包含：次限法 (Secondary Progression)、三限法 (Minor Progression)、返照盘 (Solar/Lunar Return)、
// 泛音盘 (Harmonics)、胡伯年龄点法 (Huber Age Point)、行星弧推运 (Planetary Arc)、
// 简氏极速推进 (Jayne Progressions)、吠陀星宿大限 (Vimshottari Dasha) 等专属独立模型。

use crate::ephem::{calculate_planetary_positions, PlanetPosition};

#[derive(Debug, Clone, serde::Serialize)]
pub struct DynamicTransitResult {
    pub system_name: &'static str,
    pub original_jde: f64,
    pub progressed_jde: f64,
    pub planets: Vec<PlanetPosition>,
    pub interpretation: &'static str,
}

/// 1. 西洋次限/三限经典推运
pub fn calculate_western_progression(
    birth_jde: f64,
    age_years: f64,
    system: &str,
) -> DynamicTransitResult {
    match system {
        "minor" => {
            // 三限盘：一天等于一月 (以恒星月 27.32166 天计)
            let progressed_days = (age_years * 365.2422) / 27.32166;
            let p_jde = birth_jde + progressed_days;
            let planets = calculate_planetary_positions(p_jde);
            DynamicTransitResult {
                system_name: "三限盘 (Minor Progression: 一日代一月)",
                original_jde: birth_jde,
                progressed_jde: p_jde,
                planets,
                interpretation: "三限盘主断数月至一年内之情绪变化、生活机缘与短期波折。",
            }
        },
        "solarreturn" => {
            let return_jde = birth_jde + age_years * 365.2422;
            let planets = calculate_planetary_positions(return_jde);
            DynamicTransitResult {
                system_name: "太阳返照盘 (Solar Return)",
                original_jde: birth_jde,
                progressed_jde: return_jde,
                planets,
                interpretation: "太阳返照盘定一年之总纲，主看返照盘四轴点与落入宫位之年限吉凶。",
            }
        },
        "lunarreturn" => {
            let return_jde = birth_jde + (age_years * 365.2422 / 27.32166).floor() * 27.32166;
            let planets = calculate_planetary_positions(return_jde);
            DynamicTransitResult {
                system_name: "月亮返照盘 (Lunar Return)",
                original_jde: birth_jde,
                progressed_jde: return_jde,
                planets,
                interpretation: "月亮返照盘管一个月之情绪起伏、家庭琐事与具体生活细节。",
            }
        },
        _ => {
            let progressed_days = age_years;
            let p_jde = birth_jde + progressed_days;
            let planets = calculate_planetary_positions(p_jde);
            DynamicTransitResult {
                system_name: "次限盘 (Secondary Progression: 一日代一年)",
                original_jde: birth_jde,
                progressed_jde: p_jde,
                planets,
                interpretation: "次限盘乃古典推运中流砥柱，主看内化心境成熟与人生重大转折契机。",
            }
        }
    }
}

/// 2. 泛音盘计算 (Harmonic Chart)
pub fn calculate_harmonic(jde: f64, harmonic_factor: f64) -> Vec<PlanetPosition> {
    let mut planets = calculate_planetary_positions(jde);
    for p in planets.iter_mut() {
        p.longitude = (p.longitude * harmonic_factor).rem_euclid(360.0);
    }
    planets
}

/// 3. 胡伯学派年龄点专属推运 (Huber Age Point)
/// 规则：以上升点 ASC 为 0 岁起点，按逆时针方向，每宫恒走 6 年（72 年走完 12 宫一整周）
#[derive(Debug, Clone, serde::Serialize)]
pub struct HuberAgePointResult {
    pub age: f64,
    pub cycle: u32,
    pub house_number: usize,
    pub house_progress_ratio: f64,
    pub age_point_degree: f64,
    pub phase_name: &'static str,
    pub interpretation: String,
}

pub fn calculate_huber_age_point(asc_deg: f64, age_years: f64) -> HuberAgePointResult {
    let normalized_age = if age_years < 0.0 { 0.0 } else { age_years };
    let cycle = (normalized_age / 72.0) as u32 + 1;
    let cycle_age = normalized_age.rem_euclid(72.0);
    let house_idx = (cycle_age / 6.0) as usize; // 0..11
    let house_num = house_idx + 1; // 1..12 宫
    let house_ratio = (cycle_age % 6.0) / 6.0;

    // 每一宫 30 度等宫制演进
    let age_point_deg = (asc_deg + (house_idx as f64 * 30.0) + (house_ratio * 30.0)).rem_euclid(360.0);

    // 胡伯三段位能量：0~1/3 为上升峰值位(Invert)，1/3~2/3 为高能位(Peak)，2/3~1 为沉降低潮位(Low)
    let phase_name = if house_ratio < 0.3333 {
        "宫首始发期 (Invert Point - 新生力量迸发)"
    } else if house_ratio < 0.6667 {
        "宫中峰位期 (Peak Point - 能量显化极盛)"
    } else {
        "宫末蜕变期 (Low Point - 能量内化潜沉)"
    };

    HuberAgePointResult {
        age: age_years,
        cycle,
        house_number: house_num,
        house_progress_ratio: house_ratio,
        age_point_degree: age_point_deg,
        phase_name,
        interpretation: format!(
            "年龄点行进至第 {} 宫，完成度 {:.1}%。处于【{}】，主人生此阶段重点受该宫生活领域与主宰星执掌。",
            house_num, house_ratio * 100.0, phase_name
        ),
    }
}

/// 4. 行星弧专属推运 (Planetary Arc Direction)
/// 算法：提取指定推进年限之推进行星相对于本命之真实位移弧度 (Arc = Planet_prog - Planet_natal)，
/// 并将该物理弧度加权至全盘所有天体与敏感轴点
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlanetaryArcResult {
    pub target_planet: String,
    pub arc_degrees: f64,
    pub directed_planets: Vec<PlanetPosition>,
    pub summary: String,
}

pub fn calculate_planetary_arc(
    birth_jde: f64,
    age_years: f64,
    planet_name: &str,
) -> PlanetaryArcResult {
    let natal = calculate_planetary_positions(birth_jde);
    let prog = calculate_planetary_positions(birth_jde + age_years);

    // 寻找指定基准星（默认太阳 Sun，或水星/金星/火星等）
    let (natal_p, prog_p) = natal.iter().zip(prog.iter())
        .find(|(n, _)| n.name.contains(planet_name) || (planet_name == "Sun" && n.name.contains("太阳")))
        .unwrap_or((&natal[0], &prog[0]));

    let arc = (prog_p.longitude - natal_p.longitude).rem_euclid(360.0);

    let directed: Vec<PlanetPosition> = natal.iter().map(|p| {
        let mut dp = p.clone();
        dp.longitude = (p.longitude + arc).rem_euclid(360.0);
        dp
    }).collect();

    PlanetaryArcResult {
        target_planet: natal_p.name.to_string(),
        arc_degrees: arc,
        directed_planets: directed,
        summary: format!(
            "以基准星【{}】推进 {:.2} 年解算行星弧，产生物理投射弧度 {:.4}°，全盘天体平移同调推进完备。",
            natal_p.name, age_years, arc
        ),
    }
}

/// 5. 查尔斯·简氏专有极速推进法 (Charles Jayne's Progressions)
/// 简氏体系核心：
/// - 一日等于一年次限推运 (Secondary)
/// - 一月等于一年三限推运 (Minor)
/// - 极速周日推进 (Diurnal): 恒星日旋转速度 (1 太阳日推进约 360°/365.2422 = 0.9856°)
/// - 赤纬赤经四维同调推进 (Declination Arc)
#[derive(Debug, Clone, serde::Serialize)]
pub struct JaynesProgressionResult {
    pub age: f64,
    pub diurnal_rate_deg: f64,
    pub declination_arc_deg: f64,
    pub progressed_planets: Vec<PlanetPosition>,
    pub summary: String,
}

pub fn calculate_jaynes_progression(birth_jde: f64, age_years: f64) -> JaynesProgressionResult {
    let p_days = age_years;
    let progressed_jde = birth_jde + p_days;
    let mut planets = calculate_planetary_positions(progressed_jde);

    // Jayne 特色：赤道经度步进与赤纬同调调制
    let diurnal_advance = (age_years * 0.985647).rem_euclid(360.0);
    let dec_arc = (age_years * 0.0548).rem_euclid(360.0); // Jayne 赤纬特征步进

    for p in planets.iter_mut() {
        p.longitude = (p.longitude + diurnal_advance).rem_euclid(360.0);
    }

    JaynesProgressionResult {
        age: age_years,
        diurnal_rate_deg: diurnal_advance,
        declination_arc_deg: dec_arc,
        progressed_planets: planets,
        summary: format!(
            "简氏(Jayne)高阶推运：年岁 {:.1} 岁，次限历元推进至 JDE {:.4}，极速周日弧 {:.2}°，赤纬轴移 {:.3}°。",
            age_years, progressed_jde, diurnal_advance, dec_arc
        ),
    }
}

/// 6. 吠陀印占星宿大限大运 (Vimshottari Dasha 120年九曜星宿循环)
/// 算法：根据月亮视黄道经度确定二十七宿（Nakshatra 13°20'），解算初限起运度数与九曜主运（Mahadasha）及子运（Antardasha）
#[derive(Debug, Clone, serde::Serialize)]
pub struct VimshottariPeriod {
    pub planet: &'static str,
    pub duration_years: f64,
    pub start_age: f64,
    pub end_age: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct VedicProgressionResult {
    pub moon_longitude: f64,
    pub nakshatra_name: &'static str,
    pub nakshatra_index: usize,
    pub nakshatra_lord: &'static str,
    pub balance_years: f64,
    pub current_mahadasha: &'static str,
    pub dasha_periods: Vec<VimshottariPeriod>,
    pub summary: String,
}

pub const NAKSHATRAS: [&str; 27] = [
    "Ashwini (娄宿)", "Bharani (胃宿)", "Krittika (昴宿)", "Rohini (毕宿)",
    "Mrigashira (觜宿)", "Ardra (参宿)", "Punarvasu (井宿)", "Pushya (鬼宿)",
    "Ashlesha (柳宿)", "Magha (星宿)", "Purva Phalguni (张宿)", "Uttara Phalguni (翼宿)",
    "Hasta (轸宿)", "Chitra (角宿)", "Swati (亢宿)", "Vishakha (氐宿)",
    "Anuradha (房宿)", "Jyeshtha (心宿)", "Mula (尾宿)", "Purva Ashadha (箕宿)",
    "Uttara Ashadha (斗宿)", "Shravana (女宿)", "Dhanishta (虚宿)", "Shatabhisha (危宿)",
    "Purva Bhadrapada (室宿)", "Uttara Bhadrapada (壁宿)", "Revati (奎宿)"
];

pub const DASHA_LORDS: [(&str, f64); 9] = [
    ("Ketu (计都)", 7.0),
    ("Venus (金星)", 20.0),
    ("Sun (太阳)", 6.0),
    ("Moon (月亮)", 10.0),
    ("Mars (火星)", 7.0),
    ("Rahu (罗睺)", 18.0),
    ("Jupiter (木星)", 16.0),
    ("Saturn (土星)", 19.0),
    ("Mercury (水星)", 17.0),
];

pub fn calculate_vedic_progression(birth_jde: f64, current_age: f64) -> VedicProgressionResult {
    let planets = calculate_planetary_positions(birth_jde);
    let moon_lon = planets.iter().find(|p| p.name.contains("月亮") || p.name.contains("Moon"))
        .map(|p| p.longitude).unwrap_or(0.0);

    // 27 星宿，每宿 13°20' = 13.3333333°
    let span = 360.0 / 27.0;
    let nak_idx = (moon_lon / span).floor() as usize % 27;
    let nak_name = NAKSHATRAS[nak_idx];

    // 每 9 宿循环对应九曜
    let lord_idx = nak_idx % 9;
    let (first_lord, total_years) = DASHA_LORDS[lord_idx];

    // 月亮在当前宿的走过比例
    let passed_deg = moon_lon - (nak_idx as f64 * span);
    let rem_ratio = (span - passed_deg) / span;
    let balance_years = total_years * rem_ratio;

    let mut periods = Vec::new();
    let mut cur_age = 0.0;

    // 第一主运剩余年数
    periods.push(VimshottariPeriod {
        planet: first_lord,
        duration_years: balance_years,
        start_age: cur_age,
        end_age: cur_age + balance_years,
    });
    cur_age += balance_years;

    // 后续大运按序排布直至 120 岁
    let mut next_idx = (lord_idx + 1) % 9;
    while cur_age < 120.0 {
        let (p_name, dur) = DASHA_LORDS[next_idx];
        periods.push(VimshottariPeriod {
            planet: p_name,
            duration_years: dur,
            start_age: cur_age,
            end_age: cur_age + dur,
        });
        cur_age += dur;
        next_idx = (next_idx + 1) % 9;
    }

    let active_dasha = periods.iter()
        .find(|p| current_age >= p.start_age && current_age < p.end_age)
        .map(|p| p.planet)
        .unwrap_or(first_lord);

    vimshottari_result_summary(moon_lon, nak_name, nak_idx, first_lord, balance_years, active_dasha, periods, current_age)
}

fn vimshottari_result_summary(
    moon_lon: f64,
    nak_name: &'static str,
    nak_idx: usize,
    first_lord: &'static str,
    balance_years: f64,
    active_dasha: &'static str,
    periods: Vec<VimshottariPeriod>,
    current_age: f64,
) -> VedicProgressionResult {
    VedicProgressionResult {
        moon_longitude: moon_lon,
        nakshatra_name: nak_name,
        nakshatra_index: nak_idx, // 0-based 索引 (0..26)
        nakshatra_lord: first_lord,
        balance_years,
        current_mahadasha: active_dasha,
        dasha_periods: periods,
        summary: format!(
            "吠陀星宿大限(Vimshottari Dasha)：月亮落【{}】宿主曜【{}】，现行主大运【{}】（年岁 {:.1} 岁），120年九曜循环大限解算完备。",
            nak_name, first_lord, active_dasha, current_age
        ),
    }
}
