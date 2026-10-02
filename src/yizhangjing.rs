// 达摩一掌经排盘与四柱神煞、十二宫推演引擎 (Yi Zhang Jing Engine)
// 纯 Rust 高精度零依赖实现，基于子丑寅卯十二宫掐指起课

pub const BRANCHES: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

#[derive(Debug, Clone, serde::Serialize)]
pub struct YizhangjingStar {
    pub branch: &'static str,
    pub star: &'static str,       // 佛历星宿: 佛道天贵星、鬼道天厄星...
    pub dao: &'static str,        // 六道轮回 (佛道、仙道、人道、阿修罗道、地狱道、饿鬼道、畜生道)
    pub grade: &'static str,      // 品级 (上品 / 中品 / 下品)
    pub poem: &'static str,       // 断诗
}

pub const STARS: [(&str, &str, &str, &str); 12] = [
    // (地支, 星名, 六道, 品级)
    ("子", "天貴", "佛道", "上品"),
    ("丑", "天厄", "鬼道", "下品"),
    ("寅", "天權", "人道", "中品"),
    ("卯", "天破", "畜道", "下品"),
    ("辰", "天奸", "修罗", "下品"),
    ("巳", "天文", "仙道", "上品"),
    ("午", "天福", "佛道", "上品"),
    ("未", "天驛", "鬼道", "中品"),
    ("申", "天孤", "人道", "下品"),
    ("酉", "天刃", "畜道", "下品"),
    ("戌", "天藝", "修罗", "中品"),
    ("亥", "天壽", "仙道", "上品"),
];

pub fn get_star_info(branch_idx: usize) -> YizhangjingStar {
    let (b, s, d, g) = STARS[branch_idx % 12];
    let poem = match s {
        "天貴" => "志气不凡象貌清，安然享受度平生。初年若是财帛耗，富贵荣华在晚成。",
        "天厄" => "时逢天厄带灾殃，多病多愁体弱防。若得吉星来拱照，逢凶化吉保平康。",
        "天權" => "天权星照命不差，操持权柄掌生杀。为人果决多聪敏，自立成家名誉佳。",
        "天破" => "天破为人多破财，空怀志气命乖蹇。若能勤俭依规矩，免至中途困惑哀。",
        "天奸" => "天奸之宿善机谋，多智多能谋不休。操心劳碌奔波走，晚岁安闲乐自悠。",
        "天文" => "天文星照大文章，锦绣胸怀品自昂。学者成名登仕版，平生显达姓名扬。",
        "天福" => "命逢天福福非常，坐享清安寿算长。出入贵人多照应，子孙满堂庆吉昌。",
        "天驛" => "天驿之星命多奔，离乡背井走风尘。求财到处皆通达，四海为家四海春。",
        "天孤" => "天孤之宿苦伶仃，男克妻兮女克夫。若不修行常作善，晚景萧条谁与顾。",
        "天刃" => "天刃为人性气刚，动如雷电发如霜。若能修省柔和道，免被刑伤横祸防。",
        "天藝" => "天艺之星百巧通，琴棋书画显奇工。聪明智慧无双比，到处随安显智功。",
        _ => "天寿之星寿算长，安闲度日免悲伤。生来衣禄自然足，福寿双全百世昌。",
    };
    YizhangjingStar {
        branch: b,
        star: s,
        dao: d,
        grade: g,
        poem,
    }
}

pub fn branch_index(b: &str) -> usize {
    match b {
        "子" => 0, "丑" => 1, "寅" => 2, "卯" => 3, "辰" => 4, "巳" => 5,
        "午" => 6, "未" => 7, "申" => 8, "酉" => 9, "戌" => 10, "亥" => 11,
        _ => 0,
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct YizhangjingResult {
    pub year_pillar: YizhangjingStar,   // 年宿 (祖业根基·前世)
    pub month_pillar: YizhangjingStar,  // 月宿 (父母兄弟·初限)
    pub day_pillar: YizhangjingStar,    // 日宿 (夫妻自身·中限)
    pub hour_pillar: YizhangjingStar,   // 时宿 (子息归宿·晚限)
    pub ming_gong: YizhangjingStar,     // 命宫佛性
    pub direction: &'static str,        // 顺行/逆行
}

/// 达摩一掌经排盘
/// 顺逆规则：男顺女逆（或阳男阴女顺、阴男阳女逆）
/// 起法：年上起月顺数生月，月上起日顺数生日，日上起时顺数生时
pub fn calculate_yizhangjing(
    year_branch: &str,
    lunar_month: u32,
    lunar_day: u32,
    hour_branch: &str,
    is_male: bool,
) -> YizhangjingResult {
    let y_idx = branch_index(year_branch);
    let h_idx = branch_index(hour_branch);

    // 阳男阴女顺，阴男阳女逆
    let is_year_yang = y_idx.is_multiple_of(2);
    let is_clockwise = (is_year_yang && is_male) || (!is_year_yang && !is_male);
    let d: i32 = if is_clockwise { 1 } else { -1 };

    let m_idx = ((y_idx as i32 + d * (lunar_month as i32 - 1)).rem_euclid(12)) as usize;
    let d_idx = ((m_idx as i32 + d * (lunar_day as i32 - 1)).rem_euclid(12)) as usize;
    let h_order = (h_idx + 1) as i32; // 子=1, 丑=2, ...
    let t_idx = ((d_idx as i32 + d * (h_order - 1)).rem_euclid(12)) as usize;

    // 默认时上起命（时宫即命宫）
    let mg_idx = t_idx;

    YizhangjingResult {
        year_pillar: get_star_info(y_idx),
        month_pillar: get_star_info(m_idx),
        day_pillar: get_star_info(d_idx),
        hour_pillar: get_star_info(t_idx),
        ming_gong: get_star_info(mg_idx),
        direction: if is_clockwise { "顺行" } else { "逆行" },
    }
}
