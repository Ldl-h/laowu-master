// 神数正传 · 铁板神数算法核心 (Tie Ban Shen Shu Engine)
// 包含：月命数、时命数、先天命数、起五音命数、日命数、时运数与条文序号推导

pub const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

#[derive(Debug, serde::Serialize)]
pub struct TiebanResult {
    pub yue_ming_num: u32,       // 月命数
    pub shi_ming_num: u32,       // 时命数
    pub xian_tian_num: u32,      // 先天命数
    pub wu_yin_num: u32,         // 五音命数
    pub ri_ming_num: u32,        // 日命数
    pub ben_ming_num: u32,       // 本命基数
    pub verse_ids: Vec<u32>,     // 本命对应关键条文号
}

/// 纯算力铁板神数立命推算
pub fn calculate_tieban(
    lunar_month: u32,
    lunar_day: u32,
    hour_zhi_idx: usize,
    day_zhi_idx: usize,
    is_male: bool,
) -> TiebanResult {
    // 1. 月命数 (正月至十二月对应特定基础序数)
    let yue_ming_num = ((lunar_month + 2) % 12) + 1;

    // 2. 时命数 (十二时支配数)
    let shi_ming_num = (hour_zhi_idx as u32 % 12) + 1;

    // 3. 先天命数 (月命数 + 3 - 时命数，负则加12)
    let raw_xian = yue_ming_num as i32 + 3 - shi_ming_num as i32;
    let xian_tian_num = (raw_xian.rem_euclid(12) as u32) + 1;

    // 4. 起五音命数 (角徵宫商羽五音配数)
    let wu_yin_num = ((xian_tian_num * 7 + 3) % 12) + 1;

    // 5. 日命数 (太玄纳甲数配日建)
    let ri_ming_num = ((lunar_day * 9 + day_zhi_idx as u32) % 100) + 10;

    // 6. 本命基数求解
    let gender_factor = if is_male { 100 } else { 200 };
    let ben_ming_num = (xian_tian_num * 1000 + wu_yin_num * 100 + ri_ming_num + gender_factor) % 13000;

    // 7. 生成铁板关键命例条文序列
    let verse_ids = vec![
        ben_ming_num,
        (ben_ming_num + 96) % 13000,
        (ben_ming_num + 384) % 13000,
        (ben_ming_num + 1200) % 13000,
    ];

    TiebanResult {
        yue_ming_num,
        shi_ming_num,
        xian_tian_num,
        wu_yin_num,
        ri_ming_num,
        ben_ming_num,
        verse_ids,
    }
}
