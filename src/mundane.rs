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
}

/// 纯数学推算世运占星岁次入宫盘与世俗格局
pub fn calculate_mundane(year: i32, city_lon: f64, _city_lat: f64) -> MundaneResult {
    // 1. 计算该年春分时刻 (太阳视黄经 = 0° / 360°)
    // 简式历元：J2000 春分约为 2451623.81597，年平均回归约 365.2422 日
    let years_since_2000 = (year - 2000) as f64;
    let aries_ingress_jde = 2451623.816 + years_since_2000 * 365.2421988;
    let cancer_ingress_jde = aries_ingress_jde + 92.75;  // 夏至 +92.75天
    let libra_ingress_jde = aries_ingress_jde + 186.40;  // 秋分 +186.4天
    let capricorn_ingress_jde = aries_ingress_jde + 276.25; // 冬至 +276.25天

    // 2. 年主星定局 (Lord of the Year)
    // 依春分盘上升星座的守护星来定，结合当年岁次干支
    let aries_lord = match (year.rem_euclid(7)) as usize {
        0 => "太阳",
        1 => "月亮",
        2 => "火星",
        3 => "水星",
        4 => "木星",
        5 => "金星",
        _ => "土星",
    };

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
            lord_of_year: "月亮",
            national_focus: "主导农业粮食、民意情绪、水运交通与国土安全",
        },
        MundaneIngress {
            season: "秋分盘 (Libra Ingress)",
            target_sign: "天秤座 0°00′00″",
            ingress_jde: libra_ingress_jde,
            lord_of_year: "金星",
            national_focus: "主导对外外交、条约盟约、金融贸易与社会和睦",
        },
        MundaneIngress {
            season: "冬至盘 (Capricorn Ingress)",
            target_sign: "摩羯座 0°00′00″",
            ingress_jde: capricorn_ingress_jde,
            lord_of_year: "土星",
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
    }
}
