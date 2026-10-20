use serde::Serialize;

use crate::scale::{
    HTMLElement, PlainText, QuestionOption, Scale, ScaleCategory, SentenceItem, Tag, Texts,
};

mod questions;
use self::questions::QUESTIONS;

mod content;
mod dimensions;
mod interpretation;
mod norms;
mod temperaments;
use self::content::INSTRUCTION;
use self::content::INTRODUCTION;
use self::interpretation::INTERPRETATION;

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
    /// Psychoticism（精神质/倔强性）
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

pub const EYSENCK_PERSONALITY_QUESTIONNAIRE_REVISED_SHORT_SCALE: Scale<Interpretation, Question> =
    Scale {
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
        references: Some(&[
            "钱铭怡, 武国城, 朱荣春, 张莘. 艾森克人格问卷简式量表中国版（EPQ-RSC）的修订. 心理学报, 2000, 32(3): 317-323.",
        ]),
        warning: Some(
            "本量表仅适用 16 岁以上的人群。 版权提示：本量表的正式版本受版权保护，需通过官方渠道获得授权后使用；此处数据为公开资料转写，仅供个人学习参考，不得用于商业或临床用途。",
        ),
        tags: &Tag {
            info: Some(&["人格"]),
            normal: None,
            warning: Some(&["16+"]),
            error: None,
        },
        interpretation: INTERPRETATION,
        questions: QUESTIONS,
    };
