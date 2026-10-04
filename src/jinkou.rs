// 大六壬金口诀推演

pub const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
pub const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

// 十二将神（传统序：0=登明亥,1=河魁戌,2=从魁酉,3=传送申,4=小吉未,5=胜光午,
//                   6=太乙巳,7=天罡辰,8=太冲卯,9=功曹寅,10=大吉丑,11=神后子）
pub const JIANG_SHEN: [&str; 12] = [
    "登明 (亥水)", "河魁 (戌土)", "从魁 (酉金)", "传送 (申金)", "小吉 (未土)", "胜光 (午火)",
    "太乙 (巳火)", "天罡 (辰土)", "太冲 (卯木)", "功曹 (寅木)", "大吉 (丑土)", "神后 (子水)"
];

// 十二贵神（天乙顺逆排布所得）
pub const GUI_SHEN: [&str; 12] = [
    "天乙贵人 (丑土)", "螣蛇 (巳火)", "朱雀 (午火)", "六合 (卯木)", "勾陈 (辰土)", "青龙 (寅木)",
    "天空 (戌土)", "白虎 (申金)", "太常 (未土)", "玄武 (子水)", "太阴 (酉金)", "天后 (亥水)"
];

// 五行属性 (水=0, 火=1, 木=2, 金=3, 土=4)
pub fn elem_of_zhi(z: &str) -> usize {
    match z {
        "亥" | "子" => 0,
        "巳" | "午" => 1,
        "寅" | "卯" => 2,
        "申" | "酉" => 3,
        _ => 4, // 辰 戌 丑 未
    }
}

pub fn elem_of_gan(g: char) -> usize {
    match g {
        '壬' | '癸' => 0,
        '丙' | '丁' => 1,
        '甲' | '乙' => 2,
        '庚' | '辛' => 3,
        _ => 4, // 戊 己
    }
}

pub fn wuxing_name(e: usize) -> &'static str {
    match e {
        0 => "水",
        1 => "火",
        2 => "木",
        3 => "金",
        _ => "土",
    }
}

// 生克关系: (0生1?): 木生火(2->1), 火生土(1->4), 土生金(4->3), 金生水(3->0), 水生木(0->2)
pub fn is_sheng(a: usize, b: usize) -> bool {
    (a == 2 && b == 1) || (a == 1 && b == 4) || (a == 4 && b == 3) || (a == 3 && b == 0) || (a == 0 && b == 2)
}

// a克b: 木克土(2->4), 土克水(4->0), 水克火(0->1), 火克金(1->3), 金克木(3->2)
pub fn is_ke(a: usize, b: usize) -> bool {
    (a == 2 && b == 4) || (a == 4 && b == 0) || (a == 0 && b == 1) || (a == 1 && b == 3) || (a == 3 && b == 2)
}

#[derive(Debug, serde::Serialize)]
pub struct JinKouFourLevel {
    pub name: &'static str,      // 位名: 人元 / 贵神 / 将神 / 地分
    pub gan_zhi: String,         // 干支或具体星神名称
    pub wuxing: &'static str,    // 五行
}

#[derive(Debug, serde::Serialize)]
pub struct JinKouResult {
    pub ren_yuan: JinKouFourLevel,  // 人元 (干)
    pub gui_shen: JinKouFourLevel,  // 贵神 (神)
    pub jiang_shen: JinKouFourLevel,// 将神 (将)
    pub di_fen: JinKouFourLevel,    // 地分 (分)
    pub dong_yao: Vec<&'static str>,// 五动格局判断 (妻动/财动/贼动/鬼动/兄弟动)
    pub summary: &'static str,      // 吉凶总断
}

/// 金口诀起课推导核心
pub fn calculate_jinkou(
    day_gan_char: char,
    hour_zhi_idx: usize,
    yue_jiang_zhi: usize,
    di_fen_zhi_idx: usize,
    is_day: bool,
) -> JinKouResult {
    // 1. 地分 (指定地支)
    let di_idx = di_fen_zhi_idx % 12;
    let di = ZHI[di_idx];
    let di_elem = elem_of_zhi(di);

    // 2. 将神 (月将加时排天盘，取落于地分之神)
    // yue_jiang_zhi 为传统十二月将序(0=登明亥)，先转为地支ZHI索引
    let yj_zhi_pos = crate::liureng::yue_jiang_to_zhi(yue_jiang_zhi);
    // jiang = (地分 + 月将地支位 - 占时) % 12
    let jiang_idx = ((di_idx as i32 + yj_zhi_pos as i32 - hour_zhi_idx as i32).rem_euclid(12)) as usize;
    let jiang_zhi = ZHI[jiang_idx];
    let jiang_elem = elem_of_zhi(jiang_zhi);
    // jiang_idx 是ZHI索引(0=子)，需转回传统序索引查JIANG_SHEN
    let jiang_trad_idx = (11 - jiang_idx) % 12;
    let jiang_name = JIANG_SHEN[jiang_trad_idx];

    // 3. 贵神 (起例默认六壬法，甲戊庚丑未...)
    let day_gan_idx = match day_gan_char {
        '甲' => 0, '乙' => 1, '丙' => 2, '丁' => 3, '戊' => 4,
        '己' => 5, '庚' => 6, '辛' => 7, '壬' => 8, _ => 9,
    };
    let (gui_start_zhi, reverse) = if is_day {
        match day_gan_char {
            '甲' | '戊' | '庚' => (1, false), // 丑
            '乙' | '己' => (0, false),       // 子
            '丙' | '丁' => (11, false),      // 亥
            '辛' => (6, false),             // 午
            _ => (3, false),                // 壬 癸: 卯
        }
    } else {
        match day_gan_char {
            '甲' | '戊' | '庚' => (7, true),  // 未
            '乙' | '己' => (8, true),       // 申
            '丙' | '丁' => (9, true),       // 酉
            '辛' => (2, true),             // 寅
            _ => (5, true),                // 壬 癸: 巳
        }
    };

    // 贵神序列：贵人、螣蛇、朱雀、六合、勾陈、青龙、天空、白虎、太常、玄武、太阴、天后
    // 顺逆自贵人起点排至地分
    let offset = ((di_idx as i32 - gui_start_zhi).rem_euclid(12)) as usize;
    let gui_god_idx = if reverse { (12 - offset) % 12 } else { offset };
    let gui_name = GUI_SHEN[gui_god_idx];
    let gui_wuxing_map = [4, 1, 1, 2, 4, 2, 4, 3, 4, 0, 3, 0]; // 丑巳午卯辰寅戌申未子酉亥
    let gui_elem = gui_wuxing_map[gui_god_idx];

    // 4. 人元 (日干五鼠遁起地分天干)
    let ren_gan_base = match day_gan_idx % 5 {
        0 => 0, // 甲己还生甲
        1 => 2, // 乙庚丙作初
        2 => 4, // 丙辛从戊起
        3 => 6, // 丁壬庚子居
        _ => 8, // 戊癸何方发，壬子是真途
    };
    let ren_gan = GAN[(ren_gan_base + di_idx) % 10];
    let ren_elem = elem_of_gan(ren_gan.chars().next().unwrap());

    // 5. 五动生克分析 (干神将分 四位生克)
    // 人元(干), 贵神(神), 将神(将), 地分(分)
    let mut dong_yao = Vec::new();

    // (1) 妻动: 下克上 (分克将，为财动/妻动)
    if is_ke(di_elem, jiang_elem) {
        dong_yao.push("妻动 (地分克将神: 妻财发动，求财易得)");
    }
    // (2) 贼动: 上克下 (神克将，为贼动/事动)
    if is_ke(gui_elem, jiang_elem) {
        dong_yao.push("贼动 (贵神克将神: 争竞破耗，门户不安)");
    }
    // (3) 鬼动: 神克干 (贵神克人元: 官讼灾殃，疾病缠身)
    if is_ke(gui_elem, ren_elem) {
        dong_yao.push("鬼动 (贵神克人元: 官司刑狱，盗贼侵害)");
    }
    // (4) 财动: 干克神 (人元克贵神: 财帛外动，忧疑不宁)
    if is_ke(ren_elem, gui_elem) {
        dong_yao.push("财动 (人元克贵神: 尊长受克，财利有失)");
    }
    // (5) 兄弟动: 同类相比 (将与分同五行)
    if jiang_elem == di_elem {
        dong_yao.push("兄弟动 (将分比和: 朋友相和，亦防分夺)");
    }

    let summary = if dong_yao.is_empty() {
        "四位相生比和，吉利祥泰"
    } else {
        "课现克动之机，动则事应"
    };

    JinKouResult {
        ren_yuan: JinKouFourLevel {
            name: "人元",
            gan_zhi: format!("{}{}", ren_gan, di),
            wuxing: wuxing_name(ren_elem),
        },
        gui_shen: JinKouFourLevel {
            name: "贵神",
            gan_zhi: gui_name.to_string(),
            wuxing: wuxing_name(gui_elem),
        },
        jiang_shen: JinKouFourLevel {
            name: "将神",
            gan_zhi: jiang_name.to_string(),
            wuxing: wuxing_name(jiang_elem),
        },
        di_fen: JinKouFourLevel {
            name: "地分",
            gan_zhi: di.to_string(),
            wuxing: wuxing_name(di_elem),
        },
        dong_yao,
        summary,
    }
}
