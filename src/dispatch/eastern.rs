use serde_json::Value;
use crate::dispatcher::UniversalInput;

/// 东方术数与三式核心分发处理器 (八字、紫微、奇门、太乙、六壬、金口诀、三式合一)
pub fn handle_eastern(tool: &str, input: &UniversalInput) -> Option<Result<Value, String>> {
    let res = match tool {
        // 八字
        "bazi" | "bazi_birth" | "bazi_direct" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let after23 = input.after23_new_day
                .or_else(|| input.params.as_ref().and_then(|p| p.get("after23NewDay")?.as_bool()))
                .or_else(|| input.options.as_ref().and_then(|o| o.get("after23NewDay")?.as_bool()))
                .unwrap_or(true);
            let late_zi = input.late_zi_use_next_day
                .or_else(|| input.params.as_ref().and_then(|p| p.get("lateZiHourUseNextDay")?.as_bool()))
                .or_else(|| input.options.as_ref().and_then(|o| o.get("lateZiHourUseNextDay")?.as_bool()))
                .unwrap_or(true);
            let g = input.gender.unwrap_or(1) as u8;
            let bazi = crate::bazi::calculate_bazi_full(y, m, d, h, min, sec, g, after23, late_zi);
            let exact = crate::bazi_exact::calculate_exact_bazi_with_switches(y, m, d, h, min, sec, after23, late_zi);
            match serde_json::to_value(&bazi) {
                Ok(mut val) => {
                    val["gender"] = serde_json::json!(if g == 1 { "乾造 (男)" } else { "坤造 (女)" });
                    val["year_pillar"] = serde_json::json!(exact.year_pillar);
                    val["month_pillar"] = serde_json::json!(exact.month_pillar);
                    val["day_pillar"] = serde_json::json!(exact.day_pillar);
                    val["hour_pillar"] = serde_json::json!(exact.hour_pillar);
                    val["sun_lon"] = serde_json::json!(exact.sun_lon);
                    val["four_pillars"] = serde_json::json!([
                        exact.year_pillar, exact.month_pillar, exact.day_pillar, exact.hour_pillar
                    ]);
                    Ok(val)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        "bazi_inverse" => {
            let get_part = |k1: Option<&str>, k2: Option<&str>, p_key: &str| -> Option<String> {
                k1.or(k2)
                    .map(|s| s.to_string())
                    .or_else(|| {
                        input.params.as_ref()
                            .and_then(|p| p.get(p_key).or_else(|| p.get(&format!("{}_pillar", p_key))).or_else(|| p.get(&format!("{}_gz", p_key))))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                    })
            };

            let (yg, mg, dg, hg) = if let Some(ref pils) = input.pillars {
                if pils.len() < 4 {
                    return Some(Err("八字反查 pillars 参数必须包含完整四柱".to_string()));
                }
                (pils[0].clone(), pils[1].clone(), pils[2].clone(), pils[3].clone())
            } else {
                let yg = get_part(input.year_gz.as_deref(), input.year_pillar.as_deref(), "year");
                let mg = get_part(input.month_gz.as_deref(), input.month_pillar.as_deref(), "month");
                let dg = get_part(input.day_gz.as_deref(), input.day_pillar.as_deref(), "day");
                let hg = get_part(input.hour_gz.as_deref(), input.hour_pillar.as_deref(), "hour");
                match (yg, mg, dg, hg) {
                    (Some(y), Some(m), Some(d), Some(h)) => (y, m, d, h),
                    _ => return Some(Err("八字反查必须提供完整的 year_gz, month_gz, day_gz, hour_gz 四柱参数".to_string())),
                }
            };

            let sy = input.start_year
                .or_else(|| input.params.as_ref().and_then(|p| p.get("start_year").or_else(|| p.get("startYear"))).and_then(|v| v.as_i64()).map(|v| v as i32))
                .unwrap_or(1900);
            let ey = input.end_year
                .or_else(|| input.params.as_ref().and_then(|p| p.get("end_year").or_else(|| p.get("endYear"))).and_then(|v| v.as_i64()).map(|v| v as i32))
                .unwrap_or(2100);

            let inv = crate::bazi_inverse::calculate_bazi_inverse(&yg, &mg, &dg, &hg, sy, ey);
            serde_json::to_value(inv).map_err(|e| e.to_string())
        }

        "ziwei" | "ziwei_birth" | "ziwei_rules" => {
            let (y, m, d, h, _, _) = input.get_datetime();
            let (target_lm, target_ld) = if input.is_lunar == Some(true) || input.lunar_month.is_some() {
                let lm = input.lunar_month.or(input.month).unwrap_or(m);
                let ld = input.lunar_day.or(input.day).unwrap_or(d);
                (lm, ld)
            } else {
                let (_ly, mut lm, ld, leap) = crate::lunar_table::solar_to_lunar(y, m, d);
                if leap && ld >= 16 {
                    lm = if lm >= 12 { 1 } else { lm + 1 };
                }
                (lm, ld)
            };

            let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, 0, 0);
            let y_gan_char = bazi.year_pillar.chars().next().unwrap_or('甲');
            let y_zhi_char = bazi.year_pillar.chars().nth(1).unwrap_or('子');
            let h_zhi_char = bazi.hour_pillar.chars().nth(1).unwrap_or('子');
            let y_gan_idx = crate::ziwei::GAN.iter().position(|&x| x.starts_with(y_gan_char)).unwrap_or(0);
            let y_zhi_idx = crate::ziwei::ZHI.iter().position(|&x| x.starts_with(y_zhi_char)).unwrap_or(0);
            let hour_zhi_idx = crate::ziwei::ZHI.iter().position(|&x| x.starts_with(h_zhi_char)).unwrap_or(0);
            let is_male = input.gender.unwrap_or(1) != 2;

            let zw = crate::ziwei::calculate_ziwei_full(target_lm, target_ld, hour_zhi_idx, y_gan_idx, y_zhi_idx, is_male);
            serde_json::to_value(zw).map_err(|e| e.to_string())
        }

        "qimen" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let ppt = input.pai_pan_type
                .or_else(|| input.options.as_ref().and_then(|o| o.get("paiPanType")?.as_u64()).map(|v| v as u32));
            if ppt == Some(0) {
                let qm = crate::qimen::calculate_qimen_nianjia(y, h);
                serde_json::to_value(qm).map_err(|e| e.to_string())
            } else {
                let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
                let d_gan_char = bazi.day_pillar.chars().next().unwrap_or('甲');
                let d_zhi_char = bazi.day_pillar.chars().nth(1).unwrap_or('子');
                let h_gan_char = bazi.hour_pillar.chars().next().unwrap_or('甲');
                let h_zhi_char = bazi.hour_pillar.chars().nth(1).unwrap_or('子');

                let day_gan_idx = crate::qimen::TIANGAN.iter().position(|&x| x.starts_with(d_gan_char)).unwrap_or(0);
                let day_zhi_idx = crate::qimen::DIZHI.iter().position(|&x| x.starts_with(d_zhi_char)).unwrap_or(0);
                let time_gan_idx = crate::qimen::TIANGAN.iter().position(|&x| x.starts_with(h_gan_char)).unwrap_or(0);
                let hour_zhi_idx = crate::qimen::DIZHI.iter().position(|&x| x.starts_with(h_zhi_char)).unwrap_or(0);

                let qm = crate::qimen::calculate_qimen_with_date(
                    y, m, d, h, min, sec,
                    day_gan_idx, hour_zhi_idx, day_zhi_idx, time_gan_idx
                );
                serde_json::to_value(qm).map_err(|e| e.to_string())
            }
        }

        // 太乙神数族
        "taiyi" => {
            let (y, m, d, h, _, _) = input.get_datetime();
            let style = input.style
                .or_else(|| input.options.as_ref().and_then(|o| o.get("style")?.as_u64()).map(|v| v as u8))
                .unwrap_or(3);
            let ty = crate::taiyi::calculate_taiyi(y, m, d, h, style, 0);
            serde_json::to_value(ty).map_err(|e| e.to_string())
        }

        // 大六壬金口诀
        "jinkou" => {
            let (_, _, _, h, _, _) = input.get_datetime();
            let day_gan = input.day_gan.as_deref().and_then(|s| s.chars().next()).unwrap_or('甲');
            let hour_zhi = input.hour_zhi.unwrap_or(h as usize % 12);
            let yue_jiang = input.yue_jiang.unwrap_or(8);
            let di_fen = input.di_fen.unwrap_or(2);
            let jk = crate::jinkou::calculate_jinkou(day_gan, hour_zhi, yue_jiang, di_fen, true);
            serde_json::to_value(jk).map_err(|e| e.to_string())
        }

        // 大六壬正宗
        "liureng" | "liureng_gods" => {
            let (_, _, _, h, _, _) = input.get_datetime();
            let yue_jiang = input.yue_jiang.unwrap_or(8);
            let zhan_shi = input.zhan_shi.unwrap_or(if input.hour.is_some() { h as usize % 12 } else { 6 });
            let day_gan = input.day_gan.as_deref().and_then(|s| s.chars().next()).unwrap_or('甲');
            let day_zhi = input.day_zhi.unwrap_or(0);
            let lr = crate::liureng::calculate_liureng(yue_jiang, zhan_shi, day_gan, day_zhi);
            serde_json::to_value(lr).map_err(|e| e.to_string())
        }
        "liureng_runyear" => {
            let age = input.age.unwrap_or(30.0) as u32;
            let gender = input.gender.unwrap_or(1);
            let ext = crate::liureng_ext::calculate_liureng_runyear(0, age, gender == 1, &[]);
            serde_json::to_value(ext).map_err(|e| e.to_string())
        }

        // 三式合一
        "sanshiunited" => {
            let (y, m, d, h, _, _) = input.get_datetime();
            let ssu = crate::composite::calculate_sanshi_united(y, m, d, h);
            serde_json::to_value(ssu).map_err(|e| e.to_string())
        }

        _ => return None,
    };
    Some(res)
}
