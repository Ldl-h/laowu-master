// 通摄法（汉代焦氏易林演生·阴阳互卦贯通）推导引擎 (Tong She Fa Engine)
// 纯 Rust 高精度零依赖实现，基于太阴、太阳、少阳、少阴四象起八卦两极六爻

use crate::liuyao::GUA64;

#[derive(Debug, Clone, serde::Serialize)]
pub struct TongSheFaResult {
    pub left_hex_name: &'static str,     // 左卦（本体）
    pub right_hex_name: &'static str,    // 右卦（功用）
    pub left_palace_elem: &'static str,  // 左卦京房本宫五行
    pub right_palace_elem: &'static str, // 右卦京房本宫五行
    pub main_relation: &'static str,     // 主生克关系 (生/克/被生/被克/比和)
    pub mutual_left: &'static str,       // 左卦互卦
    pub mutual_right: &'static str,      // 右卦互卦
    pub summary: &'static str,
}

pub fn bagua_to_lines(b: &str) -> [u8; 3] {
    match b {
        "乾" => [1, 1, 1],
        "兑" => [1, 1, 0],
        "离" => [1, 0, 1],
        "震" => [1, 0, 0],
        "巽" => [0, 1, 1],
        "坎" => [0, 1, 0],
        "艮" => [0, 0, 1],
        _ => [0, 0, 0], // 坤
    }
}

pub fn bagua_to_num(b: &str) -> usize {
    match b {
        "乾" => 0, "兑" => 1, "离" => 2, "震" => 3,
        "巽" => 4, "坎" => 5, "艮" => 6, _ => 7,
    }
}

pub fn hex_name(up: &str, lo: &str) -> &'static str {
    let u = bagua_to_num(up);
    let l = bagua_to_num(lo);
    GUA64[u][l]
}

// 京房本宫五行表
pub fn jingfang_elem(up: &str, lo: &str) -> &'static str {
    match (up, lo) {
        ("乾", "乾") | ("坤", "坤") | ("乾", "坤") | ("坤", "乾") => "金",
        ("坎", "坎") | ("离", "离") | ("坎", "离") | ("离", "坎") => "水",
        ("震", "震") | ("巽", "巽") | ("震", "巽") | ("巽", "震") => "木",
        ("艮", "艮") | ("兑", "兑") | ("艮", "兑") | ("兑", "艮") => "土",
        _ => match up {
            "乾" | "兑" => "金",
            "震" | "巽" => "木",
            "坎" => "水",
            "离" => "火",
            _ => "土",
        },
    }
}

pub fn elem_relation(e1: &str, e2: &str) -> &'static str {
    if e1 == e2 {
        return "比和同旺";
    }
    match (e1, e2) {
        ("木", "火") | ("火", "土") | ("土", "金") | ("金", "水") | ("水", "木") => "本体生客体（泄气）",
        ("火", "木") | ("土", "火") | ("金", "土") | ("水", "金") | ("木", "水") => "客体生本体（得生）",
        ("木", "土") | ("土", "水") | ("水", "火") | ("火", "金") | ("金", "木") => "本体克客体（为财）",
        _ => "客体克本体（受制）",
    }
}

/// 通摄法推演：
/// 输入四象卦象 (taiyin:太阴, taiyang:太阳, shaoyang:少阳, shaoyin:少阴)
pub fn calculate_tongshefa(taiyin: &str, taiyang: &str, shaoyang: &str, shaoyin: &str) -> TongSheFaResult {
    let left_name = hex_name(taiyin, shaoyang);
    let right_name = hex_name(taiyang, shaoyin);

    let left_elem = jingfang_elem(taiyin, shaoyang);
    let right_elem = jingfang_elem(taiyang, shaoyin);

    let rel = elem_relation(left_elem, right_elem);

    TongSheFaResult {
        left_hex_name: left_name,
        right_hex_name: right_name,
        left_palace_elem: left_elem,
        right_palace_elem: right_elem,
        main_relation: rel,
        mutual_left: "体卦互见交感",
        mutual_right: "用卦生发相生",
        summary: "通摄贯通，左右并观，体用互参，刚柔自显。",
    }
}
