// 河洛理数天地数与元堂动爻推演



// 洛书数 -> 八卦 (1坎 2坤 3震 4巽 6乾 7兑 8艮 9离)
pub const LUOSHU_GUA: [&str; 10] = ["", "坎", "坤", "震", "巽", "中", "乾", "兌", "艮", "離"];

// 天干配数 (陈抟河洛理数纳甲数):
// 甲6 乙2 丙8 丁7 戊1 己9 庚3 辛4 壬6 癸2
pub fn gan_to_number(c: char) -> u32 {
    match c {
        '甲' => 6,
        '乙' => 2,
        '丙' => 8,
        '丁' => 7,
        '戊' => 1,
        '己' => 9,
        '庚' => 3,
        '辛' => 4,
        '壬' => 6,
        _ => 2, // 癸
    }
}

// 地支河图数对 [奇数, 偶数]
pub fn zhi_pair(s: &str) -> (u32, u32) {
    match s {
        "子" | "亥" => (1, 6),
        "寅" | "卯" => (3, 8),
        "巳" | "午" => (7, 2),
        "申" | "酉" => (9, 4),
        _ => (5, 10), // 辰 戌 丑 未
    }
}

// 64 卦矩阵索引: 坎0, 坤1, 震2, 巽3, 乾4, 兌5, 艮6, 離7
pub const GUA64_HELUO: [[&str; 8]; 8] = [
    /* 坎 */ ["坎為水", "水地比", "水雷屯", "水風井", "水天需", "水澤節", "水山蹇", "水火既濟"],
    /* 坤 */ ["地水師", "坤為地", "地雷復", "地風升", "地天泰", "地澤臨", "地山謙", "地火明夷"],
    /* 震 */ ["雷水解", "雷地豫", "震為雷", "雷風恆", "雷天大壯", "雷澤歸妹", "雷山小過", "雷火豐"],
    /* 巽 */ ["風水渙", "風地觀", "風雷益", "巽為風", "風天小畜", "風澤中孚", "風山漸", "風火家人"],
    /* 乾 */ ["天水訟", "天地否", "天雷無妄", "天風姤", "乾為天", "天澤履", "天山遯", "天火同人"],
    /* 兌 */ ["澤水困", "澤地萃", "澤雷隨", "澤風大過", "澤天夬", "兌為澤", "澤山咸", "澤火革"],
    /* 艮 */ ["山水蒙", "山地剝", "山雷頤", "山風蠱", "山天大畜", "山澤損", "艮為山", "山火賁"],
    /* 離 */ ["火水未濟", "火地晉", "火雷噬嗑", "火風鼎", "火天大有", "火澤睽", "火山旅", "離為火"],
];

pub fn heluo_trigram_index(name: &str) -> usize {
    match name {
        "坎" => 0,
        "坤" => 1,
        "震" => 2,
        "巽" => 3,
        "乾" => 4,
        "兌" | "兑" => 5,
        "艮" => 6,
        _ => 7, // 離 / 离
    }
}

pub fn trigram_to_bits(name: &str) -> [u8; 3] {
    match name {
        "乾" => [1, 1, 1],
        "兌" | "兑" => [1, 1, 0],
        "離" | "离" => [1, 0, 1],
        "震" => [1, 0, 0],
        "巽" => [0, 1, 1],
        "坎" => [0, 1, 0],
        "艮" => [0, 0, 1],
        _ => [0, 0, 0], // 坤
    }
}

pub fn bits_to_trigram(b: [u8; 3]) -> &'static str {
    match b {
        [1, 1, 1] => "乾",
        [1, 1, 0] => "兌",
        [1, 0, 1] => "離",
        [1, 0, 0] => "震",
        [0, 1, 1] => "巽",
        [0, 1, 0] => "坎",
        [0, 0, 1] => "艮",
        _ => "坤",
    }
}

#[derive(Debug, serde::Serialize)]
pub struct HeLuoResult {
    pub tian_shu: u32,               // 天数 (奇数和)
    pub di_shu: u32,                 // 地数 (偶数和)
    pub tian_num: u32,               // 天数除二十五之余数
    pub di_num: u32,                 // 地数除三十之余数
    pub xian_tian_shang: &'static str, // 先天上卦
    pub xian_tian_xia: &'static str,   // 先天下卦
    pub xian_tian_gua: &'static str,   // 先天命卦 (64卦名)
    pub hou_tian_gua: &'static str,    // 后天运卦 (64卦名)
    pub yuan_tang_yao: u32,          // 元堂动爻位 (1~6爻)
    pub yao_ci_summary: &'static str, // 元堂断语
}

/// 纯数学河洛理数推演（依传统河洛秘蕴起数与元堂装爻）
pub fn calculate_heluo(
    year_gan: char, year_zhi: &str,
    month_gan: char, month_zhi: &str,
    day_gan: char, day_zhi: &str,
    hour_gan: char, hour_zhi: &str,
    birth_year: i32,
    is_male: bool,
) -> HeLuoResult {
    let mut tian: u32 = 0;
    let mut di: u32 = 0;

    for &g in &[year_gan, month_gan, day_gan, hour_gan] {
        let gn = gan_to_number(g);
        if gn % 2 == 1 { tian += gn; } else { di += gn; }
    }

    for &z in &[year_zhi, month_zhi, day_zhi, hour_zhi] {
        let (odd, even) = zhi_pair(z);
        tian += odd;
        di += even;
    }

    // 归数算法
    let mut vt = tian;
    while vt > 25 { vt -= 25; }
    let tian_num = if vt == 25 {
        5
    } else if vt >= 10 {
        let u = vt % 10;
        if u == 0 { vt / 10 } else { u }
    } else {
        vt
    };

    let mut vd = di;
    while vd > 30 { vd -= 30; }
    let di_num = if vd == 30 {
        3
    } else if vd >= 10 {
        let u = vd % 10;
        if u == 0 { vd / 10 } else { u }
    } else {
        vd
    };

    // 5寄中宫：三元寄宫法
    // 判元改用 180 年循环(1864 甲子起,每元 60 年): 0 上元 / 1 中元 / 2 下元
    let cyc = (((birth_year - 1864) % 180 + 180) % 180) as usize;
    let yuan_period = cyc / 60;
    let is_yang_year = matches!(year_zhi, "子" | "寅" | "辰" | "午" | "申" | "戌");
    let is_yang_gan = matches!(year_gan, '甲' | '丙' | '戊' | '庚' | '壬');
    let ay = (is_yang_gan && is_male) || (!is_yang_gan && !is_male);

    let wu_ji_gong = if yuan_period <= 1 {
        if ay { "艮" } else { "坤" }
    } else if ay { "離" } else { "兌" };

    let get_gua = |n: u32| -> &'static str {
        if n == 5 {
            wu_ji_gong
        } else {
            match n {
                1 => "坎",
                2 => "坤",
                3 => "震",
                4 => "巽",
                6 => "乾",
                7 => "兌",
                8 => "艮",
                _ => "離",
            }
        }
    };

    let t_gua = get_gua(tian_num);
    let d_gua = get_gua(di_num);

    // 阳男阴女 天上地下，阴男阳女 天下地上
    // 阳男 = is_male && is_yang_year; 阴女 = !is_male && !is_yang_year; 阴男 = is_male && !is_yang_year; 阳女 = !is_male && is_yang_year
    let tian_top = (is_male && is_yang_year) || (!is_male && !is_yang_year);
    let (up, low) = if tian_top { (t_gua, d_gua) } else { (d_gua, t_gua) };

    let up_idx = heluo_trigram_index(up);
    let low_idx = heluo_trigram_index(low);
    let xian_tian_gua = GUA64_HELUO[up_idx][low_idx];

    // 六爻构造：自下而上，低位 0..2 为下卦，3..5 为上卦
    let low_bits = trigram_to_bits(low);
    let up_bits = trigram_to_bits(up);
    let lines = [low_bits[0], low_bits[1], low_bits[2], up_bits[0], up_bits[1], up_bits[2]];

    // 元堂动爻算法
    let yang_hours = ["子", "丑", "寅", "卯", "辰", "巳"];
    let is_yang_hour = yang_hours.contains(&hour_zhi);
    let match_val = if is_yang_hour { 1 } else { 0 };
    let hours = if is_yang_hour {
        ["子", "丑", "寅", "卯", "辰", "巳"]
    } else {
        ["午", "未", "申", "酉", "戌", "亥"]
    };
    let hi = hours.iter().position(|&x| x == hour_zhi).unwrap_or(0);

    let mut m_vec = Vec::new();
    let mut o_vec = Vec::new();
    for p in 1..=6 {
        if lines[p - 1] == match_val {
            m_vec.push(p as u32);
        } else {
            o_vec.push(p as u32);
        }
    }

    let k = m_vec.len();
    let yuan_tang_yao = if k == 0 || k == 6 {
        // 纯卦乾坤特殊配时
        let pure_map = match hour_zhi {
            "子" | "卯" => 1,
            "丑" | "辰" => 2,
            "寅" | "巳" => 3,
            "午" | "酉" => 4,
            "未" | "戌" => 5,
            _ => 6,
        };
        let yang_ling = matches!(month_zhi, "子" | "丑" | "寅" | "卯" | "辰" | "巳");
        let reverse = if up == "乾" {
            !is_male && yang_ling
        } else {
            is_male && !yang_ling
        };
        if reverse { 7 - pure_map } else { pure_map }
    } else {
        let mut slots = Vec::new();
        if k <= 3 {
            slots.extend_from_slice(&m_vec);
            slots.extend_from_slice(&m_vec);
            slots.extend_from_slice(&o_vec);
        } else {
            slots.extend_from_slice(&m_vec);
            slots.extend_from_slice(&o_vec);
        }
        slots.truncate(6);
        slots.get(hi).copied().unwrap_or(1)
    };

    // 后天卦推演：元堂爻阴阳翻转，移外卦入内、内卦出外（三至尊卦除外）
    let mut flipped_lines = lines;
    if (1..=6).contains(&yuan_tang_yao) {
        flipped_lines[(yuan_tang_yao - 1) as usize] ^= 1;
    }

    let zhi_zun = ["坎為水", "水雷屯", "水山蹇"];
    let yang_ling = matches!(month_zhi, "子" | "丑" | "寅" | "卯" | "辰" | "巳");
    let bu_yi = zhi_zun.contains(&xian_tian_gua) && ((yuan_tang_yao == 5 && !yang_ling) || (yuan_tang_yao == 6 && yang_ling));

    let hou_tian_gua = if bu_yi {
        let h_low = bits_to_trigram([flipped_lines[0], flipped_lines[1], flipped_lines[2]]);
        let h_up = bits_to_trigram([flipped_lines[3], flipped_lines[4], flipped_lines[5]]);
        GUA64_HELUO[heluo_trigram_index(h_up)][heluo_trigram_index(h_low)]
    } else {
        // 内外卦互换
        let h_lines = [flipped_lines[3], flipped_lines[4], flipped_lines[5], flipped_lines[0], flipped_lines[1], flipped_lines[2]];
        let h_low = bits_to_trigram([h_lines[0], h_lines[1], h_lines[2]]);
        let h_up = bits_to_trigram([h_lines[3], h_lines[4], h_lines[5]]);
        GUA64_HELUO[heluo_trigram_index(h_up)][heluo_trigram_index(h_low)]
    };

    let yao_ci_summary = match yuan_tang_yao {
        1 => "初爻发端：履霜坚冰，谨始慎微，积善有庆",
        2 => "二爻得中：见龙在田，利见大人，谦和获吉",
        3 => "三爻劳谦：终日乾乾，夕惕若厉，危而不失",
        4 => "四爻进退：或跃在渊，审时度势，进退自如",
        5 => "五爻九五：飞龙在天，利见大人，当位得尊",
        _ => "上爻亢极：亢龙有悔，盈不可久，功成身退",
    };

    HeLuoResult {
        tian_shu: tian,
        di_shu: di,
        tian_num,
        di_num,
        xian_tian_shang: up,
        xian_tian_xia: low,
        xian_tian_gua,
        hou_tian_gua,
        yuan_tang_yao,
        yao_ci_summary,
    }
}
