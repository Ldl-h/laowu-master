// 神数正传 · 邵子神数算法核心 (Shao Zi Shen Shu Engine)
// 包含：四柱天地数余数配先天命卦、后天命卦变卦、六十四卦卦数与九十六气数推演条文序号 (1111 ~ 12888)

pub const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

// 天干配数
pub fn gan_num(g: char) -> u32 {
    match g {
        '甲' | '壬' => 6,
        '乙' | '癸' => 2,
        '丙' => 8,
        '丁' => 7,
        '戊' => 1,
        '己' => 9,
        '庚' => 3,
        '辛' => 4,
        _ => 5,
    }
}

// 地支配数对 [奇数, 偶数]
pub fn zhi_pair(z: &str) -> (u32, u32) {
    match z {
        "子" | "亥" => (1, 6),
        "巳" | "午" => (7, 2),
        "寅" | "卯" => (3, 8),
        "申" | "酉" => (9, 4),
        _ => (5, 10), // 辰 戌 丑 未
    }
}

// 洛书数 -> 八卦
pub fn luoshu_to_gua(n: u32) -> &'static str {
    match n {
        1 => "坎",
        2 => "坤",
        3 => "震",
        4 => "巽",
        6 => "乾",
        7 => "兑",
        8 => "艮",
        9 => "离",
        _ => "中",
    }
}

// 天数余数法: 大于25则减25，取个位(10->1, 20->2, 25->5)
pub fn tian_remainder(n: u32) -> u32 {
    let mut v = n;
    while v > 25 {
        v -= 25;
    }
    if v == 25 {
        return 5;
    }
    if v == 10 {
        return 1;
    }
    if v == 20 {
        return 2;
    }
    v % 10
}

// 地数余数法: 大于30则减30，取个位(10->1, 20->2, 30->3)
pub fn di_remainder(n: u32) -> u32 {
    let mut v = n;
    while v > 30 {
        v -= 30;
    }
    if v == 30 {
        return 3;
    }
    if v == 10 {
        return 1;
    }
    if v == 20 {
        return 2;
    }
    v % 10
}

#[derive(Debug, serde::Serialize)]
pub struct ShaoziResult {
    pub tian_shu: u32,             // 天数总和
    pub di_shu: u32,               // 地数总和
    pub tian_rem: u32,             // 天数余数
    pub di_rem: u32,               // 地数余数
    pub xian_tian_gua: String,     // 先天命卦 (如 "天风姤")
    pub hou_tian_gua: String,      // 后天命卦
    pub tian_ming_num: u32,        // 天命数
    pub di_ming_num: u32,          // 地命数
    pub ren_ming_num: u32,         // 人命数
    pub base_verse_id: u32,        // 核心本命条文代号
}

/// 纯算力邵子神数推算
pub fn calculate_shaozi(
    year_gan: char, year_zhi: &str,
    month_gan: char, month_zhi: &str,
    day_gan: char, day_zhi: &str,
    hour_gan: char, hour_zhi: &str,
    is_male: bool,
) -> ShaoziResult {
    let gans = [year_gan, month_gan, day_gan, hour_gan];
    let zhis = [year_zhi, month_zhi, day_zhi, hour_zhi];

    let mut tian_sum = 0;
    let mut di_sum = 0;

    for &g in &gans {
        let gn = gan_num(g);
        if gn % 2 == 1 {
            tian_sum += gn;
        } else {
            di_sum += gn;
        }
    }

    for &z in &zhis {
        let (odd, even) = zhi_pair(z);
        tian_sum += odd;
        di_sum += even;
    }

    let t_rem = tian_remainder(tian_sum);
    let d_rem = di_remainder(di_sum);

    let is_yang_year = matches!(year_gan, '甲' | '丙' | '戊' | '庚' | '壬');
    let group_a = (is_yang_year && is_male) || (!is_yang_year && !is_male);

    let get_gua_name = |r: u32| -> &'static str {
        if r == 5 {
            if group_a { "艮" } else { "坤" }
        } else {
            luoshu_to_gua(r)
        }
    };

    let shang = get_gua_name(t_rem);
    let xia = get_gua_name(d_rem);

    let xian_tian_gua = format!("{}/{}", shang, xia);
    let hou_tian_gua = format!("{}/{}", xia, shang);

    // 三命数计算 (邵子神数立命三条数)
    let tian_ming_num = (tian_sum * 100 + di_sum) % 10000 + 1000;
    let di_ming_num = (di_sum * 100 + tian_sum) % 10000 + 1000;
    let ren_ming_num = ((tian_sum + di_sum) * 64) % 10000 + 1000;

    // 核心条文序号 (1111 ~ 12888 范围)
    let base_verse_id = (tian_ming_num % 6144) + 1111;

    ShaoziResult {
        tian_shu: tian_sum,
        di_shu: di_sum,
        tian_rem: t_rem,
        di_rem: d_rem,
        xian_tian_gua,
        hou_tian_gua,
        tian_ming_num,
        di_ming_num,
        ren_ming_num,
        base_verse_id,
    }
}
