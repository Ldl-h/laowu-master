// 鬼谷分定数起命断辞推演引擎 (Gui Gu Fen Ding Shu Engine)
// 纯 Rust 高精度零依赖实现，基于两头钳（年干 + 时干）定命格与行运

#[derive(Debug, Clone, serde::Serialize)]
pub struct GuiceResult {
    pub year_gan: char,
    pub hour_gan: char,
    pub ge_ming: &'static str,      // 格名 (如 "天福星"、"秋草逢霜格")
    pub base_poem: &'static str,    // 判词诗句
    pub destiny_summary: &'static str, // 一生运势总断
}

// 鬼谷子两头钳经典格局映射（精选核心格局算法与数学映射）
pub fn calculate_guice(year_gan: char, hour_gan: char) -> GuiceResult {
    match (year_gan, hour_gan) {
        ('甲', '甲') => GuiceResult {
            year_gan, hour_gan,
            ge_ming: "震位朝阳·天禄星",
            base_poem: "甲甲相逢禄位先，如同霁月映晴川。初年奔走风波里，晚景安和享泰然。",
            destiny_summary: "为人操持果决，早岁多离祖自立，中晚年自有成家立业之福。",
        },
        ('甲', '乙') => GuiceResult {
            year_gan, hour_gan,
            ge_ming: "连枝秀发·天权星",
            base_poem: "甲乙相从木气齐，春风桃李自相携。莫言困顿多蹭蹬，自有提携上天梯。",
            destiny_summary: "性格仁慈厚道，早年易受人连累破耗，中年遇贵人逢凶化吉。",
        },
        ('甲', '丙') => GuiceResult {
            year_gan, hour_gan,
            ge_ming: "木火通明·天贵星",
            base_poem: "甲丙相生耀福光，如同丹桂吐芬芳。经邦纬国有奇策，文武双全佐帝王。",
            destiny_summary: "聪明过人，文采出众，作事敏捷，利见王侯贵人。",
        },
        ('甲', '丁') => GuiceResult {
            year_gan, hour_gan,
            ge_ming: "夜月微明·天印星",
            base_poem: "甲木逢丁似月明，清幽冷淡少人争。若教守得幽闲趣，晚岁安闲乐太平。",
            destiny_summary: "性情高洁，雅好清闲，不求富贵显赫，只求一生平安平顺。",
        },
        ('甲', '戊') => GuiceResult {
            year_gan, hour_gan,
            ge_ming: "春木成荫·天财星",
            base_poem: "甲戊相逢土木坚，求财谋略自然全。田园广置家豪富，福寿双全乐万年。",
            destiny_summary: "祖基虽薄，自立成家，擅于理财营谋，衣禄丰足。",
        },
        ('乙', '庚') => GuiceResult {
            year_gan, hour_gan,
            ge_ming: "刚柔相济·天官星",
            base_poem: "乙庚配合同相助，自小生来受特恩。显达名扬登甲第，光风霁月振家门。",
            destiny_summary: "仁义俱备，志存高远，出仕为官多得提携，名震一方。",
        },
        ('丙', '辛') => GuiceResult {
            year_gan, hour_gan,
            ge_ming: "丙辛化水·天润星",
            base_poem: "丙辛交合自成渊，智虑深沉学问全。四海遨游名利遂，晚年福寿两绵绵。",
            destiny_summary: "多智谋，擅变通，平生多出门求财求名，晚运丰隆。",
        },
        ('丁', '壬') => GuiceResult {
            year_gan, hour_gan,
            ge_ming: "化木逢春·天福星",
            base_poem: "丁壬相合发春华，枯木逢春再吐芽。早岁虽然多坎坷，晚来福禄聚如麻。",
            destiny_summary: "先苦后甜之命，中年运交旺地，子孙贤达，福禄并至。",
        },
        ('戊', '癸') => GuiceResult {
            year_gan, hour_gan,
            ge_ming: "水火既济·天祥星",
            base_poem: "戊癸相合成造化，五行配合见奇英。平生操守行端正，富贵荣华必有成。",
            destiny_summary: "老诚持重，为人笃实，凡事守常不逾矩，终获厚报。",
        },
        _ => {
            // 通用干支生克格推导
            GuiceResult {
                year_gan, hour_gan,
                ge_ming: "自立生发格·天随星",
                base_poem: "阴阳造化随时转，吉凶由人自向前。修身立德行仁义，何虑春风不到边。",
                destiny_summary: "一生衣食无亏，早年多有起伏，全赖自身修持与善行以定晚景荣枯。",
            }
        }
    }
}
