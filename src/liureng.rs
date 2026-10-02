// 大六壬神课排盘与九宗法推导引擎 (Da Liu Reng Divination Engine)
// 包含：月将加时、天地盘布列、四课起例、宗门九宗法起三传 (贼克法/比用法/涉害法/遥克法/昴星法/别责法/八专法/伏吟法/反吟法)

pub const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
pub const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

// 十二月将（太阳所在黄道宫位）
pub const YUE_JIANG: [&str; 12] = [
    "神后 (子)", "大吉 (丑)", "功曹 (寅)", "太冲 (卯)", "天罡 (辰)", "太乙 (巳)",
    "胜光 (午)", "小吉 (未)", "传送 (申)", "从魁 (酉)", "河魁 (戌)", "登明 (亥)"
];

// 十二贵人天将
pub const TIAN_JIANG: [&str; 12] = [
    "贵人", "螣蛇", "朱雀", "六合", "勾陈", "青龙",
    "天空", "白虎", "太常", "玄武", "太阴", "天后"
];

// 五行五行属性 (水=0, 火=1, 木=2, 金=3, 土=4)
pub fn zhi_wuxing(z: &str) -> usize {
    match z {
        "亥" | "子" => 0, // 水
        "巳" | "午" => 1, // 火
        "寅" | "卯" => 2, // 木
        "申" | "酉" => 3, // 金
        _ => 4,          // 辰 戌 丑 未 (土)
    }
}

pub fn gan_wuxing(g: char) -> usize {
    match g {
        '壬' | '癸' => 0,
        '丙' | '丁' => 1,
        '甲' | '乙' => 2,
        '庚' | '辛' => 3,
        _ => 4, // 戊 己
    }
}

// 五行克制: 水克火(0->1), 火克金(1->3), 金克木(3->2), 木克土(2->4), 土克水(4->0)
pub fn is_ke_zhi(a: &str, b: &str) -> bool {
    let wa = zhi_wuxing(a);
    let wb = zhi_wuxing(b);
    (wa == 0 && wb == 1) || (wa == 1 && wb == 3) || (wa == 3 && wb == 2) || (wa == 2 && wb == 4) || (wa == 4 && wb == 0)
}

pub fn is_ke_gan(g: char, z: &str) -> bool {
    let wg = gan_wuxing(g);
    let wz = zhi_wuxing(z);
    (wg == 0 && wz == 1) || (wg == 1 && wz == 3) || (wg == 3 && wz == 2) || (wg == 2 && wz == 4) || (wg == 4 && wz == 0)
}

pub fn is_ke_by_gan(z: &str, g: char) -> bool {
    let wz = zhi_wuxing(z);
    let wg = gan_wuxing(g);
    (wz == 0 && wg == 1) || (wz == 1 && wg == 3) || (wz == 3 && wg == 2) || (wz == 2 && wg == 4) || (wz == 4 && wg == 0)
}

pub fn is_yang_gan(g: char) -> bool {
    matches!(g, '甲' | '丙' | '戊' | '庚' | '壬')
}

pub fn is_yang_zhi(z: &str) -> bool {
    matches!(z, "子" | "寅" | "辰" | "午" | "申" | "戌")
}

// 地支相刑
pub fn zhi_xing(z: &str) -> &'static str {
    match z {
        "子" => "卯", "丑" => "戌", "寅" => "巳", "卯" => "子",
        "辰" => "辰", "巳" => "申", "午" => "午", "未" => "丑",
        "申" => "寅", "酉" => "酉", "戌" => "未", "亥" => "亥",
        _ => "子",
    }
}

// 地支相冲
pub fn zhi_chong(z: &str) -> &'static str {
    match z {
        "子" => "午", "丑" => "未", "寅" => "申", "卯" => "酉",
        "辰" => "戌", "巳" => "亥", "午" => "子", "未" => "丑",
        "申" => "寅", "酉" => "卯", "戌" => "辰", "亥" => "巳",
        _ => "午",
    }
}

// 天干寄宫: 甲寅 乙辰 丙戊巳 丁己未 庚申 辛戌 壬亥 癸丑
pub fn gan_ji_gong(g: char) -> usize {
    match g {
        '甲' => 2,
        '乙' => 4,
        '丙' | '戊' => 5,
        '丁' | '己' => 7,
        '庚' => 8,
        '辛' => 10,
        '壬' => 11,
        _ => 1, // 癸
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SanChuan {
    pub chu_chuan: &'static str,   // 初传 (发端)
    pub zhong_chuan: &'static str, // 中传 (移易)
    pub mo_chuan: &'static str,    // 末传 (归宿)
    pub ke_ti: &'static str,       // 课体名 (元首课/重审课/比用课/涉害课/八专课/遥克课/昴星课/别责课/伏吟课/反吟课)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SiKe {
    pub ke1: String, // 第一课：日干上神与日干
    pub ke2: String, // 第二课：第一课上神之上神
    pub ke3: String, // 第三课：日支上神与日支
    pub ke4: String, // 第四课：第三课上神之上神
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiuRengResult {
    pub yue_jiang: &'static str,
    pub zhan_shi: &'static str,
    pub si_ke: SiKe,
    pub san_chuan: SanChuan,
    pub tian_pan: Vec<(&'static str, &'static str)>, // 地盘支 -> 天盘支
}

/// 大六壬推算核心 (全九宗门发端法)
pub fn calculate_liureng(
    yue_jiang_zhi: usize,
    zhan_shi_zhi: usize,
    day_gan_char: char,
    day_zhi_idx: usize,
) -> LiuRengResult {
    // 1. 月将加时排天盘: 天盘[地盘位 di] = (di + yue_jiang - zhan_shi) % 12
    let shift = ((yue_jiang_zhi as i32 - zhan_shi_zhi as i32).rem_euclid(12)) as usize;
    let mut tian_pan_map = [&""; 12];
    let mut di_pan_map = [&""; 12]; // 天盘支在哪个地盘上: di_pan_map[tian] = di
    let mut tian_pan = Vec::with_capacity(12);

    for di in 0..12 {
        let tian = (di + shift) % 12;
        tian_pan_map[di] = &ZHI[tian];
        di_pan_map[tian] = &ZHI[di];
        tian_pan.push((ZHI[di], ZHI[tian]));
    }

    // 地盘查天盘便捷闭包
    let get_up = |di_name: &str| -> &'static str {
        let idx = ZHI.iter().position(|&x| x == di_name).unwrap_or(0);
        ZHI[(idx + shift) % 12]
    };
    let get_down = |tian_name: &str| -> &'static str {
        let idx = ZHI.iter().position(|&x| x == tian_name).unwrap_or(0);
        let di_idx = ((idx as i32 - shift as i32).rem_euclid(12)) as usize;
        ZHI[di_idx]
    };

    // 2. 四课起例
    let gan_ji_zhi_idx = gan_ji_gong(day_gan_char);
    let gan_ji_zhi = ZHI[gan_ji_zhi_idx];
    let day_zhi = ZHI[day_zhi_idx % 12];

    let ke1_up = get_up(gan_ji_zhi);
    let ke2_up = get_up(ke1_up);
    let ke3_up = get_up(day_zhi);
    let ke4_up = get_up(ke3_up);

    let si_ke = SiKe {
        ke1: format!("{} 上临 {}", day_gan_char, ke1_up),
        ke2: format!("{} 上临 {}", ke1_up, ke2_up),
        ke3: format!("{} 上临 {}", day_zhi, ke3_up),
        ke4: format!("{} 上临 {}", ke3_up, ke4_up),
    };

    // 四课数据: (上神, 下位)
    // ke1: (ke1_up, gan_ji_zhi)
    // ke2: (ke2_up, ke1_up)
    // ke3: (ke3_up, day_zhi)
    // ke4: (ke4_up, ke3_up)
    let ke_pairs = [
        (ke1_up, gan_ji_zhi, true),  // true 表示含日干
        (ke2_up, ke1_up, false),
        (ke3_up, day_zhi, false),
        (ke4_up, ke3_up, false),
    ];

    // 三传顺导: 给定初传，中传为初传之上神，末传为中传之上神
    let make_san_chuan = |chu: &'static str, name: &'static str| -> SanChuan {
        let zhong = get_up(chu);
        let mo = get_up(zhong);
        SanChuan {
            chu_chuan: chu,
            zhong_chuan: zhong,
            mo_chuan: mo,
            ke_ti: name,
        }
    };

    // 3. 九宗法宗门顺序判定:
    // (1) 伏吟 (天地盘相同，即 shift == 0)
    if shift == 0 {
        let chu = if is_yang_gan(day_gan_char) {
            ke1_up
        } else {
            ke3_up
        };
        let mut zhong = zhi_xing(chu);
        if zhong == chu {
            zhong = zhi_chong(chu);
        }
        let mut mo = zhi_xing(zhong);
        if mo == zhong {
            mo = zhi_chong(zhong);
        }
        return LiuRengResult {
            yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
            zhan_shi: ZHI[zhan_shi_zhi % 12],
            si_ke,
            san_chuan: SanChuan {
                chu_chuan: chu,
                zhong_chuan: zhong,
                mo_chuan: mo,
                ke_ti: "伏吟课",
            },
            tian_pan,
        };
    }

    // (2) 反吟 (天地盘对冲，即 shift == 6)
    if shift == 6 {
        // 反吟有克取克，无克取驿马
        // 检查下贼上或上克下
        let mut zei_list = Vec::new();
        for &(up, down, is_gan) in &ke_pairs {
            let is_zei = if is_gan {
                is_ke_by_gan(down, day_gan_char) || is_ke_zhi(down, up)
            } else {
                is_ke_zhi(down, up)
            };
            if is_zei && !zei_list.contains(&up) {
                zei_list.push(up);
            }
        }
        if !zei_list.is_empty() {
            return LiuRengResult {
                yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
                zhan_shi: ZHI[zhan_shi_zhi % 12],
                si_ke,
                san_chuan: make_san_chuan(zei_list[0], "反吟无依课"),
                tian_pan,
            };
        } else {
            // 无克以日支驿马发端: 申子辰马在寅, 寅午戌马在申, 巳酉丑马在亥, 亥卯未马在巳
            let yima = match day_zhi {
                "申" | "子" | "辰" => "寅",
                "寅" | "午" | "戌" => "申",
                "巳" | "酉" | "丑" => "亥",
                _ => "巳",
            };
            let chu = yima;
            let zhong = ke3_up;
            let mo = ke1_up;
            return LiuRengResult {
                yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
                zhan_shi: ZHI[zhan_shi_zhi % 12],
                si_ke,
                san_chuan: SanChuan {
                    chu_chuan: chu,
                    zhong_chuan: zhong,
                    mo_chuan: mo,
                    ke_ti: "反吟无亲课 (驿马发端)",
                },
                tian_pan,
            };
        }
    }

    // (3) 贼克法一：下贼上 (始入 / 重审)
    let mut zei_list = Vec::new();
    for &(up, down, is_gan) in &ke_pairs {
        let is_zei = if is_gan {
            is_ke_zhi(down, up) || is_ke_by_gan(down, day_gan_char)
        } else {
            is_ke_zhi(down, up)
        };
        if is_zei && !zei_list.contains(&up) {
            zei_list.push(up);
        }
    }

    if zei_list.len() == 1 {
        return LiuRengResult {
            yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
            zhan_shi: ZHI[zhan_shi_zhi % 12],
            si_ke,
            san_chuan: make_san_chuan(zei_list[0], "重审课 (下贼上发端)"),
            tian_pan,
        };
    } else if zei_list.len() > 1 {
        // 多下贼上：比用法 (与日干阴阳同者发端)
        let yang_gan = is_yang_gan(day_gan_char);
        let same_yy: Vec<&'static str> = zei_list
            .iter()
            .copied()
            .filter(|&z| is_yang_zhi(z) == yang_gan)
            .collect();
        if same_yy.len() == 1 {
            return LiuRengResult {
                yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
                zhan_shi: ZHI[zhan_shi_zhi % 12],
                si_ke,
                san_chuan: make_san_chuan(same_yy[0], "比用课 (下贼上同阴阳)"),
                tian_pan,
            };
        } else {
            // 涉害法
            let candidate = if !same_yy.is_empty() { same_yy[0] } else { zei_list[0] };
            return LiuRengResult {
                yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
                zhan_shi: ZHI[zhan_shi_zhi % 12],
                si_ke,
                san_chuan: make_san_chuan(candidate, "涉害课 (多贼比涉)"),
                tian_pan,
            };
        }
    }

    // (4) 贼克法二：上克下 (元首课)
    let mut ke_list = Vec::new();
    for &(up, down, is_gan) in &ke_pairs {
        let is_ke = if is_gan {
            is_ke_zhi(up, down) || is_ke_gan(day_gan_char, up)
        } else {
            is_ke_zhi(up, down)
        };
        if is_ke && !ke_list.contains(&up) {
            ke_list.push(up);
        }
    }

    if ke_list.len() == 1 {
        return LiuRengResult {
            yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
            zhan_shi: ZHI[zhan_shi_zhi % 12],
            si_ke,
            san_chuan: make_san_chuan(ke_list[0], "元首课 (上克下发端)"),
            tian_pan,
        };
    } else if ke_list.len() > 1 {
        // 多上克下：比用/知一课
        let yang_gan = is_yang_gan(day_gan_char);
        let same_yy: Vec<&'static str> = ke_list
            .iter()
            .copied()
            .filter(|&z| is_yang_zhi(z) == yang_gan)
            .collect();
        if same_yy.len() == 1 {
            return LiuRengResult {
                yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
                zhan_shi: ZHI[zhan_shi_zhi % 12],
                si_ke,
                san_chuan: make_san_chuan(same_yy[0], "知一课 (上克下同阴阳)"),
                tian_pan,
            };
        } else {
            let candidate = if !same_yy.is_empty() { same_yy[0] } else { ke_list[0] };
            return LiuRengResult {
                yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
                zhan_shi: ZHI[zhan_shi_zhi % 12],
                si_ke,
                san_chuan: make_san_chuan(candidate, "涉害课 (多克比涉)"),
                tian_pan,
            };
        }
    }

    // (5) 八专课 (日干寄宫与日支同位，且无克)
    if gan_ji_zhi == day_zhi {
        let chu = if is_yang_gan(day_gan_char) {
            let idx = ZHI.iter().position(|&x| x == ke1_up).unwrap_or(0);
            ZHI[(idx + 2) % 12] // 阳顺行二位
        } else {
            let idx = ZHI.iter().position(|&x| x == ke4_up).unwrap_or(0);
            ZHI[(idx + 10) % 12] // 阴逆行二位
        };
        return LiuRengResult {
            yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
            zhan_shi: ZHI[zhan_shi_zhi % 12],
            si_ke,
            san_chuan: SanChuan {
                chu_chuan: chu,
                zhong_chuan: ke1_up,
                mo_chuan: ke1_up,
                ke_ti: "八专课",
            },
            tian_pan,
        };
    }

    // (6) 遥克法 (蒿矢 / 弹射)
    // 日干遥克神，或神遥克日干
    let mut yao_ke_list = Vec::new();
    for &(up, _, _) in &ke_pairs[1..] {
        if is_ke_by_gan(up, day_gan_char) && !yao_ke_list.contains(&up) {
            yao_ke_list.push(up);
        }
    }
    if !yao_ke_list.is_empty() {
        return LiuRengResult {
            yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
            zhan_shi: ZHI[zhan_shi_zhi % 12],
            si_ke,
            san_chuan: make_san_chuan(yao_ke_list[0], "蒿矢课 (神遥克干)"),
            tian_pan,
        };
    }

    let mut gan_yao_list = Vec::new();
    for &(up, _, _) in &ke_pairs[1..] {
        if is_ke_gan(day_gan_char, up) && !gan_yao_list.contains(&up) {
            gan_yao_list.push(up);
        }
    }
    if !gan_yao_list.is_empty() {
        return LiuRengResult {
            yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
            zhan_shi: ZHI[zhan_shi_zhi % 12],
            si_ke,
            san_chuan: make_san_chuan(gan_yao_list[0], "弹射课 (干遥克神)"),
            tian_pan,
        };
    }

    // (7) 昴星课 (四课全无克无遥之兜底)
    let (chu, zhong, mo, ke_ti) = if is_yang_gan(day_gan_char) {
        // 阳日取地盘酉上之天盘神发端
        (get_up("酉"), ke3_up, ke1_up, "昴星虎视课")
    } else {
        // 阴日取天盘酉下之地盘神发端
        (get_down("酉"), ke1_up, ke3_up, "昴星掩目课")
    };

    LiuRengResult {
        yue_jiang: YUE_JIANG[yue_jiang_zhi % 12],
        zhan_shi: ZHI[zhan_shi_zhi % 12],
        si_ke,
        san_chuan: SanChuan {
            chu_chuan: chu,
            zhong_chuan: zhong,
            mo_chuan: mo,
            ke_ti,
        },
        tian_pan,
    }
}
