use serde::Serialize;

use crate::scale::category::ScaleCategory;
use crate::scale::common::{
    HTMLElement, PlainText, QuestionOption, Scale, SentenceItem, Tag, Texts,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct NormDimension {
    m: f64,
    sd: f64,
}

#[derive(Debug, Serialize)]
struct NormRange {
    range: [Option<u8>; 2],
    #[serde(rename(serialize = "P"))]
    p: NormDimension,
    #[serde(rename(serialize = "E"))]
    e: NormDimension,
    #[serde(rename(serialize = "N"))]
    n: NormDimension,
    #[serde(rename(serialize = "L"))]
    l: NormDimension,
}

#[derive(Debug, Serialize)]
struct Norm {
    male: [NormRange; 7],
    female: [NormRange; 7],
}

const NORM: Norm = Norm {
    male: [
        NormRange {
            range: [Some(16), Some(19)],
            p: NormDimension { m: 3.15, sd: 1.82 },
            e: NormDimension { m: 7.74, sd: 2.77 },
            n: NormDimension { m: 4.70, sd: 2.96 },
            l: NormDimension { m: 4.43, sd: 2.55 },
        },
        NormRange {
            range: [Some(20), Some(29)],
            p: NormDimension { m: 3.00, sd: 2.00 },
            e: NormDimension { m: 8.05, sd: 2.67 },
            n: NormDimension { m: 4.57, sd: 3.06 },
            l: NormDimension { m: 4.90, sd: 2.66 },
        },
        NormRange {
            range: [Some(30), Some(39)],
            p: NormDimension { m: 2.88, sd: 2.04 },
            e: NormDimension { m: 7.82, sd: 2.68 },
            n: NormDimension { m: 4.01, sd: 2.78 },
            l: NormDimension { m: 5.61, sd: 2.66 },
        },
        NormRange {
            range: [Some(40), Some(49)],
            p: NormDimension { m: 2.91, sd: 2.34 },
            e: NormDimension { m: 7.34, sd: 2.88 },
            n: NormDimension { m: 4.34, sd: 2.95 },
            l: NormDimension { m: 6.55, sd: 2.78 },
        },
        NormRange {
            range: [Some(50), Some(59)],
            p: NormDimension { m: 2.67, sd: 2.21 },
            e: NormDimension { m: 6.95, sd: 2.98 },
            n: NormDimension { m: 3.90, sd: 2.89 },
            l: NormDimension { m: 7.19, sd: 2.66 },
        },
        NormRange {
            range: [Some(60), Some(69)],
            p: NormDimension { m: 2.68, sd: 2.31 },
            e: NormDimension { m: 7.08, sd: 3.01 },
            n: NormDimension { m: 3.70, sd: 3.00 },
            l: NormDimension { m: 7.73, sd: 3.08 },
        },
        NormRange {
            range: [Some(70), None],
            p: NormDimension { m: 2.92, sd: 2.79 },
            e: NormDimension { m: 6.89, sd: 3.08 },
            n: NormDimension { m: 4.38, sd: 3.39 },
            l: NormDimension { m: 8.00, sd: 3.13 },
        },
    ],
    female: [
        NormRange {
            range: [Some(16), Some(19)],
            p: NormDimension { m: 2.63, sd: 1.81 },
            e: NormDimension { m: 8.13, sd: 2.58 },
            n: NormDimension { m: 4.93, sd: 2.75 },
            l: NormDimension { m: 4.86, sd: 2.43 },
        },
        NormRange {
            range: [Some(20), Some(29)],
            p: NormDimension { m: 2.68, sd: 1.82 },
            e: NormDimension { m: 7.44, sd: 2.79 },
            n: NormDimension { m: 4.81, sd: 2.95 },
            l: NormDimension { m: 5.32, sd: 2.70 },
        },
        NormRange {
            range: [Some(30), Some(39)],
            p: NormDimension { m: 2.44, sd: 1.82 },
            e: NormDimension { m: 7.50, sd: 2.87 },
            n: NormDimension { m: 4.49, sd: 2.89 },
            l: NormDimension { m: 6.64, sd: 2.76 },
        },
        NormRange {
            range: [Some(40), Some(49)],
            p: NormDimension { m: 2.55, sd: 2.30 },
            e: NormDimension { m: 7.15, sd: 2.86 },
            n: NormDimension { m: 4.44, sd: 2.95 },
            l: NormDimension { m: 7.45, sd: 2.98 },
        },
        NormRange {
            range: [Some(50), Some(59)],
            p: NormDimension { m: 2.36, sd: 1.82 },
            e: NormDimension { m: 6.92, sd: 2.90 },
            n: NormDimension { m: 4.48, sd: 2.88 },
            l: NormDimension { m: 7.73, sd: 2.68 },
        },
        NormRange {
            range: [Some(60), Some(69)],
            p: NormDimension { m: 2.51, sd: 1.98 },
            e: NormDimension { m: 7.28, sd: 2.95 },
            n: NormDimension { m: 4.44, sd: 3.12 },
            l: NormDimension { m: 7.72, sd: 2.96 },
        },
        NormRange {
            range: [Some(70), None],
            p: NormDimension { m: 2.32, sd: 1.89 },
            e: NormDimension { m: 7.28, sd: 3.48 },
            n: NormDimension { m: 4.88, sd: 3.25 },
            l: NormDimension { m: 8.84, sd: 2.58 },
        },
    ],
};

#[derive(Debug, Serialize)]
pub struct ScoreInterpretItem {
    title: PlainText,
    content: PlainText,
}

#[derive(Debug, Serialize)]
struct HighLow {
    high: &'static [ScoreInterpretItem],
    low: &'static [ScoreInterpretItem],
}

#[derive(Debug, Serialize)]
struct ScoreInterpret {
    /// 中间型
    medium: &'static [ScoreInterpretItem],
    /// 倾向型
    moderate: HighLow,
    /// 典型
    extreme: HighLow,
}

#[derive(Debug, Serialize)]
struct DimensionInterpretation {
    /// 维度名称
    label: PlainText,
    /// 重要提示
    notice: Option<PlainText>,
    /// 核心解释
    core_explain: PlainText,
    /// 补充说明
    supplementary: Option<PlainText>,
    /// 得分水平解读
    score_interpret: ScoreInterpret,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct Dimensions {
    e: &'static DimensionInterpretation,
    n: &'static DimensionInterpretation,
    p: &'static DimensionInterpretation,
    l: &'static DimensionInterpretation,
}

#[derive(Debug, Serialize)]
pub struct Interpretation {
    norm: Norm,
    dimensions: Dimensions,
    temperaments: Temperaments,
    // todo: 结果解释暂时直接写在前端，需要后续移动回后端
}

#[derive(Debug, Serialize)]
enum Dimension {
    /// Extraversion/Introversion，内外倾向量表
    E,
    /// Neuroticism/Stability，情绪性量表
    N,
    /// Psychoticism/Socialisation，心理变态量表，又称精神质
    P,
    /// Lie/Social Desirability，效度量表
    L,
}

#[derive(Debug, Serialize)]
struct SceneSuggestion {
    study: PlainText, // 学习建议
    life: PlainText,  // 生活建议
    work: PlainText,  // 工作/职业建议
}

// 气质类型
#[derive(Debug, Serialize)]
struct Temperament {
    core_trait: PlainText,             // 核心特质（通俗版）
    daily_performance: PlainText,      // 日常表现（接地气描述）
    scene_suggestion: SceneSuggestion, // 场景建议（学习/生活/工作）
    notice: PlainText,                 // 注意事项（避坑/发挥优势）
    summary: PlainText,                // 极简总结（一句话记住）
}

#[derive(Debug, Serialize)]
struct Temperaments {
    sanguine: &'static Temperament,    // 多血质
    choleric: &'static Temperament,    // 胆汁质
    melancholic: &'static Temperament, // 抑郁质
    phlegmatic: &'static Temperament,  // 粘液质
}

#[derive(Debug, Serialize)]
pub struct Question {
    title: PlainText,
    options: &'static [QuestionOption],
    dimension: Dimension,
}

/// 气质类型 - 多血质（sanguine）
const SANGUINE: Temperament = Temperament {
    core_trait: "多血质（活泼型）：天生反应快、爱社交、适应力强，注意力容易分散（无好坏，只是先天倾向）",
    daily_performance: "跟人相处轻松不尴尬，学新东西上手快，但做长期枯燥的事容易没耐心；情绪来得快去得也快，很少钻牛角尖",
    scene_suggestion: SceneSuggestion {
        study: "适合拆分长任务（比如把背书拆成20分钟一段），用小组讨论、边讲边学的方式（比如给同学讲题），避免独自死记硬背",
        life: "可以多参加社交、运动类活动释放精力，但要留一点独处时间（比如睡前10分钟安静看书），避免精力透支",
        work: "适配需要临场应变、对外沟通的岗位（销售、活动策划、主持人、客服）；避开长期独处、重复机械的工作（如仓库管理、数据录入）"
    },
    notice: "优势是灵活善交际，短板是容易三分钟热度；可以用“定小目标+及时奖励”的方式，弥补专注力不足的问题",
    summary: "活泼灵活，适配多变场景，注意稳住专注力"
};

/// 气质类型 - 胆汁质（choleric）
const CHOLERIC: Temperament = Temperament {
    core_trait: "胆汁质（冲动型）：天生精力足、做事果断、脾气急，情绪反应强烈（无好坏，只是先天倾向）",
    daily_performance: "做事情雷厉风行，喜欢主导和挑战，但容易因为小事发火；目标感强，不服输，讨厌拖沓",
    scene_suggestion: SceneSuggestion {
        study: "适合短时间高强度冲刺（比如考前集中刷题），避免磨磨蹭蹭；生气时先停1分钟再学习，别带着情绪做题",
        life: "可以通过跑步、拳击等运动释放情绪，少跟人硬碰硬；说话时放慢语速，避免脱口而出伤到人",
        work: "适配需要决策、抗压、快速行动的岗位（导游、销售管理、应急救援、演讲）；避开需要耐心、反复沟通的工作（如客服、心理咨询）"
    },
    notice: "优势是果敢有冲劲，短板是容易急躁；可以养成“先思考后行动”的习惯，比如做决定前数3秒",
    summary: "果敢有冲劲，适配高压场景，注意控制情绪"
};

/// 气质类型 - 抑郁质（melancholic）
const MELANCHOLIC: Temperament = Temperament {
    core_trait: "抑郁质（敏感型）：天生观察细、想得多、重细节，情绪体验深（无好坏，和抑郁症无关）",
    daily_performance: "能注意到别人忽略的细节，做事追求完美，但容易多愁善感；不喜欢热闹，独处时更放松",
    scene_suggestion: SceneSuggestion {
        study: "适合深度思考的方式（比如精读、整理笔记、复盘错题），避开嘈杂的学习环境；别过度纠结细节，先完成再完美",
        life: "可以写日记梳理情绪，少参加人多的聚会，跟1-2个知心朋友相处就好；别胡思乱想，不确定的事直接问清楚",
        work: "适配需要细心、专注、独立思考的岗位（校对、设计师、科研、档案管理、艺术创作）；避开需要频繁社交、快速决策的工作（如销售、公关）"
    },
    notice: "优势是细心有深度，短板是容易内耗；可以用“列清单”的方式，减少胡思乱想，聚焦具体事",
    summary: "细腻有深度，适配精细场景，注意别内耗"
};

/// 气质类型 - 粘液质（phlegmatic）
const PHLEGMATIC: Temperament = Temperament {
    core_trait: "粘液质（稳重型）：天生冷静、有耐心、能坚持，反应偏慢（无好坏，只是先天倾向）",
    daily_performance: "做事慢条斯理但不容易出错，不喜欢变来变去；情绪稳定，很少大喜大悲，是身边人的“定心丸”",
    scene_suggestion: SceneSuggestion {
        study: "适合长期积累的方式（比如每天背10个单词），避开临时抱佛脚；学新东西时别着急，给自己足够的适应时间",
        life: "可以尝试偶尔突破舒适区（比如学一个新技能），别太循规蹈矩；跟急性子的人相处时，提前沟通节奏",
        work: "适配需要稳重、耐心、长期坚持的岗位（医生、法官、会计、教师、心理咨询）；避开需要快速应变、频繁变动的工作（如创业、直播）"
    },
    notice: "优势是稳重有耐力，短板是不够灵活；可以多关注新信息，偶尔尝试“快速做小决定”，提升应变力",
    summary: "稳重有耐力，适配长期场景，注意提升灵活度"
};

/// 人格维度 - 内外倾向（E）
const E_DIMENSION: DimensionInterpretation = DimensionInterpretation {
    label: "内外倾向",
    notice: None,
    core_explain: "这个维度看你“精力来源”：外向的人靠社交充电，内向的人靠独处充电，没有好坏之分",
    supplementary: Some(
        "人格是复杂的，即使外向的人也需要独处，内向的人也能做好社交，关键是适配自己的节奏",
    ),
    score_interpret: ScoreInterpret {
        medium: &[
            ScoreInterpretItem {
                title: "核心特点",
                content: "既喜欢跟人玩，也能独处，精力分配比较均衡",
            },
            ScoreInterpretItem {
                title: "实用建议",
                content: "学习/工作可以“社交+独处”结合（比如上午小组讨论，下午独自整理）",
            },
        ],
        moderate: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "喜欢社交，精力靠跟人相处充电，做事偏主动",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "用小组讨论、边讲边学的方式，避免长期独自死记硬背",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "多参加社交活动，但每天留10分钟独处，避免精力透支",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选团队协作、对外沟通的岗位（销售、运营），避开长期独处的工作",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "喜欢独处，精力靠安静环境充电，做事偏谨慎",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "选安静的环境学习，用精读、整理笔记的方式，避开频繁小组讨论",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "少参加人多的聚会，跟1-2个朋友相处就好，别勉强自己社交",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选独立工作、专注思考的岗位（科研、设计），避开频繁对外沟通的工作",
                },
            ],
        },
        extreme: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "几乎离不开社交，一独处就觉得无聊，做事特别主动",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "刻意留独处时间（比如睡前看书），避免过度依赖外界认可",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "别因为太想社交，忽略自己的真实感受",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "非常不喜欢社交，甚至回避多人场合，独处时最放松",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "必要社交前做心理建设（比如提前想好想说的话），别完全封闭自己",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "别因为怕社交，错过重要的机会（比如面试、合作）",
                },
            ],
        },
    },
};

/// 人格维度 - 神经质/情绪稳定性（N）
const N_DIMENSION: DimensionInterpretation = DimensionInterpretation {
    label: "神经质",
    notice: Some("这是正常人格特质，和“精神病/心理疾病”无关！"),
    core_explain:
        "这个维度看你“情绪波动程度”：高分=情绪容易波动（敏感），低分=情绪稳定（冷静），无好坏",
    supplementary: Some("情绪波动大的人更能感知细节，情绪稳定的人更能抗压，只是适配不同场景"),
    score_interpret: ScoreInterpret {
        medium: &[
            ScoreInterpretItem {
                title: "核心特点",
                content: "情绪不算特别稳定，但也不会轻易崩溃，能应对日常压力",
            },
            ScoreInterpretItem {
                title: "实用建议",
                content: "压力大时做3次深呼吸，别立刻做决定",
            },
        ],
        moderate: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "情绪容易波动，一点小事可能开心/难过很久，对细节特别敏感",
                },
                ScoreInterpretItem {
                    title: "学习建议",
                    content: "情绪不好时先停学，做5分钟拉伸，别带着情绪做题",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "写日记梳理情绪，别憋在心里；少刷负面信息，避免情绪内耗",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选能发挥“敏感优势”的岗位（文案、设计、用户调研），避开高压应急的工作",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "情绪特别稳定，很少生气/难过，面对压力也能冷静",
                },
                ScoreInterpretItem {
                    title: "学习建议",
                    content: "可以偶尔逼自己“感性一点”，比如多思考题目背后的逻辑，别只看答案",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "多关注身边人的情绪，别因为自己冷静，忽略别人的感受",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选高压、需要决策的岗位（管理、应急救援），避开需要细腻感知的工作",
                },
            ],
        },
        extreme: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "情绪极易波动，容易焦虑、紧张，一点小事就想很多",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "每天花10分钟冥想，把担心的事写下来，逐个解决，别想太多",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "别因为情绪不好，否定自己的能力",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "情绪过于稳定，甚至有点“冷漠”，对开心/难过的事反应淡",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "刻意体验情绪（比如看一部感人的电影），别让自己变得太麻木",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "别因为自己冷静，忽略身边人的情感需求",
                },
            ],
        },
    },
};

/// 人格维度 - 精神质（P）
const P_DIMENSION: DimensionInterpretation = DimensionInterpretation {
    label: "精神质",
    notice: Some("这是正常人格特质，和“精神病/心理疾病”无关！"),
    core_explain:
        "这个维度看你“是否从众”：高分=更独立、不盲从规则，低分=更温和、愿意配合别人，无好坏",
    supplementary: Some("独立的人适合创新，配合的人适合团队，只是适配不同角色"),
    score_interpret: ScoreInterpret {
        medium: &[
            ScoreInterpretItem {
                title: "核心特点",
                content: "既愿意配合别人，也有自己的想法，不会完全盲从",
            },
            ScoreInterpretItem {
                title: "实用建议",
                content: "团队中可以先听别人的意见，再说出自己的想法，兼顾协作和独立",
            },
        ],
        moderate: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "特别独立，不喜欢被规则束缚，做事有自己的节奏，不太在意别人的看法",
                },
                ScoreInterpretItem {
                    title: "学习建议",
                    content: "按自己的方法学习，别照搬别人的计划；遇到不同意见，先听再反驳",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "可以保留自己的个性，但别故意跟人对着干，避免冲突",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content:
                        "选需要创新、独立决策的岗位（创业、科研、设计），避开需要严格遵守规则的工作",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "特别温和，愿意配合别人，在意身边人的感受，不喜欢冲突",
                },
                ScoreInterpretItem {
                    title: "学习建议",
                    content: "别因为怕别人不高兴，不敢提自己的问题；可以主动跟同学交流想法",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "保留自己的底线，别一味迁就别人，避免委屈自己",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content:
                        "选需要团队协作、沟通的岗位（人力资源、客服、教师），避开需要独断决策的工作",
                },
            ],
        },
        extreme: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "极度独立，甚至有点“自我”，完全不按规则来，不在意别人的评价",
                },
                ScoreInterpretItem {
                    title: "学习建议",
                    content: "按自己的方法学习，别照搬别人的计划；遇到不同意见，先听再反驳",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content:
                        "选需要创新、独立决策的岗位（创业、科研、设计），避开需要严格遵守规则的工作",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "可以保留自己的个性，但别故意跟人对着干，避免冲突",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "适当考虑别人的感受，别因为太自我，影响人际关系",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "创新也要兼顾实际，别脱离团队/规则空谈想法",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "极度温和，甚至有点“没主见”，总是迁就别人，害怕冲突",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "刻意练习说“不”，比如不想帮别人做事时，直接说明原因",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "别因为迁就别人，放弃自己的目标和想法",
                },
            ],
        },
    },
};

/// 人格维度 - 效度/掩饰性（L）
const L_DIMENSION: DimensionInterpretation = DimensionInterpretation {
    label: "掩饰性",
    notice: None,
    core_explain:
        "这个维度看你“回答是否真实”：高分=可能想展现自己的好形象，低分=回答更坦诚，仅参考测试效度",
    supplementary: None,
    score_interpret: ScoreInterpret {
        medium: &[
            ScoreInterpretItem {
                title: "核心特点",
                content: "回答比较真实，没有刻意隐瞒，也没有刻意表现自己",
            },
            ScoreInterpretItem {
                title: "实用建议",
                content: "测试结果有参考价值，可以结合自己的实际情况看",
            },
        ],
        moderate: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "可能想展现自己“更好”的一面，回答时有所隐瞒",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "可以重新做一次测试，按真实想法回答，结果会更准",
                },
                ScoreInterpretItem {
                    title: "提醒",
                    content: "这不是“不诚实”，只是想给人留好印象，很正常",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "回答特别坦诚，完全按自己的真实想法来，没有隐瞒",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "测试结果参考性高，可以重点看自己的人格倾向",
                },
            ],
        },
        extreme: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "回答时过度掩饰，可能完全没按真实想法来",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "隔几天再做一次测试，放松心态，别刻意“装好人”，结果才有用",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "回答极度坦诚，甚至有点“口无遮拦”",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "测试结果很真实，但也要结合实际，别完全按测试下结论",
                },
            ],
        },
    },
};

const INTRODUCTION: Texts = &[
    &[
        SentenceItem::Plain("艾森克人格问卷（Eysenck Personality Questionnaire, 简称 EPQ）是英国伦敦大学心理系和精神病研究所艾森克教授编制的，归纳了三个人格的基本因素："),
        SentenceItem::HTMLElement(HTMLElement::Strong("内外向性")),
        SentenceItem::Plain("(E)、"),
        SentenceItem::HTMLElement(HTMLElement::Strong("神经质")),
        SentenceItem::Plain("(又称情绪性)(N)和"),
        SentenceItem::HTMLElement(HTMLElement::Strong("精神质")),
        SentenceItem::Plain("(又称倔强、讲求实际)(P)。"),
    ],
    &[
        SentenceItem::Plain("三个人格度不但经过许多数学统计上的和行为观察方面的分析，而且也得到实验室内多种心理实验的考察，被广泛应用于医学、司法、教育和心理咨询等领域，适合各种人群测试。"),
    ],
    &[
        SentenceItem::Plain("艾森克人格问卷简式量表中国版在原量表的基础上针对中国进行了修订，在保持甚至提高信度的同时精简了部分题目。"),
    ]
];

const INSTRUCTION: Texts = &[&[
    SentenceItem::Plain("该量表共有 "),
    SentenceItem::HTMLElement(HTMLElement::Strong("48")),
    SentenceItem::Plain(
        " 个项目，请仔细阅读每一条，然后根据该句话与您自己的实际情况相符合的程度进行选择。",
    ),
]];

const INTERPRETATION: Interpretation = Interpretation {
    norm: NORM,
    temperaments: Temperaments {
        sanguine: &SANGUINE,
        choleric: &CHOLERIC,
        melancholic: &MELANCHOLIC,
        phlegmatic: &PHLEGMATIC,
    },
    dimensions: Dimensions {
        e: &E_DIMENSION,
        n: &N_DIMENSION,
        p: &P_DIMENSION,
        l: &L_DIMENSION,
    },
};

pub const EYSENCK_PERSONALITY_QUESTIONNAIRE_REVISED_SHORT_SCALE: Scale<Interpretation, Question> = Scale {
    id: 4,
    name: "艾森克人格问卷简式量表中国版",
    description: "源自国际经典、专为中国优化的性格探索工具",
    primary_category: ScaleCategory::Personality,
    related_categories: Some(&[ScaleCategory::Emotion, ScaleCategory::MentalHealth]),
    abbreviation: "EPQ-RSC",
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: Some(&[
        "此测试能够帮助你了解自己性格中的核心特质，它基于经典的心理学理论，将人格分为情绪稳定性、内外向倾向、行事风格三个主要维度。",
        "它的原理就像描绘一幅性格简笔画：通过一系列生活情境中的感受与选择，客观地勾勒出你在这些维度上的位置。中国版量表经过了严谨的调整，使其更贴合我们的文化背景和日常表达。",
        "在科学性上，该量表拥有良好的信度与效度。简单来说，就是测试结果稳定可靠，且能真实反映它想要测量的人格特质，因此被广泛应用于心理健康、职业评估和个人成长等领域。",
        "你可以将测试结果视为一份实用的“性格参考地图”，它不能定义你，但能帮助你更清晰、更系统地观察自己，为自我了解与提升提供一个科学的起点。",
    ]),
    references: Some(&["钱铭怡等. 艾森克人格问卷简式量表中国版（EPQ-RSC）的修订. 心理学报. 2000"]),
    formula_mode: None,
    warning: Some("本量表仅适用 16 岁以上的人群。"),
    tags: &Tag{ info: Some(&[ "人格"]), normal: None, warning: Some(&["16+"]), error: None },
    interpretation: INTERPRETATION,
    questions: &[
        Question {
            title: "你的情绪是否时起时落?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "当你看到小孩(或动物)受折磨时是否感到难受?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是个健谈的人吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果你说了要做什么事，是否不论此事可能如果不顺利你都总能遵守诺言?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否会无言无故地感到“很惨”?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "欠债会使你感到忧虑吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是个生气勃勃的人吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否曾贪图过超过你应得的分外之物?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是个容易被激怒的人吗?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你会服用能产生奇异或危险效果的药物吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你愿意认识陌生人吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否曾经有过明知自己做错了事却责备别人的情况?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你的感情容易受伤害吗?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否愿意按照自己的方式行事，而不愿意按照规则办事?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在热闹的聚会中你能使自己放得开，使自己玩得开心吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你所有的习惯是否都是好的?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否时常感到“极其厌倦”?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "良好的举止和整洁对你来说很重要吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "在结交新朋友时，你经常是积极主动的吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否有过随口骂人的时候?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你认为自己是一个胆怯不安的人吗?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否认为婚姻是不合时宜的，应该废除?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你能否很容易地给一个沉闷的聚会注入活力?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你曾毁坏或丢失过别人的东西吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是个忧心忡忡的人吗?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你爱和别人合作吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "在社交场合你是否倾向于呆在不显眼的地方?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "如果在你的工作中出现了错误，你知道后会感到忧虑吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你讲过别人的坏话或脏话吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你认为自己是个神经紧张或“弦绷得过紧”的人吗?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否觉得人们为了未来有保障，而在储蓄和保险方面花费的时间太多了?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否喜欢和人们相处在一起?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "当你还是个小孩子的时候，你是否曾有过对父母耍赖或不听话的行为?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "在经历了一次令人难堪的事之后，你是否会为此烦恼很长时间?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否努力使自己对人不粗鲁?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是否喜欢在自己周围有许多热闹和令人兴奋的事情?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你曾在玩游戏时作过弊吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是否因自己的“神经过敏”而感到痛苦?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你愿意别人怕你吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你曾利用过别人吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是否喜欢说笑话和谈论有趣的事?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否时常感到孤独?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否认为遵循社会规范比按照个人方式行事更好一些?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "在别人眼里你总是充满活力的吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你总能做到言行一致吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否时常被内疚感所困扰?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你有时将今天该做的事情拖到明天去做吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你能使一个聚会顺利进行下去吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
    ]
};
