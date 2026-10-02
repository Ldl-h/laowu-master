// 小六壬起课与三传断语推导引擎 (Xiao Liu Ren Divination Engine)
// 纯 Rust 高精度零依赖实现，支持主流六宫与道门九宫

#[derive(Debug, Clone, serde::Serialize)]
pub struct ShenInfo {
    pub name: &'static str,
    pub wuxing: &'static str,
    pub nature: &'static str,
    pub jixiong: &'static str,
    pub poem: &'static str,
}

pub const MAIN_SIX: [&str; 6] = ["大安", "留连", "速喜", "赤口", "小吉", "空亡"];
pub const DAO_NINE: [&str; 9] = ["大安", "留连", "速喜", "赤口", "小吉", "空亡", "病符", "桃花", "天德"];

pub fn get_shen_info(name: &str) -> ShenInfo {
    match name {
        "大安" => ShenInfo {
            name: "大安",
            wuxing: "木",
            nature: "身不动时，五行属木，青龙吉神",
            jixiong: "大吉",
            poem: "大安事事昌，求谋在东方，失物去不远，宅舍保安康。",
        },
        "留连" => ShenInfo {
            name: "留连",
            wuxing: "水",
            nature: "人未归时，五行属水，玄武凶神",
            jixiong: "次凶",
            poem: "留连事难成，求谋日未明，官事只宜缓，去者未回程。",
        },
        "速喜" => ShenInfo {
            name: "速喜",
            wuxing: "火",
            nature: "人即至时，五行属火，朱雀吉神",
            jixiong: "中吉",
            poem: "速喜喜来临，求财向南行，失物申未午，逢人路上寻。",
        },
        "赤口" => ShenInfo {
            name: "赤口",
            wuxing: "金",
            nature: "官事凶时，五行属金，白虎凶神",
            jixiong: "大凶",
            poem: "赤口主口舌，官事且紧防，失物急去寻，行人有惊慌。",
        },
        "小吉" => ShenInfo {
            name: "小吉",
            wuxing: "木",
            nature: "人来喜时，五行属木，六合吉神",
            jixiong: "小吉",
            poem: "小吉最吉昌，路上好商量，阴人来报喜，失物在坤方。",
        },
        "空亡" => ShenInfo {
            name: "空亡",
            wuxing: "土",
            nature: "音信稀时，五行属土，勾陈大凶",
            jixiong: "极凶",
            poem: "空亡事不详，阴人多乖张，求财无利益，行人有灾殃。",
        },
        "病符" => ShenInfo {
            name: "病符",
            wuxing: "水",
            nature: "病魔缠身，五行属水",
            jixiong: "小凶",
            poem: "病符临身疾，求医不可迟，问事多迟滞，破财更生悲。",
        },
        "桃花" => ShenInfo {
            name: "桃花",
            wuxing: "火",
            nature: "情欲欢爱，五行属火",
            jixiong: "半吉",
            poem: "桃花满面春，男女动私情，谋事多阻滞，暗昧少分明。",
        },
        _ => ShenInfo {
            name: "天德",
            wuxing: "金",
            nature: "贵人扶持，五行属金",
            jixiong: "大吉",
            poem: "天德降吉祥，贵人引路长，逢凶皆化吉，百事保安康。",
        },
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct XiaoLiuRenResult {
    pub school: &'static str,
    pub nums: [u32; 3],
    pub chu_chuan: ShenInfo,    // 初传 (我/起因)
    pub zhong_chuan: ShenInfo,  // 中传 (他人外界/经过)
    pub mo_chuan: ShenInfo,     // 末传 (结果)
    pub special_notes: Vec<&'static str>,
}

/// 计算小六壬三传
/// n1, n2, n3 分别为月/日/时，或报数三数
/// is_dao: 是否采用道门九宫（默认 false 为正统六宫大安~空亡）
pub fn calculate_xiaoliuren(n1: u32, n2: u32, n3: u32, is_dao: bool) -> XiaoLiuRenResult {
    let ring = if is_dao { &DAO_NINE[..] } else { &MAIN_SIX[..] };
    let len = ring.len();

    let mut idx = 0;
    let idx1 = (idx + (n1.saturating_sub(1) as usize)) % len;
    idx = idx1;
    let idx2 = (idx + (n2.saturating_sub(1) as usize)) % len;
    idx = idx2;
    let idx3 = (idx + (n3.saturating_sub(1) as usize)) % len;

    let s1 = ring[idx1];
    let s2 = ring[idx2];
    let s3 = ring[idx3];

    let mut notes = Vec::new();
    let chuan = [s1, s2, s3];
    let count_liulian = chuan.iter().filter(|&&x| x == "留连").count();
    let count_chikou = chuan.iter().filter(|&&x| x == "赤口").count();
    let has_kongwang = chuan.contains(&"空亡");

    if count_liulian >= 2 {
        notes.push("重叠留连：事情反复牵连，拖延难解。");
    }
    if count_chikou >= 2 {
        notes.push("重叠赤口：口舌是非纷争极烈，须防争斗官讼。");
    }
    if has_kongwang {
        notes.push("见空亡：所谋易空，防谋事中途落空或受欺瞒。");
    }

    XiaoLiuRenResult {
        school: if is_dao { "道门九宫" } else { "主流六宫" },
        nums: [n1, n2, n3],
        chu_chuan: get_shen_info(s1),
        zhong_chuan: get_shen_info(s2),
        mo_chuan: get_shen_info(s3),
        special_notes: notes,
    }
}
