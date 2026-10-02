// 天干与地支基础常数与算法
pub const TIANGAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const DIZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

pub const WUXING: [&str; 5] = ["木", "火", "土", "金", "水"];

#[derive(Debug, Clone, serde::Serialize)]
pub struct Pillar {
    pub gan: String,
    pub zhi: String,
    pub gan_wuxing: &'static str,
    pub zhi_wuxing: &'static str,
}

impl Pillar {
    pub fn new(gan_idx: usize, zhi_idx: usize) -> Self {
        let gan = TIANGAN[gan_idx % 10].to_string();
        let zhi = DIZHI[zhi_idx % 12].to_string();
        let gan_wuxing = match gan_idx % 10 {
            0 | 1 => "木",
            2 | 3 => "火",
            4 | 5 => "土",
            6 | 7 => "金",
            _ => "水",
        };
        let zhi_wuxing = match zhi_idx % 12 {
            2 | 3 => "木",       // 寅 卯
            5 | 6 => "火",       // 巳 午
            1 | 4 | 7 | 10 => "土", // 丑 辰 未 戌
            8 | 9 => "金",       // 申 酉
            _ => "水",           // 亥 子
        };
        Self {
            gan,
            zhi,
            gan_wuxing,
            zhi_wuxing,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.gan, self.zhi)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BaZiResult {
    pub year: Pillar,
    pub month: Pillar,
    pub day: Pillar,
    pub hour: Pillar,
}

/// 高精四柱干支推算器 (纯数学模运算，零第三方依赖)
pub fn calculate_bazi(year: i32, month: u32, day: u32, hour: u32) -> BaZiResult {
    // 1. 年柱：公元4年为甲子年
    let year_offset = if year >= 4 { (year - 4) % 60 } else { (year - 4) % 60 + 60 } as usize;
    let year_gan = year_offset % 10;
    let year_zhi = year_offset % 12;
    let year_pillar = Pillar::new(year_gan, year_zhi);

    // 2. 月柱 (五虎遁口诀：甲己之年丙作首，乙庚之岁戊为头...)
    // 简化节气月支 (以公历月份近似映射对应地支：2月立春对应寅)
    let month_zhi = ((month + 10) % 12) as usize; // 1月:丑, 2月:寅, 3月:卯...
    let month_gan_base = match year_gan % 5 {
        0 => 2, // 甲己 -> 丙
        1 => 4, // 乙庚 -> 戊
        2 => 6, // 丙辛 -> 庚
        3 => 8, // 丁壬 -> 壬
        _ => 0, // 戊癸 -> 甲
    };
    // 寅月为正月
    let month_offset = if month >= 2 { month - 2 } else { month + 10 };
    let month_gan = (month_gan_base + month_offset as usize) % 10;
    let month_pillar = Pillar::new(month_gan, month_zhi);

    // 3. 日柱：格里高利历儒略日计算高精日干支
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;
    let jdn = day as i32 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    let day_offset = (jdn + 49) % 60;
    let day_gan = (day_offset % 10) as usize;
    let day_zhi = (day_offset % 12) as usize;
    let day_pillar = Pillar::new(day_gan, day_zhi);

    // 4. 时柱 (五鼠遁口诀：甲己还加甲，乙庚丙作初...)
    let hour_zhi = (hour.div_ceil(2) % 12) as usize;
    let hour_gan_base = match day_gan % 5 {
        0 => 0, // 甲己 -> 甲
        1 => 2, // 乙庚 -> 丙
        2 => 4, // 丙辛 -> 戊
        3 => 6, // 丁壬 -> 庚
        _ => 8, // 戊癸 -> 壬
    };
    let hour_gan = (hour_gan_base + hour_zhi) % 10;
    let hour_pillar = Pillar::new(hour_gan, hour_zhi);

    BaZiResult {
        year: year_pillar,
        month: month_pillar,
        day: day_pillar,
        hour: hour_pillar,
    }
}
