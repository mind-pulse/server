use serde::Serialize;

use crate::scale::{
    HTMLElement, PlainText, QuestionOption, Scale, ScaleCategory, SentenceItem, Tag, Texts,
};

mod questions;
use self::questions::QUESTIONS;

mod content;
mod interpretation;
mod norms;
use self::content::INSTRUCTION;
use self::content::INTRODUCTION;
use self::interpretation::INTERPRETATION;

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 维度
enum Dimension {
    /// 神经质/情绪稳定性，Neuroticism (Negative Emotionality)
    N,
    /// 外向性，Extraversion
    E,
    /// 开放性，Openness to Experience (Intellect/Imagination)
    O,
    /// 宜人性，Agreeableness
    A,
    /// 尽责性，Conscientiousness
    C,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Subdimension {
    N(NegativeEmotionalitySubdimension),
    E(ExtraversionSubdimension),
    O(OpenMindednessSubdimension),
    A(AgreeablenessSubdimension),
    C(ConscientiousnessSubdimension),
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 神经质子维度
enum NegativeEmotionalitySubdimension {
    /// 焦虑
    N1,
    /// 愤怒、敌意
    N2,
    /// 抑郁
    N3,
    /// 自我意识/人际关系敏感
    N4,
    /// 冲动性
    N5,
    /// 脆弱性
    N6,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 开放性子维度
enum OpenMindednessSubdimension {
    /// 想象力
    O1,
    /// 审美/艺术美感
    O2,
    /// 感受丰富度
    O3,
    /// 尝鲜。体验新鲜事物，而不是熟悉事物。
    O4,
    /// 思辨。求知。
    O5,
    /// 价值观
    O6,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 外向性子维度
enum ExtraversionSubdimension {
    /// 热情性
    E1,
    /// 乐群性
    E2,
    /// 自我肯定
    E3,
    /// 活跃性
    E4,
    /// 寻求刺激
    E5,
    /// 积极情感
    E6,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 宜人性子维度
enum AgreeablenessSubdimension {
    /// 信任
    A1,
    /// 坦城
    A2,
    /// 利他
    A3,
    /// 顺从
    A4,
    /// 谦逊
    A5,
    /// 同理心
    A6,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 尽责性/责任心子维度
enum ConscientiousnessSubdimension {
    /// 能力
    C1,
    /// 条理性
    C2,
    /// 责任感
    C3,
    /// 追求成就
    C4,
    /// 自律
    C5,
    /// 审慎
    C6,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct NormData {
    m: f64,
    sd: f64,
}

#[derive(Serialize)]
struct NormGender {
    dimension_norm: &'static [DimensionNorm; 5],
    subdimension_norm: &'static [SubdimensionNorm; 30],
}

#[derive(Serialize)]
struct DimensionNorm {
    dimension: Dimension,
    data: NormData,
}

#[derive(Serialize)]
struct SubdimensionNorm {
    dimension: Subdimension,
    data: NormData,
}

#[derive(Serialize)]
struct Norm {
    male: NormGender,
    female: NormGender,
}

#[derive(Serialize)]
pub struct Question {
    title: PlainText,
    options: &'static [QuestionOption; 5],
    dimension: Dimension,
    subdimension: Subdimension,
}

#[derive(Serialize)]
struct DimensionInterpretation {
    dimension: Dimension,
    name: PlainText,
    terminology: PlainText,
    description: PlainText,
    analysis: &'static [PlainText; 10],
    subdimension_interpretations: &'static [SubdimensionInterpretation; 6],
}

#[derive(Serialize)]
struct SubdimensionInterpretation {
    dimension: Subdimension,
    name: PlainText,
    description: PlainText,
    analysis: &'static [PlainText; 10],
}

#[derive(Serialize)]
pub struct Interpretation {
    norm: Norm,
    dimensions: &'static [DimensionInterpretation; 5],
}

// ===================== 神经质（N）子维度解释 =====================

// ===================== 外向性（E）子维度解释 =====================

// ===================== 开放性（O）子维度解释 =====================

// ===================== 宜人性（A）子维度解释 =====================

// ===================== 尽责性（C）子维度解释 =====================

// ===================== 核心维度解释 =====================

pub const NEO_PERSONALITY_INVENTORY_REVISED: Scale<Interpretation, Question> = Scale {
    id: 1,
    name: "大五人格测试",
    abbreviation: "NEO-PI-R",
    description: "五大维度，一眼看懂你独一无二的性格底色",
    primary_category: ScaleCategory::Personality,
    related_categories: Some(&[
        ScaleCategory::Emotion,
        ScaleCategory::MentalHealth,
        ScaleCategory::Interpersonal,
        ScaleCategory::CareerAndAcademics,
        ScaleCategory::Wellbeing,
    ]),
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: Some(&[
        "人格理论有几种，大五人格特质理论是近年来最受重用和欢迎的一个，也是最简单实用的一个，它将人格归为5种特质，这5种特质一般翻译为：外倾性、宜人性、严谨性、神经质和开放性。",
        "有学者称大五人格特质理论为“人格心理学领域的一场静悄悄的革命”，并认为大五特质理论是适合全人类的，它在过去的半个多世纪里被心理学研究者证明具有跨语言、跨文化、跨情景、跨评定者的一致性和稳定性。",
        "需要说明的是：下面看到的是你的人格倾向，不是心理疾病诊断。人格特质没有绝对的好坏，关键是看是否适合当下的情境。",
    ]),
    references: Some(&[
        "Costa, P. T., & McCrae, R. R. (1992). Revised NEO Personality Inventory (NEO-PI-R) and NEO Five-Factor Inventory (NEO-FFI) professional manual. Psychological Assessment Resources.",
    ]),
    warning: Some(
        "适用于初中三年级以上文化水平的人群。 版权提示：本量表的正式版本受版权保护，需通过官方渠道获得授权后使用；此处数据为公开资料转写，仅供个人学习参考，不得用于商业或临床用途。",
    ),
    tags: &Tag {
        info: Some(&["人格"]),
        normal: None,
        warning: Some(&["初三+"]),
        error: None,
    },
    interpretation: INTERPRETATION,
    questions: QUESTIONS,
};
