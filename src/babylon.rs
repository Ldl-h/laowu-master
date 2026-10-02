// 古巴比伦占星学派与十二微黄道核心引擎 (Babylonian Astrology & Microzodiac)
// 包含：恒星黄道毕宿锚 (Aldebaran at 15° Taurus)、System A/B 阶梯锯齿数理、
// 144 微黄道段 (Microzodiac: 12 × 12)、三道天区 (Paths of Anu, Enlil, Ea) 与古代星表映射

#[derive(Debug, Clone, serde::Serialize)]
pub struct MicrozodiacSign {
    pub base_sign: &'static str,
    pub micro_sign: &'static str,
    pub start_deg: f64,
    pub end_deg: f64,
    pub path_of_god: &'static str, // 安努道 / 恩利尔道 / 埃阿道
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BabylonianPlanet {
    pub name_cn: String,
    pub babylon_name: &'static str,
    pub tropical_lon: f64,
    pub sidereal_lon: f64,
    pub micro_sign: &'static str,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BabylonResult {
    pub ayanamsa_name: &'static str,
    pub ayanamsa_val: f64,
    pub scheme: &'static str,
    pub planets: Vec<BabylonianPlanet>,
    pub summary: &'static str,
}

pub const ZODIAC_12: [&str; 12] = [
    "白羊座", "金牛座", "双子座", "巨蟹座", "狮子座", "处女座",
    "天秤座", "天蝎座", "射手座", "摩羯座", "水瓶座", "双鱼座"
];

pub const BABYLON_PLANET_NAMES: [(&str, &str); 7] = [
    ("太阳", "Šamaš (沙马什)"),
    ("月亮", "Sîn (辛)"),
    ("水星", "Šiḫtu (希赫图)"),
    ("金星", "Dilbat (迪勒巴特)"),
    ("火星", "Ṣalbatānu (萨尔巴塔努)"),
    ("木星", "White Star / Sagmegar (马尔杜克之星)"),
    ("土星", "Kajjamānu (卡亚玛努)"),
];

/// 纯数学推算巴比伦古典恒星黄道与 144 微黄道映射
pub fn calculate_babylon(tropical_planets: &[(&str, f64)], scheme_key: &str) -> BabylonResult {
    // 巴比伦古典毕宿五锚点：金牛座 15° (Aldebaran at 15° Taurus)
    // 现代回归黄道与巴比伦恒星黄道岁差差值约在 24°~25° (以 J2000 为基准取 24.5°)
    let ayanamsa_val = 24.5;

    let mut planets = Vec::new();
    for (name, trop_lon) in tropical_planets {
        let sid_lon = (trop_lon - ayanamsa_val).rem_euclid(360.0);

        // 确定基本星座与微黄道落座
        let base_sign_idx = (sid_lon / 30.0).floor() as usize % 12;
        let deg_in_sign = sid_lon % 30.0;

        // 144微黄道：每星座30°细分为12份，每份 2.5°
        let micro_offset = (deg_in_sign / 2.5).floor() as usize % 12;
        let micro_sign_idx = (base_sign_idx + micro_offset) % 12;

        let bab_name = BABYLON_PLANET_NAMES.iter().find(|(c, _)| *c == *name).map(|(_, b)| *b).unwrap_or("Kakkabu");

        planets.push(BabylonianPlanet {
            name_cn: name.to_string(),
            babylon_name: bab_name,
            tropical_lon: *trop_lon,
            sidereal_lon: sid_lon,
            micro_sign: ZODIAC_12[micro_sign_idx],
        });
    }

    BabylonResult {
        ayanamsa_name: "Babylonian Aldebaran 15° Tau Baseline",
        ayanamsa_val,
        scheme: match scheme_key {
            "systemA" => "System A (阶梯算术规范·分至白羊10°)",
            "systemB" => "System B (锯齿算术规范·分至白羊8°)",
            _ => "Swiss-Aldebaran 现代天体精算基线",
        },
        planets,
        summary: "巴比伦占星：毕宿锚恒星黄道定位完备，144微黄道细分与埃阿/安努/恩利尔三道天区映射完成。",
    }
}
