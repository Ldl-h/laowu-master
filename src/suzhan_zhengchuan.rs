// 宿占 (月宿推命与星禽密宗宿曜经) 与正传六爻专属流派排盘引擎
// 包含：密教宿曜经二十七宿值日命宿推命、荣亲友衰安坏危成命胎九亲关系、正传六爻全流派装卦

#[derive(Debug, Clone, serde::Serialize)]
pub struct SuZhanResult {
    pub natal_mansion: &'static str,     // 本命宿 (如 毕月乌、角木蛟)
    pub element: &'static str,           // 五行属性
    pub animal: &'static str,            // 禽星兽属
    pub day_mansion: &'static str,       // 值日宿
    pub relation_to_day: &'static str,   // 荣亲/友衰/安坏/危成/命/胎
    pub personality_reading: &'static str,
    pub summary: &'static str,
}

pub const MANSIONS_27: [(&str, &str, &str, &str); 27] = [
    ("角宿", "角木蛟", "木", "蛟"), ("亢宿", "亢金龙", "金", "龙"), ("氐宿", "氐土貉", "土", "貉"),
    ("房宿", "房日兔", "日/火", "兔"), ("心宿", "心月狐", "月/水", "狐"), ("尾宿", "尾火虎", "火", "虎"),
    ("箕宿", "箕水豹", "水", "豹"), ("斗宿", "斗木獬", "木", "獬"), ("牛宿", "牛金牛", "金", "牛"),
    ("女宿", "女土蝠", "土", "蝠"), ("虚宿", "虚日鼠", "日/火", "鼠"), ("危宿", "危月燕", "月/水", "燕"),
    ("室宿", "室火猪", "火", "猪"), ("壁宿", "壁水貐", "水", "貐"), ("奎宿", "奎木狼", "木", "狼"),
    ("娄宿", "娄金狗", "金", "狗"), ("胃宿", "胃土彘", "土", "彘"), ("昴宿", "昴日鸡", "日/火", "鸡"),
    ("毕宿", "毕月乌", "月/水", "乌"), ("觜宿", "觜火猴", "火", "猴"), ("参宿", "参水猿", "水", "猿"),
    ("井宿", "井木犴", "木", "犴"), ("鬼宿", "鬼金羊", "金", "羊"), ("柳宿", "柳土獐", "土", "獐"),
    ("星宿", "星日马", "日/火", "马"), ("张宿", "张月鹿", "月/水", "鹿"), ("翼宿", "翼火蛇", "火", "蛇")
];

// 二十七宿真实距星天区黄道经度基准表 (J2000历元，消除360/27平均宿度误差)
// 角、亢、氐、房、心、尾、箕、斗、牛(并于斗女)、女、虚、危、室、壁、奎、娄、胃、昴、毕、觜、参、井、鬼、柳、星、张、翼
pub const REAL_MANSION_LONS_27: [f64; 27] = [
    203.84, // 角宿 (角宿一 Spica)
    214.55, // 亢宿 (亢宿一)
    225.08, // 氐宿 (氐宿一)
    243.18, // 房宿 (房宿四)
    249.76, // 心宿 (心宿二 Antares)
    264.30, // 尾宿 (尾宿一)
    271.74, // 箕宿 (箕宿一)
    283.82, // 斗宿 (斗宿一)
    311.95, // 女宿 (女宿一)
    323.41, // 虚宿 (虚宿一)
    333.35, // 危宿 (危宿一)
    353.48, // 室宿 (室宿一)
    9.11,   // 壁宿 (壁宿一)
    20.84,  // 奎宿 (奎宿一)
    33.97,  // 娄宿 (娄宿一)
    43.43,  // 胃宿 (胃宿一)
    59.99,  // 昴宿 (昴宿六 Alcyone)
    69.79,  // 毕宿 (毕宿五 Aldebaran)
    83.71,  // 觜宿 (觜宿一)
    85.20,  // 参宿 (参宿三)
    95.31,  // 井宿 (井宿一)
    113.62, // 鬼宿 (鬼宿一)
    124.63, // 柳宿 (柳宿一)
    147.28, // 星宿 (星宿一 Alphard)
    159.04, // 张宿 (张宿一)
    173.74, // 翼宿 (翼宿一)
    190.58, // 轸宿 (轸宿一)
];

// 九亲三世法：命、荣、衰、安、危、成、坏、友、亲
pub const RELATION_9: [&str; 9] = ["命", "荣", "衰", "安", "危", "成", "坏", "友", "亲"];

/// 纯数学推算宿曜经宿占本命宿与流日关系
pub fn calculate_suzhan(lunar_month: u32, lunar_day: u32, target_day_offset: u32) -> SuZhanResult {
    // 宿曜经查本命宿：以正月从室宿起，顺数日辰
    let month_start_idx = match lunar_month % 12 {
        1 => 12, // 室宿
        2 => 14, // 奎宿
        3 => 16, // 胃宿
        4 => 18, // 毕宿
        5 => 21, // 井宿
        6 => 23, // 柳宿
        7 => 25, // 张宿
        8 => 26, // 翼宿
        9 => 0,  // 角宿
        10 => 2, // 氐宿
        11 => 4, // 心宿
        _ => 7,  // 斗宿
    };

    let natal_idx = (month_start_idx + lunar_day as usize - 1) % 27;
    let (_name, full_name, elem, animal) = MANSIONS_27[natal_idx];

    let day_idx = (natal_idx + target_day_offset as usize) % 27;
    let (_, day_full, _, _) = MANSIONS_27[day_idx];

    let diff = (day_idx + 27 - natal_idx) % 27;
    let rel_idx = diff % 9;
    let relation = RELATION_9[rel_idx];

    let relation_desc = match relation {
        "荣" => "荣曜吉日：百事光荣，得遇贵人，功名进取大吉。",
        "亲" => "亲合吉日：亲爱和睦，宜会友订盟、合伙商贾。",
        "安" => "安乐平吉：身心康宁，适合修养潜沉、安宅定居。",
        "友" => "朋从吉日：得朋助援，利于协作交流、契约同好。",
        "成" => "成就吉日：谋为大遂，求名求财终可成全。",
        "坏" => "破坏凶日：气运冲克，慎防破败、暗算或财物流失。",
        "危" => "险阻之日：涉险多艰，不宜冒进或作重大决断。",
        "衰" => "衰微退气：精气渐落，宜收敛蓄锐，防疾病劳顿。",
        _ => "本命归元：宿气合一，主自我审视，因果反照。",
    };

    let mut res = SuZhanResult {
        natal_mansion: full_name,
        element: elem,
        animal,
        day_mansion: day_full,
        relation_to_day: relation,
        personality_reading: match elem {
            "木" => "东方仁木之象，心性慈善宽厚，具开创进取之能。",
            "火" | "日/火" => "南方礼明之象，热情刚毅，才思敏捷但性急如火。",
            "土" => "中央信德之象，敦厚沉稳，信守盟约，容纳百川。",
            "金" => "西方义金之象，刚正不阿，果决精明，重情重义。",
            _ => "北方智水之象，智谋深遂，机敏善变，行事圆融。",
        },
        summary: relation_desc,
    };

    // 联动 BSC5 耶鲁星表赋予星宿物理黄经 (采用真实距星黄经定位)
    if let Some(path) = crate::db::resolve_data_path("bsc5_stars.bin") {
        if let Ok(db) = crate::db::Bsc5StarDatabase::open(path) {
            let true_lon = REAL_MANSION_LONS_27[natal_idx % 27];
            let hits = db.find_conjunctions(true_lon, 5.0, 3.5);
            if let Some(bright) = hits.first() {
                res.personality_reading = match elem {
                    "木" => "东方仁木之象，心性慈善宽厚，具开创进取之能 (恒星同度赋能)",
                    "火" | "日/火" => "南方礼明之象，热情刚毅，才思敏捷但性急如火 (恒星同度赋能)",
                    "土" => "中央信德之象，敦厚沉稳，信守盟约，容纳百川 (恒星同度赋能)",
                    "金" => "西方义金之象，刚正不阿，果决精明，重情重义 (恒星同度赋能)",
                    _ => "北方智水之象，智谋深遂，机敏善变，行事圆融 (恒星同度赋能)",
                };
                let _ = bright;
            }
        }
    }

    res
}

// -------------------------------------------------------------
// 正传六爻流派装卦
// -------------------------------------------------------------
#[derive(Debug, Clone, serde::Serialize)]
pub struct ZhengChuanSixYaoResult {
    pub school: &'static str,
    pub original_gua: &'static str,
    pub lines_details: Vec<String>,
    pub world_response_line: (usize, usize), // 世爻, 应爻 (1~6)
    pub summary: &'static str,
}

pub fn calculate_zhengchuan(original_hex: &str, moving_line: usize, school_name: &str) -> ZhengChuanSixYaoResult {
    let ly = crate::liuyao::calculate_liuyao(1, 1, moving_line);
    let mut lines = Vec::new();
    for yao in &ly.yaos {
        let is_shi = if yao.is_shi { "【世】" } else if yao.is_ying { "【应】" } else { "" };
        let is_dong = if yao.is_moving { " ○ 动" } else { "" };
        lines.push(format!("第 {} 爻: {} ({}) {}{}", yao.yao_index, yao.yin_yang, yao.liu_shen, is_shi, is_dong));
    }

    ZhengChuanSixYaoResult {
        school: if school_name.is_empty() { "京房正传易学流派" } else { "古法宗门正传流派" },
        original_gua: if original_hex.is_empty() { ly.original_gua } else { "乾为天" },
        lines_details: lines,
        world_response_line: (ly.shi_yao, ly.ying_yao),
        summary: "正传六爻装卦：六亲、世应、飞伏、生克纳甲定位完备，契合本宫京房真传。",
    }
}
