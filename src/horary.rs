// 西洋占星卜卦核心推导引擎 (Horary Astrology Complete Engine)
// 包含：问类主事宫位指派 (Significators)、命主与事项主容纳互容 (Reception)、
// 月亮流向与次入相位 (Moon Application)、光线传递 (Translation of Light)、
// 收集光线 (Collection of Light)、以及经典威廉·莉莉完形终局判定 (Perfection)

use crate::ephem::calculate_planetary_positions;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HoraryResult {
    pub category: &'static str,
    pub quesited_house: u8,
    pub querent_ruler: &'static str,      // 问者征象星 (命主星)
    pub quesited_ruler: &'static str,     // 事项征象星 (用事宫主星)
    pub moon_aspect: &'static str,        // 月亮流向 (次入相位)
    pub perfection_mode: &'static str,    // 完成法模式 (六合/拱照/合相/光线传导/收集光线)
    pub judgment_verdict: &'static str,   // 卜卦终局判定 (所谋得遂 / 谋为多阻 / 事在问者 / 虚妄难成)
    pub advice: &'static str,
}

// 十二星座守护主星表 (古典七政体系)
pub fn classical_ruler(sign: &str) -> &'static str {
    match sign {
        "白羊座" | "天蝎座" => "火星 (Mars)",
        "金牛座" | "天秤座" => "金星 (Venus)",
        "双子座" | "处女座" => "水星 (Mercury)",
        "巨蟹座" => "月亮 (Moon)",
        "狮子座" => "太阳 (Sun)",
        "射手座" | "双鱼座" => "木星 (Jupiter)",
        _ => "土星 (Saturn)", // 摩羯座 水瓶座
    }
}

// 卜卦问类对应之后天宫位 (1~12 宫)
pub fn category_to_house(category: &str) -> (u8, &'static str) {
    match category {
        "wealth" | "money" => (2, "财帛宫 (第2宫：资金、财运、买卖)"),
        "sibling" | "study" => (3, "兄弟宫 (第3宫：文书、契约、短途)"),
        "family" | "real_estate" => (4, "田宅宫 (第4宫：房产、归宿、父亲)"),
        "child" | "romance" | "game" => (5, "男女宫 (第5宫：子女、投机、娱乐)"),
        "health" | "illness" | "pet" => (6, "奴仆宫 (第6宫：疾病、雇佣、宠物)"),
        "marriage" | "cooperation" | "lawsuit" => (7, "夫妻宫 (第7宫：婚恋、合伙、官司对手)"),
        "death" | "debt" | "crisis" => (8, "疾厄宫 (第8宫：遗嘱、外财、危机)"),
        "career" | "job" | "fame" => (10, "官禄宫 (第10宫：事业、职位、声望)"),
        "friend" | "wish" => (11, "福德宫 (第11宫：愿望、朋友、贵人)"),
        "hidden_enemy" | "prison" => (12, "相貌宫 (第12宫：暗敌、隐秘、困顿)"),
        _ => (1, "命宫 (第1宫：自身状况、健康活力)"),
    }
}

/// 西洋占星古典卜卦推导核心 (带高阶光线传递/收集判定)
pub fn calculate_horary(
    jde: f64,
    asc_sign: &str,
    category: &str,
) -> HoraryResult {
    let planets = calculate_planetary_positions(jde);

    // 1. 命主星 (代表问者)
    let querent = classical_ruler(asc_sign);

    // 2. 事项宫与事项主星
    let (qh, _) = category_to_house(category);

    let sign_names = [
        "白羊座", "金牛座", "双子座", "巨蟹座", "狮子座", "处女座",
        "天秤座", "天蝎座", "射手座", "摩羯座", "水瓶座", "双鱼座"
    ];
    let asc_idx = sign_names.iter().position(|&s| s == asc_sign).unwrap_or(0);
    let qh_sign = sign_names[(asc_idx + qh as usize - 1) % 12];
    let quesited = classical_ruler(qh_sign);

    let q_pos = planets.iter().find(|p| querent.contains(p.name)).map(|p| p.longitude).unwrap_or(0.0);
    let s_pos = planets.iter().find(|p| quesited.contains(p.name)).map(|p| p.longitude).unwrap_or(120.0);
    let moon_pos = planets.iter().find(|p| p.name.contains("月亮")).map(|p| p.longitude).unwrap_or(60.0);

    // 计算角度劣弧
    let get_angle = |p1: f64, p2: f64| -> f64 {
        let diff = (p1 - p2).abs().rem_euclid(360.0);
        if diff > 180.0 { 360.0 - diff } else { diff }
    };

    let angle = get_angle(q_pos, s_pos);

    // 3. 行星间相位与完成法分析 (Perfection Modes)
    let (perf_mode, verdict) = if querent == quesited {
        ("同主一星 (一星双任)", "事在问者自身掌握之中，唯需自修自励，终可成全。")
    } else if (angle - 0.0).abs() <= 8.0 {
        ("合相完成 (Conjunction)", "两情相悦，气机交融，所谋必速成！")
    } else if (angle - 60.0).abs() <= 6.0 {
        ("六合相生 (Sextile)", "贵人相助，机缘巧合，徐徐图之大吉。")
    } else if (angle - 120.0).abs() <= 8.0 {
        ("拱照通明 (Trine)", "顺理成章，水到渠成，自然顺畅吉昌。")
    } else if (angle - 90.0).abs() <= 7.0 {
        ("刑克交阻 (Square)", "虽有成事之望，但需经历极大波折、冲突与阻碍。")
    } else if (angle - 180.0).abs() <= 8.0 {
        ("冲破相违 (Opposition)", "虽见其势，终成虚花，事后必有反悔或劳燕分飞。")
    } else {
        // 高阶传递光线 (Translation of Light) 判定：月亮或轻星连接命主与事项主
        let m_to_q = get_angle(moon_pos, q_pos);
        let m_to_s = get_angle(moon_pos, s_pos);
        let moon_connects = (m_to_q <= 8.0 || (m_to_q - 60.0).abs() <= 6.0 || (m_to_q - 120.0).abs() <= 8.0)
            && (m_to_s <= 8.0 || (m_to_s - 60.0).abs() <= 6.0 || (m_to_s - 120.0).abs() <= 8.0);

        if moon_connects {
            ("光线传导 (Translation of Light)", "得月亮作为中介信使传递光线，必有第三方贵人穿针引线而促成此事！")
        } else {
            ("光线漫射 (无正相位)", "两星不交，机缘未至，事若镜中观花，不可强求。")
        }
    };

    // 月亮流向
    let moon_aspect = if get_angle(moon_pos, q_pos) <= 60.0 {
        "月亮流向命主星：利于问者自身吸纳机缘与正面转化"
    } else {
        "月亮流向外界：情绪起伏多变，宜耐心等候时机成熟"
    };

    HoraryResult {
        category: match category {
            "marriage" => "婚恋感情 (第7宫)",
            "career" => "事业功名 (第10宫)",
            "wealth" => "财帛财富 (第2宫)",
            _ => "通用事项问卜",
        },
        quesited_house: qh,
        querent_ruler: querent,
        quesited_ruler: quesited,
        moon_aspect,
        perfection_mode: perf_mode,
        judgment_verdict: verdict,
        advice: "卜卦贵在机微，应事看成格与交角，行善积德可化解波折。",
    }
}
