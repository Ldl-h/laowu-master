use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct UniversalInput {
    pub date: Option<String>,
    pub time: Option<String>,
    pub year: Option<i32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
    pub hour: Option<u32>,
    pub minute: Option<u32>,
    pub second: Option<u32>,
    pub gender: Option<u32>, // 1: 男, 2: 女
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub options: Option<serde_json::Map<String, Value>>,
    pub params: Option<serde_json::Map<String, Value>>,

    // 术数特定参数
    #[serde(alias = "yueJiang")]
    pub yue_jiang: Option<usize>,
    #[serde(alias = "zhanShi")]
    pub zhan_shi: Option<usize>,
    #[serde(alias = "dayGan")]
    pub day_gan: Option<String>,
    pub day_zhi: Option<usize>,
    #[serde(alias = "dayZhi")]
    #[serde(alias = "dayZhiStr")]
    pub day_zhi_str: Option<String>,
    pub hour_zhi: Option<usize>,
    pub di_fen: Option<usize>,
    pub numbers: Option<Vec<u32>>,
    pub nums: Option<Vec<u32>>,
    pub year_gz: Option<String>,
    pub month_gz: Option<String>,
    pub day_gz: Option<String>,
    pub hour_gz: Option<String>,
    pub age: Option<f64>,
    pub after23_new_day: Option<bool>,
    pub late_zi_use_next_day: Option<bool>,
    pub school: Option<String>,
    pub ascendant: Option<f64>,
    pub question_type: Option<String>,
    #[serde(alias = "yearGan")]
    pub year_gan: Option<String>,
    #[serde(alias = "hourGan")]
    pub hour_gan: Option<String>,
    pub domain: Option<String>,
    pub format: Option<String>,
    pub family_key: Option<String>,
    pub text: Option<String>,
    pub seed: Option<u64>,
    pub harmonic: Option<f64>,
    pub orb: Option<f64>,
    pub target_date: Option<String>,
    pub target_year: Option<i32>,
    pub target_lat: Option<f64>,
    pub target_lon: Option<f64>,
    pub hsys: Option<String>,
    pub lines: Option<Vec<Value>>,
    pub pillars: Option<Vec<String>>,
    pub spread: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub year_pillar: Option<String>,
    pub month_pillar: Option<String>,
    pub day_pillar: Option<String>,
    pub hour_pillar: Option<String>,
    pub up: Option<String>,
    pub lo: Option<String>,
    #[serde(alias = "dongYaos")]
    pub dong_yaos: Option<Vec<usize>>,

    #[serde(alias = "qiZhi")]
    pub qi_zhi: Option<String>,

    #[serde(alias = "isLunar")]
    pub is_lunar: Option<bool>,
    #[serde(alias = "lunarMonth")]
    pub lunar_month: Option<u32>,
    #[serde(alias = "lunarDay")]
    pub lunar_day: Option<u32>,

    // 高级控制旋钮与流派参数
    pub conditions: Option<Vec<crate::zeri::ZeriCondition>>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    #[serde(alias = "paiPanType")]
    pub pai_pan_type: Option<u32>,
    pub style: Option<u8>,
    pub qiju_method: Option<String>,
    pub city: Option<String>,
}

impl UniversalInput {
    /// 统一提取有效经纬度（优先显式坐标，次之城市名称匹配高精地理库 CGEO）
    pub fn get_location(&self) -> (f64, f64, String) {
        if let (Some(la), Some(lo)) = (self.lat, self.lon) {
            return (la, lo, self.city.clone().unwrap_or_else(|| "自定义坐标".to_string()));
        }
        if let Some(ref c) = self.city {
            if let Some((lo, la, name)) = crate::db::find_geo_location(c) {
                return (la, lo, name);
            }
        }
        if let Some(ref n) = self.name {
            if let Some((lo, la, name)) = crate::db::find_geo_location(n) {
                return (la, lo, name);
            }
        }
        (self.lat.unwrap_or(31.23), self.lon.unwrap_or(121.47), "默认位置".to_string())
    }

    /// 统一提取有效年月日时分秒（优先从 date/time 字符串解析，回退到数字字段）
    pub fn get_datetime(&self) -> (i32, u32, u32, u32, u32, u32) {
        let mut y = self.year.unwrap_or(2026);
        let mut m = self.month.unwrap_or(9);
        let mut d = self.day.unwrap_or(30);
        let mut h = self.hour.unwrap_or(14);
        let mut min = self.minute.unwrap_or(0);
        let mut sec = self.second.unwrap_or(0);

        if let Some(ref date_str) = self.date {
            let parts: Vec<&str> = date_str.split('-').collect();
            if parts.len() == 3 {
                if let Ok(py) = parts[0].parse() { y = py; }
                if let Ok(pm) = parts[1].parse() { m = pm; }
                if let Ok(pd) = parts[2].parse() { d = pd; }
            }
        }

        if let Some(ref time_str) = self.time {
            let parts: Vec<&str> = time_str.split(':').collect();
            if !parts.is_empty() {
                if let Ok(ph) = parts[0].parse() { h = ph; }
                if parts.len() > 1 {
                    if let Ok(pmin) = parts[1].parse() { min = pmin; }
                }
                if parts.len() > 2 {
                    if let Ok(psec) = parts[2].parse() { sec = psec; }
                }
            }
        }

        (y, m, d, h, min, sec)
    }

    /// 严格检查是否显式传入了有效日期/时间参数（防止未传时间时静默回退默认时间）
    pub fn has_explicit_datetime(&self) -> bool {
        self.date.is_some() || (self.year.is_some() && self.month.is_some() && self.day.is_some())
    }

    /// 检查指定字段是否在当前输入中已被提供（非 None 且非空）
    pub fn has_field(&self, param: &str) -> bool {
        match param {
            "date" => self.date.is_some() || (self.year.is_some() && self.month.is_some() && self.day.is_some()),
            "time" => self.time.is_some() || self.hour.is_some(),
            "year" => self.year.is_some() || self.date.is_some(),
            "month" => self.month.is_some() || self.date.is_some(),
            "day" => self.day.is_some() || self.date.is_some(),
            "hour" => self.hour.is_some() || self.time.is_some(),
            "minute" => self.minute.is_some() || self.time.is_some(),
            "second" => self.second.is_some() || self.time.is_some(),
            "gender" => self.gender.is_some(),
            "age" => self.age.is_some(),
            "lat" => self.lat.is_some() || self.city.is_some(),
            "lon" => self.lon.is_some() || self.city.is_some(),
            "city" => self.city.is_some() || self.name.is_some() || (self.lat.is_some() && self.lon.is_some()),
            "target_lat" => self.target_lat.is_some(),
            "target_lon" => self.target_lon.is_some(),
            "target_date" => self.target_date.is_some() || self.target_year.is_some() || self.age.is_some(),
            "target_year" => self.target_year.is_some() || self.target_date.is_some() || self.age.is_some(),
            "start_date" => self.start_date.is_some(),
            "end_date" => self.end_date.is_some(),
            "start_year" => self.start_year.is_some(),
            "end_year" => self.end_year.is_some(),
            "numbers" => self.numbers.as_ref().map_or(false, |v| !v.is_empty()) || self.nums.as_ref().map_or(false, |v| !v.is_empty()),
            "spread" => self.spread.is_some(),
            "seed" => self.seed.is_some(),
            "school" => self.school.is_some(),
            "hsys" => self.hsys.is_some(),
            "harmonic" => self.harmonic.is_some(),
            "orb" => self.orb.is_some(),
            "question_type" => self.question_type.is_some(),
            "family_key" => self.family_key.is_some(),
            "name" => self.name.is_some() || self.text.is_some(),
            "text" => self.text.is_some() || self.name.is_some(),
            "domain" => self.domain.is_some() || self.text.is_some(),
            "format" => self.format.is_some(),
            "year_gan" => self.year_gan.is_some() || self.year_gz.is_some() || self.year_pillar.is_some(),
            "hour_gan" => self.hour_gan.is_some() || self.hour_gz.is_some() || self.hour_pillar.is_some(),
            "year_gz" => self.year_gz.is_some() || self.year_pillar.is_some() || self.pillars.as_ref().map_or(false, |p| !p.is_empty()),
            "month_gz" => self.month_gz.is_some() || self.month_pillar.is_some() || self.pillars.as_ref().map_or(false, |p| p.len() >= 2),
            "day_gz" => self.day_gz.is_some() || self.day_pillar.is_some() || self.pillars.as_ref().map_or(false, |p| p.len() >= 3),
            "hour_gz" => self.hour_gz.is_some() || self.hour_pillar.is_some() || self.pillars.as_ref().map_or(false, |p| p.len() >= 4),
            "day_gan" => self.day_gan.is_some() || self.day_gz.is_some(),
            "day_zhi" => self.day_zhi.is_some() || self.day_zhi_str.is_some() || self.day_gz.is_some(),
            "hour_zhi" => self.hour_zhi.is_some() || self.hour_gz.is_some() || self.time.is_some() || self.hour.is_some(),
            "yue_jiang" => self.yue_jiang.is_some(),
            "zhan_shi" => self.zhan_shi.is_some() || self.hour_zhi.is_some() || self.hour.is_some(),
            "di_fen" => self.di_fen.is_some(),
            "qi_zhi" => self.qi_zhi.is_some(),
            "up" => self.up.is_some(),
            "lo" => self.lo.is_some(),
            "pai_pan_type" => self.pai_pan_type.is_some(),
            "style" => self.style.is_some(),
            "is_lunar" => self.is_lunar.is_some(),
            "lunar_month" => self.lunar_month.is_some(),
            "lunar_day" => self.lunar_day.is_some(),
            "after23_new_day" => self.after23_new_day.is_some(),
            "late_zi_use_next_day" => self.late_zi_use_next_day.is_some(),
            _ => true,
        }
    }

    /// 提取儒略日
    pub fn get_julian_day(&self) -> f64 {
        let (y, m, d, h, min, sec) = self.get_datetime();
        crate::bazi_exact::to_julian_day(y, m, d, h, min, sec)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchResponse {
    pub ok: bool,
    pub tool: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

pub fn split_gz(gz: &str, default_gan: char, default_zhi: &'static str) -> (char, &'static str) {
    let mut chars = gz.chars();
    let g = chars.next().unwrap_or(default_gan);
    let z = match chars.next() {
        Some('子') => "子", Some('丑') => "丑", Some('寅') => "寅", Some('卯') => "卯",
        Some('辰') => "辰", Some('巳') => "巳", Some('午') => "午", Some('未') => "未",
        Some('申') => "申", Some('酉') => "酉", Some('戌') => "戌", Some('亥') => "亥",
        _ => default_zhi,
    };
    (g, z)
}

/// 对特定技法的专属入参进行前置严格健全性校验，拒绝默认假数据静默填充
pub fn validate_technique_input(tool: &str, input: &UniversalInput) -> Result<(), String> {
    // 0. 自动比对 110 项规范中该工具声明的所有专属参数
    if let Some(meta) = crate::dispatch::techniques_spec::get_technique_meta(tool) {
        let mut missing = Vec::new();
        for &p in meta.params {
            if !input.has_field(p) {
                missing.push(p);
            }
        }
        if !missing.is_empty() {
            return Err(format!(
                "技法 [{}] 参数校验不合格：缺少必要参数 [{}]。该技法是用于【{}】，所需专属参数规范为: {:?}。拒绝静默填充默认假数据！",
                tool,
                missing.join(", "),
                meta.name_zh,
                meta.params
            ));
        }
    }

    match tool {
        // 1. 八字干支逆推
        "bazi_inverse" => {
            let has_four = (input.year_gz.is_some() || input.year_pillar.is_some())
                && (input.month_gz.is_some() || input.month_pillar.is_some())
                && (input.day_gz.is_some() || input.day_pillar.is_some())
                && (input.hour_gz.is_some() || input.hour_pillar.is_some());
            let has_pillars = input.pillars.as_ref().map_or(false, |p| p.len() >= 4);
            if !has_four && !has_pillars {
                return Err("技法 [bazi_inverse] 缺少完整的四柱干支参数，必须传入 year_gz, month_gz, day_gz, hour_gz 或 pillars 列表，禁止静默使用默认干支".to_string());
            }
        }

        // 2. 六爻起卦 / 灵棋经 / 地占 / 古通社法
        "liuyao" | "gua_desc" | "gua_meiyi" => {
            if let Some(nums) = input.numbers.as_ref().or(input.nums.as_ref()) {
                if nums.is_empty() {
                    return Err(format!("技法 [{}] 传入的起卦数字列表 numbers 不能为空", tool));
                }
            }
        }
        "lingqi" => {
            if let Some(nums) = input.numbers.as_ref().or(input.nums.as_ref()) {
                if nums.len() < 3 {
                    return Err("技法 [lingqi] 灵棋经起卦必须传入至少 3 组子数 [上, 中, 下]".to_string());
                }
            }
        }
        "geomancy" => {
            if let Some(nums) = input.numbers.as_ref().or(input.nums.as_ref()) {
                if nums.len() < 4 {
                    return Err("技法 [geomancy] 地占排盘必须传入至少 4 组种子数生成四母亲卦".to_string());
                }
            }
        }

        // 3. 鬼谷子两头钳
        "guice" => {
            let has_yg = input.year_gan.is_some() || input.year_gz.is_some() || input.year_pillar.is_some();
            let has_hg = input.hour_gan.is_some() || input.hour_gz.is_some() || input.hour_pillar.is_some();
            if !has_yg || !has_hg {
                return Err("技法 [guice] 缺少必要参数: 必须同时传入年干 (year_gan) 与时干 (hour_gan)".to_string());
            }
        }

        // 4. 金口诀
        "jinkou" => {
            if input.day_gan.is_none() && input.day_gz.is_none() {
                return Err("技法 [jinkou] 缺少必要参数: day_gan (日干)".to_string());
            }
        }

        // 5. 霍斐然小成图
        "xiaochengtu" => {
            if input.up.is_none() || input.lo.is_none() {
                return Err("技法 [xiaochengtu] 缺少必要参数: 必须同时指定上卦 (up) 与下卦 (lo)".to_string());
            }
        }

        // 6. 神数家族
        "nanji" | "beiji" | "taixuan" | "wangji" | "cetian" | "chunzi" | "fendjing" | "jingjue" | "shenyishu" | "wuzhao" | "shaozi" => {
            let has_gz = input.year_gz.is_some() || input.year_pillar.is_some();
            let has_dt = input.has_explicit_datetime();
            if !has_gz && !has_dt {
                return Err(format!("技法 [{}] 缺少排盘基准: 必须提供生辰年月日时 (date/time) 或四柱干支 (year_gz 等)", tool));
            }
        }

        // 7. 择吉区间扫描类
        "bazizeri" | "qimenzeri" | "taiyizeri" | "ziweizeri" | "liurengzeri" | "sanshizeri" | "qizhengzeri" | "indiazeri" | "huanglizeri" | "zeri" | "qizhengelection" => {
            if input.start_date.is_none() || input.end_date.is_none() {
                return Err(format!("择吉技法 [{}] 必须提供扫描时间区间: start_date 与 end_date", tool));
            }
        }

        // 8. 知识检索与名人案例库
        "xuanshi" | "astrodata" => {
            let has_query = input.name.is_some() || input.text.is_some();
            if !has_query {
                return Err(format!("检索工具 [{}] 必须提供检索关键词 (name 或 text)", tool));
            }
        }

        _ => {}
    }
    Ok(())
}

/// 全量技法动态调度入口：分发至子领域处理器 (东方术数、西洋天星、民间杂占与实用历算)
pub fn dispatch_tool(tool: &str, input: UniversalInput) -> DispatchResponse {
    // 0. 特殊内置元调度查询：返回110项技法清单、入参规范与元数据总览
    if tool == "list" || tool == "techniques" || tool == "registry_110" || tool == "spec_110" {
        return DispatchResponse {
            ok: true,
            tool: tool.to_string(),
            error: None,
            data: Some(crate::dispatch::techniques_spec::generate_ai_registry_json()),
        };
    }

    if tool == "spec" || tool == "technique_info" {
        let target_tool = input.text.as_deref().or(input.name.as_deref()).unwrap_or("");
        if let Some(meta) = crate::dispatch::techniques_spec::get_technique_meta(target_tool) {
            return DispatchResponse {
                ok: true,
                tool: tool.to_string(),
                error: None,
                data: Some(serde_json::to_value(meta).unwrap_or(Value::Null)),
            };
        } else {
            return DispatchResponse {
                ok: false,
                tool: tool.to_string(),
                error: Some(format!("未找到指定技法元数据: '{}'", target_tool)),
                data: None,
            };
        }
    }

    // 0.1 统一入参前置严格健全性校验：若缺少核心参数直接报错，杜绝静默假数据
    if let Err(err_msg) = validate_technique_input(tool, &input) {
        return DispatchResponse {
            ok: false,
            tool: tool.to_string(),
            error: Some(err_msg),
            data: None,
        };
    }


    // 1. 东方术数与三式核心 (八字、紫微、奇门、太乙、六壬、金口诀、三式合一)
    if let Some(res) = crate::dispatch::eastern::handle_eastern(tool, &input) {
        return to_response(tool, res);
    }

    // 2. 西洋占星、古典希腊、巴比伦、乌拉尼亚、吠陀印占及各阶推运
    if let Some(res) = crate::dispatch::western::handle_western_and_astro(tool, &input) {
        return to_response(tool, res);
    }

    // 3. 易经六爻、河洛理数、神数矩阵、择吉通书、民间杂占及历法实用
    if let Some(res) = crate::dispatch::practical::handle_practical(tool, &input) {
        return to_response(tool, res);
    }

    // 兜底未匹配
    DispatchResponse {
        ok: false,
        tool: tool.to_string(),
        error: Some(format!("未识别或暂未实现的技法工具: {}", tool)),
        data: None,
    }
}

fn to_response(tool: &str, res: Result<Value, String>) -> DispatchResponse {
    match res {
        Ok(data) => DispatchResponse {
            ok: true,
            tool: tool.to_string(),
            error: None,
            data: Some(data),
        },
        Err(err) => DispatchResponse {
            ok: false,
            tool: tool.to_string(),
            error: Some(err),
            data: None,
        },
    }
}
