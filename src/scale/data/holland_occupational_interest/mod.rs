use serde::Serialize;

use crate::scale::{
    HTMLElement, PlainText, QuestionOption, Scale, ScaleCategory, SentenceItem, Tag, Texts,
};

mod questions;
use self::questions::QUESTIONS;

mod careers;
mod categories;
mod content;
use self::careers::CAREER_INFORMATION;
use self::categories::CAPACITY_CATEGORY_INTERPRETATIONS;
use self::content::INSTRUCTION;
use self::content::INTRODUCTION;

/// 问题类型
#[derive(Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
pub enum QuestionType {
    /// 感兴趣的事
    InterestedEvent,
    /// 擅长的事
    GoodAt,
    /// 喜欢的职业
    LovedJob,
    /// 能力类型
    CapacityCategory,
}

/// 人格类型
#[derive(Serialize)]
pub enum CapacityCategory {
    R,
    A,
    I,
    S,
    E,
    C,
}

#[derive(Serialize)]
pub struct Question {
    pub title: PlainText,
    pub question_type: QuestionType,
    pub capacity_category: CapacityCategory,
    pub options: &'static [QuestionOption],
    pub is_multiple: bool,
}

#[derive(Serialize)]
pub struct CapacityCategoryInterpretation {
    pub capacity_category: CapacityCategory,
    pub name: PlainText,
    /// 人格特征
    pub personality_trait: PlainText,
    /// 职业特征
    pub occupational_stigma: PlainText,
}

#[derive(Serialize)]
pub struct CareerInformation {
    code: PlainText,
    information: PlainText,
}

#[derive(Serialize)]
pub struct Interpretation {
    capacity_category_interpretations: &'static [CapacityCategoryInterpretation],
    career_information: &'static [CareerInformation],
}

pub const HOLLAND_OCCUPATIONAL_INTEREST: Scale<Interpretation, Question> = Scale {
    id: 0,
    name: "霍兰德职业兴趣测评",
    description: "发现你天生热爱的事，匹配最适合你的职业方向",
    primary_category: ScaleCategory::CareerAndAcademics,
    related_categories: Some(&[ScaleCategory::Personality, ScaleCategory::AttitudeAndValues]),
    abbreviation: "SDS", // 与抑郁自评量表缩写相同
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: Some(&[
        "霍兰德认为，个人职业兴趣特性与职业之间应有一种内在的对应关系。根据兴趣的不同，人格可分为研究型（I）、艺术型（A）、社会型（S）、企业型（E）、传统型（C）、现实型（R）六个维度，每个人的性格都是这六个维度的不同程度组合。",
        "霍兰德的职业兴趣理论主要从兴趣的角度出发来探索职业指导的问题。他明确提出了职业兴趣的人格观，使人们对职业兴趣的认识有了质的变化。霍兰德的职业兴趣理论反映了他长期专注于职业指导的实践经历，他把对职业环境的研究与对职业兴趣个体差异的研究有机地结合起来，而在霍兰德的职业兴趣类型理论提出之前，二者的研究是相对独立进行的。",
        "兴趣测试和能力测试的结合在职业指导和职业咨询的实际操作中起到了促进作用。",
    ]),
    references: Some(&[
        "Holland, J. L. (1959). A theory of vocational choice. Journal of Counseling Psychology, 6(1), 35-45.",
    ]),
    warning: None,
    formula_mode: None,
    tags: &Tag {
        info: Some(&["职业", "高考", "就业", "专业"]),
        normal: None,
        warning: None,
        error: None,
    },
    interpretation: Interpretation {
        capacity_category_interpretations: &CAPACITY_CATEGORY_INTERPRETATIONS,
        career_information: &CAREER_INFORMATION,
    },
    questions: QUESTIONS,
};
