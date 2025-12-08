mod bdi;
mod epq_rsc;
mod ept;
mod h_sds;
mod hamd;
mod neo_pi_r;
mod sas;
mod scl90;
mod sds;
mod sixteen_pf;
mod y_bocs;

use serde::Serialize;

use crate::error::{MindPulseError, MindPulseResult};

pub(super) use self::bdi::BECK_DEPRESSION_RATING_SCALE;
pub(super) use self::epq_rsc::EPQ_RSC;
pub(super) use self::ept::ENNEAGRAM_PERSONALITY_TEST;
pub(super) use self::h_sds::SELF_DIRECTED_SEARCH;
pub(super) use self::hamd::HAMILTON_DEPRESSION_SCALE;
pub(super) use self::neo_pi_r::REVISED_NEOPERSONALITY_INVENTORY;
pub(super) use self::sas::SELF_RATING_ANXIETY_SCALE;
pub(super) use self::scl90::SYMPTOM_CHECKLIST_90;
pub(super) use self::sds::SELF_RATING_DEPRESSION_SCALE;
pub(super) use self::sixteen_pf::SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE;
pub(super) use self::y_bocs::YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE;

/// 心理学自评量表的顶层分类枚举（覆盖全面、结构稳定）
/// 每个变体下方注释列出该类别的经典量表示例（中文名 + 英文名 + 说明），
/// 所有量表均为公开使用、有中文版、且经信效度验证。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ScaleCategory {
    /// 人格特质
    /// - 艾森克人格问卷（EPQ）
    /// - 卡特尔16种人格因素问卷（16PF）
    /// - 中文版10项大五人格量表（TIPI-C）：结构清晰，α系数 >0.7，广泛用于快速人格筛查
    /// - 大五人格量表（NEO-PI-R）：
    Personality,

    /// 情绪与情感状态
    /// - 抑郁自评量表（SDS）
    /// - 焦虑自评量表（SAS）
    /// - 患者健康问卷-9项（PHQ-9）：抑郁筛查，中文版α=0.86–0.92
    /// - 广泛性焦虑障碍量表（GAD-7）：焦虑筛查，α>0.85
    /// - 正负性情绪量表（PANAS）
    Emotion,

    /// 心理健康综合评估
    /// - 症状自评量表（SCL-90）：90项，10个因子，使用最广的心理健康筛查工具
    /// - 一般健康问卷（GHQ-12/28）：评估近期心理功能状态
    /// - Kessler心理困扰量表（K10）：用于社区心理风险筛查
    MentalHealth,

    /// 认知与能力（自评类）
    /// - 自我效能感量表（GSES）：感知应对能力，α≈0.85
    /// - 元认知问卷（MAI）
    /// - 大学生学习策略量表（MSLQ）
    /// - 注意力/记忆力自评量表（如Cognitive Failures Questionnaire, CFQ）
    CognitionAndAbility,

    /// 职业与学业发展
    /// - 霍兰德职业兴趣量表（Holland Vocational Preference Inventory）
    /// - 马勒奇职业倦怠量表（MBI）：三维度（情绪耗竭、去人格化、个人成就感）
    /// - 大学生学业压力量表（含中文常模）
    /// - 生涯决策自我效能量表（CDMSE）
    CareerAndAcademics,

    /// 行为问题与成瘾
    /// - Young网络成瘾量表（YDQ）：中文版广泛用于青少年研究
    /// - Barratt冲动性量表（BIS-11）
    /// - 饮酒识别测试（AUDIT）：WHO推荐，含中文版
    /// - 儿童/青少年行为量表自评版（YSR/ASR）
    Behavior,

    /// 应激、创伤与应对
    /// - 感知压力量表（PSS-10）：中文版信效度良好
    /// - 简易应对方式问卷（SCSQ）：积极/消极应对，中国学者编制
    /// - 生活事件量表（LES）：评估近期应激源
    /// - 创伤后应激障碍检查表（PCL-5）：DSM-5标准，中文版已验证
    #[allow(unused)]
    StressAndCoping,

    /// 人际关系与社会支持
    /// - 人际关系综合诊断量表（ICDS）
    /// - 人际反应指针（IRI）：共情能力评估
    /// - 领悟社会支持量表（MSPSS）：家庭、朋友、他人三维度
    /// - UCLA孤独量表（第3版）
    Interpersonal,

    /// 生活质量与幸福感
    /// - 世界卫生组织生活质量简表（WHOQOL-BREF）：4领域，中文版标准化
    /// - 生活满意度量表（SWLS）：5项，α>0.8
    /// - 主观幸福感量表（SHS）
    Wellbeing,

    /// 态度、价值观与动机
    /// - 核心自我评价量表（CSES）：12项，α=0.84，3个月重测r=0.81
    /// - 一般态度与信念量表（GABS）
    /// - 职业价值观问卷（Super, 1970）
    /// - 成就动机量表（AMS）
    AttitudeAndValues,

    /// 躯体化与身心症状
    /// - 躯体症状量表-8（SSS-8）：简短高效，中文版信效度良好
    /// - 躯体症状障碍B标准量表（SSD-12）：评估疾病焦虑与躯体关注
    /// - SCL-90中的“躯体化”因子也可单独使用
    Somatic,

    /// 其他未归类量表（兜底项）
    /// 例如：自悯量表（SCS-SF）、宗教信念量表、AI接受度量表等新兴或跨领域工具
    #[allow(unused)]
    Other,
}

type PlainText = &'static str;
type PlainTexts = &'static [PlainText];

#[derive(Debug, Serialize, Clone)]
pub(super) struct QuestionOption {
    text: PlainText,
    point: i8,
}

#[derive(Debug, Serialize)]
pub(super) struct Question {
    title: PlainText,
    is_multiple: bool,
    options: &'static [QuestionOption],
}

#[derive(Debug, Serialize)]
pub(super) struct InterpretationItem<I> {
    range: [I; 2],
    description: PlainText,
    #[serde(skip_serializing_if = "Option::is_none")]
    advice: Option<PlainTexts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symptom: Option<PlainTexts>,
    status: Status,
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub(super) enum OperationalRule {
    Multiply(f64),
    // Divide(f64),
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "UPPERCASE"))]
pub(super) enum Integer {
    // Floor,
    // Ceil,
    /// 四舍五入
    Round,
}

#[derive(Debug, Serialize)]
pub(super) struct FormulaMode {
    operational_rule: OperationalRule,
    integer: Option<Integer>,
}

#[derive(Debug, Serialize)]
pub(super) struct Scale<'r, I, Q> {
    name: PlainText,
    /// 分类
    primary_category: ScaleCategory,
    /// 相关分类
    #[serde(skip_serializing_if = "Option::is_none")]
    related_categories: Option<&'r [ScaleCategory]>,
    abbreviation: PlainText,
    /// 对量表的说明
    instruction: Texts,
    questions: &'r [Q],
    interpretation: I,
    /// 简介
    introduction: Texts,
    #[serde(skip_serializing_if = "Option::is_none")]
    references: Option<PlainTexts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning: Option<PlainText>,
    /// js 计算用到的表达式，和用"<SUM>"替代
    #[serde(skip_serializing_if = "Option::is_none")]
    formula_mode: Option<FormulaMode>,
    tags: Tag,
    /// 理念
    idea: Option<PlainTexts>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub(super) enum Status {
    /// 正常
    Normal,
    /// 轻度
    Mild,
    /// 中度
    Moderate,
    /// 重度
    Major,
}

#[derive(Debug, Serialize)]
pub(super) struct Tag {
    info: Option<PlainTexts>,
    normal: Option<PlainTexts>,
    warning: Option<PlainTexts>,
    error: Option<PlainTexts>,
}

/// 特征
#[derive(Debug, Serialize)]
pub(super) struct Characteristic {
    low: PlainTexts,
    high: PlainTexts,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "text", rename_all = "UPPERCASE")]
pub(super) enum HTMLElement {
    Strong(PlainText),
    //Mark(PlainText),
    //A { text: PlainText, href: PlainText },
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub(super) enum SentenceItem {
    Plain(PlainText),
    HTMLElement(HTMLElement),
}

pub(super) type Sentence = &'static [SentenceItem];
pub(super) type Texts = &'static [Sentence];

#[derive(Debug, Serialize)]
pub(super) struct ScaleListItem<'r> {
    name: PlainText,
    path: PlainText,
    /// 分类
    primary_category: ScaleCategory,
    /// 相关分类
    #[serde(skip_serializing_if = "Option::is_none")]
    related_categories: Option<&'r [ScaleCategory]>,
    introduction: Texts,
    warning: Option<PlainText>,
    tags: Tag,
    disabled: bool,
}

impl ScaleListItem<'_> {
    pub(crate) fn name(&self) -> &str {
        self.name
    }
}

pub(super) const PATHS: [ScaleListItem<'static>; 11] = [
    ScaleListItem {
        name: SELF_DIRECTED_SEARCH.name,
        path: "h_sds",
        primary_category: SELF_DIRECTED_SEARCH.primary_category,
        related_categories: SELF_DIRECTED_SEARCH.related_categories,
        introduction: SELF_DIRECTED_SEARCH.introduction,
        warning: SELF_DIRECTED_SEARCH.warning,
        tags: SELF_DIRECTED_SEARCH.tags,
        disabled: false,
    },
    ScaleListItem {
        name: REVISED_NEOPERSONALITY_INVENTORY.name,
        path: "neo_pi_r",
        primary_category: REVISED_NEOPERSONALITY_INVENTORY.primary_category,
        related_categories: REVISED_NEOPERSONALITY_INVENTORY.related_categories,
        introduction: REVISED_NEOPERSONALITY_INVENTORY.introduction,
        warning: REVISED_NEOPERSONALITY_INVENTORY.warning,
        tags: REVISED_NEOPERSONALITY_INVENTORY.tags,
        disabled: false,
    },
    ScaleListItem {
        name: SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE.name,
        path: "16pf",
        primary_category: SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE.primary_category,
        related_categories: SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE.related_categories,
        introduction: SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE.introduction,
        warning: SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE.warning,
        tags: SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE.tags,
        disabled: false,
    },
    ScaleListItem {
        name: ENNEAGRAM_PERSONALITY_TEST.name,
        path: "ept",
        primary_category: ENNEAGRAM_PERSONALITY_TEST.primary_category,
        related_categories: ENNEAGRAM_PERSONALITY_TEST.related_categories,
        introduction: ENNEAGRAM_PERSONALITY_TEST.introduction,
        warning: ENNEAGRAM_PERSONALITY_TEST.warning,
        tags: ENNEAGRAM_PERSONALITY_TEST.tags,
        disabled: false,
    },
    ScaleListItem {
        name: EPQ_RSC.name,
        path: "epq_rsc",
        primary_category: EPQ_RSC.primary_category,
        related_categories: EPQ_RSC.related_categories,
        introduction: EPQ_RSC.introduction,
        warning: EPQ_RSC.warning,
        tags: EPQ_RSC.tags,
        disabled: false,
    },
    ScaleListItem {
        name: SYMPTOM_CHECKLIST_90.name,
        path: "scl90",
        primary_category: SYMPTOM_CHECKLIST_90.primary_category,
        related_categories: SYMPTOM_CHECKLIST_90.related_categories,
        introduction: SYMPTOM_CHECKLIST_90.introduction,
        warning: SYMPTOM_CHECKLIST_90.warning,
        tags: SYMPTOM_CHECKLIST_90.tags,
        disabled: false,
    },
    ScaleListItem {
        name: BECK_DEPRESSION_RATING_SCALE.name,
        path: "bdi",
        primary_category: BECK_DEPRESSION_RATING_SCALE.primary_category,
        related_categories: BECK_DEPRESSION_RATING_SCALE.related_categories,
        introduction: BECK_DEPRESSION_RATING_SCALE.introduction,
        warning: BECK_DEPRESSION_RATING_SCALE.warning,
        tags: BECK_DEPRESSION_RATING_SCALE.tags,
        disabled: false,
    },
    ScaleListItem {
        name: SELF_RATING_DEPRESSION_SCALE.name,
        path: "sds",
        primary_category: SELF_RATING_DEPRESSION_SCALE.primary_category,
        related_categories: SELF_RATING_DEPRESSION_SCALE.related_categories,
        introduction: SELF_RATING_DEPRESSION_SCALE.introduction,
        warning: SELF_RATING_DEPRESSION_SCALE.warning,
        tags: SELF_RATING_DEPRESSION_SCALE.tags,
        disabled: false,
    },
    ScaleListItem {
        name: SELF_RATING_ANXIETY_SCALE.name,
        path: "sas",
        primary_category: SELF_RATING_ANXIETY_SCALE.primary_category,
        related_categories: SELF_RATING_ANXIETY_SCALE.related_categories,
        introduction: SELF_RATING_ANXIETY_SCALE.introduction,
        warning: SELF_RATING_ANXIETY_SCALE.warning,
        tags: SELF_RATING_ANXIETY_SCALE.tags,
        disabled: false,
    },
    ScaleListItem {
        name: YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE.name,
        path: "y_bocs",
        primary_category: YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE.primary_category,
        related_categories: YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE.related_categories,
        introduction: YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE.introduction,
        warning: YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE.warning,
        tags: YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE.tags,
        disabled: false,
    },
    ScaleListItem {
        name: HAMILTON_DEPRESSION_SCALE.name,
        path: "hamd",
        primary_category: HAMILTON_DEPRESSION_SCALE.primary_category,
        related_categories: HAMILTON_DEPRESSION_SCALE.related_categories,
        introduction: HAMILTON_DEPRESSION_SCALE.introduction,
        warning: HAMILTON_DEPRESSION_SCALE.warning,
        tags: HAMILTON_DEPRESSION_SCALE.tags,
        disabled: false,
    },
];

pub(super) fn get_scale_index_by_path(path: &str) -> MindPulseResult<usize> {
    match PATHS.iter().position(|sp| sp.path == path) {
        None => {
            error!(message = "scale 无效", scale = path);
            Err(MindPulseError::Response("无效的 scale".to_owned()))
        }
        Some(idx) => Ok(idx),
    }
}
