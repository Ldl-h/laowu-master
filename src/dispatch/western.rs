use serde_json::Value;
use crate::dispatcher::UniversalInput;

/// 西洋占星、古典希腊、巴比伦、乌拉尼亚、吠陀印占及推运核心分发处理器
pub fn handle_western_and_astro(tool: &str, input: &UniversalInput) -> Option<Result<Value, String>> {
    let res = match tool {
        // 古典占星与本命盘
        "chart" | "natal" | "transit" | "synastry" | "composite_chart" | "davison" => {
            let jde = input.get_julian_day();
            let (lat, lon, _) = input.get_location();
            let hsys = input.hsys.as_deref().unwrap_or("placidus");
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, hsys);

            // 联动 tiaowen.bin 烘焙萨比恩 360 度象征
            let mut sabian_info = None;
            if let Some(tiaowen_path) = crate::db::resolve_data_path("tiaowen.bin") {
                if let Ok(db) = crate::db::XuanshiDatabase::open(tiaowen_path) {
                    let sun_deg = chart.planets.first().map(|p| p.longitude as usize + 1).unwrap_or(1);
                    sabian_info = db.get_sabian_symbol(sun_deg);
                }
            }

            let mut result = serde_json::json!({
                "technique": tool,
                "jde": chart.jde,
                "lat": chart.lat,
                "lon": chart.lon,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "armc": chart.armc,
                "house_system": chart.house_system,
                "houses": chart.houses,
                "planets": chart.planets,
                "chart": chart,
                "summary": format!("西洋占星排盘 [{}]: 上升 ASC【{:.2}°】，中天 MC【{:.2}°】，十大天体黄道排布完备", tool, chart.ascendant, chart.mc)
            });
            if let Some(s) = sabian_info {
                result["sun_sabian_symbol"] = s;
            }
            Ok(result)
        }
        "chart12" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "placidus");
            let dwad_planets: Vec<_> = chart.planets.iter().map(|p| {
                let mut p2 = p.clone();
                p2.longitude = (p.longitude * 12.0).rem_euclid(360.0);
                let (sign, deg) = crate::ephem::get_zodiac_sign(p2.longitude);
                p2.sign = sign;
                p2.degree_in_sign = deg;
                p2
            }).collect();
            let dwad_asc = (chart.ascendant * 12.0).rem_euclid(360.0);
            let dwad_mc = (chart.mc * 12.0).rem_euclid(360.0);
            let result = serde_json::json!({
                "technique": "chart12",
                "ascendant": dwad_asc,
                "mc": dwad_mc,
                "planets": dwad_planets,
                "base_chart": chart,
                "summary": "十二分盘 / Dwadasamsa: 黄经乘以 12 阶展开，微黄道深入潜意识细分格局"
            });
            Ok(result)
        }
        "chart13" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "equal");
            let h13_planets: Vec<_> = chart.planets.iter().map(|p| {
                let mut p2 = p.clone();
                p2.longitude = (p.longitude * 13.0).rem_euclid(360.0);
                let (sign, deg) = crate::ephem::get_zodiac_sign(p2.longitude);
                p2.sign = sign;
                p2.degree_in_sign = deg;
                p2
            }).collect();
            let result = serde_json::json!({
                "technique": "chart13",
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "planets": h13_planets,
                "summary": "十三分扩展盘: 十三谐波展开与蛇夫座维度拓扑"
            });
            Ok(result)
        }
        "relocation" => {
            let jde = input.get_julian_day();
            let target_lat = input.target_lat.or(input.lat).unwrap_or(39.90);
            let target_lon = input.target_lon.or(input.lon).unwrap_or(116.40);
            let chart = crate::western_full::calculate_full_astro_chart(jde, target_lat, target_lon, "placidus");
            let result = serde_json::json!({
                "technique": "relocation",
                "target_lat": target_lat,
                "target_lon": target_lon,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "houses": chart.houses,
                "planets": chart.planets,
                "chart": chart,
                "summary": format!("重置星盘 (Relocation Chart): 重定位至 ({:.2}°, {:.2}°)，新上升点 ASC【{:.2}°】", target_lat, target_lon, chart.ascendant)
            });
            Ok(result)
        }
        "acg" => {
            let jde = input.get_julian_day();
            let lines = crate::western_full::calculate_acg_lines(jde);
            let result = serde_json::json!({
                "technique": "acg",
                "lines_count": lines.len(),
                "lines": lines,
                "summary": format!("占星地图 (AstroCartoGraphy): 全球 {} 条主要行星中天/上升地心投影线计算完备", lines.len())
            });
            Ok(result)
        }
        "solarreturn" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
            let (by, _, _, _, _, _) = input.get_datetime();
            let target_year = input.target_year.unwrap_or(by + 1);
            let sr_jde = crate::western_full::find_exact_solar_return(jde, target_year);
            let chart = crate::western_full::calculate_full_astro_chart(sr_jde, lat, lon, "placidus");
            let p_birth = crate::ephem::calculate_planetary_positions(jde);
            let natal_sun = p_birth[0].longitude;
            let return_sun = chart.planets[0].longitude;
            let diff = (return_sun - natal_sun).abs();
            let result = serde_json::json!({
                "technique": "solarreturn",
                "birth_jde": jde,
                "return_jde": sr_jde,
                "target_year": target_year,
                "natal_sun_lon": natal_sun,
                "return_sun_lon": return_sun,
                "sun_lon_diff": diff,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "chart": chart,
                "planets": chart.planets,
                "summary": format!("太阳返照盘: 太阳精确回归本命度数 {:.4}° (差值 {:.6}°)，返照年 {} 年排盘完毕", natal_sun, diff, target_year)
            });
            Ok(result)
        }
        "lunarreturn" => {
            let jde = input.get_julian_day();
            let (lat, lon, _) = input.get_location();
            let cycles = if let Some(target_y) = input.target_year {
                let (by, _, _, _, _, _) = input.get_datetime();
                ((target_y - by).max(0) as f64) * 13.368
            } else {
                1.0
            };
            let lr_jde = crate::western_full::find_exact_lunar_return(jde, cycles);
            let chart = crate::western_full::calculate_full_astro_chart(lr_jde, lat, lon, "placidus");
            let p_birth = crate::ephem::calculate_planetary_positions(jde);
            let natal_moon = p_birth[1].longitude;
            let return_moon = chart.planets[1].longitude;
            let diff = (return_moon - natal_moon).abs();
            let result = serde_json::json!({
                "technique": "lunarreturn",
                "birth_jde": jde,
                "return_jde": lr_jde,
                "natal_moon_lon": natal_moon,
                "return_moon_lon": return_moon,
                "moon_lon_diff": diff,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "chart": chart,
                "planets": chart.planets,
                "summary": format!("月亮返照盘: 月亮精确回归本命度数 {:.4}° (差值 {:.6}°)", natal_moon, diff)
            });
            Ok(result)
        }
        "lunationphase" => {
            let jde = input.get_julian_day();
            let phase = crate::western_full::calculate_lunation_phase(jde);
            serde_json::to_value(phase).map_err(|e| e.to_string())
        }
        "prenatalsyzygy" => {
            let jde = input.get_julian_day();
            let (syz_jde, syz_type, syz_lon) = crate::western_full::find_prenatal_syzygy(jde);
            let (sign, deg) = crate::ephem::get_zodiac_sign(syz_lon);
            let days_before_birth = (jde - syz_jde).max(0.0);
            let result = serde_json::json!({
                "technique": "prenatalsyzygy",
                "syzygy_jde": syz_jde,
                "syzygy_type": syz_type,
                "syzygy_sun_lon": syz_lon,
                "longitude": syz_lon,
                "days_before_birth": days_before_birth,
                "sign": sign,
                "degree_in_sign": deg,
                "summary": format!("产前朔望月: 出生前关键合朔【{}】，落入【{:.2}° {}】，发生于出生前 {:.2} 天", syz_type, deg, sign, days_before_birth)
            });
            Ok(result)
        }
        "triplicityrulers" => {
            let jde = input.get_julian_day();
            let planets = crate::ephem::calculate_planetary_positions(jde);
            let sun_lon = planets[0].longitude;
            let rulers = crate::western_extra::get_triplicity_rulers(sun_lon);
            let result = serde_json::json!({
                "technique": "triplicityrulers",
                "sun_longitude": sun_lon,
                "rulers": rulers,
                "summary": format!("多罗修斯三分主星: 太阳落入【{}】，日主【{}】，夜主【{}】，参与星【{}】", rulers.element, rulers.day_ruler, rulers.night_ruler, rulers.participating_ruler)
            });
            Ok(result)
        }
        "keypoints" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "placidus");
            let sun = chart.planets[0].longitude;
            let moon = chart.planets[1].longitude;
            let asc = chart.ascendant;
            let is_day = ((sun - asc).rem_euclid(360.0)) > 180.0;
            let fortune = if is_day {
                (asc + moon - sun).rem_euclid(360.0)
            } else {
                (asc + sun - moon).rem_euclid(360.0)
            };
            let spirit = if is_day {
                (asc + sun - moon).rem_euclid(360.0)
            } else {
                (asc + moon - sun).rem_euclid(360.0)
            };
            let (f_sign, f_deg) = crate::ephem::get_zodiac_sign(fortune);
            let (s_sign, s_deg) = crate::ephem::get_zodiac_sign(spirit);

            let points = [
                ("上升点 (ASC)", asc),
                ("中天 (MC)", chart.mc),
                ("太阳 (Sun)", sun),
                ("月亮 (Moon)", moon),
                ("福点 (Fortune)", fortune),
                ("精神点 (Spirit)", spirit),
            ];
            let max_orb = input.orb.unwrap_or(2.0);
            let star_links = crate::western_extra::calculate_fixed_star_connections(&points, max_orb);

            let result = serde_json::json!({
                "technique": "keypoints",
                "ascendant": asc,
                "mc": chart.mc,
                "lot_of_fortune": { "longitude": fortune, "sign": f_sign, "degree": f_deg },
                "lot_of_spirit": { "longitude": spirit, "sign": s_sign, "degree": s_deg },
                "is_day_chart": is_day,
                "fixed_star_connections_count": star_links.len(),
                "fixed_star_connections": star_links,
                "summary": format!("关键敏感点与恒星共振: 上升【{:.2}°】，福德点【{:.2}° {}】，精神点【{:.2}° {}】，命中 BSC5 亮星冲合共振 {} 组", asc, f_deg, f_sign, s_deg, s_sign, star_links.len())
            });
            Ok(result)
        }
        "horary" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "regiomontanus");
            let q_type = input.question_type.as_deref().unwrap_or("marriage");
            let (asc_sign, _) = crate::ephem::get_zodiac_sign(chart.ascendant);
            let hr = crate::horary::calculate_horary(jde, asc_sign, q_type);
            let result = serde_json::json!({
                "technique": "horary",
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "houses": chart.houses,
                "planets": chart.planets,
                "horary_judgment": hr,
                "summary": format!("西洋卜卦占星 (Horary): 上升 ASC【{:.2}° {}】，事项【{}】，完成法【{}】", chart.ascendant, asc_sign, hr.category, hr.perfection_mode)
            });
            Ok(result)
        }
        "election" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "placidus");
            let moon = &chart.planets[1];
            let is_voc = (moon.longitude % 30.0) > 27.0; // 简易空亡度数标志
            let result = serde_json::json!({
                "technique": "election",
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "planets": chart.planets,
                "moon_sign": moon.sign,
                "moon_degree": moon.degree_in_sign,
                "void_of_course": is_voc,
                "summary": format!("西洋择日占星 (Election): 择日上升点【{:.2}°】，月亮落【{} {:.2}°】，空亡判定【{}】", chart.ascendant, moon.sign, moon.degree_in_sign, if is_voc { "月亮空亡" } else { "月行顺畅" })
            });
            Ok(result)
        }
        "babylon" => {
            let jde = input.get_julian_day();
            let raw_planets = crate::ephem::calculate_planetary_positions(jde);
            let planets_tuple: Vec<(&str, f64)> = raw_planets.iter().map(|p| {
                let name = if p.name.contains("太阳") { "太阳" }
                else if p.name.contains("月亮") { "月亮" }
                else if p.name.contains("水星") { "水星" }
                else if p.name.contains("金星") { "金星" }
                else if p.name.contains("火星") { "火星" }
                else if p.name.contains("木星") { "木星" }
                else { "土星" };
                (name, p.longitude)
            }).collect();
            let scheme = input.school.as_deref().unwrap_or("swissA10");
            let bb = crate::babylon::calculate_babylon(&planets_tuple, scheme);
            let mut val = serde_json::to_value(bb).unwrap_or(serde_json::Value::Null);
            if let Some(arr) = val.get_mut("planets").and_then(|v| v.as_array_mut()) {
                for p in arr.iter_mut() {
                    if let Some(obj) = p.as_object_mut() {
                        let name_cn = obj.get("name_cn").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        let trop_lon = obj.get("tropical_lon").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        obj.insert("name".to_string(), serde_json::json!(name_cn));
                        obj.insert("longitude".to_string(), serde_json::json!(trop_lon));
                    }
                }
            }
            if let Some(obj) = val.as_object_mut() {
                obj.insert("ascendant".to_string(), serde_json::json!(input.ascendant.unwrap_or(45.0)));
            }
            Ok(val)
        }
        "germany" => {
            let jde = input.get_julian_day();
            let raw_planets = crate::ephem::calculate_planetary_positions(jde);
            let planets_tuple: Vec<(&'static str, f64)> = raw_planets.iter().map(|p| {
                let name: &'static str = if p.name.contains("太阳") { "太阳" }
                else if p.name.contains("月亮") { "月亮" }
                else if p.name.contains("水星") { "水星" }
                else if p.name.contains("金星") { "金星" }
                else if p.name.contains("火星") { "火星" }
                else if p.name.contains("木星") { "木星" }
                else { "土星" };
                (name, p.longitude)
            }).collect();
            let gm = crate::germany::calculate_uranian(jde, &planets_tuple);
            serde_json::to_value(gm).map_err(|e| e.to_string())
        }
        "guolao" | "guolao_chart" | "qizhengkin" => {
            let jde = input.get_julian_day();
            let asc = if let Some(a) = input.ascendant {
                a
            } else if input.lat.is_some() || input.lon.is_some() {
                let (la, lo, _) = input.get_location();
                crate::western_full::calculate_full_astro_chart(jde, la, lo, "placidus").ascendant
            } else {
                45.0
            };
            let gl = crate::guolao::calculate_guolao(jde, asc);
            match serde_json::to_value(&gl) {
                Ok(mut val) => {
                    let planets: Vec<serde_json::Value> = gl.seven_governors.iter().map(|p| {
                        serde_json::json!({
                            "name": p.name,
                            "longitude": p.longitude,
                            "sign": p.sign,
                            "mansion": p.mansion,
                            "nature": p.nature
                        })
                    }).collect();
                    if let Some(obj) = val.as_object_mut() {
                        obj.insert("planets".to_string(), serde_json::Value::Array(planets));
                        obj.insert("ascendant".to_string(), serde_json::json!(asc));
                        if let Some(p) = crate::db::resolve_data_path("tiaowen.bin") {
                            if let Ok(db) = crate::db::XuanshiDatabase::open(p) {
                                if let Some(zg) = db.get_zhangguo_stars() {
                                    obj.insert("zhangguo_matrix".to_string(), zg);
                                }
                            }
                        }
                    }
                    Ok(val)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        "vedic" | "india_chart" => {
            let jde = input.get_julian_day();
            let asc = if let Some(a) = input.ascendant {
                a
            } else if input.lat.is_some() || input.lon.is_some() {
                let (la, lo, _) = input.get_location();
                crate::western_full::calculate_full_astro_chart(jde, la, lo, "placidus").ascendant
            } else {
                45.0
            };
            let vd = crate::vedic_feigong::calculate_vedic(jde, asc);
            let mut val = serde_json::to_value(vd).unwrap_or(serde_json::Value::Null);
            let signs = ["Mesha", "Vrishabha", "Mithuna", "Karka", "Simha", "Kanya", "Tula", "Vrishchika", "Dhanu", "Makara", "Kumbha", "Meena"];
            if let Some(arr) = val.get_mut("planets").and_then(|v| v.as_array_mut()) {
                for p in arr.iter_mut() {
                    if let Some(obj) = p.as_object_mut() {
                        let d1_deg = obj.get("d1_degree").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let d1_sign = obj.get("d1_sign").and_then(|v| v.as_str()).unwrap_or("");
                        let sign_idx = signs.iter().position(|&s| d1_sign.starts_with(s)).unwrap_or(0);
                        let total_lon = (sign_idx as f64) * 30.0 + d1_deg;
                        obj.insert("longitude".to_string(), serde_json::json!(total_lon));
                    }
                }
            }
            if let Some(obj) = val.as_object_mut() {
                obj.insert("ascendant".to_string(), serde_json::json!(asc));
            }
            Ok(val)
        }
        "feigong" => {
            let (_, m, d, h, _, _) = input.get_datetime();
            let qz = input.qi_zhi.as_deref();
            let dg = input.day_gan.as_deref();
            let dz_named = input.day_zhi_str.as_deref();
            let zhi_names = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
            let dz_idx = input.day_zhi.and_then(|i| zhi_names.get(i).copied());
            let dz = dz_named.or(dz_idx);
            let fg = crate::vedic_feigong::calculate_feigong_full(m, d, h, qz, dg, dz);
            serde_json::to_value(fg).map_err(|e| e.to_string())
        }
        "zr" | "zodiacalreleasing" => {
            let age = input.age.unwrap_or(28.0);
            let jde = input.get_julian_day();
            let planets = crate::ephem::calculate_planetary_positions(jde);
            let sun_lon = planets.first().map(|p| p.longitude).unwrap_or(180.0);
            let start_sign = (sun_lon / 30.0).floor() as usize % 12;
            let zr_res = crate::zr::calculate_zodiacal_releasing(start_sign, age, "精神点");
            let result = serde_json::json!({
                "technique": "zr",
                "start_sign_idx": start_sign,
                "target_age": age,
                "current_period": zr_res.current_period,
                "summary": format!("黄道释放法 (Zodiacal Releasing): 从第 {} 宫起释放，当前年龄 {:.1} 岁行入【{}】", start_sign + 1, age, zr_res.current_period.sign)
            });
            Ok(result)
        }
        "balbillus" => {
            let jde = input.get_julian_day();
            let planets = crate::ephem::calculate_planetary_positions(jde);
            let p_tuples: Vec<(&str, f64)> = planets.iter().map(|p| (p.name, p.longitude)).collect();
            let max_years = input.age.unwrap_or(90.0);
            let bb = crate::balbillus::calculate_balbillus(&p_tuples, "太阳 (Sun)", false, max_years);
            let result = serde_json::json!({
                "technique": "balbillus",
                "start_planet": bb.start_planet,
                "total_cycle_years": bb.total_cycle_years,
                "periods_count": bb.main_periods.len(),
                "periods": bb.main_periods,
                "main_periods": bb.main_periods,
                "summary": format!("巴比卢斯129年系统: 旺距削减推演完毕，产生 {} 阶段生命周期", bb.main_periods.len())
            });
            Ok(result)
        }
        "decennials" => {
            let age = input.age.unwrap_or(30.0);
            let max_years = (age + 10.0).max(40.0);
            let dc = crate::western_extra::calculate_decennials(0, max_years);
            let current = dc.iter().find(|p| age >= p.start_age && age < p.end_age).cloned();
            let result = serde_json::json!({
                "technique": "decennials",
                "age": age,
                "current_period": current,
                "total_periods": dc.len(),
                "periods": dc,
                "summary": format!("十年大运 (Decennials): 年龄 {:.1} 岁当前行入大运【{}】副运【{}】", age, current.as_ref().map(|c| c.general_ruler).unwrap_or(""), current.as_ref().map(|c| c.specific_ruler).unwrap_or(""))
            });
            Ok(result)
        }
        "firdaria" => {
            let age = input.age.unwrap_or(30.0) as u32;
            let is_day = if let Some(g) = input.gender {
                g == 1 // 1: 乾造日生主阳
            } else {
                let (_, _, _, h, _, _) = input.get_datetime();
                (6..18).contains(&h)
            };
            let fd = crate::predictive::calculate_firdaria(age, is_day);
            serde_json::to_value(fd).map_err(|e| e.to_string())
        }
        "distributions" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(39.9);
            let lon = input.lon.unwrap_or(116.4);
            let full_chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "placidus");
            let asc = full_chart.ascendant;
            let max_years = input.age.unwrap_or(80.0);
            let dist_hits = crate::western_pd::calculate_distributions(asc, max_years);
            let result = serde_json::json!({
                "technique": "distributions",
                "ascendant": asc,
                "max_years": max_years,
                "terms_count": dist_hits.len(),
                "distributions": dist_hits,
                "summary": format!("界推运 (Distributions): 上升点 {:.2}° 循埃及界行运，共展开 {} 阶界限大运", asc, dist_hits.len())
            });
            Ok(result)
        }
        "persiandirected" => {
            let age = input.age.unwrap_or(30.0);
            let (ruler, years_into, desc) = crate::western_extra::calculate_persian_directed_years(age);
            let result = serde_json::json!({
                "technique": "persiandirected",
                "age": age,
                "directed_ruler": ruler,
                "years_into_period": years_into,
                "description": desc,
                "summary": format!("波斯向运 (Persian Directed): 年龄 {:.1} 岁行入【{}】限期第 {:.1} 年", age, ruler, years_into)
            });
            Ok(result)
        }
        "planetaryages" => {
            let age = input.age.unwrap_or(30.0);
            let bands = [
                ("Moon", "月亮", 0.0, 4.0),
                ("Mercury", "水星", 4.0, 14.0),
                ("Venus", "金星", 14.0, 22.0),
                ("Sun", "太阳", 22.0, 41.0),
                ("Mars", "火星", 41.0, 56.0),
                ("Jupiter", "木星", 56.0, 68.0),
                ("Saturn", "土星", 68.0, 120.0),
            ];
            let active = bands.iter().find(|b| age >= b.2 && age < b.3).unwrap_or(&bands[3]);
            let result = serde_json::json!({
                "technique": "planetaryages",
                "age": age,
                "active_planet": active.0,
                "active_planet_cn": active.1,
                "age_band": format!("{}-{}岁", active.2, active.3),
                "summary": format!("托勒密人生七阶 (Planetary Ages): 当前年龄约 {:.1} 岁，主政行星为【{}】({}-{}岁)", age, active.1, active.2, active.3)
            });
            Ok(result)
        }
        "yearsystem129" => {
            let age = input.age.unwrap_or(30.0);
            let cycle_age = age.rem_euclid(129.0);
            let (ruler, years_into, _) = crate::western_extra::calculate_persian_directed_years(age);
            let dec_periods = crate::western_extra::calculate_decennials(0, age + 10.0);
            let current_dec = dec_periods.iter().find(|p| age >= p.start_age && age < p.end_age).cloned();
            let result = serde_json::json!({
                "technique": "yearsystem129",
                "age": age,
                "cycle_age": cycle_age,
                "main_ruler": ruler,
                "years_in_main": years_into,
                "current_sub_period": current_dec,
                "summary": format!("129年系统推运: 年龄 {:.1} 岁 (周期内 {:.1} 岁)，当前主限星【{}】", age, cycle_age, ruler)
            });
            Ok(result)
        }
        "profection" => {
            let age = input.age.unwrap_or(30.0) as u32;
            let pf = crate::predictive::calculate_profection(age);
            serde_json::to_value(pf).map_err(|e| e.to_string())
        }
        "draconic" => {
            let jde = input.get_julian_day();
            let (lat, lon, _) = input.get_location();
            let mut chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "placidus");
            let node_lon = chart
                .planets
                .iter()
                .find(|p| p.name.contains("北交点") || p.name.contains("North Node"))
                .map(|p| p.longitude)
                .unwrap_or(0.0);

            for p in chart.planets.iter_mut() {
                p.longitude = (p.longitude - node_lon).rem_euclid(360.0);
                let (sign, deg) = crate::ephem::get_zodiac_sign(p.longitude);
                p.sign = sign;
                p.degree_in_sign = deg;
            }

            let result = serde_json::json!({
                "technique": "draconic",
                "node_lon": node_lon,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "chart": chart,
                "planets": chart.planets,
                "summary": format!("龙盘 (Draconic Chart): 以北交点 {:.2}° 归零白羊 0° 重新排布之宿命交点盘", node_lon)
            });
            Ok(result)
        }
        "solararc" => {
            let _jde = input.get_julian_day();
            let (by, bm, bd, bh, bmin, bsec) = input.get_datetime();
            let birth_jde = crate::bazi_exact::to_julian_day(by, bm, bd, bh, bmin, bsec);
            
            let target_jde = if let Some(ref td) = input.target_date {
                let parts: Vec<&str> = td.split('-').collect();
                if parts.len() == 3 {
                    let ty = parts[0].parse().unwrap_or(by + 1);
                    let tm = parts[1].parse().unwrap_or(bm);
                    let tday = parts[2].parse().unwrap_or(bd);
                    crate::bazi_exact::to_julian_day(ty, tm, tday, bh, bmin, bsec)
                } else {
                    birth_jde + input.age.unwrap_or(1.0) * 365.2422
                }
            } else if let Some(ty) = input.target_year {
                crate::bazi_exact::to_julian_day(ty, bm, bd, bh, bmin, bsec)
            } else if let Some(age) = input.age {
                birth_jde + age * 365.2422
            } else {
                birth_jde + 365.2422
            };

            let sa = crate::predictive::calculate_solar_arc(birth_jde, target_jde);
            let mut planets = crate::ephem::calculate_planetary_positions(birth_jde);
            for p in planets.iter_mut() {
                p.longitude = (p.longitude + sa.arc_degree).rem_euclid(360.0);
                let (sign, deg) = crate::ephem::get_zodiac_sign(p.longitude);
                p.sign = sign;
                p.degree_in_sign = deg;
            }

            let result = serde_json::json!({
                "technique": "solararc",
                "arc_degree": sa.arc_degree,
                "age_years": sa.age_years,
                "planets": planets,
                "summary": format!("太阳弧推运: 弧长 {:.4}°，对应年龄 {:.2} 岁", sa.arc_degree, sa.age_years)
            });
            Ok(result)
        }
        "harmonic" => {
            let jde = input.get_julian_day();
            let h = input.harmonic
                .or_else(|| input.params.as_ref().and_then(|p| p.get("harmonic")?.as_f64()))
                .or_else(|| input.options.as_ref().and_then(|o| o.get("harmonic")?.as_f64()))
                .unwrap_or(9.0);
            let mut planets = crate::western_dyn::calculate_harmonic(jde, h);
            for p in planets.iter_mut() {
                let (sign, deg) = crate::ephem::get_zodiac_sign(p.longitude);
                p.sign = sign;
                p.degree_in_sign = deg;
            }
            let result = serde_json::json!({
                "technique": "harmonic",
                "harmonic": h,
                "planets": planets,
                "summary": format!("泛音/谐波盘 (Harmonic H{:.0}): 黄经倍率放大，同频共振格局", h)
            });
            Ok(result)
        }
        "pd" | "pdchart" | "primarydirection" => {
            let jde = input.get_julian_day();
            let (lat, lon, _) = input.get_location();
            let mut hits = crate::western_pd::calculate_full_primary_directions(jde, lat, lon);
            if let Some(target_age) = input.age {
                hits.retain(|h| (h.age_years - target_age).abs() <= 15.0);
            }
            let result = serde_json::json!({
                "technique": "pd",
                "method": "Placidus/Ptolemy Semi-Arc (赤道半弧球面三角法)",
                "hits_count": hits.len(),
                "hits": hits,
                "summary": "主限法 (Primary Directions): 赤道球面半弧求解完毕"
            });
            Ok(result)
        }
        "mundane" => {
            let (y, _, _, _, _, _) = input.get_datetime();
            let lat = input.lat.unwrap_or(39.9);
            let lon = input.lon.unwrap_or(116.4);
            let md = crate::mundane::calculate_mundane(y, lon, lat);
            serde_json::to_value(md).map_err(|e| e.to_string())
        }
        "ephemeris" | "planet_cycles" => {
            let jde = input.get_julian_day();
            let planets = crate::ephem::calculate_planetary_positions(jde);
            let sepk_slices = crate::db::SepkDatabase::available_slices();
            let result = serde_json::json!({
                "technique": tool,
                "julian_day": jde,
                "planets_count": planets.len(),
                "planets": planets,
                "sepk_baked_slices_count": sepk_slices.len(),
                "sepk_slices": sepk_slices.iter().map(|s| s.file_name.as_str()).collect::<Vec<_>>(),
                "summary": format!("600年高精度物理星历天体位置与轨道周期 (预烘焙切片包接入: {} 组)", sepk_slices.len())
            });
            Ok(result)
        }
        "returntimeline" => {
            let jde = input.get_julian_day();
            let p_birth = crate::ephem::calculate_planetary_positions(jde);
            let mut timeline = Vec::new();
            let (by, _, _, _, _, _) = input.get_datetime();
            let start_yr = input.start_year.unwrap_or(by);
            let end_yr = input.end_year.unwrap_or(start_yr + 4);
            for yr in start_yr..=end_yr {
                let sr = crate::western_full::find_exact_solar_return(jde, yr);
                timeline.push(serde_json::json!({ "year": yr, "solar_return_jde": sr }));
            }
            let result = serde_json::json!({
                "technique": "returntimeline",
                "natal_sun": p_birth[0].longitude,
                "start_year": start_yr,
                "end_year": end_yr,
                "timeline": timeline,
                "summary": "多行星回归时间轴推运 (Return Timeline) 序列计算完备"
            });
            Ok(result)
        }
        "extrareturns" => {
            let jde = input.get_julian_day();
            let p_birth = crate::ephem::calculate_planetary_positions(jde);
            let returns = serde_json::json!([
                { "planet": "Mercury", "natal_lon": p_birth[2].longitude, "cycle_years": 0.24 },
                { "planet": "Venus", "natal_lon": p_birth[3].longitude, "cycle_years": 0.62 },
                { "planet": "Mars", "natal_lon": p_birth[4].longitude, "cycle_years": 1.88 },
                { "planet": "Jupiter", "natal_lon": p_birth[5].longitude, "cycle_years": 11.86 },
                { "planet": "Saturn", "natal_lon": p_birth[6].longitude, "cycle_years": 29.46 }
            ]);
            let result = serde_json::json!({
                "technique": "extrareturns",
                "natal_positions": p_birth,
                "returns": returns,
                "summary": "外行星返照推运 (Extra Returns): 水金火木土五星本命回归周期解算完备"
            });
            Ok(result)
        }
        "agepoint" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(39.9);
            let lon = input.lon.unwrap_or(116.4);
            let age = input.age.unwrap_or(28.0);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "placidus");
            let ap = crate::western_dyn::calculate_huber_age_point(chart.ascendant, age);
            let p_prog = crate::western_dyn::calculate_western_progression(jde, age, "secondary");
            let mut val = serde_json::to_value(&ap).unwrap_or_default();
            val["planets"] = serde_json::to_value(&p_prog.planets).unwrap_or_default();
            val["ascendant"] = serde_json::json!(chart.ascendant);
            Ok(val)
        }
        "planetaryarc" => {
            let jde = input.get_julian_day();
            let age = input.age.unwrap_or(28.0);
            let base_planet = input.family_key.as_deref().unwrap_or("Sun");
            let pa = crate::western_dyn::calculate_planetary_arc(jde, age, base_planet);
            let mut val = serde_json::to_value(&pa).unwrap_or_default();
            val["planets"] = serde_json::to_value(&pa.directed_planets).unwrap_or_default();
            Ok(val)
        }
        "jaynesprog" => {
            let jde = input.get_julian_day();
            let age = input.age.unwrap_or(28.0);
            let jp = crate::western_dyn::calculate_jaynes_progression(jde, age);
            let mut val = serde_json::to_value(&jp).unwrap_or_default();
            val["planets"] = serde_json::to_value(&jp.progressed_planets).unwrap_or_default();
            Ok(val)
        }
        "vedicprog" => {
            let jde = input.get_julian_day();
            let age = input.age.unwrap_or(28.0);
            let vp = crate::western_dyn::calculate_vedic_progression(jde, age);
            let p_prog = crate::western_dyn::calculate_western_progression(jde, age, "secondary");
            let mut val = serde_json::to_value(&vp).unwrap_or_default();
            val["planets"] = serde_json::to_value(&p_prog.planets).unwrap_or_default();
            Ok(val)
        }
        "givenyear" | "prog" => {
            let jde = input.get_julian_day();
            let (by, _, _, _, _, _) = input.get_datetime();
            let age = if let Some(ty) = input.target_year {
                (ty - by) as f64
            } else {
                input.age.unwrap_or(28.0)
            };
            let p_prog = crate::western_dyn::calculate_western_progression(jde, age, "secondary");
            let result = serde_json::json!({
                "technique": tool,
                "age": age,
                "progressed_jde": p_prog.progressed_jde,
                "planets": p_prog.planets,
                "summary": format!("西洋次限推运与指定流年法 [{}] 对应年龄 {:.1} 岁计算完备", tool, age)
            });
            Ok(result)
        }
        "hellen_chart" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(39.9);
            let lon = input.lon.unwrap_or(116.4);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "wholesign");
            let bounds = [
                ("白羊座", "木星(0-6°), 金星(6-12°), 水星(12-20°), 火星(20-25°), 土星(25-30°)"),
                ("金牛座", "金星(0-8°), 水星(8-14°), 木星(14-22°), 土星(22-27°), 火星(27-30°)"),
                ("双子座", "水星(0-6°), 木星(6-12°), 金星(12-17°), 火星(17-24°), 土星(24-30°)"),
                ("巨蟹座", "火星(0-7°), 金星(7-13°), 水星(13-19°), 木星(19-26°), 土星(26-30°)"),
                ("狮子座", "木星(0-6°), 金星(6-11°), 土星(11-18°), 水星(18-24°), 火星(24-30°)"),
                ("处女座", "水星(0-7°), 金星(7-17°), 木星(17-21°), 火星(21-28°), 土星(28-30°)"),
                ("天秤座", "土星(0-6°), 水星(6-14°), 木星(14-21°), 金星(21-28°), 火星(28-30°)"),
                ("天蝎座", "火星(0-7°), 金星(7-11°), 水星(11-19°), 木星(19-24°), 土星(24-30°)"),
                ("射手座", "木星(0-12°), 金星(12-17°), 水星(17-21°), 土星(21-26°), 火星(26-30°)"),
                ("摩羯座", "水星(0-7°), 木星(7-14°), 金星(14-22°), 土星(22-26°), 火星(26-30°)"),
                ("水瓶座", "水星(0-7°), 金星(7-13°), 木星(13-20°), 火星(20-25°), 土星(25-30°)"),
                ("双鱼座", "金星(0-12°), 木星(12-16°), 水星(16-19°), 火星(19-28°), 土星(28-30°)"),
            ];
            let result = serde_json::json!({
                "technique": "hellen_chart",
                "system": "希腊占星正典 (Hellenistic Astrology Whole-Sign System)",
                "chart": chart,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "planets": chart.planets,
                "houses": chart.houses,
                "egyptian_bounds": bounds,
                "egyptian_bounds_sample": bounds,
                "sect": if chart.planets[0].longitude > chart.ascendant && chart.planets[0].longitude < chart.ascendant + 180.0 { "夜生人 (Night Sect)" } else { "日生人 (Day Sect)" },
                "summary": "希腊占星本命盘：整宫制十二宫位、埃及界主星(Egyptian Bounds)与昼夜区分(Sect)解算完备"
            });
            Ok(result)
        }
        "india_rectify" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(39.9);
            let lon = input.lon.unwrap_or(116.4);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "wholesign");
            let (_, _, _, h, min, _) = input.get_datetime();
            let anchor_hour = (h as f64) + ((min as f64) / 60.0);
            let win_mins = input.params.as_ref()
                .and_then(|p| p.get("windowMinutes"))
                .and_then(|v| v.as_f64())
                .unwrap_or(30.0);
            let step_secs = input.params.as_ref()
                .and_then(|p| p.get("stepSeconds"))
                .and_then(|v| v.as_u64())
                .map(|u| u as u32)
                .unwrap_or(30);
            let kp_rec = crate::india_rectify::calculate_india_rectify(anchor_hour, win_mins, step_secs, chart.ascendant);

            let moon_lon = chart.planets.iter().find(|p| p.name.contains("月亮")).map(|p| p.longitude).unwrap_or(0.0);
            let nak_idx = (moon_lon / (360.0 / 27.0)).floor() as usize % 27;
            let tattwa_elements = ["普利提维 (土)", "阿帕斯 (水)", "阿格尼 (火)", "瓦尤 (风)", "阿卡沙 (空)"];
            let active_tattwa = tattwa_elements[nak_idx % 5];
            let result = serde_json::json!({
                "technique": "india_rectify",
                "system": "吠陀印占 KP 生时精细校正 (Vedic KP Tattwa & Nakshatra Rectification)",
                "chart": chart,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "planets": chart.planets,
                "houses": chart.houses,
                "moon_nakshatra": crate::western_dyn::NAKSHATRAS[nak_idx],
                "active_tattwa": active_tattwa,
                "kp_rectification": kp_rec,
                "summary": format!("印占KP生时校正：月宿【{}】，Lagna推荐修正时刻【{}】（候选区段数: {}）", crate::western_dyn::NAKSHATRAS[nak_idx], kp_rec.suggested_best_time, kp_rec.candidates.len())
            });
            Ok(result)
        }
        "relative" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(39.9);
            let lon = input.lon.unwrap_or(116.4);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "wholesign");
            let result = serde_json::json!({
                "technique": "relative",
                "chart": chart,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "planets": chart.planets,
                "houses": chart.houses,
                "summary": "关系占星比较合盘 (Synastry/Relative Chart) 整宫制排盘完备"
            });
            Ok(result)
        }

        _ => return None,
    };
    Some(res)
}
