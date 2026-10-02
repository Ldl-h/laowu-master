// 易经六爻与梅花心易起卦推演

pub const BAGUA: [&str; 8] = ["乾", "兑", "离", "震", "巽", "坎", "艮", "坤"];

pub fn get_gua_name_by_up_lo(up: &str, lo: &str) -> &'static str {
    let u = BAGUA.iter().position(|&x| x == up).unwrap_or(0);
    let l = BAGUA.iter().position(|&x| x == lo).unwrap_or(0);
    GUA64[u][l]
}

// 六十四卦卦名表 (8x8 二维矩阵: 上卦 x 下卦)
pub const GUA64: [[&str; 8]; 8] = [
    ["乾为天", "天泽履", "天火同人", "天雷无妄", "天风姤", "天水讼", "天山遁", "天地否"],
    ["泽天夬", "兑为泽", "泽火革", "泽雷随", "泽风大过", "泽水困", "泽山咸", "泽地萃"],
    ["火天大有", "火泽睽", "离为火", "火雷噬嗑", "火风鼎", "火水未济", "火山旅", "火地晋"],
    ["雷天大壮", "雷泽归妹", "雷火丰", "震为雷", "雷风恒", "雷水解", "雷山小过", "雷地豫"],
    ["风天小畜", "风泽中孚", "风火家人", "风雷益", "巽为风", "风水涣", "风山渐", "风地观"],
    ["水天需", "水泽节", "水火既济", "水雷屯", "水风井", "坎为水", "水山蹇", "水地比"],
    ["山天大畜", "山泽损", "山火贲", "山雷颐", "山风蛊", "山水蒙", "艮为山", "山地剥"],
    ["地天泰", "地泽临", "地火明夷", "地雷复", "地风升", "地水师", "地山谦", "坤为地"],
];

// 八卦先天二进制 (上爻, 中爻, 下爻: 阳爻1, 阴爻0)
// 乾: 111(7), 兑: 011(3), 离: 101(5), 震: 001(1), 巽: 110(6), 坎: 010(2), 艮: 100(4), 坤: 000(0)
pub const BAGUA_LINES: [u8; 8] = [7, 3, 5, 1, 6, 2, 4, 0];

// 京房八宫五行
pub const GONG_WUXING: [&str; 8] = ["金", "金", "火", "木", "木", "水", "土", "土"];

// 八纯卦纳甲天干与地支
// 乾(0): 内甲子甲寅甲辰, 外壬午壬申壬戌
// 兑(1): 内丁巳丁卯丁丑, 外丁亥丁酉丁未
// 离(2): 内己卯己丑己亥, 外己酉己未己巳
// 震(3): 内庚子庚寅庚辰, 外庚午庚申庚戌
// 巽(4): 内辛丑辛亥辛酉, 外辛未辛巳辛卯
// 坎(5): 内戊寅戊辰戊午, 外戊申戊戌戊子
// 艮(6): 内丙辰丙午丙申, 外丙戌丙子丙寅
// 坤(7): 内乙未乙巳乙卯, 外癸丑癸亥癸酉
pub const NAJIA_TABLE: [[(&str, &str); 6]; 8] = [
    // 乾 (0)
    [("甲", "子"), ("甲", "寅"), ("甲", "辰"), ("壬", "午"), ("壬", "申"), ("壬", "戌")],
    // 兑 (1)
    [("丁", "巳"), ("丁", "卯"), ("丁", "丑"), ("丁", "亥"), ("丁", "酉"), ("丁", "未")],
    // 离 (2)
    [("己", "卯"), ("己", "丑"), ("己", "亥"), ("己", "酉"), ("己", "未"), ("己", "巳")],
    // 震 (3)
    [("庚", "子"), ("庚", "寅"), ("庚", "辰"), ("庚", "午"), ("庚", "申"), ("庚", "戌")],
    // 巽 (4)
    [("辛", "丑"), ("辛", "亥"), ("辛", "酉"), ("辛", "未"), ("辛", "巳"), ("辛", "卯")],
    // 坎 (5)
    [("戊", "寅"), ("戊", "辰"), ("戊", "午"), ("戊", "申"), ("戊", "戌"), ("戊", "子")],
    // 艮 (6)
    [("丙", "辰"), ("丙", "午"), ("丙", "申"), ("丙", "戌"), ("丙", "子"), ("丙", "寅")],
    // 坤 (7)
    [("乙", "未"), ("乙", "巳"), ("乙", "卯"), ("癸", "丑"), ("癸", "亥"), ("癸", "酉")],
];

pub fn get_zhi_wuxing(zhi: &str) -> &'static str {
    match zhi {
        "寅" | "卯" => "木",
        "巳" | "午" => "火",
        "申" | "酉" => "金",
        "亥" | "子" => "水",
        _ => "土", // 辰 戌 丑 未
    }
}

pub fn get_liuqin(gong_wuxing: &str, zhi: &str) -> &'static str {
    let z_wx = get_zhi_wuxing(zhi);
    let wx_idx = |w: &str| match w {
        "木" => 0,
        "火" => 1,
        "土" => 2,
        "金" => 3,
        _ => 4, // 水
    };
    let me = wx_idx(gong_wuxing);
    let other = wx_idx(z_wx);
    let diff = (other + 5 - me) % 5;
    match diff {
        0 => "兄弟",
        1 => "子孙",
        2 => "妻财",
        3 => "官鬼",
        _ => "父母",
    }
}

// 六神 (青龙, 朱雀, 勾陈, 螣蛇, 白虎, 玄武)
pub const LIU_SHEN: [&str; 6] = ["青龙", "朱雀", "勾陈", "螣蛇", "白虎", "玄武"];

#[derive(Debug, Clone, serde::Serialize)]
pub struct YaoDetail {
    pub yao_index: usize,          // 1~6 爻 (初爻到上爻)
    pub yin_yang: &'static str,    // 阳爻 / 阴爻
    pub is_moving: bool,           // 是否动爻
    pub is_shi: bool,              // 是否世爻
    pub is_ying: bool,             // 是否应爻
    pub liu_shen: &'static str,    // 六神神煞
    pub na_jia: &'static str,      // 纳甲天干
    pub na_zhi: &'static str,      // 纳支地支
    pub liu_qin: &'static str,     // 本爻六亲
    pub bian_na_jia: Option<&'static str>, // 变爻纳甲
    pub bian_na_zhi: Option<&'static str>, // 变爻纳支
    pub bian_liu_qin: Option<&'static str>, // 变爻六亲
}

#[derive(Debug, serde::Serialize)]
pub struct SixYaoResult {
    pub shang_gua: &'static str,    // 上卦
    pub xia_gua: &'static str,      // 下卦
    pub original_gua: &'static str, // 本卦全称
    pub gong_name: &'static str,    // 京房本宫归属 (八宫卦)
    pub gong_wuxing: &'static str,  // 卦宫五行
    pub shi_yao: usize,             // 世爻位置 (1~6)
    pub ying_yao: usize,            // 应爻位置 (1~6)
    pub moving_yao: usize,          // 动爻 (1~6 爻)
    pub bian_gua: &'static str,     // 变卦全称
    pub yaos: Vec<YaoDetail>,       // 六爻精细全息切片
}

/// 纯数学按数起卦 / 时间起卦 (邵康节梅花心易法 + 京房纳甲世应与六亲变爻)
pub fn calculate_liuyao(num1: usize, num2: usize, num3: usize) -> SixYaoResult {
    // 1. 上卦 = num1 % 8 (0为坤8)
    let shang_idx = (num1 % 8 + 7) % 8;
    // 2. 下卦 = num2 % 8
    let xia_idx = (num2 % 8 + 7) % 8;
    // 3. 动爻 = (num1 + num2 + num3) % 6 (0为上爻6)
    let moving_yao = match (num1 + num2 + num3) % 6 {
        0 => 6,
        n => n,
    };

    let original_gua = GUA64[shang_idx][xia_idx];

    // 本卦六爻二进制序列 (从下爻0到上爻5)
    let xia_bin = BAGUA_LINES[xia_idx];
    let shang_bin = BAGUA_LINES[shang_idx];
    let total_bin = (shang_bin << 3) | xia_bin; // 6位整数

    // 京房八宫寻世诀推导 (天同二世天变五，地同四世地变初，本宫六世三世异，人同游魂人变归)
    let diff = shang_bin ^ xia_bin;
    let (shi_yao, gong_idx) = match diff {
        0 => (6, shang_idx), // 本宫卦，世在六
        1 => (1, shang_idx), // 初爻变，一世卦
        3 => (2, shang_idx), // 一二爻变，二世卦
        7 => (3, shang_idx), // 一二三爻变，三世卦
        6 => (4, (shang_idx ^ 7) % 8), // 四爻变，四世卦
        4 => (5, (shang_idx ^ 7) % 8), // 五爻变，五世卦
        5 => (4, (shang_idx ^ 7) % 8), // 游魂卦，世在四
        2 => (3, shang_idx),           // 归魂卦，世在三
        _ => (6, shang_idx),
    };
    let ying_yao = if shi_yao <= 3 { shi_yao + 3 } else { shi_yao - 3 };

    // 变卦推导 (单爻动变: 对应二进制位翻转)
    let moving_bit = 1 << (moving_yao - 1);
    let bian_total_bin = total_bin ^ moving_bit;
    let bian_xia_bin = bian_total_bin & 7;
    let bian_shang_bin = (bian_total_bin >> 3) & 7;

    let bian_xia_idx = BAGUA_LINES.iter().position(|&b| b == bian_xia_bin).unwrap_or(0);
    let bian_shang_idx = BAGUA_LINES.iter().position(|&b| b == bian_shang_bin).unwrap_or(0);
    let bian_gua = GUA64[bian_shang_idx][bian_xia_idx];

    let gong_wuxing = GONG_WUXING[gong_idx];

    // 构建六爻精细切片 (纳甲 + 纳支 + 六亲 + 六神 + 变爻)
    let mut yaos = Vec::with_capacity(6);
    for i in 1..=6 {
        let bit = (total_bin >> (i - 1)) & 1;
        let is_moving = i == moving_yao;

        // 本卦纳甲纳支
        let (na_jia, na_zhi) = if i <= 3 {
            NAJIA_TABLE[xia_idx][i - 1]
        } else {
            NAJIA_TABLE[shang_idx][i - 1]
        };
        let liu_qin = get_liuqin(gong_wuxing, na_zhi);

        // 动爻产生变卦纳甲
        let (bian_na_jia, bian_na_zhi, bian_liu_qin) = if is_moving {
            let (bg_g, bg_z) = if i <= 3 {
                NAJIA_TABLE[bian_xia_idx][i - 1]
            } else {
                NAJIA_TABLE[bian_shang_idx][i - 1]
            };
            (Some(bg_g), Some(bg_z), Some(get_liuqin(gong_wuxing, bg_z)))
        } else {
            (None, None, None)
        };

        yaos.push(YaoDetail {
            yao_index: i,
            yin_yang: if bit == 1 { "阳爻 (⚊)" } else { "阴爻 (⚋)" },
            is_moving,
            is_shi: i == shi_yao,
            is_ying: i == ying_yao,
            liu_shen: LIU_SHEN[i - 1],
            na_jia,
            na_zhi,
            liu_qin,
            bian_na_jia,
            bian_na_zhi,
            bian_liu_qin,
        });
    }

    SixYaoResult {
        shang_gua: BAGUA[shang_idx],
        xia_gua: BAGUA[xia_idx],
        original_gua,
        gong_name: BAGUA[gong_idx],
        gong_wuxing,
        shi_yao,
        ying_yao,
        moving_yao,
        bian_gua,
        yaos,
    }
}

// 64 卦与八宫元数据
pub const GUA64_DATA: [(u8, &str, [u8; 6], &str); 64] = [
    (63, "乾为天", [1, 1, 1, 1, 1, 1], "乾"),
    (62, "天风姤", [0, 1, 1, 1, 1, 1], "乾"),
    (60, "天山遁", [0, 0, 1, 1, 1, 1], "乾"),
    (56, "天地否", [0, 0, 0, 1, 1, 1], "乾"),
    (48, "风地观", [0, 0, 0, 0, 1, 1], "乾"),
    (32, "山地剥", [0, 0, 0, 0, 0, 1], "乾"),
    (40, "火地晋", [0, 0, 0, 1, 0, 1], "乾"),
    (47, "火天大有", [1, 1, 1, 1, 0, 1], "乾"),
    (18, "坎为水", [0, 1, 0, 0, 1, 0], "坎"),
    (19, "水泽节", [1, 1, 0, 0, 1, 0], "坎"),
    (17, "水雷屯", [1, 0, 0, 0, 1, 0], "坎"),
    (21, "水火既济", [1, 0, 1, 0, 1, 0], "坎"),
    (29, "泽火革", [1, 0, 1, 1, 1, 0], "坎"),
    (13, "雷火丰", [1, 0, 1, 1, 0, 0], "坎"),
    (5, "地火明夷", [1, 0, 1, 0, 0, 0], "坎"),
    (2, "地水师", [0, 1, 0, 0, 0, 0], "坎"),
    (36, "艮为山", [0, 0, 1, 0, 0, 1], "艮"),
    (37, "山火贲", [1, 0, 1, 0, 0, 1], "艮"),
    (39, "山天大畜", [1, 1, 1, 0, 0, 1], "艮"),
    (35, "山泽损", [1, 1, 0, 0, 0, 1], "艮"),
    (43, "火泽睽", [1, 1, 0, 1, 0, 1], "艮"),
    (59, "天泽履", [1, 1, 0, 1, 1, 1], "艮"),
    (51, "风泽中孚", [1, 1, 0, 0, 1, 1], "艮"),
    (52, "风山渐", [0, 0, 1, 0, 1, 1], "艮"),
    (9, "震为雷", [1, 0, 0, 1, 0, 0], "震"),
    (8, "雷地豫", [0, 0, 0, 1, 0, 0], "震"),
    (10, "雷水解", [0, 1, 0, 1, 0, 0], "震"),
    (14, "雷风恒", [0, 1, 1, 1, 0, 0], "震"),
    (6, "地风升", [0, 1, 1, 0, 0, 0], "震"),
    (22, "水风井", [0, 1, 1, 0, 1, 0], "震"),
    (30, "泽风大过", [0, 1, 1, 1, 1, 0], "震"),
    (25, "泽雷随", [1, 0, 0, 1, 1, 0], "震"),
    (54, "巽为风", [0, 1, 1, 0, 1, 1], "巽"),
    (55, "风天小畜", [1, 1, 1, 0, 1, 1], "巽"),
    (53, "风火家人", [1, 0, 1, 0, 1, 1], "巽"),
    (49, "风雷益", [1, 0, 0, 0, 1, 1], "巽"),
    (57, "天雷无妄", [1, 0, 0, 1, 1, 1], "巽"),
    (41, "火雷噬嗑", [1, 0, 0, 1, 0, 1], "巽"),
    (33, "山雷颐", [1, 0, 0, 0, 0, 1], "巽"),
    (38, "山风蛊", [0, 1, 1, 0, 0, 1], "巽"),
    (45, "离为火", [1, 0, 1, 1, 0, 1], "离"),
    (44, "火山旅", [0, 0, 1, 1, 0, 1], "离"),
    (46, "火风鼎", [0, 1, 1, 1, 0, 1], "离"),
    (42, "火水未济", [0, 1, 0, 1, 0, 1], "离"),
    (34, "山水蒙", [0, 1, 0, 0, 0, 1], "离"),
    (50, "风水涣", [0, 1, 0, 0, 1, 1], "离"),
    (58, "天水讼", [0, 1, 0, 1, 1, 1], "离"),
    (61, "天火同人", [1, 0, 1, 1, 1, 1], "离"),
    (0, "坤为地", [0, 0, 0, 0, 0, 0], "坤"),
    (1, "地雷复", [1, 0, 0, 0, 0, 0], "坤"),
    (3, "地泽临", [1, 1, 0, 0, 0, 0], "坤"),
    (7, "地天泰", [1, 1, 1, 0, 0, 0], "坤"),
    (15, "雷天大壮", [1, 1, 1, 1, 0, 0], "坤"),
    (31, "泽天夬", [1, 1, 1, 1, 1, 0], "坤"),
    (23, "水天需", [1, 1, 1, 0, 1, 0], "坤"),
    (16, "水地比", [0, 0, 0, 0, 1, 0], "坤"),
    (27, "兑为泽", [1, 1, 0, 1, 1, 0], "兑"),
    (26, "泽水困", [0, 1, 0, 1, 1, 0], "兑"),
    (24, "泽地萃", [0, 0, 0, 1, 1, 0], "兑"),
    (28, "泽山咸", [0, 0, 1, 1, 1, 0], "兑"),
    (20, "水山蹇", [0, 0, 1, 0, 1, 0], "兑"),
    (4, "地山谦", [0, 0, 1, 0, 0, 0], "兑"),
    (12, "雷山小过", [0, 0, 1, 1, 0, 0], "兑"),
    (11, "雷泽归妹", [1, 1, 0, 1, 0, 0], "兑"),
];

/// 逐爻装卦物理推演 (输入 6 个爻值 [初爻..上爻]，0 为阴，1 为阳；以及变动标识)
pub fn calculate_liuyao_lines(lines: &[u8], moving: &[bool]) -> serde_json::Value {
    let mut key = 0u8;
    for (i, &v) in lines.iter().enumerate().take(6) {
        if v == 1 {
            key |= 1 << i;
        }
    }

    let item = GUA64_DATA.iter().find(|(k, _, _, _)| *k == key).unwrap_or(&GUA64_DATA[0]);
    let name = item.1;
    let house = item.3;

    // 互卦 (2,3,4爻为下卦，3,4,5爻为上卦)
    let hu_lines = [lines[1], lines[2], lines[3], lines[2], lines[3], lines[4]];
    let mut hu_key = 0u8;
    for (i, &v) in hu_lines.iter().enumerate() {
        if v == 1 { hu_key |= 1 << i; }
    }
    let hu_name = GUA64_DATA.iter().find(|(k, _, _, _)| *k == hu_key).map(|x| x.1).unwrap_or("未定");

    // 变卦
    let mut bian_key = 0u8;
    for i in 0..6 {
        let is_mov = moving.get(i).copied().unwrap_or(false);
        let orig = lines.get(i).copied().unwrap_or(0);
        let b = if is_mov { 1 - orig } else { orig };
        if b == 1 { bian_key |= 1 << i; }
    }
    let bian_name = GUA64_DATA.iter().find(|(k, _, _, _)| *k == bian_key).map(|x| x.1).unwrap_or("无变卦");

    // 错卦 (阴阳全反)
    let cuo_key = (!key) & 0x3F;
    let cuo_name = GUA64_DATA.iter().find(|(k, _, _, _)| *k == cuo_key).map(|x| x.1).unwrap_or("未定");

    // 综卦 (上下颠倒)
    let mut zong_key = 0u8;
    for i in 0..6 {
        if ((key >> (5 - i)) & 1) == 1 {
            zong_key |= 1 << i;
        }
    }
    let zong_name = GUA64_DATA.iter().find(|(k, _, _, _)| *k == zong_key).map(|x| x.1).unwrap_or("未定");

    // 计算世爻与应爻
    let xia_3 = key & 7;
    let shang_3 = (key >> 3) & 7;
    let diff = shang_3 ^ xia_3;
    let shi_yao = match diff {
        0 => 6,
        1 => 1,
        3 => 2,
        7 => 3,
        6 => 4,
        4 => 5,
        5 => 4,
        2 => 3,
        _ => 6,
    };
    let ying_yao = if shi_yao <= 3 { shi_yao + 3 } else { shi_yao - 3 };

    let mut yaos_json = Vec::new();
    for i in 0..6 {
        let v = lines.get(i).copied().unwrap_or(0);
        let m = moving.get(i).copied().unwrap_or(false);
        yaos_json.push(serde_json::json!({
            "index": i + 1,
            "value": v,
            "change": m,
            "yin_yang": if v == 1 { "阳爻" } else { "阴爻" },
            "is_shi": i + 1 == shi_yao,
            "is_ying": i + 1 == ying_yao,
        }));
    }

    serde_json::json!({
        "current_gua": {
            "name": name,
            "house": house,
            "key": key
        },
        "hu_gua": hu_name,
        "bian_gua": bian_name,
        "cuo_gua": cuo_name,
        "zong_gua": zong_name,
        "shi_yao": shi_yao,
        "ying_yao": ying_yao,
        "yaos": yaos_json,
        "summary": format!("六爻装卦: 本卦【{}】(属{}宫) 世在{}爻 应在{}爻，互卦【{}】，错卦【{}】，综卦【{}】", name, house, shi_yao, ying_yao, hu_name, cuo_name, zong_name)
    })
}
