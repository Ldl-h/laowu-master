// 托勒密人生七阶行星年限 (Planetary Ages)
// P2-20: 将原 western.rs dispatch 层内联硬编码的七阶年龄段抽取为独立数据结构，
// dispatch 层只做调用，不改变任何输出结果。

/// 托勒密人生七阶主政行星年龄段表: (行星EN, 行星CN, 起岁, 止岁)
pub const PLANETARY_AGE_BANDS: [(&str, &str, f64, f64); 7] = [
    ("Moon", "月亮", 0.0, 4.0),
    ("Mercury", "水星", 4.0, 14.0),
    ("Venus", "金星", 14.0, 22.0),
    ("Sun", "太阳", 22.0, 41.0),
    ("Mars", "火星", 41.0, 56.0),
    ("Jupiter", "木星", 56.0, 68.0),
    ("Saturn", "土星", 68.0, 120.0),
];

/// 给定年龄，返回当前主政行星年龄段 (EN, CN, 起岁, 止岁)。
/// 超出表范围时回退到太阳主政段 (第四段)。
pub fn active_age_band(age: f64) -> &'static (&'static str, &'static str, f64, f64) {
    PLANETARY_AGE_BANDS
        .iter()
        .find(|b| age >= b.2 && age < b.3)
        .unwrap_or(&PLANETARY_AGE_BANDS[3])
}
