// 天文地占术十六图形与十二宫推演引擎 (Geomancy 16 Figures & Houses Engine)
// 包含：四母、四女、四侄、二证人、一大法官十六图体系，遵从经典欧洲与阿拉伯沙盘 (Raml) 规制

#[derive(Debug, Clone, serde::Serialize)]
pub struct GeomancyFigure {
    pub id: u8,
    pub name_cn: &'static str,
    pub name_en: &'static str,
    pub element: &'static str,
    pub planet: &'static str,
    pub lines: [u8; 4], // 从上到下 4 爻: 1=奇数(单点·阳), 2=偶数(双点·阴)
    pub nature: &'static str,
}

pub const GEOMANCY_FIGURES: [GeomancyFigure; 16] = [
    GeomancyFigure { id: 1,  name_cn: "大吉", name_en: "Fortuna Major", element: "土", planet: "太阳", lines: [2, 2, 1, 1], nature: "大吉，成功，权威，名誉，长久之利" },
    GeomancyFigure { id: 2,  name_cn: "小吉", name_en: "Fortuna Minor", element: "火", planet: "太阳", lines: [1, 1, 2, 2], nature: "次吉，短期之成，速战速决，贵人扶助" },
    GeomancyFigure { id: 3,  name_cn: "白面", name_en: "Albus",         element: "水", planet: "水星", lines: [2, 2, 1, 2], nature: "纯洁，智慧，和平，清白，谋求文书吉" },
    GeomancyFigure { id: 4,  name_cn: "红面", name_en: "Rubeus",        element: "火", planet: "火星", lines: [2, 1, 2, 2], nature: "大凶，暴怒，血光，冲动，只利征战伏魔" },
    GeomancyFigure { id: 5,  name_cn: "得入", name_en: "Acquisitio",    element: "风", planet: "木星", lines: [2, 1, 2, 1], nature: "得利，收益，进财，富饶，进取大吉" },
    GeomancyFigure { id: 6,  name_cn: "损失", name_en: "Amissio",       element: "火", planet: "金星", lines: [1, 2, 1, 2], nature: "失去，破耗，放手，不利求财，利舍身" },
    GeomancyFigure { id: 7,  name_cn: "欢喜", name_en: "Laetitia",      element: "风", planet: "木星", lines: [1, 2, 2, 2], nature: "喜庆，健康，愉悦，高尚，逢凶化吉" },
    GeomancyFigure { id: 8,  name_cn: "悲伤", name_en: "Tristitia",     element: "土", planet: "土星", lines: [2, 2, 2, 1], nature: "忧愁，压抑，迟滞，秘匿，利沉潜与地产" },
    GeomancyFigure { id: 9,  name_cn: "女子", name_en: "Puella",        element: "水", planet: "金星", lines: [1, 2, 1, 1], nature: "和美，欢爱，阴柔，艺术，利婚姻社交" },
    GeomancyFigure { id: 10, name_cn: "少年", name_en: "Puer",          element: "火", planet: "火星", lines: [1, 1, 2, 1], nature: "鲁莽，热血，争斗，急躁，利武力突围" },
    GeomancyFigure { id: 11, name_cn: "结合", name_en: "Conjunctio",   element: "风", planet: "水星", lines: [2, 1, 1, 2], nature: "聚会，契约，联合，信息，谈判与盟约" },
    GeomancyFigure { id: 12, name_cn: "囚禁", name_en: "Carcer",        element: "土", planet: "土星", lines: [1, 2, 2, 1], nature: "束缚，封闭，防守，监禁，利秘密与固守" },
    GeomancyFigure { id: 13, name_cn: "大众", name_en: "Populus",       element: "水", planet: "月亮", lines: [2, 2, 2, 2], nature: "群体，随波逐流，中立，变动，利集会" },
    GeomancyFigure { id: 14, name_cn: "道路", name_en: "Via",           element: "水", planet: "月亮", lines: [1, 1, 1, 1], nature: "旅途，行进，孤独，加速，变动不居" },
    GeomancyFigure { id: 15, name_cn: "龙头", name_en: "Caput Draconis",element: "土", planet: "北交", lines: [2, 1, 1, 1], nature: "开端，升华，接纳，福源注入之门" },
    GeomancyFigure { id: 16, name_cn: "龙尾", name_en: "Cauda Draconis",element: "火", planet: "南交", lines: [1, 1, 1, 2], nature: "终结，排泄，阻滞，决裂，防诈欺毒害" },
];

pub fn figure_by_lines(lines: [u8; 4]) -> GeomancyFigure {
    for f in &GEOMANCY_FIGURES {
        if f.lines == lines {
            return f.clone();
        }
    }
    GEOMANCY_FIGURES[0].clone()
}

// 两图逐爻相加求余得子图 (1+1=2, 1+2=1, 2+2=2)
pub fn combine_figures(a: &GeomancyFigure, b: &GeomancyFigure) -> GeomancyFigure {
    let mut lines = [0u8; 4];
    for i in 0..4 {
        lines[i] = if (a.lines[i] + b.lines[i]).is_multiple_of(2) { 2 } else { 1 };
    }
    figure_by_lines(lines)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GeomancyShieldChart {
    pub mothers: [GeomancyFigure; 4],      // 1~4: 四母
    pub daughters: [GeomancyFigure; 4],    // 5~8: 四女 (横取母图之四行)
    pub nieces: [GeomancyFigure; 4],       // 9~12: 四侄 (相克衍生)
    pub right_witness: GeomancyFigure,    // 13: 右证人 (主过去/问者)
    pub left_witness: GeomancyFigure,     // 14: 左证人 (主未来/对方)
    pub judge: GeomancyFigure,            // 15: 法官 (最终判决)
    pub reconciler: GeomancyFigure,       // 16: 调和者 (法官加第一母)
    pub verdict: &'static str,
}

/// 计算地占盾牌盘 (Shield Chart)
/// 传入 4 组四爻母图 (例如通过点点法随机产生)
pub fn calculate_geomancy(m1: [u8; 4], m2: [u8; 4], m3: [u8; 4], m4: [u8; 4]) -> GeomancyShieldChart {
    let fig_m1 = figure_by_lines(m1);
    let fig_m2 = figure_by_lines(m2);
    let fig_m3 = figure_by_lines(m3);
    let fig_m4 = figure_by_lines(m4);

    let mothers = [fig_m1.clone(), fig_m2.clone(), fig_m3.clone(), fig_m4.clone()];

    // 四女由四母按列横切提取：第 i 女 = 四母之第 i 爻
    let daughters = [
        figure_by_lines([mothers[0].lines[0], mothers[1].lines[0], mothers[2].lines[0], mothers[3].lines[0]]),
        figure_by_lines([mothers[0].lines[1], mothers[1].lines[1], mothers[2].lines[1], mothers[3].lines[1]]),
        figure_by_lines([mothers[0].lines[2], mothers[1].lines[2], mothers[2].lines[2], mothers[3].lines[2]]),
        figure_by_lines([mothers[0].lines[3], mothers[1].lines[3], mothers[2].lines[3], mothers[3].lines[3]]),
    ];

    // 四侄：9=1+2, 10=3+4, 11=5+6, 12=7+8
    let n1 = combine_figures(&mothers[0], &mothers[1]);
    let n2 = combine_figures(&mothers[2], &mothers[3]);
    let n3 = combine_figures(&daughters[0], &daughters[1]);
    let n4 = combine_figures(&daughters[2], &daughters[3]);
    let nieces = [n1, n2, n3, n4];

    // 二证人：13 (右证人)=9+10, 14 (左证人)=11+12
    let right_witness = combine_figures(&nieces[0], &nieces[1]);
    let left_witness = combine_figures(&nieces[2], &nieces[3]);

    // 大法官：15=13+14
    let judge = combine_figures(&right_witness, &left_witness);

    // 调和者 (Reconciler)：16=15+1
    let reconciler = combine_figures(&judge, &mothers[0]);

    let verdict = if judge.lines == [2, 2, 1, 1] || judge.lines == [2, 1, 2, 1] || judge.lines == [1, 2, 2, 2] {
        "大势顺畅，法官显吉，所谋事大成。"
    } else if judge.lines == [2, 1, 2, 2] || judge.lines == [1, 2, 2, 1] {
        "险阻重重，多有变故阻碍，宜守不宜进。"
    } else {
        "平稳之局，吉凶参半，当看调和者与证人之向背。"
    };

    GeomancyShieldChart {
        mothers,
        daughters,
        nieces,
        right_witness,
        left_witness,
        judge,
        reconciler,
        verdict,
    }
}
