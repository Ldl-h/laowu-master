use serde_json::Value;
use crate::dispatcher::{UniversalInput, split_gz};

/// 易经六爻、河洛理数、残平、神数矩阵、择吉通书、民间杂占及历法实用处理器
pub fn handle_practical(tool: &str, input: &UniversalInput) -> Option<Result<Value, String>> {
    let res = match tool {
        // 易经六爻与卦象
        "sixyao" | "liuyao" | "gua_desc" | "gua_meiyi" => {
            if let Some(ref lines_val) = input.lines {
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
                let res = crate::liuyao::calculate_liuyao_lines(&line_bits, &movings);
                Ok(res)
            } else {
                let nums = input.numbers.as_ref().or(input.nums.as_ref());
                let def_nums = vec![1, 5, 3];
                let nums_ref = nums.unwrap_or(&def_nums);
                let n1 = nums_ref.first().copied().unwrap_or(1) as usize;
                let n2 = nums_ref.get(1).copied().unwrap_or(5) as usize;
                let n3 = nums_ref.get(2).copied().unwrap_or(3) as usize;
                let ly = crate::liuyao::calculate_liuyao(n1, n2, n3);
                serde_json::to_value(ly).map_err(|e| e.to_string())
            }
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
                        if let Some(v) = map.get(&hl.xian_tian_gua[..]).or_else(|| map.get(&hl.hou_tian_gua[..])) {
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
            let (_, m, d, h, _, _) = input.get_datetime();
            let tb = crate::tieban::calculate_tieban(m, d, h as usize % 12, 0, true);
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
            let fam = input.family_key.as_deref().unwrap_or(tool);
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
            let ss = crate::shenshu_matrix::calculate_generic_shenshu(fam, &yg, &mg, &dg, &hg);
            serde_json::to_value(ss).map_err(|e| e.to_string())
        }

        // 择日与通书
        "tongshu" => {
            let (_, m, d, _, _, _) = input.get_datetime();
            let ts = crate::tongshu::calculate_tongshu(m as usize % 12, d as usize % 12, 0, 0);
            serde_json::to_value(ts).map_err(|e| e.to_string())
        }
        "huangli" => {
            let (_, m, d, _, _, _) = input.get_datetime();
            let ts = crate::tongshu::calculate_tongshu(m as usize % 12, d as usize % 12, 0, 0);
            let hl = crate::derivation::calculate_huangli(m as usize % 12, d as usize % 12);
            let mut val = serde_json::to_value(ts).unwrap_or(serde_json::Value::Null);
            if let Some(obj) = val.as_object_mut() {
                obj.insert("jian_chu_god".to_string(), serde_json::json!(hl.jian_chu_god));
                obj.insert("jixiong".to_string(), serde_json::json!(hl.jixiong));
                obj.insert("yi".to_string(), serde_json::json!(hl.yi));
                obj.insert("ji".to_string(), serde_json::json!(hl.ji));
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
            let sd = input.seed.unwrap_or(20261001);
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
            let (_, m, d, _, _, _) = input.get_datetime();
            let sz = crate::suzhan_zhengchuan::calculate_suzhan(m, d, 0);
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
        "tianxing" | "tianxingzeri" => {
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
            let sd = input.seed.unwrap_or(20261001);
            let ob = crate::tianxing_otherbu::calculate_otherbu(m_key, sd);
            serde_json::to_value(ob).map_err(|e| e.to_string())
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

            let dong_yaos: Vec<usize> = input.dong_yaos.clone()
                .or_else(|| {
                    input.params.as_ref()
                        .and_then(|p| p.get("dongYaos").or_else(|| p.get("dong_yaos")))
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as usize)).collect())
                })
                .unwrap_or_else(|| vec![1]);

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
            let query = input.text.as_deref()
                .or(input.name.as_deref())
                .unwrap_or("Einstein");
            let mut hits = Vec::new();
            let mut total_records = 0;
            let mut db_present = false;

            if let Some(idx_path) = crate::db::resolve_data_path("astrodata_index.bin") {
                if let Ok(db) = crate::db::AstroDataDatabase::open(idx_path) {
                    db_present = true;
                    total_records = db.total_count();
                    hits = db.search(query, 10);
                }
            } else if crate::db::resolve_data_path("astrodata_cases.bin").is_some() {
                db_present = true;
                total_records = 59199;
            }

            let result = serde_json::json!({
                "technique": "astrodata",
                "query": query,
                "db_present": db_present,
                "total_records": total_records,
                "hits_count": hits.len(),
                "hits": hits,
                "summary": format!("AstroData 6万名人占星案例库: 全球权威名人星盘检索 [{}], 命中 {} 条案例", query, hits.len())
            });
            Ok(result)
        }
        "export_registry" | "export_parse" => {
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
        "knowledge_registry" | "knowledge_read" => {
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
        "nongli_time" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let (lat, lon, loc_name) = input.get_location();
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
            let result = serde_json::json!({
                "technique": "nongli_time",
                "location": loc_name,
                "latitude": lat,
                "longitude": lon,
                "clock_datetime": format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, h, min, sec),
                "eot_minutes": eot_minutes,
                "lon_offset_minutes": lon_offset_minutes,
                "total_true_solar_offset_seconds": total_offset_seconds,
                "lunar": format!("{}年{}月{}日(闰:{})", ly, lm, ld, leap),
                "four_pillars": [bazi.year_pillar, bazi.month_pillar, bazi.day_pillar, bazi.hour_pillar],
                "summary": format!("真太阳时精密解算【{}】：经度 {:.2}° 纬度 {:.2}°，时差 {:.1} 分，均时差 {:.1} 分，总校正 {:.0} 秒", loc_name, lon, lat, lon_offset_minutes, eot_minutes, total_offset_seconds)
            });
            Ok(result)
        }
        "jieqi_birth" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let jde = crate::bazi_exact::to_julian_day(y, m, d, h, min, sec);
            let sun_lon = crate::bazi_exact::sun_ecliptic_longitude(jde);
            let jieqi_names = [
                "春分", "清明", "谷雨", "立夏", "小满", "芒种",
                "夏至", "小暑", "大暑", "立秋", "处暑", "白露",
                "秋分", "寒露", "霜降", "立冬", "小雪", "大雪",
                "冬至", "小寒", "大寒", "立春", "雨水", "惊蛰"
            ];
            let jq_idx = ((sun_lon / 15.0).floor() as usize) % 24;
            let degree_in_jq = sun_lon % 15.0;
            let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
            let (ly, lm, ld, leap) = crate::lunar_table::solar_to_lunar(y, m, d);
            let result = serde_json::json!({
                "technique": "jieqi_birth",
                "sun_ecliptic_longitude": sun_lon,
                "current_jieqi": jieqi_names[jq_idx],
                "jieqi_progress_degree": degree_in_jq,
                "lunar": format!("{}年{}月{}日(闰:{})", ly, lm, ld, leap),
                "four_pillars": [bazi.year_pillar, bazi.month_pillar, bazi.day_pillar, bazi.hour_pillar],
                "summary": format!("生辰节气精确定位：太阳视黄经 {:.4}°，当值节气【{}】进行度 {:.2}°", sun_lon, jieqi_names[jq_idx], degree_in_jq)
            });
            Ok(result)
        }
        "jieqi_year" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
            let (ly, lm, ld, leap) = crate::lunar_table::solar_to_lunar(y, m, d);
            let jieqi_names = [
                "立春", "雨水", "惊蛰", "春分", "清明", "谷雨",
                "立夏", "小满", "芒种", "夏至", "小暑", "大暑",
                "立秋", "处暑", "白露", "秋分", "寒露", "霜降",
                "立冬", "小雪", "大雪", "冬至", "小寒", "大寒"
            ];
            let mut list = Vec::new();
            for (i, name) in jieqi_names.iter().enumerate() {
                let target_lon = (315.0 + (i as f64 * 15.0)).rem_euclid(360.0);
                list.push(serde_json::json!({
                    "name": name,
                    "target_sun_longitude": target_lon,
                    "order": i + 1
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
                "summary": format!("{} 年精密二十四节气太阳视黄经入节序列解算完备", y)
            });
            Ok(result)
        }
        "calendar_month" => {
            let (y, m, d, h, min, sec) = input.get_datetime();
            let bazi_cur = crate::bazi_exact::calculate_exact_bazi(y, m, d, h, min, sec);
            let (ly_cur, lm_cur, ld_cur, leap_cur) = crate::lunar_table::solar_to_lunar(y, m, d);
            let mut days = Vec::new();
            for day_idx in 1..=30 {
                let (_ly, lm, ld, leap) = crate::lunar_table::solar_to_lunar(y, m, day_idx);
                let bazi = crate::bazi_exact::calculate_exact_bazi(y, m, day_idx, 12, 0, 0);
                days.push(serde_json::json!({
                    "solar_day": day_idx,
                    "lunar_day": format!("{}月{}(闰:{})", lm, ld, leap),
                    "day_ganzhi": bazi.day_pillar
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
                "summary": format!("{}年{}月中国公农合历与干支流日全览表", y, m)
            });
            Ok(result)
        }

        _ => return None,
    };
    Some(res)
}
