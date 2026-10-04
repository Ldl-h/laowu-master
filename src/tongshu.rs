// 传统通书择日推演

#[derive(Debug, Clone, serde::Serialize)]
pub struct TongshuEvent {
    pub category: &'static str,
    pub name: &'static str,
    pub suitable: bool,
    pub reason: String,
}

/// 金神七煞（P2-7）：值日宿命中七星 → 大凶，三吉星亦不能解。对齐 JS donggong.js。
#[derive(Debug, Clone, serde::Serialize)]
pub struct JinShenQiSha {
    pub active: bool,
    pub xiu: String,
    pub note: String,
}

/// 三煞方（P2-7）：节气月建支三合局帝旺对冲三山。对齐 JS donggong.js。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SanShaItem {
    pub name: &'static str, // 劫煞 / 灾煞 / 岁煞
    pub zhi: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SanShaDirection {
    pub direction: String, // 东/南/西/北
    pub ju: String,        // 金局/木局/水局/火局
    pub zhi: Vec<String>,  // 三煞三支，如 [寅,卯,辰]
    pub sha_list: Vec<SanShaItem>, // 三支具名：劫煞/灾煞/岁煞（对齐 zeri.js sanshaLabel）
    pub note: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TongshuResult {
    pub ganzhi_date: String,
    pub donggong_rating: &'static str, // 上吉 / 次吉 / 平 / 凶 / 大凶
    pub xiu28_star: &'static str,      // 二十八宿值日星 (角亢氏房...)
    pub wutu_solar: &'static str,      // 乌兔太阳到山到向
    pub events: Vec<TongshuEvent>,
    pub summary: &'static str,
    pub jin_shen_qisha: JinShenQiSha,
    pub san_sha_direction: SanShaDirection,
}

pub const XIU_28: [&str; 28] = [
    "角木蛟", "亢金龙", "氐土貉", "房日兔", "心月狐", "尾火虎", "箕水豹",
    "斗木獬", "牛金牛", "女土蝠", "虚日鼠", "危月燕", "室火猪", "壁水貐",
    "奎木狼", "娄金狗", "胃土彘", "昴日鸡", "毕月乌", "觜火猴", "参水猿",
    "井木犴", "鬼金羊", "柳土獐", "星日马", "张月鹿", "翼火蛇", "轸水蚓"
];

/// 纯数学推算通书黄历择日体系
pub fn calculate_tongshu(
    month_zhi_idx: usize,
    day_zhi_idx: usize,
    day_gan_idx: usize,
    day_since_epoch: i64,
) -> TongshuResult {
    let ganzhi_date = format!(
        "{}{}",
        crate::bazi_exact::TIANGAN[day_gan_idx % 10],
        crate::bazi_exact::DIZHI[day_zhi_idx % 12]
    );

    // 1. 二十八宿值日推算 (每28天一轮转)
    // 修复 P1-8：原锚点未对齐历表，day_since_epoch 直接 rem_euclid(28) 导致值日宿整体偏移 11 天。
    // 经与 JS 原版/权威历表核对：2026-10-01=奎木狼[14]、2026-10-02=娄金狗[15]、2026-09-25=牛金牛[8]，
    // 线性一致，故在日序上补偿 +11 个宿位使纪元锚点对齐。
    const XIU_EPOCH_OFFSET: i64 = 11;
    let xiu_idx = (day_since_epoch + XIU_EPOCH_OFFSET).rem_euclid(28) as usize;
    let xiu_name = XIU_28[xiu_idx];

    // 2. 董公择日评级 (根据月令与日支生克关系)
    let step = (day_zhi_idx + 12 - month_zhi_idx) % 12;
    let (donggong_rating, summary) = match step {
        0 => ("上吉", "建日吉星高照，利见大人，所作顺达"),
        1 => ("次吉", "除日扫荡宿殃，求医治病大吉，纳采进人口"),
        2 => ("上吉", "满日丰饶富足，适合立契开市、进财聚福"),
        3 => ("平", "平日修造平直，诸事平顺，无大险亦无奇喜"),
        4 => ("大吉", "定日冠带订盟，百事大成，长长久久"),
        5 => ("次吉", "执日固守操持，宜纳财立券，忌远行迁居"),
        6 => ("凶", "破日冲破太岁，大事不宜，唯利除旧布新、破贼破土"),
        7 => ("平", "危日登高涉险，行事宜慎，安床修造吉"),
        8 => ("大吉", "成日成全圆满，大吉大利，婚姻立业开市俱吉"),
        9 => ("次吉", "收日收获收藏，纳财聚宝大吉，进人口大喜"),
        10 => ("大吉", "开日普天同庆，开市兴造、出门纳财最显光华"),
        _ => ("凶", "闭日天地闭塞，诸事潜伏，宜筑堤修墙、埋藏休养"),
    };

    // 3. 乌兔太阳到山
    let wutu = match day_zhi_idx % 4 {
        0 => "太阳天德高照，百恶辟易",
        1 => "太阴吉星拱临，阴阳调和",
        2 => "紫气迎阳，利祈福造作",
        _ => "天乙护佑，吉神随行",
    };

    // 4. 关键用事宜忌判定
    let events = vec![
        TongshuEvent {
            category: "婚姻",
            name: "嫁娶·订盟",
            suitable: step == 4 || step == 8 || step == 10,
            reason: if step == 4 || step == 8 || step == 10 {
                "定成开吉日，天作之合，白头偕老。".to_string()
            } else {
                "非婚姻大吉之日，防冲克生龃龉。".to_string()
            },
        },
        TongshuEvent {
            category: "营建",
            name: "动土·修造",
            suitable: step == 0 || step == 8 || step == 10,
            reason: if step == 0 || step == 8 || step == 10 {
                "地气和顺，兴工动土家宅安宁。".to_string()
            } else {
                "动土防破败损伤，宜择吉开工。".to_string()
            },
        },
        TongshuEvent {
            category: "商贾",
            name: "开市·纳财",
            suitable: step == 2 || step == 8 || step == 10,
            reason: if step == 2 || step == 8 || step == 10 {
                "满开成吉日，财门大开，生意兴隆。".to_string()
            } else {
                "利息平平，宜稳妥结算。".to_string()
            },
        },
        TongshuEvent {
            category: "出行",
            name: "远行·移徙",
            suitable: step == 0 || step == 10,
            reason: if step == 0 || step == 10 {
                "建开之日利出关远行，前程广阔。".to_string()
            } else {
                "不宜远涉关津，以防舟车劳顿或阻隔。".to_string()
            },
        },
    ];

    // 5. 金神七煞（P2-7，对齐 JS donggong.js:DONGGONG_JINSHEN_XIU）：
    //    值日宿命中 角/亢/奎/娄/鬼/牛/星 七星 → 大凶，三吉星亦不能解。
    //    对应 XIU_28 索引：角0 亢1 牛8 奎14 娄15 鬼21 星22。
    const JINSHEN_XIU_IDX: [usize; 7] = [0, 1, 8, 14, 15, 21, 22];
    let jin_active = JINSHEN_XIU_IDX.contains(&xiu_idx);
    let jin_shen_qisha = JinShenQiSha {
        active: jin_active,
        xiu: xiu_name.to_string(),
        note: if jin_active {
            format!("{}值日，金神七煞·大凶切不可犯（三吉星亦不能解）", xiu_name)
        } else {
            "当日未遇金神七煞".to_string()
        },
    };

    // 6. 三煞方（P2-7，对齐 JS donggong.js + fengshuiData ZHI_SANHE_JU/SANSHA_BY_JU）：
    //    按节气月建支(month_zhi_idx)三合局，帝旺对冲三山；中支定方位。
    let month_zhi = crate::bazi_exact::DIZHI[month_zhi_idx % 12];
    let (ju, sha_zhi): (&str, [&str; 3]) = match month_zhi {
        "申" | "子" | "辰" => ("水局", ["巳", "午", "未"]), // 水局煞南
        "寅" | "午" | "戌" => ("火局", ["亥", "子", "丑"]), // 火局煞北
        "亥" | "卯" | "未" => ("木局", ["申", "酉", "戌"]), // 木局煞西
        _ => ("金局", ["寅", "卯", "辰"]),                 // 巳酉丑金局煞东
    };
    let direction = match sha_zhi[1] {
        "午" => "南",
        "子" => "北",
        "卯" => "东",
        _ => "西", // 酉
    };
    let san_sha_direction = SanShaDirection {
        direction: direction.to_string(),
        ju: ju.to_string(),
        zhi: sha_zhi.iter().map(|s| s.to_string()).collect(),
        // 三支具名：zeri.js sanshaLabel = [劫煞, 灾煞, 岁煞]，与 SANSHA_BY_JU 顺序一一对应。
        sha_list: [
            SanShaItem { name: "劫煞", zhi: sha_zhi[0].to_string() },
            SanShaItem { name: "灾煞", zhi: sha_zhi[1].to_string() },
            SanShaItem { name: "岁煞", zhi: sha_zhi[2].to_string() },
        ]
        .to_vec(),
        note: format!("{}方（{}·{}）忌修造动土", direction, sha_zhi.join(""), ju),
    };

    TongshuResult {
        ganzhi_date,
        donggong_rating,
        xiu28_star: xiu_name,
        wutu_solar: wutu,
        events,
        summary,
        jin_shen_qisha,
        san_sha_direction,
    }
}
