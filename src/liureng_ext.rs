use crate::liureng::{ZHI, TIAN_JIANG};

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiuRengRunyearItem {
    pub age: u32,
    pub xing_nian_zhi: &'static str,
    pub god_name: &'static str,
    pub luck_verdict: &'static str,
}

/// 大六壬行年推运与十二贵人神煞扩展 (Run Year & Shen Jiang Distribution)
/// 男一岁起丙寅顺行，女一岁起壬申逆行（传统大六壬行年起法，与JS LiuRengMain.js 一致）
/// 行年地支由 age+gender 固定决定，不从出生年支起。
#[allow(unused_variables)] // birth_zhi_idx 保留参数签名供未来本命展示扩展，当前行年计算不使用
pub fn calculate_liureng_runyear(
    birth_zhi_idx: usize,
    current_age: u32,
    is_male: bool,
) -> Vec<LiuRengRunyearItem> {
    let mut runyears = Vec::new();

    // 固定起点：男起寅(地支索引2)，女起申(地支索引8)
    let start_base = if is_male { 2 } else { 8 };

    for a in 1..=current_age {
        let xing_idx = if is_male {
            (start_base + (a as i32 - 1)).rem_euclid(12) as usize
        } else {
            (start_base - (a as i32 - 1)).rem_euclid(12) as usize
        };
        let xing_zhi = ZHI[xing_idx];

        // 配十二天将
        let god = TIAN_JIANG[xing_idx % 12];

        // 行年地支五行格局断语
        let verdict = match xing_zhi {
            "辰" | "戌" | "丑" | "未" => "四墓行年：主关煞阻隔，宜守静安详，防跌撞晦气。",
            "子" | "午" | "卯" | "酉" => "四正行年：桃花气旺，交游广阔，名声显耀。",
            _ => "四马行年：奔波走动频繁，驿马加临，动中求达。",
        };

        if a == current_age || a > current_age.saturating_sub(3) {
            runyears.push(LiuRengRunyearItem {
                age: a,
                xing_nian_zhi: xing_zhi,
                god_name: god,
                luck_verdict: verdict,
            });
        }
    }

    runyears
}
