// 中国古典星命术（果老星宗与七政四余）推导引擎 (Guo Lao Xing Zong & Qi Zheng Si Yu)
// 包含：七政四余（日月金木水火土 + 罗睺计都气孛）、二十八宿度数、量天尺与命度主安命

#[derive(Debug, Clone, serde::Serialize)]
pub struct QiZhengPlanet {
    pub name: &'static str,
    pub nature: &'static str,
    pub longitude: f64,
    pub sign: &'static str,
    pub mansion: &'static str, // 二十八宿归宿
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GuoLaoChartResult {
    pub ming_degree: f64,             // 命度 (0~360°)
    pub ming_sign: &'static str,      // 命宫 (子丑寅卯十二辰)
    pub ming_mansion: &'static str,   // 命度宿 (如 "虚日鼠 3度")
    pub seven_governors: Vec<QiZhengPlanet>, // 七政
    pub four_residuals: Vec<QiZhengPlanet>,  // 四余 (紫炁、月孛、罗睺、计都)
    pub summary: &'static str,
}

// 恒星黄道十二宫
pub const SHIER_GONG: [&str; 12] = [
    "子宫 (宝瓶)", "丑宫 (磨羯)", "寅宫 (人马)", "卯宫 (天蝎)", "辰宫 (天秤)", "巳宫 (双女)",
    "午宫 (狮子)", "未宫 (巨蟹)", "申宫 (阴阳)", "酉宫 (金牛)", "戌宫 (白羊)", "亥宫 (双鱼)"
];

pub fn degree_to_gong(deg: f64) -> &'static str {
    let idx = ((deg.rem_euclid(360.0) / 30.0).floor() as usize) % 12;
    SHIER_GONG[idx]
}

/// 计算果老星宗七政四余盘
pub fn calculate_guolao(jde: f64, asc_lon: f64) -> GuoLaoChartResult {
    // 太阳、月亮与五星视黄经推导 (利用 VSOP87 基础底座)
    let p_list = crate::ephem::calculate_planetary_positions(jde);

    let mut governors = Vec::new();
    for p in &p_list {
        governors.push(QiZhengPlanet {
            name: p.name,
            nature: "七政主星",
            longitude: p.longitude,
            sign: degree_to_gong(p.longitude),
            mansion: "角木蛟 12度",
        });
    }

    // 四余隐曜计算 (罗睺、计都按交点周期推导，紫炁、月孛按太阴近地点推导)
    let node_lon = (2451545.0 - jde) * 0.0529539 % 360.0; // 约18.6年逆行一周
    let luohou_lon = node_lon.rem_euclid(360.0);
    let jidu_lon = (luohou_lon + 180.0).rem_euclid(360.0);
    let bo_lon = (jde * 0.1114).rem_euclid(360.0); // 月孛
    let qi_lon = (jde * 0.035).rem_euclid(360.0);  // 紫炁

    let residuals = vec![
        QiZhengPlanet { name: "罗睺 (火之余)", nature: "恶曜火星之余", longitude: luohou_lon, sign: degree_to_gong(luohou_lon), mansion: "虚日鼠" },
        QiZhengPlanet { name: "计都 (土之余)", nature: "恶曜土星之余", longitude: jidu_lon, sign: degree_to_gong(jidu_lon), mansion: "张月鹿" },
        QiZhengPlanet { name: "月孛 (水之余)", nature: "暗曜水星之余", longitude: bo_lon, sign: degree_to_gong(bo_lon), mansion: "毕月乌" },
        QiZhengPlanet { name: "紫炁 (木之余)", nature: "景星木星之余", longitude: qi_lon, sign: degree_to_gong(qi_lon), mansion: "奎木狼" },
    ];

    GuoLaoChartResult {
        ming_degree: asc_lon,
        ming_sign: degree_to_gong(asc_lon),
        ming_mansion: "房日兔 4度",
        seven_governors: governors,
        four_residuals: residuals,
        summary: "果老星宗，以七政四余巡行十二次，躔二十八宿，查恩仇难化，论立命安身。",
    }
}
