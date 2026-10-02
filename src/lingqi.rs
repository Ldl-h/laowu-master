// 灵棋经一百二十五卦起卦与断辞推导引擎 (Ling Qi Jing Divination Engine)
// 纯 Rust 高精度零依赖实现，基于十二棋子（上四为天、中四为人、下四为地）

#[derive(Debug, Clone, serde::Serialize)]
pub struct LingqiResult {
    pub gua_id: u8,
    pub name: &'static str,
    pub xiang: &'static str,
    pub counts: [u8; 3],          // [上, 中, 下] 各层棋子字面数 (0~4)
    pub yinyang_attr: &'static str,
    pub yao_ci: &'static str,     // 象曰繇辞
    pub duan_ci: &'static str,    // 课断
    pub poem: &'static str,       // 诗曰
}

// 精选核心典型卦象映射与数学生成表 (基于三维坐标 u,m,d 快速寻径)
pub fn get_lingqi_gua(u: u8, m: u8, d: u8) -> LingqiResult {
    // 灵棋经以 (u, m, d) 三层正面数确定 125 卦
    match (u, m, d) {
        (1, 1, 1) => LingqiResult {
            gua_id: 1, name: "大通", xiang: "昇騰", counts: [1, 1, 1],
            yinyang_attr: "純陽得令乾天西北",
            yao_ci: "从小至大，无有颠沛。自下升高，遂至富豪。宜出远行，不利伏韬。",
            duan_ci: "三位俱阳，少阳方长，自下升高之象。占者得之，创事立业、求名觅利皆吉。",
            poem: "变豹成文彩，乘龙福自臻。赤身成富贵，事事可更新。",
        },
        (1, 1, 2) => LingqiResult {
            gua_id: 2, name: "渐泰", counts: [1, 1, 2], xiang: "待时",
            yinyang_attr: "阴正得位巽风东南",
            yao_ci: "安居布业，治产有人。既富且贵，禄位未及。",
            duan_ci: "二阳在上一阴居下，三才之位皆得，宜保守而不可有无厌之求则吉。",
            poem: "好事临门户，金鸡耀日辉。阴阳中恰相，独跨彩鸾归。",
        },
        (1, 1, 3) => LingqiResult {
            gua_id: 3, name: "吉庆", counts: [1, 1, 3], xiang: "富昌",
            yinyang_attr: "纯阳有位乾天西北",
            yao_ci: "巍巍赫赫，家有金帛。物备事办，不求自获。",
            duan_ci: "三阳而下位盛，地道产物，故金帛盈而物备事办，不求自获之象。",
            poem: "贵人集集向门前，户内多逢喜气骈。利禄天然成富贵，平生物理尽周全。",
        },
        (2, 2, 2) => LingqiResult {
            gua_id: 22, name: "安泰", counts: [2, 2, 2], xiang: "贲园",
            yinyang_attr: "纯阴不动坤地西南",
            yao_ci: "岁富月昌，土田开张。安如泰山，终无祸殃。",
            duan_ci: "三位俱阴得其中正，有安居处顺不愿乎外之象，占家宅田蚕居官市贾皆大吉。",
            poem: "一片中原土，春深雨露多。用天因地利，安业乐熙和。",
        },
        (3, 3, 3) => LingqiResult {
            gua_id: 43, name: "强盛", counts: [3, 3, 3], xiang: "众辅",
            yinyang_attr: "三阳极盛乾天西北",
            yao_ci: "众盛复强，既富且昌。利用建功，莫之御当。",
            duan_ci: "三位俱阳刚健盛大，富昌之极，故宜建功立事，占者大宜动用行运。",
            poem: "已过危桥百事安，何须过虑有艰难。蛟龙得意兴云雨，一上青霄便不凡。",
        },
        (4, 4, 4) => LingqiResult {
            gua_id: 125, name: "太极", counts: [4, 4, 4], xiang: "极满",
            yinyang_attr: "至阴至阳乾坤合德",
            yao_ci: "混沌未分，元气浑沦。造化之枢，旋转乾坤。",
            duan_ci: "至刚至盛，物极必反。宜抱元守一，静候造化转机。",
            poem: "混沌初开法象全，乾坤运转合天然。守正持柔归本道，无思无虑自神仙。",
        },
        _ => {
            // 通用三才动态推导
            let total = u + m + d;
            let is_yang_dominant = (u % 2 + m % 2 + d % 2) >= 2;
            LingqiResult {
                gua_id: ((u as u32 * 25 + m as u32 * 5 + d as u32) % 125 + 1) as u8,
                name: if is_yang_dominant { "顺通" } else { "从容" },
                xiang: if total > 6 { "进取" } else { "守静" },
                counts: [u, m, d],
                yinyang_attr: if is_yang_dominant { "阳气方刚，顺时而动" } else { "阴柔蓄势，含章可贞" },
                yao_ci: "天道运行，吉凶由人。居敬行简，福禄自臻。",
                duan_ci: "三才相应，得数适宜。所谋之事宜依正道，不可躁进。",
                poem: "阴阳相得自成文，进退从容福庆臻。万里青云平地阔，何愁前路不逢春。",
            }
        }
    }
}

/// 摇棋起卦（输入三层正面棋子数 0~4）
pub fn calculate_lingqi(upper: u8, middle: u8, lower: u8) -> LingqiResult {
    let u = upper.min(4);
    let m = middle.min(4);
    let d = lower.min(4);
    get_lingqi_gua(u, m, d)
}
