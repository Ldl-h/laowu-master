// 十大神数同构家族统一参数矩阵驱动器 (Unified Shenshu Family Engine)
// 包含：南极、北极、太玄、皇极、策天、蠢子、分经、精决、神易、五兆
// 数学同构模型：四柱洛书数、纳音数经加权系数求模定轨，直接检索条文

use crate::canping::NAYIN_TABLE;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ShenshuConfig {
    pub name: &'static str,
    pub key: &'static str,
    pub base_offset: u32,
    pub mod_factor: u32,
    pub step_multiplier: u32,
    pub description: &'static str,
}

// 十大衍生神数参数矩阵
pub const SHENSHU_FAMILIES: [ShenshuConfig; 10] = [
    ShenshuConfig {
        name: "南极神数", key: "nanji",
        base_offset: 3100, mod_factor: 12000, step_multiplier: 12,
        description: "南极延寿星君定数，主寿夭康宁、六亲福泽",
    },
    ShenshuConfig {
        name: "北极神数", key: "beiji",
        base_offset: 1200, mod_factor: 12000, step_multiplier: 16,
        description: "紫微北极星垣定数，主功名仕版、官禄荣显",
    },
    ShenshuConfig {
        name: "太玄数", key: "taixuan",
        base_offset: 2400, mod_factor: 10800, step_multiplier: 8,
        description: "扬雄太玄三方九序八十一首推演数",
    },
    ShenshuConfig {
        name: "皇极经世数", key: "wangji",
        base_offset: 1000, mod_factor: 12960, step_multiplier: 30,
        description: "邵雍元会运世大数，推演治乱兴衰与宿命大定",
    },
    ShenshuConfig {
        name: "策天神数", key: "cetian",
        base_offset: 4500, mod_factor: 12000, step_multiplier: 14,
        description: "策天神算，测四时休咎与祸福成败",
    },
    ShenshuConfig {
        name: "蠢子数", key: "chunzi",
        base_offset: 5200, mod_factor: 9600, step_multiplier: 6,
        description: "邵雍蠢子神数秘传，直断生老病死与流年关煞",
    },
    ShenshuConfig {
        name: "分经数", key: "fendjing",
        base_offset: 6100, mod_factor: 12000, step_multiplier: 10,
        description: "分经分度神数，推敲骨肉亲疏与财源厚薄",
    },
    ShenshuConfig {
        name: "精决数", key: "jingjue",
        base_offset: 3800, mod_factor: 12000, step_multiplier: 18,
        description: "先天精决铁券数，指点迷津定乾坤",
    },
    ShenshuConfig {
        name: "神易数", key: "shenyishu",
        base_offset: 2900, mod_factor: 11000, step_multiplier: 9,
        description: "先天神易玄机算，通达事物造化机微",
    },
    ShenshuConfig {
        name: "五兆数", key: "wuzhao",
        base_offset: 1800, mod_factor: 10000, step_multiplier: 15,
        description: "五兆幽微神算，测机变隐匿与吉凶前兆",
    },
];

pub fn get_nayin_num(gz: &str) -> u32 {
    for (i, (name, _)) in NAYIN_TABLE.iter().enumerate() {
        if *name == gz {
            return (i as u32 % 5) + 1;
        }
    }
    1
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GenericShenshuResult {
    pub name: &'static str,
    pub key: &'static str,
    pub verse_id: u32,
    pub verse_text: Option<String>,
    pub calculation_route: String,
    pub description: &'static str,
}

/// 计算任意通用神数家族条文 ID
pub fn calculate_generic_shenshu(
    family_key: &str,
    year_gz: &str,
    month_gz: &str,
    day_gz: &str,
    hour_gz: &str,
) -> GenericShenshuResult {
    let cfg = SHENSHU_FAMILIES.iter().find(|c| c.key == family_key).unwrap_or(&SHENSHU_FAMILIES[0]);

    let y_val = get_nayin_num(year_gz);
    let m_val = get_nayin_num(month_gz);
    let d_val = get_nayin_num(day_gz);
    let h_val = get_nayin_num(hour_gz);

    let sum = y_val * 1000 + m_val * 100 + d_val * 10 + h_val;
    let verse_id = cfg.base_offset + ((sum * cfg.step_multiplier) % cfg.mod_factor);

    // 尝试从 tiaowen.bin 真实检索古代典籍诗词条文正文 (自适应工作目录与根目录)
    let mut verse_text = None;
    if let Some(tiaowen_path) = crate::db::resolve_data_path("tiaowen.bin") {
        if let Ok(db) = crate::db::XuanshiDatabase::open(tiaowen_path) {
            let table_name = match family_key {
                "nanji" => "nanji_verses",
                "beiji" => "beiji_verses",
                "chunzi" => "chunzi_verses",
                "taixuan" => "taixuan_verses",
                "fendjing" => "fendjing_tables",
                "wangji" => "guanwu_yanyi",
                "cetian" => "xinyi_fawei",
                "tieban" => "tieban_verses",
                "shaozi" => "shaozi_verses",
                "shenyishu" | "jingjue" | "wuzhao" => "fendjing_tables",
                _ => "",
            };
            if !table_name.is_empty() {
                if family_key == "taixuan" {
                    // 太玄 81 首，按四进制 (1~3 进位) 或取模 81
                    let shou_idx = (verse_id as usize) % 81;
                    let d1 = (shou_idx / 27) % 3 + 1;
                    let d2 = (shou_idx / 9) % 3 + 1;
                    let d3 = (shou_idx / 3) % 3 + 1;
                    let d4 = shou_idx % 3 + 1;
                    let code = format!("{}{}{}{}", d1, d2, d3, d4);
                    if let Ok(Some(text)) = db.get_verse(table_name, &code) {
                        verse_text = Some(text);
                    } else if let Ok(serde_json::Value::Object(map)) = db.load_table(table_name) {
                        if let Some(val) = map.values().nth(shou_idx) {
                            verse_text = Some(val.to_string());
                        }
                    }
                } else if family_key == "nanji" {
                    if let Ok(serde_json::Value::Array(arr)) = db.load_table(table_name) {
                        let idx = (verse_id as usize) % arr.len().max(1);
                        if let Some(item) = arr.get(idx) {
                            if let Some(obj) = item.as_object() {
                                let v_str = obj.get("verse").and_then(|x| x.as_str()).unwrap_or("");
                                let c_str = obj.get("code").and_then(|x| x.as_str()).unwrap_or("");
                                verse_text = Some(format!("【{}】{}", c_str, v_str));
                            } else {
                                verse_text = Some(item.to_string());
                            }
                        }
                    }
                } else if family_key == "fendjing" {
                    if let Ok(serde_json::Value::Object(map)) = db.load_table(table_name) {
                        // 以年干+日干组合查分经表
                        let y_gan = year_gz.chars().next().map(|c| c.to_string()).unwrap_or_else(|| "甲".to_string());
                        let d_gan = day_gz.chars().next().map(|c| c.to_string()).unwrap_or_else(|| "甲".to_string());
                        let combo = format!("{}{}", y_gan, d_gan);
                        if let Some(val) = map.get(&combo).or_else(|| map.values().next()) {
                            if let Some(obj) = val.as_object() {
                                let mut pan = obj.get("判斷").and_then(|x| x.as_str()).unwrap_or("");
                                if pan.is_empty() {
                                    pan = obj.get("基業").and_then(|x| x.as_str()).unwrap_or("");
                                }
                                let ge = obj.get("命格").and_then(|x| x.as_array()).and_then(|a| a.first()).and_then(|x| x.as_str()).unwrap_or("");
                                let verse_content = if ge.is_empty() {
                                    pan.to_string()
                                } else {
                                    format!("【{}】{}", ge, pan)
                                };
                                verse_text = Some(verse_content);
                            } else {
                                verse_text = Some(val.to_string());
                            }
                        }
                    }
                } else if family_key == "wangji" {
                    if let Ok(serde_json::Value::Object(map)) = db.load_table(table_name) {
                        let quotes = map.get("key_quotes").and_then(|x| x.as_array());
                        if let Some(q_arr) = quotes {
                            let idx = (verse_id as usize) % q_arr.len().max(1);
                            if let Some(q) = q_arr.get(idx) {
                                verse_text = Some(q.as_str().unwrap_or(&q.to_string()).to_string());
                            }
                        } else {
                            verse_text = map.get("title").and_then(|x| x.as_str()).map(|s| s.to_string());
                        }
                    }
                } else if family_key == "cetian" {
                    if let Ok(serde_json::Value::Object(map)) = db.load_table(table_name) {
                        let quotes = map.get("volumes").and_then(|x| x.as_array());
                        if let Some(q_arr) = quotes {
                            let idx = (verse_id as usize) % q_arr.len().max(1);
                            if let Some(q) = q_arr.get(idx) {
                                verse_text = Some(q.as_str().unwrap_or(&q.to_string()).to_string());
                            }
                        } else {
                            verse_text = map.get("title").and_then(|x| x.as_str()).map(|s| s.to_string());
                        }
                    }
                } else if family_key == "chunzi" {
                    if let Ok(serde_json::Value::Array(arr)) = db.load_table(table_name) {
                        // 剔除首行表头
                        let total_rows = if arr.len() > 1 { arr.len() - 1 } else { arr.len() };
                        let idx = 1 + ((verse_id as usize) % total_rows.max(1));
                        if let Some(item) = arr.get(idx) {
                            if let Some(row) = item.as_array() {
                                // 第 6 列是正文诗词/断辞: ["category", "star", "degree", "branch", "code", "aux_num", "verse", ...]
                                if let Some(v_str) = row.get(6).and_then(|v| v.as_str()) {
                                    verse_text = Some(v_str.to_string());
                                } else if let Some(last_str) = row.last().and_then(|v| v.as_str()) {
                                    verse_text = Some(last_str.to_string());
                                } else {
                                    verse_text = Some(item.to_string());
                                }
                            } else {
                                verse_text = Some(item.as_str().unwrap_or(&item.to_string()).to_string());
                            }
                        }
                    }
                } else if let Ok(Some(text)) = db.get_verse(table_name, &verse_id.to_string()) {
                    verse_text = Some(text);
                } else if let Ok(serde_json::Value::Array(arr)) = db.load_table(table_name) {
                    let idx = (verse_id as usize) % arr.len().max(1);
                    if let Some(item) = arr.get(idx) {
                        if let Some(row) = item.as_array() {
                            // 若是 CSV 行数组（如 chunzi_verses: ["軫","氣","13","酉","...", "真实断语"]），取最后一列或有效断语
                            if let Some(last_str) = row.last().and_then(|v| v.as_str()) {
                                verse_text = Some(last_str.to_string());
                            } else {
                                verse_text = Some(item.to_string());
                            }
                        } else {
                            verse_text = Some(item.as_str().unwrap_or(&item.to_string()).to_string());
                        }
                    }
                } else if let Ok(serde_json::Value::Object(map)) = db.load_table(table_name) {
                    let idx = (verse_id as usize) % map.len().max(1);
                    if let Some(val) = map.values().nth(idx) {
                        verse_text = Some(val.to_string());
                    }
                }
            }
        }
    }

    GenericShenshuResult {
        name: cfg.name,
        key: cfg.key,
        verse_id,
        verse_text,
        calculation_route: format!(
            "四柱纳音数 ({}, {}, {}, {}) -> 基准偏移 {} + 积数步进 {} = 轨数 {}",
            y_val, m_val, d_val, h_val, cfg.base_offset, cfg.step_multiplier, verse_id
        ),
        description: cfg.description,
    }
}
