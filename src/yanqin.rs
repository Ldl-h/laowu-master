// 演禽神数核心：二十八宿翻禽倒将与活曜推演引擎 (Yan Qin Shen Shu Engine)
// 纯 Rust 高精度零依赖实现，依古籍《演禽通纂》与《禽星易见》推演

#[derive(Debug, Clone, serde::Serialize)]
pub struct Mansion {
    pub idx: u8,               // 1..28 (1=角 ... 28=轸)
    pub name: &'static str,    // 如 "角木蛟"
    pub head: char,            // 如 '角'
    pub yao: char,             // 七曜: '木','金','土','日','月','火','水'
    pub animal: &'static str,  // 如 "蛟"
    pub wuxing: &'static str,  // 对应五行
}

pub const MANSIONS: [Mansion; 28] = [
    Mansion { idx: 1,  name: "角木蛟", head: '角', yao: '木', animal: "蛟", wuxing: "木" },
    Mansion { idx: 2,  name: "亢金龙", head: '亢', yao: '金', animal: "龙", wuxing: "金" },
    Mansion { idx: 3,  name: "氐土貉", head: '氐', yao: '土', animal: "貉", wuxing: "土" },
    Mansion { idx: 4,  name: "房日兔", head: '房', yao: '日', animal: "兔", wuxing: "火" },
    Mansion { idx: 5,  name: "心月狐", head: '心', yao: '月', animal: "狐", wuxing: "水" },
    Mansion { idx: 6,  name: "尾火虎", head: '尾', yao: '火', animal: "虎", wuxing: "火" },
    Mansion { idx: 7,  name: "箕水豹", head: '箕', yao: '水', animal: "豹", wuxing: "水" },
    Mansion { idx: 8,  name: "斗木獬", head: '斗', yao: '木', animal: "獬", wuxing: "木" },
    Mansion { idx: 9,  name: "牛金牛", head: '牛', yao: '金', animal: "牛", wuxing: "金" },
    Mansion { idx: 10, name: "女土蝠", head: '女', yao: '土', animal: "蝠", wuxing: "土" },
    Mansion { idx: 11, name: "虚日鼠", head: '虚', yao: '日', animal: "鼠", wuxing: "火" },
    Mansion { idx: 12, name: "危月燕", head: '危', yao: '月', animal: "燕", wuxing: "水" },
    Mansion { idx: 13, name: "室火猪", head: '室', yao: '火', animal: "猪", wuxing: "火" },
    Mansion { idx: 14, name: "壁水貐", head: '壁', yao: '水', animal: "貐", wuxing: "水" },
    Mansion { idx: 15, name: "奎木狼", head: '奎', yao: '木', animal: "狼", wuxing: "木" },
    Mansion { idx: 16, name: "娄金狗", head: '娄', yao: '金', animal: "狗", wuxing: "金" },
    Mansion { idx: 17, name: "胃土雉", head: '胃', yao: '土', animal: "雉", wuxing: "土" },
    Mansion { idx: 18, name: "昴日鸡", head: '昴', yao: '日', animal: "鸡", wuxing: "火" },
    Mansion { idx: 19, name: "毕月乌", head: '毕', yao: '月', animal: "乌", wuxing: "水" },
    Mansion { idx: 20, name: "觜火猴", head: '觜', yao: '火', animal: "猴", wuxing: "火" },
    Mansion { idx: 21, name: "参水猿", head: '参', yao: '水', animal: "猿", wuxing: "水" },
    Mansion { idx: 22, name: "井木犴", head: '井', yao: '木', animal: "犴", wuxing: "木" },
    Mansion { idx: 23, name: "鬼金羊", head: '鬼', yao: '金', animal: "羊", wuxing: "金" },
    Mansion { idx: 24, name: "柳土獐", head: '柳', yao: '土', animal: "獐", wuxing: "土" },
    Mansion { idx: 25, name: "星日马", head: '星', yao: '日', animal: "马", wuxing: "火" },
    Mansion { idx: 26, name: "张月鹿", head: '张', yao: '月', animal: "鹿", wuxing: "水" },
    Mansion { idx: 27, name: "翼火蛇", head: '翼', yao: '火', animal: "蛇", wuxing: "火" },
    Mansion { idx: 28, name: "轸水蚓", head: '轸', yao: '水', animal: "蚓", wuxing: "水" },
];

// 二十八宿真实距星天区黄道经度基准表 (J2000历元，消除360/28平均宿度误差)
// 1角, 2亢, 3氐, 4房, 5心, 6尾, 7箕, 8斗, 9牛, 10女, 11虚, 12危, 13室, 14壁,
// 15奎, 16娄, 17胃, 18昴, 19毕, 20觜, 21参, 22井, 23鬼, 24柳, 25星, 26张, 27翼, 28轸
pub const REAL_MANSION_LONS_28: [f64; 28] = [
    203.84, // 1. 角木蛟 (角宿一 Spica)
    214.55, // 2. 亢金龙 (亢宿一)
    225.08, // 3. 氐土貉 (氐宿一)
    243.18, // 4. 房日兔 (房宿四)
    249.76, // 5. 心月狐 (心宿二 Antares)
    264.30, // 6. 尾火虎 (尾宿一)
    271.74, // 7. 箕水豹 (箕宿一)
    283.82, // 8. 斗木獬 (斗宿一)
    303.85, // 9. 牛金牛 (牛宿一)
    311.95, // 10. 女土蝠 (女宿一)
    323.41, // 11. 虚日鼠 (虚宿一)
    333.35, // 12. 危月燕 (危宿一)
    353.48, // 13. 室火猪 (室宿一)
    9.11,   // 14. 壁水貐 (壁宿一)
    20.84,  // 15. 奎木狼 (奎宿一)
    33.97,  // 16. 娄金狗 (娄宿一)
    43.43,  // 17. 胃土雉 (胃宿一)
    59.99,  // 18. 昴日鸡 (昴宿六 Alcyone)
    69.79,  // 19. 毕月乌 (毕宿五 Aldebaran)
    83.71,  // 20. 觜火猴 (觜宿一)
    85.20,  // 21. 参水猿 (参宿三)
    95.31,  // 22. 井木犴 (井宿一)
    113.62, // 23. 鬼金羊 (鬼宿一)
    124.63, // 24. 柳土獐 (柳宿一)
    147.28, // 25. 星日马 (星宿一 Alphard)
    159.04, // 26. 张月鹿 (张宿一)
    173.74, // 27. 翼火蛇 (翼宿一)
    190.58, // 28. 轸水蚓 (轸宿一)
];

pub fn get_mansion_by_idx(idx: usize) -> &'static Mansion {
    let i = (idx - 1) % 28;
    &MANSIONS[i]
}

pub fn get_mansion_by_head(head: char) -> &'static Mansion {
    for m in &MANSIONS {
        if m.head == head {
            return m;
        }
    }
    &MANSIONS[0]
}

// 连续日序儒略日编号计算 (JDN)
pub fn day_number(year: i32, month: u32, day: u32) -> i64 {
    let ay = if year < 0 { year + 1 } else { year } as i64;
    let a = (14 - month as i64) / 12;
    let y = ay + 4800 - a;
    let m = month as i64 + 12 * a - 3;
    let is_greg = year > 1582 || (year == 1582 && (month > 10 || (month == 10 && day >= 15)));
    if is_greg {
        day as i64 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
    } else {
        day as i64 + (153 * m + 2) / 5 + 365 * y + y / 4 - 32083
    }
}

// 锚点：1996-01-28 = 甲子日 = 虚日鼠(idx=11)
pub const ANCHOR_MANSION_IDX: i64 = 11;

pub fn mansion_idx_of_day(year: i32, month: u32, day: u32) -> usize {
    let anchor = day_number(1996, 1, 28);
    let diff = day_number(year, month, day) - anchor;
    ((diff + ANCHOR_MANSION_IDX - 1).rem_euclid(28) + 1) as usize
}

// 七元四将：一元60日，七元420日
pub fn yuan_jiang_of_day(year: i32, month: u32, day: u32) -> (u8, u8) {
    let anchor = day_number(1996, 1, 28);
    let diff = day_number(year, month, day) - anchor;
    let yuan = (diff.rem_euclid(420) / 60 + 1) as u8;
    let jiang = (diff.rem_euclid(60) / 15 + 1) as u8;
    (yuan, jiang)
}

// 年禽：(year + 15) mod 28, 0 -> 28
pub fn year_qin(year: i32) -> &'static Mansion {
    let mut idx = (year + 15).rem_euclid(28) as usize;
    if idx == 0 { idx = 28; }
    get_mansion_by_idx(idx)
}

// 时禽 R 环 (七曜基准起宿): 虚 鬼 箕 毕 氐 奎 翼
pub const R_RING: [char; 7] = ['虚', '鬼', '箕', '毕', '氐', '奎', '翼'];

pub fn yao_order(yao: char) -> usize {
    match yao {
        '日' => 0, '月' => 1, '火' => 2, '水' => 3, '木' => 4, '金' => 5, '土' => 6,
        _ => 0,
    }
}

// 子时正禽起宿
pub fn hour_zi_start_idx(year: i32, month: u32, day: u32, use_xun: bool) -> usize {
    let day_m = get_mansion_by_idx(mansion_idx_of_day(year, month, day));
    let (yuan, _) = yuan_jiang_of_day(year, month, day);
    let base_r = (yao_order(day_m.yao) + (yuan as usize - 1)) % 7;
    let anchor = day_number(1996, 1, 28);
    let diff = day_number(year, month, day) - anchor;
    let gz_idx = diff.rem_euclid(60) as usize;
    let xun = if use_xun { gz_idx % 10 } else { 0 };
    let head = R_RING[(base_r + xun) % 7];
    get_mansion_by_head(head).idx as usize
}

// 时禽
pub fn hour_qin(year: i32, month: u32, day: u32, hour_branch: u8, use_xun: bool) -> &'static Mansion {
    let zi_idx = hour_zi_start_idx(year, month, day, use_xun);
    let idx = (zi_idx - 1 + hour_branch as usize) % 28 + 1;
    get_mansion_by_idx(idx)
}

// 翻禽 (他禽)
#[derive(Debug, Clone, serde::Serialize)]
pub struct FanQinResult {
    pub fan_qin: &'static Mansion,
    pub hour_mansion: &'static Mansion,
    pub land_branch: u8, // 落地支 (0=子 ... 11=亥)
}

pub fn fan_qin(year: i32, month: u32, day: u32, hour_branch: u8, use_xun: bool) -> FanQinResult {
    let zi_idx = hour_zi_start_idx(year, month, day, use_xun);
    let hour_m = get_mansion_by_idx((zi_idx - 1 + hour_branch as usize) % 28 + 1);
    let day_idx = mansion_idx_of_day(year, month, day);
    let k = ((day_idx as i32 - hour_m.idx as i32).rem_euclid(28)) as usize;
    let land_branch = ((hour_branch as usize + k) % 12) as u8;
    let fan = get_mansion_by_idx((zi_idx - 1 + land_branch as usize) % 28 + 1);
    FanQinResult {
        fan_qin: fan,
        hour_mansion: hour_m,
        land_branch,
    }
}

// 倒将 (次将与主将)
#[derive(Debug, Clone, serde::Serialize)]
pub struct DaoJiangResult {
    pub ci_jiang: &'static Mansion,   // 次将 (顺数)
    pub zhu_jiang: &'static Mansion,  // 主将 (倒回)
}

pub fn dao_jiang(year: i32, month: u32, day: u32, hour_branch: u8, use_xun: bool) -> DaoJiangResult {
    let day_idx = mansion_idx_of_day(year, month, day);
    let anchor = day_number(1996, 1, 28);
    let diff = day_number(year, month, day) - anchor;
    let day_branch = (diff.rem_euclid(60) % 12) as usize;
    let fq = fan_qin(year, month, day, hour_branch, use_xun);
    let step = (hour_branch as i32 - day_branch as i32).rem_euclid(12) as usize;
    let ci = get_mansion_by_idx((day_idx - 1 + step) % 28 + 1);
    let zhu = get_mansion_by_idx(((fq.hour_mansion.idx as i32 - 1 - step as i32).rem_euclid(28) + 1) as usize);
    DaoJiangResult {
        ci_jiang: ci,
        zhu_jiang: zhu,
    }
}

// 活曜头诀自寅起
pub const HUOYAO_START: [(char, char); 7] = [
    ('日', '毕'), ('月', '尾'), ('金', '牛'), ('木', '虚'), ('水', '氐'), ('火', '奎'), ('土', '翼')
];

pub fn huo_yao(year: i32, month: u32, day: u32, hour_branch: u8) -> &'static Mansion {
    let day_m = get_mansion_by_idx(mansion_idx_of_day(year, month, day));
    let mut start_head = '毕';
    for (y, h) in HUOYAO_START.iter() {
        if *y == day_m.yao {
            start_head = *h;
            break;
        }
    }
    let start_idx = get_mansion_by_head(start_head).idx as usize;
    let steps = (hour_branch as i32 - 2).rem_euclid(12) as usize; // 寅=2
    get_mansion_by_idx((start_idx - 1 + steps) % 28 + 1)
}

// 投胎度数十二禽兽 (寅时正月起凤凰)
pub const TOUTAI_BIRDS: [&str; 12] = [
    "凤凰", "鸿雁", "白鸽", "金鸡", "孔雀", "双雁", "朱雀", "燕子", "白鹿", "仙鹤", "鸳鸯", "狮子"
];

pub fn toutai_du(lunar_month: u32, hour_branch: u8) -> &'static str {
    let ring_pos = ((lunar_month as i32 - 1) - (hour_branch as i32 - 2)).rem_euclid(12) as usize;
    TOUTAI_BIRDS[ring_pos]
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct YanQinResult {
    pub day_mansion: &'static Mansion,
    pub year_mansion: &'static Mansion,
    pub hour_mansion: &'static Mansion,
    pub fan_qin: &'static Mansion,
    pub ci_jiang: &'static Mansion,
    pub zhu_jiang: &'static Mansion,
    pub huo_yao: &'static Mansion,
    pub toutai_animal: &'static str,
    pub yuan: u8,
    pub jiang: u8,
    pub bsc5_star_alignment: Option<String>,
}

/// 演禽神数综合起课
pub fn calculate_yanqin(year: i32, month: u32, day: u32, hour_branch: u8, lunar_month: u32) -> YanQinResult {
    let day_m = get_mansion_by_idx(mansion_idx_of_day(year, month, day));
    let year_m = year_qin(year);
    let (yuan, jiang) = yuan_jiang_of_day(year, month, day);
    let fq = fan_qin(year, month, day, hour_branch, true);
    let dj = dao_jiang(year, month, day, hour_branch, true);
    let hy = huo_yao(year, month, day, hour_branch);
    let tt = toutai_du(lunar_month, hour_branch);

    // 二十八宿演禽关联耶鲁星表 (采用真实距星黄道经度定位天区亮星)
    let star_alignment = if let Some(path) = crate::db::resolve_data_path("bsc5_stars.bin") {
        if let Ok(db) = crate::db::Bsc5StarDatabase::open(path) {
            let m_idx = mansion_idx_of_day(year, month, day);
            let true_lon = REAL_MANSION_LONS_28[(m_idx.saturating_sub(1)) % 28];
            let hits = db.find_conjunctions(true_lon, 5.0, 3.5);
            hits.first().map(|s| format!("当值宿【{}】对应星官亮星 HR-{} (黄经 {:.2}°, 星等 {:.2})", day_m.name, s.hr, s.ecl_lon, s.vmag))
        } else {
            None
        }
    } else {
        None
    };

    YanQinResult {
        day_mansion: day_m,
        year_mansion: year_m,
        hour_mansion: fq.hour_mansion,
        fan_qin: fq.fan_qin,
        ci_jiang: dj.ci_jiang,
        zhu_jiang: dj.zhu_jiang,
        huo_yao: hy,
        toutai_animal: tt,
        yuan,
        jiang,
        bsc5_star_alignment: star_alignment,
    }
}
