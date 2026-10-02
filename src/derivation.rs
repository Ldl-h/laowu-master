// 易学与黄历衍生技法统摄引擎 (Derivations & Huangli System Engine)
// 包含：霍斐然小成图 (Xiao Cheng Tu) 真正九宫飞布与正推旁推、老黄历建除十二神 (Huangli Jianchu 12)

#[derive(Debug, Clone, serde::Serialize)]
pub struct XiaoChengTuResult {
    pub main_gua: String,
    pub ben_gua: String,
    pub zhi_gua: String,
    pub gong_palaces: Vec<(&'static str, String)>, // (九宫位, 触落卦)
    pub summary: String,
}

// 八卦三爻定义 (下爻、中爻、上爻，true=阳, false=阴)
fn get_gua_lines(name: &str) -> [bool; 3] {
    match name {
        "乾" => [true, true, true],
        "兑" => [true, true, false],
        "离" => [true, false, true],
        "震" => [true, false, false],
        "巽" => [false, true, true],
        "坎" => [false, true, false],
        "艮" => [false, false, true],
        _ => [false, false, false], // 坤
    }
}

fn lines_to_gua(lines: [bool; 3]) -> &'static str {
    match lines {
        [true, true, true] => "乾",
        [true, true, false] => "兑",
        [true, false, true] => "离",
        [true, false, false] => "震",
        [false, true, true] => "巽",
        [false, true, false] => "坎",
        [false, false, true] => "艮",
        _ => "坤",
    }
}

/// 霍斐然小成图正宗九宫天盘排布 (依《小成图秘旨》正典：
/// 9宫落本卦上, 1宫落本卦下, 3宫落之卦上, 7宫落之卦下;
/// 4宫落本卦上互(爻345), 2宫落本卦下互(爻234);
/// 8宫落之卦上互, 6宫落之卦下互; 5宫中宫无卦)
pub fn calculate_xiaochengtu_exact(
    up_gua: &str,
    lo_gua: &str,
    dong_yaos: &[usize],
) -> XiaoChengTuResult {
    let b_up = get_gua_lines(up_gua);
    let b_lo = get_gua_lines(lo_gua);

    // 6爻合成 (0..2为下卦, 3..5为上卦)
    let hex_lines = [b_lo[0], b_lo[1], b_lo[2], b_up[0], b_up[1], b_up[2]];

    // 求变卦 (之卦)
    let mut zhi_lines = hex_lines;
    for &d in dong_yaos {
        if (1..=6).contains(&d) {
            zhi_lines[d - 1] = !zhi_lines[d - 1];
        }
    }
    // 若无动爻，按正典“履之履”同卦处理
    let z_lo = [zhi_lines[0], zhi_lines[1], zhi_lines[2]];
    let z_up = [zhi_lines[3], zhi_lines[4], zhi_lines[5]];

    let ben_shang_hu = lines_to_gua([hex_lines[2], hex_lines[3], hex_lines[4]]);
    let ben_xia_hu = lines_to_gua([hex_lines[1], hex_lines[2], hex_lines[3]]);

    let zhi_shang_hu = lines_to_gua([zhi_lines[2], zhi_lines[3], zhi_lines[4]]);
    let zhi_xia_hu = lines_to_gua([zhi_lines[1], zhi_lines[2], zhi_lines[3]]);

    let z_up_name = lines_to_gua(z_up);
    let z_lo_name = lines_to_gua(z_lo);

    // 依 BU_JU_SLOTS 规则构建九宫飞布天盘落卦:
    let palaces = vec![
        ("坎一宫", lo_gua.to_string()),
        ("坤二宫", ben_xia_hu.to_string()),
        ("震三宫", z_up_name.to_string()),
        ("巽四宫", ben_shang_hu.to_string()),
        ("中五宫", "中".to_string()),
        ("乾六宫", zhi_xia_hu.to_string()),
        ("兑七宫", z_lo_name.to_string()),
        ("艮八宫", zhi_shang_hu.to_string()),
        ("离九宫", up_gua.to_string()),
    ];

    let ben_full = crate::liuyao::get_gua_name_by_up_lo(up_gua, lo_gua);
    let zhi_full = crate::liuyao::get_gua_name_by_up_lo(z_up_name, z_lo_name);

    XiaoChengTuResult {
        main_gua: format!("{}之{}", ben_full, zhi_full),
        ben_gua: ben_full.to_string(),
        zhi_gua: zhi_full.to_string(),
        gong_palaces: palaces,
        summary: format!("霍斐然小成图：天地阖辟，四正四维九宫天盘真卦飞布成局 (本卦【{}】，之卦【{}】)", ben_full, zhi_full),
    }
}

pub fn calculate_xiaochengtu(up_gua: &str, lo_gua: &str) -> XiaoChengTuResult {
    calculate_xiaochengtu_exact(up_gua, lo_gua, &[1])
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HuangliResult {
    pub day_zhi: &'static str,
    pub jian_chu_god: &'static str,    // 建、除、满、平、定、执、破、危、成、收、开、闭
    pub jixiong: &'static str,
    pub yi: &'static str,               // 宜
    pub ji: &'static str,               // 忌
}

pub const JIAN_CHU_12: [&str; 12] = ["建", "除", "满", "平", "定", "执", "破", "危", "成", "收", "开", "闭"];

/// 老黄历建除十二神推算
pub fn calculate_huangli(month_zhi_idx: usize, day_zhi_idx: usize) -> HuangliResult {
    let step = (day_zhi_idx + 12 - month_zhi_idx) % 12;
    let god = JIAN_CHU_12[step];

    let (jx, yi, ji) = match god {
        "建" => ("吉", "出行、纳财、祈福", "动土、开仓、破土"),
        "除" => ("吉", "求医、扫舍、除服", "纳财、嫁娶、开市"),
        "满" => ("次吉", "祈福、开市、纳财", "治病、服药、动土"),
        "平" => ("平", "修饰垣墙、平治道涂", "祈福、嫁娶、动土"),
        "定" => ("吉", "订盟、纳采、会友", "词讼、出征、开市"),
        "执" => ("次吉", "立券、交易、纳财", "出行、搬迁、开渠"),
        "破" => ("凶", "破屋坏垣、求医破贼", "一切吉事皆忌"),
        "危" => ("平", "安床、修造、出行", "乘船、涉水、登高"),
        "成" => ("吉", "嫁娶、开市、成婚、入学", "词讼、开仓"),
        "收" => ("次吉", "进人口、收麦、捕捉", "开业、出军、破土"),
        "开" => ("大吉", "开市、纳财、出货、修造", "安葬、伐木、破土"),
        _ => ("凶", "安葬、填塞、筑堤", "开张、出行、嫁娶"), // 闭
    };

    let zhi_names = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

    HuangliResult {
        day_zhi: zhi_names[day_zhi_idx % 12],
        jian_chu_god: god,
        jixiong: jx,
        yi,
        ji,
    }
}
