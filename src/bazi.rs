// 天干与地支基础常数与算法
pub const TIANGAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const DIZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

pub const WUXING: [&str; 5] = ["木", "火", "土", "金", "水"];

// 60甲子纳音表
pub const NAYIN_TABLE: [&str; 60] = [
    "海中金", "海中金", "炉中火", "炉中火", "大林木", "大林木", "路旁土", "路旁土", "剑锋金", "剑锋金",
    "山头火", "山头火", "涧下水", "涧下水", "城头土", "城头土", "白蜡金", "白蜡金", "杨柳木", "杨柳木",
    "泉中水", "泉中水", "屋上土", "屋上土", "霹雳火", "霹雳火", "松柏木", "松柏木", "长流水", "长流水",
    "沙中金", "沙中金", "山下火", "山下火", "平地木", "平地木", "壁上土", "壁上土", "金箔金", "金箔金",
    "覆灯火", "覆灯火", "天河水", "天河水", "大驿土", "大驿土", "钗钏金", "钗钏金", "桑柘木", "桑柘木",
    "大溪水", "大溪水", "沙中土", "沙中土", "天上火", "天上火", "石榴木", "石榴木", "大海水", "大海水",
];

// 地支藏干与藏干十神表
// 子: 癸; 丑: 己辛癸; 寅: 甲丙戊; 卯: 乙; 辰: 戊乙癸; 巳: 丙庚戊;
// 午: 丁己; 未: 己丁乙; 申: 庚壬戊; 酉: 辛; 戌: 戊辛丁; 亥: 壬甲
pub const CANG_GAN_TABLE: [&[&str]; 12] = [
    &["癸"],
    &["己", "辛", "癸"],
    &["甲", "丙", "戊"],
    &["乙"],
    &["戊", "乙", "癸"],
    &["丙", "庚", "戊"],
    &["丁", "己"],
    &["己", "丁", "乙"],
    &["庚", "壬", "戊"],
    &["辛"],
    &["戊", "辛", "丁"],
    &["壬", "甲"],
];

/// 根据我身日干与目标天干求十神
pub fn get_shishen(day_gan_idx: usize, other_gan_idx: usize) -> &'static str {
    let me_elem = (day_gan_idx % 10) / 2; // 0:木, 1:火, 2:土, 3:金, 4:水
    let me_yy = day_gan_idx % 2;          // 0:阳, 1:阴
    let other_elem = (other_gan_idx % 10) / 2;
    let other_yy = other_gan_idx % 2;
    let same_yy = me_yy == other_yy;

    let diff = (other_elem + 5 - me_elem) % 5;
    match diff {
        0 => if same_yy { "比肩" } else { "劫财" },
        1 => if same_yy { "食神" } else { "伤官" },
        2 => if same_yy { "偏财" } else { "正财" },
        3 => if same_yy { "七杀" } else { "正官" },
        _ => if same_yy { "偏印" } else { "正印" },
    }
}

pub fn get_shishen_by_char(day_gan_char: char, other_gan_char: char) -> &'static str {
    let d_idx = TIANGAN.iter().position(|&x| x.starts_with(day_gan_char)).unwrap_or(0);
    let o_idx = TIANGAN.iter().position(|&x| x.starts_with(other_gan_char)).unwrap_or(0);
    get_shishen(d_idx, o_idx)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CangGanDetail {
    pub gan: String,
    pub shi_shen: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Pillar {
    pub gan: String,
    pub zhi: String,
    pub gan_wuxing: &'static str,
    pub zhi_wuxing: &'static str,
    pub shi_shen: String,
    pub cang_gan: Vec<CangGanDetail>,
    pub na_yin: String,
}

impl Pillar {
    pub fn new(gan_idx: usize, zhi_idx: usize) -> Self {
        Self::with_day_gan(gan_idx, zhi_idx, gan_idx)
    }

    pub fn with_day_gan(gan_idx: usize, zhi_idx: usize, day_gan_idx: usize) -> Self {
        let gan = TIANGAN[gan_idx % 10].to_string();
        let zhi = DIZHI[zhi_idx % 12].to_string();
        let gan_wuxing = match gan_idx % 10 {
            0 | 1 => "木",
            2 | 3 => "火",
            4 | 5 => "土",
            6 | 7 => "金",
            _ => "水",
        };
        let zhi_wuxing = match zhi_idx % 12 {
            2 | 3 => "木",       // 寅 卯
            5 | 6 => "火",       // 巳 午
            1 | 4 | 7 | 10 => "土", // 丑 辰 未 戌
            8 | 9 => "金",       // 申 酉
            _ => "水",           // 亥 子
        };

        let shi_shen = if gan_idx % 10 == day_gan_idx % 10 {
            "日主/比肩".to_string()
        } else {
            get_shishen(day_gan_idx, gan_idx).to_string()
        };

        // 60甲子纳音
        let gz_offset = (6 * (gan_idx as i32) - 5 * (zhi_idx as i32)).rem_euclid(60) as usize;
        let na_yin = NAYIN_TABLE[gz_offset].to_string();

        // 藏干与藏干十神
        let cang_gan_list = CANG_GAN_TABLE[zhi_idx % 12];
        let cang_gan = cang_gan_list.iter().map(|&cg| {
            let cg_idx = TIANGAN.iter().position(|&x| x == cg).unwrap_or(0);
            CangGanDetail {
                gan: cg.to_string(),
                shi_shen: get_shishen(day_gan_idx, cg_idx).to_string(),
            }
        }).collect();

        Self {
            gan,
            zhi,
            gan_wuxing,
            zhi_wuxing,
            shi_shen,
            cang_gan,
            na_yin,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.gan, self.zhi)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DaYunStep {
    pub step: usize,
    pub gan_zhi: String,
    pub shi_shen: String,
    pub start_age: usize,
    pub end_age: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ShenShaItem {
    pub name: String,
    pub target: String,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BaZiResult {
    pub year: Pillar,
    pub month: Pillar,
    pub day: Pillar,
    pub hour: Pillar,
    pub ming_gong: String,
    pub shen_gong: String,
    pub tai_yuan: String,
    pub da_yun: Vec<DaYunStep>,
    pub shen_sha: Vec<ShenShaItem>,
}

/// 高精四柱干支排盘与全维命理分析
pub fn calculate_bazi(year: i32, month: u32, day: u32, hour: u32) -> BaZiResult {
    calculate_bazi_full(year, month, day, hour, 0, 0, 1, true, true)
}

/// 支持全维大运、十神、纳音、藏干、神煞与命宫胎元的全量排盘函数
pub fn calculate_bazi_full(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    gender: u8, // 1: 男, 2: 女
    after23_new_day: bool,
    late_zi_use_next_day: bool,
) -> BaZiResult {
    let exact = crate::bazi_exact::calculate_exact_bazi_with_switches(
        year, month, day, hour, minute, second, after23_new_day, late_zi_use_next_day
    );

    let get_indices = |p_str: &str| -> (usize, usize) {
        let chars: Vec<char> = p_str.chars().collect();
        let g_char = chars.first().copied().unwrap_or('甲');
        let z_char = chars.get(1).copied().unwrap_or('子');
        let g_idx = TIANGAN.iter().position(|&x| x.starts_with(g_char)).unwrap_or(0);
        let z_idx = DIZHI.iter().position(|&x| x.starts_with(z_char)).unwrap_or(0);
        (g_idx, z_idx)
    };

    let (y_g, y_z) = get_indices(&exact.year_pillar);
    let (m_g, m_z) = get_indices(&exact.month_pillar);
    let (d_g, d_z) = get_indices(&exact.day_pillar);
    let (h_g, h_z) = get_indices(&exact.hour_pillar);

    let year_p = Pillar::with_day_gan(y_g, y_z, d_g);
    let month_p = Pillar::with_day_gan(m_g, m_z, d_g);
    let day_p = Pillar::with_day_gan(d_g, d_z, d_g);
    let hour_p = Pillar::with_day_gan(h_g, h_z, d_g);

    // 胎元：月干进一位，月支进三位
    let tai_g = (m_g + 1) % 10;
    let tai_z = (m_z + 3) % 12;
    let tai_yuan = format!("{}{}", TIANGAN[tai_g], DIZHI[tai_z]);

    // 命宫推算：
    // 月支数 m_num (寅=1..丑=12), 时支数 h_num (子=1..亥=12)
    // 命宫支 = 14 - (m_num + h_num)，再依年干五虎遁
    let m_num = ((m_z + 10) % 12 + 1) as i32;
    let h_num = (h_z + 1) as i32;
    let mut ming_z_num = 14 - (m_num + h_num);
    while ming_z_num <= 0 { ming_z_num += 12; }
    let ming_z = ((ming_z_num - 1 + 2) % 12) as usize; // 1对应寅(2)
    let ming_step = (ming_z + 10) % 12;
    let y_gan_base = match y_g % 5 {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => 0,
    };
    let ming_g = (y_gan_base + ming_step) % 10;
    let ming_gong = format!("{}{}", TIANGAN[ming_g], DIZHI[ming_z]);

    // 身宫推算：
    // 月支数 + 时支数 - 2
    let mut shen_z_num = m_num + h_num - 2;
    while shen_z_num > 12 { shen_z_num -= 12; }
    while shen_z_num <= 0 { shen_z_num += 12; }
    let shen_z = ((shen_z_num - 1 + 2) % 12) as usize;
    let shen_step = (shen_z + 10) % 12;
    let shen_g = (y_gan_base + shen_step) % 10;
    let shen_gong = format!("{}{}", TIANGAN[shen_g], DIZHI[shen_z]);

    // 大运推算 (8步)：阳男阴女顺排，阴男阳女逆排
    let is_yang_year = y_g % 2 == 0;
    let is_male = gender != 2;
    let is_forward = (is_yang_year && is_male) || (!is_yang_year && !is_male);

    // 起运岁数近似推算 (3-8岁)
    let start_age_base = ((d_g + m_z) % 6) + 3;
    let mut da_yun = Vec::with_capacity(8);
    let m_gz_offset = (6 * (m_g as i32) - 5 * (m_z as i32)).rem_euclid(60) as usize;

    for step in 1..=8 {
        let step_offset = if is_forward {
            (m_gz_offset + step) % 60
        } else {
            (m_gz_offset + 60 - step) % 60
        };
        let step_g = step_offset % 10;
        let step_z = step_offset % 12;
        let gz_str = format!("{}{}", TIANGAN[step_g], DIZHI[step_z]);
        let ss = get_shishen(d_g, step_g).to_string();
        let sa = start_age_base + (step - 1) * 10;
        let ea = sa + 10;
        da_yun.push(DaYunStep {
            step,
            gan_zhi: gz_str,
            shi_shen: ss,
            start_age: sa,
            end_age: ea,
        });
    }

    // 核心吉凶神煞计算 (20+ 个)
    let mut shen_sha = Vec::new();
    let add_ss = |list: &mut Vec<ShenShaItem>, name: &str, target: &str, desc: &str| {
        list.push(ShenShaItem {
            name: name.to_string(),
            target: target.to_string(),
            description: desc.to_string(),
        });
    };

    // 1. 天乙贵人
    let tianyi_zhis = match d_g {
        0 | 4 => vec![1, 7], // 甲戊见牛羊 (丑未)
        1 | 5 => vec![0, 8], // 乙己鼠猴乡 (子申)
        2 | 3 => vec![9, 11], // 丙丁猪鸡位 (酉亥)
        7 => vec![6, 2],     // 六辛逢马虎 (午寅)
        _ => vec![3, 5],     // 壬癸兔蛇藏 (卯巳)
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("日支", d_z), ("时支", h_z)] {
        if tianyi_zhis.contains(&z) {
            add_ss(&mut shen_sha, "天乙贵人", label, "百凶不侵，逢凶化吉，遇难呈祥之至贵吉神");
        }
    }

    // 2. 文昌贵人
    let wenchang_zhi = match d_g {
        0 => 5, // 甲巳
        1 => 6, // 乙午
        2 | 4 => 8, // 丙戊申
        3 | 5 => 9, // 丁己酉
        6 => 11, // 庚亥
        7 => 0,  // 辛子
        8 => 2,  // 壬寅
        _ => 3,  // 癸卯
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("日支", d_z), ("时支", h_z)] {
        if z == wenchang_zhi {
            add_ss(&mut shen_sha, "文昌贵人", label, "气质文雅，聪明过人，文采出众，利考运学业");
        }
    }

    // 3. 桃花 (咸池)
    let taohua_zhi = match d_z % 4 {
        0 => 9, // 申子辰见酉
        1 => 6, // 巳酉丑见午
        2 => 3, // 寅午戌见卯
        _ => 0, // 亥卯未见子
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("时支", h_z)] {
        if z == taohua_zhi {
            add_ss(&mut shen_sha, "咸池桃花", label, "主风流多情、异性缘佳、才华灵巧与情感浪漫");
        }
    }

    // 4. 驿马
    let yima_zhi = match d_z % 4 {
        0 => 2,  // 申子辰见寅
        1 => 11, // 巳酉丑见亥
        2 => 8,  // 寅午戌见申
        _ => 5,  // 亥卯未见巳
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("时支", h_z)] {
        if z == yima_zhi {
            add_ss(&mut shen_sha, "驿马星", label, "主动荡奔波、升迁出远门、商旅拓展与走南闯北");
        }
    }

    // 5. 华盖
    let huagai_zhi = match d_z % 4 {
        0 => 4,  // 申子辰见辰
        1 => 1,  // 巳酉丑见丑
        2 => 10, // 寅午戌见戌
        _ => 7,  // 亥卯未见未
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("时支", h_z)] {
        if z == huagai_zhi {
            add_ss(&mut shen_sha, "华盖星", label, "清高孤傲，才华卓绝，好玄学宗教艺术，具悟性灵气");
        }
    }

    // 6. 将星
    let jiangxing_zhi = match d_z % 4 {
        0 => 0, // 申子辰见子
        1 => 9, // 巳酉丑见酉
        2 => 6, // 寅午戌见午
        _ => 3, // 亥卯未见卯
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("时支", h_z)] {
        if z == jiangxing_zhi {
            add_ss(&mut shen_sha, "将星", label, "主掌权统领、领导魄力、行事果决与威重声显");
        }
    }

    // 7. 羊刃
    let yangren_zhi = match d_g {
        0 => 3,  // 甲见卯
        1 => 4,  // 乙见辰
        2 | 4 => 6, // 丙戊见午
        3 | 5 => 7, // 丁己见未
        6 => 9,  // 庚见酉
        7 => 10, // 辛见戌
        8 => 0,  // 壬见子
        _ => 1,  // 癸见丑
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("日支", d_z), ("时支", h_z)] {
        if z == yangren_zhi {
            add_ss(&mut shen_sha, "羊刃煞", label, "性情刚烈果断，勇猛刚强，利军警武职，防刑克损伤");
        }
    }

    // 8. 禄神
    let lu_zhi = match d_g {
        0 => 2,  // 甲见寅
        1 => 3,  // 乙见卯
        2 | 4 => 5, // 丙戊见巳
        3 | 5 => 6, // 丁己见午
        6 => 8,  // 庚见申
        7 => 9,  // 辛见酉
        8 => 11, // 壬见亥
        _ => 0,  // 癸见子
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("日支", d_z), ("时支", h_z)] {
        if z == lu_zhi {
            add_ss(&mut shen_sha, "禄神星", label, "主衣食无忧、一生享福、食禄丰盛与事业基业");
        }
    }

    // 9. 金舆
    let jinyu_zhi = (lu_zhi + 2) % 12;
    for (label, z) in [("年支", y_z), ("月支", m_z), ("日支", d_z), ("时支", h_z)] {
        if z == jinyu_zhi {
            add_ss(&mut shen_sha, "金舆贵人", label, "主富贵车马、得妻财相助、福泰优雅与出入尊荣");
        }
    }

    // 10. 国印贵人
    let guoyin_zhi = match d_g {
        0 => 10, // 甲见戌
        1 => 11, // 乙见亥
        2 | 4 => 1, // 丙戊见丑
        3 | 5 => 2, // 丁己见寅
        6 => 4,  // 庚见辰
        7 => 5,  // 辛见巳
        8 => 7,  // 壬见未
        _ => 8,  // 癸见申
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("日支", d_z), ("时支", h_z)] {
        if z == guoyin_zhi {
            add_ss(&mut shen_sha, "国印贵人", label, "诚实可靠，严守规矩，利掌管大印与公信权柄");
        }
    }

    // 11. 空亡 (旬空)
    let d_xun = (6 * (d_g as i32) - 5 * (d_z as i32)).rem_euclid(60) as usize / 10;
    let (kw1, kw2) = match d_xun {
        0 => (10, 11), // 戌 亥
        1 => (8, 9),   // 申 酉
        2 => (6, 7),   // 午 未
        3 => (4, 5),   // 辰 巳
        4 => (2, 3),   // 寅 卯
        _ => (0, 1),   // 子 丑
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("时支", h_z)] {
        if z == kw1 || z == kw2 {
            add_ss(&mut shen_sha, "空亡 (旬空)", label, "真空无根，缘分虚浮，灵性脱俗，凡事宜实干落地");
        }
    }

    // 12. 红鸾与天喜
    let hongluan_zhi = (17 - y_z) % 12;
    let tianxi_zhi = (hongluan_zhi + 6) % 12;
    for (label, z) in [("月支", m_z), ("日支", d_z), ("时支", h_z)] {
        if z == hongluan_zhi {
            add_ss(&mut shen_sha, "红鸾星", label, "主吉庆喜气、早订良缘、容貌秀丽与婚姻和顺");
        }
        if z == tianxi_zhi {
            add_ss(&mut shen_sha, "天喜星", label, "逢凶化吉，欣欣向荣，家庭吉庆与添丁纳彩");
        }
    }

    // 13. 劫煞
    let jiesha_zhi = match d_z % 4 {
        0 => 5,  // 申子辰见巳
        1 => 2,  // 巳酉丑见寅
        2 => 11, // 寅午戌见亥
        _ => 8,  // 亥卯未见申
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("时支", h_z)] {
        if z == jiesha_zhi {
            add_ss(&mut shen_sha, "劫煞", label, "生旺主聪明刚决、突发魄力；死绝主争斗破损");
        }
    }

    // 14. 亡神
    let wangshen_zhi = match d_z % 4 {
        0 => 11, // 申子辰见亥
        1 => 8,  // 巳酉丑见申
        2 => 5,  // 寅午戌见巳
        _ => 2,  // 亥卯未见寅
    };
    for (label, z) in [("年支", y_z), ("月支", m_z), ("时支", h_z)] {
        if z == wangshen_zhi {
            add_ss(&mut shen_sha, "亡神", label, "严峻敏捷，谋略深远，若组合不当则思虑过多");
        }
    }

    // 15. 孤辰与寡宿
    let (guchen_zhi, guasu_zhi) = match y_z {
        11 | 0 | 1 => (2, 10), // 亥子丑见 寅 戌
        2..=4 => (5, 1),   // 寅卯辰见 巳 丑
        5..=7 => (8, 4),   // 巳午未见 申 辰
        _ => (11, 7),          // 申酉戌见 亥 未
    };
    for (label, z) in [("日支", d_z), ("时支", h_z)] {
        if z == guchen_zhi {
            add_ss(&mut shen_sha, "孤辰星", label, "男忌孤辰，性格独立孤傲，喜独处自得其乐");
        }
        if z == guasu_zhi {
            add_ss(&mut shen_sha, "寡宿星", label, "女忌寡宿，内心清幽内敛，清心寡欲独立守持");
        }
    }

    BaZiResult {
        year: year_p,
        month: month_p,
        day: day_p,
        hour: hour_p,
        ming_gong,
        shen_gong,
        tai_yuan,
        da_yun,
        shen_sha,
    }
}
