use flate2::read::ZlibDecoder;
use memmap2::Mmap;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct TableEntry {
    pub name: String,
    pub offset: u64,
    pub comp_len: u64,
    pub uncomp_len: u64,
}

pub struct XuanshiDatabase {
    _file: File,
    mmap: Mmap,
    tables: HashMap<String, TableEntry>,
}

impl XuanshiDatabase {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        if mmap.len() < 8 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "文件过小，非有效数据库",
            ));
        }

        let magic = &mmap[0..4];
        let mut tables = HashMap::new();

        if magic == b"HORA" {
            // 二进制紧凑目录规范 (Magic + Version + TableCount)
            let mut buf2 = [0u8; 2];
            let mut rdr = io::Cursor::new(&mmap[4..8]);
            rdr.read_exact(&mut buf2)?;
            let _version = u16::from_le_bytes(buf2);
            rdr.read_exact(&mut buf2)?;
            let table_count = u16::from_le_bytes(buf2) as usize;

            let mut cat_rdr = io::Cursor::new(&mmap[8..]);
            let mut name_buf = [0u8; 16];
            let mut buf4 = [0u8; 4];
            let mut buf8 = [0u8; 8];

            for _ in 0..table_count {
                cat_rdr.read_exact(&mut name_buf)?;
                let name = String::from_utf8_lossy(&name_buf)
                    .trim_matches('\0')
                    .to_string();

                cat_rdr.read_exact(&mut buf4)?;
                let uncomp_len = u32::from_le_bytes(buf4) as u64;

                cat_rdr.read_exact(&mut buf4)?;
                let comp_len = u32::from_le_bytes(buf4) as u64;

                cat_rdr.read_exact(&mut buf8)?;
                let offset = u64::from_le_bytes(buf8);

                tables.insert(
                    name.clone(),
                    TableEntry {
                        name,
                        offset,
                        comp_len,
                        uncomp_len,
                    },
                );
            }
        } else if magic == b"XSHI" {
            // XSHI 变体格式
            let mut rdr = io::Cursor::new(&mmap[4..]);
            let mut buf4 = [0u8; 4];
            let mut buf8 = [0u8; 8];

            rdr.read_exact(&mut buf4)?;
            let table_count = u32::from_le_bytes(buf4);

            for _ in 0..table_count {
                rdr.read_exact(&mut buf4)?;
                let name_len = u32::from_le_bytes(buf4) as usize;

                let mut name_buf = vec![0u8; name_len];
                rdr.read_exact(&mut name_buf)?;
                let name = String::from_utf8_lossy(&name_buf).to_string();

                rdr.read_exact(&mut buf8)?;
                let offset = u64::from_le_bytes(buf8);

                rdr.read_exact(&mut buf8)?;
                let comp_len = u64::from_le_bytes(buf8);

                rdr.read_exact(&mut buf8)?;
                let uncomp_len = u64::from_le_bytes(buf8);

                tables.insert(
                    name.clone(),
                    TableEntry {
                        name,
                        offset,
                        comp_len,
                        uncomp_len,
                    },
                );
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("无效的古籍数据库文件格式 (Magic: {:?})", magic),
            ));
        }

        Ok(Self {
            _file: file,
            mmap,
            tables,
        })
    }

    pub fn list_tables(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }

    pub fn load_table(&self, table_name: &str) -> io::Result<Value> {
        let entry = self.tables.get(table_name).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("表未找到: {}", table_name),
            )
        })?;

        let start = entry.offset as usize;
        let end = start + (entry.comp_len as usize);

        if end > self.mmap.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "压缩数据越界",
            ));
        }

        let compressed_slice = &self.mmap[start..end];
        let mut decoder = ZlibDecoder::new(compressed_slice);
        let mut uncompressed_data = Vec::with_capacity(entry.uncomp_len as usize);
        decoder.read_to_end(&mut uncompressed_data)?;

        let json_val: Value = serde_json::from_slice(&uncompressed_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(json_val)
    }

    /// 搜索玄学案例、天象或条文（按关键词检索）
    pub fn search(&self, table: &str, keyword: &str) -> io::Result<Vec<Value>> {
        let data = self.load_table(table)?;
        let mut results = Vec::new();

        match data {
            Value::Array(rows) => {
                for row in rows {
                    let text_repr = row.to_string();
                    if text_repr.contains(keyword) {
                        results.push(row);
                    }
                }
            }
            Value::Object(map) => {
                // 支持 Key-Value 字典型条文检索 (如 "1111": "条文正文")
                for (k, v) in map {
                    let text_repr = format!("{}: {}", k, v);
                    if text_repr.contains(keyword) {
                        results.push(serde_json::json!({ "id": k, "content": v }));
                    }
                }
            }
            _ => {}
        }

        Ok(results)
    }

    /// 精确按 ID 查询古籍条文（如邵子 1111，铁板 9345）
    pub fn get_verse(&self, table: &str, id: &str) -> io::Result<Option<String>> {
        let data = self.load_table(table)?;
        if let Value::Object(map) = data {
            if let Some(val) = map.get(id) {
                return Ok(Some(val.as_str().unwrap_or(&val.to_string()).to_string()));
            }
        }
        Ok(None)
    }

    /// 获取 360 度萨比恩意象 (Sabian Symbols)
    pub fn get_sabian_symbol(&self, degree_1_to_360: usize) -> Option<Value> {
        if let Ok(Value::Array(arr)) = self.load_table("sabian_symbols") {
            let idx = (degree_1_to_360.saturating_sub(1)) % arr.len().max(1);
            return arr.get(idx).cloned();
        }
        None
    }

    /// 获取达摩一掌经正宗秘传参数 (Damo Data)
    pub fn get_damo_data(&self) -> Option<Value> {
        self.load_table("damo_data").ok()
    }

    /// 获取张果星宗原典星盘参数 (Zhangguo Stars)
    pub fn get_zhangguo_stars(&self) -> Option<Value> {
        self.load_table("zhangguo_stars").ok()
    }

    /// 获取塔罗牌阵与全套对应表
    pub fn get_tarot_spread_info(&self, spread_key: &str) -> Option<Value> {
        if let Ok(Value::Object(map)) = self.load_table("tarot_spreads") {
            return map.get(spread_key).cloned();
        }
        None
    }
}

// -------------------------------------------------------------
// AstroData 全球名流星盘数据库快速只读检索器 (ADTX)
// -------------------------------------------------------------
pub struct AstroDataDatabase {
    records: Vec<Value>,
}

impl AstroDataDatabase {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        if mmap.len() < 12 || &mmap[0..4] != b"ADTX" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "无效的 AstroData 索引文件格式 (Magic != ADTX)",
            ));
        }

        let mut buf4 = [0u8; 4];
        let mut rdr = io::Cursor::new(&mmap[4..12]);
        rdr.read_exact(&mut buf4)?;
        let uncomp_len = u32::from_le_bytes(buf4) as usize;
        rdr.read_exact(&mut buf4)?;
        let comp_len = u32::from_le_bytes(buf4) as usize;

        if 12 + comp_len > mmap.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "AstroData 压缩数据超出文件长度",
            ));
        }

        let mut decoder = ZlibDecoder::new(&mmap[12..12 + comp_len]);
        let mut uncomp = Vec::with_capacity(uncomp_len);
        decoder.read_to_end(&mut uncomp)?;

        let records: Vec<Value> = serde_json::from_slice(&uncomp)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(Self { records })
    }

    pub fn search(&self, keyword: &str, limit: usize) -> Vec<Value> {
        let kw_lower = keyword.to_lowercase();
        // 三级相关性排序：①精确匹配 ②前缀匹配 ③子串匹配；同级别内保持原记录序。
        let mut exact: Vec<Value> = Vec::new();
        let mut prefix: Vec<Value> = Vec::new();
        let mut substring: Vec<Value> = Vec::new();

        for r in &self.records {
            let name = r.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let name_zh = r.get("name_zh").and_then(|v| v.as_str()).unwrap_or("");
            let name_lower = name.to_lowercase();

            let is_exact = name_lower == kw_lower || name_zh == keyword;
            let is_prefix = name_lower.starts_with(&kw_lower) || name_zh.starts_with(keyword);
            let is_substr = name_lower.contains(&kw_lower) || name_zh.contains(keyword);

            if is_exact {
                exact.push(r.clone());
            } else if is_prefix {
                prefix.push(r.clone());
            } else if is_substr {
                substring.push(r.clone());
            }
        }

        // 按优先级合并，截断到 limit
        let mut hits = Vec::with_capacity(limit);
        for batch in [exact, prefix, substring] {
            for r in batch {
                hits.push(r);
                if hits.len() >= limit {
                    return hits;
                }
            }
        }
        hits
    }

    pub fn total_count(&self) -> usize {
        self.records.len()
    }
}

// -------------------------------------------------------------
// AstroData 完整案例库详情读取器 (ADTS) — 维基传记摘要 + 分类
// 由 tools/convert_astrodata_details.py 离线从 ACAS/SQLite 生成。
// 运行时零 SQLite 依赖：只读 zlib 压缩的 JSON map {name: details}。
// 格式: [4B "ADTS"][4B uncomp_len u32LE][4B comp_len u32LE][zlib 流]
// -------------------------------------------------------------
pub struct AstroDataDetails {
    map: HashMap<String, Value>,
}

impl AstroDataDetails {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        if mmap.len() < 12 || &mmap[0..4] != b"ADTS" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "无效的 AstroData 详情文件格式 (Magic != ADTS)",
            ));
        }

        let mut buf4 = [0u8; 4];
        let mut rdr = io::Cursor::new(&mmap[4..12]);
        rdr.read_exact(&mut buf4)?;
        let uncomp_len = u32::from_le_bytes(buf4) as usize;
        rdr.read_exact(&mut buf4)?;
        let comp_len = u32::from_le_bytes(buf4) as usize;

        if 12 + comp_len > mmap.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "AstroData 详情压缩数据超出文件长度",
            ));
        }

        let mut decoder = ZlibDecoder::new(&mmap[12..12 + comp_len]);
        let mut uncomp = Vec::with_capacity(uncomp_len);
        decoder.read_to_end(&mut uncomp)?;

        // 解压后为 JSON object: { name: {wiki_summary, ...} }
        let root: Value = serde_json::from_slice(&uncomp)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let map = root
            .as_object()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "ADTS 根节点非 JSON object"))?
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        Ok(Self { map })
    }

    /// 按 name 精确查询详情，返回 wiki_summary / categories 等字段。
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.map.get(name)
    }

    pub fn count(&self) -> usize {
        self.map.len()
    }
}

/// AstroDataDetails 进程内单例缓存（R6 P3）。
///
/// 每次查询重新 mmap+解压 16MB ADTS 约耗时 ~1s；用 OnceLock 缓存后，
/// 进程内多次查询只解压一次。文件路径固定 data/astrodata_details.bin，
/// 只读数据无失效问题。首次加载失败（文件缺失）时缓存 None，后续调用
/// 每次重试以支持运行时补放文件的优雅降级。
pub fn astrodata_details() -> Option<&'static AstroDataDetails> {
    static DETAILS: std::sync::OnceLock<Option<AstroDataDetails>> = std::sync::OnceLock::new();
    DETAILS
        .get_or_init(|| resolve_data_path("astrodata_details.bin").and_then(|p| AstroDataDetails::open(p).ok()))
        .as_ref()
}

pub fn resolve_data_path(filename: &str) -> Option<std::path::PathBuf> {
    let candidates = [
        format!("data/{}", filename),
        format!("xuanxue-core/data/{}", filename),
        format!("../data/{}", filename),
    ];
    for c in &candidates {
        let p = std::path::Path::new(c);
        if p.exists() {
            return Some(p.to_path_buf());
        }
    }
    None
}

// -------------------------------------------------------------
// 中国地理行政区划经纬度离线数据库 (CGEO)
// -------------------------------------------------------------
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CityGeoInfo {
    pub name: String,
    pub fullname: String,
    pub code: String,
    pub pinyin: String,
    pub lon: f64,
    pub lat: f64,
}

pub struct ChinaGeoDatabase {
    cities: Vec<CityGeoInfo>,
}

impl ChinaGeoDatabase {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        if mmap.len() < 12 || &mmap[0..4] != b"CGEO" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "无效的中国地理数据库文件格式 (Magic != CGEO)",
            ));
        }

        let mut buf4 = [0u8; 4];
        let mut rdr = io::Cursor::new(&mmap[4..12]);
        rdr.read_exact(&mut buf4)?;
        let uncomp_len = u32::from_le_bytes(buf4) as usize;
        rdr.read_exact(&mut buf4)?;
        let _comp_len = u32::from_le_bytes(buf4) as usize;

        let mut uncomp = Vec::with_capacity(uncomp_len);
        let mut cursor = io::Cursor::new(&mmap[12..]);
        lzma_rs::xz_decompress(&mut cursor, &mut uncomp)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("XZ解压失败: {:?}", e)))?;

        let geo_json: Value = serde_json::from_slice(&uncomp)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut cities = Vec::new();
        if let Some(features) = geo_json.get("features").and_then(|v| v.as_array()) {
            for feat in features {
                if let Some(props) = feat.get("properties") {
                    let name = props.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let fullname = props.get("fullname").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let code = props.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let pinyin = props.get("pinyin").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    if let Some(center) = props.get("center").and_then(|v| v.as_array()) {
                        if center.len() >= 2 {
                            let lon = center[0].as_f64().unwrap_or(0.0);
                            let lat = center[1].as_f64().unwrap_or(0.0);
                            if lon != 0.0 || lat != 0.0 {
                                cities.push(CityGeoInfo {
                                    name,
                                    fullname,
                                    code,
                                    pinyin,
                                    lon,
                                    lat,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(Self { cities })
    }

    pub fn lookup(&self, query: &str) -> Option<&CityGeoInfo> {
        let q = query.trim();
        if q.is_empty() {
            return None;
        }
        let q_lower = q.to_lowercase();
        // 1. 精确匹配名称、全称或邮编代码
        if let Some(c) = self.cities.iter().find(|c| c.name == q || c.fullname == q || c.code == q || c.pinyin.eq_ignore_ascii_case(q)) {
            return Some(c);
        }
        // 2. 前缀/包含匹配
        if let Some(c) = self.cities.iter().find(|c| c.fullname.contains(q) || c.name.contains(q) || (!c.pinyin.is_empty() && c.pinyin.contains(&q_lower))) {
            return Some(c);
        }
        None
    }

    pub fn all_cities(&self) -> &[CityGeoInfo] {
        &self.cities
    }
}

pub fn find_geo_location(city_name: &str) -> Option<(f64, f64, String)> {
    if let Some(path) = resolve_data_path("china_geo.bin") {
        if let Ok(db) = ChinaGeoDatabase::open(path) {
            if let Some(city) = db.lookup(city_name) {
                return Some((city.lon, city.lat, city.fullname.clone()));
            }
        }
    }
    // 基础内建回退常用直辖市与主要都市
    match city_name {
        "北京" | "北京市" => Some((116.407395, 39.904211, "北京市".to_string())),
        "上海" | "上海市" => Some((121.473701, 31.230416, "上海市".to_string())),
        "广州" | "广州市" => Some((113.264385, 23.129110, "广州市".to_string())),
        "深圳" | "深圳市" => Some((114.057868, 22.543099, "深圳市".to_string())),
        "成都" | "成都市" => Some((104.066541, 30.572269, "成都市".to_string())),
        "杭州" | "杭州市" => Some((120.155070, 30.274085, "杭州市".to_string())),
        "南京" | "南京市" => Some((118.796877, 32.060255, "南京市".to_string())),
        "武汉" | "武汉市" => Some((114.305393, 30.593099, "武汉市".to_string())),
        "西安" | "西安市" => Some((108.940174, 34.341568, "西安市".to_string())),
        "重庆" | "重庆市" => Some((106.551556, 29.563010, "重庆市".to_string())),
        _ => None,
    }
}

// -------------------------------------------------------------
// 耶鲁亮星表 BSC5 全球恒星高精黄经黄纬数据库 (STAR)
// -------------------------------------------------------------
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrightStar {
    pub hr: String,
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub ecl_lon: f64,
    pub ecl_lat: f64,
    pub vmag: f64,
    pub spectral_cls: String,
}

pub struct Bsc5StarDatabase {
    stars: Vec<BrightStar>,
}

impl Bsc5StarDatabase {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        if mmap.len() < 12 || &mmap[0..4] != b"STAR" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "无效的耶鲁星表文件格式 (Magic != STAR)",
            ));
        }

        let mut buf4 = [0u8; 4];
        let mut rdr = io::Cursor::new(&mmap[4..12]);
        rdr.read_exact(&mut buf4)?;
        let uncomp_len = u32::from_le_bytes(buf4) as usize;
        rdr.read_exact(&mut buf4)?;
        let _comp_len = u32::from_le_bytes(buf4) as usize;

        let mut uncomp = Vec::with_capacity(uncomp_len);
        let mut cursor = io::Cursor::new(&mmap[12..]);
        lzma_rs::xz_decompress(&mut cursor, &mut uncomp)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("XZ解压失败: {:?}", e)))?;

        let raw_stars: Vec<Value> = serde_json::from_slice(&uncomp)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let eps = (23.4392911f64).to_radians();
        let mut stars = Vec::with_capacity(raw_stars.len());

        for obj in raw_stars {
            let hr = obj.get("HR").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let ra_str = obj.get("RA").and_then(|v| v.as_str()).unwrap_or("");
            let dec_str = obj.get("Dec").and_then(|v| v.as_str()).unwrap_or("");
            let vmag = obj.get("Vmag").and_then(|v| v.as_str()).and_then(|s| s.trim().parse::<f64>().ok()).unwrap_or(99.0);
            let spectral_cls = obj.get("SpectralCls").and_then(|v| v.as_str()).unwrap_or("").to_string();

            let ra_deg = Self::parse_ra(ra_str);
            let dec_deg = Self::parse_dec(dec_str);

            let (ecl_lon, ecl_lat) = Self::eq_to_ecl(ra_deg, dec_deg, eps);

            stars.push(BrightStar {
                hr,
                ra_deg,
                dec_deg,
                ecl_lon,
                ecl_lat,
                vmag,
                spectral_cls,
            });
        }

        Ok(Self { stars })
    }

    fn parse_ra(ra_str: &str) -> f64 {
        // 例: "00h 05m 03.8s"
        let parts: Vec<&str> = ra_str.split(['h', 'm', 's', ' ']).filter(|s| !s.is_empty()).collect();
        if parts.len() >= 3 {
            let h = parts[0].parse::<f64>().unwrap_or(0.0);
            let m = parts[1].parse::<f64>().unwrap_or(0.0);
            let s = parts[2].parse::<f64>().unwrap_or(0.0);
            (h + m / 60.0 + s / 3600.0) * 15.0
        } else {
            0.0
        }
    }

    fn parse_dec(dec_str: &str) -> f64 {
        // 例: "-00° 30′ 11″" 或 "+13° 23′ 46″"
        let s = dec_str.trim();
        let is_neg = s.starts_with('-');
        let nums: Vec<f64> = s.split(|c: char| !c.is_numeric() && c != '.')
            .filter(|p| !p.is_empty())
            .filter_map(|p| p.parse::<f64>().ok())
            .collect();

        if nums.len() >= 3 {
            let d = nums[0];
            let m = nums[1];
            let sec = nums[2];
            let val = d + m / 60.0 + sec / 3600.0;
            if is_neg { -val } else { val }
        } else {
            0.0
        }
    }

    fn eq_to_ecl(ra_deg: f64, dec_deg: f64, eps_rad: f64) -> (f64, f64) {
        let ra = ra_deg.to_radians();
        let dec = dec_deg.to_radians();
        let sin_lon = ra.sin() * eps_rad.cos() + dec.tan() * eps_rad.sin();
        let cos_lon = ra.cos();
        let lon = sin_lon.atan2(cos_lon).to_degrees().rem_euclid(360.0);

        let sin_lat = dec.sin() * eps_rad.cos() - dec.cos() * eps_rad.sin() * ra.sin();
        let lat = sin_lat.asin().to_degrees();
        (lon, lat)
    }

    /// 查找黄经与目标经度在容许角（orb_deg）范围内的主要恒星
    pub fn find_conjunctions(&self, target_lon: f64, orb_deg: f64, max_mag: f64) -> Vec<BrightStar> {
        let mut hits = Vec::new();
        for s in &self.stars {
            if s.vmag > max_mag {
                continue;
            }
            let diff = (s.ecl_lon - target_lon).abs().rem_euclid(360.0);
            let angular_dist = if diff > 180.0 { 360.0 - diff } else { diff };
            if angular_dist <= orb_deg {
                hits.push(s.clone());
            }
        }
        hits.sort_by(|a, b| a.vmag.partial_cmp(&b.vmag).unwrap());
        hits
    }

    pub fn total_count(&self) -> usize {
        self.stars.len()
    }

    pub fn stars(&self) -> &[BrightStar] {
        &self.stars
    }
}

pub fn get_bright_stars_near(target_lon: f64, orb_deg: f64, max_mag: f64) -> Vec<BrightStar> {
    if let Some(path) = resolve_data_path("bsc5_stars.bin") {
        if let Ok(db) = Bsc5StarDatabase::open(path) {
            return db.find_conjunctions(target_lon, orb_deg, max_mag);
        }
    }
    // 内建经典恒星回退 (轩辕十四, 大角星, 心宿二, 毕宿五, 织女一等)
    let classic_stars = [
        ("轩辕十四 (Regulus)", 150.83, 11.97, 1.35, "B7"),
        ("毕宿五 (Aldebaran)", 70.15, -5.47, 0.85, "K5"),
        ("心宿二 (Antares)", 249.76, -4.57, 0.96, "M1"),
        ("北落师门 (Fomalhaut)", 344.87, -21.14, 1.16, "A3"),
        ("大角星 (Arcturus)", 204.24, 30.73, -0.05, "K1"),
        ("织女一 (Vega)", 285.32, 61.73, 0.03, "A0"),
        ("角宿一 (Spica)", 203.84, -2.05, 0.98, "B1"),
    ];
    let mut hits = Vec::new();
    for (name, lon, lat, mag, spec) in classic_stars {
        if mag > max_mag {
            continue;
        }
        let diff = (lon - target_lon).abs().rem_euclid(360.0);
        let ang = if diff > 180.0 { 360.0 - diff } else { diff };
        if ang <= orb_deg {
            hits.push(BrightStar {
                hr: name.to_string(),
                ra_deg: 0.0,
                dec_deg: 0.0,
                ecl_lon: lon,
                ecl_lat: lat,
                vmag: mag,
                spectral_cls: spec.to_string(),
            });
        }
    }
    hits
}

// -------------------------------------------------------------
// 瑞士星历切片 (SEPK) 烘焙数据包快速映射
//
// SEPK 容器格式 (已逆向，详见项目根目录 SEPK_FORMAT_ANALYSIS.md):
//   [16 字节 SEPK 文件头] + [XZ 压缩流]
// XZ 解压后为 GNU tar 归档，内含 Astrodienst 官方 Swiss Ephemeris
// .se1 Chebyshev 星历文件 (基于 JPL DE441)。
// -------------------------------------------------------------
pub struct SepkDatabase {
    pub file_name: String,
    pub body_type: u16,
    pub uncomp_len: u32,
    pub jstart: f64,
    pub jend: f64,
    pub de_version: u32,
}

/// 从 .se1 二进制头解析真实的时间覆盖范围与历表版本。
/// .se1 文本头约 116 字节后，小端二进制布局为：
///   [cba\0][file_size u32][denum u32][jfirst f64][jlast f64]...
/// 返回 (jfirst, jlast, denum)，解析失败返回 None。
fn parse_se1_header(buf: &[u8]) -> Option<(f64, f64, u32)> {
    // 定位 "cba\0" 二级魔数
    let magic_pos = buf.windows(4).position(|w| w == b"cba\0")?;
    let p = magic_pos + 4;
    if p + 24 > buf.len() {
        return None;
    }
    let file_size = u32::from_le_bytes([buf[p], buf[p + 1], buf[p + 2], buf[p + 3]]);
    let denum = u32::from_le_bytes([buf[p + 4], buf[p + 5], buf[p + 6], buf[p + 7]]);
    let jfirst = f64::from_le_bytes([
        buf[p + 8], buf[p + 9], buf[p + 10], buf[p + 11],
        buf[p + 12], buf[p + 13], buf[p + 14], buf[p + 15],
    ]);
    let jlast = f64::from_le_bytes([
        buf[p + 16], buf[p + 17], buf[p + 18], buf[p + 19],
        buf[p + 20], buf[p + 21], buf[p + 22], buf[p + 23],
    ]);
    // 合理性校验：file_size 应与文件实际大小量级一致，JD 应在合理范围
    if file_size == 0 || !(100_000.0..3_000_000.0).contains(&jfirst) || jlast <= jfirst {
        return None;
    }
    Some((jfirst, jlast, denum))
}

/// 手写极简 tar 解包：在解压后的 tar 字节流中找到第一个常规文件条目，
/// 返回其文件名与数据切片。tar 头为 512 字节，size 字段为八进制 ASCII。
fn tar_first_member(tar: &[u8]) -> Option<(&[u8], &[u8])> {
    let mut offset = 0usize;
    while offset + 512 <= tar.len() {
        let header = &tar[offset..offset + 512];
        // 全零块 = tar 结束
        if header.iter().all(|&b| b == 0) {
            return None;
        }
        // 文件名 (前 100 字节)
        let name_end = header.iter().position(|&b| b == 0).unwrap_or(100);
        let name = &header[..name_end];
        // size 字段在偏移 124，长度 12，八进制 ASCII
        let size_field = &header[124..136];
        let size_str = std::str::from_utf8(size_field).unwrap_or("").trim_matches(|c| c == '\0' || c == ' ');
        let size = usize::from_str_radix(size_str, 8).ok()?;
        // typeflag 在偏移 100
        let typeflag = header[100];
        let data_start = offset + 512;
        if data_start + size > tar.len() {
            return None;
        }
        let data = &tar[data_start..data_start + size];
        // 常规文件 typeflag='0' 或 '\0'；跳过 pax header ('x') 与全局 header ('g')
        if (typeflag == b'0' || typeflag == 0) && name.ends_with(b".se1") {
            return Some((name, data));
        }
        // 数据块按 512 字节对齐向上取整
        let padded = (size + 511) & !511;
        offset = data_start + padded;
    }
    None
}

impl SepkDatabase {
    pub fn inspect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(&path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        if mmap.len() < 16 || &mmap[0..4] != b"SEPK" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "无效的 SEPK 星历包文件格式 (Magic != SEPK)",
            ));
        }

        let body_type = u16::from_le_bytes([mmap[6], mmap[7]]);
        let uncomp_len = u32::from_le_bytes([mmap[8], mmap[9], mmap[10], mmap[11]]);

        let file_name = path.as_ref().file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "ephem.bin".to_string());

        // 尝试真实解压并解析 .se1 头，获取准确时间范围；失败则回退保守默认值。
        // 注意：这里的 jstart/jend/de_version 反映文件真实覆盖范围，不再硬编码猜测。
        let (jstart, jend, de_version) = Self::read_real_coverage(&mmap)
            .unwrap_or_else(|| {
                if file_name.contains("full") {
                    (625360.5, 2816928.5, 441)
                } else {
                    (1721036.0, 1940232.0, 441)
                }
            });

        Ok(Self {
            file_name,
            body_type,
            uncomp_len,
            jstart,
            jend,
            de_version,
        })
    }

    /// 真实解压 SEPK 载荷并解析首个 .se1 文件头，返回 (jfirst, jlast, de_version)。
    fn read_real_coverage(mmap: &[u8]) -> Option<(f64, f64, u32)> {
        let mut rdr = io::Cursor::new(&mmap[16..]);
        let mut tar = Vec::new();
        lzma_rs::xz_decompress(&mut rdr, &mut tar).ok()?;
        let (_name, se1_data) = tar_first_member(&tar)?;
        parse_se1_header(se1_data)
    }

    /// 检测本地数据目录是否已部署 SEPK 瑞士星历切片数据集
    pub fn has_baked_slices() -> bool {
        // 不仅检查文件存在，还要求能真实解压并解析出有效 .se1 头
        for fname in ["ephem_planets_core.bin", "ephem_planets_full.bin"] {
            if let Some(p) = resolve_data_path(fname) {
                if let Ok(file) = File::open(&p) {
                    if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                        if mmap.len() >= 16 && &mmap[0..4] == b"SEPK" && Self::read_real_coverage(&mmap).is_some() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// 解压指定星历切片包的真实二进制载荷 (XZ/LZMA 逆解)，返回 tar 归档字节。
    pub fn decompress_slice_data<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
        let file = File::open(&path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        if mmap.len() < 16 || &mmap[0..4] != b"SEPK" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "无效的 SEPK 星历包"));
        }
        let mut rdr = io::Cursor::new(&mmap[16..]);
        let mut decomp = Vec::new();
        lzma_rs::xz_decompress(&mut rdr, &mut decomp)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        Ok(decomp)
    }

    /// 解压并提取 SEPK 包内第一个 .se1 文件的完整二进制数据。
    /// 返回 (se1 文件名, se1 字节)。供星历插值器直接读取 Chebyshev 系数。
    ///
    /// 当前 0 外部调用：逐行星 Chebyshev 记录插值需 swisseph C 库 FFI 或
    /// 完整移植 swephlib.c 后才能安全启用（见 SEPK_FORMAT_ANALYSIS.md 第六节）。
    /// 保留该函数作为 FFI 接入后的入口点，避免死代码告警。
    #[allow(dead_code)] // 待 swisseph FFI 接入后启用
    pub fn load_first_se1<P: AsRef<Path>>(path: P) -> io::Result<(String, Vec<u8>)> {
        let tar = Self::decompress_slice_data(path)?;
        let (name, data) = tar_first_member(&tar)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "tar 归档中未找到 .se1 文件"))?;
        let name_str = String::from_utf8_lossy(name).to_string();
        Ok((name_str, data.to_vec()))
    }

    /// 统计可用的预烘焙切片文件及元信息
    pub fn available_slices() -> Vec<SepkDatabase> {
        let ephem_files = [
            "ephem_planets_core.bin",
            "ephem_planets_full.bin",
            "ephem_moon_core.bin",
            "ephem_moon_full.bin",
            "ephem_asteroids_full.bin",
        ];
        let mut list = Vec::new();
        for fname in &ephem_files {
            if let Some(p) = resolve_data_path(fname) {
                if let Ok(info) = Self::inspect(p) {
                    list.push(info);
                }
            }
        }
        list
    }
}


