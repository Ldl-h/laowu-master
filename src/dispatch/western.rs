use serde_json::Value;
use crate::dispatcher::UniversalInput;

/// P1-13: 按行星名称定位黄经（替代硬编码数组下标）。
/// en 匹配英文名子串，cn 匹配中文名子串；未找到返回 0.0。
fn planet_lon_by_name(planets: &[crate::ephem::PlanetPosition], en: &str, cn: &str) -> f64 {
    planets
        .iter()
        .find(|p| p.name.contains(en) || p.name.contains(cn))
        .map(|p| p.longitude)
        .unwrap_or(0.0)
}

/// P2-18: 改进月亮空亡 (Void Of Course, VOC) 判定。
/// 不再用"星座末 3°"近似，而是检查月亮在离开当前星座前，
/// 是否还会与其他主要行星形成主要相位 (合/冲/拱/刑/六合)；若不会则为空亡。
fn moon_is_void_of_course(moon_lon: f64, planets: &[crate::ephem::PlanetPosition]) -> bool {
    let sign_start = (moon_lon / 30.0).floor() * 30.0;
    let sign_end = sign_start + 30.0;
    let remaining = sign_end - moon_lon; // 0..30
    // 主要相位触发点相对行星的偏移（±方向都算）
    let triggers: [f64; 9] = [0.0, 60.0, 90.0, 120.0, 180.0, -60.0, -90.0, -120.0, -180.0];
    for p in planets {
        if p.name.contains("Moon") || p.name.contains("月亮") {
            continue;
        }
        for &off in &triggers {
            let trigger = (p.longitude + off).rem_euclid(360.0);
            let rel = (trigger - moon_lon).rem_euclid(360.0);
            // 月亮在离开星座前会经过该相位点 → 不空亡
            if rel > 0.01 && rel <= remaining + 0.01 {
                return false;
            }
        }
    }
    true
}

/// 解析目标时间参数为儒略日（第二盘用）：target_date > target_year > age(岁→日)
fn target_jde(input: &UniversalInput, base_jde: f64) -> Option<f64> {
    if let Some(ref td) = input.target_date {
        if let Some((y, m, d)) = crate::dispatcher::parse_date_str(td) {
            return Some(crate::bazi_exact::to_julian_day(y, m, d, 12, 0, 0));
        }
    }
    if let Some(ty) = input.target_year {
        return Some(crate::bazi_exact::to_julian_day(ty, 6, 15, 12, 0, 0));
    }
    if let Some(a) = input.age {
        return Some(base_jde + a * 365.2422);
    }
    None
}

/// 西洋占星、古典希腊、巴比伦、乌拉尼亚、吠陀印占及推运核心分发处理器
pub fn handle_western_and_astro(tool: &str, input: &UniversalInput) -> Option<Result<Value, String>> {
    let res = match tool {
        // 古典占星与本命盘
        "chart" | "natal" | "transit" => {
            let jde = input.get_julian_day();
            let (lat, lon, _) = input.get_location();
            let hsys = input.hsys.as_deref().unwrap_or("placidus");
            // P1-16: 用户传入白名单外分宫制 → 显式错误，不再静默回退 Placidus
            if let Err(e) = crate::western_full::validate_hsys(hsys) {
                return Some(Err(e));
            }
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, hsys);

            // 联动 tiaowen.bin 烘焙萨比恩 360 度象征
            let mut sabian_info = None;
            if let Some(tiaowen_path) = crate::db::resolve_data_path("tiaowen.bin") {
                if let Ok(db) = crate::db::XuanshiDatabase::open(tiaowen_path) {
                    let sun_deg = chart.planets.iter().find(|p| p.name.contains("Sun") || p.name.contains("太阳")).map(|p| p.longitude as usize + 1).unwrap_or(1);
                    sabian_info = db.get_sabian_symbol(sun_deg);
                }
            }

            // R3/P2-4: 移除冗余 has_ephem_slices 绑定（ephemeris_source 统一由
            // crate::ephem::get_ephemeris_source() 内部一次性探测，避免重复解压）。
            let mut result = serde_json::json!({
                "technique": tool,
                "jde": chart.jde,
                "lat": chart.lat,
                "lon": chart.lon,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "armc": chart.armc,
                "house_system": chart.house_system,
                "house_system_note": chart.house_system_note,
                "houses": chart.houses,
                "planets": chart.planets,
                "aspects": chart.aspects,
                "lots": chart.lots,
                "fixed_stars": chart.fixed_stars,
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "chart": chart,
                "summary": format!("西洋占星本命盘 [{}]: 上升 ASC【{:.2}°】，中天 MC【{:.2}°】", tool, chart.ascendant, chart.mc)
            });
            if let Some(s) = sabian_info {
                result["sun_sabian_symbol"] = s;
            }
            Ok(result)
        }
        // 修复 P1-16：composite/synastry/davison 不再静默映射为单盘，改为真正的双盘计算
        "composite_chart" | "synastry" | "davison" => {
            let jde1 = input.get_julian_day();
            let (lat1, lon1, _) = input.get_location();
            // P0-2: target_date/target_year/age 缺失时返回清晰错误（保持现有清晰报错文案）
            let jde2 = match target_jde(input, jde1) {
                Some(v) => v,
                None => return Some(Err(format!("技法 [{}] 需要第二时间参数: target_date / target_year / age", tool))),
            };
            let (lat2, lon2) = if input.target_lat.is_some() && input.target_lon.is_some() {
                (input.target_lat.unwrap_or(lat1), input.target_lon.unwrap_or(lon1))
            } else {
                (lat1, lon1)
            };
            let hsys = input.hsys.as_deref().unwrap_or("placidus");
            // P1-16: 白名单外分宫制显式报错
            if let Err(e) = crate::western_full::validate_hsys(hsys) {
                return Some(Err(e));
            }

            // 组合中点时间与中点经纬度（composite 与 davison 共用）
            let jde_mid = (jde1 + jde2) / 2.0;
            let lat_mid = (lat1 + lat2) / 2.0;
            let lon_mid = (lon1 + lon2) / 2.0;

            if tool == "composite_chart" {
                // P1-5: 基于组合中点时间 + 中点经纬度构建完整星盘（ASC/MC、12宫位、行星、相位、阿拉伯点、恒星），
                // 输出结构与 davison 对齐；同时保留逐行星黄经中点数组。
                let chart1 = crate::western_full::calculate_full_astro_chart(jde1, lat1, lon1, hsys);
                let chart2 = crate::western_full::calculate_full_astro_chart(jde2, lat2, lon2, hsys);
                let cchart = crate::western_full::calculate_full_astro_chart(jde_mid, lat_mid, lon_mid, hsys);
                let mids = crate::western_full::calculate_composite_chart(jde1, jde2);
                Ok(serde_json::json!({
                    "technique": "composite_chart",
                    "jde1": jde1, "jde2": jde2,
                    "lat1": lat1, "lon1": lon1,
                    "lat2": lat2, "lon2": lon2,
                    "chart1": chart1, "chart2": chart2,
                    "jde_mid": jde_mid,
                    "lat_mid": lat_mid, "lon_mid": lon_mid,
                    "ascendant": cchart.ascendant,
                    "mc": cchart.mc,
                    "house_system": cchart.house_system,
                    "house_system_note": cchart.house_system_note,
                    "houses": cchart.houses,
                    "planets": cchart.planets,
                    "aspects": cchart.aspects,
                    "lots": cchart.lots,
                    "fixed_stars": cchart.fixed_stars,
                    "ephemeris_source": crate::ephem::get_ephemeris_source(),
                    "chart": cchart,
                    "midpoint_count": mids.len(),
                    "midpoints": mids,
                    "summary": format!("组合盘 (Composite): 中点时间 {:.1}°经纬 ({:.2},{:.2}) 完整星盘，含 {} 组逐行星中点", jde_mid, lat_mid, lon_mid, mids.len())
                }))
            } else if tool == "synastry" {
                let chart1 = crate::western_full::calculate_full_astro_chart(jde1, lat1, lon1, hsys);
                let chart2 = crate::western_full::calculate_full_astro_chart(jde2, lat2, lon2, hsys);
                let pairs: Vec<serde_json::Value> = chart1.planets.iter().zip(chart2.planets.iter()).map(|(a, b)| {
                    let mut diff = (a.longitude - b.longitude).abs();
                    if diff > 180.0 { diff = 360.0 - diff; }
                    serde_json::json!({
                        "planet": a.name,
                        "person_a_lon": a.longitude,
                        "person_b_lon": b.longitude,
                        "orb": diff,
                        "aspect_hint": if diff < 8.0 { "合相" } else if (diff - 60.0).abs() < 6.0 { "六合" } else if (diff - 90.0).abs() < 8.0 { "刑" } else if (diff - 120.0).abs() < 8.0 { "三合" } else if (diff - 180.0).abs() < 8.0 { "冲" } else { "无主相位" }
                    })
                }).collect();
                Ok(serde_json::json!({
                    "technique": "synastry",
                    "jde1": jde1, "jde2": jde2,
                    "chart1": chart1,
                    "chart2": chart2,
                    "synastry_pairs": pairs,
                    "ephemeris_source": crate::ephem::get_ephemeris_source(),
                    "summary": format!("合盘 (Synastry): 双人本命盘逐行星对照，共 {} 组交互相位", pairs.len())
                }))
            } else {
                // davison: 两盘时空与经纬的中点盘（中点值已在分支顶部统一计算）
                // R3/P2-6: 输出结构与 composite_chart 对齐，补扁平字段 + ephemeris_source
                let chart1 = crate::western_full::calculate_full_astro_chart(jde1, lat1, lon1, hsys);
                let chart2 = crate::western_full::calculate_full_astro_chart(jde2, lat2, lon2, hsys);
                let dchart = crate::western_full::calculate_full_astro_chart(jde_mid, lat_mid, lon_mid, hsys);
                Ok(serde_json::json!({
                    "technique": "davison",
                    "jde1": jde1, "jde2": jde2,
                    "chart1": chart1, "chart2": chart2,
                    "jde_mid": jde_mid,
                    "lat_mid": lat_mid, "lon_mid": lon_mid,
                    "ascendant": dchart.ascendant,
                    "mc": dchart.mc,
                    "house_system": dchart.house_system,
                    "house_system_note": dchart.house_system_note,
                    "houses": dchart.houses,
                    "planets": dchart.planets,
                    "aspects": dchart.aspects,
                    "lots": dchart.lots,
                    "fixed_stars": dchart.fixed_stars,
                    "ephemeris_source": crate::ephem::get_ephemeris_source(),
                    "chart": dchart,
                    "summary": format!("戴维斯关系盘 (Davison): 双人出生时空与经纬中点盘，关系演化主轴")
                }))
            }
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
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
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
            // P2-4: 十三分盘相位（在 13 倍展开后的行星位置上复用相位计算逻辑）
            let h13_aspects = crate::western_full::calculate_aspects(&h13_planets);
            let result = serde_json::json!({
                "technique": "chart13",
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "planets": h13_planets,
                "aspects": h13_aspects,
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "base_chart": chart,
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
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
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
            // 修复 P1：按名称定位太阳（不再依赖第 0 位索引，防顺序变更错算）
            let natal_sun = p_birth.iter().find(|p| p.name.contains("Sun") || p.name.contains("太阳")).map(|p| p.longitude).unwrap_or(p_birth[0].longitude);
            let return_sun = chart.planets.iter().find(|p| p.name.contains("Sun") || p.name.contains("太阳")).map(|p| p.longitude).unwrap_or(chart.planets[0].longitude);
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
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "chart": chart,
                "planets": chart.planets,
                "return_chart_location_note": "返照时刻仅由太阳回归决定（与地点无关）；返照盘的宫位/ASC按出生地经纬度计算",
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
            // P1-13: 按名称定位月亮，不再硬编码 p_birth[1]
            let natal_moon = planet_lon_by_name(&p_birth, "Moon", "月亮");
            let return_moon = planet_lon_by_name(&chart.planets, "Moon", "月亮");
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
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "chart": chart,
                "planets": chart.planets,
                "return_chart_location_note": "返照时刻仅由月亮回归决定（与地点无关）；返照盘的宫位/ASC按出生地经纬度计算",
                "summary": format!("月亮返照盘: 月亮精确回归本命度数 {:.4}° (差值 {:.6}°)", natal_moon, diff)
            });
            Ok(result)
        }
        "lunationphase" => {
            let jde = input.get_julian_day();
            let phase = crate::western_full::calculate_lunation_phase(jde);
            // R4/P2-1: 月相含 moon_lon/sun_lon 行星黄经，补 ephemeris_source
            let mut val = match serde_json::to_value(phase) {
                Ok(v) => v,
                Err(e) => return Some(Err(e.to_string())),
            };
            if let Some(obj) = val.as_object_mut() {
                obj.insert("ephemeris_source".to_string(), serde_json::json!(crate::ephem::get_ephemeris_source()));
            }
            Ok(val)
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
            // P2-1: 卜卦盘输出主要相位数组（复用 chart 相位计算）
            let result = serde_json::json!({
                "technique": "horary",
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "houses": chart.houses,
                "planets": chart.planets,
                "aspects": chart.aspects,
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
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
            let moon_lon = planet_lon_by_name(&chart.planets, "Moon", "月亮");
            // P2-18: 改进月亮空亡判定——检查离宫前是否还会与主要行星成主要相位
            let is_voc = moon_is_void_of_course(moon_lon, &chart.planets);
            let (moon_sign, moon_deg) = crate::ephem::get_zodiac_sign(moon_lon);
            let result = serde_json::json!({
                "technique": "election",
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "planets": chart.planets,
                "aspects": chart.aspects,
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "moon_sign": moon_sign,
                "moon_degree": moon_deg,
                "void_of_course": is_voc,
                "summary": format!("西洋择日占星 (Election): 择日上升点【{:.2}°】，月亮落【{} {:.2}°】，空亡判定【{}】", chart.ascendant, moon_sign, moon_deg, if is_voc { "月亮空亡" } else { "月行顺畅" })
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
            // R3/P2-2: ascendant 缺省时不再静默回退 45°，附加 note 提示用户
            let asc_from_input = input.ascendant.is_some();
            let asc_val = input.ascendant.unwrap_or(45.0);
            if let Some(obj) = val.as_object_mut() {
                obj.insert("ascendant".to_string(), serde_json::json!(asc_val));
                obj.insert("ephemeris_source".to_string(), serde_json::json!(crate::ephem::get_ephemeris_source()));
                if !asc_from_input {
                    obj.insert("note".to_string(), serde_json::json!("ascendant 未提供，使用默认 45.0°（白羊座15°），建议提供精确出生时间/地点以获得准确上升点"));
                }
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
            // R4/P2-1: 汉堡学派含行星中点/黄经，补 ephemeris_source
            let mut val = match serde_json::to_value(gm) {
                Ok(v) => v,
                Err(e) => return Some(Err(e.to_string())),
            };
            if let Some(obj) = val.as_object_mut() {
                obj.insert("ephemeris_source".to_string(), serde_json::json!(crate::ephem::get_ephemeris_source()));
            }
            Ok(val)
        }
        "guolao" | "guolao_chart" | "qizhengkin" => {
            let jde = input.get_julian_day();
            // R3/P2-2: ascendant 缺省且无经纬度时回退 45°，并附加 note 提示（不再静默）
            let mut asc_fellback = false;
            let asc = if let Some(a) = input.ascendant {
                a
            } else if input.lat.is_some() || input.lon.is_some() {
                let (la, lo, _) = input.get_location();
                crate::western_full::calculate_full_astro_chart(jde, la, lo, "placidus").ascendant
            } else {
                asc_fellback = true;
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
                        obj.insert("ephemeris_source".to_string(), serde_json::json!(crate::ephem::get_ephemeris_source()));
                        if asc_fellback {
                            obj.insert("note".to_string(), serde_json::json!("ascendant 未提供且无经纬度，使用默认 45.0°（白羊座15°），建议提供精确出生时间/地点以获得准确上升点"));
                        }
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
                obj.insert("ephemeris_source".to_string(), serde_json::json!(crate::ephem::get_ephemeris_source()));
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
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
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
            // P2-20: 七阶年龄段已抽取到 src/planetaryages.rs，dispatch 层只做调用
            let active = crate::planetaryages::active_age_band(age);
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
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
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
            // P2-5: 保留本命行星副本，用于计算太阳弧推运行星与本命行星的交叉相位
            let natal_planets = crate::ephem::calculate_planetary_positions(birth_jde);
            let mut planets = natal_planets.clone();
            for p in planets.iter_mut() {
                p.longitude = (p.longitude + sa.arc_degree).rem_euclid(360.0);
                let (sign, deg) = crate::ephem::get_zodiac_sign(p.longitude);
                p.sign = sign;
                p.degree_in_sign = deg;
            }
            let cross_aspects = crate::western_full::calculate_cross_aspects(&planets, &natal_planets);

            let result = serde_json::json!({
                "technique": "solararc",
                "arc_degree": sa.arc_degree,
                "age_years": sa.age_years,
                "planets": planets,
                "aspects": cross_aspects,
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
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
            // P2-5: 谐波行星与本命行星交叉相位
            let natal_planets = crate::ephem::calculate_planetary_positions(jde);
            let cross_aspects = crate::western_full::calculate_cross_aspects(&planets, &natal_planets);
            let result = serde_json::json!({
                "technique": "harmonic",
                "harmonic": h,
                "planets": planets,
                "aspects": cross_aspects,
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
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
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
            let md = crate::mundane::calculate_mundane(y, lon, lat);
            // R4/P2-1: 世运盘含 houses/ingresses 行星黄经，补 ephemeris_source
            let mut val = match serde_json::to_value(md) {
                Ok(v) => v,
                Err(e) => return Some(Err(e.to_string())),
            };
            if let Some(obj) = val.as_object_mut() {
                obj.insert("ephemeris_source".to_string(), serde_json::json!(crate::ephem::get_ephemeris_source()));
            }
            Ok(val)
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
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "sepk_baked_slices_count": sepk_slices.len(),
                "sepk_slices": sepk_slices.iter().map(|s| s.file_name.as_str()).collect::<Vec<_>>(),
                "summary": format!("600年高精度物理星历天体位置与轨道周期 (预烘焙切片包接入: {} 组；行星位置口径见 ephemeris_source)", sepk_slices.len())
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
                // P1-13: 按名称定位太阳，不再硬编码 p_birth[0]
                "natal_sun": planet_lon_by_name(&p_birth, "Sun", "太阳"),
                "start_year": start_yr,
                "end_year": end_yr,
                "timeline": timeline,
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "return_chart_location_note": "各年太阳回归时刻仅由太阳回归决定（与地点无关）；若据此排返照盘，宫位/ASC按出生地经纬度计算",
                "summary": "多行星回归时间轴推运 (Return Timeline) 序列计算完备"
            });
            Ok(result)
        }
        "extrareturns" => {
            let jde = input.get_julian_day();
            let p_birth = crate::ephem::calculate_planetary_positions(jde);
            // P1-13: 按名称定位水金火木土，不再硬编码 p_birth[2..6]
            let returns = serde_json::json!([
                { "planet": "Mercury", "natal_lon": planet_lon_by_name(&p_birth, "Mercury", "水星"), "cycle_years": 0.24 },
                { "planet": "Venus", "natal_lon": planet_lon_by_name(&p_birth, "Venus", "金星"), "cycle_years": 0.62 },
                { "planet": "Mars", "natal_lon": planet_lon_by_name(&p_birth, "Mars", "火星"), "cycle_years": 1.88 },
                { "planet": "Jupiter", "natal_lon": planet_lon_by_name(&p_birth, "Jupiter", "木星"), "cycle_years": 11.86 },
                { "planet": "Saturn", "natal_lon": planet_lon_by_name(&p_birth, "Saturn", "土星"), "cycle_years": 29.46 }
            ]);
            let result = serde_json::json!({
                "technique": "extrareturns",
                "natal_positions": p_birth,
                "returns": returns,
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "return_chart_location_note": "各行星回归时刻仅由该行星回归决定（与地点无关）；若据此排返照盘，宫位/ASC按出生地经纬度计算",
                "summary": "外行星返照推运 (Extra Returns): 水金火木土五星本命回归周期解算完备"
            });
            Ok(result)
        }
        "agepoint" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
            let age = input.age.unwrap_or(28.0);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "placidus");
            let ap = crate::western_dyn::calculate_huber_age_point(chart.ascendant, age);
            let p_prog = crate::western_dyn::calculate_western_progression(jde, age, "secondary");
            let mut val = serde_json::to_value(&ap).unwrap_or_default();
            val["planets"] = serde_json::to_value(&p_prog.planets).unwrap_or_default();
            val["ascendant"] = serde_json::json!(chart.ascendant);
            val["ephemeris_source"] = serde_json::json!(crate::ephem::get_ephemeris_source());
            Ok(val)
        }
        "planetaryarc" => {
            let jde = input.get_julian_day();
            let age = input.age.unwrap_or(28.0);
            let base_planet = input.family_key.as_deref().unwrap_or("Sun");
            let pa = crate::western_dyn::calculate_planetary_arc(jde, age, base_planet);
            let mut val = serde_json::to_value(&pa).unwrap_or_default();
            val["planets"] = serde_json::to_value(&pa.directed_planets).unwrap_or_default();
            val["ephemeris_source"] = serde_json::json!(crate::ephem::get_ephemeris_source());
            Ok(val)
        }
        "jaynesprog" => {
            let jde = input.get_julian_day();
            let age = input.age.unwrap_or(28.0);
            let jp = crate::western_dyn::calculate_jaynes_progression(jde, age);
            let mut val = serde_json::to_value(&jp).unwrap_or_default();
            val["planets"] = serde_json::to_value(&jp.progressed_planets).unwrap_or_default();
            // P2-5: 杰恩斯推运行星与本命行星交叉相位
            let natal_planets = crate::ephem::calculate_planetary_positions(jde);
            let cross_aspects = crate::western_full::calculate_cross_aspects(&jp.progressed_planets, &natal_planets);
            val["aspects"] = serde_json::to_value(&cross_aspects).unwrap_or_default();
            val["ephemeris_source"] = serde_json::json!(crate::ephem::get_ephemeris_source());
            Ok(val)
        }
        "vedicprog" => {
            let jde = input.get_julian_day();
            let age = input.age.unwrap_or(28.0);
            let vp = crate::western_dyn::calculate_vedic_progression(jde, age);
            let p_prog = crate::western_dyn::calculate_western_progression(jde, age, "secondary");
            let mut val = serde_json::to_value(&vp).unwrap_or_default();
            val["planets"] = serde_json::to_value(&p_prog.planets).unwrap_or_default();
            val["ephemeris_source"] = serde_json::json!(crate::ephem::get_ephemeris_source());
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
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "summary": format!("西洋次限推运与指定流年法 [{}] 对应年龄 {:.1} 岁计算完备", tool, age)
            });
            Ok(result)
        }
        "hellen_chart" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "wholesign");
            // P2-11: 埃及界整表已移至 src/hellen.rs 的 EGYPTIAN_BOUNDS 常量，dispatch 只做引用
            let bounds = crate::hellen::EGYPTIAN_BOUNDS;
            let result = serde_json::json!({
                "technique": "hellen_chart",
                "system": "希腊占星正典 (Hellenistic Astrology Whole-Sign System)",
                "chart": chart,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "planets": chart.planets,
                "houses": chart.houses,
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "egyptian_bounds": bounds,
                "egyptian_bounds_sample": bounds,
                "sect": if chart.planets[0].longitude > chart.ascendant && chart.planets[0].longitude < chart.ascendant + 180.0 { "夜生人 (Night Sect)" } else { "日生人 (Day Sect)" },
                "summary": "希腊占星本命盘：整宫制十二宫位、埃及界主星(Egyptian Bounds)与昼夜区分(Sect)解算完备"
            });
            Ok(result)
        }
        "india_rectify" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
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
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "moon_nakshatra": crate::western_dyn::NAKSHATRAS[nak_idx],
                "active_tattwa": active_tattwa,
                "kp_rectification": kp_rec,
                "summary": format!("印占KP生时校正：月宿【{}】，Lagna推荐修正时刻【{}】（候选区段数: {}）", crate::western_dyn::NAKSHATRAS[nak_idx], kp_rec.suggested_best_time, kp_rec.candidates.len())
            });
            Ok(result)
        }
        "relative" => {
            let jde = input.get_julian_day();
            let lat = input.lat.unwrap_or(31.23);
            let lon = input.lon.unwrap_or(121.47);
            let chart = crate::western_full::calculate_full_astro_chart(jde, lat, lon, "wholesign");
            let result = serde_json::json!({
                "technique": "relative",
                "chart": chart,
                "ascendant": chart.ascendant,
                "mc": chart.mc,
                "planets": chart.planets,
                "houses": chart.houses,
                "ephemeris_source": crate::ephem::get_ephemeris_source(),
                "summary": "关系占星比较合盘 (Synastry/Relative Chart) 整宫制排盘完备"
            });
            Ok(result)
        }

        _ => return None,
    };
    Some(res)
}
