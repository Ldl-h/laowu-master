#![allow(dead_code)]

mod bazi;
mod jieqi_table;
mod lunar_table;
mod bazi_exact;
mod ephem;
mod precession;
mod qimen;
mod ziwei;
mod liuyao;
mod zeri;
mod db;
mod liureng;
mod liureng_ext;
mod jinkou;
mod heluo;
mod shaozi;
mod tieban;
mod predictive;
mod taiyi;
mod canping;
mod yanqin;
mod xiaoliuren;
mod yizhangjing;
mod lingqi;
mod tarot;
mod guice;
mod tongshefa;
mod composite;
mod shenshu_matrix;
mod western_dyn;
mod derivation;
mod geomancy;
mod zr;
mod horary;
mod guolao;
mod vedic_feigong;
mod balbillus;
mod germany;
mod babylon;
mod hellen;
mod tongshu;
mod mundane;
mod tianxing_otherbu;
mod india_rectify;
mod bazi_inverse;
mod suzhan_zhengchuan;
mod nongli_calendar;
mod western_full;
mod western_pd;
mod western_extra;
mod formatter;
pub mod dispatch;
pub mod dispatcher;
#[cfg(test)]
mod tests;

use db::XuanshiDatabase;
use zeri::{ZeriCondition, scan_zeri};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("test");

    // 全局支持结构化 JSON 输入: xuanxue-core --tool <tool_name> --input '<json>'
    if command == "--tool" || command == "-t" {
        let tool_name = args.get(2).map(|s| s.as_str()).unwrap_or("");
        let input_str = if args.get(3).map(|s| s.as_str()) == Some("--input") {
            args.get(4).cloned().unwrap_or_default()
        } else {
            // 支持从 stdin 读取 JSON
            use std::io::Read;
            let mut buf = String::new();
            let _ = std::io::stdin().read_to_string(&mut buf);
            buf
        };

        let input: dispatcher::UniversalInput = serde_json::from_str(&input_str).unwrap_or_default();
        let res = dispatcher::dispatch_tool(tool_name, input);
        println!("{}", serde_json::to_string(&res).unwrap());
        return;
    }

    // 支持直接以子命令名执行查询元数据: xuanxue-core list / xuanxue-core spec <tool>
    if command == "list" || command == "techniques" {
        let res = dispatcher::dispatch_tool("list", Default::default());
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
        return;
    }
    if command == "spec" {
        let target = args.get(2).cloned().unwrap_or_default();
        let res = dispatcher::dispatch_tool("spec", dispatcher::UniversalInput {
            text: Some(target),
            ..Default::default()
        });
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
        return;
    }

    match command {
        "bazi" => {
            // 工具 1: 高精八字干支排盘 (以当前时刻或指定年月日时)
            let y = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2026);
            let m = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(9);
            let d = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(30);
            let h = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(14);
            let bazi = bazi_exact::calculate_exact_bazi(y, m, d, h, 0, 0);
            
            if args.iter().any(|a| a == "--snapshot") {
                let text = formatter::format_bazi_snapshot(&bazi, &format!("{}-{:02}-{:02}", y, m, d), &format!("{:02}:00", h), "男");
                let out = formatter::UnifiedToolOutput {
                    ok: true,
                    tool: "bazi",
                    data: serde_json::to_value(&bazi).unwrap(),
                    snapshot_text: text,
                };
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(&bazi).unwrap());
            }
        }
        "ziwei" => {
            // 工具 2: 紫微斗数十二宫星曜排盘
            let m = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(9);
            let d = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(30);
            let zw = ziwei::calculate_ziwei(m, d, 7, 2);
            println!("{}", serde_json::to_string_pretty(&zw).unwrap());
        }
        "qimen" => {
            // 工具 3: 奇门遁甲九宫排盘
            let qm = qimen::calculate_qimen(9, 2, 7);
            if args.iter().any(|a| a == "--snapshot") {
                let text = formatter::format_qimen_snapshot(&qm, "2026-04-04", "21:18");
                let out = formatter::UnifiedToolOutput {
                    ok: true,
                    tool: "qimen",
                    data: serde_json::to_value(&qm).unwrap(),
                    snapshot_text: text,
                };
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(&qm).unwrap());
            }
        }
        "liureng" => {
            // 工具 4: 大六壬月将加时与九宗门发端三传
            let yue_jiang = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(8); // 申将
            let zhan_shi = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(4);  // 辰时
            let day_gan = args.get(4).and_then(|s| s.chars().next()).unwrap_or('甲');
            let day_zhi = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);  // 子日
            let lr = liureng::calculate_liureng(yue_jiang, zhan_shi, day_gan, day_zhi);
            println!("{}", serde_json::to_string_pretty(&lr).unwrap());
        }
        "jinkou" => {
            // 工具 5: 大六壬金口诀四位生克与五动推断
            let day_gan = args.get(2).and_then(|s| s.chars().next()).unwrap_or('甲');
            let hour_zhi = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(6);  // 午时
            let yue_jiang = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(8); // 申将
            let di_fen = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(2);    // 寅位
            let jk = jinkou::calculate_jinkou(day_gan, hour_zhi, yue_jiang, di_fen, true);
            if args.iter().any(|a| a == "--snapshot") {
                let text = formatter::format_jinkou_snapshot(&jk, "2026-04-04", "21:18");
                let out = formatter::UnifiedToolOutput {
                    ok: true,
                    tool: "jinkou",
                    data: serde_json::to_value(&jk).unwrap(),
                    snapshot_text: text,
                };
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(&jk).unwrap());
            }
        }
        "heluo" => {
            // 工具 6: 河洛理数天地数与元堂起卦
            let y_gan = args.get(2).and_then(|s| s.chars().next()).unwrap_or('甲');
            let y_zhi = args.get(3).map(|s| s.as_str()).unwrap_or("子");
            let m_gan = args.get(4).and_then(|s| s.chars().next()).unwrap_or('丁');
            let m_zhi = args.get(5).map(|s| s.as_str()).unwrap_or("卯");
            let d_gan = args.get(6).and_then(|s| s.chars().next()).unwrap_or('庚');
            let d_zhi = args.get(7).map(|s| s.as_str()).unwrap_or("申");
            let h_gan = args.get(8).and_then(|s| s.chars().next()).unwrap_or('庚');
            let h_zhi = args.get(9).map(|s| s.as_str()).unwrap_or("辰");
            let hl = heluo::calculate_heluo(y_gan, y_zhi, m_gan, m_zhi, d_gan, d_zhi, h_gan, h_zhi, 2026, true);
            println!("{}", serde_json::to_string_pretty(&hl).unwrap());
        }
        "shaozi" => {
            // 工具 7: 邵子神数起卦与本命条文查取
            let y_gan = args.get(2).and_then(|s| s.chars().next()).unwrap_or('甲');
            let y_zhi = args.get(3).map(|s| s.as_str()).unwrap_or("子");
            let m_gan = args.get(4).and_then(|s| s.chars().next()).unwrap_or('丁');
            let m_zhi = args.get(5).map(|s| s.as_str()).unwrap_or("卯");
            let d_gan = args.get(6).and_then(|s| s.chars().next()).unwrap_or('庚');
            let d_zhi = args.get(7).map(|s| s.as_str()).unwrap_or("申");
            let h_gan = args.get(8).and_then(|s| s.chars().next()).unwrap_or('庚');
            let h_zhi = args.get(9).map(|s| s.as_str()).unwrap_or("辰");
            let sz = shaozi::calculate_shaozi(y_gan, y_zhi, m_gan, m_zhi, d_gan, d_zhi, h_gan, h_zhi, true);

            let verse_path = "xuanxue-core/data/tiaowen.bin";
            let alt_path = "data/tiaowen.bin";
            let target = if std::path::Path::new(verse_path).exists() { verse_path } else { alt_path };
            let _verse_text = if let Ok(db) = XuanshiDatabase::open(target) {
                db.get_verse("shaozi_verses", &sz.base_verse_id.to_string()).unwrap_or(None)
            } else {
                None
            };
            println!("{}", serde_json::to_string_pretty(&sz).unwrap());
        }
        "tieban" => {
            // 工具 8: 铁板神数排盘
            let m = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(8);
            let d = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(15);
            let h_zhi = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(6);
            let y_gan = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);
            let tb = tieban::calculate_tieban(m, d, h_zhi, y_gan, true);
            println!("{}", serde_json::to_string_pretty(&tb).unwrap());
        }
        "predictive" => {
            // 工具 9: 西洋古典法达与小限推运
            let age = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(28);
            let is_day = args.get(3).map(|s| s == "day").unwrap_or(true);
            let fir = predictive::calculate_firdaria(age, is_day);
            let prof = predictive::calculate_profection(age);
            let pd = predictive::calculate_primary_directions(
                &[("太阳", 185.0), ("月亮", 65.0), ("火星", 298.0)],
                &[("上升 (ASC)", 0.0), ("中天 (MC)", 270.0)],
                "Placidus 半弧法",
            );
            println!("=== 法达星限 ===\n{}\n=== 小限推运 ===\n{}\n=== 主限法 (Primary Directions) ===\n{}",
                serde_json::to_string_pretty(&fir).unwrap(),
                serde_json::to_string_pretty(&prof).unwrap(),
                serde_json::to_string_pretty(&pd).unwrap()
            );
        }
        "taiyi" => {
            // 工具 10: 太乙神数排盘
            let y = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2026);
            let m = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(9);
            let d = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(30);
            let h = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(14);
            let ty = taiyi::calculate_taiyi(y, m, d, h, 3, 0);
            if args.iter().any(|a| a == "--snapshot") {
                let text = formatter::format_taiyi_snapshot(&ty, &format!("{}-{:02}-{:02}", y, m, d), &format!("{:02}:00", h));
                let out = formatter::UnifiedToolOutput {
                    ok: true,
                    tool: "taiyi",
                    data: serde_json::to_value(&ty).unwrap(),
                    snapshot_text: text,
                };
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(&ty).unwrap());
            }
        }
        "canping" => {
            // 工具 11: 邵子参评数
            let y_gz = args.get(2).map(|s| s.as_str()).unwrap_or("丙午");
            let m_z = args.get(3).map(|s| s.as_str()).unwrap_or("酉");
            let d_z = args.get(4).map(|s| s.as_str()).unwrap_or("未");
            let h_z = args.get(5).map(|s| s.as_str()).unwrap_or("未");
            let cp = canping::calculate_canping(y_gz, m_z, d_z, h_z, false);
            if args.iter().any(|a| a == "--snapshot") {
                let text = formatter::format_canping_snapshot(&cp, y_gz, m_z, d_z, h_z, false);
                let out = formatter::UnifiedToolOutput {
                    ok: true,
                    tool: "canping",
                    data: serde_json::to_value(&cp).unwrap(),
                    snapshot_text: text,
                };
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(&cp).unwrap());
            }
        }
        "yanqin" => {
            // 工具 12: 演禽三世相法
            let y = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2026);
            let m = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(9);
            let d = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(30);
            let hz = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(7);
            let lm = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(8);
            let yq = yanqin::calculate_yanqin(y, m, d, hz, lm);
            println!("{}", serde_json::to_string_pretty(&yq).unwrap());
        }
        "xiaoliuren" => {
            // 工具 13: 小六壬三传排盘
            let n1 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(8);
            let n2 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(15);
            let n3 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(6);
            let xlr = xiaoliuren::calculate_xiaoliuren(n1, n2, n3, false);
            println!("{}", serde_json::to_string_pretty(&xlr).unwrap());
        }
        "yizhangjing" => {
            // 工具 14: 达摩一掌经
            let yz = args.get(2).map(|s| s.as_str()).unwrap_or("午");
            let lm = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(8);
            let ld = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(15);
            let hz = args.get(5).map(|s| s.as_str()).unwrap_or("申");
            let yzj = yizhangjing::calculate_yizhangjing(yz, lm, ld, hz, true);
            println!("{}", serde_json::to_string_pretty(&yzj).unwrap());
        }
        "lingqi" => {
            // 工具 15: 灵棋经
            let u = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
            let m = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
            let l = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1);
            let lq = lingqi::calculate_lingqi(u, m, l);
            println!("{}", serde_json::to_string_pretty(&lq).unwrap());
        }
        "tarot" => {
            // 工具 16: 塔罗牌
            let tr = tarot::calculate_tarot("celtic", 123456);
            println!("{}", serde_json::to_string_pretty(&tr).unwrap());
        }
        "guice" => {
            // 工具 17: 鬼谷分定经
            let y_gan = args.get(2).and_then(|s| s.chars().next()).unwrap_or('甲');
            let h_gan = args.get(3).and_then(|s| s.chars().next()).unwrap_or('丙');
            let gc = guice::calculate_guice(y_gan, h_gan);
            println!("{}", serde_json::to_string_pretty(&gc).unwrap());
        }
        "tongshefa" => {
            // 工具 18: 统摄法
            let tsf = tongshefa::calculate_tongshefa("巽", "坤", "震", "震");
            println!("{}", serde_json::to_string_pretty(&tsf).unwrap());
        }
        "sanshiunited" => {
            // 工具 19: 三式合一 (奇门 + 太乙 + 六壬)
            let ssu = composite::calculate_sanshi_united(2026, 9, 30, 14);
            println!("{}", serde_json::to_string_pretty(&ssu).unwrap());
        }
        "shenshu" => {
            // 工具 20: 十大神数同构参数矩阵
            let fam = args.get(2).map(|s| s.as_str()).unwrap_or("nanji");
            let ss = shenshu_matrix::calculate_generic_shenshu(fam, "丙午", "丁酉", "丁未", "丁未");
            println!("{}", serde_json::to_string_pretty(&ss).unwrap());
        }
        "westerndyn" => {
            // 工具 21: 西洋推运泛型算子
            let sys = args.get(2).map(|s| s.as_str()).unwrap_or("prog");
            let wd = western_dyn::calculate_western_progression(2461313.75, 28.0, sys);
            println!("{}", serde_json::to_string_pretty(&wd).unwrap());
        }
        "balbillus" => {
            // 工具 22: 巴尔比卢斯 129 年古典推运系统
            let planets = [("太阳", 185.0), ("月亮", 65.0), ("火星", 298.0), ("木星", 120.0)];
            let res = balbillus::calculate_balbillus(&planets, "太阳", false, 100.0);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "germany" => {
            // 工具 23: 汉堡学派与乌拉尼亚海外虚星
            let planets = [("太阳", 185.0), ("月亮", 65.0), ("火星", 298.0)];
            let res = germany::calculate_uranian(2461313.75, &planets);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "babylon" => {
            // 工具 24: 古巴比伦恒星黄道与微黄道
            let planets = [("太阳", 185.0), ("月亮", 65.0), ("火星", 298.0)];
            let res = babylon::calculate_babylon(&planets, "swissA10");
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "hellen" => {
            // 工具 25: 希腊古典整宫制与多玛主宰
            let planets = [("太阳", 185.0), ("月亮", 65.0), ("火星", 298.0)];
            let res = hellen::calculate_hellenistic_chart(15.0, &planets, true);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "tongshu" => {
            // 工具 26: 通书择日与董公用事吉凶
            let res = tongshu::calculate_tongshu(8, 7, 3, 1000);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "mundane" => {
            // 工具 27: 世运占星与四季入宫盘
            let res = mundane::calculate_mundane(2026, 121.47, 31.23);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "tianxing" => {
            // 工具 28: 天星择日与古法卜筮
            let conds = [tianxing_otherbu::TianXingCondition {
                body: "太阳",
                aspect: "三合 (120°)",
                target: "木星",
                orb_deg: 3.0,
            }];
            let res = tianxing_otherbu::calculate_tianxing(2461313.75, 7, &conds);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "otherbu" => {
            // 工具 29: 古法卜筮（马前课 / 文王神课）
            let res = tianxing_otherbu::calculate_otherbu("maqian", 888);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "india_rectify" => {
            // 工具 30: 印度 KP 生时校正
            let res = india_rectify::calculate_india_rectify(14.5, 30.0, 60, 45.0);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "bazi_inverse" => {
            // 工具 31: 八字四柱逆向求解反查
            let y_gz = args.get(2).map(|s| s.as_str()).unwrap_or("丙午");
            let m_gz = args.get(3).map(|s| s.as_str()).unwrap_or("丁酉");
            let d_gz = args.get(4).map(|s| s.as_str()).unwrap_or("丁未");
            let h_gz = args.get(5).map(|s| s.as_str()).unwrap_or("丁未");
            let res = bazi_inverse::calculate_bazi_inverse(y_gz, m_gz, d_gz, h_gz, 1980, 2030);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "suzhan" => {
            // 工具 32: 宿曜经宿占本命宿与流日
            let m = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(8);
            let d = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(15);
            let res = suzhan_zhengchuan::calculate_suzhan(m, d, 0);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "zhengchuan" => {
            // 工具 33: 正传六爻流派装卦
            let res = suzhan_zhengchuan::calculate_zhengchuan("乾为天", 3, "京房本宫");
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "nongli" => {
            // 工具 34: 阴阳合历高精度农历日历
            let y = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2026);
            let m = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(9);
            let res = nongli_calendar::calculate_calendar_month(y, m);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "xiaochengtu" => {
            // 工具 35: 霍斐然小成图九宫排盘
            let xct = derivation::calculate_xiaochengtu("乾", "坤");
            println!("{}", serde_json::to_string_pretty(&xct).unwrap());
        }
        "huangli" => {
            // 工具 36: 老黄历建除十二神
            let hl = derivation::calculate_huangli(8, 7);
            println!("{}", serde_json::to_string_pretty(&hl).unwrap());
        }
        "geomancy" => {
            // 工具 37: 天文地占盾牌盘
            let m1 = [2, 2, 1, 1];
            let m2 = [2, 1, 2, 1];
            let m3 = [1, 2, 2, 2];
            let m4 = [2, 1, 1, 1];
            let geo = geomancy::calculate_geomancy(m1, m2, m3, m4);
            println!("{}", serde_json::to_string_pretty(&geo).unwrap());
        }
        "zr" => {
            // 工具 38: 希腊化黄道释放大限
            let zr_res = zr::calculate_zodiacal_releasing(0, 28.0, "精神点");
            println!("{}", serde_json::to_string_pretty(&zr_res).unwrap());
        }
        "horary" => {
            // 工具 39: 西洋古典卜卦
            let hr = horary::calculate_horary(2461313.75, "白羊座", "marriage");
            println!("{}", serde_json::to_string_pretty(&hr).unwrap());
        }
        "guolao" => {
            // 工具 40: 果老星宗七政四余
            let gl = guolao::calculate_guolao(2461313.75, 45.0);
            println!("{}", serde_json::to_string_pretty(&gl).unwrap());
        }
        "vedic" => {
            // 工具 41: 印度吠陀占星 D1&D9 恒星盘
            let vd = vedic_feigong::calculate_vedic(2461313.75, 45.0);
            println!("{}", serde_json::to_string_pretty(&vd).unwrap());
        }
        "feigong" => {
            // 工具 42: 飞宫小奇门九星起局
            let fg = vedic_feigong::calculate_feigong(9, 30, 7);
            println!("{}", serde_json::to_string_pretty(&fg).unwrap());
        }
        "liuyao" => {
            // 工具 43: 易经六爻与梅花心易起卦
            let n1 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
            let n2 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);
            let n3 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(3);
            let ly = liuyao::calculate_liuyao(n1, n2, n3);
            if args.iter().any(|a| a == "--snapshot") {
                let text = formatter::format_sixyao_snapshot(&ly, "2026-04-04", "21:18");
                let out = formatter::UnifiedToolOutput {
                    ok: true,
                    tool: "sixyao",
                    data: serde_json::to_value(&ly).unwrap(),
                    snapshot_text: text,
                };
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(&ly).unwrap());
            }
        }
        "ephem" => {
            // 工具 44: 纯 Rust VSOP87 高精行星真视黄经
            let jde = 2461313.75;
            let planets = ephem::calculate_planetary_positions(jde);
            println!("{}", serde_json::to_string_pretty(&planets).unwrap());
        }
        "zeri" => {
            // 工具 45: 区间良辰吉时条件扫描器
            let s_date = args.get(2).map(|s| s.as_str()).unwrap_or("2026-10-01");
            let e_date = args.get(3).map(|s| s.as_str()).unwrap_or("2026-10-03");
            let field = args.get(4).map(|s| s.as_str()).unwrap_or("door");
            let val = args.get(5).map(|s| s.as_str()).unwrap_or("生门");

            let conds = vec![ZeriCondition {
                school: None,
                field: field.to_string(),
                expected: val.to_string(),
            }];

            match scan_zeri(s_date, e_date, &conds) {
                Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
                Err(e) => eprintln!("扫描错误: {}", e),
            }
        }
        "search" => {
            // 知识检索工具: 二十四史天象与古籍条文 MMap 极速只读检索
            let table = args.get(2).map(|s| s.as_str()).unwrap_or("all");
            let kw = args.get(3).map(|s| s.as_str()).unwrap_or("李淳风");

            if let Some(tiao_target) = crate::db::resolve_data_path("tiaowen.bin") {
                if let Ok(db) = XuanshiDatabase::open(tiao_target) {
                    if let Ok(records) = db.search(table, kw) {
                        println!("数据表 [{}] 查询关键词 [{}] 命中 {} 条记录:", table, kw, records.len());
                        for r in records.iter().take(3) {
                            println!("- {}", r);
                        }
                        return;
                    }
                }
            }

            if let Some(xs_target) = crate::db::resolve_data_path("xuanshi.bin") {
                if let Ok(db) = XuanshiDatabase::open(xs_target) {
                    if let Ok(records) = db.search(table, kw) {
                        println!("正史玄史表 [{}] 查询关键词 [{}] 命中 {} 条记录:", table, kw, records.len());
                        for r in records.iter().take(3) {
                            println!("- {}", r);
                        }
                        return;
                    }
                }
            }

            eprintln!("未找到记录或表 [{}]", table);
        }
        _ => {
            println!("============================================================");
            println!("  Xuanxue-Core (玄学全算力微引擎 - 纯 Rust 高精度版)");
            println!("  零 Java / 零 Node / 零 SQLite / 全自研数学与物理推导");
            println!("============================================================\n");
            println!("全功能指令集 (微工具模式，按需独立加载，内存 1.2MB):");
            println!("  xuanxue-core bazi [年] [月] [日] [时]       : 四柱八字节气排盘");
            println!("  xuanxue-core ziwei [月] [日]              : 紫微斗数十二宫主星推算");
            println!("  xuanxue-core qimen                        : 奇门遁甲九宫飞泊排盘");
            println!("  xuanxue-core liureng [月将] [占时] [干] [支]: 大六壬九宗门发端三传");
            println!("  xuanxue-core jinkou [日干] [时支] [月将] [位]: 大六壬金口诀四位生克");
            println!("  xuanxue-core heluo [四柱干支各两项...]     : 河洛理数天地数与元堂起卦");
            println!("  xuanxue-core shaozi [四柱干支各两项...]    : 邵子神数起卦与条文查取");
            println!("  xuanxue-core tieban [月] [日] [时] [支]    : 铁板神数月命五音立命与条文");
            println!("  xuanxue-core predictive [年龄] [day/night] : 西洋古典法达与小限推运");
            println!("  xuanxue-core taiyi [年] [月] [日] [时]     : 太乙神数九宫起盘与算数");
            println!("  xuanxue-core canping [年干支] [月支] [日支] [时支] : 邵子参评金锁银匙");
            println!("  xuanxue-core yanqin [年] [月] [日] [时支] [农历月] : 演禽神数翻禽倒将起课");
            println!("  xuanxue-core xiaoliuren [数1] [数2] [数3] [dao] : 小六壬三传起课");
            println!("  xuanxue-core yizhangjing [年支] [农历月] [农历日] [时支] : 达摩一掌经排盘");
            println!("  xuanxue-core lingqi [上] [中] [下]        : 灵棋经一百二十五卦推断");
            println!("  xuanxue-core tarot [牌阵] [随机种子]      : 经典塔罗牌阵与抽牌判读");
            println!("  xuanxue-core guice [年干] [时干]          : 鬼谷分定数两头钳定命");
            println!("  xuanxue-core tongshefa [太阴] [太阳] [少阳] [少阴] : 通摄法两极贯通");
            println!("  xuanxue-core sanshiunited [年] [月] [日] [时] : 三式合一 (奇门+太乙+六壬)");
            println!("  xuanxue-core shenshu [家族键] [年柱] [月柱] [日柱] [时柱] : 十大神数统摄");
            println!("  xuanxue-core westerndyn [年龄] [prog/minor/solarreturn] : 西洋推运泛型算子");
            println!("  xuanxue-core balbillus                    : 巴尔比卢斯 129 年古典推运系统");
            println!("  xuanxue-core germany                      : 汉堡学派与乌拉尼亚海外虚星");
            println!("  xuanxue-core babylon                      : 古巴比伦恒星黄道与微黄道");
            println!("  xuanxue-core hellen                       : 希腊古典整宫制与多玛主宰");
            println!("  xuanxue-core tongshu                      : 通书择日与董公用事吉凶");
            println!("  xuanxue-core mundane                      : 世运占星与四季入宫盘");
            println!("  xuanxue-core tianxing                     : 天星择日与古法卜筮");
            println!("  xuanxue-core otherbu                      : 诸葛马前课与文王神课");
            println!("  xuanxue-core india_rectify                : 印度 KP 生时校正");
            println!("  xuanxue-core bazi_inverse [四柱各一项]    : 八字四柱逆向求解反查");
            println!("  xuanxue-core suzhan                       : 密教宿曜经二十七宿宿占");
            println!("  xuanxue-core zhengchuan                   : 正传六爻流派装卦");
            println!("  xuanxue-core nongli [年] [月]             : 阴阳合历高精度农历月历");
            println!("  xuanxue-core xiaochengtu [上卦] [下卦]     : 霍斐然小成图九宫排盘");
            println!("  xuanxue-core huangli [月支索引] [日支索引] : 老黄历建除十二神");
            println!("  xuanxue-core geomancy                    : 天文地占术十六图盾牌盘");
            println!("  xuanxue-core zr [星座索引] [年龄]        : 希腊化黄道释放 L1 大限");
            println!("  xuanxue-core horary [上升星座] [问类]     : 西洋古典卜卦推断");
            println!("  xuanxue-core guolao [上升度数]           : 果老星宗七政四余排盘");
            println!("  xuanxue-core vedic [上升度数]            : 印度吠陀占星 D1&D9 恒星盘");
            println!("  xuanxue-core feigong [月] [日] [时]       : 飞宫小奇门九星起局");
            println!("  xuanxue-core liuyao [数1] [数2] [数3]      : 易经六爻与梅花心易起卦");
            println!("  xuanxue-core ephem                        : 西洋天体真视黄经与落座");
            println!("  xuanxue-core zeri [起] [止] [字段] [值]   : 区间良辰吉时条件扫描器");
            println!("  xuanxue-core search [表名] [关键词]       : 正史天象与古籍条文检索");
            println!("============================================================");
        }
    }
}
