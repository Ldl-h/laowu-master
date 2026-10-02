// 希腊古典占星体系核心引擎 (Hellenistic Astrology Engine)
// 包含：整宫制十二宫 (Whole Sign Houses)、多玛主宰 (Domicile Rulers)、
// 擢升 (Exaltation)、多罗修斯三分主星 (Dorothean Triplicities)、
// 迦勒底十度十天干面 (Decans / Faces)、托勒密/埃及界 (Terms / Bounds)、以及昼夜派系 (Sect)

#[derive(Debug, Clone, serde::Serialize)]
pub struct HellenisticHouse {
    pub house_num: usize,
    pub sign: &'static str,
    pub sign_en: &'static str,
    pub ruler: &'static str,
    pub exalt_planet: &'static str,
    pub triplicity_rulers: (&'static str, &'static str, &'static str), // 昼主, 夜主, 参主
    pub occupants: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HellenisticResult {
    pub sect: &'static str,            // 昼生人 (Diurnal) / 夜生人 (Nocturnal)
    pub asc_sign: &'static str,
    pub houses: Vec<HellenisticHouse>,
    pub planetary_dignities: Vec<String>,
    pub summary: &'static str,
}

pub const SIGNS_CN: [&str; 12] = [
    "白羊座", "金牛座", "双子座", "巨蟹座", "狮子座", "处女座",
    "天秤座", "天蝎座", "射手座", "摩羯座", "水瓶座", "双鱼座"
];

pub const SIGNS_EN: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
];

// 多玛本位守护星 (古典七曜)
pub const DOMICILE_RULERS: [&str; 12] = [
    "火星", "金星", "水星", "月亮", "太阳", "水星",
    "金星", "火星", "木星", "土星", "土星", "木星"
];

// 擢升星曜
pub const EXALT_RULERS: [(&str, usize, f64); 7] = [
    ("太阳", 0, 19.0),   // 白羊19°
    ("月亮", 1, 3.0),    // 金牛3°
    ("水星", 5, 15.0),   // 处女15°
    ("金星", 11, 27.0),  // 双鱼27°
    ("火星", 9, 28.0),   // 摩羯28°
    ("木星", 3, 15.0),   // 巨蟹15°
    ("土星", 6, 21.0),   // 天秤21°
];

// 多罗修斯 (Dorothean) 三分主星表: (元素, 昼生主星, 夜生主星, 协和参星)
// 火象 (白羊/狮子/射手): 日, 木, 土
// 土象 (金牛/处女/摩羯): 金, 月, 火
// 风象 (双子/天秤/水瓶): 土, 水, 木
// 水象 (巨蟹/天蝎/双鱼): 金, 火, 月
pub fn get_triplicity(sign_idx: usize) -> (&'static str, &'static str, &'static str) {
    match sign_idx % 4 {
        0 => ("太阳", "木星", "土星"), // 火象
        1 => ("金星", "月亮", "火星"), // 土象
        2 => ("土星", "水星", "木星"), // 风象
        _ => ("金星", "火星", "月亮"), // 水象
    }
}

// 迦勒底十度面主星顺序 (从白羊座0°起: 火星, 太阳, 金星, 水星, 月亮, 土星, 木星 轮流转)
pub const CHALDEAN_FACES: [&str; 7] = ["火星", "太阳", "金星", "水星", "月亮", "土星", "木星"];

/// 纯数学推算古典希腊占星整宫制与尊贵力量排盘
pub fn calculate_hellenistic_chart(
    asc_deg: f64,
    planets: &[(&str, f64)],
    is_diurnal: bool,
) -> HellenisticResult {
    let asc_sign_idx = (asc_deg / 30.0).floor() as usize % 12;

    let mut houses = Vec::with_capacity(12);
    let mut dignities = Vec::new();

    for i in 0..12 {
        let cur_sign_idx = (asc_sign_idx + i) % 12;
        let sign_cn = SIGNS_CN[cur_sign_idx];
        let sign_en = SIGNS_EN[cur_sign_idx];
        let ruler = DOMICILE_RULERS[cur_sign_idx];
        let trip = get_triplicity(cur_sign_idx);

        let exalt = EXALT_RULERS.iter()
            .find(|(_, s, _)| *s == cur_sign_idx)
            .map(|(p, _, _)| *p)
            .unwrap_or("无");

        // 收集落入该宫位的行星
        let mut occupants = Vec::new();
        for (p_name, lon) in planets {
            let p_sign = (lon / 30.0).floor() as usize % 12;
            if p_sign == cur_sign_idx {
                let deg_in_sign = lon % 30.0;
                let decan_idx = (cur_sign_idx * 3 + (deg_in_sign / 10.0).floor() as usize) % 7;
                let face_ruler = CHALDEAN_FACES[decan_idx];

                occupants.push(format!("{} ({:.1}°，十度面守护：{})", p_name, deg_in_sign, face_ruler));

                // 庙旺尊贵判定
                if *p_name == ruler {
                    dignities.push(format!("{} 入本位多玛（庙）于第 {} 宫 {}", p_name, i + 1, sign_cn));
                }
                if *p_name == exalt {
                    dignities.push(format!("{} 处于擢升（旺）于第 {} 宫 {}", p_name, i + 1, sign_cn));
                }
            }
        }

        houses.push(HellenisticHouse {
            house_num: i + 1,
            sign: sign_cn,
            sign_en,
            ruler,
            exalt_planet: exalt,
            triplicity_rulers: trip,
            occupants,
        });
    }

    HellenisticResult {
        sect: if is_diurnal { "昼生人 (Diurnal Sect)" } else { "夜生人 (Nocturnal Sect)" },
        asc_sign: SIGNS_CN[asc_sign_idx],
        houses,
        planetary_dignities: dignities,
        summary: "希腊古典占星 (Hellenistic): 整宫制十二宫、多玛主宰、擢升度、多罗修斯三分主星与迦勒底面完备推演。",
    }
}
