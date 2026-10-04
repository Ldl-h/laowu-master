use crate::ephem::calculate_planetary_positions;

// -------------------------------------------------------------
// 印度吠陀占星 (Vedic/Jyotish) Lahiri 分盘与二十七星宿
// -------------------------------------------------------------
#[derive(Debug, Clone, serde::Serialize)]
pub struct VedicPlanet {
    pub name: &'static str,
    pub tropical_lon: f64,
    pub sidereal_lon: f64,
    pub d1_sign: &'static str,
    pub d1_degree: f64,
    pub d9_sign: &'static str,
    pub nakshatra: &'static str,
    pub pada: u8,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DashaPeriod {
    pub lord: &'static str,
    pub start_year: f64,
    pub end_year: f64,
    pub duration: f64,
}

pub const NAKSHATRAS: [&str; 27] = [
    "Ashwini", "Bharani", "Krittika", "Rohini", "Mrigashira", "Ardra",
    "Punarvasu", "Pushya", "Ashlesha", "Magha", "Purva Phalguni", "Uttara Phalguni",
    "Hasta", "Chitra", "Swaati", "Vishakha", "Anuradha", "Jyeshtha",
    "Mula", "Purva Ashadha", "Uttara Ashadha", "Shravana", "Dhanishta", "Shatabhisha",
    "Purva Bhadrapada", "Uttara Bhadrapada", "Revati"
];

pub const VEDIC_SIGNS: [&str; 12] = [
    "Mesha (白羊)", "Vrishabha (金牛)", "Mithuna (双子)", "Karka (巨蟹)",
    "Simha (狮子)", "Kanya (室女)", "Tula (天秤)", "Vrishchika (天蝎)",
    "Dhanu (人马)", "Makara (摩羯)", "Kumbha (宝瓶)", "Meena (双鱼)"
];

// Vimshottari 120年九曜主运周期 (年)
pub const VIMSHOTTARI_YEARS: [(&str, f64); 9] = [
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

#[derive(Debug, Clone, serde::Serialize)]
pub struct VedicChartResult {
    pub ayanamsa: f64,
    pub ascendant_sign: &'static str,
    pub planets: Vec<VedicPlanet>,
    pub vimshottari_dasha: Vec<DashaPeriod>,
    pub summary: &'static str,
}

pub fn calculate_vedic(jde: f64, asc_tropical: f64) -> VedicChartResult {
    // 1. Lahiri Ayanamsa (Chitra Paksha) 精密多项式计算 (对齐 IAU 2006 岁差模型与瑞士星历):
    // J2000.0 历元 Lahiri 岁差基准值 A0 = 23°51'25.53" = 23.8570917°
    // 加上考虑二阶/三阶非线性加速效应的累积岁差 (IAU 2006 general precession)
    let t_centuries = (jde - 2451545.0) / 36525.0;
    let prec_deg = (5028.796195 * t_centuries + 1.1054348 * t_centuries * t_centuries + 0.00007964 * t_centuries * t_centuries * t_centuries) / 3600.0;
    let ayanamsa = (23.8570917 + prec_deg).rem_euclid(360.0);

    // 2. 本命行星回归黄道坐标
    let trop_planets = calculate_planetary_positions(jde);
    let mut vedic_planets = Vec::with_capacity(trop_planets.len());

    let mut moon_sidereal = 0.0;

    for p in &trop_planets {
        let sid_lon = (p.longitude - ayanamsa).rem_euclid(360.0);
        if p.name.contains("月亮") || p.name.contains("Moon") {
            moon_sidereal = sid_lon;
        }

        // D1 (Rashi) 宫位: 30度一宫
        let s_idx = (sid_lon / 30.0).floor() as usize % 12;
        let s_deg = sid_lon % 30.0;

        // D9 (Navamsha) 分盘: 3度20分一分盘 (3.3333333度)
        let nav_idx = ((sid_lon / (30.0 / 9.0)).floor() as usize) % 12;

        // 27月宿 Nakshatra: 13度20分 (13.3333333度)
        let nak_idx = ((sid_lon / (360.0 / 27.0)).floor() as usize) % 27;
        let pada = (((sid_lon % (360.0 / 27.0)) / (360.0 / 108.0)).floor() as u8) + 1;

        vedic_planets.push(VedicPlanet {
            name: p.name,
            tropical_lon: p.longitude,
            sidereal_lon: sid_lon,
            d1_sign: VEDIC_SIGNS[s_idx],
            d1_degree: s_deg,
            d9_sign: VEDIC_SIGNS[nav_idx],
            nakshatra: NAKSHATRAS[nak_idx],
            pada,
        });
    }

    // 3. 计算上升恒星黄道
    let asc_sid = (asc_tropical - ayanamsa).rem_euclid(360.0);
    let asc_s_idx = (asc_sid / 30.0).floor() as usize % 12;

    // 4. Vimshottari Dasha 起运流年
    let nak_len = 360.0 / 27.0;
    let moon_nak_idx = ((moon_sidereal / nak_len).floor() as usize) % 27;
    let passed_in_nak = moon_sidereal % nak_len;
    let remain_ratio = 1.0 - (passed_in_nak / nak_len);

    let first_lord_idx = moon_nak_idx % 9;
    let (first_lord, total_y) = VIMSHOTTARI_YEARS[first_lord_idx];
    let first_remain = total_y * remain_ratio;

    let mut dasha_periods = Vec::new();
    let mut cur_y = 0.0;
    dasha_periods.push(DashaPeriod {
        lord: first_lord,
        start_year: cur_y,
        end_year: cur_y + first_remain,
        duration: first_remain,
    });
    cur_y += first_remain;

    for i in 1..9 {
        let (lord, dur) = VIMSHOTTARI_YEARS[(first_lord_idx + i) % 9];
        dasha_periods.push(DashaPeriod {
            lord,
            start_year: cur_y,
            end_year: cur_y + dur,
            duration: dur,
        });
        cur_y += dur;
    }

    VedicChartResult {
        ayanamsa,
        ascendant_sign: VEDIC_SIGNS[asc_s_idx % 12],
        planets: vedic_planets,
        vimshottari_dasha: dasha_periods,
        summary: "印度占星 (Jyotish/Vedic): Lahiri 恒星盘 D1/D9 分盘、二十七月宿、以及 Vimshottari Dasha 120 年大运推演完成。",
    }
}

// -------------------------------------------------------------
// 飞宫小奇门 (Fei Gong)
// -------------------------------------------------------------
pub const FEIGONG_STARS: [&str; 9] = ["贪狼", "巨门", "禄存", "文曲", "廉贞", "武曲", "破军", "左辅", "右弼"];

pub const FG_GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const FG_ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
pub const FG_ZHI_GONG: [usize; 12] = [1, 8, 8, 3, 4, 4, 9, 2, 2, 7, 6, 6];
pub const FG_GAN_SLOTS: [usize; 10] = [1, 2, 3, 4, 0, 6, 7, 8, 9, 0]; // 0 表示中宫
pub const FG_SHU_XU: [usize; 8] = [1, 2, 3, 4, 6, 7, 8, 9];
pub const FG_FANG_WEI_RING: [usize; 8] = [1, 8, 3, 4, 9, 2, 7, 6];
pub const FG_YUAN_SHEN: [&str; 12] = ["青龙", "明堂", "天刑", "朱雀", "金匮", "天德", "白虎", "玉堂", "天牢", "玄武", "司命", "勾陈"];
pub const FG_TIAN_XING: [&str; 12] = ["建", "除", "满", "平", "定", "执", "破", "危", "成", "收", "开", "闭"];
pub const FG_BA_MEN: [&str; 8] = ["休", "生", "伤", "杜", "景", "死", "惊", "开"];

#[derive(Debug, Clone, serde::Serialize)]
pub struct FeiGongResult {
    pub qi_zhi: String,
    pub jian_zhi: String,
    pub long_gong: usize,
    pub day_gan: Option<String>,
    pub day_zhi: Option<String>,
    pub zhong_gong: Vec<String>,
    pub palace_stars: Vec<(&'static str, &'static str)>, // (宫位名, 落入星曜)
    pub gan_gong: std::collections::BTreeMap<String, usize>, // 天干 -> 宫号 (0为中宫)
    pub men_gong: std::collections::BTreeMap<String, usize>, // 八门 -> 宫号
    pub snapshot_text: String,
    pub summary: &'static str,
}

pub fn calculate_feigong(month: u32, day: u32, hour: u32) -> FeiGongResult {
    calculate_feigong_full(month, day, hour, None, None, None)
}

#[allow(dead_code)] // P2-13: 带节气支的飞宫入口，当前无外部调用
pub fn calculate_feigong_with_qizhi(month: u32, day: u32, hour: u32, qi_zhi: Option<&str>) -> FeiGongResult {
    calculate_feigong_full(month, day, hour, qi_zhi, None, None)
}

pub fn calculate_feigong_full(
    _month: u32,
    _day: u32,
    hour: u32,
    qi_zhi: Option<&str>,
    day_gan: Option<&str>,
    day_zhi: Option<&str>,
) -> FeiGongResult {
    let qz_str = if let Some(qz) = qi_zhi {
        qz
    } else {
        // 根据时辰换算地支
        let z_idx = (hour.div_ceil(2) % 12) as usize;
        FG_ZHI[z_idx]
    };

    let zhi_idx = FG_ZHI.iter().position(|&z| z == qz_str).unwrap_or(7); // 缺省未时(7)
    let qi_zhi_val = FG_ZHI[zhi_idx];

    // 1. 青龙宫 (ZHI_GONG)
    let long_gong = FG_ZHI_GONG[zhi_idx];

    // 2. 建星支: (2 * s + 7) % 12 || 12, s 为 1-based 支序
    let s = zhi_idx + 1;
    let b = (2 * s + 7) % 12;
    let b = if b == 0 { 12 } else { b };
    let jian_zhi_val = FG_ZHI[b - 1];

    // 3. 甲乘龙飞九宫 (GAN_SLOTS)
    // 找到 long_gong 在 GAN_SLOTS 中的第一个索引
    let i0_gan = FG_GAN_SLOTS.iter().position(|&slot| slot == long_gong).unwrap_or(1);
    let mut gan_gong_map = std::collections::BTreeMap::new();
    let mut zhong_gong_vec = Vec::new();
    for k in 0..10 {
        let slot = FG_GAN_SLOTS[(i0_gan + k) % 10];
        let gan_name = FG_GAN[k].to_string();
        gan_gong_map.insert(gan_name.clone(), slot);
        if slot == 0 {
            zhong_gong_vec.push(gan_name);
        }
    }

    // 4. 龙头开八门 (休门宫依九宫数序青龙下一宫)
    let idx_shu = FG_SHU_XU.iter().position(|&g| g == long_gong).unwrap_or(1);
    let xiu_men_gong = FG_SHU_XU[(idx_shu + 1) % 8];
    let idx_ring = FG_FANG_WEI_RING.iter().position(|&g| g == xiu_men_gong).unwrap_or(0);
    let mut men_gong_map = std::collections::BTreeMap::new();
    for k in 0..8 {
        let g = FG_FANG_WEI_RING[(idx_ring + k) % 8];
        men_gong_map.insert(FG_BA_MEN[k].to_string(), g);
    }

    // 5. 传统九星飞泊
    let names = ["坎一宫", "坤二宫", "震三宫", "巽四宫", "中五宫", "乾六宫", "兑七宫", "艮八宫", "离九宫"];
    let base_step = (long_gong + 8) % 9;
    let mut palaces = Vec::with_capacity(9);
    for i in 0..9 {
        let star = FEIGONG_STARS[(base_step + i) % 9];
        palaces.push((names[i], star));
    }

    // 6. 生成古籍排盘快照文本
    let mut snapshot_lines = Vec::new();
    snapshot_lines.push("[问事]".to_string());
    snapshot_lines.push("(未录问事)".to_string());
    snapshot_lines.push("".to_string());
    snapshot_lines.push("[起局]".to_string());
    snapshot_lines.push(format!("起支:{};建星起于{};青龙(甲)落 {} 宫", qi_zhi_val, jian_zhi_val, long_gong));
    snapshot_lines.push("".to_string());
    snapshot_lines.push("[干支]".to_string());
    let gan_str = FG_GAN.iter().map(|g| {
        let slot = gan_gong_map.get(*g).cloned().unwrap_or(0);
        if slot == 0 { format!("{}中", g) } else { format!("{}{}", g, slot) }
    }).collect::<Vec<_>>().join(" ");
    snapshot_lines.push(format!("甲乘龙飞九宫:{}", gan_str));
    snapshot_lines.push(format!("中宫双干:{}(五十居中)", zhong_gong_vec.join("")));
    let men_str = ["休", "生", "伤", "杜", "景", "死", "惊", "开"].iter().map(|m| {
        let g = men_gong_map.get(*m).cloned().unwrap_or(0);
        format!("{}{}", m, g)
    }).collect::<Vec<_>>().join(" ");
    snapshot_lines.push(format!("八门:休门起 {} 宫;{}", xiu_men_gong, men_str));

    let snapshot_text = snapshot_lines.join("\n");

    FeiGongResult {
        qi_zhi: qi_zhi_val.to_string(),
        jian_zhi: jian_zhi_val.to_string(),
        long_gong,
        day_gan: day_gan.map(|s| s.to_string()),
        day_zhi: day_zhi.map(|s| s.to_string()),
        zhong_gong: zhong_gong_vec,
        palace_stars: palaces,
        gan_gong: gan_gong_map,
        men_gong: men_gong_map,
        snapshot_text,
        summary: "飞宫小奇门九星九宫飞布、十天干飞泊与八门环布完备，主看流年流月客星加临与五行生克吉凶。",
    }
}
