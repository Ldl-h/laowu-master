// 希腊化黄道释放 (Zodiacal Releasing) 与七政小限核心推运引擎
// 纯 Rust 高精度零依赖实现，基于幸运点 (Lot of Fortune) 与精神点各星座年限

#[derive(Debug, Clone, serde::Serialize)]
pub struct ZrPeriod {
    pub sign: &'static str,
    pub lord: &'static str,
    pub years: u32,
    pub start_age: f64,
    pub end_age: f64,
    pub is_culmination: bool, // 是否到达天顶四轴峰期
}

// 希腊化各星座主星年限表 (ZR Years)
pub const ZR_YEARS_TABLE: [(&str, &str, u32); 12] = [
    ("白羊座", "火星", 15),
    ("金牛座", "金星", 8),
    ("双子座", "水星", 20),
    ("巨蟹座", "月亮", 25),
    ("狮子座", "太阳", 19),
    ("处女座", "水星", 20),
    ("天秤座", "金星", 8),
    ("天蝎座", "火星", 15),
    ("射手座", "木星", 12),
    ("摩羯座", "土星", 27),
    ("水瓶座", "土星", 30),
    ("双鱼座", "木星", 12),
];

pub fn get_sign_zr_info(sign_idx: usize) -> (&'static str, &'static str, u32) {
    ZR_YEARS_TABLE[sign_idx % 12]
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ZrResult {
    pub base_point: &'static str,       // 释放基点 (幸运点/精神点)
    pub start_sign: &'static str,       // 起始星座
    pub current_period: ZrPeriod,       // 当前所处大限周期
    pub major_periods: Vec<ZrPeriod>,   // 前期至未来各阶周期
    pub summary: &'static str,
}

/// 计算黄道释放 L1 大限周期
/// start_sign_idx: 起始星座索引 (0=白羊 ... 11=双鱼)
/// age: 当前年龄
pub fn calculate_zodiacal_releasing(
    start_sign_idx: usize,
    age: f64,
    point_name: &'static str,
) -> ZrResult {
    let mut periods = Vec::with_capacity(12);
    let mut cur_age = 0.0;
    let mut cur_period = None;

    for i in 0..12 {
        let idx = (start_sign_idx + i) % 12;
        let (sign, lord, y) = get_sign_zr_info(idx);
        let span = y as f64;
        let p = ZrPeriod {
            sign,
            lord,
            years: y,
            start_age: cur_age,
            end_age: cur_age + span,
            is_culmination: idx == 9 || idx == 0, // 摩羯/白羊四轴峰期
        };

        if age >= cur_age && age < cur_age + span {
            cur_period = Some(p.clone());
        }

        periods.push(p);
        cur_age += span;
    }

    let current = cur_period.unwrap_or_else(|| periods[0].clone());

    ZrResult {
        base_point: point_name,
        start_sign: periods[0].sign,
        current_period: current,
        major_periods: periods,
        summary: "黄道释放乃希腊化古典绝学，主看名誉、事业转折与生命高峰期之释放转关。",
    }
}
