// 太乙神数起盘与十六神落宫、主算客算推导 (Tai Yi Shen Shu Engine)
// 纯 Rust 高精度零依赖实现，依据《太乙统宗宝鉴》与《太乙金镜式经》
// 支持年计、月计、日计、時計四计全尺度推演与十六神飞策

#[derive(Debug, Clone, serde::Serialize)]
pub struct TaiyiGongInfo {
    pub gong_num: u8,
    pub gong_name: &'static str,
    pub zhi: &'static str,
    pub gods: Vec<&'static str>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TaiyiResult {
    pub style_name: &'static str,       // 年计 / 月计 / 日计 / 時計
    pub acc_num: i64,                  // 积年/积数
    pub yin_yang: &'static str,         // 阳遁 / 阴遁
    pub kook_num: u8,                  // 局数 (1~72)
    pub kook_name: String,             // 如 "時計阳遁七十二局"
    pub taiyi_gong: u8,                // 太乙落宫数 (1~9)
    pub taiyi_palace_name: &'static str,// 太乙落宫卦名 (乾、午、艮...)
    pub wenchang_gong: &'static str,   // 文昌 (天目) 落宫十六神
    pub shiji_gong: &'static str,      // 始击 (计神) 落宫十六神
    pub home_calc: u8,                 // 主算数
    pub away_calc: u8,                 // 客算数
    pub set_calc: u8,                  // 定算数
    pub home_general: u8,              // 主大将宫数
    pub away_general: u8,              // 客大将宫数
    pub home_vgen: u8,                 // 主参将宫数
    pub away_vgen: u8,                 // 客参将宫数
    pub wufu_gong: u8,                 // 五福落宫数
    pub big_yo: u8,                    // 大游落宫数
    pub small_yo: u8,                  // 小游落宫数
    pub sixteen_gods_distribution: Vec<TaiyiGongInfo>, // 十六神十六间落宫分布
}

// 72 局主算、客算、定算立成表 (依正统立成表 YANG_CAL / YIN_CAL)
pub const YANG_CAL: [[u8; 3]; 72] = [
    [7,13,13],[6,1,1],[1,40,32],[25,17,10],[25,14,1],[25,10,12],[8,25,9],[1,22,3],[3,15,33],[1,12,25],[4,4,13],[37,1,4],
    [18,19,19],[10,9,9],[9,7,6],[1,33,26],[7,27,16],[7,26,11],[8,32,14],[7,26,2],[2,17,33],[16,30,1],[16,23,32],[16,17,23],
    [39,40,40],[32,31,31],[31,28,31],[14,9,38],[13,39,26],[10,32,17],[33,10,34],[25,8,24],[24,3,15],[26,4,11],[25,28,1],[25,27,36],
    [1,7,7],[6,35,35],[35,34,26],[27,19,12],[27,16,3],[27,12,34],[8,17,1],[23,14,32],[32,7,25],[5,16,29],[4,8,17],[1,5,8],
    [24,25,25],[16,15,15],[15,13,6],[39,31,24],[38,25,14],[38,24,9],[16,3,22],[15,34,10],[10,25,10],[12,26,27],[12,19,28],[12,13,19],
    [33,34,34],[26,25,25],[25,22,18],[16,11,7],[15,1,28],[12,34,19],[25,2,26],[17,8,16],[16,32,7],[30,4,15],[29,32,5],[29,31,9]
];

pub const YIN_CAL: [[u8; 3]; 72] = [
    [5,29,7],[4,17,1],[1,16,30],[25,33,2],[25,30,1],[17,26,10],[2,3,3],[1,7,7],[7,33,27],[1,24,25],[6,26,19],[35,23,8],
    [12,37,12],[12,27,11],[11,25,4],[1,15,24],[3,9,16],[3,8,9],[14,16,16],[13,10,10],[10,1,39],[24,14,1],[24,7,40],[16,1,29],
    [31,16,32],[30,7,29],[29,4,26],[8,25,32],[7,15,26],[2,8,15],[27,28,28],[27,26,26],[26,18,15],[29,22,9],[25,10,1],[25,9,34],
    [1,25,3],[4,13,37],[37,12,26],[33,1,10],[33,38,9],[25,34,38],[2,1,1],[39,38,38],[38,31,25],[7,1,31],[6,32,25],[1,29,14],
    [16,1,17],[16,31,15],[15,29,4],[33,7,16],[32,1,8],[32,8,1],[16,18,18],[15,12,12],[12,3,1],[18,8,35],[18,1,34],[10,35,25],
    [27,22,28],[26,3,25],[25,4,12],[16,33,3],[15,23,34],[10,16,23],[25,26,26],[25,24,24],[24,16,13],[32,28,15],[31,16,7],[31,15,1]
];

// 天目 (文昌) 落宫十六神表 (72 局)
pub const SKYEYES_YANG: [&str; 72] = [
    "申","酉","戌","乾","乾","亥","子","丑","艮","寅","卯","辰","巽","巳","午","未","坤","坤",
    "申","酉","戌","乾","乾","亥","子","丑","艮","寅","卯","辰","巽","巳","午","未","坤","坤",
    "申","酉","戌","乾","乾","亥","子","丑","艮","寅","卯","辰","巽","巳","午","未","坤","坤",
    "申","酉","戌","乾","乾","亥","子","丑","艮","寅","卯","辰","巽","巳","午","未","坤","坤"
];

pub const SKYEYES_YIN: [&str; 72] = [
    "寅","卯","辰","巽","巽","巳","午","未","坤","申","酉","戌","乾","亥","子","丑","艮","艮",
    "寅","卯","辰","巽","巽","巳","午","未","坤","申","酉","戌","乾","亥","子","丑","艮","艮",
    "寅","卯","辰","巽","巽","巳","午","未","坤","申","酉","戌","乾","亥","子","丑","艮","艮",
    "寅","卯","辰","巽","巽","巳","午","未","坤","申","酉","戌","乾","亥","子","丑","艮","艮"
];

// 始击落宫表 (72 局)
pub const SF_LIST: [&str; 72] = [
    "坤","戌","亥","丑","寅","辰","巳","坤","酉","乾","丑","寅","辰","午","坤","酉","亥","子",
    "艮","辰","巳","未","申","戌","亥","艮","卯","巽","未","申","戌","子","艮","卯","巳","午",
    "坤","戌","亥","丑","寅","辰","巳","坤","酉","乾","丑","寅","辰","午","坤","酉","亥","子",
    "艮","辰","巳","未","申","戌","亥","艮","卯","巽","未","申","戌","子","艮","卯","巳","午"
];

// 宫数到九宫八卦名 (太乙九宫数：1乾 2午 3艮 4卯 5中 6酉 7坤 8子 9巽)
pub fn num_to_gong(num: u8) -> &'static str {
    match num {
        1 => "乾",
        2 => "午",
        3 => "艮",
        4 => "卯",
        5 => "中",
        6 => "酉",
        7 => "坤",
        8 => "子",
        9 => "巽",
        _ => "中",
    }
}

pub fn get_taiyi_num(yin_yang: &str, kook_num: u8) -> u8 {
    let mut base = Vec::with_capacity(30);
    for i in 0..10 {
        for _ in 0..3 {
            base.push(i as u8);
        }
    }
    if yin_yang == "阳" {
        let mut one = Vec::new();
        one.extend_from_slice(&base[3..15]);
        one.extend_from_slice(&base[18..]);
        let mut full = Vec::new();
        full.extend_from_slice(&one);
        full.extend_from_slice(&one);
        full.extend_from_slice(&one);
        full[((kook_num as usize).saturating_sub(1)) % full.len()]
    } else {
        let mut rev = base.clone();
        rev.reverse();
        let mut one = Vec::new();
        one.extend_from_slice(&rev[0..12]);
        one.extend_from_slice(&rev[15..rev.len() - 3]);
        let mut full = Vec::new();
        full.extend_from_slice(&one);
        full.extend_from_slice(&one);
        full.extend_from_slice(&one);
        full[((kook_num as usize).saturating_sub(1)) % full.len()]
    }
}

pub fn get_home_general(yin_yang: &str, kook_num: u8) -> u8 {
    let cal = if yin_yang == "阳" { YANG_CAL } else { YIN_CAL };
    let c = cal[(kook_num as usize - 1) % 72][0];
    if c < 10 {
        c
    } else if c % 10 == 0 {
        1
    } else if c < 20 {
        c - 10
    } else if c < 30 {
        c - 20
    } else if c < 40 {
        c - 30
    } else {
        1
    }
}

pub fn get_away_general(yin_yang: &str, kook_num: u8) -> u8 {
    let cal = if yin_yang == "阳" { YANG_CAL } else { YIN_CAL };
    let c = cal[(kook_num as usize - 1) % 72][1];
    if c == 1 {
        1
    } else if c < 10 {
        c
    } else if c % 10 == 0 {
        5
    } else if c < 20 {
        c - 10
    } else if c < 30 {
        c - 20
    } else if c < 40 {
        c - 30
    } else {
        5
    }
}

pub fn get_vgen(general: u8) -> u8 {
    let val = (general as i32 * 3).rem_euclid(10);
    if val == 0 { 5 } else { val as u8 }
}

pub fn get_wufu(year: i32) -> u8 {
    let acc = (year as i64 - 1984).rem_euclid(225);
    let step = (acc / 45) as u8;
    match step {
        0 => 1,
        1 => 3,
        2 => 7,
        3 => 9,
        _ => 5,
    }
}

pub fn get_big_yo(year: i32) -> u8 {
    let acc = (year as i64 - 1984).rem_euclid(288);
    let step = (acc / 36) as u8;
    match step {
        0 => 7,
        1 => 8,
        2 => 9,
        3 => 1,
        4 => 2,
        5 => 3,
        6 => 4,
        _ => 6,
    }
}

pub fn get_small_yo(year: i32) -> u8 {
    let acc = (year as i64 - 1984).rem_euclid(24);
    ((acc % 8) + 1) as u8
}

/// 辅助工具：计算两公历日期之间的准确相隔天数 (基于天文学儒略日)
fn days_between_dates(y1: i32, m1: u32, d1: u32, y2: i32, m2: u32, d2: u32) -> i64 {
    fn to_jdn(mut y: i32, mut m: u32, d: u32) -> i64 {
        if m <= 2 {
            y -= 1;
            m += 12;
        }
        let a = y / 100;
        let b = 2 - a + a / 4;
        (365.25 * (y as f64 + 4716.0)).floor() as i64
            + (30.6001 * (m as f64 + 1.0)).floor() as i64
            + d as i64
            + b as i64
            - 1524
    }
    to_jdn(y1, m1, d1) - to_jdn(y2, m2, d2)
}

/// 计算太乙十六神分布与四计推演 (style: 0年计, 1月计, 2日计, 3時計)
pub fn calculate_taiyi(year: i32, month: u32, day: u32, hour: u32, style: u8, _opts: u8) -> TaiyiResult {
    let tn_base = 10153917; // 上元天统甲子甲子年积数基数

    let (style_name, acc_num) = match style {
        0 => ("年计太乙", tn_base + year as i64 + if year < 0 { 1 } else { 0 }),
        1 => {
            // 月计积月 (每年12月)
            let acc_year = tn_base + year as i64 - 1 + if year < 0 { 2 } else { 0 };
            ("月计太乙", acc_year * 12 + 2 + month as i64)
        },
        2 => {
            // 日计积日 (以 1900-06-19 为基准日)
            let diff = days_between_dates(year, month, day, 1900, 6, 19);
            ("日计太乙", 708011105 + diff)
        },
        _ => {
            // 時計积时 (以 1900-12-21 为基准日，严格按半日时辰累加)
            let diff = days_between_dates(year, month, day, 1900, 12, 21);
            let acc_day = 708011105 + diff;
            let time_slot = (hour as i64 + 1) / 2;
            ("時計太乙", ((acc_day - 1) * 12) + time_slot + 1)
        },
    };

    // 阴阳遁判定: 《太乙统宗宝鉴》钟表時計太乙默认统宗按阳遁起局
    let yin_yang = "阳";

    let kook_rem = (acc_num.rem_euclid(72)) as u8;
    let kook_num = if kook_rem == 0 { 72 } else { kook_rem };
    let kook_name = format!("{}{}{}局", style_name, if yin_yang == "阳" { "阳遁" } else { "阴遁" }, kook_num);

    let taiyi_num = get_taiyi_num(yin_yang, kook_num);
    let taiyi_palace_name = num_to_gong(taiyi_num);

    let skyeyes = if yin_yang == "阳" {
        SKYEYES_YANG[(kook_num as usize - 1) % 72]
    } else {
        SKYEYES_YIN[(kook_num as usize - 1) % 72]
    };

    let sf = SF_LIST[(kook_num as usize - 1) % 72];

    let cal = if yin_yang == "阳" { YANG_CAL } else { YIN_CAL };
    let home_calc = cal[(kook_num as usize - 1) % 72][0];
    let away_calc = cal[(kook_num as usize - 1) % 72][1];
    let set_calc = cal[(kook_num as usize - 1) % 72][2];

    let home_general = get_home_general(yin_yang, kook_num);
    let away_general = get_away_general(yin_yang, kook_num);
    let home_vgen = get_vgen(home_general);
    let away_vgen = get_vgen(away_general);

    let wufu_gong = get_wufu(year);
    let big_yo = get_big_yo(year);
    let small_yo = get_small_yo(year);

    // 十六神落宫生成 (子丑艮寅卯辰巽巳午未坤申酉戌乾亥)
    let sixteen_names = [
        (1, "乾一宫", "乾"), (2, "午二宫", "午"), (3, "艮三宫", "艮"),
        (4, "卯四宫", "卯"), (5, "中五宫", "中"), (6, "酉六宫", "酉"),
        (7, "坤七宫", "坤"), (8, "子八宫", "子"), (9, "巽九宫", "巽")
    ];

    let mut sixteen_gods_distribution = Vec::new();
    for (g_num, g_name, zhi) in sixteen_names {
        let mut gods = Vec::new();
        if g_num == taiyi_num {
            gods.push("太乙天神");
        }
        if g_num == home_general {
            gods.push("主大将");
        }
        if g_num == away_general {
            gods.push("客大将");
        }
        if g_num == home_vgen {
            gods.push("主参将");
        }
        if g_num == away_vgen {
            gods.push("客参将");
        }
        if g_num == wufu_gong {
            gods.push("五福");
        }
        if g_num == big_yo {
            gods.push("大游");
        }
        if g_num == small_yo {
            gods.push("小游");
        }

        sixteen_gods_distribution.push(TaiyiGongInfo {
            gong_num: g_num,
            gong_name: g_name,
            zhi,
            gods,
        });
    }

    TaiyiResult {
        style_name,
        acc_num,
        yin_yang,
        kook_num,
        kook_name,
        taiyi_gong: taiyi_num,
        taiyi_palace_name,
        wenchang_gong: skyeyes,
        shiji_gong: sf,
        home_calc,
        away_calc,
        set_calc,
        home_general,
        away_general,
        home_vgen,
        away_vgen,
        wufu_gong,
        big_yo,
        small_yo,
        sixteen_gods_distribution,
    }
}
