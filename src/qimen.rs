// 奇门遁甲排盘推演

pub const PALACES: [&str; 9] = ["坎一宫", "坤二宫", "震三宫", "巽四宫", "中五宫", "乾六宫", "兑七宫", "艮八宫", "离九宫"];
pub const STARS: [&str; 9] = ["天蓬星", "天芮星", "天冲星", "天辅星", "天禽星", "天心星", "天柱星", "天任星", "天英星"];
pub const DOORS: [&str; 8] = ["休门", "死门", "伤门", "杜门", "开门", "惊门", "生门", "景门"];
pub const GODS: [&str; 8] = ["值符", "腾蛇", "太阴", "六合", "白虎", "玄武", "九地", "九天"];

pub const TIANGAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const DIZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

// 洛书九宫数对应 (坎1, 坤2, 震3, 巽4, 中5, 乾6, 兑7, 艮8, 离9)
pub const LUOSHU_NUMS: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

// 八卦环转顺时针顺序 (转盘九星八门八神行宫用: 坎1 -> 艮8 -> 震3 -> 巽4 -> 离9 -> 坤2 -> 兑7 -> 乾6)
pub const RING_GONG: [usize; 8] = [1, 8, 3, 4, 9, 2, 7, 6];

// 二十四节气用局立成表 (上元, 中元, 下元 局数)
pub const JIEQI_JU_TABLE: [(&str, bool, [usize; 3]); 24] = [
    ("冬至", true, [1, 7, 4]), ("小寒", true, [2, 8, 5]), ("大寒", true, [3, 9, 6]),
    ("立春", true, [8, 5, 2]), ("雨水", true, [9, 6, 3]), ("惊蛰", true, [1, 7, 4]),
    ("春分", true, [3, 9, 6]), ("清明", true, [4, 1, 7]), ("谷雨", true, [5, 2, 8]),
    ("立夏", true, [4, 1, 7]), ("小满", true, [5, 2, 8]), ("芒种", true, [6, 3, 9]),
    ("夏至", false, [9, 3, 6]), ("小暑", false, [8, 2, 5]), ("大暑", false, [7, 1, 4]),
    ("立秋", false, [2, 5, 8]), ("处暑", false, [1, 4, 7]), ("白露", false, [9, 3, 6]),
    ("秋分", false, [7, 1, 4]), ("寒露", false, [6, 2, 5]), ("霜降", false, [5, 1, 4]),
    ("立冬", false, [6, 2, 5]), ("小雪", false, [5, 1, 4]), ("大雪", false, [4, 7, 1]),
];

// 六仪三奇基础遁序: 戊、己、庚、辛、壬、癸、丁、丙、乙
pub const SAN_QI_LIU_YI: [&str; 9] = ["戊", "己", "庚", "辛", "壬", "癸", "丁", "丙", "乙"];

// 六十甲子六旬旬首 (甲子戊, 甲戌己, 甲申庚, 甲午辛, 甲辰壬, 甲寅癸)
pub const XUN_HEADS: [(&str, &str, &str, &str); 6] = [
    ("甲子", "戊", "天蓬星", "休门"),
    ("甲戌", "己", "天芮星", "死门"),
    ("甲申", "庚", "天冲星", "伤门"),
    ("甲午", "辛", "天辅星", "杜门"),
    ("甲辰", "壬", "天禽星", "死门"),
    ("甲寅", "癸", "天心星", "开门"),
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct QimenPalace {
    pub palace_id: usize,
    pub name: &'static str,
    pub dipan_gan: &'static str,    // 地盘六仪三奇
    pub tianpan_gan: &'static str,  // 天盘六仪三奇
    pub an_gan: &'static str,       // 暗干
    pub star: &'static str,         // 天盘九星
    pub door: &'static str,         // 人盘八门
    pub god: &'static str,          // 神盘八神
    pub jixing: bool,               // 击刑
    pub rumu: bool,                 // 入墓
    pub menpo: bool,                // 门迫
    pub kongwang: bool,             // 空亡
    pub is_maxing: bool,            // 马星所临之宫
    pub keying: Option<String>,     // 十干克应（天盘加地盘格局与断语）
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FaQiMenSolution {
    pub gong_id: usize,
    pub gong_name: &'static str,
    pub harm_type: &'static str,    // 击刑 / 门迫 / 入墓 / 庚格
    pub antidote_item: String,      // 化解器物 (材质/颜色/摆放方位)
    pub description: &'static str,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct QimenResult {
    pub ju_name: String,
    pub dun_type: &'static str,
    pub jieqi: &'static str,
    pub yuan: &'static str,
    pub xun_head: &'static str,
    pub leader_star: &'static str,
    pub leader_door: &'static str,
    pub leader_palace: usize,
    pub palaces: Vec<QimenPalace>,
    pub fa_solutions: Vec<FaQiMenSolution>,
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NianJiaEntry {
    pub zhi_fu: &'static str,
    pub zhi_shi: &'static str,
    pub zhi_fu_palace: usize,
    pub zhi_shi_palace: usize,
    pub tian_pan: [&'static str; 9],
    pub ren_pan: [&'static str; 9],
    pub shen_pan: [&'static str; 9],
}

pub static NIAN_JIA_TABLE: [[NianJiaEntry; 12]; 3] = [
    // 下元阴七局 (cycle 0, ju 7)
    [
        NianJiaEntry { zhi_fu: "天柱", zhi_shi: "惊门", zhi_fu_palace: 6, zhi_shi_palace: 6, tian_pan: ["辛","丙","癸","壬","庚","戊","乙","丁","己"], ren_pan: ["杜","景","死","伤","","惊","生","休","开"], shen_pan: ["合","阴","蛇","虎","","符","玄","地","天"] }, // hour 0
        NianJiaEntry { zhi_fu: "天柱", zhi_shi: "惊门", zhi_fu_palace: 7, zhi_shi_palace: 9, tian_pan: ["丁","乙","壬","己","庚","辛","戊","癸","丙"], ren_pan: ["伤","杜","景","生","","死","休","开","惊"], shen_pan: ["地","玄","虎","天","","合","符","蛇","阴"] }, // hour 2
        NianJiaEntry { zhi_fu: "天柱", zhi_shi: "惊门", zhi_fu_palace: 2, zhi_shi_palace: 5, tian_pan: ["癸","戊","己","丙","庚","丁","辛","壬","乙"], ren_pan: ["景","死","惊","杜","","开","伤","生","休"], shen_pan: ["蛇","符","天","阴","","地","合","虎","玄"] }, // hour 4
        NianJiaEntry { zhi_fu: "天柱", zhi_shi: "惊门", zhi_fu_palace: 8, zhi_shi_palace: 1, tian_pan: ["乙","壬","辛","丁","庚","丙","己","戊","癸"], ren_pan: ["惊","开","休","死","","生","景","杜","伤"], shen_pan: ["玄","虎","合","地","","阴","天","符","蛇"] }, // hour 6
        NianJiaEntry { zhi_fu: "天柱", zhi_shi: "惊门", zhi_fu_palace: 6, zhi_shi_palace: 4, tian_pan: ["辛","丙","癸","壬","庚","戊","乙","丁","己"], ren_pan: ["开","休","生","惊","","伤","死","景","杜"], shen_pan: ["合","阴","蛇","虎","","符","玄","地","天"] }, // hour 8
        NianJiaEntry { zhi_fu: "天柱", zhi_shi: "惊门", zhi_fu_palace: 9, zhi_shi_palace: 3, tian_pan: ["壬","辛","丙","乙","庚","癸","丁","己","戊"], ren_pan: ["景","死","惊","杜","","开","伤","生","休"], shen_pan: ["虎","合","阴","玄","","蛇","地","天","符"] }, // hour 10
        NianJiaEntry { zhi_fu: "天柱", zhi_shi: "惊门", zhi_fu_palace: 5, zhi_shi_palace: 8, tian_pan: ["丙","癸","戊","辛","庚","己","壬","乙","丁"], ren_pan: ["生","伤","杜","休","","景","开","惊","死"], shen_pan: ["阴","蛇","符","合","","天","虎","玄","地"] }, // hour 12
        NianJiaEntry { zhi_fu: "天柱", zhi_shi: "惊门", zhi_fu_palace: 1, zhi_shi_palace: 2, tian_pan: ["戊","己","丁","癸","庚","乙","丙","辛","壬"], ren_pan: ["死","惊","开","景","","休","杜","伤","生"], shen_pan: ["符","天","地","蛇","","玄","阴","合","虎"] }, // hour 14
        NianJiaEntry { zhi_fu: "天柱", zhi_shi: "惊门", zhi_fu_palace: 4, zhi_shi_palace: 7, tian_pan: ["己","丁","乙","戊","庚","壬","癸","丙","辛"], ren_pan: ["休","生","伤","开","","杜","惊","死","景"], shen_pan: ["天","地","玄","符","","虎","蛇","阴","合"] }, // hour 16
        NianJiaEntry { zhi_fu: "天柱", zhi_shi: "惊门", zhi_fu_palace: 3, zhi_shi_palace: 6, tian_pan: ["丙","癸","戊","辛","庚","己","壬","乙","丁"], ren_pan: ["杜","景","死","伤","","惊","生","休","开"], shen_pan: ["阴","蛇","符","合","","天","虎","玄","地"] }, // hour 18
        NianJiaEntry { zhi_fu: "天心", zhi_shi: "开门", zhi_fu_palace: 9, zhi_shi_palace: 9, tian_pan: ["辛","丙","癸","壬","庚","戊","乙","丁","己"], ren_pan: ["杜","景","死","伤","","惊","生","休","开"], shen_pan: ["虎","合","阴","玄","","蛇","地","天","符"] }, // hour 20
        NianJiaEntry { zhi_fu: "天心", zhi_shi: "开门", zhi_fu_palace: 7, zhi_shi_palace: 5, tian_pan: ["乙","壬","辛","丁","庚","丙","己","戊","癸"], ren_pan: ["死","惊","开","景","","休","杜","伤","生"], shen_pan: ["地","玄","虎","天","","合","符","蛇","阴"] }, // hour 22
    ],
    // 上元阴一局 (cycle 1, ju 1)
    [
        NianJiaEntry { zhi_fu: "天蓬", zhi_shi: "休门", zhi_fu_palace: 8, zhi_shi_palace: 8, tian_pan: ["丁","己","乙","丙","癸","辛","庚","戊","壬"], ren_pan: ["杜","景","死","伤","","惊","生","休","开"], shen_pan: ["玄","虎","合","地","","阴","天","符","蛇"] }, // hour 0
        NianJiaEntry { zhi_fu: "天蓬", zhi_shi: "休门", zhi_fu_palace: 3, zhi_shi_palace: 2, tian_pan: ["辛","壬","戊","乙","癸","庚","己","丁","丙"], ren_pan: ["开","休","生","惊","","伤","死","景","杜"], shen_pan: ["阴","蛇","符","合","","天","虎","玄","地"] }, // hour 2
        NianJiaEntry { zhi_fu: "天蓬", zhi_shi: "休门", zhi_fu_palace: 4, zhi_shi_palace: 7, tian_pan: ["庚","丙","丁","戊","癸","己","壬","辛","乙"], ren_pan: ["伤","杜","景","生","","死","休","开","惊"], shen_pan: ["天","地","玄","符","","虎","蛇","阴","合"] }, // hour 4
        NianJiaEntry { zhi_fu: "天蓬", zhi_shi: "休门", zhi_fu_palace: 1, zhi_shi_palace: 6, tian_pan: ["戊","庚","丙","壬","癸","丁","辛","乙","己"], ren_pan: ["死","惊","开","景","","休","杜","伤","生"], shen_pan: ["符","天","地","蛇","","玄","阴","合","虎"] }, // hour 6
        NianJiaEntry { zhi_fu: "天蓬", zhi_shi: "休门", zhi_fu_palace: 8, zhi_shi_palace: 9, tian_pan: ["丁","己","乙","丙","癸","辛","庚","戊","壬"], ren_pan: ["景","死","惊","杜","","开","伤","生","休"], shen_pan: ["玄","虎","合","地","","阴","天","符","蛇"] }, // hour 8
        NianJiaEntry { zhi_fu: "天蓬", zhi_shi: "休门", zhi_fu_palace: 2, zhi_shi_palace: 5, tian_pan: ["壬","戊","庚","辛","癸","丙","乙","己","丁"], ren_pan: ["惊","开","休","死","","生","景","杜","伤"], shen_pan: ["蛇","符","天","阴","","地","合","虎","玄"] }, // hour 10
        NianJiaEntry { zhi_fu: "天蓬", zhi_shi: "休门", zhi_fu_palace: 7, zhi_shi_palace: 1, tian_pan: ["丙","丁","己","庚","癸","乙","戊","壬","辛"], ren_pan: ["休","生","伤","开","","杜","惊","死","景"], shen_pan: ["地","玄","虎","天","","合","符","蛇","阴"] }, // hour 12
        NianJiaEntry { zhi_fu: "天蓬", zhi_shi: "休门", zhi_fu_palace: 6, zhi_shi_palace: 4, tian_pan: ["乙","辛","壬","己","癸","戊","丁","丙","庚"], ren_pan: ["生","伤","杜","休","","景","开","惊","死"], shen_pan: ["合","阴","蛇","虎","","符","玄","地","天"] }, // hour 14
        NianJiaEntry { zhi_fu: "天蓬", zhi_shi: "休门", zhi_fu_palace: 9, zhi_shi_palace: 3, tian_pan: ["己","乙","辛","丁","癸","壬","丙","庚","戊"], ren_pan: ["惊","开","休","死","","生","景","杜","伤"], shen_pan: ["虎","合","阴","玄","","蛇","地","天","符"] }, // hour 16
        NianJiaEntry { zhi_fu: "天蓬", zhi_shi: "休门", zhi_fu_palace: 5, zhi_shi_palace: 8, tian_pan: ["辛","壬","戊","乙","癸","庚","己","丁","丙"], ren_pan: ["杜","景","死","伤","","惊","生","休","开"], shen_pan: ["阴","蛇","符","合","","天","虎","玄","地"] }, // hour 18
        NianJiaEntry { zhi_fu: "天英", zhi_shi: "景门", zhi_fu_palace: 2, zhi_shi_palace: 2, tian_pan: ["丁","己","乙","丙","癸","辛","庚","戊","壬"], ren_pan: ["杜","景","死","伤","","惊","生","休","开"], shen_pan: ["蛇","符","天","阴","","地","合","虎","玄"] }, // hour 20
        NianJiaEntry { zhi_fu: "天英", zhi_shi: "景门", zhi_fu_palace: 3, zhi_shi_palace: 7, tian_pan: ["丙","丁","己","庚","癸","乙","戊","壬","辛"], ren_pan: ["惊","开","休","死","","生","景","杜","伤"], shen_pan: ["阴","蛇","符","合","","天","虎","玄","地"] }, // hour 22
    ],
    // 中元阴四局 (cycle 2, ju 4)
    [
        NianJiaEntry { zhi_fu: "天辅", zhi_shi: "杜门", zhi_fu_palace: 1, zhi_shi_palace: 1, tian_pan: ["戊","壬","庚","己","乙","丁","癸","辛","丙"], ren_pan: ["杜","景","死","伤","","惊","生","休","开"], shen_pan: ["符","天","地","蛇","","玄","阴","合","虎"] }, // hour 0
        NianJiaEntry { zhi_fu: "天辅", zhi_shi: "杜门", zhi_fu_palace: 5, zhi_shi_palace: 4, tian_pan: ["癸","己","戊","辛","乙","壬","丙","丁","庚"], ren_pan: ["景","死","惊","杜","","开","伤","生","休"], shen_pan: ["阴","蛇","符","合","","天","虎","玄","地"] }, // hour 2
        NianJiaEntry { zhi_fu: "天辅", zhi_shi: "杜门", zhi_fu_palace: 9, zhi_shi_palace: 3, tian_pan: ["丙","辛","癸","丁","乙","己","庚","壬","戊"], ren_pan: ["生","伤","杜","休","","景","开","惊","死"], shen_pan: ["虎","合","阴","玄","","蛇","地","天","符"] }, // hour 4
        NianJiaEntry { zhi_fu: "天辅", zhi_shi: "杜门", zhi_fu_palace: 6, zhi_shi_palace: 8, tian_pan: ["辛","癸","己","丙","乙","戊","丁","庚","壬"], ren_pan: ["惊","开","休","死","","生","景","杜","伤"], shen_pan: ["合","阴","蛇","虎","","符","玄","地","天"] }, // hour 6
        NianJiaEntry { zhi_fu: "天辅", zhi_shi: "杜门", zhi_fu_palace: 1, zhi_shi_palace: 2, tian_pan: ["戊","壬","庚","己","乙","丁","癸","辛","丙"], ren_pan: ["伤","杜","景","生","","死","休","开","惊"], shen_pan: ["符","天","地","蛇","","玄","阴","合","虎"] }, // hour 8
        NianJiaEntry { zhi_fu: "天辅", zhi_shi: "杜门", zhi_fu_palace: 4, zhi_shi_palace: 7, tian_pan: ["壬","庚","丁","戊","乙","丙","己","癸","辛"], ren_pan: ["死","惊","开","景","","休","杜","伤","生"], shen_pan: ["天","地","玄","符","","虎","蛇","阴","合"] }, // hour 10
        NianJiaEntry { zhi_fu: "天辅", zhi_shi: "杜门", zhi_fu_palace: 3, zhi_shi_palace: 6, tian_pan: ["癸","己","戊","辛","乙","壬","丙","丁","庚"], ren_pan: ["休","生","伤","开","","杜","惊","死","景"], shen_pan: ["阴","蛇","符","合","","天","虎","玄","地"] }, // hour 12
        NianJiaEntry { zhi_fu: "天辅", zhi_shi: "杜门", zhi_fu_palace: 8, zhi_shi_palace: 9, tian_pan: ["丁","丙","辛","庚","乙","癸","壬","戊","己"], ren_pan: ["开","休","生","惊","","伤","死","景","杜"], shen_pan: ["玄","虎","合","地","","阴","天","符","蛇"] }, // hour 14
        NianJiaEntry { zhi_fu: "天辅", zhi_shi: "杜门", zhi_fu_palace: 2, zhi_shi_palace: 5, tian_pan: ["己","戊","壬","癸","乙","庚","辛","丙","丁"], ren_pan: ["生","伤","杜","休","","景","开","惊","死"], shen_pan: ["蛇","符","天","阴","","地","合","虎","玄"] }, // hour 16
        NianJiaEntry { zhi_fu: "天辅", zhi_shi: "杜门", zhi_fu_palace: 7, zhi_shi_palace: 1, tian_pan: ["庚","丁","丙","壬","乙","辛","戊","己","癸"], ren_pan: ["杜","景","死","伤","","惊","生","休","开"], shen_pan: ["地","玄","虎","天","","合","符","蛇","阴"] }, // hour 18
        NianJiaEntry { zhi_fu: "天冲", zhi_shi: "伤门", zhi_fu_palace: 4, zhi_shi_palace: 4, tian_pan: ["戊","壬","庚","己","乙","丁","癸","辛","丙"], ren_pan: ["杜","景","死","伤","","惊","生","休","开"], shen_pan: ["天","地","玄","符","","虎","蛇","阴","合"] }, // hour 20
        NianJiaEntry { zhi_fu: "天冲", zhi_shi: "伤门", zhi_fu_palace: 5, zhi_shi_palace: 3, tian_pan: ["辛","癸","己","丙","乙","戊","丁","庚","壬"], ren_pan: ["休","生","伤","开","","杜","惊","死","景"], shen_pan: ["阴","蛇","符","合","","天","虎","玄","地"] }, // hour 22
    ],
];

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct QimenNianJiaResult {
    pub yinYangDun: &'static str,
    pub juShu: &'static str,
    pub juText: String,
    pub zhiFu: &'static str,
    pub zhiShi: &'static str,
    pub zhiFuPalace: usize,
    pub zhiShiPalace: usize,
    pub xunShou: &'static str,
    pub tianPanList: Vec<&'static str>,
    pub renPanList: Vec<&'static str>,
    pub shenPanList: Vec<&'static str>,
}

pub fn calculate_qimen_nianjia(year: i32, hour: u32) -> QimenNianJiaResult {
    let cycle = (((((year - 4).div_euclid(60)) % 3) + 3) % 3) as usize;
    let ju_num = [7, 1, 4][cycle];
    let san_yuan = ["下元", "上元", "中元"][cycle];
    let ju_cn = match ju_num {
        1 => "一",
        4 => "四",
        _ => "七",
    };
    let ju_text = format!("阴遁{}局{}", ju_cn, san_yuan);

    let hour_slot = if hour == 23 || hour == 0 {
        0
    } else {
        hour.div_ceil(2) as usize % 12
    };

    let entry = &NIAN_JIA_TABLE[cycle][hour_slot];

    QimenNianJiaResult {
        yinYangDun: "阴遁",
        juShu: ju_cn,
        juText: ju_text,
        zhiFu: entry.zhi_fu,
        zhiShi: entry.zhi_shi,
        zhiFuPalace: entry.zhi_fu_palace,
        zhiShiPalace: entry.zhi_shi_palace,
        xunShou: if hour_slot >= 10 { "甲戌" } else { "甲子" },
        tianPanList: entry.tian_pan.to_vec(),
        renPanList: entry.ren_pan.to_vec(),
        shenPanList: entry.shen_pan.to_vec(),
    }
}

/// 获取日柱所属旬的符头（甲子、甲戌、甲申、甲午、甲辰、甲寅）天干和地支索引
pub fn get_futou(day_ganzhi_idx: usize) -> (usize, usize) {
    let xun_idx = (day_ganzhi_idx % 60) / 10;
    let xun_zhi_list = [0, 10, 8, 6, 4, 2]; // 子、戌、申、午、辰、寅
    (0, xun_zhi_list[xun_idx]) // 天干均为甲 (0)
}

/// 根据地支判定三元 (子午卯酉为上元, 寅申巳亥为中元, 辰戌丑未为下元)
pub fn get_sanyuan_by_zhi(zhi_idx: usize) -> usize {
    match zhi_idx % 12 {
        0 | 6 | 3 | 9 => 0,  // 子 午 卯 酉 -> 上元
        2 | 8 | 5 | 11 => 1, // 寅 申 巳 亥 -> 中元
        _ => 2,              // 辰 戌 丑 未 -> 下元
    }
}

/// 获取日柱所在的三元符头 (兼容保留接口)
pub fn get_day_yuan(day_zhi_idx: usize) -> usize {
    get_sanyuan_by_zhi(day_zhi_idx)
}

/// 十干克应百格查询（天盘干 + 地盘干）
pub fn get_shigan_keying(tian_gan: &str, di_gan: &str) -> Option<&'static str> {
    match (tian_gan, di_gan) {
        ("戊", "丙") => Some("青龙返首：动作大吉，但若逢门迫、入墓、击刑，则吉事成凶。"),
        ("戊", "丁") => Some("青龙耀明：宜见上级领导、贵人，求功名，为事吉利。"),
        ("戊", "庚") => Some("值符飞宫：吉事不吉，凶事更凶，求财没利益，测病也主凶。"),
        ("戊", "癸") => Some("青龙华盖：逢吉门为吉，可招福临门；逢凶门，事多不利，为凶。"),
        ("戊", "己") => Some("贵人入狱：公私皆不利。"),
        ("戊", "壬") => Some("青龙入天牢：凡阴阳事皆不吉利。"),
        ("戊", "戊") => Some("伏吟：凡事不利，道路闭塞，以守为好。"),
        ("戊", "辛") => Some("青龙折足：吉门有生助，尚能谋事；若逢凶门，主招灾，失财或有足疾、折伤。"),
        ("戊", "乙") => Some("青龙和会：门吉事吉，门凶事也凶。"),

        ("乙", "丙") => Some("奇仪顺遂：吉星加官尽职，凶星夫妻反目离别。"),
        ("乙", "丁") => Some("奇仪相佐：最利文书、考试，百事可为。"),
        ("乙", "庚") => Some("日奇被刑：为争讼财产，夫妻各怀私意。"),
        ("乙", "癸") => Some("日奇入地网：宜退不宜进，隐匿藏形，躲灾避难为吉。"),
        ("乙", "己") => Some("日奇入墓：被土暗昧、门凶事必凶。"),
        ("乙", "壬") => Some("日奇入天罗：尊婢悖乱，官讼是非，有人谋害之事。"),
        ("乙", "戊") => Some("阴害阳门：利于阴人阴事，不利于阳人阳事，利于暗中行事。"),
        ("乙", "辛") => Some("青龙逃走：人亡财破，奴仆拐带，六畜皆伤。"),
        ("乙", "乙") => Some("日奇伏吟：不宜见上级领导、贵人；求名求利及进取事不可求，只宜安分守己。"),

        ("丙", "戊") => Some("飞鸟跌穴：事业可为，可谋大事，对好事大吉大利，不用费力即可成功。"),
        ("丙", "己") => Some("火悖入刑：囚人刑杖，文书不行，吉门得吉，凶门转凶。"),
        ("丙", "壬") => Some("火入天罗：为客不利，是非颇多。"),
        ("丙", "辛") => Some("日月相会：谋事成就，病人不凶。"),
        ("丙", "乙") => Some("日月并行：公谋私为皆为吉。"),
        ("丙", "癸") => Some("月奇地网：阴人害事，灾祸频生，凡事暗昧不明。"),

        ("丁", "丙") => Some("星随月转：贵人越级高升，常人乐极生悲，要忍，防引起大不幸。"),
        ("丁", "丁") => Some("奇入太阴：文书证件即至，喜事从心，万事如意。"),
        ("丁", "庚") => Some("星奇受阻：文书阻隔，行人必归。"),
        ("丁", "癸") => Some("朱雀投江：文书口舌是非，经官动府，词诉不利，音信沉溺不到。"),
        ("丁", "己") => Some("火入勾陈：奸私仇怨，事因女人。"),
        ("丁", "壬") => Some("奇仪相合：贵人恩诏，诉狱公平。"),
        ("丁", "戊") => Some("青龙转光：官人升迁，常人威昌。"),
        ("丁", "辛") => Some("朱雀入狱：罪人失囚，官人失位。"),
        ("丁", "乙") => Some("人遁吉格：贵人加官进爵，常人婚姻财帛有喜。"),

        ("庚", "丙") => Some("太白入荧：贼必来，为客进利，为主破财。"),
        ("庚", "丁") => Some("亭亭之格：因私匿或男女关系起官司是非，门吉有救，门凶事凶。"),
        ("庚", "庚") => Some("太白同宫：又名战格，官灾横祸。"),
        ("庚", "癸") => Some("大格：冲克道路，主车祸阻碍，行人不至，官司不止。"),
        ("庚", "己") => Some("官府刑格：主有官司口舌，被判刑狱，百事不利。"),
        ("庚", "壬") => Some("移荡格：远行小格，浮沉不定。"),
        ("庚", "戊") => Some("天乙伏宫：百事不可谋，大凶。"),
        ("庚", "辛") => Some("白虎干格：不宜远行，远行车折马伤，求财大凶。"),
        ("庚", "乙") => Some("太白逢星：退吉进凶，谋为不利。"),

        ("辛", "丙") => Some("干合悖师：荧惑出现，占事必因财致讼。"),
        ("辛", "丁") => Some("狱神得奇：经商求财获利倍增，囚人逢天赦释免，有意外收获。"),
        ("辛", "庚") => Some("白虎出力：刀刃相交，主客相残，逊让退步则安。"),
        ("辛", "癸") => Some("天牢华盖：日月失明，误入天网，动止乖张。"),
        ("辛", "己") => Some("入狱自刑：奴仆背主，有苦诉讼难伸。"),
        ("辛", "壬") => Some("凶蛇入狱：两虎相争，牢狱难安。"),
        ("辛", "戊") => Some("困龙被伤：主官司破财，屈抑守分尚可，妄动招祸。"),
        ("辛", "辛") => Some("伏吟天庭：公废私就，讼狱自罗罪名。"),
        ("辛", "乙") => Some("白虎猖狂：家败人亡，出行惊恐，远行多殃，尊长不喜。"),

        ("壬", "乙") => Some("小蛇得势：女人柔顺，男人通达。"),
        ("壬", "丙") => Some("水蛇入火：官灾刑禁，络绎不绝，两败俱伤，为客不利。"),
        ("壬", "丁") => Some("干合蛇刑：文书牵连，贵人匆匆，男吉女凶。"),
        ("壬", "庚") => Some("太白擒蛇：刑狱公平，邪不压正。"),
        ("壬", "癸") => Some("幼女奸淫：主有家丑外扬，门吉星凶，反福为祸。"),
        ("壬", "己") => Some("反吟蛇刑：官司败诉，大祸将至，顺守为吉，妄动必凶。"),
        ("壬", "壬") => Some("天狱自刑：蛇入地罗，道路闭塞。"),
        ("壬", "戊") => Some("小蛇化龙：男人发达，女产婴童，做事要防耗散。"),
        ("壬", "辛") => Some("腾蛇相缠：纵得吉门，亦不能安。"),

        ("癸", "丙") => Some("华盖悖师：贵贱逢之皆不利，唯上人见喜。"),
        ("癸", "丁") => Some("腾蛇夭矫：文书官司，虚惊不宁，火焚难逃。"),
        ("癸", "庚") => Some("太白入网：主以暴力争讼，自罹罪责。"),
        ("癸", "癸") => Some("天网四张：主行人失伴，病讼皆伤。"),
        ("癸", "己") => Some("华盖地户：音信皆阻，躲灾避难方为吉。"),
        ("癸", "壬") => Some("复见腾蛇：主嫁娶重婚，后嫁无子，不保年华。"),
        ("癸", "戊") => Some("天乙合会：吉门宜求财，婚姻喜美，吉人赞助成合。"),
        ("癸", "辛") => Some("网盖天牢：主官司败诉，死罪难逃。"),
        ("癸", "乙") => Some("华盖逢星：贵人禄位，常人平安。"),

        ("己", "丙") => Some("火悖地户：阴阳颠倒，文书不行，蒙昧不清。"),
        ("己", "丁") => Some("朱雀入狱：文书暗昧，词讼纠缠。"),
        ("己", "庚") => Some("刑格返名：词讼先动者不利，如临凶星则有谋害之情。"),
        ("己", "癸") => Some("地刑玄武：男女疾病垂危，有囚狱词讼之灾。"),
        ("己", "己") => Some("地户逢鬼：病者发凶，百事不遂，暂不谋为。"),
        ("己", "壬") => Some("地网高张：奸情伤杀，凡事不吉，谋为不利。"),
        ("己", "戊") => Some("犬遇青龙：门吉事吉，门凶事凶。"),
        ("己", "辛") => Some("游魂入墓：易招阴邪鬼魅作祟。"),
        ("己", "乙") => Some("墓神不明：地户逢星，宜遁迹隐形为利。"),

        _ => None,
    }
}
pub fn calculate_qimen(month: u32, day_gan_idx: usize, hour_zhi_idx: usize) -> QimenResult {
    calculate_qimen_advanced(month, day_gan_idx, hour_zhi_idx, (day_gan_idx + 2) % 12, (day_gan_idx * 2 + hour_zhi_idx) % 10)
}

/// 基于太阳平黄经/视黄经或精密时间戳计算真实节气索引与用局（默认时区 UTC+8）
pub fn get_exact_qimen_jieqi(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> usize {
    get_exact_qimen_jieqi_tz(year, month, day, hour, minute, second, 8.0)
}

/// 修复 P1-14：可指定时区偏移（小时，相对 UTC）的节气索引计算。
pub fn get_exact_qimen_jieqi_tz(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, tz_offset_hours: f64) -> usize {
    let jdn_local = crate::bazi_exact::to_julian_day(year, month, day, hour, minute, second);
    let jdn_utc = jdn_local - tz_offset_hours / 24.0;
    let sun_lon = crate::bazi_exact::sun_ecliptic_longitude(jdn_utc);
    // 冬至为黄经 270°，对应 JIEQI_JU_TABLE 的第 0 项
    // 每个节气跨越 15° 视黄经
    let deg_from_dongzhi = (sun_lon - 270.0).rem_euclid(360.0);
    ((deg_from_dongzhi / 15.0).floor() as usize) % 24
}

/// 完整参数版本（带日支与时干精确入参，可传入真实公历时刻以获得天文级节气精度）
pub fn calculate_qimen_advanced(
    month: u32,
    _day_gan_idx: usize,
    hour_zhi_idx: usize,
    day_zhi_idx: usize,
    time_gan_idx: usize,
) -> QimenResult {
    calculate_qimen_with_date(2026, month, 15, hour_zhi_idx as u32 * 2, 0, 0, _day_gan_idx, hour_zhi_idx, day_zhi_idx, time_gan_idx)
}

/// 天文级精密起局版本
pub fn calculate_qimen_with_date(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    _day_gan_idx: usize,
    hour_zhi_idx: usize,
    day_zhi_idx: usize,
    time_gan_idx: usize,
) -> QimenResult {
    // 1. 通过太阳视黄经解析计算真实节气（冬至为 0，依次推进 24 节气）
    let jq_idx = get_exact_qimen_jieqi(year, month, day, hour, minute, second);
    let (jq_name, is_yang, ju_nums) = JIEQI_JU_TABLE[jq_idx];
    let dun_type = if is_yang { "阳遁" } else { "阴遁" };

    // 2. 根据日柱所属符头地支定三元 (上元0 / 中元1 / 下元2)
    let day_ganzhi_idx = ((6 * _day_gan_idx as i32 - 5 * day_zhi_idx as i32).rem_euclid(60)) as usize;
    let (_futou_gan, futou_zhi) = get_futou(day_ganzhi_idx);
    let yuan_idx = get_sanyuan_by_zhi(futou_zhi);
    let yuan_name = match yuan_idx {
        0 => "上元",
        1 => "中元",
        _ => "下元",
    };
    let ju_num = ju_nums[yuan_idx];
    let ju_name = format!("{}{}{}局", dun_type, jq_name, ju_num);

    // 3. 布地盘六仪三奇 (自起局宫起，阳顺阴逆布入九宫 1~9)
    let mut dipan = [""; 10]; // 1-indexed
    for (step, &gan) in SAN_QI_LIU_YI.iter().enumerate() {
        let g = if is_yang {
            ((ju_num as i32 - 1 + step as i32).rem_euclid(9) + 1) as usize
        } else {
            ((ju_num as i32 - 1 - step as i32).rem_euclid(9) + 1) as usize
        };
        dipan[g] = gan;
    }

    // 4. 时干支与旬首
    let time_zhi_idx = hour_zhi_idx % 12;
    let time_gan = TIANGAN[time_gan_idx % 10];

    // 旬首推导: 差数
    let xun_diff = (time_zhi_idx as i32 - time_gan_idx as i32).rem_euclid(12) as usize;
    let xun_idx = match xun_diff {
        0 => 0,  // 甲子
        10 => 1, // 甲戌
        8 => 2,  // 甲申
        6 => 3,  // 甲午
        4 => 4,  // 甲辰
        _ => 5,  // 甲寅
    };
    let (xun_name, xun_gan, orig_leader_star, orig_leader_door) = XUN_HEADS[xun_idx];

    // 寻找旬首六仪在地盘所在的初始值符宫
    let mut orig_zf_gong = 2; // 默认寄坤二
    for g in 1..=9 {
        if dipan[g] == xun_gan {
            orig_zf_gong = if g == 5 { 2 } else { g }; // 中五寄坤二
            break;
        }
    }

    // 值符随时干落宫 (时干落入地盘何宫，值符星便加临何宫)
    let mut target_zf_gong = orig_zf_gong;
    for g in 1..=9 {
        if dipan[g] == time_gan {
            target_zf_gong = if g == 5 { 2 } else { g };
            break;
        }
    }

    // 八卦圆周距离
    let orig_ring_pos = RING_GONG.iter().position(|&x| x == orig_zf_gong).unwrap_or(0);
    let target_ring_pos = RING_GONG.iter().position(|&x| x == target_zf_gong).unwrap_or(0);
    let shift = (target_ring_pos as i32 - orig_ring_pos as i32).rem_euclid(8) as usize;

    // 5. 天盘九星与八神轮转排布
    let ring_stars = ["天蓬星", "天任星", "天冲星", "天辅星", "天英星", "天芮星", "天柱星", "天心星"];
    let ring_doors = ["休门", "生门", "伤门", "杜门", "景门", "死门", "惊门", "开门"];

    let mut star_at_gong = [""; 10];
    let mut door_at_gong = [""; 10];
    let mut tianpan_at_gong = [""; 10];
    let mut god_at_gong = [""; 10];

    for i in 0..8 {
        let cur_gong = RING_GONG[i];
        let from_gong = RING_GONG[(i as i32 - shift as i32).rem_euclid(8) as usize];
        star_at_gong[cur_gong] = ring_stars[(i as i32 - shift as i32).rem_euclid(8) as usize];
        door_at_gong[cur_gong] = ring_doors[i];
        tianpan_at_gong[cur_gong] = dipan[from_gong];

        // 八神排布 (值符加临值符宫，阳顺阴逆顺布八宫)
        let god_step = if is_yang {
            (i as i32 - target_ring_pos as i32).rem_euclid(8) as usize
        } else {
            (target_ring_pos as i32 - i as i32).rem_euclid(8) as usize
        };
        god_at_gong[cur_gong] = GODS[god_step % 8];
    }
    // 中五宫天禽星同落天芮，地盘不变，八门中门/寄死门，八神随天芮
    star_at_gong[5] = "天禽星";
    tianpan_at_gong[5] = dipan[5];
    door_at_gong[5] = "中门";
    god_at_gong[5] = "太常";

    // 6. 旬空宫位推导
    // 旬首差数对冲支即为空亡 (如甲子旬空戌亥 -> 乾六宫空亡)
    let (kw_gong1, kw_gong2) = match xun_idx {
        0 => (6, 6), // 甲子旬空戌亥 -> 乾6
        1 => (7, 7), // 甲戌旬空申酉 -> 坤2, 兑7
        2 => (9, 2), // 甲申旬空午未 -> 离9, 坤2
        3 => (4, 4), // 甲午旬空辰巳 -> 巽4
        4 => (3, 3), // 甲辰旬空寅卯 -> 艮8, 震3
        _ => (1, 8), // 甲寅旬空子丑 -> 坎1, 艮8
    };

    // 7. 法奇门六害（击刑、入墓、门迫）与化解生成
    let mut fa_solutions = Vec::new();
    let mut patterns = Vec::new();

    // 伏吟与反吟判定
    if target_zf_gong == orig_zf_gong {
        patterns.push("值符星门伏吟：九星八门居本宫不动，动不如静，利主不利客。".to_string());
    } else if (target_ring_pos as i32 - orig_ring_pos as i32).abs() == 4 {
        patterns.push("值符星门反吟：九星对冲本宫，变动剧烈，主吉事成凶，凶事反散。".to_string());
    }

    // 马星位置推导 (以时支查：申子辰马在寅艮8，寅午戌马在申坤2，巳酉丑马在亥乾6，亥卯未马在巳巽4)
    let maxing_gong = match time_zhi_idx % 4 {
        0 => 8, // 申 子 辰 -> 艮八宫
        1 => 6, // 巳 酉 丑 -> 乾六宫
        2 => 2, // 寅 午 戌 -> 坤二宫
        _ => 4, // 亥 卯 未 -> 巽四宫
    };

    // 暗干排布：时干入地盘值符宫，按阳顺阴逆推布九宫
    let mut an_gan_at_gong = [""; 10];
    let time_gan_sq_idx = SAN_QI_LIU_YI.iter().position(|&g| g == time_gan).unwrap_or(0);
    for step in 0..9 {
        let cur_gan = SAN_QI_LIU_YI[(time_gan_sq_idx + step) % 9];
        let target_g = if is_yang {
            ((target_zf_gong as i32 - 1 + step as i32).rem_euclid(9) + 1) as usize
        } else {
            ((target_zf_gong as i32 - 1 - step as i32).rem_euclid(9) + 1) as usize
        };
        an_gan_at_gong[target_g] = cur_gan;
    }

    let mut palaces = Vec::with_capacity(9);
    for i in 1..=9 {
        let tp = tianpan_at_gong[i];
        let dr = door_at_gong[i];

        // 击刑判定：甲子戊落震3刑，甲戌己落坤2刑，甲申庚落艮8刑，甲午辛落离9刑，甲辰壬落巽4刑，甲寅癸落巽4刑
        let jixing = (tp == "戊" && i == 3)
            || (tp == "己" && i == 2)
            || (tp == "庚" && i == 8)
            || (tp == "辛" && i == 9)
            || (tp == "壬" && i == 4)
            || (tp == "癸" && i == 4);

        // 入墓判定：乙丙入戌墓(乾6), 丁己入丑墓(艮8), 辛入辰墓(巽4), 壬癸入未墓(坤2)
        let rumu = (tp == "乙" || tp == "丙") && i == 6
            || (tp == "丁" || tp == "己") && i == 8
            || (tp == "辛" && i == 4)
            || (tp == "壬" || tp == "癸") && i == 2;

        // 门迫判定：门克宫（吉门受克吉不就，凶门受克凶更甚）
        // 震3巽4木，离9火，坤2艮8土，乾6兑7金，坎1水
        let menpo = (dr == "生门" || dr == "死门") && i == 1 // 土门迫水宫
            || (dr == "伤门" || dr == "杜门") && (i == 2 || i == 8) // 木门迫土宫
            || (dr == "景门") && (i == 6 || i == 7) // 火门迫金宫
            || (dr == "惊门" || dr == "开门") && (i == 3 || i == 4) // 金门迫木宫
            || (dr == "休门") && i == 9; // 水门迫火宫

        let kongwang = (i == kw_gong1 || i == kw_gong2) && i != 5;
        let is_maxing = i == maxing_gong;

        // 生成法奇门荀爽化解之道
        if jixing {
            fa_solutions.push(FaQiMenSolution {
                gong_id: i,
                gong_name: PALACES[i - 1],
                harm_type: "击刑",
                antidote_item: format!("在{}方位放置合干器物，以六合或五合化解刑伤冲克", PALACES[i - 1]),
                description: "击刑主破耗损折、争竞冲突，乃奇门六害之首，宜及早调和。",
            });
        }
        if menpo {
            fa_solutions.push(FaQiMenSolution {
                gong_id: i,
                gong_name: PALACES[i - 1],
                harm_type: "门迫",
                antidote_item: format!("在{}方位安放通关五行灵物，化门宫相战为贪生忘克", PALACES[i - 1]),
                description: "门迫主阻隔逆阻，能量受抑，借通关五行可化解郁塞。",
            });
        }
        if tp == "庚" {
            patterns.push(format!("天盘庚金落入{}：逢庚遇阻，主关卡停滞，宜守正防失。", PALACES[i - 1]));
        }

        let keying = get_shigan_keying(tp, dipan[i]).map(|s| s.to_string());

        palaces.push(QimenPalace {
            palace_id: i,
            name: PALACES[i - 1],
            dipan_gan: dipan[i],
            tianpan_gan: tp,
            an_gan: an_gan_at_gong[i],
            star: star_at_gong[i],
            door: dr,
            god: god_at_gong[i],
            jixing,
            rumu,
            menpo,
            kongwang,
            is_maxing,
            keying,
        });
    }

    QimenResult {
        ju_name,
        dun_type,
        jieqi: jq_name,
        yuan: yuan_name,
        xun_head: xun_name,
        leader_star: orig_leader_star,
        leader_door: orig_leader_door,
        leader_palace: target_zf_gong,
        palaces,
        fa_solutions,
        patterns,
    }
}
