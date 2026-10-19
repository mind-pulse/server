use serde::Serialize;

use crate::scale::{
    Characteristic, HTMLElement, PlainText, QuestionOption, Scale, ScaleCategory, SentenceItem,
    Tag, Texts,
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
enum Factor {
    /// 乐群性
    A,
    /// 聪慧性
    B,
    /// 稳定性
    C,
    /// 恃强性
    E,
    /// 兴奋性
    F,
    /// 有恒性
    G,
    /// 敢为性
    H,
    /// 敏感性
    I,
    /// 怀疑性
    L,
    /// 幻想性
    M,
    /// 世故性
    N,
    /// 忧虑性
    O,
    /// 实验性
    Q1,
    /// 独立性
    Q2,
    /// 自律性
    Q3,
    /// 紧张性
    Q4,
}

#[derive(Serialize)]
pub struct Question {
    title: PlainText,
    options: &'static [QuestionOption],
    factor: Option<Factor>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Range {
    Array([u8; 2]),
    Value(u8),
}

type Ranges = &'static [Range; 10];

#[derive(Serialize)]
struct NormItem {
    ranges: Ranges,
    mean: f32,
    standard_deviation: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct Norm {
    a: NormItem,
    b: NormItem,
    c: NormItem,
    e: NormItem,
    f: NormItem,
    g: NormItem,
    h: NormItem,
    i: NormItem,
    l: NormItem,
    m: NormItem,
    n: NormItem,
    o: NormItem,
    q1: NormItem,
    q2: NormItem,
    q3: NormItem,
    q4: NormItem,
}

#[derive(Serialize)]
struct FirstPersonalityFactor {
    factor: Factor,
    name: PlainText,
    characteristic: Characteristic,
    occupations: PlainText,
}

#[derive(Serialize)]
struct SecondPersonalityFactor {
    key: PlainText,
    name: PlainText,
    expression: PlainText,
    characteristic: Characteristic,
}

#[derive(Serialize)]
pub struct Interpretation {
    norm: Norm,
    normal_range: [u8; 2],
    first_personality_factor: &'static [FirstPersonalityFactor; 16],
    second_personality_factor: &'static [SecondPersonalityFactor; 4],
}

pub const SIXTEEN_PERSONALITY_FACTORS: Scale<Interpretation, Question> = Scale {
    id: 2,
    name: "卡特尔16种人格因素问卷",
    description: "16个维度，一眼看懂你独一无二的人格特征",
    abbreviation: "16PF",
    primary_category: ScaleCategory::Personality,
    related_categories: Some(&[
        ScaleCategory::CognitionAndAbility,
        ScaleCategory::Interpersonal,
        ScaleCategory::Wellbeing,
    ]),
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: Some(&[
        "本测试采用心理学家卡特尔的经典人格理论框架。要理解这个测试，了解三个关键概念会很有帮助：",
        "1. 特质是基石：人格并非不可捉摸，它由许多稳定的“特质”组成。有些特质更多受遗传影响，有些则更受环境影响。",
        "2. 从表面到根源：我们日常表现出的行为（表面特质），往往源于内心更深层的性格根源（根源特质）。本问卷旨在测量这些更稳定的根源特质",
        "3. 十六维地图：通过科学分析，卡特尔将这些根源特质归纳为16个核心因素，涵盖了你为人处世的风格（如乐群性、世故性）、情绪模式（如稳定性、紧张性）、以及动力与自律（如有恒性、自律性）等主要方面。",
        "接下来，你将通过回答一系列问题，在这张16维人格地图上找到自己的坐标。",
    ]),
    references: Some(&[
        "Cattell, R. B., Eber, H. W., & Tatsuoka, M. M. (1970). Handbook for the Sixteen Personality Factor Questionnaire (16PF). Institute for Personality and Ability Testing.",
    ]),
    formula_mode: None,
    warning: Some("本量表仅适用 16 岁以上的人群。"),
    tags: &Tag {
        info: Some(&["人格"]),
        normal: None,
        warning: Some(&["16+"]),
        error: None,
    },
    interpretation: INTERPRETATION,
    questions: QUESTIONS,
};
