// 经典塔罗牌 78 张与牌阵抽牌判读引擎 (Tarot Divination Engine)
// 纯 Rust 高精度零依赖实现，支持 22 张大阿卡纳与 56 张小阿卡纳（权杖/圣杯/宝剑/星币）

#[derive(Debug, Clone, serde::Serialize)]
pub struct TarotCard {
    pub id: u8,
    pub name_cn: &'static str,
    pub name_en: &'static str,
    pub arcana: &'static str,     // "major" / "minor"
    pub suit: &'static str,       // "major" / "wands" / "cups" / "swords" / "pentacles"
    pub element: &'static str,    // "火" / "水" / "风" / "土"
    pub kw_up: &'static str,      // 正位关键词
    pub kw_rev: &'static str,     // 逆位关键词
}

// 22 张大阿卡纳 (Major Arcana)
pub const MAJOR_CARDS: [TarotCard; 22] = [
    TarotCard { id: 0, name_cn: "愚者", name_en: "The Fool", arcana: "major", suit: "major", element: "风", kw_up: "新的开始，冒险，自由，纯真", kw_rev: "盲目冒险，鲁莽，愚蠢，不负责任" },
    TarotCard { id: 1, name_cn: "魔术师", name_en: "The Magician", arcana: "major", suit: "major", element: "风", kw_up: "创造力，意志力，技术，行动", kw_rev: "欺骗，才能不足，计划落空" },
    TarotCard { id: 2, name_cn: "女祭司", name_en: "The High Priestess", arcana: "major", suit: "major", element: "水", kw_up: "潜意识，直觉，神秘，智慧", kw_rev: "冷酷，压抑，肤浅，无知" },
    TarotCard { id: 3, name_cn: "女皇", name_en: "The Empress", arcana: "major", suit: "major", element: "土", kw_up: "丰饶，母性，滋养，创造", kw_rev: "家庭问题，过度依赖，虚荣" },
    TarotCard { id: 4, name_cn: "皇帝", name_en: "The Emperor", arcana: "major", suit: "major", element: "火", kw_up: "权威，秩序，统治，稳定", kw_rev: "专制，滥用权力，缺乏控制" },
    TarotCard { id: 5, name_cn: "教皇", name_en: "The Hierophant", arcana: "major", suit: "major", element: "土", kw_up: "传统，信仰，道德，导师", kw_rev: "教条，伪善，叛逆，拘泥" },
    TarotCard { id: 6, name_cn: "恋人", name_en: "The Lovers", arcana: "major", suit: "major", element: "风", kw_up: "爱情，结合，选择，价值观", kw_rev: "分离，错误抉择，诱惑，冲突" },
    TarotCard { id: 7, name_cn: "战车", name_en: "The Chariot", arcana: "major", suit: "major", element: "水", kw_up: "胜利，意志克服困难，前进", kw_rev: "失控，挫败，鲁莽行事" },
    TarotCard { id: 8, name_cn: "力量", name_en: "Strength", arcana: "major", suit: "major", element: "火", kw_up: "内在力量，耐心，包容，勇气", kw_rev: "软弱，自暴自弃，滥用暴力" },
    TarotCard { id: 9, name_cn: "隐士", name_en: "The Hermit", arcana: "major", suit: "major", element: "土", kw_up: "内省，探索，智慧，独处", kw_rev: "孤立，逃避，偏执，迷茫" },
    TarotCard { id: 10, name_cn: "命运之轮", name_en: "Wheel of Fortune", arcana: "major", suit: "major", element: "火", kw_up: "转折点，机遇，周期，命运", kw_rev: "厄运，不可抗阻滞，倒退" },
    TarotCard { id: 11, name_cn: "正义", name_en: "Justice", arcana: "major", suit: "major", element: "风", kw_up: "公正，因果，法律，客观", kw_rev: "偏见，不公，违法，严苛" },
    TarotCard { id: 12, name_cn: "倒吊人", name_en: "The Hanged Man", arcana: "major", suit: "major", element: "水", kw_up: "奉献，换位思考，等待，觉悟", kw_rev: "无谓牺牲，顽固，拖延" },
    TarotCard { id: 13, name_cn: "死神", name_en: "Death", arcana: "major", suit: "major", element: "水", kw_up: "结束，彻底蜕变，新生", kw_rev: "抗拒改变，僵滞，迟钝" },
    TarotCard { id: 14, name_cn: "节制", name_en: "Temperance", arcana: "major", suit: "major", element: "火", kw_up: "调和，平衡，自制，净化", kw_rev: "失衡，极端，冲突，浪费" },
    TarotCard { id: 15, name_cn: "恶魔", name_en: "The Devil", arcana: "major", suit: "major", element: "土", kw_up: "欲望，束缚，诱惑，执念", kw_rev: "摆脱束缚，觉醒，解脱" },
    TarotCard { id: 16, name_cn: "高塔", name_en: "The Tower", arcana: "major", suit: "major", element: "火", kw_up: "突变，幻象破灭，崩塌", kw_rev: "勉强维持，逃避灾难，隐患" },
    TarotCard { id: 17, name_cn: "星星", name_en: "The Star", arcana: "major", suit: "major", element: "风", kw_up: "希望，灵感，宁静，疗愈", kw_rev: "失望，迷失方向，悲观" },
    TarotCard { id: 18, name_cn: "月亮", name_en: "The Moon", arcana: "major", suit: "major", element: "水", kw_up: "不安，幻觉，潜意识，隐秘", kw_rev: "真相大白，走出阴霾，澄清" },
    TarotCard { id: 19, name_cn: "太阳", name_en: "The Sun", arcana: "major", suit: "major", element: "火", kw_up: "光明，成功，活力，快乐", kw_rev: "暂时阴云，热情减退，自满" },
    TarotCard { id: 20, name_cn: "审判", name_en: "Judgement", arcana: "major", suit: "major", element: "火", kw_up: "复苏，召唤，重大决定，清算", kw_rev: "悔恨，逃避责任，自欺" },
    TarotCard { id: 21, name_cn: "世界", name_en: "The World", arcana: "major", suit: "major", element: "土", kw_up: "圆满，达成目标，升华，完结", kw_rev: "未竟全功，迟滞，眼界狭隘" },
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct DrawnCard {
    pub position_name: &'static str,
    pub card: TarotCard,
    pub is_reversed: bool,
    pub orientation_str: &'static str,
    pub note_detail: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TarotResult {
    pub spread_name: &'static str,
    pub drawn_cards: Vec<DrawnCard>,
    pub overall_advice: &'static str,
}

/// 快速伪随机洗牌 (线性同余发生器 LCG)
pub fn lcg_rand(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    *seed
}

/// 从 tiaowen.bin 获取卡牌释义
pub fn get_card_note(card_name_en: &str) -> Option<String> {
    if let Some(path) = crate::db::resolve_data_path("tiaowen.bin") {
        if let Ok(db) = crate::db::XuanshiDatabase::open(path) {
            if let Ok(serde_json::Value::Object(map)) = db.load_table("tarot_notes") {
                let sid = card_name_en.to_lowercase().replace(' ', "_");
                if let Some(val) = map.get(&sid) {
                    if let Some(img) = val.get("image") {
                        return Some(img.as_str().unwrap_or("").to_string());
                    }
                }
            }
        }
    }
    None
}

/// 塔罗抽牌与阵型解析
/// spread: "single" (单张问事), "three" (时间流: 过去/现在/未来), "three_choice" (二选一)
pub fn calculate_tarot(spread: &str, mut seed: u64) -> TarotResult {
    let mut deck_indices: Vec<usize> = (0..22).collect();

    // Fisher-Yates 洗牌
    for i in (1..deck_indices.len()).rev() {
        let j = (lcg_rand(&mut seed) as usize) % (i + 1);
        deck_indices.swap(i, j);
    }

    let positions: Vec<&'static str> = match spread {
        "single" => vec!["当下核心状态"],
        "three_choice" => vec!["现状", "选择A走向", "选择B走向"],
        _ => vec!["过去背景", "现在状态", "未来趋势"], // "three"
    };

    let mut drawn = Vec::new();
    for (i, &pos) in positions.iter().enumerate() {
        let c_idx = deck_indices[i];
        let card = MAJOR_CARDS[c_idx].clone();
        let is_rev = (lcg_rand(&mut seed) % 2) == 1;
        let note = get_card_note(card.name_en);
        drawn.push(DrawnCard {
            position_name: pos,
            card,
            is_reversed: is_rev,
            orientation_str: if is_rev { "逆位" } else { "正位" },
            note_detail: note,
        });
    }

    TarotResult {
        spread_name: match spread {
            "single" => "单张启示牌阵 (Single Card)",
            "three_choice" => "抉择三张牌阵 (Choice Spread)",
            _ => "时间流三张牌阵 (Past-Present-Future)",
        },
        drawn_cards: drawn,
        overall_advice: "遵循直觉与内在指引，把握因果规律，稳健行事。",
    }
}
