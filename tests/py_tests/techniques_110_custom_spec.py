# -*- coding: utf-8 -*-
"""
110项玄学占卜技法专属入参与命理逻辑严格校验规范
针对每一项占卜技法，量身定制专属的生辰、干支、卦数、牌阵、行星推进或择吉参数，
并对其命理产出进行真实的逻辑与数学合理性校验。
"""


TECHNIQUES_110_CUSTOM_SPEC = [
    # =========================================================================
    # 一、东方术数与正统三式 (9项)
    # =========================================================================
    {
        "id": 1,
        "tool": "bazi",
        "category": "东方术数与三式",
        "name_zh": "高精八字四柱干支排盘",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "gender": 1,
            "after23_new_day": True,
            "late_zi_use_next_day": True
        },
        "description": "公历1998年10月24日辰时排四柱八字，验证干支平黄经交节",
        "validate": lambda d: (
            d.get("year_pillar") == "戊寅" and
            d.get("month_pillar") == "壬戌" and
            d.get("day_pillar") == "甲辰" and
            d.get("hour_pillar") == "戊辰" and
            0.0 <= d.get("sun_lon", -1) <= 360.0
        ),
        "expect_desc": "准确排出戊寅年壬戌月甲辰日戊辰时四柱干支"
    },
    {
        "id": 2,
        "tool": "bazi_inverse",
        "category": "东方术数与三式",
        "name_zh": "八字四柱干支逆推公历生辰",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰",
            "start_year": 1990,
            "end_year": 2010
        },
        "description": "输入戊寅年壬戌月甲辰日戊辰时，逆推公历候选生辰",
        "validate": lambda d: (
            isinstance(d.get("candidates"), list) and len(d["candidates"]) >= 1 and
            d["candidates"][0].get("year") == 1998 and
            d["candidates"][0].get("month") == 10 and
            d["candidates"][0].get("day") == 24
        ),
        "expect_desc": "成功逆推出1998年10月24日历史真实公历时刻"
    },
    {
        "id": 3,
        "tool": "ziwei",
        "category": "东方术数与三式",
        "name_zh": "紫微斗数安星命盘与十二宫",
        "input": {
            "is_lunar": True,
            "lunar_month": 9,
            "lunar_day": 5,
            "hour": 8,
            "gender": 1
        },
        "description": "农历九月初五辰时乾造，安命身宫起十四主星与五行局",
        "validate": lambda d: (
            isinstance(d.get("ming_palace_zhi"), str) and
            isinstance(d.get("shen_palace_zhi"), str) and
            isinstance(d.get("wuxing_ju"), str) and "局" in d["wuxing_ju"] and
            isinstance(d.get("palaces"), list) and len(d["palaces"]) == 12
        ),
        "expect_desc": "安立命身宫、定五行局、输出标准十二宫盘"
    },
    {
        "id": 4,
        "tool": "qimen",
        "category": "东方术数与三式",
        "name_zh": "奇门遁甲转盘时家奇门局",
        "input": {
            "year": 2026,
            "month": 6,
            "day": 21,
            "hour": 11,
            "minute": 30,
            "second": 0,
            "pai_pan_type": 1
        },
        "description": "夏至节气后时家奇门阴遁排盘，九宫八门八神值符值使",
        "validate": lambda d: (
            d.get("dun_type") in ["阴遁", "阳遁"] and
            isinstance(d.get("leader_star"), str) and
            isinstance(d.get("leader_door"), str) and
            isinstance(d.get("palaces"), list) and len(d["palaces"]) == 9
        ),
        "expect_desc": "定阴阳遁局，确立值符值使门星与九宫飞泊"
    },
    {
        "id": 5,
        "tool": "taiyi",
        "category": "东方术数与三式",
        "name_zh": "太乙神数天目客算主算局",
        "input": {
            "year": 2026,
            "month": 2,
            "day": 4,
            "hour": 15,
            "style": 3
        },
        "description": "立春日申时太乙时计七十二局立成，定主算客算",
        "validate": lambda d: (
            isinstance(d.get("kook_name"), str) and
            isinstance(d.get("home_calc"), int) and
            isinstance(d.get("away_calc"), int) and
            isinstance(d.get("home_general"), int) and
            isinstance(d.get("away_general"), int)
        ),
        "expect_desc": "立局成功，主算与客算数理清晰，主将客将得位"
    },
    {
        "id": 6,
        "tool": "jinkou",
        "category": "东方术数与三式",
        "name_zh": "大六壬金口诀四位推命",
        "input": {
            "day_gan": "庚",
            "hour_zhi": 4,      # 辰时 (4)
            "yue_jiang": 2,     # 登明亥 (2)
            "di_fen": 6         # 午地分 (6)
        },
        "description": "庚日辰时登明将午地分，立人元贵神将神地分四位立式",
        "validate": lambda d: (
            isinstance(d.get("ren_yuan"), dict) and
            isinstance(d.get("gui_shen"), dict) and
            isinstance(d.get("jiang_shen"), dict) and
            isinstance(d.get("di_fen"), dict) and
            isinstance(d.get("dong_yao"), list)
        ),
        "expect_desc": "人元、贵神、将神、地分四位完整且判定生克动爻"
    },
    {
        "id": 7,
        "tool": "liureng",
        "category": "东方术数与三式",
        "name_zh": "大六壬正宗三传四课九宗门",
        "input": {
            "yue_jiang": 6,     # 巳月将 (太乙)
            "zhan_shi": 0,      # 子时占
            "day_gan": "丙",
            "day_zhi": 4        # 辰日 (4)
        },
        "description": "丙辰日巳将子时大六壬占课，起天地盘、四课与三传发端",
        "validate": lambda d: (
            isinstance(d.get("si_ke"), dict) and len(d["si_ke"]) >= 4 and
            isinstance(d.get("san_chuan"), dict) and "chu_chuan" in d["san_chuan"] and
            isinstance(d.get("tian_pan"), list) and len(d["tian_pan"]) == 12
        ),
        "expect_desc": "天盘十二支就位，四课排定，初中末三传发端成课"
    },
    {
        "id": 8,
        "tool": "liureng_runyear",
        "category": "东方术数与三式",
        "name_zh": "大六壬男女行年神煞推运",
        "input": {
            "age": 36.0,
            "gender": 1
        },
        "description": "男命三十六岁行年神煞轮转推算",
        "validate": lambda d: (
            isinstance(d, list) and len(d) >= 1 and
            any(item.get("age") == 36 and item.get("xing_nian_zhi") is not None for item in d)
        ),
        "expect_desc": "返回行年吉凶神煞分布列表"
    },
    {
        "id": 9,
        "tool": "sanshiunited",
        "category": "东方术数与三式",
        "name_zh": "太乙奇门六壬三式合一",
        "input": {
            "year": 2026,
            "month": 8,
            "day": 8,
            "hour": 9
        },
        "description": "天地人三才综合枢纽盘联动，同时触发太乙天道、奇门地道、六壬人道",
        "validate": lambda d: (
            isinstance(d.get("bazi"), dict) and d["bazi"].get("year_pillar") == "丙午" and
            isinstance(d.get("taiyi"), dict) and "kook_name" in d["taiyi"] and
            isinstance(d.get("qimen"), dict) and "dun_type" in d["qimen"] and
            isinstance(d.get("liureng"), dict) and "san_chuan" in d["liureng"]
        ),
        "expect_desc": "同契输出四柱八字、太乙局、奇门遁甲盘与大六壬课"
    },

    # =========================================================================
    # 二、西洋占星与古典流派 (19项)
    # =========================================================================
    {
        "id": 10,
        "tool": "chart",
        "category": "西洋古典与现代占星",
        "name_zh": "本命占星盘 (Natal Chart) 与萨比恩度数",
        "input": {
            "date": "1995-05-18",
            "time": "15:45:00",
            "lat": 39.9042,     # 北京
            "lon": 116.4074,
            "hsys": "placidus"
        },
        "description": "北京生辰本命星盘排盘，普拉西德分宫制，解析十大行星与上升天顶",
        "validate": lambda d: (
            0.0 <= d.get("ascendant", -1) <= 360.0 and
            0.0 <= d.get("mc", -1) <= 360.0 and
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 10 and
            isinstance(d.get("houses"), list) and len(d["houses"]) == 12
        ),
        "expect_desc": "输出精准ASC/MC、十大行星黄道经度及12宫头坐标"
    },
    {
        "id": 11,
        "tool": "transit",
        "category": "西洋古典与现代占星",
        "name_zh": "流年行运盘 (Transit Chart)",
        "input": {
            "date": "2026-10-01",
            "time": "12:00:00",
            "lat": 51.5074,
            "lon": -0.1278,
            "hsys": "placidus"
},
        "description": "伦敦上空即时行运行星分布与四轴交角",
        "validate": lambda d: (
            d.get("technique") == "transit" and
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 10
        ),
        "expect_desc": "科赫分宫制下流年十大天体实时黄道坐标"
    },
    {
        "id": 12,
        "tool": "synastry",
        "category": "西洋古典与现代占星",
        "name_zh": "双人占星比较合盘 (Synastry)",
        "input": {
            "date": "2026-02-14",
            "time": "20:00:00",
            "lat": 40.7128,
            "lon": -74.006,
            "hsys": "placidus",
            "target_date": "1998-10-24"
},
        "description": "情人节双人比较盘星象互映与宫位叠合",
        "validate": lambda d: (
            d.get("technique") == "synastry" and
            isinstance(d.get("chart1"), dict) and
            isinstance(d.get("chart2"), dict) and
            isinstance(d.get("synastry_pairs"), list)
        ),
        "expect_desc": "输出合盘参照宫位与天体落宫坐标"
    },
    {
        "id": 13,
        "tool": "composite_chart",
        "category": "西洋古典与现代占星",
        "name_zh": "双人组合中点盘 (Composite Chart)",
        "input": {
            "date": "2026-05-20",
            "time": "13:14:00",
            "lat": 31.2304,
            "lon": 121.4737,
            "target_date": "1998-10-24"
},
        "description": "组合盘中点计算，揭示关系本质能量",
        "validate": lambda d: (
            d.get("technique") == "composite_chart" and
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 10
        ),
        "expect_desc": "成功合成组合天体黄道中点度数"
    },
    {
        "id": 14,
        "tool": "davison",
        "category": "西洋古典与现代占星",
        "name_zh": "戴维森时空关系盘 (Davison Relationship)",
        "input": {
            "date": "2026-07-07",
            "time": "18:00:00",
            "lat": 39.9042,
            "lon": 116.4074,
            "target_date": "1998-10-24"
},
        "description": "时空双中点实体排盘分析伴侣深层宿命",
        "validate": lambda d: (
            d.get("technique") == "davison" and 0.0 <= d.get("ascendant", -1) <= 360.0
        ),
        "expect_desc": "计算时空关系盘有效上升与天顶"
    },
    {
        "id": 15,
        "tool": "chart12",
        "category": "西洋古典与现代占星",
        "name_zh": "十二分盘 (Dwadasamsa) 微黄道展开",
        "input": {
            "date": "1990-12-25",
            "time": "06:00:00",
            "lat": 31.23,
            "lon": 121.47
        },
        "description": "黄道乘以12阶展开，深入微观潜意识与家族宿业",
        "validate": lambda d: (
            d.get("technique") == "chart12" and
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 10 and
            0.0 <= d.get("ascendant", -1) <= 360.0
        ),
        "expect_desc": "12倍阶展开行星黄道微度与微上升ASC"
    },
    {
        "id": 16,
        "tool": "chart13",
        "category": "西洋古典与现代占星",
        "name_zh": "十三分扩展盘与蛇夫座维度拓扑",
        "input": {
            "date": "2000-01-01",
            "time": "00:00:00",
            "lat": 0.0,
            "lon": 0.0
        },
        "description": "千禧年历元十三谐波展开与蛇夫座拓扑排布",
        "validate": lambda d: (
            d.get("technique") == "chart13" and
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 10
        ),
        "expect_desc": "13谐波行星度数与蛇夫座能量映射"
    },
    {
        "id": 17,
        "tool": "relocation",
        "category": "西洋古典与现代占星",
        "name_zh": "重置地理星盘 (Relocation Chart)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 39.90,       # 原生北京
            "lon": 116.40,
            "target_lat": 48.8566,  # 重置至巴黎
            "target_lon": 2.3522
        },
        "description": "出生于北京迁居至巴黎，重置经纬度计算当地新上升ASC与四轴",
        "validate": lambda d: (
            d.get("technique") == "relocation" and
            abs(d.get("target_lat", 0) - 48.8566) < 0.01 and
            0.0 <= d.get("ascendant", -1) <= 360.0
        ),
        "expect_desc": "成功更新目标地经纬度并计算出新地域四轴"
    },
    {
        "id": 18,
        "tool": "acg",
        "category": "西洋古典与现代占星",
        "name_zh": "占星制图天球投影 (AstroCartoGraphy)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00"
        },
        "description": "计算全球各主要行星上升ASC/天顶MC地心投影地理经线",
        "validate": lambda d: (
            d.get("technique") == "acg" and
            isinstance(d.get("lines"), list) and d.get("lines_count", 0) > 0 and
            any("mc_longitude" in x for x in d["lines"])
        ),
        "expect_desc": "输出全球行星四轴投射地理经线集合"
    },
    {
        "id": 19,
        "tool": "solarreturn",
        "category": "西洋古典与现代占星",
        "name_zh": "太阳返照推运盘 (Solar Return)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "target_year": 2026,
            "lat": 31.23,
            "lon": 121.47
        },
        "description": "推算2026年太阳重返出生精确黄经时刻，建立该年运盘",
        "validate": lambda d: (
            d.get("target_year") == 2026 and
            abs(d.get("sun_lon_diff", 999)) < 0.05 and
            0.0 <= d.get("return_sun_lon", -1) <= 360.0
        ),
        "expect_desc": "返照太阳黄经与本命太阳黄经残差<0.05度"
    },
    {
        "id": 20,
        "tool": "lunarreturn",
        "category": "西洋古典与现代占星",
        "name_zh": "月亮返照推运盘 (Lunar Return)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "target_year": 2026,
            "month": 11,
            "lat": 31.23,
            "lon": 121.47
        },
        "description": "推算2026年11月月亮精确重返本命黄经时刻构建月运盘",
        "validate": lambda d: (
            abs(d.get("moon_lon_diff", 999)) < 0.1 and
            0.0 <= d.get("return_moon_lon", -1) <= 360.0
        ),
        "expect_desc": "返照月亮黄经与本命月亮黄经残差<0.1度"
    },
    {
        "id": 21,
        "tool": "lunationphase",
        "category": "西洋古典与现代占星",
        "name_zh": "生辰月相八阶段与生命节律",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00"
        },
        "description": "日月黄经差推算生辰月相八阶段与光照百分比",
        "validate": lambda d: (
            isinstance(d.get("phase_name"), str) and
            0.0 <= d.get("illumination_pct", -1) <= 100.0 and
            0.0 <= d.get("phase_angle", -1) <= 360.0
        ),
        "expect_desc": "识别月相名称，月面光照度在0%~100%之间"
    },
    {
        "id": 22,
        "tool": "prenatalsyzygy",
        "category": "西洋古典与现代占星",
        "name_zh": "出生产前朔望合朔点 (Prenatal Syzygy)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00"
        },
        "description": "古典希腊占星计算出生产前最近一次新月/满月合朔点",
        "validate": lambda d: (
            isinstance(d.get("syzygy_type"), str) and
            0.0 <= d.get("longitude", -1) <= 360.0 and
            d.get("days_before_birth", -1) >= 0.0
        ),
        "expect_desc": "标定产前朔望类型，且合朔时刻位于生辰之前"
    },
    {
        "id": 23,
        "tool": "triplicityrulers",
        "category": "西洋古典与现代占星",
        "name_zh": "多罗修斯三分性主星 (Triplicity Rulers)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00"
        },
        "description": "多罗修斯古典占星依昼夜分得查四正三分性三大主星",
        "validate": lambda d: (
            isinstance(d.get("rulers"), dict) and
            "day_ruler" in d["rulers"] and "night_ruler" in d["rulers"]
        ),
        "expect_desc": "返回日主星、夜主星与参与星三分性结构"
    },
    {
        "id": 24,
        "tool": "keypoints",
        "category": "西洋古典与现代占星",
        "name_zh": "阿拉伯点与耶鲁恒星相交矩阵",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 31.23,
            "lon": 121.47,
            "orb": 1.5
        },
        "description": "容许度1.5度内检索全盘天体及阿拉伯虚点与全天亮星合相",
        "validate": lambda d: (
            isinstance(d.get("lot_of_fortune"), dict) and
            isinstance(d.get("lot_of_spirit"), dict) and
            d.get("fixed_star_connections_count", 0) > 0
        ),
        "expect_desc": "计算福点与精神点阿拉伯虚点坐标，并输出恒星合相"
    },
    {
        "id": 25,
        "tool": "horary",
        "category": "西洋古典与现代占星",
        "name_zh": "卜卦占星学 (Horary Astrology) 格局判定",
        "input": {
            "date": "2026-10-02",
            "time": "14:15:00",
            "lat": 31.23,
            "lon": 121.47,
            "question_type": "career"
        },
        "description": "问事业前程，立第一宫主星与第十宫官禄主星会合转化",
        "validate": lambda d: (
            isinstance(d.get("horary_judgment"), dict) and
            "quesited_house" in d["horary_judgment"] and
            "querent_ruler" in d["horary_judgment"]
        ),
        "expect_desc": "确立问卦人命主与所问官禄主星并做出断辞"
    },
    {
        "id": 26,
        "tool": "election",
        "category": "西洋古典与现代占星",
        "name_zh": "西洋择日占星学 (Electional Astrology) 评分",
        "input": {
            "date": "2026-10-08",
            "time": "09:18:00",
            "lat": 39.90,
            "lon": 116.40
        },
        "description": "择开业吉辰，评估月亮空亡(VOC)与四轴吉凶天体得分",
        "validate": lambda d: (
            isinstance(d.get("void_of_course"), bool) and
            isinstance(d.get("moon_sign"), str) and
            0.0 <= d.get("moon_degree", -1) <= 30.0
        ),
        "expect_desc": "判断月亮是否空亡，并计算月亮星座度数"
    },
    {
        "id": 27,
        "tool": "babylon",
        "category": "西洋古典与现代占星",
        "name_zh": "巴比伦原始恒星宿度 (Babylonian Zodiac)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "school": "swissA10"
},
        "description": "巴比伦原始恒星黄道折算行星位置与恒星宿度",
        "validate": lambda d: (
            isinstance(d.get("ayanamsa_name"), str) and
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 10
        ),
        "expect_desc": "应用巴比伦岁差模型输出原始恒星黄道坐标"
    },
    {
        "id": 28,
        "tool": "germany",
        "category": "西洋古典与现代占星",
        "name_zh": "汉堡学派乌拉尼亚行星中点与八虚星",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00"
        },
        "description": "维特中点占星术与超海王星八大虚星（丘比特、哈迪斯、宙斯等）",
        "validate": lambda d: (
            isinstance(d.get("tnps"), list) and len(d["tnps"]) == 8 and
            isinstance(d.get("midpoints"), list)
        ),
        "expect_desc": "成功定位汉堡学派八大虚星(TNP)并计算中点树"
    },

    # =========================================================================
    # 三、东方七政与吠陀印占流派 (6项)
    # =========================================================================
    {
        "id": 29,
        "tool": "guolao",
        "category": "七政四余与吠陀印占",
        "name_zh": "琴堂果老星宗七政四余排盘与神煞",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 31.23,
            "lon": 121.47
        },
        "description": "七政四余十一曜排盘入二十八宿度，命度主与张果星宗神煞",
        "validate": lambda d: (
            isinstance(d.get("seven_governors"), list) and len(d["seven_governors"]) >= 7 and
            isinstance(d.get("four_residuals"), list) and len(d["four_residuals"]) == 4 and
            isinstance(d.get("ming_mansion"), str) and len(d["ming_mansion"]) > 0 and
            0.0 <= d.get("ming_degree", -1) <= 360.0
        ),
        "expect_desc": "输出日月五星七政与罗计炁孛四余，计算命度主"
    },
    {
        "id": 30,
        "tool": "qizhengkin",
        "category": "七政四余与吠陀印占",
        "name_zh": "七政四余生克制化与洞微飞星",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 31.23,
            "lon": 121.47,
            "ascendant": 45.0
},
        "description": "洞微大限、政余相生相克与殿驾得位分析",
        "validate": lambda d: (
            isinstance(d.get("seven_governors"), list) and
            isinstance(d.get("four_residuals"), list)
        ),
        "expect_desc": "完成七政四余得地与克制生化综合断论"
    },
    {
        "id": 31,
        "tool": "vedic",
        "category": "七政四余与吠陀印占",
        "name_zh": "印度吠陀占星 (Vedic D1/D9 九分盘)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 28.6139,     # 新德里
            "lon": 77.2090
        },
        "description": "拉希里恒星黄道，D1 拉希盘与 D9 纳瓦姆沙分盘",
        "validate": lambda d: (
            isinstance(d.get("ascendant_sign"), str) and
            isinstance(d.get("vimshottari_dasha"), list) and
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 9
        ),
        "expect_desc": "输出吠陀上升星座与维姆绍塔里大限运程结构"
    },
    {
        "id": 32,
        "tool": "feigong",
        "category": "七政四余与吠陀印占",
        "name_zh": "吠陀印占飞宫星曜互溶格局",
        "input": {
            "year": 1998,
            "month": 10,
            "day": 24,
            "hour": 8,
            "qi_zhi": "辰"
        },
        "description": "九宫星曜互坐与飞宫飞星吉凶格局分析",
        "validate": lambda d: (
            isinstance(d.get("palace_stars"), list) and
            isinstance(d.get("gan_gong"), dict)
        ),
        "expect_desc": "输出飞宫分布与格局生克态势"
    },
    {
        "id": 33,
        "tool": "india_rectify",
        "category": "七政四余与吠陀印占",
        "name_zh": "印度占星生时微调校正法 (KP/Tattva)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 28.61,
            "lon": 77.21
        },
        "description": "五行微刻(Tattwa)与KP子领主反演生时真实刻度",
        "validate": lambda d: (
            isinstance(d.get("active_tattwa"), str) and
            isinstance(d.get("kp_rectification"), dict) and
            isinstance(d.get("moon_nakshatra"), str)
        ),
        "expect_desc": "确定活动五行微刻与月宿纳克夏特拉"
    },
    {
        "id": 34,
        "tool": "relative",
        "category": "七政四余与吠陀印占",
        "name_zh": "六亲衍生分盘推运 (Derived Houses)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 31.23,
            "lon": 121.47
        },
        "description": "以第四宫/第七宫为太极点重排十二宫推演六亲吉凶",
        "validate": lambda d: (
            d.get("technique") == "relative" and len(d.get("houses", [])) == 12
        ),
        "expect_desc": "生成以太极衍生宫位为基准的派生星盘"
    },

    # =========================================================================
    # 四、高阶星限与推运技术 (19项)
    # =========================================================================
    {
        "id": 35,
        "tool": "zr",
        "category": "高阶星限与推运",
        "name_zh": "希腊化黄道释放法 (Zodiacal Releasing)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "age": 28.0
        },
        "description": "从精神点起算黄道释放，定28岁时大运层级与峰值星座",
        "validate": lambda d: (
            d.get("target_age") == 28.0 and
            isinstance(d.get("current_period"), dict) and
            "sign" in d["current_period"]
        ),
        "expect_desc": "输出28岁当前所落释放星座与运势阶段"
    },
    {
        "id": 36,
        "tool": "balbillus",
        "category": "高阶星限与推运",
        "name_zh": "巴比卢斯太阳主限极距推运",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "age": 28.0
        },
        "description": "巴比卢斯太阳赤道半弧极距主限",
        "validate": lambda d: (
            isinstance(d.get("main_periods"), list) and
            d.get("periods_count", 0) > 0 and
            d.get("total_cycle_years", 0) > 0
        ),
        "expect_desc": "计算主运弧各阶段主星与周天年数"
    },
    {
        "id": 37,
        "tool": "decennials",
        "category": "高阶星限与推运",
        "name_zh": "古典十年限法推运 (Decennials)",
        "input": {
            "age": 28.0
        },
        "description": "十年大限法推算28岁大运主星与流年辅星",
        "validate": lambda d: (
            d.get("age") == 28.0 and
            isinstance(d.get("current_period"), dict) and
            "general_ruler" in d["current_period"]
        ),
        "expect_desc": "确定当前总运主宰星(General Ruler)"
    },
    {
        "id": 38,
        "tool": "firdaria",
        "category": "高阶星限与推运",
        "name_zh": "中世纪波斯法达星限法 (Firdaria)",
        "input": {
            "age": 28.0,
            "gender": 1
        },
        "description": "波斯75年昼生法达大运循环，查28岁主限星与副限星",
        "validate": lambda d: (
            isinstance(d.get("active_lord"), str) and
            isinstance(d.get("cycle_periods"), list) and
            d.get("current_age") == 28.0
        ),
        "expect_desc": "输出28岁当前执掌主星与各周期分段"
    },
    {
        "id": 39,
        "tool": "distributions",
        "category": "高阶星限与推运",
        "name_zh": "埃及界分布主限推运 (Distributions through Bounds)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 31.23,
            "lon": 121.47,
            "age": 28.0
},
        "description": "上升点步进埃及界与流年分段主星推衍",
        "validate": lambda d: (
            isinstance(d.get("distributions"), list) and
            d.get("terms_count", 0) > 0
        ),
        "expect_desc": "生成埃及界流年步进分布序列"
    },
    {
        "id": 40,
        "tool": "persiandirected",
        "category": "高阶星限与推运",
        "name_zh": "波斯流年限星主导法 (Persian Directed System)",
        "input": {
            "age": 28.0
        },
        "description": "波斯九曜步进年限度数主宰星",
        "validate": lambda d: (
            d.get("age") == 28.0 and isinstance(d.get("directed_ruler"), str)
        ),
        "expect_desc": "输出波斯步进主导星曜"
    },
    {
        "id": 41,
        "tool": "planetaryages",
        "category": "高阶星限与推运",
        "name_zh": "行星生命周期论 (Planetary Ages of Man)",
        "input": {
            "age": 28.0
        },
        "description": "人生七阶段行星执掌：28岁处于太阳/火星主导青年阶段",
        "validate": lambda d: (
            d.get("age") == 28.0 and
            isinstance(d.get("active_planet"), str) and
            isinstance(d.get("age_band"), str)
        ),
        "expect_desc": "划分人生年龄段并匹配执政行星"
    },
    {
        "id": 42,
        "tool": "yearsystem129",
        "category": "高阶星限与推运",
        "name_zh": "瓦伦斯 129 年周天限法",
        "input": {
            "age": 28.0
        },
        "description": "古典占星瓦伦斯129年全息周天限数理分段",
        "validate": lambda d: (
            d.get("age") == 28.0 and
            isinstance(d.get("main_ruler"), str) and
            isinstance(d.get("current_sub_period"), dict)
        ),
        "expect_desc": "输出当前所处瓦伦斯大主星与小周期"
    },
    {
        "id": 43,
        "tool": "profection",
        "category": "高阶星限与推运",
        "name_zh": "古典希腊小限法 (Annual Profections)",
        "input": {
            "age": 28.0
        },
        "description": "每年移一宫，28岁移入第五宫(28 mod 12 = 4 + 1)",
        "validate": lambda d: (
            d.get("age") == 28.0 and
            d.get("profection_house") == 5 and
            isinstance(d.get("ruler_planet"), str)
        ),
        "expect_desc": "精准命中28岁对应第五宫年限主星"
    },
    {
        "id": 44,
        "tool": "draconic",
        "category": "高阶星限与推运",
        "name_zh": "龙首交点盘 (Draconic Chart / 灵魂盘)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 31.23,
            "lon": 121.47
        },
        "description": "北交点归零重构黄道，透视灵魂本质",
        "validate": lambda d: (
            d.get("technique") == "draconic" and
            0.0 <= d.get("node_lon", -1) <= 360.0 and
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 10
        ),
        "expect_desc": "以交点为基准重排各大行星黄道度数"
    },
    {
        "id": 45,
        "tool": "solararc",
        "category": "高阶星限与推运",
        "name_zh": "太阳弧推运法 (Solar Arc Directions)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "target_date": "2026-10-24"
        },
        "description": "28年太阳弧推进约28度，步进全盘天体",
        "validate": lambda d: (
            d.get("technique") == "solararc" and
            26.0 <= d.get("arc_degree", 0) <= 30.0 and
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 10
        ),
        "expect_desc": "太阳弧累积弧度在26°~30°之间，天体步进正确"
    },
    {
        "id": 46,
        "tool": "harmonic",
        "category": "高阶星限与推运",
        "name_zh": "高阶谐波占星分析 (Harmonic Chart)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "harmonic": 7.0
        },
        "description": "第七谐波盘展开，透视灵感创造力与灵性共振",
        "validate": lambda d: (
            d.get("harmonic") == 7.0 and
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 10
        ),
        "expect_desc": "7倍谐波黄道重算各大行星分布"
    },
    {
        "id": 47,
        "tool": "pd",
        "category": "高阶星限与推运",
        "name_zh": "主限推运半弧赤道直升法 (Primary Directions)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 31.23,
            "lon": 121.47,
            "age": 28.0
        },
        "description": "普拉西德赤道斜升与子午半弧高精主限相位触发检测",
        "validate": lambda d: (
            d.get("technique") == "pd" and
            isinstance(d.get("hits"), list) and
            d.get("hits_count", 0) >= 0
        ),
        "expect_desc": "输出主限推运相交事件列表"
    },
    {
        "id": 48,
        "tool": "mundane",
        "category": "高阶星限与推运",
        "name_zh": "世俗占星白羊春分入宫盘 (Aries Ingress)",
        "input": {
            "year": 2026,
            "lon": 116.40,
            "lat": 39.90
        },
        "description": "2026年太阳进入白羊座0度北京地区世俗国运春分盘",
        "validate": lambda d: (
            d.get("year") == 2026 and
            isinstance(d.get("primary_ingress"), dict) and
            "ingress_jde" in d["primary_ingress"] and
            isinstance(d.get("seasonal_ingresses"), list) and len(d["seasonal_ingresses"]) == 4
        ),
        "expect_desc": "成功锁定春分JDE入宫时刻，生成四季入宫盘"
    },
    {
        "id": 49,
        "tool": "ephemeris",
        "category": "高阶星限与推运",
        "name_zh": "现代高精瑞士物理星历切片自检 (SEPK)",
        "input": {
            "date": "2026-10-02",
            "time": "12:00:00"
        },
        "description": "自检核心600年与全史6000年物理星历切比雪夫切片",
        "validate": lambda d: (
            d.get("planets_count", 0) >= 10 and
            d.get("sepk_baked_slices_count", 0) >= 1
        ),
        "expect_desc": "输出星历天体数量及内置切片卷册"
    },
    {
        "id": 50,
        "tool": "planet_cycles",
        "category": "高阶星限与推运",
        "name_zh": "外行星周期与世界历史周期律",
        "input": {
            "date": "2026-10-02",
            "time": "12:00:00"
        },
        "description": "木土会合、土天合相与宏观历史长周期测算",
        "validate": lambda d: (
            isinstance(d.get("planets"), list) and len(d["planets"]) >= 10
        ),
        "expect_desc": "完成外行星黄道动力学位置测定"
    },
    {
        "id": 51,
        "tool": "returntimeline",
        "category": "高阶星限与推运",
        "name_zh": "返照周期连续时间线 (Return Timeline)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "start_year": 2024,
            "end_year": 2028
        },
        "description": "2024-2028五年连续太阳返照时刻轨迹",
        "validate": lambda d: (
            isinstance(d.get("timeline"), list) and len(d["timeline"]) == 5
        ),
        "expect_desc": "返回连续5年精确太阳返照时刻序列"
    },
    {
        "id": 52,
        "tool": "extrareturns",
        "category": "高阶星限与推运",
        "name_zh": "各大行星扩展返照 (火星/金星/水星)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00"
        },
        "description": "金星、火星重返本命度数个人成长返照盘",
        "validate": lambda d: (
            isinstance(d.get("returns"), list) and len(d["returns"]) >= 1
        ),
        "expect_desc": "生成火星等次要天体返照时刻周期"
    },
    {
        "id": 53,
        "tool": "agepoint",
        "category": "高阶星限与推运",
        "name_zh": "胡伯学派年龄点推运 (Huber Life Clock)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 31.23,
            "lon": 121.47,
            "age": 28.0
},
        "description": "胡伯生命时钟：六岁一宫位，28岁落入第五宫深层心理转化",
        "validate": lambda d: (
            d.get("age") == 28.0 and
            d.get("cycle") == 1 and
            d.get("house_number") == 5 and
            0.0 <= d.get("age_point_degree", -1) <= 360.0
        ),
        "expect_desc": "精准定位胡伯年龄点于第五宫与对应度数"
    },
    {
        "id": 54,
        "tool": "planetaryarc",
        "category": "高阶星限与推运",
        "name_zh": "各大行星物理弧长推运 (Planetary Arc)",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "age": 28.0
        },
        "description": "火星弧与木星弧等不同行星真实运行弧长推进",
        "validate": lambda d: (
            isinstance(d.get("arc_degrees"), float) and
            isinstance(d.get("directed_planets"), list) and len(d["directed_planets"]) >= 10
        ),
        "expect_desc": "输出各大天体专有物理弧长与步进坐标"
    },
    {
        "id": 55,
        "tool": "jaynesprog",
        "category": "高阶星限与推运",
        "name_zh": "查尔斯·简恩综合次限推进",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "age": 28.0
        },
        "description": "一日一年次限结合赤纬推进弧",
        "validate": lambda d: (
            d.get("age") == 28.0 and
            isinstance(d.get("progressed_planets"), list) and
            len(d["progressed_planets"]) >= 10
        ),
        "expect_desc": "输出28日折算年后次限天体分布"
    },
    {
        "id": 56,
        "tool": "vedicprog",
        "category": "高阶星限与推运",
        "name_zh": "吠陀 Vimshottari 维姆绍塔里大限双向推进",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "age": 28.0
        },
        "description": "按本命月亮宿度起大运，计算28岁当前所落星宿主运与副限",
        "validate": lambda d: (
            isinstance(d.get("current_mahadasha"), str) and
            isinstance(d.get("dasha_periods"), list)
        ),
        "expect_desc": "标定当前大运领主(Maha Dasha)与大限周期"
    },
    {
        "id": 57,
        "tool": "givenyear",
        "category": "高阶星限与推运",
        "name_zh": "指定目标流年综合占星快照",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "target_year": 2026
        },
        "description": "2026年流年整体星盘推进快照",
        "validate": lambda d: (
            d.get("age") == 28.0 and
            isinstance(d.get("planets"), list) and
            d.get("progressed_jde", 0) > 0
        ),
        "expect_desc": "生成28岁目标流年星盘并完成JDE步进"
    },
    {
        "id": 58,
        "tool": "hellen_chart",
        "category": "高阶星限与推运",
        "name_zh": "希腊化整宫制本命盘与多罗修斯界律",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "lat": 31.23,
            "lon": 121.47
        },
        "description": "希腊古典整宫制(Whole Sign)与埃及界(Egyptian Bounds)",
        "validate": lambda d: (
            "Whole-Sign" in d.get("system", "") and
            isinstance(d.get("houses"), list) and len(d["houses"]) == 12 and
            isinstance(d.get("egyptian_bounds"), list)
        ),
        "expect_desc": "严格按整宫制划定十二宫头与埃及界分布"
    },

    # =========================================================================
    # 五、易经卦象与数术秘典 (7项)
    # =========================================================================
    {
        "id": 59,
        "tool": "liuyao",
        "category": "易经卦象与数术",
        "name_zh": "周易纳甲六爻梅花起卦",
        "input": {
            "numbers": [1, 5, 3]  # 上乾1 下巽5 动爻3
        },
        "description": "数1、5、3起卦：上天风姤，三爻动化天水讼",
        "validate": lambda d: (
            d.get("shang_gua") == "乾" and
            d.get("xia_gua") == "巽" and
            d.get("original_gua") == "天风姤" and
            d.get("moving_yao") == 3 and
            isinstance(d.get("yaos"), list) and len(d["yaos"]) == 6
        ),
        "expect_desc": "本卦为天风姤，三爻发动，装配六爻干支六亲"
    },
    {
        "id": 60,
        "tool": "gua_desc",
        "category": "易经卦象与数术",
        "name_zh": "六十四卦卦名彖传深度详解",
        "input": {
            "numbers": [7, 8, 2]  # 上艮7 下坤8 -> 山地剥
        },
        "description": "起山地剥卦，二爻动，解析卦辞彖辞大象",
        "validate": lambda d: (
            d.get("shang_gua") == "艮" and
            d.get("xia_gua") == "坤" and
            d.get("original_gua") == "山地剥"
        ),
        "expect_desc": "本卦为山地剥，并附带宫位与五行属性"
    },
    {
        "id": 61,
        "tool": "gua_meiyi",
        "category": "易经卦象与数术",
        "name_zh": "梅花易数体用生克决断",
        "input": {
            "numbers": [3, 2, 4]  # 上离3 下兑2 -> 泽火革变火天大有
        },
        "description": "火泽睽/泽火革卦体用生克决断",
        "validate": lambda d: (
            isinstance(d.get("original_gua"), str) and
            isinstance(d.get("bian_gua"), str) and
            isinstance(d.get("gong_wuxing"), str)
        ),
        "expect_desc": "输出本卦与变卦，定五行与体用生克"
    },
    {
        "id": 62,
        "tool": "heluo",
        "category": "易经卦象与数术",
        "name_zh": "河洛理数天地数与先后天卦典籍诗赋",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "gender": 1
        },
        "description": "天数二十五、地数三十生成先后天卦，配元堂爻与断辞",
        "validate": lambda d: (
            isinstance(d.get("tian_num"), int) and
            isinstance(d.get("di_num"), int) and
            isinstance(d.get("xian_tian_gua"), str) and
            isinstance(d.get("hou_tian_gua"), str) and
            isinstance(d.get("yuan_tang_yao"), int)
        ),
        "expect_desc": "计算天地总数，定先天卦、后天卦与元堂发爻"
    },
    {
        "id": 63,
        "tool": "canping",
        "category": "易经卦象与数术",
        "name_zh": "邵子参评数五行日宫命宫大运",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00"
        },
        "description": "五行日宫与命宫干支推导参评秘数与大运数",
        "validate": lambda d: (
            isinstance(d.get("day_palace"), str) and
            isinstance(d.get("birth_numbers"), dict) and
            isinstance(d.get("dayun_numbers"), list)
        ),
        "expect_desc": "立日宫命宫，输出生辰起数与大运数理轨迹"
    },
    {
        "id": 64,
        "tool": "shaozi",
        "category": "易经卦象与数术",
        "name_zh": "邵子神数六千一百四十四数原典条文",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "四柱天干地支数理推演，命中六千一百四十四数",
        "validate": lambda d: (
            isinstance(d.get("tian_shu"), int) and
            isinstance(d.get("di_shu"), int) and
            isinstance(d.get("base_verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "演化天地数，定位基本条文ID并输出原典诗句"
    },
    {
        "id": 65,
        "tool": "tieban",
        "category": "易经卦象与数术",
        "name_zh": "铁板神数一万二千条文秘数推导",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "gender": 1
},
        "description": "先天数、后天数、日命数推演，命中一万二千条文",
        "validate": lambda d: (
            isinstance(d.get("xian_tian_num"), int) and
            isinstance(d.get("ben_ming_num"), int) and
            isinstance(d.get("verse_ids"), list) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "生成命宫数序列并输出铁板断命条文"
    },

    # =========================================================================
    # 六、十大神数典籍家族 (11项)
    # =========================================================================
    {
        "id": 66,
        "tool": "shenshu",
        "category": "十大神数典籍",
        "name_zh": "皇极观物演义神数",
        "input": {
            "family_key": "nanji",
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "通用神数家族矩阵分发调度",
        "validate": lambda d: (
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "神数矩阵成功定位条文编号并输出对应批语"
    },
    {
        "id": 67,
        "tool": "nanji",
        "category": "十大神数典籍",
        "name_zh": "南极神数秘典条文",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "南极延寿星君二百四十六数寿夭康宁条文",
        "validate": lambda d: (
            d.get("key") == "nanji" and
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "输出南极神数专有条文"
    },
    {
        "id": 68,
        "tool": "beiji",
        "category": "十大神数典籍",
        "name_zh": "北极神数正统卦数条文",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "紫微北极星垣二千三百四十一数官禄荣显条文",
        "validate": lambda d: (
            d.get("key") == "beiji" and
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "输出北极神数专有条文"
    },
    {
        "id": 69,
        "tool": "taixuan",
        "category": "十大神数典籍",
        "name_zh": "太玄经八十一首太玄诗法",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "扬雄太玄八十一首原典诗赋",
        "validate": lambda d: (
            d.get("key") == "taixuan" and
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "命中太玄八十一首卦象诗文"
    },
    {
        "id": 70,
        "tool": "wangji",
        "category": "十大神数典籍",
        "name_zh": "心易发微皇极四象望蓟数",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "心易发微秘传四象望蓟数理",
        "validate": lambda d: (
            d.get("key") == "wangji" and
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "输出皇极观物演义四象哲理批注"
    },
    {
        "id": 71,
        "tool": "cetian",
        "category": "十大神数典籍",
        "name_zh": "策天神数大衍策数",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "大衍策数推命直断四时休咎",
        "validate": lambda d: (
            d.get("key") == "cetian" and
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "输出策天神数吉凶条文"
    },
    {
        "id": 72,
        "tool": "chunzi",
        "category": "十大神数典籍",
        "name_zh": "蠢子神数四千五百七十五数",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "邵雍蠢子神数秘传生老病死与流年关煞",
        "validate": lambda d: (
            d.get("key") == "chunzi" and
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "输出蠢子神数流年批注"
    },
    {
        "id": 73,
        "tool": "fendjing",
        "category": "十大神数典籍",
        "name_zh": "分经神数易理分经数",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "六十四卦分经分度神数，推骨肉亲疏与财运",
        "validate": lambda d: (
            d.get("key") == "fendjing" and
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "输出分经神数格局诗"
    },
    {
        "id": 74,
        "tool": "jingjue",
        "category": "十大神数典籍",
        "name_zh": "警觉神数醒世吉凶秘数",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "先天精决铁券醒世秘数",
        "validate": lambda d: (
            d.get("key") == "jingjue" and
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "输出警觉神数醒世批断"
    },
    {
        "id": 75,
        "tool": "shenyishu",
        "category": "十大神数典籍",
        "name_zh": "神易数易数心法",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "先天神易玄机算通达事理微奥",
        "validate": lambda d: (
            d.get("key") == "shenyishu" and
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "输出神易数运势论断"
    },
    {
        "id": 76,
        "tool": "wuzhao",
        "category": "十大神数典籍",
        "name_zh": "五兆神数先知吉凶秘数",
        "input": {
            "year_gz": "戊寅",
            "month_gz": "壬戌",
            "day_gz": "甲辰",
            "hour_gz": "戊辰"
        },
        "description": "五兆幽微神算测机变隐匿前兆",
        "validate": lambda d: (
            d.get("key") == "wuzhao" and
            isinstance(d.get("verse_id"), int) and
            isinstance(d.get("verse_text"), str) and len(d["verse_text"]) > 0
        ),
        "expect_desc": "输出五兆神数幽微断语"
    },

    # =========================================================================
    # 七、通胜老黄历与全域择吉体系 (12项)
    # =========================================================================
    {
        "id": 77,
        "tool": "tongshu",
        "category": "通胜择吉与历法",
        "name_zh": "通胜老黄历建除十二神廿八宿",
        "input": {
            "date": "2026-10-02"
        },
        "description": "公历2026年10月2日通胜：建除神、值日二十八宿与黄黑道神煞",
        "validate": lambda d: (
            isinstance(d.get("ganzhi_date"), str) and
            isinstance(d.get("xiu28_star"), str) and
            isinstance(d.get("donggong_rating"), str)
        ),
        "expect_desc": "输出干支日期、廿八星宿值日及董公择日评级"
    },
    {
        "id": 78,
        "tool": "bazizeri",
        "category": "通胜择吉与历法",
        "name_zh": "八字五行生克吉时扫描",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "遍历区间筛选八字天乙贵人吉时",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("hits"), list)
        ),
        "expect_desc": "成功命中并筛选出八字大吉时辰"
    },
    {
        "id": 79,
        "tool": "qimenzeri",
        "category": "通胜择吉与历法",
        "name_zh": "奇门遁甲三吉门得奇择时扫描",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "筛选开休生三吉门会合乙丙丁三奇之上上吉时",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("hits"), list)
        ),
        "expect_desc": "成功命中奇门开休生三吉门良辰"
    },
    {
        "id": 80,
        "tool": "taiyizeri",
        "category": "通胜择吉与历法",
        "name_zh": "太乙神数天目客大将得位择日",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "太乙天目与客大将、主大将生旺不犯囚迫之吉日",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("hits"), list)
        ),
        "expect_desc": "成功命中太乙得算得位吉时"
    },
    {
        "id": 81,
        "tool": "ziweizeri",
        "category": "通胜择吉与历法",
        "name_zh": "紫微斗数三方四正吉曜加临择日",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "紫微紫府朝垣吉曜临宫择吉",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("hits"), list)
        ),
        "expect_desc": "成功命中紫微吉星拱照时辰"
    },
    {
        "id": 82,
        "tool": "liurengzeri",
        "category": "通胜择吉与历法",
        "name_zh": "大六壬三传吉将吉神汇聚择吉",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "大六壬青龙太常贵人发用择吉",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("hits"), list)
        ),
        "expect_desc": "成功命中六壬元首发端之良辰"
    },
    {
        "id": 83,
        "tool": "sanshizeri",
        "category": "通胜择吉与历法",
        "name_zh": "三式合一同契天地大成择日",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "太乙奇门六壬三式共振上吉良辰扫描",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("hits"), list)
        ),
        "expect_desc": "成功命中三式同契高标准复合吉时"
    },
    {
        "id": 84,
        "tool": "qizhengzeri",
        "category": "通胜择吉与历法",
        "name_zh": "七政四余日月拱照天星择日",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "天星七政日月夹拱四正吉时",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("hits"), list)
        ),
        "expect_desc": "成功输出七政天星择日时辰"
    },
    {
        "id": 85,
        "tool": "indiazeri",
        "category": "通胜择吉与历法",
        "name_zh": "吠陀印度占星星宿择日 (Muhurta)",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "月宿吉祥 Nakshatra 与 Tithi 时位择吉",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("hits"), list)
        ),
        "expect_desc": "成功输出吠陀时位穆胡尔塔吉时"
    },
    {
        "id": 86,
        "tool": "huanglizeri",
        "category": "通胜择吉与历法",
        "name_zh": "通书黄历黄道吉日神煞避凶",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "黄道青龙明堂金匮吉神值日筛选",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("jian_chu_god"), str)
        ),
        "expect_desc": "匹配建除客神与黄道吉日"
    },
    {
        "id": 87,
        "tool": "zeri",
        "category": "通胜择吉与历法",
        "name_zh": "通用多维条件良辰扫描器",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "通用条件引擎扫描良辰吉时",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("hits"), list)
        ),
        "expect_desc": "完成全周期通用条件遍历"
    },
    {
        "id": 88,
        "tool": "qizhengelection",
        "category": "通胜择吉与历法",
        "name_zh": "琴堂七政星曜入度天机择吉",
        "input": {
            "start_date": "2026-10-01",
            "end_date": "2026-10-03"
        },
        "description": "琴堂科名科甲恩星临四轴择吉",
        "validate": lambda d: (
            d.get("hits_count", 0) > 0 and isinstance(d.get("hits"), list)
        ),
        "expect_desc": "扫描七政天星恩曜临垣时刻"
    },

    # =========================================================================
    # 八、民间杂占与西方神秘学 (11项)
    # =========================================================================
    {
        "id": 89,
        "tool": "xiaoliuren",
        "category": "民间杂占与西方神秘学",
        "name_zh": "小六壬六神流转断事",
        "input": {
            "month": 8,
            "day": 15,
            "hour": 6
        },
        "description": "八月十五卯时掐算：大安、留连、速喜、赤口、小吉、空亡",
        "validate": lambda d: (
            isinstance(d.get("chu_chuan"), dict) and
            isinstance(d.get("zhong_chuan"), dict) and
            isinstance(d.get("mo_chuan"), dict) and
            "name" in d["mo_chuan"]
        ),
        "expect_desc": "输出初中末三传六神实体断语"
    },
    {
        "id": 90,
        "tool": "yizhangjing",
        "category": "民间杂占与西方神秘学",
        "name_zh": "达摩一掌经佛道儒鬼四柱轮转",
        "input": {
            "month": 5,
            "day": 10,
            "hour": 8,
            "gender": 1,
            "after23_new_day": True
},
        "description": "五月初十辰时达摩一掌经轮转十二星盘",
        "validate": lambda d: (
            isinstance(d.get("year_pillar"), dict) and
            isinstance(d.get("month_pillar"), dict) and
            isinstance(d.get("day_pillar"), dict) and
            isinstance(d.get("hour_pillar"), dict) and
            "dao" in d["day_pillar"]
        ),
        "expect_desc": "排定年、月、日、时四道轮转星宿及命宫"
    },
    {
        "id": 91,
        "tool": "lingqi",
        "category": "民间杂占与西方神秘学",
        "name_zh": "汉代灵棋经上中下子成卦演辞",
        "input": {
            "numbers": [2, 1, 1]  # 上2 中1 下1
        },
        "description": "灵棋十二子投掷成卦，演成神卦卦辞诗句",
        "validate": lambda d: (
            isinstance(d.get("name"), str) and
            isinstance(d.get("duan_ci"), str) and
            isinstance(d.get("poem"), str) and len(d["poem"]) > 0
        ),
        "expect_desc": "得出灵棋卦名、断词与诗辞"
    },
    {
        "id": 92,
        "tool": "tarot",
        "category": "民间杂占与西方神秘学",
        "name_zh": "西方塔罗牌78张洗牌与三牌阵牌义",
        "input": {
            "spread": "three",
            "seed": 20261002
},
        "description": "经典三牌阵(过去/现在/未来)洗牌、抽牌与正逆位裁决",
        "validate": lambda d: (
            isinstance(d.get("spread_name"), str) and "三" in d["spread_name"] and
            isinstance(d.get("drawn_cards"), list) and len(d["drawn_cards"]) == 3 and
            isinstance(d.get("overall_advice"), str)
        ),
        "expect_desc": "抽三张牌，给出牌名、正逆位与综合建议"
    },
    {
        "id": 93,
        "tool": "geomancy",
        "category": "民间杂占与西方神秘学",
        "name_zh": "大地占卜泥土点十六卦天文地占",
        "input": {
            "numbers": [1, 2, 1, 2]  # 母亲卦发生种子
        },
        "description": "天文地占十六形：母亲卦、女儿卦、侄女卦、左右证人与最终法官",
        "validate": lambda d: (
            isinstance(d.get("mothers"), list) and len(d["mothers"]) == 4 and
            isinstance(d.get("daughters"), list) and len(d["daughters"]) == 4 and
            isinstance(d.get("judge"), dict) and "name_cn" in d["judge"]
        ),
        "expect_desc": "生成四母亲、四女儿、左右证人与终审法官卦"
    },
    {
        "id": 94,
        "tool": "guice",
        "category": "民间杂占与西方神秘学",
        "name_zh": "鬼谷子分定经两头钳命局诗断",
        "input": {
            "year_gan": "戊",
            "hour_gan": "癸"
        },
        "description": "年干戊配时干癸两头钳，定鬼谷子基业断语诗词",
        "validate": lambda d: (
            isinstance(d.get("ge_ming"), str) and
            isinstance(d.get("base_poem"), str) and len(d["base_poem"]) > 0
        ),
        "expect_desc": "确立两头钳格局名称，输出命局诗词"
    },
    {
        "id": 95,
        "tool": "tongshefa",
        "category": "民间杂占与西方神秘学",
        "name_zh": "古通社法四象推演断法",
        "input": {
            "numbers": [2, 1, 2, 1]
        },
        "description": "通社法左卦右卦互卦交互推衍吉凶生克",
        "validate": lambda d: (
            isinstance(d.get("left_hex_name"), str) and
            isinstance(d.get("right_hex_name"), str) and
            isinstance(d.get("main_relation"), str)
        ),
        "expect_desc": "确立左右两卦并判定主生克关系"
    },
    {
        "id": 96,
        "tool": "yanqin",
        "category": "民间杂占与西方神秘学",
        "name_zh": "演禽神数二十八宿翻禽倒将",
        "input": {
            "year_gz": "戊寅",
            "hour_gz": "戊辰"
        },
        "description": "七元甲子禽星演化：主将、次将、禽星翻禽倒将生克",
        "validate": lambda d: (
            isinstance(d.get("day_mansion"), dict) and
            isinstance(d.get("hour_mansion"), dict) and
            isinstance(d.get("zhu_jiang"), dict) and
            isinstance(d.get("fan_qin"), dict)
        ),
        "expect_desc": "确定日宿时宿，定主将次将并判定翻禽倒将"
    },
    {
        "id": 97,
        "tool": "xianqin",
        "category": "民间杂占与西方神秘学",
        "name_zh": "禽星六十花甲值宿命理",
        "input": {
            "year_gz": "戊寅",
            "hour_gz": "戊辰"
        },
        "description": "花甲禽星配属与投胎动物象征",
        "validate": lambda d: (
            isinstance(d.get("toutai_animal"), str) and
            isinstance(d.get("zhu_jiang"), dict) and
            isinstance(d.get("day_mansion"), dict)
        ),
        "expect_desc": "输出值宿主将与投胎化身动物"
    },
    {
        "id": 98,
        "tool": "suzhan",
        "category": "民间杂占与西方神秘学",
        "name_zh": "宿占正传二十八宿值日断吉凶",
        "input": {
            "month": 8,
            "day": 15
        },
        "description": "八月十五仲秋值日宿占吉凶",
        "validate": lambda d: (
            isinstance(d.get("day_mansion"), str) and
            isinstance(d.get("animal"), str) and
            isinstance(d.get("element"), str) and
            isinstance(d.get("personality_reading"), str)
        ),
        "expect_desc": "确定日宿、五行禽兽与性情论断"
    },
    {
        "id": 99,
        "tool": "otherbu",
        "category": "民间杂占与西方神秘学",
        "name_zh": "太公太玄金钱课杂占诸法",
        "input": {
            "spread": "coin",
            "seed": 8888,
            "school": "文王课"
},
        "description": "周文王金钱课摇六爻掷钱断卦",
        "validate": lambda d: (
            isinstance(d.get("hexagram_name"), str) and
            isinstance(d.get("upper_trigram"), str) and
            isinstance(d.get("lower_trigram"), str) and
            isinstance(d.get("poetic_sign"), str)
        ),
        "expect_desc": "起成金钱卦并附带诗签断词"
    },

    # =========================================================================
    # 九、天星外步、小成图与宏观知识库 (11项)
    # =========================================================================
    {
        "id": 100,
        "tool": "tianxing",
        "category": "天星小成图与知识库",
        "name_zh": "天星五星聚舍交角吉凶断法",
        "input": {
            "date": "2026-10-02",
            "time": "12:00:00",
            "city": "上海",
            "lat": 31.23,
            "lon": 121.47
},
        "description": "五星聚会与恒星相交吉曜落度",
        "validate": lambda d: (
            d.get("technique") == "tianxing" and
            isinstance(d.get("planets"), list) and
            len(d.get("planets", [])) >= 7 and
            isinstance(d.get("houses"), list)
        ),
        "expect_desc": "输出天星交会吉曜扫描结果"
    },
    {
        "id": 101,
        "tool": "tianxingzeri",
        "category": "天星小成图与知识库",
        "name_zh": "天星外步选择吉度吉时",
        "input": {
            "date": "2026-10-02",
            "time": "12:00:00"
        },
        "description": "天星外步度数择日择时推算",
        "validate": lambda d: (
            isinstance(d.get("hits"), list)
        ),
        "expect_desc": "完成天星外步吉时推衍"
    },
    {
        "id": 102,
        "tool": "xiaochengtu",
        "category": "天星小成图与知识库",
        "name_zh": "霍斐然小成图九宫归藏天地排盘",
        "input": {
            "up": "乾",
            "lo": "坤"
        },
        "description": "天地地天泰否，小成图九宫归藏天地排盘",
        "validate": lambda d: (
            d.get("up_gua") == "乾" and
            d.get("lo_gua") == "坤" and
            isinstance(d.get("gong_palaces"), list) and len(d["gong_palaces"]) == 9
        ),
        "expect_desc": "按九宫生成小成图九宫归藏八卦落宫"
    },
    {
        "id": 103,
        "tool": "xuanshi",
        "category": "天星小成图与知识库",
        "name_zh": "二十四史历代天象志历史事件检索",
        "input": {
            "name": "客星"
        },
        "description": "检索正史天象志中超新星、客星与彗星实测历史档案",
        "validate": lambda d: (
            isinstance(d.get("hits"), list) and d.get("hits_count", 0) > 0
        ),
        "expect_desc": "成功检索到历史客星记录"
    },
    {
        "id": 104,
        "tool": "astrodata",
        "category": "天星小成图与知识库",
        "name_zh": "全球近6万名流占星案例库检索",
        "input": {
            "name": "Newton"
        },
        "description": "检索全球知名历史人物（艾萨克·牛顿）星盘档案",
        "validate": lambda d: (
            d.get("db_present") is True and
            isinstance(d.get("hits"), list) and d.get("hits_count", 0) > 0 and
            d.get("total_records", 0) > 50000
        ),
        "expect_desc": "从近6万案例库中检索命中牛顿权威星盘数据"
    },
    {
        "id": 105,
        "tool": "export_registry",
        "category": "天星小成图与知识库",
        "name_zh": "全量110技法输出协议与格式快照注册表",
        "input": {
            "format": "json",
            "text": "all"
},
        "description": "系统全量技法输出协议注册表导出",
        "validate": lambda d: (
            d.get("technique") == "export_registry" and
            d.get("total_techniques", 0) >= 110
        ),
        "expect_desc": "确认注册表中收录技法总数>=110"
    },
    {
        "id": 106,
        "tool": "knowledge_registry",
        "category": "天星小成图与知识库",
        "name_zh": "玄学全谱系典籍元数据总索引",
        "input": {
            "domain": "all",
            "text": "astro"
},
        "description": "查询占星学体系典籍知识索引与文献出处",
        "validate": lambda d: (
            d.get("technique") == "knowledge_registry" and
            d.get("citations", 0) > 0
        ),
        "expect_desc": "输出权威学术文献引用条目数"
    },
    {
        "id": 107,
        "tool": "nongli_time",
        "category": "天星小成图与知识库",
        "name_zh": "真太阳时经纬度自动折算",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00",
            "city": "成都"
        },
        "description": "成都市（东经104.07度）相较北京时间（东经120度）真太阳时时差修正",
        "validate": lambda d: (
            "成都市" in d.get("location", "") and
            d.get("longitude", 0) < 105.0 and
            d.get("total_true_solar_offset_seconds", 0) < -2000  # 时差-40分钟以上
        ),
        "expect_desc": "匹配成都市经度并完成负向真太阳时修正"
    },
    {
        "id": 108,
        "tool": "jieqi_birth",
        "category": "天星小成图与知识库",
        "name_zh": "秒级平黄经交节节气分界算法",
        "input": {
            "date": "1998-10-24",
            "time": "08:30:00"
        },
        "description": "计算1998年10月24日出生所处节气（霜降）及节气推进度数",
        "validate": lambda d: (
            d.get("current_jieqi") == "霜降" and
            0.0 <= d.get("sun_ecliptic_longitude", -1) <= 360.0 and
            0.0 <= d.get("jieqi_progress_degree", -1) <= 15.0
        ),
        "expect_desc": "确定当前节气为霜降，太阳平黄经及15度内节气进阶度"
    },
    {
        "id": 109,
        "tool": "jieqi_year",
        "category": "天星小成图与知识库",
        "name_zh": "全公历年二十四节气公历时刻精密时刻表",
        "input": {
            "year": 2026
        },
        "description": "推算2026全年二十四节气精确儒略日与公历秒级时刻",
        "validate": lambda d: (
            d.get("year") == 2026 and
            d.get("jieqi_count") == 24 and
            isinstance(d.get("annual_jieqi_schedule"), list) and
            len(d["annual_jieqi_schedule"]) == 24
        ),
        "expect_desc": "完整生成2026年立春至大寒24节气精确时刻表"
    },
    {
        "id": 110,
        "tool": "calendar_month",
        "category": "天星小成图与知识库",
        "name_zh": "万年历公农历干支交节与月相矩阵",
        "input": {
            "year": 2026,
            "month": 10
        },
        "description": "生成2026年10月份30天/31天完整日历：公历、农历、干支、建除、节气",
        "validate": lambda d: (
            d.get("year") == 2026 and
            d.get("month") == 10 and
            d.get("days_count") >= 30 and
            isinstance(d.get("calendar_days"), list) and
            len(d["calendar_days"]) == d["days_count"]
        ),
        "expect_desc": "完整生成10月份全月逐日命理干支与农历数据矩阵"
    }
]
