// 邵子参评数（金锁银匙）起数与本命大运推演引擎 (Shao Zi Can Ping Divination Engine)
// 纯 Rust 高精度零依赖实现，基于四柱纳音与日宫、时支生克起数

pub const BRANCHES: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

pub const NAYIN_TABLE: [(&str, &str); 60] = [
    ("甲子", "金"), ("乙丑", "金"), ("丙寅", "火"), ("丁卯", "火"), ("戊辰", "木"), ("己巳", "木"),
    ("庚午", "土"), ("辛未", "土"), ("壬申", "金"), ("癸酉", "金"), ("甲戌", "火"), ("乙亥", "火"),
    ("丙子", "水"), ("丁丑", "水"), ("戊寅", "土"), ("己卯", "土"), ("庚辰", "金"), ("辛巳", "金"),
    ("壬午", "木"), ("癸未", "木"), ("甲申", "水"), ("乙酉", "水"), ("丙戌", "土"), ("丁亥", "土"),
    ("戊子", "火"), ("己丑", "火"), ("庚寅", "木"), ("辛卯", "木"), ("壬辰", "水"), ("癸巳", "水"),
    ("甲午", "金"), ("乙未", "金"), ("丙申", "火"), ("丁酉", "火"), ("戊戌", "木"), ("己亥", "木"),
    ("庚子", "土"), ("辛丑", "土"), ("壬寅", "金"), ("癸卯", "金"), ("甲辰", "火"), ("乙巳", "火"),
    ("丙午", "水"), ("丁未", "水"), ("戊申", "土"), ("己酉", "土"), ("庚戌", "金"), ("辛亥", "金"),
    ("壬子", "木"), ("癸丑", "木"), ("甲寅", "水"), ("乙卯", "水"), ("丙辰", "土"), ("丁巳", "土"),
    ("戊午", "火"), ("己未", "火"), ("庚申", "木"), ("辛酉", "木"), ("壬戌", "水"), ("癸亥", "水"),
];

pub fn branch_index(b: &str) -> i32 {
    match b {
        "子" => 1, "丑" => 2, "寅" => 3, "卯" => 4, "辰" => 5, "巳" => 6,
        "午" => 7, "未" => 8, "申" => 9, "酉" => 10, "戌" => 11, "亥" => 12,
        _ => 1,
    }
}

pub fn branch_from_index(idx: i32) -> &'static str {
    let i = (((idx - 1) % 12 + 12) % 12) as usize;
    BRANCHES[i]
}

// 明法日宫映射：月支反向取日宫支
// 寅->亥, 卯->戌, 辰->酉, 巳->申, 午->未, 未->午, 申->巳, 酉->辰, 戌->卯, 亥->寅, 子->丑, 丑->子
pub fn month_to_day_palace(month_zhi: &str) -> &'static str {
    match month_zhi {
        "寅" => "亥", "卯" => "戌", "辰" => "酉", "巳" => "申", "午" => "未", "未" => "午",
        "申" => "巳", "酉" => "辰", "戌" => "卯", "亥" => "寅", "子" => "丑", "丑" => "子",
        _ => "子",
    }
}

// 纳音五行附加数：水火+27，土+50，木金+0
pub fn element_add(element: &str) -> u32 {
    match element {
        "水" | "火" => 27,
        "土" => 50,
        _ => 0, // 木 金
    }
}

// 纳音五行配数：水1、火2、木3、金4、土5
pub fn element_pei(element: &str) -> u32 {
    match element {
        "水" => 1,
        "火" => 2,
        "木" => 3,
        "金" => 4,
        "土" => 5,
        _ => 1,
    }
}

pub fn get_nayin_element(gz: &str) -> &'static str {
    for (name, elem) in NAYIN_TABLE.iter() {
        if *name == gz {
            return elem;
        }
    }
    "金"
}

// 命宫推算：日宫支配卯时起，逆数至生时
pub fn calculate_ming_gong(day_palace_branch: &str, hour_zhi: &str) -> &'static str {
    let mao = 4; // 卯=4
    let dp = branch_index(day_palace_branch);
    let hb = branch_index(hour_zhi);
    let idx = ((dp - (hb - mao) - 1) % 12 + 12) % 12 + 1;
    branch_from_index(idx)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CanPingNumbers {
    pub shun_step: u32,
    pub ni_step: u32,
    pub zi_round: u32,
    pub num_shun: u32,
    pub num_ni: u32,
}

pub fn compute_canping_numbers(day_branch: &str, hour_branch: &str, element: &str) -> CanPingNumbers {
    let dp = branch_index(day_branch) as u32;
    let hb = branch_index(hour_branch) as u32;
    let shun = ((12 + (hb as i32 - dp as i32)) % 12 + 1) as u32;
    let ni = 14 - shun;
    let zi_round = dp + hb;
    let add = element_add(element);
    let pei = element_pei(element);
    let base = 2000 + zi_round + add + pei;
    CanPingNumbers {
        shun_step: shun,
        ni_step: ni,
        zi_round,
        num_shun: base + shun * 100,
        num_ni: base + ni * 100,
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CanPingResult {
    pub day_palace: &'static str,
    pub ming_gong: &'static str,
    pub nayin_element: &'static str,
    pub birth_numbers: CanPingNumbers,
    pub dayun_numbers: Vec<(u32, &'static str, CanPingNumbers)>, // (起运年, 大运地支, 数)
}

/// 计算邵子参评数
pub fn calculate_canping(
    year_gz: &str,
    month_zhi: &str,
    day_zhi: &str,
    hour_zhi: &str,
    is_gu_method: bool,
) -> CanPingResult {
    let nayin = get_nayin_element(year_gz);
    let dp = if is_gu_method {
        branch_from_index(branch_index(day_zhi))
    } else {
        month_to_day_palace(month_zhi)
    };
    let mg = calculate_ming_gong(dp, hour_zhi);
    let birth_nums = compute_canping_numbers(dp, hour_zhi, nayin);

    // 大运推导：从命宫顺行，10年一运，与日宫支计算
    let mut dayun = Vec::with_capacity(9);
    let mg_idx = branch_index(mg);
    for i in 0..9 {
        let b = branch_from_index(mg_idx + i as i32);
        let nums = compute_canping_numbers(dp, b, nayin);
        dayun.push((i * 10 + 1, b, nums));
    }

    CanPingResult {
        day_palace: dp,
        ming_gong: mg,
        nayin_element: nayin,
        birth_numbers: birth_nums,
        dayun_numbers: dayun,
    }
}
