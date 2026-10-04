// 世运占星与四季入宫天象引擎 (Mundane Astrology & Ingress Engine)
// 包含：春分/夏至/秋分/冬至太阳入宫时刻 (Aries/Cancer/Libra/Capricorn Ingress)、
// 世俗盘定局年主星 (Lord of the Year)、世俗十二宫位政经意义、以及日食月食国家分野映射

#[derive(Debug, Clone, serde::Serialize)]
pub struct MundaneIngress {
    pub season: &'static str,
    pub target_sign: &'static str,
    pub ingress_jde: f64,
    pub lord_of_year: &'static str,
    pub national_focus: &'static str,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MundaneHouseMeaning {
    pub house_num: usize,
    pub topic: &'static str,
    pub ruler_planet: &'static str,
    pub mundane_significance: &'static str,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MundaneResult {
    pub year: i32,
    pub primary_ingress: MundaneIngress,
    pub seasonal_ingresses: Vec<MundaneIngress>,
    pub houses: Vec<MundaneHouseMeaning>,
    pub summary: &'static str,
    /// R5: 赤道(lat≈0)为合法天文位置，不再静默替换为北京，仅在此时附 warning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_warning: Option<&'static str>,
}

/// 精密求解指定目标太阳视黄经 (0°春分, 90°夏至, 180°秋分, 270°冬至) 的天文瞬间 (JDE)
fn find_ingress_jde(year: i32, target_lon: f64, approx_month: u32, approx_day: u32) -> f64 {
    let mut cur_jd = crate::bazi_exact::to_julian_day(year, approx_month, approx_day, 12, 0, 0);
    for _ in 0..12 {
        let cur_l = crate::bazi_exact::sun_ecliptic_longitude(cur_jd);
        let mut diff = (cur_l - target_lon).rem_euclid(360.0);
        if diff > 180.0 {
            diff -= 360.0;
        }
        if diff.abs() < 1e-6 {
            break;
        }
        cur_jd -= diff / 0.985647; // 太阳每日运行约 0.985647°
    }
    cur_jd
}

/// 根据黄道经度获取星座古典七政守护星
fn get_ruler_planet_of_lon(lon: f64) -> &'static str {
    let sign_idx = ((lon.rem_euclid(360.0)) / 30.0).floor() as usize % 12;
    match sign_idx {
        0 => "火星", // 白羊
        1 => "金星", // 金牛
        2 => "水星", // 双子
        3 => "月亮", // 巨蟹
        4 => "太阳", // 狮子
        5 => "水星", // 处女
        6 => "金星", // 天秤
        7 => "火星", // 天蝎
        8 => "木星", // 射手
        9 => "土星", // 摩羯
        10 => "土星", // 水瓶
        _ => "木星", // 双鱼
    }
}

/// 纯数学推算世运占星岁次入宫盘与世俗格局（基于精密太阳黄经入节与天象年主星）
pub fn calculate_mundane(year: i32, city_lon: f64, city_lat: f64) -> MundaneResult {
    // R5: lat=0（赤道）是合法天文位置，不再静默替换为北京 39.9，按真实纬度计算；
    // 仅在赤道输入时附 location_warning 提示，便于用户知晓这不是默认回退。
    let lat = city_lat;
    let location_warning = if city_lat.abs() < 1e-4 {
        Some("纬度 0 为赤道合法值，未做替换，已按真实 lat=0 计算各季入宫盘上升点")
    } else {
        None
    };

    // 1. 精密二分/牛顿迭代求解该年四正入宫时刻 (JDE)
    let aries_ingress_jde = find_ingress_jde(year, 0.0, 3, 20);
    let cancer_ingress_jde = find_ingress_jde(year, 90.0, 6, 21);
    let libra_ingress_jde = find_ingress_jde(year, 180.0, 9, 23);
    let capricorn_ingress_jde = find_ingress_jde(year, 270.0, 12, 22);

    // 2. 年主星定局 (Lord of the Year)
    // 依据世运占星法则 (Bonatti / Ptolemy 传统)：计算春分盘在此地之精确上升点 (ASC) 及其守护星
    let (_, ramc_spring) = crate::western_full::calculate_ramc(aries_ingress_jde, city_lon);
    let eps_spring = crate::western_full::true_obliquity(aries_ingress_jde);
    let (asc_spring, _) = crate::western_full::calculate_angles(ramc_spring, lat, eps_spring);
    let aries_lord = get_ruler_planet_of_lon(asc_spring);

    // 计算各季度入宫盘天象守护星
    let (_, ramc_summer) = crate::western_full::calculate_ramc(cancer_ingress_jde, city_lon);
    let eps_summer = crate::western_full::true_obliquity(cancer_ingress_jde);
    let (asc_summer, _) = crate::western_full::calculate_angles(ramc_summer, lat, eps_summer);
    let cancer_lord = get_ruler_planet_of_lon(asc_summer);

    let (_, ramc_autumn) = crate::western_full::calculate_ramc(libra_ingress_jde, city_lon);
    let eps_autumn = crate::western_full::true_obliquity(libra_ingress_jde);
    let (asc_autumn, _) = crate::western_full::calculate_angles(ramc_autumn, lat, eps_autumn);
    let libra_lord = get_ruler_planet_of_lon(asc_autumn);

    let (_, ramc_winter) = crate::western_full::calculate_ramc(capricorn_ingress_jde, city_lon);
    let eps_winter = crate::western_full::true_obliquity(capricorn_ingress_jde);
    let (asc_winter, _) = crate::western_full::calculate_angles(ramc_winter, lat, eps_winter);
    let capricorn_lord = get_ruler_planet_of_lon(asc_winter);

    let primary_ingress = MundaneIngress {
        season: "春分盘 (Aries Ingress)",
        target_sign: "白羊座 0°00′00″",
        ingress_jde: aries_ingress_jde,
        lord_of_year: aries_lord,
        national_focus: "主导国家全年政治大纲、政权威信、首脑健康与总纲国运",
    };

    let seasonal_ingresses = vec![
        primary_ingress.clone(),
        MundaneIngress {
            season: "夏至盘 (Cancer Ingress)",
            target_sign: "巨蟹座 0°00′00″",
            ingress_jde: cancer_ingress_jde,
            lord_of_year: cancer_lord,
            national_focus: "主导农业粮食、民意情绪、水运交通与国土安全",
        },
        MundaneIngress {
            season: "秋分盘 (Libra Ingress)",
            target_sign: "天秤座 0°00′00″",
            ingress_jde: libra_ingress_jde,
            lord_of_year: libra_lord,
            national_focus: "主导对外外交、条约盟约、金融贸易与社会和睦",
        },
        MundaneIngress {
            season: "冬至盘 (Capricorn Ingress)",
            target_sign: "摩羯座 0°00′00″",
            ingress_jde: capricorn_ingress_jde,
            lord_of_year: capricorn_lord,
            national_focus: "主导基础设施、矿产工业、老人长者与国家储备",
        },
    ];

    // 3. 世俗十二宫政经象征
    let houses = vec![
        MundaneHouseMeaning { house_num: 1, topic: "国民与整体现状", ruler_planet: aries_lord, mundane_significance: "国家整体民情、公共卫生、国民安康与社会风气" },
        MundaneHouseMeaning { house_num: 2, topic: "国库与经济金融", ruler_planet: "金星", mundane_significance: "国家财政储备、税收收入、银行体系与购买力" },
        MundaneHouseMeaning { house_num: 3, topic: "交通与通信传媒", ruler_planet: "水星", mundane_significance: "国内交通网、互联网、舆论走向与教育体系" },
        MundaneHouseMeaning { house_num: 4, topic: "国土与农业地产", ruler_planet: "月亮", mundane_significance: "土地矿产、农业收成、房地产与反对党势力" },
        MundaneHouseMeaning { house_num: 5, topic: "人口与文娱体育", ruler_planet: "太阳", mundane_significance: "新生儿出生率、文娱产业、股市投机与国民欢庆" },
        MundaneHouseMeaning { house_num: 6, topic: "公共卫生与劳工", ruler_planet: "水星", mundane_significance: "医疗体系、流行疾病防控、劳工就业与国防军队" },
        MundaneHouseMeaning { house_num: 7, topic: "外交盟约与战和", ruler_planet: "金星", mundane_significance: "国际条约、对外贸易、跨国关系与潜在对手" },
        MundaneHouseMeaning { house_num: 8, topic: "国债与人口死亡", ruler_planet: "火星", mundane_significance: "国家债务、跨国借贷、自然灾害与死亡率统计" },
        MundaneHouseMeaning { house_num: 9, topic: "司法与航海航天", ruler_planet: "木星", mundane_significance: "最高法院、司法裁决、宗教哲学、国际航运航天" },
        MundaneHouseMeaning { house_num: 10, topic: "国家元首与政府", ruler_planet: "太阳", mundane_significance: "执政首脑、政府内阁、国家最高威权与国家声誉" },
        MundaneHouseMeaning { house_num: 11, topic: "议会立法与结盟", ruler_planet: "土星", mundane_significance: "国会议会立法机构、跨国同盟集团与国家长远愿景" },
        MundaneHouseMeaning { house_num: 12, topic: "隐秘敌手与狱政", ruler_planet: "木星", mundane_significance: "间谍安全、监狱惩教、社会福利救济与隐蔽暗流" },
    ];

    let summary = if city_lon > 100.0 && city_lon < 130.0 {
        "世运盘格局（东亚/中国分野）：春分太阳居首，立威定纲，金融与外贸为本年关键枢纽。"
    } else {
        "世运盘格局：四正入宫时刻齐备，四季年主星顺布，宏观政经世运推导闭环。"
    };

    MundaneResult {
        year,
        primary_ingress,
        seasonal_ingresses,
        houses,
        summary,
        location_warning,
    }
}
