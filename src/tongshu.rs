// 传统通书择日推演

#[derive(Debug, Clone, serde::Serialize)]
pub struct TongshuEvent {
    pub category: &'static str,
    pub name: &'static str,
    pub suitable: bool,
    pub reason: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TongshuResult {
    pub ganzhi_date: String,
    pub donggong_rating: &'static str, // 上吉 / 次吉 / 平 / 凶 / 大凶
    pub xiu28_star: &'static str,      // 二十八宿值日星 (角亢氏房...)
    pub wutu_solar: &'static str,      // 乌兔太阳到山到向
    pub events: Vec<TongshuEvent>,
    pub summary: &'static str,
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
    let xiu_idx = (day_since_epoch.rem_euclid(28)) as usize;
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

    TongshuResult {
        ganzhi_date,
        donggong_rating,
        xiu28_star: xiu_name,
        wutu_solar: wutu,
        events,
        summary,
    }
}
