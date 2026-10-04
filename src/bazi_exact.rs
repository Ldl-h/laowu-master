// 高精度二十四节气与八字干支排盘引擎
// 包含立春年界、节令换月（精准到节气分界）、儒略日日干支、五鼠遁时柱

pub const TIANGAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const DIZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

// 24节气标准黄经度数 (从春分0度开始: 春分=0, 清明=15, 谷雨=30, 立夏=45, 小满=60, 芒种=75, 夏至=90, 小暑=105, 大暑=120, 立秋=135, 处暑=150, 白露=165, 秋分=180, 寒露=195, 霜降=210, 立冬=225, 小雪=240, 大雪=255, 冬至=270, 小寒=285, 大寒=300, 立春=315, 雨水=330, 惊蛰=345)
// 12节（月界线）：立春(315°), 惊蛰(345°), 清明(15°), 立夏(45°), 芒种(75°), 小暑(105°), 立秋(135°), 白露(165°), 寒露(195°), 立冬(225°), 大雪(255°), 小寒(285°)

/// 太阳视黄经解析计算 (根据简明天文算法高精推导太阳视黄经 λ)
///
/// 修复说明 (P0-10)：原实现只给出太阳“真几何黄经”，缺少 光行差(-20.5")
/// 与 黄经章动(±17") 的视位置修正，导致节气入节时刻系统性提前约 10~12 分钟
/// （如 2026 立春实测 03:50 vs 权威 04:02）。此处补上两项修正后与
/// 预计算的 JIE_TABLE_1900_2100 节令表误差收敛到 1~2 分钟内。
pub fn sun_ecliptic_longitude(jdn_utc: f64) -> f64 {
    let t = (jdn_utc - 2451545.0) / 36525.0; // 儒略世纪数 J2000.0 起
    let mut l0 = 280.46646 + 36000.76983 * t + 0.0003032 * t * t;
    let mut m = 357.52911 + 35999.05029 * t - 0.0001537 * t * t;
    l0 = l0.rem_euclid(360.0);
    m = m.rem_euclid(360.0);
    let m_rad = m.to_radians();

    // 太阳中心差方程 C
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * m_rad.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * m_rad).sin()
        + 0.000289 * (3.0 * m_rad).sin();

    let mut true_long = l0 + c; // 太阳真几何黄经
    true_long = true_long.rem_euclid(360.0);

    // ── 视位置修正（依据 Meeus《Astronomical Algorithms》第22/25章）─────────────
    // 1) 光行差（Aberration of Light）：地球公转速度 ~29.8 km/s 导致恒星/太阳视位置
    //    沿地球轨道切线方向偏移 ~20.49″（常量主项，与日地距离弱相关）。
    //    公式：Δλ_aberration = -k / R，k=20.49552″（光行差常数），R 日地距离(AU)。
    //    此处取主项 -20.4898″（平均日地距离下的近似值）。
    let aberration_arcsec = -20.4898;
    // 2) 黄经章动（Nutation in Longitude, IAU 1980 Theory of Nutation）：
    //    月球引力使地球自转轴进动，黄经方向最大偏移 ±17.2″。
    //    此处采用 12 主项中最重要的三项近似：
    //    - Δψ₁ = -17.20″·sin(Ω)   （月球升交点黄经 Ω，最大项）
    //    - Δψ₂ = -1.32″·sin(2L☉)   （太阳平黄经二次项）
    //    - Δψ₃ =  0.21″·sin(2Ω)    （升交点二次项）
    //    Ω = 125.04452° - 1934.136261°·T （T = J2000起儒略世纪数）
    let omega = (125.04452 - 1934.136261 * t).rem_euclid(360.0);
    let nutation_arcsec = -17.20 * omega.to_radians().sin()
        - 1.32 * (2.0 * l0.to_radians()).sin()
        + 0.21 * (2.0 * omega.to_radians()).sin();

    true_long += (aberration_arcsec + nutation_arcsec) / 3600.0;
    true_long.rem_euclid(360.0)
}

/// 计算格里高利历日期时刻的儒略日数 (Julian Day Number)
pub fn to_julian_day(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> f64 {
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;
    let jdn_day = day as i32 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    let fraction = (hour as f64 - 12.0) / 24.0 + (minute as f64) / 1440.0 + (second as f64) / 86400.0;
    jdn_day as f64 + fraction
}

/// 高精度排盘输出
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExactBaZi {
    pub year_pillar: String,
    pub month_pillar: String,
    pub day_pillar: String,
    pub hour_pillar: String,
    pub sun_lon: f64,
}

pub fn calculate_exact_bazi(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> ExactBaZi {
    calculate_exact_bazi_with_switches(year, month, day, hour, minute, second, true, true)
}

/// 修复 P1-14：带时区偏移（小时，相对 UTC；北京时间应传 8.0）的八字排盘。
/// 其余参数与 calculate_exact_bazi 完全一致。
pub fn calculate_exact_bazi_tz(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, tz_offset_hours: f64) -> ExactBaZi {
    calculate_exact_bazi_with_switches_tz(year, month, day, hour, minute, second, true, true, tz_offset_hours)
}

fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

/// 儒略日（本地历法时刻，视作 UTC）转公历分解
pub fn jd_to_civil(jd_local: f64) -> (i32, u32, u32, u32, u32, u32) {
    let z = (jd_local + 0.5).floor() as i64;
    let f = (jd_local + 0.5) - z as f64;
    let mut l = z + 68569;
    let n = (4 * l) / 146097;
    l = l - (146097 * n + 3) / 4;
    let yr_idx = (4000 * (l + 1)) / 1461001;
    l = l - (1461 * yr_idx) / 4 + 31;
    let j = (80 * l) / 2447;
    let cal_day = (l - (2447 * j) / 80) as u32;
    l = j / 11;
    let cal_mon = (j + 2 - 12 * l) as u32;
    let cal_yr = (100 * (n - 49) + yr_idx + l) as i32;
    let tot_sec = (f * 86400.0).round() as u32;
    let cal_h = (tot_sec / 3600) as u32;
    let cal_min = ((tot_sec % 3600) / 60) as u32;
    let cal_sec = (tot_sec % 60) as u32;
    (cal_yr, cal_mon, cal_day, cal_h, cal_min, cal_sec)
}

/// 本地历法时间转朴素时间戳（与 jieqi_table.rs 同基准：把本地钟表时间按 UTC 秒计数，
/// 用于与节令表的同口径比较；公开供大运起运等模块复用）
pub fn to_timestamp_seconds(y: i32, m: u32, d: u32, h: u32, min: u32, sec: u32) -> i64 {
    let mut days: i64 = 0;
    if y >= 1970 {
        for yr in 1970..y {
            days += if is_leap_year(yr) { 366 } else { 365 };
        }
    } else {
        for yr in (y..1970).rev() {
            days -= if is_leap_year(yr) { 366 } else { 365 };
        }
    }
    let mdays = [0, 31, if is_leap_year(y) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for mo in 1..m {
        days += mdays[mo as usize];
    }
    days += (d as i64) - 1;
    days * 86400 + (h as i64) * 3600 + (min as i64) * 60 + (sec as i64)
}

/// 支持晚子时双开关的高精八字排盘（默认时区 UTC+8，保持历史行为）
pub fn calculate_exact_bazi_with_switches(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    after23_new_day: bool,
    late_zi_use_next_day: bool,
) -> ExactBaZi {
    // 修复 P1-14：历史调用方默认按北京时间 UTC+8 折算真太阳黄经
    calculate_exact_bazi_with_switches_tz(year, month, day, hour, minute, second, after23_new_day, late_zi_use_next_day, 8.0)
}

/// 支持晚子时双开关的高精八字排盘（可指定时区偏移 tz_offset_hours，小时，相对 UTC）
pub fn calculate_exact_bazi_with_switches_tz(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    after23_new_day: bool,
    late_zi_use_next_day: bool,
    tz_offset_hours: f64,
) -> ExactBaZi {
    let jdn_local = to_julian_day(year, month, day, hour, minute, second);
    // 修复 P1-14：本地钟表 JD → 真 UTC JD 的折算改为可配置时区偏移（原为硬编码 8.0）
    let jdn_utc = jdn_local - tz_offset_hours / 24.0;
    let sun_lon = sun_ecliptic_longitude(jdn_utc);

    let year_offset_base = if year >= 4 { (year - 4) % 60 } else { (year - 4) % 60 + 60 } as usize;

    let (year_pillar, month_pillar) = if (1900..=2100).contains(&year) {
        let ts = to_timestamp_seconds(year, month, day, hour, minute, second);
        let y_idx = (year - 1900) as usize;
        let jie_row = crate::jieqi_table::JIE_TABLE_1900_2100[y_idx];
        let lichun_ts = jie_row[1];

        let effective_year_offset = if ts < lichun_ts {
            (year_offset_base + 59) % 60
        } else {
            year_offset_base
        };
        let y_gan = effective_year_offset % 10;
        let y_zhi = effective_year_offset % 12;
        let y_p = format!("{}{}", TIANGAN[y_gan], DIZHI[y_zhi]);

        // 月柱节气判断：
        // 0:小寒, 1:立春, 2:惊蛰, 3:清明, 4:立夏, 5:芒种, 6:小暑, 7:立秋, 8:白露, 9:寒露, 10:立冬, 11:大雪
        // 对应月支：
        // 小寒前：上一年子月 (zhi = 0)
        // 小寒到立春前：丑月 (zhi = 1)
        // 立春到惊蛰前：寅月 (zhi = 2)
        // ...
        // 大雪后：当年子月 (zhi = 0)
        let (month_zhi_idx, month_step) = if ts < jie_row[0] {
            (0, 10) // 去年大雪到今年小寒之间为子月，寅起点偏移 10
        } else if ts < jie_row[1] {
            (1, 11) // 丑月，偏移 11
        } else if ts < jie_row[2] {
            (2, 0)  // 寅月，偏移 0
        } else if ts < jie_row[3] {
            (3, 1)  // 卯月，偏移 1
        } else if ts < jie_row[4] {
            (4, 2)  // 辰月，偏移 2
        } else if ts < jie_row[5] {
            (5, 3)  // 巳月，偏移 3
        } else if ts < jie_row[6] {
            (6, 4)  // 午月，偏移 4
        } else if ts < jie_row[7] {
            (7, 5)  // 未月，偏移 5
        } else if ts < jie_row[8] {
            (8, 6)  // 申月，偏移 6
        } else if ts < jie_row[9] {
            (9, 7)  // 酉月，偏移 7
        } else if ts < jie_row[10] {
            (10, 8) // 戌月，偏移 8
        } else if ts < jie_row[11] {
            (11, 9) // 亥月，偏移 9
        } else {
            (0, 10) // 当年大雪后为子月，偏移 10
        };

        // 五虎遁月干
        let month_gan_base = match y_gan % 5 {
            0 => 2, // 甲己 -> 丙
            1 => 4, // 乙庚 -> 戊
            2 => 6, // 丙辛 -> 庚
            3 => 8, // 丁壬 -> 壬
            _ => 0, // 戊癸 -> 甲
        };
        let m_gan = (month_gan_base + month_step) % 10;
        let m_p = format!("{}{}", TIANGAN[m_gan], DIZHI[month_zhi_idx]);
        (y_p, m_p)
    } else {
        // 回退机制：对于超出 1900-2100 的极端历史年份采用太阳视黄经连续积分
        let effective_year_offset = if sun_lon < 315.0 && month < 3 {
            (year_offset_base + 59) % 60
        } else {
            year_offset_base
        };
        let y_gan = effective_year_offset % 10;
        let y_zhi = effective_year_offset % 12;
        let y_p = format!("{}{}", TIANGAN[y_gan], DIZHI[y_zhi]);

        let month_zhi_idx = if (315.0..345.0).contains(&sun_lon) {
            2
        } else if !(15.0..345.0).contains(&sun_lon) {
            3
        } else if (15.0..45.0).contains(&sun_lon) {
            4
        } else if (45.0..75.0).contains(&sun_lon) {
            5
        } else if (75.0..105.0).contains(&sun_lon) {
            6
        } else if (105.0..135.0).contains(&sun_lon) {
            7
        } else if (135.0..165.0).contains(&sun_lon) {
            8
        } else if (165.0..195.0).contains(&sun_lon) {
            9
        } else if (195.0..225.0).contains(&sun_lon) {
            10
        } else if (225.0..255.0).contains(&sun_lon) {
            11
        } else if (255.0..285.0).contains(&sun_lon) {
            0
        } else {
            1
        };
        let month_gan_base = match y_gan % 5 {
            0 => 2,
            1 => 4,
            2 => 6,
            3 => 8,
            _ => 0,
        };
        let month_step = (month_zhi_idx + 10) % 12;
        let m_gan = (month_gan_base + month_step) % 10;
        let m_p = format!("{}{}", TIANGAN[m_gan], DIZHI[month_zhi_idx]);
        (y_p, m_p)
    };

    // 3. 日柱：根据格里高利历日儒略日连续排干支 (格林尼治正午分界修正 + 0.5)
    let day_jdn = (jdn_local + 0.5).floor() as i64;
    let base_day_offset = ((day_jdn + 49) % 60 + 60) % 60;
    // 晚子时开关 after23_new_day：若 hour == 23 且开关开启，日柱计入下一天
    let effective_day_offset = if hour == 23 && after23_new_day {
        (base_day_offset + 1) % 60
    } else {
        base_day_offset
    };
    let day_gan = (effective_day_offset % 10) as usize;
    let day_zhi = (effective_day_offset % 12) as usize;
    let day_pillar = format!("{}{}", TIANGAN[day_gan], DIZHI[day_zhi]);

    // 4. 时柱：五鼠遁时
    let hour_zhi_idx = (hour.div_ceil(2) % 12) as usize;
    // 晚子时时干开关 late_zi_use_next_day：若在 23 点且开启，时干依明天之干起遁
    let time_gan_source_day_gan = if hour == 23 && late_zi_use_next_day {
        ((base_day_offset + 1) % 10) as usize
    } else {
        (base_day_offset % 10) as usize
    };
    let hour_gan_base = match time_gan_source_day_gan % 5 {
        0 => 0, // 甲己 -> 甲
        1 => 2, // 乙庚 -> 丙
        2 => 4, // 丙辛 -> 戊
        3 => 6, // 丁壬 -> 庚
        _ => 8, // 戊癸 -> 壬
    };
    let hour_gan = (hour_gan_base + hour_zhi_idx) % 10;
    let hour_pillar = format!("{}{}", TIANGAN[hour_gan], DIZHI[hour_zhi_idx]);

    ExactBaZi {
        year_pillar,
        month_pillar,
        day_pillar,
        hour_pillar,
        sun_lon,
    }
}
