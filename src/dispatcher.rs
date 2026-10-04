use serde_json::Value;
use serde::Deserialize;

/// 性别入参归一化反序列化器：接受数字 (1男/2女/0女) 与常见字符串表示
/// （"男"/"女"/"male"/"female"/"M"/"F"/"1"/"2"/"0" 等），统一归一为 1(男)/2(女)。
fn de_gender<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: Option<serde_json::Value> = Option::<serde_json::Value>::deserialize(deserializer)?;
    match v {
        None => Ok(None),
        Some(serde_json::Value::Number(n)) => {
            let g = n.as_u64().unwrap_or(0) as u32;
            match g {
                1 => Ok(Some(1)),
                0 | 2 => Ok(Some(2)),
                _ => Err(serde::de::Error::custom(format!("性别代码 [{}] 非法，必须为 1 (男) 或 2 (女)", g))),
            }
        }
        Some(serde_json::Value::String(s)) => {
            let norm = s.trim().to_lowercase();
            match norm.as_str() {
                "1" | "男" | "乾" | "male" | "m" | "man" | "yang" => Ok(Some(1)),
                "0" | "2" | "女" | "坤" | "female" | "f" | "woman" | "yin" => Ok(Some(2)),
                _ => Err(serde::de::Error::custom(format!("性别字符串 [{}] 无法识别，必须为 男/女 或 1/2", s))),
            }
        }
        Some(_) => Err(serde::de::Error::custom("性别参数类型非法，必须为数字或字符串")),
    }
}

/// 解析日期字符串，支持 "YYYY-MM-DD" 与 "-YYYY-MM-DD"（公元前，天文年编号）
pub fn parse_date_str(s: &str) -> Option<(i32, u32, u32)> {
    let (neg, rest) = if let Some(r) = s.strip_prefix('-') {
        (true, r)
    } else {
        (false, s)
    };
    let parts: Vec<&str> = rest.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let y: i32 = parts[0].parse().ok()?;
    let m: u32 = parts[1].parse().ok()?;
    let d: u32 = parts[2].parse().ok()?;
    Some((if neg { -y } else { y }, m, d))
}

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
    #[serde(default, deserialize_with = "de_gender")]
    pub gender: Option<u32>, // 1: 男, 2: 女（入参自动归一化，接受 0/男/female 等）
    #[serde(alias = "timeAlg")]
    pub time_alg: Option<u8>,
    #[serde(alias = "gpsLat")]
    pub gps_lat: Option<f64>,
    #[serde(alias = "gpsLon")]
    pub gps_lon: Option<f64>,
    pub ad: Option<i32>,
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
    #[serde(alias = "after23NewDay")]
    pub after23_new_day: Option<bool>,
    #[serde(alias = "lateZiHourUseNextDay")]
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
    /// P1-5：前端/Pydantic schema 用 `query` 作为检索关键词字段名。
    /// astrodata 顶层 {"query":"Einstein"} 等效于 {"text":"Einstein"}（dispatch 内归一化到 text）。
    pub query: Option<String>,
    pub seed: Option<u64>,
    pub harmonic: Option<f64>,
    pub orb: Option<f64>,
    pub target_date: Option<String>,
    pub target_year: Option<i32>,
    #[serde(alias = "targetLat")]
    pub target_lat: Option<f64>,
    #[serde(alias = "targetLon")]
    pub target_lon: Option<f64>,
    pub hsys: Option<String>,
    pub lines: Option<Vec<Value>>,
    pub pillars: Option<Vec<String>>,
    pub spread: Option<String>,
    #[serde(alias = "startYear")]
    pub start_year: Option<i32>,
    #[serde(alias = "endYear")]
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
    /// 时区偏移（小时，相对 UTC）。修复 P1-14：原引擎硬编码 UTC+8，
    /// 引入入参允许其他时区（如纽约 -5）。默认 8.0（北京时间）。
    #[serde(alias = "timezoneOffset")]
    #[serde(alias = "tzOffset")]
    pub timezone_offset: Option<f64>,
}

impl UniversalInput {
    /// 时区偏移（小时），默认 8.0（北京时间）。修复 P1-14。
    pub fn timezone_offset(&self) -> f64 {
        self.timezone_offset.unwrap_or(8.0)
    }

    /// 内建主要城市经纬度兜底表（db.rs 城市库未命中时使用，返回 (lat, lon, name)）。
    /// 修复 P1-9：西部/边远城市（乌鲁木齐/拉萨等）原会静默回退上海导致时柱错 1~2 小时，
    /// 此处补入主要城市真实坐标，减少对外部地理库的依赖。
    pub fn builtin_city_geo(city: &str) -> Option<(f64, f64, String)> {
        let t: (f64, f64, &str) = match city {
            "乌鲁木齐" => (43.82, 87.62, "乌鲁木齐"),
            "拉萨" => (29.65, 91.14, "拉萨"),
            "昆明" => (25.04, 102.68, "昆明"),
            "兰州" => (36.06, 103.83, "兰州"),
            "哈尔滨" => (45.80, 126.53, "哈尔滨"),
            "沈阳" => (41.80, 123.43, "沈阳"),
            "台北" => (25.03, 121.56, "台北"),
            "北京" => (39.90, 116.40, "北京"),
            "上海" => (31.23, 121.47, "上海"),
            "天津" => (39.13, 117.20, "天津"),
            "广州" => (23.13, 113.26, "广州"),
            "深圳" => (22.54, 114.06, "深圳"),
            "成都" => (30.57, 104.07, "成都"),
            "重庆" => (29.56, 106.55, "重庆"),
            "西安" => (34.34, 108.94, "西安"),
            "杭州" => (30.27, 120.15, "杭州"),
            "武汉" => (30.59, 114.31, "武汉"),
            "南京" => (32.06, 118.80, "南京"),
            "香港" => (22.32, 114.17, "香港"),
            "澳门" => (22.20, 113.55, "澳门"),
            _ => return None,
        };
        Some((t.0, t.1, t.2.to_string()))
    }

    /// 统一提取有效经纬度（优先显式坐标，次之城市名称匹配高精地理库 CGEO）
    pub fn get_location(&self) -> (f64, f64, String) {
        self.get_location_detail().0
    }

    /// 带定位警告的经纬度提取。返回 ((lat, lon, name), location_warning)。
    /// 修复 P1-9：当 city 入参存在但地理库与内建表均未命中时，不再静默回退上海，
    /// 而是返回 location_warning 显式告知调用方/最终用户“已使用默认坐标”。
    pub fn get_location_detail(&self) -> ((f64, f64, String), Option<String>) {
        if let (Some(la), Some(lo)) = (self.lat, self.lon) {
            return (
                (la, lo, self.city.clone().unwrap_or_else(|| "自定义坐标".to_string())),
                None,
            );
        }
        if let Some(ref c) = self.city {
            if let Some((lo, la, name)) = crate::db::find_geo_location(c) {
                return ((la, lo, name), None);
            }
            // 内建主要城市兜底（含西部城市真实经度）
            if let Some((la, lo, name)) = Self::builtin_city_geo(c) {
                return ((la, lo, name), None);
            }
            // 未命中 → 显式警告，禁止静默回退
            let warn = format!(
                "城市 [{}] 未在地理库与内建城市表中命中，已回退使用默认坐标 (上海 31.23°N,121.47°E)。西部/边远城市会导致时柱偏差，请核对城市名或显式传入 lat/lon",
                c
            );
            return (
                (31.23, 121.47, format!("默认位置(上海)[城市{}未命中]", c)),
                Some(warn),
            );
        }
        if let Some(ref n) = self.name {
            if let Some((lo, la, name)) = crate::db::find_geo_location(n) {
                return ((la, lo, name), None);
            }
        }
        // 修复 P2-6：全局默认位置统一为上海(31.23°N,121.47°E)，全引擎只此一处默认定义；
        // 无 lat/lon/city 时落到此处（不产生 location_warning）。北京(39.90,116.40)仅为城市查表项，
        // 不作默认；mundane 世运盘另用北京为设计特例，不在此处。
        (
            (self.lat.unwrap_or(31.23), self.lon.unwrap_or(121.47), "默认位置".to_string()),
            None,
        )
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
            if let Some((py, pm, pd)) = parse_date_str(date_str) {
                y = py;
                m = pm;
                d = pd;
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
                    // P2-4：秒段支持小数秒（对齐 JS），如 "08:30:00.5"，截断为整数秒。
                    if let Ok(psec) = parts[2].parse::<f64>() { sec = psec.trunc() as u32; }
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
        // 修复 P1-3：下列为运行时“可选调参”（均有内建默认值），在 techniques_meta_data.inc 中
        // 声明仅作契约/文档对齐，不做硬性必填校验。
        // 修复 P1-A(R3)：date/time/year/lat/lon/city/gender/text/school 等核心参数不再无条件白名单，
        // 改由下方 match 分支的真实判定逻辑生效——缺 date 等将被显式拒绝，而非静默默认 2026-09-30。
        // 此处仅保留确实安全、带内建默认值的“可选调参”。
        const OPTIONAL_DECLARED_PARAMS: &[&str] = &[
            "target_date", "after23_new_day", "late_zi_use_next_day",
            "window_minutes", "step_seconds", "lines", "nums", "numbers",
            "dong_yaos", "target_day_offset", "days", "span_days", "timezone_offset",
            "ascendant", "pillars", "year_pillar", "day_gan", "day_zhi",
            "harmonic", "seed", "age", "target_year",
            "year_gz", "hour_gz", "month_gz", "day_gz", "year_gan", "hour_gan",
        ];
        if OPTIONAL_DECLARED_PARAMS.contains(&param) {
            return true;
        }
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
            "numbers" => self.numbers.as_ref().is_some_and(|v| !v.is_empty()) || self.nums.as_ref().is_some_and(|v| !v.is_empty()),
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
            "year_gz" => self.year_gz.is_some() || self.year_pillar.is_some() || self.pillars.as_ref().is_some_and(|p| !p.is_empty()),
            "month_gz" => self.month_gz.is_some() || self.month_pillar.is_some() || self.pillars.as_ref().is_some_and(|p| p.len() >= 2),
            "day_gz" => self.day_gz.is_some() || self.day_pillar.is_some() || self.pillars.as_ref().is_some_and(|p| p.len() >= 3),
            "hour_gz" => self.hour_gz.is_some() || self.hour_pillar.is_some() || self.pillars.as_ref().is_some_and(|p| p.len() >= 4),
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
            "is_lunar" => self.is_lunar.is_some() || self.date.is_some() || (self.year.is_some() && self.month.is_some()),
            "lunar_month" => self.lunar_month.is_some() || self.date.is_some() || self.month.is_some(),
            "lunar_day" => self.lunar_day.is_some() || self.date.is_some() || self.day.is_some(),
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
    // 0. 基础日期时间合法性校验（支持公元前 "-YYYY-MM-DD"，但术数排盘暂不支持则明确报错）
    if let Some(ref date_str) = input.date {
        match parse_date_str(date_str) {
            Some((y, m, d)) => {
                if y < 1 {
                    return Err(format!(
                        "输入日期 [{}] 为公元前年份，当前术数引擎暂不支持公元前排盘（无对应星历/节气表），请改用公元后日期",
                        date_str
                    ));
                }
                if !(1..=12).contains(&m) {
                    return Err(format!("输入日期月份 [{}] 越界，必须在 1 到 12 之间", m));
                }
                let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
                let max_d = match m {
                    2 => if is_leap { 29 } else { 28 },
                    4 | 6 | 9 | 11 => 30,
                    _ => 31,
                };
                if d < 1 || d > max_d {
                    return Err(format!("输入日期天数 [{}] 越界，{} 年 {} 月最大天数为 {} 天", d, y, m, max_d));
                }
            }
            None => {
                return Err(format!("输入日期格式非法: [{}]，必须符合 YYYY-MM-DD 规范", date_str));
            }
        }
    } else if let (Some(y), Some(m), Some(d)) = (input.year, input.month, input.day) {
        if y < 1 {
            return Err(format!(
                "输入年份 [{}] 为公元前年份，当前术数引擎暂不支持公元前排盘，请改用公元后年份",
                y
            ));
        }
        if !(1..=12).contains(&m) {
            return Err(format!("输入月份 [{}] 越界，必须在 1 到 12 之间", m));
        }
        let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
        let max_d = match m {
            2 => if is_leap { 29 } else { 28 },
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };
        if d < 1 || d > max_d {
            return Err(format!("输入天数 [{}] 越界，{} 年 {} 月最大天数为 {} 天", d, y, m, max_d));
        }
    }

    // 0.1 经纬度合法性校验
    if let Some(la) = input.lat {
        if !(-90.0..=90.0).contains(&la) {
            return Err(format!("纬度坐标 [{}] 越界，必须在 -90.0 到 90.0 之间", la));
        }
    }
    if let Some(lo) = input.lon {
        if !(-180.0..=180.0).contains(&lo) {
            return Err(format!("经度坐标 [{}] 越界，必须在 -180.0 到 180.0 之间", lo));
        }
    }

    // 0.15 时区偏移合法性校验（R5-04）：UTC 偏移实际范围约 -12 ~ +14 小时，
    // 越界值（如 999）此前被静默接受，此处显式拒绝。
    if let Some(tz) = input.timezone_offset {
        if !(-14.0..=14.0).contains(&tz) {
            return Err(format!(
                "时区偏移 timezone_offset [{}] 越界，必须在 -14.0 到 14.0 小时之间（相对 UTC）",
                tz
            ));
        }
    }

    // 0.16 时间字符串解析与范围校验（R4-01 / P2-N1）：原 get_datetime 解析失败时静默回退
    // 默认 hour=14，ok:true。现改为结构化拒绝，与 date/gender/tz 严格口径一致。
    if let Some(ref t) = input.time {
        let parts: Vec<&str> = t.split(':').collect();
        if parts.is_empty() || parts.len() > 3 {
            return Err(format!("输入时间格式非法: [{}]，必须符合 HH:MM:SS（可省略 SS）", t));
        }
        let parse_seg = |s: &str, name: &str| -> Result<u32, String> {
            s.parse::<u32>()
                .map_err(|_| format!("输入时间 [{}] 的 {} 段 [{}] 不是合法整数", t, name, s))
        };
        let hh = parse_seg(parts[0], "小时")?;
        let mm = if parts.len() >= 2 { parse_seg(parts[1], "分钟")? } else { 0 };
        // P2-4：秒段对齐 JS，支持小数秒（如 "08:30:00.5"），按 f64 接受、截断为整数秒。
        // P2-N2：收紧秒段格式为 \d+(\.\d+)? —— 必须有整数秒部分；尾点("00.")或缺整数(".5")一律拒绝。
        let sec_format_ok = |s: &str| -> bool {
            let b = s.as_bytes();
            if b.is_empty() {
                return false;
            }
            let mut seen_dot = false;
            let mut int_digit = false;
            let mut frac_digit = false;
            for &c in b {
                match c {
                    b'0'..=b'9' => {
                        if seen_dot { frac_digit = true; } else { int_digit = true; }
                    }
                    b'.' if !seen_dot => seen_dot = true,
                    _ => return false,
                }
            }
            int_digit && (!seen_dot || frac_digit)
        };
        let ss: f64 = if parts.len() >= 3 {
            let s = parts[2];
            if !sec_format_ok(s) {
                return Err(format!(
                    "输入时间 [{}] 的 秒 段 [{}] 格式非法，须为整数秒或小数秒(如 00 / 00.5)，不得尾点或缺整数位",
                    t, s
                ));
            }
            s.parse::<f64>()
                .map_err(|_| format!("输入时间 [{}] 的 秒 段 [{}] 不是合法数字", t, s))?
        } else {
            0.0
        };
        // 严格 0-23 / 0-59 / 0~<60（不放行 24:00，与 lunar-javascript 口径一致）
        if hh > 23 {
            return Err(format!("输入时间小时 [{}] 越界，必须在 0 到 23 之间", hh));
        }
        if mm > 59 {
            return Err(format!("输入时间分钟 [{}] 越界，必须在 0 到 59 之间", mm));
        }
        if !(0.0..60.0).contains(&ss) {
            return Err(format!("输入时间秒 [{}] 越界，必须在 0 到 60 之间", ss));
        }
    }
    // 数值型 hour/minute/second 字段同样做范围校验
    if let Some(hh) = input.hour {
        if hh > 23 {
            return Err(format!("小时参数 hour [{}] 越界，必须在 0 到 23 之间", hh));
        }
    }
    if let Some(mm) = input.minute {
        if mm > 59 {
            return Err(format!("分钟参数 minute [{}] 越界，必须在 0 到 59 之间", mm));
        }
    }
    if let Some(ss) = input.second {
        if ss > 59 {
            return Err(format!("秒参数 second [{}] 越界，必须在 0 到 59 之间", ss));
        }
    }

    // 0.2 性别参数合法性校验（反序列化已归一化为 1/2，此处防御程序化构造的非法值）
    if let Some(g) = input.gender {
        if g != 1 && g != 2 {
            return Err(format!("性别代码 [{}] 非法，必须为 1 (乾造/男) 或 2 (坤造/女)", g));
        }
    }

    // 0.3 自动比对 110 项规范中该工具声明的所有专属参数
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
            let has_pillars = input.pillars.as_ref().is_some_and(|p| p.len() >= 4);
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

        // 9. 年历工具：拒绝公元 0 年（天文年编号中不存在，历表无覆盖）
        "jieqi_year" | "calendar_month" | "jieqi_birth" | "huangli" | "tongshu" => {
            let (y, _, _, _, _, _) = input.get_datetime();
            if y == 0 {
                return Err(format!("技法 [{}] 输入年份为公元 0 年，历史上不存在公元 0 年，请使用公元前 1 年 (-1) 或公元 1 年 (1)", tool));
            }
        }

        // 10. 词表白名单校验（P2-02）：未识别的流派/牌阵/分宫制显式报错，杜绝静默回退
        "babylon" => {
            if let Some(school) = input.school.as_deref() {
                const SCHEMES: [&str; 3] = ["swissA10", "systemA", "systemB"];
                if !SCHEMES.contains(&school) {
                    return Err(format!("技法 [babylon] 流派 school [{}] 不在支持词表内，可选: {}", school, SCHEMES.join("/")));
                }
            }
        }
        "tarot" => {
            if let Some(spread) = input.spread.as_deref() {
                const SPREADS: [&str; 3] = ["single", "three", "three_choice"];
                if !SPREADS.contains(&spread) {
                    return Err(format!("技法 [tarot] 牌阵 spread [{}] 不在支持词表内，可选: {}", spread, SPREADS.join("/")));
                }
            }
        }
        "chart" | "natal" | "transit" | "composite_chart" | "davison" | "relocation" | "chart12" | "chart13" | "keypoints" | "election" | "draconic" | "agepoint" | "distributions" | "solarreturn" | "lunarreturn" | "horary" => {
            if let Some(hsys) = input.hsys.as_deref() {
                const HSYS: [&str; 7] = ["placidus", "whole", "wholesign", "equal", "regiomontanus", "koch", "campanus"];
                if !HSYS.contains(&hsys.to_lowercase().as_str()) {
                    return Err(format!("技法 [{}] 分宫制 hsys [{}] 不在支持词表内，可选: placidus/wholesign/equal/regiomontanus", tool, hsys));
                }
                if hsys.eq_ignore_ascii_case("koch") || hsys.eq_ignore_ascii_case("campanus") {
                    return Err(format!("技法 [{}] 分宫制 [{}] 尚未实现，当前支持: placidus/wholesign/equal/regiomontanus（拒绝静默回退为 Placidus）", tool, hsys));
                }
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

    // P1-5：顶层 `query` 别名归一化——前端/Pydantic schema 用 `query` 作检索关键词字段名。
    // astrodata/xuanshi 检索时 {"query":"X"} 等效于 {"text":"X"}：仅当 text/name 均缺时
    // 把 query 值映射到 text；只对检索类工具生效，不影响其他技法。
    let mut input = input;
    if matches!(tool, "astrodata" | "xuanshi")
        && input.text.is_none()
        && input.name.is_none()
    {
        if let Some(q) = input.query.take() {
            input.text = Some(q);
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
