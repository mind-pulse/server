use serde::Serialize;

use crate::scale::{
    FormulaMode, HTMLElement, Integer, OperationalRule, PlainText, PlainTexts, QuestionOption,
    Scale, ScaleCategory, SentenceItem, SymptomGuidance, Tag, Texts,
};

mod questions;
use self::questions::QUESTIONS;

mod content;
mod interpretation;
use self::content::INSTRUCTION;
use self::content::INTRODUCTION;
use self::interpretation::INTERPRETATION;

#[derive(Debug, Serialize, Hash, Eq, PartialEq)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
enum Symptom {
    /// 躯体化
    Somatization,
    /// 强迫症状
    ObsessiveCompulsive,
    /// 人际关系敏感
    InterpersonalSensitivity,
    /// 抑郁
    Depression,
    /// 焦虑
    Anxiety,
    /// 敌对
    Hostility,
    /// 恐怖
    PhobicAnxiety,
    /// 偏执
    ParanoidIdeation,
    /// 精神病性
    Psychoticism,
    /// 其他
    Others,
}

#[derive(Debug, Serialize)]
pub struct Question {
    title: PlainText,
    options: &'static [QuestionOption],
    symptom: Symptom,
}

const OPTIONS: [QuestionOption; 5] = [
    QuestionOption {
        text: "从无",
        point: 1,
    },
    QuestionOption {
        text: "很轻",
        point: 2,
    },
    QuestionOption {
        text: "中等",
        point: 3,
    },
    QuestionOption {
        text: "偏重",
        point: 4,
    },
    QuestionOption {
        text: "严重",
        point: 5,
    },
];

#[derive(Debug, Serialize)]
struct Rule {
    value: u8,
    comparison_operator: PlainText,
}

#[derive(Debug, Serialize)]
struct Positive {
    total: Rule,
    positive_amount: Rule,
    any_symptom_average: Rule,
}

#[derive(Debug, Serialize)]
pub struct SymptomInterpretation {
    /// 中文名称
    name: PlainText,
    /// 专业术语
    terminology: PlainText,
    /// 描述
    description: PlainText,
    /// 阳性描述
    positive: PlainTexts,
    /// 阴性描述
    negative: PlainText,
    /// 建议
    guidance: SymptomGuidance,
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "UPPERCASE"))]
struct Symptoms {
    somatization: SymptomInterpretation,
    obsessive_compulsive: SymptomInterpretation,
    interpersonal_sensitivity: SymptomInterpretation,
    depression: SymptomInterpretation,
    anxiety: SymptomInterpretation,
    hostility: SymptomInterpretation,
    phobic_anxiety: SymptomInterpretation,
    paranoid_ideation: SymptomInterpretation,
    psychoticism: SymptomInterpretation,
    others: SymptomInterpretation,
}

#[derive(Debug, Serialize)]
pub struct Interpretation {
    positive: Positive,
    symptoms: Symptoms,
}

pub const SYMPTOM_CHECKLIST_90: Scale<Interpretation, Question> = Scale {
    id: 5,
    name: "症状自评量表",
    description: "像体检一样简单，为你的心理健康做一次全面快筛",
    abbreviation: "SCL-90",
    primary_category: ScaleCategory::MentalHealth,
    related_categories: Some(&[
        ScaleCategory::Emotion,
        ScaleCategory::Somatic,
        ScaleCategory::Interpersonal,
        ScaleCategory::Behavior,
    ]),
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: None,
    warning: Some("本量表仅适用 16 岁以上的人群。"),
    formula_mode: Some(FormulaMode {
        operational_rule: OperationalRule::Multiply(1.25),
        integer: Some(Integer::Round),
    }),
    tags: &Tag {
        info: Some(&["多症状"]),
        normal: None,
        warning: Some(&["16+"]),
        error: None,
    },
    interpretation: INTERPRETATION,
    questions: QUESTIONS,
};
