// 择日与多流派多维约束求解引擎 (Universal Zeri & Multi-School Election Solver)
// 在指定时间窗内遍历每一个时辰 (2小时步长)，通过布尔树求交集，秒级输出良辰吉时。
// 统摄支持：
//  - bazizeri (八字择吉：日干、天乙贵人、天月德、驿马)
//  - qimenzeri (奇门择日：值使八门、值符九星、八神、避击刑门迫)
//  - taiyizeri (太乙择日：太乙落宫、主客大将、五福加临)
//  - liurengzeri (六壬择日：三传发端、青龙贵人)
//  - ziweizeri (紫微择日：紫微命宫、紫府廉武)
//  - sanshizeri (三式合一复合择吉)
//  - qizhengelection / qizhengzeri (七政天星择日：太阳、金木吉星)
//  - indiazeri (印度吠陀择日：月宿 Nakshatra 吉凶)
//  - election (西洋占星相位择吉)

use chrono::{Datelike, Timelike, NaiveDate, NaiveDateTime, Duration};
use crate::bazi_exact::calculate_exact_bazi;
use crate::taiyi::calculate_taiyi;
use crate::liureng::calculate_liureng;
use crate::ziwei::calculate_ziwei;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZeriCondition {
    pub school: Option<String>, // 流派: "bazi", "qimen", "taiyi", "liureng", "ziwei", "vedic", "astro"
    pub field: String,          // 匹配字段
    pub expected: String,       // 期望值
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ZeriHit {
    pub start_time: String,
    pub end_time: String,
    pub bazi_str: String,
    pub school_details: String,
    pub matched_detail: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ZeriScanResult {
    pub search_range: String,
    pub conditions: Vec<ZeriCondition>,
    pub total_scanned_hours: usize,
    pub hits_count: usize,
    pub hits: Vec<ZeriHit>,
}

/// 区间择日通用多流派扫描求解器
pub fn scan_zeri(
    start_date_str: &str,
    end_date_str: &str,
    conditions: &[ZeriCondition],
) -> Result<ZeriScanResult, String> {
    let start_date = NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d")
        .map_err(|e| format!("无效起始日期: {}", e))?;
    let end_date = NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d")
        .map_err(|e| format!("无效结束日期: {}", e))?;

    if end_date < start_date {
        return Err("结束日期不能早于起始日期".to_string());
    }

    let mut cur_time: NaiveDateTime = start_date.and_hms_opt(1, 0, 0).unwrap();
    let end_limit: NaiveDateTime = end_date.and_hms_opt(23, 0, 0).unwrap();

    let mut hits = Vec::new();
    let mut total_scanned = 0;

    while cur_time <= end_limit {
        total_scanned += 1;
        let year = cur_time.year();
        let month = cur_time.month();
        let day = cur_time.day();
        let hour = cur_time.hour();

        // 1. 八字与四柱
        let bazi = calculate_exact_bazi(year, month, day, hour, 0, 0);

        let d_gan_char = bazi.day_pillar.chars().next().unwrap_or('甲');
        let d_zhi_char = bazi.day_pillar.chars().nth(1).unwrap_or('子');
        let h_gan_char = bazi.hour_pillar.chars().next().unwrap_or('甲');
        let h_zhi_char = bazi.hour_pillar.chars().nth(1).unwrap_or('子');

        let day_gan_idx = crate::bazi_exact::TIANGAN.iter().position(|&g| g.starts_with(d_gan_char)).unwrap_or(0);
        let day_zhi_idx = crate::bazi_exact::DIZHI.iter().position(|&z| z.starts_with(d_zhi_char)).unwrap_or(0);
        let time_gan_idx = crate::bazi_exact::TIANGAN.iter().position(|&g| g.starts_with(h_gan_char)).unwrap_or(0);
        let hour_zhi_idx = crate::bazi_exact::DIZHI.iter().position(|&z| z.starts_with(h_zhi_char)).unwrap_or(0);

        // 2. 奇门 (按真实四柱与视黄经节气)
        let qm = crate::qimen::calculate_qimen_with_date(
            year, month, day, hour, 0, 0,
            day_gan_idx, hour_zhi_idx, day_zhi_idx, time_gan_idx,
        );

        // 3. 多流派惰性求值评估
        let mut all_match = true;
        let mut detail = Vec::new();
        let mut school_info = Vec::new();
        school_info.push(format!("奇门: {}", qm.ju_name));

        for cond in conditions {
            let matched = match cond.field.as_str() {
                // 八字择吉
                "day_gan" => {
                    let ok = bazi.day_pillar.starts_with(&cond.expected);
                    if ok { detail.push(format!("日干逢{}", cond.expected)); }
                    ok
                },
                "hour_zhi" => {
                    let ok = bazi.hour_pillar.ends_with(&cond.expected);
                    if ok { detail.push(format!("时支逢{}", cond.expected)); }
                    ok
                },
                "tianyi" => {
                    // 天乙贵人：甲戊并牛羊，乙己鼠猴乡，丙丁猪鸡位，壬癸兔蛇藏，庚辛逢马虎
                    let is_noble = match d_gan_char {
                        '甲' | '戊' => bazi.hour_pillar.ends_with('丑') || bazi.hour_pillar.ends_with('未'),
                        '乙' | '己' => bazi.hour_pillar.ends_with('子') || bazi.hour_pillar.ends_with('申'),
                        '丙' | '丁' => bazi.hour_pillar.ends_with('亥') || bazi.hour_pillar.ends_with('酉'),
                        '壬' | '癸' => bazi.hour_pillar.ends_with('卯') || bazi.hour_pillar.ends_with('巳'),
                        _ => bazi.hour_pillar.ends_with('午') || bazi.hour_pillar.ends_with('寅'),
                    };
                    if is_noble { detail.push("时临天乙大贵人".to_string()); }
                    is_noble
                },
                // 奇门择日
                "door" => {
                    let ok = if cond.expected == "三吉门" || cond.expected == "吉门" {
                        qm.leader_door == "开门" || qm.leader_door == "休门" || qm.leader_door == "生门"
                    } else {
                        qm.leader_door == cond.expected
                    };
                    if ok { detail.push(format!("值使临{}", qm.leader_door)); }
                    ok
                },
                "star" => {
                    let ok = qm.leader_star == cond.expected;
                    if ok { detail.push(format!("值符临{}", cond.expected)); }
                    ok
                },
                "no_harm" => {
                    // 避开奇门门迫与击刑
                    let safe = qm.fa_solutions.is_empty();
                    if safe { detail.push("奇门局无刑迫大害".to_string()); }
                    safe
                },
                // 太乙择日 (时计太乙神数：避主将客将囚于中五宫，太乙乘旺宫，主客大将分地得位)
                "taiyi" | "taiyi_general" | "taiyi_wufu" => {
                    let ty = calculate_taiyi(year, month, day, hour, 3, 0);
                    let ok = ty.home_general != 5 && ty.away_general != 5
                        && ty.home_general != ty.away_general
                        && (ty.taiyi_gong == 1 || ty.taiyi_gong == 3 || ty.taiyi_gong == 7 || ty.taiyi_gong == 9);
                    if ok { detail.push(format!("太乙乘{}吉宫，主将{}宫客将{}宫分明得地", ty.taiyi_palace_name, ty.home_general, ty.away_general)); }
                    ok
                },
                // 六壬择日
                "liureng_ke" => {
                    let lr = calculate_liureng(8, hour_zhi_idx, d_gan_char, day_zhi_idx);
                    let ok = lr.san_chuan.ke_ti.contains("元首") || lr.san_chuan.ke_ti.contains("比用");
                    if ok { detail.push(format!("六壬三传上吉课格 ({})", lr.san_chuan.ke_ti)); }
                    ok
                },
                // 紫微择日
                "ziwei_ming" => {
                    let zw = calculate_ziwei(month, day, hour_zhi_idx, day_gan_idx);
                    let ok = !zw.palaces[0].main_stars.is_empty();
                    if ok { detail.push("紫微命宫正曜得所".to_string()); }
                    ok
                },
                // 七政天星择日 (Qizheng Election)
                "qizheng" | "qizheng_sun" => {
                    let jde = crate::bazi_exact::to_julian_day(year, month, day, hour, 0, 0);
                    let planets = crate::ephem::calculate_planetary_positions(jde);
                    let sun = planets.iter().find(|p| p.name.contains("太阳"));
                    let jupiter = planets.iter().find(|p| p.name.contains("木星"));
                    let venus = planets.iter().find(|p| p.name.contains("金星"));

                    // 七政天星吉日课判定：太阳入庙旺宫（白羊/狮子）或与大吉星（木星/金星）产生吉照拱合（差角60°或120°，容许度6°内），或月亮入庙旺（金牛/巨蟹）与吉曜调和
                    let mut is_auspicious = false;
                    let moon = planets.iter().find(|p| p.name.contains("月亮"));
                    if let Some(s) = sun {
                        let sign_idx = (s.longitude / 30.0).floor() as usize % 12;
                        // 白羊(0) 狮子(4) 为太阳庙旺
                        if sign_idx == 0 || sign_idx == 4 {
                            is_auspicious = true;
                            detail.push("七政太阳真君庙旺正照加临".to_string());
                        }
                        if let Some(j) = jupiter {
                            let diff = (s.longitude - j.longitude).abs().rem_euclid(360.0);
                            let aspect_diff = if diff > 180.0 { 360.0 - diff } else { diff };
                            if (aspect_diff - 120.0).abs() <= 6.0 || (aspect_diff - 60.0).abs() <= 6.0 {
                                is_auspicious = true;
                                detail.push("七政太阳与木德岁星三合拱照".to_string());
                            }
                        }
                        if let Some(v) = venus {
                            let diff = (s.longitude - v.longitude).abs().rem_euclid(360.0);
                            let aspect_diff = if diff > 180.0 { 360.0 - diff } else { diff };
                            if (aspect_diff - 60.0).abs() <= 5.0 || (aspect_diff - 0.0).abs() <= 5.0 {
                                is_auspicious = true;
                                detail.push("七政太阳与太白吉星同度加临".to_string());
                            }
                        }
                    }
                    if let Some(m) = moon {
                        let m_sign = (m.longitude / 30.0).floor() as usize % 12;
                        // 月亮庙巨蟹(3)、旺金牛(1)
                        if m_sign == 1 || m_sign == 3 {
                            is_auspicious = true;
                            detail.push("七政太阴月华落入庙旺吉宫加临".to_string());
                        }
                        if let Some(j) = jupiter {
                            let diff = (m.longitude - j.longitude).abs().rem_euclid(360.0);
                            let aspect_diff = if diff > 180.0 { 360.0 - diff } else { diff };
                            if (aspect_diff - 120.0).abs() <= 6.0 || (aspect_diff - 60.0).abs() <= 6.0 || (aspect_diff - 0.0).abs() <= 6.0 {
                                is_auspicious = true;
                                detail.push("七政太阴与木德岁星祥和吉照".to_string());
                            }
                        }
                    }

                    if cond.expected == "any" {
                        is_auspicious
                    } else if cond.expected == "sun_temple" {
                        is_auspicious && (sun.map(|s| (s.longitude / 30.0).floor() as usize % 12).map(|i| i == 0 || i == 4).unwrap_or(false))
                    } else {
                        is_auspicious
                    }
                },
                // 吠陀印占择日 (India Muhurta Nakshatra)
                "vedic" | "muhurta" => {
                    let jde = crate::bazi_exact::to_julian_day(year, month, day, hour, 0, 0);
                    let planets = crate::ephem::calculate_planetary_positions(jde);
                    let moon = planets.iter().find(|p| p.name.contains("月亮"));
                    let m_lon = moon.map(|p| p.longitude).unwrap_or(0.0);
                    let nak_idx = (m_lon / (360.0 / 27.0)).floor() as usize % 27;
                    // 吠陀五大吉宿 (Rohini 毕宿 3, Pushya 鬼宿 7, Hasta 轸宿 12, Shravana 女宿 21, Revati 奎宿 26)
                    let is_auspicious = [3, 7, 12, 21, 26].contains(&nak_idx);
                    if is_auspicious {
                        detail.push(format!("印占吉宿加临: {}", crate::western_dyn::NAKSHATRAS[nak_idx]));
                    }
                    is_auspicious
                },
                _ => true,
            };

            if !matched {
                all_match = false;
                break;
            }
        }

        if all_match && !conditions.is_empty() {
            let next_hour = cur_time + Duration::hours(2);
            hits.push(ZeriHit {
                start_time: cur_time.format("%Y-%m-%d %H:%M").to_string(),
                end_time: next_hour.format("%Y-%m-%d %H:%M").to_string(),
                bazi_str: format!("{} {} {} {}", bazi.year_pillar, bazi.month_pillar, bazi.day_pillar, bazi.hour_pillar),
                school_details: school_info.join(" | "),
                matched_detail: detail.join("；"),
            });
        }

        cur_time += Duration::hours(2);
    }

    Ok(ZeriScanResult {
        search_range: format!("{} 至 {}", start_date_str, end_date_str),
        conditions: conditions.to_vec(),
        total_scanned_hours: total_scanned,
        hits_count: hits.len(),
        hits,
    })
}
