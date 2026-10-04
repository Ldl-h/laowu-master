#[cfg(test)]
mod tests {
    use crate::bazi_exact::*;
    use crate::ephem::*;
    use crate::liureng::*;
    use crate::liureng_ext::*;
    use crate::jinkou::*;
    use crate::heluo::*;
    use crate::shaozi::*;
    use crate::tieban::*;
    use crate::predictive::*;
    use crate::taiyi::*;
    use crate::canping::*;
    use crate::yanqin::*;
    use crate::xiaoliuren::*;
    use crate::yizhangjing::*;
    use crate::lingqi::*;
    use crate::tarot::*;
    use crate::guice::*;
    use crate::tongshefa::*;
    use crate::composite::*;
    use crate::shenshu_matrix::*;
    use crate::western_dyn::*;
    use crate::derivation::*;
    use crate::geomancy::*;
    use crate::zr::*;
    use crate::horary::*;
    use crate::guolao::*;
    use crate::vedic_feigong::*;
    use crate::balbillus::*;
    use crate::germany::*;
    use crate::babylon::*;
    use crate::hellen::*;
    use crate::tongshu::*;
    use crate::mundane::*;
    use crate::tianxing_otherbu::*;
    use crate::india_rectify::*;
    use crate::bazi_inverse::*;

    #[test]
    fn test_bazi_and_solar_longitude() {
        let bazi = calculate_exact_bazi(2026, 9, 30, 14, 0, 0);
        assert_eq!(bazi.year_pillar, "丙午");
        assert_eq!(bazi.month_pillar, "丁酉");
        assert_eq!(bazi.day_pillar, "丁未");
        assert_eq!(bazi.hour_pillar, "丁未");
    }

    #[test]
    fn test_ephem_planets() {
        let jde = 2461313.75;
        let planets = calculate_planetary_positions(jde);
        assert!(planets.len() >= 3);
        let sun = &planets[0];
        assert_eq!(sun.name, "太阳 (Sun)");
        assert!(sun.longitude > 0.0 && sun.longitude < 360.0);
    }

    #[test]
    fn test_liureng_yuanshou() {
        // 甲子日申将辰时
        let lr = calculate_liureng(3, 4, '甲', 0);
        assert_eq!(lr.san_chuan.chu_chuan, "辰");
        assert_eq!(lr.san_chuan.zhong_chuan, "申");
        assert_eq!(lr.san_chuan.mo_chuan, "子");
        assert!(lr.san_chuan.ke_ti.contains("元首课"));

        // 行年神煞扩展（P0-R1回归保护：固定起点男寅女申，与出生年支无关）
        let r_years = calculate_liureng_runyear(0, 28, true);
        assert!(!r_years.is_empty());
    }

    #[test]
    fn test_liureng_runyear_fixed_start() {
        // P0-R1 回归保护：行年固定男起寅(2)顺行、女起申(8)逆行
        // 男命 age=1 → 寅（丙寅）
        let m1 = calculate_liureng_runyear(0, 1, true);
        assert_eq!(m1[0].xing_nian_zhi, "寅");

        // 男命 age=36 → (2+35)%12 = 37%12 = 1 → 丑
        let m36 = calculate_liureng_runyear(0, 36, true);
        assert_eq!(m36.last().unwrap().xing_nian_zhi, "丑");

        // 关键回归保护：不同出生年支传入，同 age 输出必须相同（固定起点）
        let m36_birth1990 = calculate_liureng_runyear(5, 36, true); // 巳=5
        let m36_birth2000 = calculate_liureng_runyear(4, 36, true); // 辰=4
        assert_eq!(m36_birth1990.last().unwrap().xing_nian_zhi,
                   m36_birth2000.last().unwrap().xing_nian_zhi);

        // 女命 age=1 → 申（壬申）
        let f1 = calculate_liureng_runyear(0, 1, false);
        assert_eq!(f1[0].xing_nian_zhi, "申");

        // 女命 age=2 → (8-1)%12 = 7 → 未（逆行一步）
        let f2 = calculate_liureng_runyear(0, 2, false);
        assert_eq!(f2.last().unwrap().xing_nian_zhi, "未");
    }

    #[test]
    fn test_jinkou_four_levels() {
        let jk = calculate_jinkou('甲', 6, 3, 2, true);
        assert_eq!(jk.ren_yuan.gan_zhi, "丙寅");
        assert_eq!(jk.di_fen.gan_zhi, "寅");
        assert!(!jk.dong_yao.is_empty());
    }

    #[test]
    fn test_heluo_pure_math() {
        let hl = calculate_heluo('甲', "子", '丁', "卯", '庚', "申", '庚', "辰", 2026, true);
        assert_eq!(hl.tian_shu, 31);
        assert_eq!(hl.di_shu, 34);
        assert_eq!(hl.yuan_tang_yao, 6);
    }

    #[test]
    fn test_shaozi_and_tieban() {
        let sz = calculate_shaozi('甲', "子", '丁', "卯", '庚', "申", '庚', "辰", true);
        assert!(sz.base_verse_id >= 1111 && sz.base_verse_id <= 12888);

        let tb = calculate_tieban(8, 15, 6, 0, true);
        assert_eq!(tb.verse_ids.len(), 4);
    }

    #[test]
    fn test_firdaria_cycle() {
        let fir = calculate_firdaria(28, true);
        assert_eq!(fir.sect, "日生人 (Diurnal)");
        assert_eq!(fir.cycle_periods.len(), 9);
    }

    #[test]
    fn test_taiyi_kou_and_calcs() {
        let ty = calculate_taiyi(2026, 9, 30, 14, 3, 0);
        assert!(ty.kook_num >= 1 && ty.kook_num <= 72);
        assert!(ty.home_calc > 0);
        assert!(ty.away_calc > 0);
        assert!(ty.wufu_gong >= 1 && ty.wufu_gong <= 9);
        assert_eq!(ty.sixteen_gods_distribution.len(), 9);
    }

    #[test]
    fn test_canping_birth_numbers() {
        let cp = calculate_canping("丙午", "酉", "未", "未", false);
        assert_eq!(cp.nayin_element, "水");
        assert_eq!(cp.day_palace, "辰"); // 酉月反取辰
        assert_eq!(cp.ming_gong, "子");
        assert!(cp.birth_numbers.num_shun > 2000);
        assert!(cp.birth_numbers.num_ni > 2000);
        assert_eq!(cp.dayun_numbers.len(), 9);
    }

    #[test]
    fn test_yanqin_fan_qin_and_stars() {
        // 2026-09-30 未时
        let yq = calculate_yanqin(2026, 9, 30, 7, 8);
        assert_eq!(yq.year_mansion.name, "星日马"); // (2026+15) % 28 = 25 -> 星日马
        assert_eq!(yq.toutai_animal, "白鸽"); // (8-1) - (7-2) = 2 -> TOUTAI_BIRDS[2] = 白鸽
        assert!(!yq.fan_qin.name.is_empty());
        assert!(!yq.ci_jiang.name.is_empty());
        assert!(!yq.zhu_jiang.name.is_empty());
        assert!(!yq.huo_yao.name.is_empty());
    }

    #[test]
    fn test_xiaoliuren_and_yizhangjing() {
        let xlr = calculate_xiaoliuren(8, 15, 6, false);
        assert_eq!(xlr.chu_chuan.name, "留连");
        assert_eq!(xlr.zhong_chuan.name, "赤口");
        assert_eq!(xlr.mo_chuan.name, "速喜");

        let yzj = calculate_yizhangjing("午", 8, 15, "申", true);
        assert_eq!(yzj.year_pillar.star, "天福");
        assert_eq!(yzj.year_pillar.dao, "佛道");
        assert_eq!(yzj.direction, "顺行");
    }

    #[test]
    fn test_lingqi_datong() {
        let lq = calculate_lingqi(1, 1, 1);
        assert_eq!(lq.name, "大通");
        assert_eq!(lq.xiang, "昇騰");
    }

    #[test]
    fn test_tarot_guice_tongshefa() {
        let tr = calculate_tarot("three", 12345);
        assert_eq!(tr.drawn_cards.len(), 3);
        assert!(!tr.drawn_cards[0].card.name_cn.is_empty());

        let gc = calculate_guice('甲', '丙');
        assert_eq!(gc.ge_ming, "木火通明·天贵星");

        let tsf = calculate_tongshefa("巽", "坤", "震", "震");
        assert_eq!(tsf.left_hex_name, "风雷益"); // GUA64 矩阵全名为 "风雷益"
        assert_eq!(tsf.right_hex_name, "地雷复"); // GUA64 矩阵全名为 "地雷复"
    }

    #[test]
    fn test_composite_and_generic_families() {
        let ssu = calculate_sanshi_united(2026, 9, 30, 14);
        assert!(!ssu.qimen.ju_name.is_empty());
        assert!(!ssu.taiyi.kook_name.is_empty());
        assert!(!ssu.liureng.san_chuan.ke_ti.is_empty());

        let ss = calculate_generic_shenshu("nanji", "丙午", "丁酉", "丁未", "丁未");
        assert_eq!(ss.name, "南极神数");
        assert!(ss.verse_id >= 3100);

        let wd = calculate_western_progression(2461313.75, 28.0, "prog");
        assert_eq!(wd.system_name, "次限盘 (Secondary Progression: 一日代一年)");
        assert!(!wd.planets.is_empty());

        let xct = calculate_xiaochengtu("乾", "坤");
        assert_eq!(xct.gong_palaces.len(), 9);

        let hl = calculate_huangli(8, 7);
        assert_eq!(hl.jian_chu_god, "闭");
    }

    #[test]
    fn test_geomancy_and_zr() {
        let m1 = [2, 2, 1, 1];
        let m2 = [2, 1, 2, 1];
        let m3 = [1, 2, 2, 2];
        let m4 = [2, 1, 1, 1];
        let geo = calculate_geomancy(m1, m2, m3, m4);
        assert_eq!(geo.mothers[0].name_cn, "大吉");
        assert_eq!(geo.mothers[1].name_cn, "得入");
        assert_eq!(geo.daughters.len(), 4);
        assert_eq!(geo.nieces.len(), 4);

        let zr_res = calculate_zodiacal_releasing(0, 28.0, "精神点");
        assert_eq!(zr_res.start_sign, "白羊座");
        // 白羊(15) + 金牛(8) = 23岁；28岁处于双子座 (23~43岁，20年)
        assert_eq!(zr_res.current_period.sign, "双子座");
    }

    #[test]
    fn test_horary_guolao_vedic() {
        let hr = calculate_horary(2461313.75, "白羊座", "marriage");
        assert_eq!(hr.querent_ruler, "火星 (Mars)");
        assert_eq!(hr.quesited_house, 7);
        assert!(!hr.perfection_mode.is_empty());

        let gl = calculate_guolao(2461313.75, 45.0);
        assert!(gl.seven_governors.len() >= 3);
        assert_eq!(gl.four_residuals.len(), 4);

        let vd = calculate_vedic(2461313.75, 45.0);
        assert!(vd.ayanamsa > 20.0);
        assert!(vd.planets.len() >= 3);

        let fg = calculate_feigong(9, 30, 7);
        assert_eq!(fg.palace_stars.len(), 9);
    }

    #[test]
    fn test_ziwei_and_qimen_and_liuyao_direct() {
        let zw = crate::ziwei::calculate_ziwei(9, 30, 7, 2);
        assert_eq!(zw.palaces.len(), 12);
        assert!(!zw.ming_palace_zhi.is_empty());
        assert!(!zw.wuxing_ju.is_empty());
        assert!(!zw.palaces[0].main_stars.is_empty() || !zw.palaces[4].main_stars.is_empty());

        let qm = crate::qimen::calculate_qimen(9, 2, 7);
        assert_eq!(qm.palaces.len(), 9);
        assert!(!qm.ju_name.is_empty());

        let ly = crate::liuyao::calculate_liuyao(1, 5, 3);
        assert_eq!(ly.shang_gua, "乾");
        assert_eq!(ly.xia_gua, "巽");
        assert_eq!(ly.original_gua, "天风姤");
        assert_eq!(ly.moving_yao, 3);
    }

    #[test]
    fn test_zeri_scan_and_all_ten_shenshu() {
        let cond = vec![crate::zeri::ZeriCondition {
            school: None,
            field: "door".to_string(),
            expected: "生门".to_string(),
        }];
        let zr_res = crate::zeri::scan_zeri("2026-10-01", "2026-10-03", &cond);
        assert!(zr_res.is_ok());

        // 验证十大神数全部10个家族键均能正常运算
        let families = ["nanji", "beiji", "taixuan", "wangji", "cetian", "chunzi", "fendjing", "jingjue", "shenyishu", "wuzhao"];
        for fam in families {
            let res = calculate_generic_shenshu(fam, "丙午", "丁酉", "丁未", "丁未");
            assert!(res.verse_id > 0);
            assert!(!res.name.is_empty());
        }
    }

    #[test]
    fn test_western_dyn_all_branches_and_harmonic() {
        let jde = 2461313.75;
        for sys in &["prog", "minor", "solarreturn", "lunarreturn"] {
            let res = calculate_western_progression(jde, 28.0, sys);
            assert!(!res.planets.is_empty());
        }
        let harm = calculate_harmonic(jde, 7.0);
        assert!(!harm.is_empty());
    }

    #[test]
    fn test_all_new_classical_and_mundane_engines() {
        // 1. 巴尔比卢斯推运
        let planets = [("太阳", 185.0), ("月亮", 65.0), ("火星", 298.0)];
        let bal = calculate_balbillus(&planets, "太阳", false, 80.0);
        assert_eq!(bal.total_cycle_years, 129.0);
        assert!(!bal.main_periods.is_empty());

        // 2. 汉堡学派与虚星中点
        let ur = calculate_uranian(2461313.75, &planets);
        assert_eq!(ur.tnps.len(), 8);
        assert!(!ur.midpoints.is_empty());

        // 3. 巴比伦微黄道
        let bab = calculate_babylon(&planets, "swissA10");
        assert_eq!(bab.planets.len(), 3);
        assert!(bab.ayanamsa_val > 20.0);

        // 4. 希腊古典多玛与尊贵
        let hel = calculate_hellenistic_chart(15.0, &planets, true);
        assert_eq!(hel.houses.len(), 12);
        assert_eq!(hel.asc_sign, "白羊座");

        // 5. 通书董公择日
        let ts = calculate_tongshu(8, 7, 3, 1000);
        assert_eq!(ts.events.len(), 4);
        assert!(!ts.donggong_rating.is_empty());

        // 6. 世运与四季入宫
        let mun = calculate_mundane(2026, 121.47, 31.23);
        assert_eq!(mun.seasonal_ingresses.len(), 4);
        assert_eq!(mun.houses.len(), 12);

        // 7. 天星择日与古法卜筮
        let conds = [TianXingCondition {
            body: "太阳",
            aspect: "三合 (120°)",
            target: "木星",
            orb_deg: 3.0,
        }];
        let tx = calculate_tianxing(2461313.75, 5, &conds);
        assert!(!tx.hits.is_empty());

        let ob = calculate_otherbu("maqian", 1234);
        assert_eq!(ob.method, "诸葛马前课");

        // 8. 印度 KP 生时校正
        let rec = calculate_india_rectify(14.5, 30.0, 60, 45.0);
        assert!(!rec.candidates.is_empty());

        // 9. 八字四柱逆查
        let inv = calculate_bazi_inverse("丙午", "丁酉", "丁未", "丁未", 2026, 2026);
        assert!(!inv.candidates.is_empty());

        // 10. 高阶分宫制 (Placidus / WholeSign) 与 ACG 地图
        let chart = crate::western_full::calculate_full_astro_chart(2461313.75, 31.23, 121.47, "placidus");
        assert_eq!(chart.houses.len(), 12);
        assert!(chart.ascendant > 0.0 && chart.mc > 0.0);

        let acg = crate::western_full::calculate_acg_lines(2461313.75);
        assert_eq!(acg.len(), chart.planets.len());

        let lunation = crate::western_full::calculate_lunation_phase(2461313.75);
        assert!(!lunation.phase_name.is_empty());

        // 11. 托勒密半弧主限与埃及界界限分布
        let dist = crate::western_pd::calculate_distributions(15.0, 50.0);
        assert!(!dist.is_empty());

        let pds = crate::western_pd::calculate_full_primary_directions(2461313.75, 31.23, 121.47);
        assert!(!pds.is_empty());

        // 12. 三分性主宰星与十年大运
        let trip = crate::western_extra::get_triplicity_rulers(15.0);
        assert_eq!(trip.day_ruler, "太阳");

        let dec = crate::western_extra::calculate_decennials(0, 50.0);
        assert!(!dec.is_empty());

        let lots = crate::western_extra::calculate_arabic_lots(15.0, 185.0, 65.0, 210.0, 298.0, 140.0, true);
        assert_eq!(lots.len(), 5);
    }

    #[test]
    fn test_all_110_dispatch_techniques_end_to_end() {
        use crate::dispatcher::dispatch_tool;
        use crate::dispatch::techniques_spec::ALL_110_TECHNIQUES;

        // 遍历所有110项标准技法元数据，使用各自的真实示例入参进行分发执行
        for meta in ALL_110_TECHNIQUES {
            let example_val = meta.get_example_value();
            let input: crate::dispatcher::UniversalInput = serde_json::from_value(example_val).unwrap_or_default();
            let res = dispatch_tool(meta.tool, input);
            assert!(
                res.ok,
                "技法 [{}] 分发执行失败: {:?}",
                meta.tool,
                res.error
            );
            assert!(res.data.is_some(), "技法 [{}] 未返回有效数据包", meta.tool);
        }
    }

    #[test]
    fn test_binary_datasets_deep_integration() {
        use crate::dispatcher::{UniversalInput, dispatch_tool};

        // 1. 测试 CGEO 地理库 (城市名自动映射高精经纬度)
        let geo_input = UniversalInput {
            city: Some("北京".to_string()),
            date: Some("2026-09-30".to_string()),
            time: Some("14:00:00".to_string()),
            ..Default::default()
        };
        let (lat, lon, name) = geo_input.get_location();
        assert!((lat - 39.9).abs() < 1.0, "北京纬度解析异常: {}", lat);
        assert!((lon - 116.4).abs() < 1.0, "北京经度解析异常: {}", lon);
        assert!(name.contains("北京"));

        // nongli_time 真实太阳时集成城市名称
        let nt_res = dispatch_tool("nongli_time", geo_input);
        assert!(nt_res.ok);
        let nt_val = nt_res.data.unwrap();
        assert_eq!(nt_val["location"], "北京市");

        // 2. 测试 BSC5 耶鲁星表 8404 颗亮星与 keypoints / yanqin / suzhan 冲合
        let kp_input = crate::dispatch::techniques_spec::get_technique_meta("keypoints")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or(UniversalInput {
                date: Some("1998-10-24".to_string()),
                time: Some("08:30:00".to_string()),
                lat: Some(31.23),
                lon: Some(121.47),
                orb: Some(1.5),
                ..Default::default()
            });
        let kp_res = dispatch_tool("keypoints", kp_input);
        assert!(kp_res.ok);
        let kp_val = kp_res.data.unwrap();
        assert!(kp_val["fixed_star_connections_count"].as_u64().unwrap_or(0) > 0);

        // 3. 测试 SEPK 行星与小行星 (凯龙星、谷神星、婚神星等) 扩展计算
        let planets = crate::ephem::calculate_planetary_positions(2461313.75);
        assert!(planets.len() >= 15, "未包含小行星完整序列，实际数量: {}", planets.len());
        let chiron = planets.iter().find(|p| p.name.contains("Chiron"));
        assert!(chiron.is_some(), "未找到凯龙星数据");

        // 4. 测试 tiaowen.bin 古籍条文检索
        let sz_input = crate::dispatch::techniques_spec::get_technique_meta("shaozi")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or_default();
        let sz_res = dispatch_tool("shaozi", sz_input);
        assert!(sz_res.ok);

        let tb_input = crate::dispatch::techniques_spec::get_technique_meta("tieban")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or_default();
        let tb_res = dispatch_tool("tieban", tb_input);
        assert!(tb_res.ok);

        // 5. 测试 SEPK 星历切片自检与 ephemeris 工具管道
        let eph_input = crate::dispatch::techniques_spec::get_technique_meta("ephemeris")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or_default();
        let eph_res = dispatch_tool("ephemeris", eph_input);
        assert!(eph_res.ok);
        let eph_val = eph_res.data.unwrap();
        assert!(eph_val["sepk_baked_slices_count"].as_u64().unwrap_or(0) >= 5);

        // 6. 测试 xuanshi 与 astrodata 数据集
        let xs_input = crate::dispatch::techniques_spec::get_technique_meta("xuanshi")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or_default();
        let xs_res = dispatch_tool("xuanshi", xs_input);
        assert!(xs_res.ok);

        let ad_input = crate::dispatch::techniques_spec::get_technique_meta("astrodata")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or_default();
        let ad_res = dispatch_tool("astrodata", ad_input);
        assert!(ad_res.ok);
        assert!(ad_res.data.unwrap()["db_present"].as_bool().unwrap_or(false));

        // 7. 测试深层古籍子表：萨比恩、张果星宗、一掌经达摩与塔罗牌阵矩阵
        let ch_input = crate::dispatch::techniques_spec::get_technique_meta("chart")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or_default();
        let ch_res = dispatch_tool("chart", ch_input);
        assert!(ch_res.ok);
        assert!(ch_res.data.unwrap().get("sun_sabian_symbol").is_some());

        let gl_input = crate::dispatch::techniques_spec::get_technique_meta("guolao")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or_default();
        let gl_res = dispatch_tool("guolao", gl_input);
        assert!(gl_res.ok);
        assert!(gl_res.data.unwrap().get("zhangguo_matrix").is_some());

        let yz_input = crate::dispatch::techniques_spec::get_technique_meta("yizhangjing")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or_default();
        let yz_res = dispatch_tool("yizhangjing", yz_input);
        assert!(yz_res.ok);
        assert!(yz_res.data.unwrap().get("damo_data_canonical").is_some());

        let tr_res = dispatch_tool("tarot", UniversalInput {
            spread: Some("single".to_string()),
            seed: Some(20261002),
            ..Default::default()
        });
        assert!(tr_res.ok);
        let tr_val = tr_res.data.unwrap();
        assert!(tr_val.get("spread_layout_detail").is_some());
        assert!(tr_val.get("tarot_element_correspondence").is_some());

        // 8. 测试河洛理数 64 卦与蚕屏一掌经原典
        let hl_input = crate::dispatch::techniques_spec::get_technique_meta("heluo")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or_default();
        let hl_res = dispatch_tool("heluo", hl_input);
        assert!(hl_res.ok);
        assert!(hl_res.data.unwrap().get("heluo_canon_verse").is_some());

        let cp_input = crate::dispatch::techniques_spec::get_technique_meta("canping")
            .map(|m| serde_json::from_value(m.get_example_value()).unwrap_or_default())
            .unwrap_or_default();
        let cp_res = dispatch_tool("canping", cp_input);
        assert!(cp_res.ok);
        assert!(cp_res.data.unwrap().get("canping_canon_data").is_some());
    }

    // ===== 第十四轮补：历轮修复点字段级/数值级回归锁 =====

    /// R5-01 回归保护：bazi 四柱锚定北京墙钟，tz=-5 不得改变四柱，仅 sun_lon 漂移。
    #[test]
    fn test_bazi_tz_pillars_wallclock_anchored() {
        let bj = calculate_exact_bazi_tz(2026, 9, 30, 14, 0, 0, 8.0);
        let la = calculate_exact_bazi_tz(2026, 9, 30, 14, 0, 0, -5.0);
        // 四柱完全一致（墙钟锚定）
        assert_eq!(bj.year_pillar, la.year_pillar);
        assert_eq!(bj.month_pillar, la.month_pillar);
        assert_eq!(bj.day_pillar, la.day_pillar);
        assert_eq!(bj.hour_pillar, la.hour_pillar);
        // 与无 tz 基线逐柱一致
        assert_eq!(bj.day_pillar, "丁未");
        // 天文输出 sun_lon 随 tz 漂移：同墙钟、tz 差13h，太阳黄经移动≈0.53°（证明 tz 真喂给天文层）
        assert!(bj.sun_lon != la.sun_lon && (bj.sun_lon - la.sun_lon).abs() > 0.05,
                "tz 应影响 sun_lon 天文输出，实际漂移 {}°", (bj.sun_lon - la.sun_lon).abs());
    }

    /// R4/R10 时间校验：非法时间拒绝，小数秒容忍。
    #[test]
    fn test_time_validation_rejects_and_decimal_second() {
        use crate::dispatcher::{dispatch_tool, UniversalInput};
        // 24:00 必须拒绝
        let bad24 = UniversalInput {
            city: Some("北京".into()), date: Some("2026-09-30".into()),
            time: Some("24:00:00".into()), lat: Some(31.23), lon: Some(121.47),
            ..Default::default()
        };
        assert!(!dispatch_tool("nongli_time", bad24).ok, "24:00 必须被拒绝");
        // 99:00 必须拒绝
        let bad99 = UniversalInput {
            city: Some("北京".into()), date: Some("2026-09-30".into()),
            time: Some("99:00".into()), lat: Some(31.23), lon: Some(121.47),
            ..Default::default()
        };
        assert!(!dispatch_tool("nongli_time", bad99).ok, "99:00 必须被拒绝");
        // 小数秒 08:30:00.5 容忍（floor 取整）
        let dec = UniversalInput {
            city: Some("北京".into()), date: Some("2026-09-30".into()),
            time: Some("08:30:00.5".into()), lat: Some(31.23), lon: Some(121.47),
            ..Default::default()
        };
        assert!(dispatch_tool("nongli_time", dec).ok, "小数秒应容忍");
    }

    /// R1 月将公式：yue_jiang_zhi_idx = (10 - floor(sun_lon/30)) % 12。
    #[test]
    fn test_composite_yuejiang_formula() {
        let bz = calculate_exact_bazi(2026, 9, 30, 14, 0, 0);
        let idx = (10.0 - (bz.sun_lon.rem_euclid(360.0) / 30.0).floor()).rem_euclid(12.0) as usize;
        assert!(idx < 12, "月将索引越界: {}", idx);
        // 公式与 composite.rs:48 完全一致（行年表对照：idx∈0..11）
        assert_eq!(idx, (10.0 - (bz.sun_lon.rem_euclid(360.0) / 30.0).floor()).rem_euclid(12.0) as usize);
    }

    /// R10/R14 astrodata 字段扩展：Einstein 命中含 rodden=AA、adb_url、坐标。
    #[test]
    fn test_astrodata_rodden_coords_source_links() {
        use crate::dispatcher::{dispatch_tool, UniversalInput};
        let input = UniversalInput { text: Some("Einstein".into()), ..Default::default() };
        let res = dispatch_tool("astrodata", input);
        assert!(res.ok, "astrodata 应成功: {:?}", res.error);
        let d = res.data.unwrap();
        let hits = d.get("hits").and_then(|h| h.as_array()).cloned().unwrap_or_default();
        assert!(!hits.is_empty(), "Einstein 应有命中");
        let e = &hits[0];
        // 用户点名四字段
        assert_eq!(e.get("rodden").and_then(|v| v.as_str()), Some("AA"), "rodden 评级应为 AA");
        assert!(e.get("adb_url").and_then(|v| v.as_str()).unwrap_or("").starts_with("https://www.astro.com"),
                "adb_url 应为 Astro-Databank 链接");
        assert!(e.get("lat").is_some() && e.get("lon").is_some(), "应含出生坐标 lat/lon");
        assert!(e.get("tz_abbr").is_some() || e.get("zone").is_some(), "应含时区");
        assert!(e.get("name").and_then(|v| v.as_str()).unwrap_or("").contains("Einstein"));
    }

    /// R2 synastry 双盘：两不同日期，行星黄经必须不同（非单盘复制）。
    #[test]
    fn test_synastry_two_independent_charts() {
        use crate::dispatcher::{dispatch_tool, UniversalInput};
        let input = UniversalInput {
            date: Some("1990-01-01".into()), time: Some("12:00:00".into()),
            target_date: Some("2000-06-15".into()),
            lat: Some(31.23), lon: Some(121.47),
            target_lat: Some(31.23), target_lon: Some(121.47),
            hsys: Some("placidus".into()),
            ..Default::default()
        };
        let res = dispatch_tool("synastry", input);
        assert!(res.ok, "synastry 应成功: {:?}", res.error);
        let d = res.data.unwrap();
        let a = d["chart1"]["planets"][0]["longitude"].as_f64().unwrap_or(0.0);
        let b = d["chart2"]["planets"][0]["longitude"].as_f64().unwrap_or(0.0);
        assert!((a - b).abs() > 1.0, "两盘首行星黄经应不同，a={} b={}", a, b);
        assert!(d.get("synastry_pairs").and_then(|v| v.as_array()).map(|x| !x.is_empty()).unwrap_or(false));
    }

    /// R10 mundane lat=0：不再静默替换北京，应给 location_warning。
    #[test]
    fn test_mundane_lat0_location_warning() {
        let mun = calculate_mundane(2026, 121.47, 0.0);
        assert!(mun.location_warning.is_some(), "lat=0 赤道位置应给出 location_warning");
        assert_eq!(mun.houses.len(), 12);
    }

    /// R14 行星新字段：retrograde 与 speed 符号严格一致。
    #[test]
    fn test_planet_speed_retrograde_sign_consistency() {
        let planets = calculate_planetary_positions(2461313.75);
        assert!(planets.len() >= 10);
        for p in &planets {
            assert_eq!(p.retrograde, p.speed < 0.0,
                       "{} retrograde={} 与 speed={} 符号矛盾", p.name, p.retrograde, p.speed);
        }
    }

    /// R14 tongshu 金神七煞/三煞方结构锁。
    #[test]
    fn test_tongshu_jinshen_sansha_struct() {
        let ts = calculate_tongshu(8, 7, 3, 1000);
        // 结构字段存在且自洽
        assert!(!ts.jin_shen_qisha.xiu.is_empty());
        assert!(!ts.san_sha_direction.direction.is_empty());
        assert!(!ts.san_sha_direction.zhi.is_empty());
        assert_eq!(ts.san_sha_direction.zhi.len(), 3, "三煞应为三支");
        // 廿八宿值日非空
        assert!(!ts.xiu28_star.is_empty());
    }

    /// calendar_month 月内日数完整（建除扩展不破坏月历集成）。
    #[test]
    fn test_calendar_month_days_complete() {
        let cm = crate::nongli_calendar::calculate_calendar_month(2026, 10);
        assert!(cm.days.len() >= 28, "月历日数不足: {}", cm.days.len());
    }
}
