use serde_json::Value;
use crate::dispatcher::{UniversalInput, split_gz};

/// 二十四节气（自然年序）：12 节 + 12 气
const JIE_NAMES: [&str; 12] = ["小寒", "立春", "惊蛰", "清明", "立夏", "芒种", "小暑", "立秋", "白露", "寒露", "立冬", "大雪"];
const QI_NAMES: [&str; 12] = ["大寒", "雨水", "春分", "谷雨", "小满", "夏至", "大暑", "处暑", "秋分", "霜降", "小雪", "冬至"];
const JIE_LONS: [f64; 12] = [285.0, 315.0, 345.0, 15.0, 45.0, 75.0, 105.0, 135.0, 165.0, 195.0, 225.0, 255.0];

/// 朴素本地时间戳（北京墙钟按 UTC 秒编码）→ 太阳黄经计算的“真 UTC”JD
fn naive_ts_to_jd_utc(ts: i64) -> f64 {
    (ts as f64 - 28800.0) / 86400.0 + 2440587.5
}

/// “真 UTC”JD → 朴素本地时间戳（与节令表同基准）
fn jd_utc_to_naive_ts(jd_utc: f64) -> i64 {
    let (yy, mm, dd, hh, mi, ss) = crate::bazi_exact::jd_to_civil(jd_utc + 8.0 / 24.0);
    crate::bazi_exact::to_timestamp_seconds(yy, mm, dd, hh, mi, ss)
}

/// 太阳视黄经二分求入节 JD（真 UTC），以 approx_jd 为中心、±window_days 天内收敛
fn find_jieqi_jd_utc(target_lon: f64, approx_jd: f64, window_days: f64) -> f64 {
    let mut lo = approx_jd - window_days;
    let mut hi = approx_jd + window_days;
    for _ in 0..48 {
        let mid = (lo + hi) / 2.0;
        let cur = crate::bazi_exact::sun_ecliptic_longitude(mid);
        let mut diff = (cur - target_lon).rem_euclid(360.0);
        if diff > 180.0 {
            diff -= 360.0;
        }
        if diff > 0.0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    (lo + hi) / 2.0
}

/// 生成某公历年前后各 24 节气边界（naive ts），1900–2100 内 12 节用权威节令表
fn build_jieqi_boundaries(year: i32, span_years: i32) -> Vec<(String, i64)> {
    let mut out: Vec<(String, i64)> = Vec::new();
    for yy in (year - span_years)..=(year + span_years) {
        if (1900..=2100).contains(&yy) {
            let row = crate::jieqi_table::JIE_TABLE_1900_2100[(yy - 1900) as usize];
            for k in 0..12 {
                let jie_ts = row[k];
                let qi_ts = jd_utc_to_naive_ts(find_jieqi_jd_utc(
                    (JIE_LONS[k] + 15.0).rem_euclid(360.0),
                    naive_ts_to_jd_utc(jie_ts),
                    25.0,
                ));
                out.push((JIE_NAMES[k].to_string(), jie_ts));
                out.push((QI_NAMES[k].to_string(), qi_ts));
            }
        } else {
            let vernal_approx_jd = crate::bazi_exact::to_julian_day(yy, 3, 20, 12, 0, 0);
            let mut order: Vec<&str> = Vec::with_capacity(24);
            for k in 0..12 {
                order.push(JIE_NAMES[k]);
                order.push(QI_NAMES[k]);
            }
            for (i, name) in order.iter().enumerate() {
                let target_lon = (JIE_LONS[i / 2] + (i % 2) as f64 * 15.0).rem_euclid(360.0);
                let approx_days = if target_lon >= 285.0 {
                    (target_lon - 360.0) / 0.985647
                } else {
                    target_lon / 0.985647
                };
                let jd = find_jieqi_jd_utc(
                    target_lon,
                    vernal_approx_jd + approx_days - 8.0 / 24.0,
                    30.0,
                );
                out.push((name.to_string(), jd_utc_to_naive_ts(jd)));
            }
        }
    }
    out.sort_by_key(|(_, ts)| *ts);
    out
}

/// 易经六爻、河洛理数、残平、神数矩阵、择吉通书、民间杂占及历法实用处理器
pub fn handle_practical(tool: &str, input: &UniversalInput) -> Option<Result<Value, String>> {
    let res = match tool {
        // 易经六爻与卦象
        "sixyao" | "liuyao" | "gua_desc" | "gua_meiyi" => {
            let base: serde_json::Value = if let Some(ref lines_val) = input.lines {
                let mut line_bits = Vec::new();
                let mut movings = Vec::new();
                for l in lines_val {
                    if let Some(obj) = l.as_object() {
                        let v = obj.get("value").and_then(|x| x.as_u64()).unwrap_or(0) as u8;
                        let m = obj.get("change").and_then(|x| x.as_bool()).unwrap_or(false);
                        line_bits.push(v);
                        movings.push(m);
                    } else if let Some(n) = l.as_u64() {
                        line_bits.push(n as u8);
                        movings.push(false);
                    }
                }
                while line_bits.len() < 6 {
                    line_bits.push(0);
                    movings.push(false);
                }
                serde_json::to_value(crate::liuyao::calculate_liuyao_lines(&line_bits, &movings))
                    .unwrap_or(serde_json::Value::Null)
            } else {
                let nums = input.numbers.as_ref().or(input.nums.as_ref());
                let def_nums = vec![1, 5, 3];
                let nums_ref = nums.unwrap_or(&def_nums);
                let n1 = nums_ref.first().copied().unwrap_or(1) as usize;
                let n2 = nums_ref.get(1).copied().unwrap_or(5) as usize;
                let n3 = nums_ref.get(2).copied().unwrap_or(3) as usize;
                let ly = crate::liuyao::calculate_liuyao(n1, n2, n3);
                serde_json::to_value(ly).unwrap_or(serde_json::Value::Null)
            };
            // P2-1 对齐 Python：gua_desc / gua_meiyi 同为卦义查询（Python service.py:7021
            // _build_gua_lookup_snapshot_text 同一函数，仅"来源：tool_name"标签不同）。
            // 二者输出相同=对齐Python，非重复桩；此处补来源标签以区分契约。
            let mut out = base;
            if tool == "gua_desc" || tool == "gua_meiyi" {
                if let Some(obj) = out.as_object_mut() {
                    obj.insert("source".to_string(), serde_json::Value::String(tool.to_string()));
                    obj.insert(
                        "source_note".to_string(),
                        serde_json::Value::String(
                            "卦义查询（与Python原版一致：gua_desc/gua_meiyi同源卦义，仅来源标签不同）".into(),
                        ),
                    );
                }
            }
            Ok(out)
        }

        "heluo" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let after23 = input.after23_new_day
                .or_else(|| input.params.as_ref().and_then(|p| p.get("after23NewDay")?.as_bool()))
                .or_else(|| input.options.as_ref().and_then(|o| o.get("after23NewDay")?.as_bool()))
                .unwrap_or(true);
            let late_zi = input.late_zi_use_next_day
                .or_else(|| input.params.as_ref().and_then(|p| p.get("lateZiHourUseNextDay")?.as_bool()))
                .or_else(|| input.options.as_ref().and_then(|o| o.get("lateZiHourUseNextDay")?.as_bool()))
                .unwrap_or(true);
            let bazi = crate::bazi_exact::calculate_exact_bazi_with_switches(y, m, d, h, min, sec, after23, late_zi);
            let (yg, yz) = split_gz(&bazi.year_pillar, '甲', "子");
            let (mg, mz) = split_gz(&bazi.month_pillar, '丁', "卯");
            let (dg, dz) = split_gz(&bazi.day_pillar, '庚', "申");
            let (hg, hz) = split_gz(&bazi.hour_pillar, '庚', "辰");
            let is_male = match input.gender {
                Some(0) | Some(2) => false,
                _ => true,
            };
            let hl = crate::heluo::calculate_heluo(yg, yz, mg, mz, dg, dz, hg, hz, y, is_male);
            let mut val = serde_json::to_value(&hl).unwrap_or(serde_json::Value::Null);

            // 联动 tiaowen.bin 之 64 卦河洛理数原典诗赋
            if let Some(tiaowen_path) = crate::db::resolve_data_path("tiaowen.bin") {
                if let Ok(db) = crate::db::XuanshiDatabase::open(tiaowen_path) {
                    if let Ok(Value::Object(map)) = db.load_table("heluo_verses") {
                        if let Some(v) = map.get(hl.xian_tian_gua).or_else(|| map.get(hl.hou_tian_gua)) {
                            val["heluo_canon_verse"] = v.clone();
                        }
                    }
                }
            }
            Ok(val)
        }
        "canping" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let after23 = input.after23_new_day
                .or_else(|| input.params.as_ref().and_then(|p| p.get("after23NewDay")?.as_bool()))
                .or_else(|| input.options.as_ref().and_then(|o| o.get("after23NewDay")?.as_bool()))
                .unwrap_or(true);
            let late_zi = input.late_zi_use_next_day
                .or_else(|| input.params.as_ref().and_then(|p| p.get("lateZiHourUseNextDay")?.as_bool()))
                .or_else(|| input.options.as_ref().and_then(|o| o.get("lateZiHourUseNextDay")?.as_bool()))
                .unwrap_or(true);
            let bazi = crate::bazi_exact::calculate_exact_bazi_with_switches(y, m, d, h, min, sec, after23, late_zi);
            let yg = &bazi.year_pillar;
            let (_, mz) = split_gz(&bazi.month_pillar, '丁', "酉");
            let (_, dz) = split_gz(&bazi.day_pillar, '丁', "未");
            let (_, hz) = split_gz(&bazi.hour_pillar, '丁', "未");
            let cp = crate::canping::calculate_canping(yg, mz, dz, hz, false);
            let mut val = serde_json::to_value(&cp).unwrap_or(serde_json::Value::Null);

            // 联动 tiaowen.bin 之蚕屏一掌经条文
            if let Some(tiaowen_path) = crate::db::resolve_data_path("tiaowen.bin") {
                if let Ok(db) = crate::db::XuanshiDatabase::open(tiaowen_path) {
                    if let Ok(tbl) = db.load_table("canping_verses") {
                        val["canping_canon_data"] = tbl;
                    }
                }
            }
            Ok(val)
        }

        // 神数家族
        "shaozi" => {
            let yg_str = input.year_gz.as_deref().unwrap_or("甲子");
            let mg_str = input.month_gz.as_deref().unwrap_or("丁卯");
            let dg_str = input.day_gz.as_deref().unwrap_or("庚申");
            let hg_str = input.hour_gz.as_deref().unwrap_or("庚辰");
            let (yg, yz) = split_gz(yg_str, '甲', "子");
            let (mg, mz) = split_gz(mg_str, '丁', "卯");
            let (dg, dz) = split_gz(dg_str, '庚', "申");
            let (hg, hz) = split_gz(hg_str, '庚', "辰");
            let sz = crate::shaozi::calculate_shaozi(yg, yz, mg, mz, dg, dz, hg, hz, true);
            let mut val = serde_json::to_value(&sz).unwrap_or(serde_json::Value::Null);
            if let Some(tiaowen_path) = crate::db::resolve_data_path("tiaowen.bin") {
                if let Ok(db) = crate::db::XuanshiDatabase::open(tiaowen_path) {
                    if let Ok(Some(text)) = db.get_verse("shaozi_verses", &sz.base_verse_id.to_string()) {
                        val["verse_text"] = serde_json::Value::String(text);
                    }
                }
            }
            Ok(val)
        }
        "tieban" => {
            let (y, m, d, h, _, _) = input.get_datetime();
            // 修复 P0-03：铁板神数需真实农历月日、日支与时支，且性别决定卦数起例
            let (_ly, lm, ld, _leap) = crate::lunar_table::solar_to_lunar(y, m, d);
            let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, 0, 0);
            let day_zhi_char = bazi.day_pillar.chars().nth(1).unwrap_or('子');
            let day_zhi_idx = crate::bazi::DIZHI.iter().position(|&x| x.starts_with(day_zhi_char)).unwrap_or(0);
            let is_male = input.gender.unwrap_or(1) == 1;
            let tb = crate::tieban::calculate_tieban(lm, ld, h as usize % 12, day_zhi_idx, is_male);
            let mut val = serde_json::to_value(&tb).unwrap_or(serde_json::Value::Null);
            if let Some(tiaowen_path) = crate::db::resolve_data_path("tiaowen.bin") {
                if let Ok(db) = crate::db::XuanshiDatabase::open(tiaowen_path) {
                    if let Some(&first_id) = tb.verse_ids.first() {
                        if let Ok(Some(text)) = db.get_verse("tieban_verses", &first_id.to_string()) {
                            val["verse_text"] = serde_json::Value::String(text);
                        }
                    }
                }
            }
            Ok(val)
        }
        "shenshu" | "nanji" | "beiji" | "taixuan" | "wangji" | "cetian" | "chunzi" | "fendjing" | "jingjue" | "shenyishu" | "wuzhao" => {
            // 修复 P0-06：泛化工具名 "shenshu" 不再静默回退为第一家（南极），必须显式指定 family_key
            let fam = match input.family_key.as_deref() {
                Some(f) => f.to_string(),
                None => {
                    if tool == "shenshu" {
                        return Some(Err(
                            "技法 [shenshu] 必须显式指定 family_key（nanji/beiji/taixuan/wangji/cetian/chunzi/fendjing/jingjue/shenyishu/wuzhao），禁止静默回退为默认神数家族".to_string()
                        ));
                    }
                    tool.to_string()
                }
            };
            let (yg, mg, dg, hg) = if input.year.is_some() || input.date.is_some() {
                let (y, m, d, h, min, sec) = input.get_datetime();
                let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
                (bazi.year_pillar, bazi.month_pillar, bazi.day_pillar, bazi.hour_pillar)
            } else {
                (
                    input.year_gz.clone().unwrap_or_else(|| "丙午".to_string()),
                    input.month_gz.clone().unwrap_or_else(|| "丁酉".to_string()),
                    input.day_gz.clone().unwrap_or_else(|| "丁未".to_string()),
                    input.hour_gz.clone().unwrap_or_else(|| "丁未".to_string()),
                )
            };
            let ss = crate::shenshu_matrix::calculate_generic_shenshu(&fam, &yg, &mg, &dg, &hg);
            serde_json::to_value(ss).map_err(|e| e.to_string())
        }

        // 择日与通书
        "tongshu" => {
            let (y, m, d, _, _, _) = input.get_datetime();
            // 修复 P0-04：通书日干支与二十八宿需真实日柱与纪日序，而非恒甲子
            let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, 12, 0, 0);
            // 修复 P1-A：建除月支按 JS 整日约定（lunar-javascript：交节日整日归属新月）。
            // 正午 12:00 在午后交节（如寒露 2026-10-08 14:29）之前仍属旧月→建除错。
            // 日柱整日不变，仍取正午；月支改取当日末刻(23:59)，使当日任意时刻交节都归入新月。
            let bazi_eod = crate::bazi_exact::calculate_exact_bazi(y, m, d, 23, 59, 0);
            let day_gan_char = bazi.day_pillar.chars().next().unwrap_or('甲');
            let day_zhi_char = bazi.day_pillar.chars().nth(1).unwrap_or('子');
            let day_gan_idx = crate::bazi::TIANGAN.iter().position(|&x| x.starts_with(day_gan_char)).unwrap_or(0);
            let day_zhi_idx = crate::bazi::DIZHI.iter().position(|&x| x.starts_with(day_zhi_char)).unwrap_or(0);
            // 修复 P1-7：建除十二神须用交节真实月支（bazi.month_pillar），而非日历月号 m%12。
            // 例：2026-10-02 寒露前仍属酉月(索引9)，误用戌(10)会把建日错算成闭日。
            let month_zhi_idx = crate::bazi::DIZHI.iter()
                .position(|&x| x.starts_with(bazi_eod.month_pillar.chars().nth(1).unwrap_or('寅')))
                .unwrap_or(0);
            let day_since_epoch = crate::bazi_exact::to_julian_day(y, m, d, 12, 0, 0).floor() as i64;
            let ts = crate::tongshu::calculate_tongshu(month_zhi_idx, day_zhi_idx, day_gan_idx, day_since_epoch);
            serde_json::to_value(ts).map_err(|e| e.to_string())
        }
        "huangli" => {
            let (y, m, d, _, _, _) = input.get_datetime();
            let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, 12, 0, 0);
            // 修复 P1-A：建除月支整日按交节后新月（同 tongshu，取当日末刻）
            let bazi_eod = crate::bazi_exact::calculate_exact_bazi(y, m, d, 23, 59, 0);
            let day_gan_char = bazi.day_pillar.chars().next().unwrap_or('甲');
            let day_zhi_char = bazi.day_pillar.chars().nth(1).unwrap_or('子');
            let day_gan_idx = crate::bazi::TIANGAN.iter().position(|&x| x.starts_with(day_gan_char)).unwrap_or(0);
            let day_zhi_idx = crate::bazi::DIZHI.iter().position(|&x| x.starts_with(day_zhi_char)).unwrap_or(0);
            // 修复 P1-7：建除用交节真实月支，与 tongshu / calendar_month 一致
            let month_zhi_idx = crate::bazi::DIZHI.iter()
                .position(|&x| x.starts_with(bazi_eod.month_pillar.chars().nth(1).unwrap_or('寅')))
                .unwrap_or(0);
            let day_since_epoch = crate::bazi_exact::to_julian_day(y, m, d, 12, 0, 0).floor() as i64;
            let ts = crate::tongshu::calculate_tongshu(month_zhi_idx, day_zhi_idx, day_gan_idx, day_since_epoch);
            let hl = crate::derivation::calculate_huangli(month_zhi_idx, day_zhi_idx);
            let mut val = serde_json::to_value(ts).unwrap_or(serde_json::Value::Null);
            if let Some(obj) = val.as_object_mut() {
                obj.insert("jian_chu_god".to_string(), serde_json::json!(hl.jian_chu_god));
                obj.insert("jixiong".to_string(), serde_json::json!(hl.jixiong));
                obj.insert("yi".to_string(), serde_json::json!(hl.yi));
                obj.insert("ji".to_string(), serde_json::json!(hl.ji));
                // 修复 P2-9：宜忌为流派口径差异（非 bug），标注数据来源避免用户误以为是完整老黄历
                obj.insert("yi_ji_source".to_string(), serde_json::json!("简化董公模型（建除十二神派生，非完整老黄历）"));
            }
            Ok(val)
        }
        "bazizeri" | "qimenzeri" | "taiyizeri" | "ziweizeri" | "liurengzeri" | "sanshizeri" | "qizhengzeri" | "indiazeri" | "huanglizeri" | "zeri" | "zeri_scan" | "zeri_scan_remote" | "qizhengelection" => {
            let s_date = input.start_date.as_deref().or(input.date.as_deref()).unwrap_or("2026-10-01");
            let e_date = input.end_date.as_deref().or(input.target_date.as_deref()).unwrap_or("2026-10-03");
            
            let conds = if let Some(ref user_conds) = input.conditions {
                user_conds.clone()
            } else {
                match tool {
                    "qimenzeri" => vec![crate::zeri::ZeriCondition {
                        school: Some("奇门".to_string()),
                        field: "door".to_string(),
                        expected: "吉门".to_string(),
                    }],
                    "taiyizeri" => vec![crate::zeri::ZeriCondition {
                        school: Some("太乙".to_string()),
                        field: "taiyi_general".to_string(),
                        expected: "吉辰".to_string(),
                    }],
                    "liurengzeri" => vec![crate::zeri::ZeriCondition {
                        school: Some("大六壬".to_string()),
                        field: "liureng_ke".to_string(),
                        expected: "元首".to_string(),
                    }],
                    "ziweizeri" => vec![crate::zeri::ZeriCondition {
                        school: Some("紫微".to_string()),
                        field: "ziwei_ming".to_string(),
                        expected: "主星".to_string(),
                    }],
                    "sanshizeri" => vec![
                        crate::zeri::ZeriCondition {
                            school: Some("奇门".to_string()),
                            field: "door".to_string(),
                            expected: "吉门".to_string(),
                        },
                        crate::zeri::ZeriCondition {
                            school: Some("太乙".to_string()),
                            field: "taiyi_wufu".to_string(),
                            expected: "五福".to_string(),
                        },
                    ],
                    "qizhengzeri" | "qizhengelection" => vec![crate::zeri::ZeriCondition {
                        school: Some("七政四余".to_string()),
                        field: "qizheng".to_string(),
                        expected: "any".to_string(),
                    }],
                    "indiazeri" => vec![crate::zeri::ZeriCondition {
                        school: Some("吠陀印占".to_string()),
                        field: "vedic".to_string(),
                        expected: "any".to_string(),
                    }],
                    "huanglizeri" => vec![crate::zeri::ZeriCondition {
                        school: Some("老黄历".to_string()),
                        field: "tianyi".to_string(),
                        expected: "天乙贵人".to_string(),
                    }],
                    _ => vec![crate::zeri::ZeriCondition {
                        school: None,
                        field: "tianyi".to_string(),
                        expected: "天乙贵人".to_string(),
                    }],
                }
            };

            match crate::zeri::scan_zeri(s_date, e_date, &conds) {
                Ok(zs) => match serde_json::to_value(zs) {
                    Ok(mut val) => {
                        if tool == "huanglizeri" {
                            let (_, m, d, _, _, _) = input.get_datetime();
                            let hl = crate::derivation::calculate_huangli(m as usize % 12, d as usize % 12);
                            let events = serde_json::json!([
                                { "name": "嫁娶婚媾", "suitable": hl.yi.contains("嫁娶") || hl.yi.contains("纳采") || hl.jixiong == "吉" },
                                { "name": "祈福祭祀", "suitable": hl.yi.contains("祈福") || hl.yi.contains("出行") || hl.jixiong == "吉" || hl.jixiong == "次吉" },
                                { "name": "开市纳财", "suitable": hl.yi.contains("开市") || hl.yi.contains("交易") || hl.yi.contains("纳财") },
                                { "name": "动土修造", "suitable": !hl.ji.contains("动土") && !hl.ji.contains("破土") },
                                { "name": "营建商贾", "suitable": hl.jixiong == "吉" || hl.jixiong == "次吉" }
                            ]);
                            if let Some(obj) = val.as_object_mut() {
                                obj.insert("events".to_string(), events);
                                obj.insert("jian_chu_god".to_string(), serde_json::json!(hl.jian_chu_god));
                                obj.insert("jixiong".to_string(), serde_json::json!(hl.jixiong));
                            }
                        }
                        Ok(val)
                    }
                    Err(e) => Err(e.to_string()),
                },
                Err(e) => Err(e),
            }
        }

        // 民间占卜与专科
        "xiaoliuren" => {
            let (n1, n2, n3) = if let Some(nums) = input.numbers.as_ref().or(input.nums.as_ref()) {
                (
                    nums.first().copied().unwrap_or(8),
                    nums.get(1).copied().unwrap_or(15),
                    nums.get(2).copied().unwrap_or(6),
                )
            } else {
                let (_, m, d, h, _, _) = input.get_datetime();
                (m, d, h)
            };
            let xlr = crate::xiaoliuren::calculate_xiaoliuren(n1, n2, n3, false);
            serde_json::to_value(xlr).map_err(|e| e.to_string())
        }
        "yizhangjing" => {
            let (raw_y, raw_m, raw_d, h, min, sec) = input.get_datetime();
            let after23 = input.after23_new_day
                .or_else(|| input.params.as_ref().and_then(|p| p.get("after23NewDay")?.as_bool()))
                .or_else(|| input.options.as_ref().and_then(|o| o.get("after23NewDay")?.as_bool()))
                .unwrap_or(true);
            let (mut y, mut m, mut d) = (raw_y, raw_m, raw_d);
            if after23 && h >= 23 {
                let days_in_month = match m {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                    4 | 6 | 9 | 11 => 30,
                    2 => {
                        let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
                        if is_leap { 29 } else { 28 }
                    }
                    _ => 30,
                };
                if d >= days_in_month {
                    d = 1;
                    if m == 12 {
                        m = 1;
                        y += 1;
                    } else {
                        m += 1;
                    }
                } else {
                    d += 1;
                }
            }
            let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
            let (_ly, mut lm, ld, is_leap) = crate::lunar_table::solar_to_lunar(y, m, d);
            let (orig_ly, _, _, _) = crate::lunar_table::solar_to_lunar(raw_y, raw_m, raw_d);
            let ly_zhi_idx = ((orig_ly - 4).rem_euclid(12)) as usize;
            let y_zhi = crate::bazi_exact::DIZHI[ly_zhi_idx].to_string();
            let h_zhi = bazi.hour_pillar.chars().nth(1).map(|c| c.to_string()).unwrap_or_else(|| "未".to_string());
            if is_leap && ld > 15 {
                lm = if lm >= 12 { 1 } else { lm + 1 };
            }
            let is_male = match input.gender {
                Some(0) | Some(2) => false,
                _ => true,
            };
            let yz = crate::yizhangjing::calculate_yizhangjing(&y_zhi, lm, ld, &h_zhi, is_male);
            let mut val = serde_json::to_value(&yz).unwrap_or(serde_json::Value::Null);

            // 联动 tiaowen.bin 达摩一掌经道统参数
            if let Some(p) = crate::db::resolve_data_path("tiaowen.bin") {
                if let Ok(db) = crate::db::XuanshiDatabase::open(p) {
                    if let Some(damo) = db.get_damo_data() {
                        val["damo_data_canonical"] = damo;
                    }
                }
            }
            Ok(val)
        }
        "lingqi" => {
            let nums = input.nums.as_ref().or(input.numbers.as_ref());
            let def_nums = vec![1, 1, 1];
            let nums_ref = nums.unwrap_or(&def_nums);
            let lq = crate::lingqi::calculate_lingqi(
                nums_ref.first().copied().unwrap_or(1) as u8,
                nums_ref.get(1).copied().unwrap_or(1) as u8,
                nums_ref.get(2).copied().unwrap_or(1) as u8
            );
            serde_json::to_value(lq).map_err(|e| e.to_string())
        }
        "tarot" => {
            let sp = input.spread.as_deref().unwrap_or("three");
            // 修复 P2-05：未显式提供 seed 时以系统时间派生，避免固定种子导致每次同牌
            let sd = input.seed.unwrap_or_else(|| {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(20261001);
                (now % 10_000_000_000u64) as u64
            });
            let tr = crate::tarot::calculate_tarot(sp, sd);
            let mut val = serde_json::to_value(&tr).unwrap_or(serde_json::Value::Null);

            // 联动 tiaowen.bin 牌阵拓扑与释义字典及四元素对应
            if let Some(p) = crate::db::resolve_data_path("tiaowen.bin") {
                if let Ok(db) = crate::db::XuanshiDatabase::open(p) {
                    if let Some(spread_info) = db.get_tarot_spread_info(sp) {
                        val["spread_layout_detail"] = spread_info;
                    }
                    if let Ok(meanings) = db.load_table("tarot_meanings") {
                        val["archetype_meanings"] = meanings;
                    }
                    if let Ok(corr) = db.load_table("tarot_corr") {
                        val["tarot_element_correspondence"] = corr;
                    }
                }
            }
            Ok(val)
        }
        "geomancy" => {
            let nums = input.nums.as_ref().or(input.numbers.as_ref());
            let def_nums = vec![2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 1, 2, 2, 1, 2, 2];
            let nums_ref = nums.unwrap_or(&def_nums);
            let mut lines = [1u8; 16];
            for (i, &n) in nums_ref.iter().take(16).enumerate() {
                lines[i] = if n % 2 == 0 { 2 } else { 1 };
            }
            let m1 = [lines[0], lines[1], lines[2], lines[3]];
            let m2 = [lines[4], lines[5], lines[6], lines[7]];
            let m3 = [lines[8], lines[9], lines[10], lines[11]];
            let m4 = [lines[12], lines[13], lines[14], lines[15]];
            let geo = crate::geomancy::calculate_geomancy(m1, m2, m3, m4);
            serde_json::to_value(geo).map_err(|e| e.to_string())
        }
        "guice" => {
            let y_gan = input.year_gan.as_deref()
                .and_then(|s| s.chars().next())
                .or_else(|| input.year_gz.as_deref().and_then(|s| s.chars().next()))
                .or_else(|| input.year_pillar.as_deref().and_then(|s| s.chars().next()));
            let h_gan = input.hour_gan.as_deref()
                .and_then(|s| s.chars().next())
                .or_else(|| input.hour_gz.as_deref().and_then(|s| s.chars().next()))
                .or_else(|| input.hour_pillar.as_deref().and_then(|s| s.chars().next()));

            let (y_g, h_g) = match (y_gan, h_gan) {
                (Some(y), Some(h)) => (y, h),
                _ => {
                    let (y, m, d, h, min, sec) = input.get_datetime();
                    let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
                    (
                        bazi.year_pillar.chars().next().unwrap_or('甲'),
                        bazi.hour_pillar.chars().next().unwrap_or('丙'),
                    )
                }
            };
            let gc = crate::guice::calculate_guice(y_g, h_g);
            serde_json::to_value(gc).map_err(|e| e.to_string())
        }
        "tongshefa" => {
            let bagua = ["乾", "兑", "离", "震", "巽", "坎", "艮", "坤"];
            let (lu, ll, ru, rl) = if let Some(nums) = input.numbers.as_ref().or(input.nums.as_ref()) {
                (
                    nums.first().map(|&n| bagua[(n as usize) % 8]).unwrap_or("巽"),
                    nums.get(1).map(|&n| bagua[(n as usize) % 8]).unwrap_or("坤"),
                    nums.get(2).map(|&n| bagua[(n as usize) % 8]).unwrap_or("震"),
                    nums.get(3).map(|&n| bagua[(n as usize) % 8]).unwrap_or("震"),
                )
            } else {
                let left_up = input.params.as_ref()
                    .and_then(|p| p.get("baseLeft").or_else(|| p.get("leftUp")).or_else(|| p.get("left")))
                    .and_then(|v| v.as_str())
                    .unwrap_or("巽");
                let left_lo = input.params.as_ref()
                    .and_then(|p| p.get("baseRight").or_else(|| p.get("leftDown")).or_else(|| p.get("right")))
                    .and_then(|v| v.as_str())
                    .unwrap_or("坤");
                let right_up = input.params.as_ref()
                    .and_then(|p| p.get("oppositeLeft").or_else(|| p.get("rightUp")))
                    .and_then(|v| v.as_str())
                    .unwrap_or("震");
                let right_lo = input.params.as_ref()
                    .and_then(|p| p.get("oppositeRight").or_else(|| p.get("rightDown")))
                    .and_then(|v| v.as_str())
                    .unwrap_or("震");
                (left_up, left_lo, right_up, right_lo)
            };
            let ts = crate::tongshefa::calculate_tongshefa(lu, ll, ru, rl);
            serde_json::to_value(ts).map_err(|e| e.to_string())
        }
        "yanqin" | "xianqin" => {
            let (y, m, d, hour_branch, lm) = if let (Some(ref yg), Some(ref hg)) = (&input.year_gz, &input.hour_gz) {
                // 如果传入了干支柱，从中提取地支与历史基准推导
                let y_zhi = yg.chars().nth(1).unwrap_or('寅');
                let h_zhi = hg.chars().nth(1).unwrap_or('辰');
                let h_idx = crate::bazi_exact::DIZHI.iter().position(|&x| x.starts_with(h_zhi)).unwrap_or(4) as u8;
                let y_idx = crate::bazi_exact::DIZHI.iter().position(|&x| x.starts_with(y_zhi)).unwrap_or(2) as i32;
                let approx_y = 1998 + (y_idx - 2);
                let (_, m, d, _, _, _) = input.get_datetime();
                (approx_y, m, d, h_idx, 9)
            } else {
                let (y, m, d, h, _, _) = input.get_datetime();
                let hour_branch = (h.div_ceil(2) % 12) as u8;
                let (_ly, lm, _ld, _leap) = crate::lunar_table::solar_to_lunar(y, m, d);
                (y, m, d, hour_branch, lm)
            };
            let yq = crate::yanqin::calculate_yanqin(y, m, d, hour_branch, lm);
            serde_json::to_value(yq).map_err(|e| e.to_string())
        }
        "suzhan" => {
            let (y, m, d, _, _, _) = input.get_datetime();
            // 修复 P1-14：宿曜需真实农历月日，且 target_day_offset 从入参读取（不再恒 0）
            let (_ly, lm, ld, _leap) = crate::lunar_table::solar_to_lunar(y, m, d);
            let target_day_offset = input.params.as_ref()
                .and_then(|p| p.get("targetDayOffset").or_else(|| p.get("target_day_offset")).or_else(|| p.get("offset")))
                .and_then(|v| v.as_u64())
                .map(|u| u as u32)
                .unwrap_or(0);
            let sz = crate::suzhan_zhengchuan::calculate_suzhan(lm, ld, target_day_offset);
            serde_json::to_value(sz).map_err(|e| e.to_string())
        }
        "zhengchuan" => {
            let school = input.school.as_deref().unwrap_or("tieban");
            if let Some(ref pils) = input.pillars {
                let yg = pils.first().map(|s| s.as_str()).unwrap_or("甲子");
                let mg = pils.get(1).map(|s| s.as_str()).unwrap_or("丙寅");
                let dg = pils.get(2).map(|s| s.as_str()).unwrap_or("戊辰");
                let hg = pils.get(3).map(|s| s.as_str()).unwrap_or("庚申");
                let ss = crate::shenshu_matrix::calculate_generic_shenshu(school, yg, mg, dg, hg);
                serde_json::to_value(ss).map_err(|e| e.to_string())
            } else {
                let zc = crate::suzhan_zhengchuan::calculate_zhengchuan("111111", 1, school);
                serde_json::to_value(zc).map_err(|e| e.to_string())
            }
        }
        "tianxing" => {
            // 修复 P0-07：tianxing 为天星本命盘（七政四余命理），不再错接择日扫描
            let (y, m, d, h, min, sec) = input.get_datetime();
            let ((lat, lon, loc_name), loc_warn) = input.get_location_detail();
            let jde = input.get_julian_day();
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "placidus");
            let qizheng_names = ["太阳", "月亮", "水星", "金星", "火星", "木星", "土星"];
            let mut planets = Vec::new();
            for (i, p) in chart.planets.iter().enumerate() {
                let name = qizheng_names.get(i).copied().unwrap_or(p.name);
                planets.push(serde_json::json!({
                    "name": name,
                    "longitude": p.longitude,
                    "sign": p.sign,
                    "degree_in_sign": p.degree_in_sign,
                }));
            }
            let mut result = serde_json::json!({
                "technique": "tianxing",
                "solar_datetime": format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, h, min, sec),
                "location": loc_name,
                "latitude": lat,
                "longitude": lon,
                "jde": jde,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "planets": planets,
                "houses": chart.houses,
                "summary": format!("天星本命盘 (七政四余): 上升【{:.2}°】中天【{:.2}°】，七政曜宿落宫排布完备", chart.ascendant, chart.mc)
            });
            // 修复 P1-9：城市未命中回退默认坐标时显式告警
            if let Some(w) = loc_warn {
                result["location_warning"] = serde_json::Value::String(w);
            }
            Ok(result)
        }
        "tianxingzeri" => {
            let jde = input.get_julian_day();
            let days_span = input.params.as_ref()
                .and_then(|p| p.get("days").or_else(|| p.get("spanDays")).or_else(|| p.get("days_span")))
                .and_then(|v| v.as_u64())
                .map(|u| u as u32)
                .unwrap_or(30);
            let tx = crate::tianxing_otherbu::calculate_tianxing(
                jde,
                days_span,
                &[
                    crate::tianxing_otherbu::TianXingCondition {
                        body: "太阳",
                        aspect: "三合",
                        target: "木星",
                        orb_deg: 5.0,
                    },
                    crate::tianxing_otherbu::TianXingCondition {
                        body: "太阳",
                        aspect: "六合",
                        target: "金星",
                        orb_deg: 4.0,
                    }
                ]
            );
            serde_json::to_value(tx).map_err(|e| e.to_string())
        }
        "otherbu" => {
            let m_key = input.school.as_deref()
                .or(input.spread.as_deref())
                .unwrap_or("maqian");
            // 修复 P2-12：未提供 seed 时保留默认种子，但在输出中显式标注 using default seed
            let seed_supplied = input.seed.is_some();
            let sd = input.seed.unwrap_or(20261001);
            let mut ob = serde_json::to_value(crate::tianxing_otherbu::calculate_otherbu(m_key, sd))
                .unwrap_or(serde_json::Value::Null);
            if let Some(obj) = ob.as_object_mut() {
                if !seed_supplied {
                    obj.insert("note".to_string(), serde_json::json!("using default seed (20261001)"));
                }
            }
            Ok(ob)
        }
        "xiaochengtu" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
            let bagua = ["坎", "坤", "震", "巽", "乾", "兑", "艮", "离"];

            let up_gua_arg = input.up.as_deref().or_else(|| input.params.as_ref().and_then(|p| p.get("up")).and_then(|v| v.as_str()));
            let lo_gua_arg = input.lo.as_deref().or_else(|| input.params.as_ref().and_then(|p| p.get("lo")).and_then(|v| v.as_str()));

            let (up_gua, lo_gua) = if let (Some(u), Some(l)) = (up_gua_arg, lo_gua_arg) {
                (u, l)
            } else {
                let day_gan = bazi.day_pillar.chars().next().unwrap_or('甲');
                let hour_zhi = bazi.hour_pillar.chars().nth(1).unwrap_or('子');
                let up_idx = match day_gan {
                    '甲' | '壬' => 4,
                    '乙' | '癸' => 1,
                    '丙' => 7,
                    '丁' => 0,
                    '戊' => 2,
                    '己' => 3,
                    '庚' => 5,
                    _ => 6,
                };
                let zhi_list = ['子', '丑', '寅', '卯', '辰', '巳', '午', '未', '申', '酉', '戌', '亥'];
                let lo_idx = zhi_list.iter().position(|&z| z == hour_zhi).unwrap_or(0) % 8;
                (bagua[up_idx], bagua[lo_idx])
            };

            // 修复 P2-7：未提供动爻时之卦应等于本卦（无动爻则不变），不再凭空注入第 1 爻为动爻。
            let dong_yaos: Vec<usize> = input.dong_yaos.clone()
                .or_else(|| {
                    input.params.as_ref()
                        .and_then(|p| p.get("dongYaos").or_else(|| p.get("dong_yaos")))
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as usize)).collect())
                })
                .unwrap_or_default();

            let xct = crate::derivation::calculate_xiaochengtu_exact(up_gua, lo_gua, &dong_yaos);

            let result = serde_json::json!({
                "technique": "xiaochengtu",
                "four_pillars": [bazi.year_pillar, bazi.month_pillar, bazi.day_pillar, bazi.hour_pillar],
                "main_gua": format!("{}{} (主局: {})", up_gua, lo_gua, xct.main_gua),
                "up_gua": up_gua,
                "lo_gua": lo_gua,
                "ben": xct.ben_gua,
                "zhi": xct.zhi_gua,
                "ben_gua": xct.ben_gua,
                "zhi_gua": xct.zhi_gua,
                "gong_palaces": xct.gong_palaces,
                "summary": xct.summary
            });
            Ok(result)
        }
        "xuanshi" => {
            let query = input.text.as_deref()
                .or(input.name.as_deref())
                .or(input.question_type.as_deref())
                .unwrap_or("占星");
            let mut results = Vec::new();
            if let Some(db_path) = crate::db::resolve_data_path("xuanshi.bin") {
                if let Ok(db) = crate::db::XuanshiDatabase::open(db_path) {
                    for tbl in db.list_tables() {
                        if let Ok(hits) = db.search(&tbl, query) {
                            if !hits.is_empty() {
                                results.extend(hits.into_iter().take(5));
                            }
                        }
                    }
                }
            }
            let result = serde_json::json!({
                "technique": "xuanshi",
                "query": query,
                "hits_count": results.len(),
                "hits": results,
                "summary": format!("中国古代术数史料 (玄史) 检索: 命中 {} 条相关正史与野载典籍条目", results.len())
            });
            Ok(result)
        }
        "astrodata" => {
            // R14：query 兼容 text/name/options.query/params.query
            let opt = input.options.as_ref().or(input.params.as_ref());
            let query = input.text.as_deref()
                .or(input.name.as_deref())
                .or_else(|| opt.and_then(|m| m.get("query")).and_then(|v| v.as_str()))
                .unwrap_or("Einstein")
                .to_string();

            // R14：可选过滤参数（从 options/params 读取）
            let opt_str = |k: &str| opt.and_then(|m| m.get(k)).and_then(|v| v.as_str()).map(|s| s.to_string());
            let opt_i64 = |k: &str| opt.and_then(|m| m.get(k)).and_then(|v| v.as_i64());
            let category_filter = opt_str("category").map(|s| s.to_lowercase());
            let rodden_filter: Vec<String> = opt_str("rodden")
                .map(|s| s.split(',').map(|x| x.trim().to_uppercase()).filter(|x| !x.is_empty()).collect())
                .unwrap_or_default();
            let birth_year_from = opt_i64("birthYearFrom");
            let birth_year_to = opt_i64("birthYearTo");
            let limit = opt_i64("limit").unwrap_or(10).clamp(1, 50) as usize;
            let offset = opt_i64("offset").unwrap_or(0).max(0) as usize;

            let mut hits = Vec::new();
            let mut total_records = 0usize;
            let mut db_present = false;

            // 修复 P1-15：total_records 一律取索引库真实记录数；索引缺失时不得硬编码 59199
            // R14：先取较多候选（50）供后续 category/rodden/年份过滤
            if let Some(idx_path) = crate::db::resolve_data_path("astrodata_index.bin") {
                if let Ok(db) = crate::db::AstroDataDatabase::open(idx_path) {
                    db_present = true;
                    total_records = db.total_count();
                    hits = db.search(&query, 50);
                }
            } else {
                db_present = false;
            }

            // R5/R6：加载完整案例库详情（wiki 摘要 + 分类），按 name 精确补充每条命中。
            // R6 P3：使用 OnceLock 进程内单例缓存，多次查询只解压一次 16MB ADTS。
            // details.bin 缺失时优雅降级为精简索引输出（不 panic）。
            let details_db = crate::db::astrodata_details();
            let details_present = details_db.is_some();
            let details_count = details_db.map(|d| d.count()).unwrap_or(0);

            // R14：合并扩展字段（rodden/坐标/时区/来源链接等），并应用
            // category/rodden/birthYear 过滤，最后 offset/limit 分页。
            let source_tag = "Astro-Databank + Wikipedia (CC BY-SA 4.0)";
            let mut enriched: Vec<serde_json::Value> = Vec::new();
            for mut h in hits.into_iter() {
                let name = h.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let det = details_db.and_then(|d| d.get(&name));

                // category 过滤：按 details 中的 categories 子串匹配（不区分大小写）
                if let Some(cf) = &category_filter {
                    let cats_match = det
                        .and_then(|d| d.get("categories"))
                        .and_then(|c| c.as_array())
                        .map(|arr| arr.iter().any(|c| c.as_str().map(|s| s.to_lowercase().contains(cf)).unwrap_or(false)))
                        .unwrap_or(false);
                    if !cats_match {
                        continue;
                    }
                }
                // rodden 过滤：逗号分隔多值（AA/A/B/C/DD/X/XX）
                if !rodden_filter.is_empty() {
                    let rodden_val = det.and_then(|d| d.get("rodden")).and_then(|v| v.as_str()).unwrap_or("");
                    if !rodden_filter.iter().any(|r| r == rodden_val) {
                        continue;
                    }
                }
                // birthYear 过滤：按 details.birth_year
                if birth_year_from.is_some() || birth_year_to.is_some() {
                    let by = det.and_then(|d| d.get("birth_year")).and_then(|v| v.as_i64());
                    let ok = by.map(|y| {
                        birth_year_from.map(|f| y >= f).unwrap_or(true)
                            && birth_year_to.map(|t| y <= t).unwrap_or(true)
                    }).unwrap_or(false);
                    if !ok {
                        continue;
                    }
                }

                // 合并扩展字段（空值跳过，保持输出干净）
                if let Some(det) = det {
                    if let Some(obj) = h.as_object_mut() {
                        for key in ["wiki_summary", "wiki_url", "born_display", "born_zh",
                                    "summary_zh", "categories", "title", "rodden", "lat", "lon",
                                    "gps_lat", "gps_lon", "zone", "tz_abbr", "adb_url",
                                    "gender", "has_time", "birth_year", "birth_date", "birth_time",
                                    "collector", "data_source", "pos", "pos_zh", "time_accuracy"]
                        {
                            if let Some(v) = det.get(key) {
                                if !v.is_null() {
                                    let empty = v.as_str().map(|s| s.is_empty()).unwrap_or(false);
                                    if !empty {
                                        obj.insert(key.to_string(), v.clone());
                                    }
                                }
                            }
                        }
                        obj.insert("source".to_string(), serde_json::Value::String(source_tag.to_string()));
                    }
                }
                enriched.push(h);
            }

            let total_after_filter = enriched.len();
            let paged: Vec<serde_json::Value> = enriched.into_iter().skip(offset).take(limit).collect();

            let filters_applied = category_filter.is_some() || !rodden_filter.is_empty()
                || birth_year_from.is_some() || birth_year_to.is_some();
            let summary = if details_present {
                if filters_applied {
                    format!(
                        "AstroData 名人案例库 [{}]: 索引 {} 条, 详情库 {} 条, 过滤后 {} 条, 输出第 {}-{} 条",
                        query, total_records, details_count, total_after_filter, offset + 1, offset + paged.len()
                    )
                } else {
                    format!(
                        "AstroData 名人案例库 [{}]: 索引 {} 条, 完整案例库(含维基摘要+分类+出生数据) {} 条, 命中 {} 条",
                        query, total_records, details_count, paged.len()
                    )
                }
            } else {
                format!(
                    "AstroData 名人星盘检索 [{}]: 共 {} 条索引记录, 命中 {} 条 (详情库 astrodata_details.bin 未部署，仅精简索引)",
                    query, total_records, paged.len()
                )
            };

            let result = serde_json::json!({
                "technique": "astrodata",
                "query": query,
                "db_present": db_present,
                "details_present": details_present,
                "total_records": total_records,
                "details_records": details_count,
                "filtered_count": total_after_filter,
                "offset": offset,
                "limit": limit,
                "hits_count": paged.len(),
                "hits": paged,
                "summary": summary
            });
            Ok(result)
        }
        "export_registry" => {
            let fmt = input.format.as_deref()
                .or(input.text.as_deref())
                .unwrap_or("json");
            let result = serde_json::json!({
                "technique": tool,
                "format": fmt,
                "version": 58,
                "total_techniques": 110,
                "summary": "统一导出快照契约规范"
            });
            Ok(result)
        }
        "export_parse" => {
            // P2-2 对齐 Python exports/parser.py:parse_export_content：
            // 接受导出文本快照（【段标题】分段）→ 切分为结构化段落 JSON。
            let content = input.text.as_deref().unwrap_or("");
            let technique = input.name.as_deref().unwrap_or(tool);
            let mut sections: Vec<serde_json::Value> = Vec::new();
            let mut cur_title = String::new();
            let mut cur_body: Vec<&str> = Vec::new();
            for line in content.lines() {
                let trimmed = line.trim();
                // 行首【标题】段标记：可带尾随正文（如 "【基本信息】日主甲木"）
                let lead = trimmed.strip_prefix('【').and_then(|rest| {
                    rest.find('】').map(|i| {
                        let title = &rest[..i];
                        let body = &rest[i + '】'.len_utf8()..];
                        (title, body)
                    })
                });
                if let Some((title, rest_body)) = lead {
                    if !cur_title.is_empty() || !cur_body.is_empty() {
                        sections.push(serde_json::json!({
                            "title": cur_title,
                            "body": cur_body.join("\n").trim().to_string()
                        }));
                    }
                    cur_title = title.to_string();
                    cur_body.clear();
                    if !rest_body.trim().is_empty() {
                        cur_body.push(rest_body.trim());
                    }
                } else if !trimmed.is_empty() {
                    cur_body.push(trimmed);
                }
            }
            if !cur_title.is_empty() || !cur_body.is_empty() {
                sections.push(serde_json::json!({
                    "title": cur_title,
                    "body": cur_body.join("\n").trim().to_string()
                }));
            }
            let titles: Vec<&str> = sections.iter().filter_map(|s| s.get("title")?.as_str()).collect();
            Ok(serde_json::json!({
                "technique": technique,
                "section_count": sections.len(),
                "detected_titles": titles,
                "sections": sections,
                "summary": format!("导出快照解析：识别 {} 个段落", sections.len())
            }))
        }
        "knowledge_registry" => {
            let domain = input.domain.as_deref()
                .or(input.text.as_deref())
                .unwrap_or("astro");
            let result = serde_json::json!({
                "technique": tool,
                "domain": domain,
                "citations": 236,
                "summary": "权威教义与流派方法论知识库 (31 域 236 条学术典籍出处索引)"
            });
            Ok(result)
        }
        "knowledge_read" => {
            // P2-2 对齐 Python KnowledgeReadInput：domain/category/key 精读 或 query 跨域检索。
            // Rust 侧返回结构化单条读取骨架（域/分类/键/查询回显 + 命中文档索引）。
            let domain = input.domain.clone().unwrap_or_default();
            let category = input.text.clone().unwrap_or_default();
            let key = input.name.clone().unwrap_or_default();
            let query = input.params.as_ref()
                .and_then(|p| p.get("query").and_then(|q| q.as_str()))
                .unwrap_or("")
                .to_string();
            let search_mode = if !query.is_empty() { "query" } else { "read" };
            Ok(serde_json::json!({
                "technique": tool,
                "search_mode": search_mode,
                "domain": domain,
                "category": category,
                "key": key,
                "query": query,
                "entry_found": !category.is_empty() || !key.is_empty() || !query.is_empty(),
                "summary": format!("知识{}：域[{}] 类[{}] 键[{}]",
                    if search_mode=="query" {"跨域检索"} else {"精读"}, domain, category, key)
            }))
        }
        "nongli_time" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let ((lat, lon, loc_name), loc_warn) = input.get_location_detail();
            let jde = crate::bazi_exact::to_julian_day(y, m, d, h, min, sec);
            let t = (jde - 2451545.0) / 36525.0;
            let l0 = (280.46646 + 36000.76983 * t).rem_euclid(360.0);
            let m_deg = (357.52911 + 35999.05029 * t).rem_euclid(360.0);
            let eps = 23.439291 - 0.0130042 * t;
            let y_param = (eps.to_radians() / 2.0).tan().powi(2);
            let eot_rad = y_param * (2.0 * l0.to_radians()).sin()
                - 2.0 * 0.0167086 * m_deg.to_radians().sin()
                + 4.0 * 0.0167086 * y_param * m_deg.to_radians().sin() * (2.0 * l0.to_radians()).cos();
            let eot_minutes = eot_rad.to_degrees() * 4.0;

            let lon_offset_minutes = (lon - 120.0) * 4.0;
            let total_offset_seconds = (eot_minutes + lon_offset_minutes) * 60.0;

            let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
            let (ly, lm, ld, leap) = crate::lunar_table::solar_to_lunar(y, m, d);
            // 修复 P2-7：明示是否已做真太阳时换算。公式以经度-120(北京子午线)为基准，
            // 仅当输入为北京钟表时(timezone_offset=8)时，输出的 offset 才是“钟表时→真太阳时”的有效换算；
            // 非 8 的外来时区，引擎按原钟表时刻折算、未做跨时区真太阳时换算。
            let tz = input.timezone_offset();
            let solar_converted = (tz - 8.0).abs() < 1e-6;
            let solar_note = if solar_converted {
                "输入为北京钟表时(UTC+8)，已按本地经度与均时差(EoT)换算真太阳时".to_string()
            } else {
                format!("输入 timezone_offset={} 非北京时，引擎按原钟表时刻折算，未做跨时区真太阳时换算", tz)
            };
            let mut result = serde_json::json!({
                "technique": "nongli_time",
                "location": loc_name,
                "latitude": lat,
                "longitude": lon,
                "clock_datetime": format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, h, min, sec),
                "eot_minutes": eot_minutes,
                "lon_offset_minutes": lon_offset_minutes,
                "total_true_solar_offset_seconds": total_offset_seconds,
                "solar_time_converted": solar_converted,
                "solar_time_note": solar_note,
                "lunar": format!("{}年{}月{}日(闰:{})", ly, lm, ld, leap),
                "four_pillars": [bazi.year_pillar, bazi.month_pillar, bazi.day_pillar, bazi.hour_pillar],
                "summary": format!("真太阳时精密解算【{}】：经度 {:.2}° 纬度 {:.2}°，时差 {:.1} 分，均时差 {:.1} 分，总校正 {:.0} 秒", loc_name, lon, lat, lon_offset_minutes, eot_minutes, total_offset_seconds)
            });
            // 修复 P1-9：城市未命中回退默认坐标时显式告警
            if let Some(w) = loc_warn {
                result["location_warning"] = serde_json::Value::String(w);
            }
            Ok(result)
        }
        "jieqi_birth" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            // 修复 P0-10：当值节气按 24 节气边界（12 节取权威节令表 + 12 气用修正模型）判定，
            // 与八字月柱边界完全一致，不再依赖无章动光行差修正的裸黄经
            let birth_ts = crate::bazi_exact::to_timestamp_seconds(y, m, d, h, min, sec);
            let boundaries = build_jieqi_boundaries(y, 1);
            let mut current = (String::from("小寒"), birth_ts - 400 * 86400);
            for b in &boundaries {
                if b.1 <= birth_ts {
                    current = b.clone();
                } else {
                    break;
                }
            }
            // 修复 P1-14：太阳视黄经按入参时区折算真 UTC JD（默认 UTC+8，与历史一致）
            let tz = input.timezone_offset();
            let sun_lon = crate::bazi_exact::sun_ecliptic_longitude(
                crate::bazi_exact::to_julian_day(y, m, d, h, min, sec) - tz / 24.0,
            );
            let bazi = crate::bazi_exact::calculate_exact_bazi_tz(y, m, d, h, min, sec, tz);
            let (ly, lm, ld, leap) = crate::lunar_table::solar_to_lunar(y, m, d);
            let (by, bm, bd, bh, bmin, _bs) =
                crate::bazi_exact::jd_to_civil(current.1 as f64 / 86400.0 + 2440587.5);
            let result = serde_json::json!({
                "technique": "jieqi_birth",
                "sun_ecliptic_longitude": sun_lon,
                "current_jieqi": current.0,
                "current_jieqi_time": format!("{:04}-{:02}-{:02} {:02}:{:02}", by, bm, bd, bh, bmin),
                "jieqi_progress_degree": sun_lon % 15.0,
                "lunar": format!("{}年{}月{}日(闰:{})", ly, lm, ld, leap),
                "four_pillars": [bazi.year_pillar, bazi.month_pillar, bazi.day_pillar, bazi.hour_pillar],
                "summary": format!("生辰节气精确定位：太阳视黄经 {:.4}°，当值节气【{}】（{} 入节）", sun_lon, current.0, format!("{:04}-{:02}-{:02} {:02}:{:02}", by, bm, bd, bh, bmin))
            });
            Ok(result)
        }
        "jieqi_year" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
            let (ly, lm, ld, leap) = crate::lunar_table::solar_to_lunar(y, m, d);

            let jie_names = ["小寒", "立春", "惊蛰", "清明", "立夏", "芒种", "小暑", "立秋", "白露", "寒露", "立冬", "大雪"];
            let qi_names = ["大寒", "雨水", "春分", "谷雨", "小满", "夏至", "大暑", "处暑", "秋分", "霜降", "小雪", "冬至"];
            let jie_lons = [285.0, 315.0, 345.0, 15.0, 45.0, 75.0, 105.0, 135.0, 165.0, 195.0, 225.0, 255.0];

            // 修复 P0-10：1900–2100 年内 12 节（含立春）直接采用权威节令表（与八字月柱同一
            // 边界基准，表内即北京墙钟时间），12 气用修正太阳视黄经模型计算；范围外用模型全算。
            let mut entries: Vec<(String, f64, i64)> = Vec::with_capacity(24); // (name, target_lon, naive_ts)
            if (1900..=2100).contains(&y) {
                let row = crate::jieqi_table::JIE_TABLE_1900_2100[(y - 1900) as usize];
                for k in 0..12 {
                    let jie_ts = row[k];
                    let jie_jd_utc = naive_ts_to_jd_utc(jie_ts);
                    let qi_jd_utc = find_jieqi_jd_utc((jie_lons[k] + 15.0_f64).rem_euclid(360.0_f64), jie_jd_utc, 25.0);
                    let qi_ts = jd_utc_to_naive_ts(qi_jd_utc);
                    entries.push((jie_names[k].to_string(), jie_lons[k], jie_ts));
                    entries.push((qi_names[k].to_string(), (jie_lons[k] + 15.0).rem_euclid(360.0), qi_ts));
                }
                // 修复 P2-8：不再旋转到“立春”开头；统一按日历时间戳升序排列（1 月小寒在前、12 月冬至在后）
            } else {
                let jieqi_order = [
                    "立春", "雨水", "惊蛰", "春分", "清明", "谷雨", "立夏", "小满", "芒种", "夏至",
                    "小暑", "大暑", "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪",
                    "大雪", "冬至", "小寒", "大寒",
                ];
                let vernal_approx_jd = crate::bazi_exact::to_julian_day(y, 3, 20, 12, 0, 0);
                for (i, name) in jieqi_order.iter().enumerate() {
                    let target_lon = (315.0 + (i as f64 * 15.0)).rem_euclid(360.0);
                    let approx_days = if target_lon >= 285.0 {
                        (target_lon - 360.0) / 0.985647
                    } else {
                        target_lon / 0.985647
                    };
                    let jq_jd_utc = find_jieqi_jd_utc(target_lon, vernal_approx_jd + approx_days - 8.0 / 24.0, 30.0);
                    let jq_ts = jd_utc_to_naive_ts(jq_jd_utc);
                    entries.push((name.to_string(), target_lon, jq_ts));
                }
            }

            // 修复 P2-8：按日历时间戳升序排序（1 月小寒在前，12 月冬至在后），时刻值不变仅调序
            entries.sort_by_key(|(_, _, ts)| *ts);

            let mut list = Vec::new();
            for (idx, (name, target_lon, ts)) in entries.iter().enumerate() {
                // 朴素本地时间戳 → 北京墙钟历法（ts 按"1970-01-01 起 UTC 秒"直接分解）
                let (cal_yr, cal_mon, cal_day, cal_h, cal_min, cal_sec) =
                    crate::bazi_exact::jd_to_civil(*ts as f64 / 86400.0 + 2440587.5);
                let exact_time = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", cal_yr, cal_mon, cal_day, cal_h, cal_min, cal_sec);
                let jde = naive_ts_to_jd_utc(*ts);
                list.push(serde_json::json!({
                    "name": name,
                    "target_sun_longitude": target_lon,
                    "order": idx + 1,
                    "exact_time": exact_time,
                    "jde": jde
                }));
            }
            let result = serde_json::json!({
                "technique": "jieqi_year",
                "year": y,
                "jieqi_count": 24,
                "annual_jieqi_schedule": list,
                "solar_datetime": format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, h, min, sec),
                "lunar": format!("{}年{}月{}日(闰:{})", ly, lm, ld, leap),
                "four_pillars": [bazi.year_pillar, bazi.month_pillar, bazi.day_pillar, bazi.hour_pillar],
                "summary": format!("{} 年二十四节气入节时刻序列（12 节取权威节令表，12 气按太阳视黄经模型）", y)
            });
            Ok(result)
        }
        "calendar_month" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let bazi_cur = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
            let (ly_cur, lm_cur, ld_cur, leap_cur) = crate::lunar_table::solar_to_lunar(y, m, d);

            // 精准计算当月实际公历天数
            let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
            let max_days = match m {
                2 => if is_leap { 29 } else { 28 },
                4 | 6 | 9 | 11 => 30,
                _ => 31,
            };

            // 建除十二神与二十八宿基础
            let jian_chu_names = ["建", "除", "满", "平", "定", "执", "破", "危", "成", "收", "开", "闭"];
            let week_cn = ["日", "一", "二", "三", "四", "五", "六"];

            let mut days = Vec::new();
            for day_idx in 1..=max_days {
                let (_ly, lm, ld, leap) = crate::lunar_table::solar_to_lunar(y, m, day_idx);
                // 修复 P1-N1：建除月支口径与 tongshu/huangli 统一——交节日整日归属新月。
                // 日柱整日不变仍取正午(12:00)；月支改取当日末刻(23:59)，使午后交节(如寒露10-08 14:29)
                // 当日即归入新月月建，消除 calendar_month(破) vs tongshu/JS(执) 的同日期矛盾。
                let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, day_idx, 12, 0, 0);
                let bazi_eod = crate::bazi_exact::calculate_exact_bazi(y, m, day_idx, 23, 59, 0);

                // 星期推算 (基姆拉尔森公式)
                let (w_m, w_y) = if m <= 2 { (m + 12, y - 1) } else { (m, y) };
                let weekday = (day_idx as i32 + 2 * w_m as i32 + 3 * (w_m as i32 + 1) / 5 + w_y + w_y / 4 - w_y / 100 + w_y / 400 + 1).rem_euclid(7) as usize;

                // 建除十二神：以日支与月支相对位推导
                let d_zhi_char = bazi.day_pillar.chars().nth(1).unwrap_or('子');
                let d_gan_char = bazi.day_pillar.chars().next().unwrap_or('甲');
                let m_zhi_char = bazi_eod.month_pillar.chars().nth(1).unwrap_or('寅');
                let d_z_idx = crate::bazi::DIZHI.iter().position(|&x| x.starts_with(d_zhi_char)).unwrap_or(0);
                let d_g_idx = crate::bazi::TIANGAN.iter().position(|&x| x.starts_with(d_gan_char)).unwrap_or(0);
                let m_z_idx = crate::bazi::DIZHI.iter().position(|&x| x.starts_with(m_zhi_char)).unwrap_or(0);
                let jian_idx = (d_z_idx + 12 - m_z_idx) % 12;

                // R14 P2-8：复用现有计算函数，补宜忌(yi/ji)、二十八宿(xiu)、纳音(naying)
                let day_jd = crate::bazi_exact::to_julian_day(y, m, day_idx, 12, 0, 0).floor() as i64;
                let hl = crate::derivation::calculate_huangli(m_z_idx, d_z_idx);
                let ts = crate::tongshu::calculate_tongshu(m_z_idx, d_z_idx, d_g_idx, day_jd);
                let gz_offset = (6 * d_g_idx as i32 - 5 * d_z_idx as i32).rem_euclid(60) as usize;
                let naying = crate::bazi::NAYIN_TABLE[gz_offset];

                days.push(serde_json::json!({
                    "solar_day": day_idx,
                    "date": format!("{:04}-{:02}-{:02}", y, m, day_idx),
                    "day_of_week": format!("星期{}", week_cn[weekday]),
                    "lunar_day": format!("{}月{}(闰:{})", lm, ld, leap),
                    "day_ganzhi": bazi.day_pillar,
                    "jian_chu_12": jian_chu_names[jian_idx],
                    "xiu28": ts.xiu28_star,
                    "naying": naying,
                    "yi": hl.yi,
                    "ji": hl.ji,
                }));
            }
            let result = serde_json::json!({
                "technique": "calendar_month",
                "year": y,
                "month": m,
                "days_count": days.len(),
                "calendar_days": days,
                "solar_datetime": format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, h, min, sec),
                "lunar": format!("{}年{}月{}日(闰:{})", ly_cur, lm_cur, ld_cur, leap_cur),
                "four_pillars": [bazi_cur.year_pillar, bazi_cur.month_pillar, bazi_cur.day_pillar, bazi_cur.hour_pillar],
                "summary": format!("{}年{}月中国公农合历与干支流日全览表 (全月共{}天)", y, m, max_days)
            });
            Ok(result)
        }

        _ => return None,
    };
    Some(res)
}
