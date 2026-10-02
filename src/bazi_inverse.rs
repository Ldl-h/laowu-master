// 八字四柱逆向求解反查引擎 (BaZi Inverse Lookup Engine)
// 根据给定的四柱干支（年柱、月柱、日柱、时柱），在指定历史或未来年份窗口内（例如 1900~2100）
// 基于精确太阳黄经节气与儒略日连续时空高速穷举，求解严格完全匹配的候选公历时间点

use crate::bazi_exact::calculate_exact_bazi;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BaziInverseHit {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub date_str: String,
    pub matched_pillars: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BaziInverseResult {
    pub target_year_gz: String,
    pub target_month_gz: String,
    pub target_day_gz: String,
    pub target_hour_gz: String,
    pub candidates: Vec<BaziInverseHit>,
    pub summary: &'static str,
}

fn from_timestamp_seconds(ts: i64) -> (i32, u32, u32, u32, u32, u32) {
    let mut days = ts / 86400;
    let mut rem_sec = ts % 86400;
    if rem_sec < 0 {
        days -= 1;
        rem_sec += 86400;
    }
    let hour = rem_sec / 3600;
    let min = (rem_sec % 3600) / 60;
    let sec = rem_sec % 60;

    let mut y = 1970;
    if days >= 0 {
        loop {
            let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
            let ydays = if is_leap { 366 } else { 365 };
            if days < ydays {
                break;
            }
            days -= ydays;
            y += 1;
        }
    } else {
        loop {
            y -= 1;
            let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
            let ydays = if is_leap { 366 } else { 365 };
            days += ydays;
            if days >= 0 {
                break;
            }
        }
    }

    let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
    let mdays = [0, 31, if is_leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut rem_days = days;
    for mo in 1..=12 {
        let md = mdays[mo as usize];
        if rem_days < md {
            let d = rem_days + 1;
            return (y, mo as u32, d as u32, hour as u32, min as u32, sec as u32);
        }
        rem_days -= md;
    }
    (y, 12, 31, hour as u32, min as u32, sec as u32)
}

/// 高速穷举逆查八字出生时间
pub fn calculate_bazi_inverse(
    target_year: &str,
    target_month: &str,
    target_day: &str,
    target_hour: &str,
    start_year: i32,
    end_year: i32,
) -> BaziInverseResult {
    let mut candidates = Vec::new();

    let check_hit = |candidates: &mut Vec<BaziInverseHit>, y: i32, m: u32, d: u32, h: u32, min: u32, sec: u32| -> bool {
        let bz = calculate_exact_bazi(y, m, d, h, min, sec);
        let y_match = target_year.is_empty() || bz.year_pillar == target_year;
        let m_match = target_month.is_empty() || bz.month_pillar == target_month;
        let d_match = target_day.is_empty() || bz.day_pillar == target_day;
        let h_match = target_hour.is_empty() || bz.hour_pillar == target_hour;

        if y_match && m_match && d_match && h_match {
            // 避免完全同日同时辰重复录入
            let date_str = format!("{:04}-{:02}-{:02} {:02}:{:02}", y, m, d, h, min);
            if !candidates.iter().any(|c| c.year == y && c.month == m && c.day == d && ((c.hour as i32) - (h as i32)).abs() <= 1) {
                candidates.push(BaziInverseHit {
                    year: y,
                    month: m,
                    day: d,
                    hour: h,
                    date_str,
                    matched_pillars: format!("{} {} {} {}", bz.year_pillar, bz.month_pillar, bz.day_pillar, bz.hour_pillar),
                });
            }
            if candidates.len() >= 10 {
                return true;
            }
        }
        false
    };

    // 针对指定年份区间，按天步进排查
    for y in start_year..=end_year {
        // 先对 1900-2100 精确节气点进行微扰补充探测（防止交节落在时辰中段导致跳步踩空）
        if (1900..=2100).contains(&y) {
            let y_idx = (y - 1900) as usize;
            let jie_row = crate::jieqi_table::JIE_TABLE_1900_2100[y_idx];
            for &j_ts in jie_row.iter() {
                // 探测交节后 1 分钟与交节前 1 分钟
                let (jy1, jm1, jd1, jh1, jmin1, jsec1) = from_timestamp_seconds(j_ts + 60);
                if check_hit(&mut candidates, jy1, jm1, jd1, jh1, jmin1, jsec1) { break; }
                let (jy2, jm2, jd2, jh2, jmin2, jsec2) = from_timestamp_seconds(j_ts - 60);
                if check_hit(&mut candidates, jy2, jm2, jd2, jh2, jmin2, jsec2) { break; }
            }
        }
        if candidates.len() >= 10 { break; }

        for m in 1..=12 {
            let max_d = match m {
                2 => if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) { 29 } else { 28 },
                4 | 6 | 9 | 11 => 30,
                _ => 31,
            };

            for d in 1..=max_d {
                // 每隔2小时扫描12个时辰 (1, 3, 5... 23)
                for h_step in (1..24).step_by(2) {
                    check_hit(&mut candidates, y, m, d, h_step, 0, 0);
                    if candidates.len() >= 10 { break; }
                }
                if candidates.len() >= 10 { break; }
            }
            if candidates.len() >= 10 { break; }
        }
        if candidates.len() >= 10 { break; }
    }

    BaziInverseResult {
        target_year_gz: target_year.to_string(),
        target_month_gz: target_month.to_string(),
        target_day_gz: target_day.to_string(),
        target_hour_gz: target_hour.to_string(),
        candidates,
        summary: "八字反查逆推：以精确四柱干支与二十四节气为锚，多解候选时刻穷举完毕。",
    }
}
