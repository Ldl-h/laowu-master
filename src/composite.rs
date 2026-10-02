// 复合玄学体系与多维择日总线 (Composite Metaphysics & Multi-dimensional Zeri Pipeline)
// 包含：三式合一 (San Shi United: 奇门 + 太乙 + 六壬同台)、复合择日扫描适配器

use crate::bazi_exact::{calculate_exact_bazi, ExactBaZi, TIANGAN, DIZHI};
use crate::qimen::{calculate_qimen_with_date, QimenResult};
use crate::taiyi::{calculate_taiyi, TaiyiResult};
use crate::liureng::{calculate_liureng, LiuRengResult};
use crate::zeri::{scan_zeri, ZeriCondition, ZeriScanResult};

#[derive(Debug, Clone, serde::Serialize)]
pub struct SanShiUnitedResult {
    pub bazi: ExactBaZi,
    pub qimen: QimenResult,
    pub taiyi: TaiyiResult,
    pub liureng: LiuRengResult,
    pub summary: &'static str,
}

/// 三式合一计算 (奇门遁甲 + 太乙神数 + 大六壬同盘合演)
/// 严格根据精确八字四柱联动奇门节气用局与大六壬月将神煞
pub fn calculate_sanshi_united(year: i32, month: u32, day: u32, hour: u32) -> SanShiUnitedResult {
    let bz = calculate_exact_bazi(year, month, day, hour, 0, 0);

    // 解析日干支与时干支在 60 甲子中的真实索引
    let d_gan_char = bz.day_pillar.chars().next().unwrap_or('甲');
    let d_zhi_str: String = bz.day_pillar.chars().skip(1).collect();
    let day_gan_idx = TIANGAN.iter().position(|&g| g.starts_with(d_gan_char)).unwrap_or(0);
    let day_zhi_idx = DIZHI.iter().position(|&z| z == d_zhi_str.as_str()).unwrap_or(0);

    let h_gan_char = bz.hour_pillar.chars().next().unwrap_or('甲');
    let time_gan_idx = TIANGAN.iter().position(|&g| g.starts_with(h_gan_char)).unwrap_or(0);
    let h_zhi_str: String = bz.hour_pillar.chars().skip(1).collect();
    let hour_zhi_idx = DIZHI.iter().position(|&z| z == h_zhi_str.as_str()).unwrap_or(0);

    // 1. 奇门遁甲 (按真实节气视黄经与四柱真实干支精细起局)
    let qm = calculate_qimen_with_date(
        year, month, day, hour, 0, 0,
        day_gan_idx, hour_zhi_idx, day_zhi_idx, time_gan_idx,
    );

    // 2. 太乙神数 (時計太乙 72 局立成与十六神落宫)
    let ty = calculate_taiyi(year, month, day, hour, 3, 0);

    // 3. 大六壬 (真实太阳视黄经求真月将：过中气换将)
    // 太阳黄经：春分0°(戌将/河魁), 谷雨30°(酉将/从魁), 小满60°(申将/传送)...
    // 月将与太阳黄经公式：yue_jiang = (11 - (sun_lon / 30.0) as usize) % 12
    let yue_jiang = (11.0 - (bz.sun_lon.rem_euclid(360.0) / 30.0).floor()).rem_euclid(12.0) as usize;
    let zhan_shi = hour_zhi_idx;
    let lr = calculate_liureng(yue_jiang, zhan_shi, d_gan_char, day_zhi_idx);

    SanShiUnitedResult {
        bazi: bz,
        qimen: qm,
        taiyi: ty,
        liureng: lr,
        summary: "三式合一：奇门主运筹、太乙主天道大数、六壬主人事机微，三盘呼应，天人地三才毕具。",
    }
}

/// 复合择日扫描器：统一将各家技法（奇门择日、太乙择日、六壬择日、八字择日、三式择日）
/// 转化为多维条件求交
pub fn scan_composite_zeri(
    start_date: &str,
    end_date: &str,
    conditions: &[ZeriCondition],
) -> Result<ZeriScanResult, String> {
    scan_zeri(start_date, end_date, conditions)
}
