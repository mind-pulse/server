//! 此为开源版霍兰德职业兴趣高中生专用中文版本，专业名参考教育部公开的《普通高等学校本科专业目录》，因专业数量多，这里只能硬编码一部分。
//!
//! 商业版使用高级算法实现，会对所有专业进行智能推荐组合。

use serde::Serialize;

use crate::scale::{
    HTMLElement, PlainText, PlainTexts, Scale, ScaleCategory, SentenceItem, Tag, Texts,
};

use super::holland_occupational_interest::{
    CapacityCategory, CapacityCategoryInterpretation, Question,
};

mod questions;
use self::questions::QUESTIONS;

mod categories;
mod content;
mod majors;
use self::categories::CAPACITY_CATEGORY_INTERPRETATIONS;
use self::content::INSTRUCTION;
use self::content::INTRODUCTION;
use self::majors::MAJORS_MATCHES;

#[derive(Serialize)]
struct MajorsMatch {
    /// 代码标识，如 "RIA"
    code: PlainText,

    /// 代码含义说明，如 "R+I+A：实操 + 研究 + 艺术"
    meaning: PlainText,

    /// 核心专业列表
    core_majors: PlainTexts,

    /// 次优专业列表
    secondary_majors: PlainTexts,

    /// 适配逻辑描述
    logic: PlainText,
}

#[derive(Serialize)]
pub struct Interpretation {
    capacity_category_interpretations: &'static [CapacityCategoryInterpretation],
    majors_matches: &'static [MajorsMatch],
}

pub const HOLLAND_OCCUPATIONAL_INTEREST_HIGH_SCHOOL_CN: Scale<Interpretation, Question> = Scale {
    id: 11,
    name: "霍兰德职业兴趣测评（高中版）",
    description: "发现你天生热爱的事，匹配最适合你的大学专业方向",
    primary_category: ScaleCategory::CareerAndAcademics,
    related_categories: Some(&[ScaleCategory::Personality, ScaleCategory::AttitudeAndValues]),
    abbreviation: "SDS", // 与抑郁自评量表缩写相同
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: Some(&[
        "兴趣是最好的专业向导。美国心理学家霍兰德提出，每个人的兴趣特点都可以归纳为六个类型：研究型（I）、艺术型（A）、社会型（S）、企业型（E）、传统型（C）、现实型（R）。你的性格可能是这六种类型的混合，但通常会有两三个特别突出的类型，而这恰恰对应着不同类型的大学专业。",
        "这个理论的核心就是：帮你找到“喜欢的”和“适合的”专业之间的交集。过去，大家选专业可能只看分数或就业前景，而霍兰德则强调兴趣才是最关键的方向标。他把“个人特点”和“专业环境”联系起来，让我们明白：选专业不只是选一个学科，更是选择一种适合自己特质的学习方式和未来发展路径。",
        "所以，在填报志愿前，不妨做个职业兴趣测试，再结合自己的学科能力，这样能更清晰地找到既感兴趣又学得好的专业方向，让大学生活更有目标感和动力。",
    ]),
    references: Some(&[
        "Holland, J. L. (1959). A theory of vocational choice. Journal of Counseling Psychology, 6(1), 35-45.",
    ]),
    warning: Some(
        "版权提示：本测评参考霍兰德 RIASEC 职业兴趣理论编制，并非 PAR 出版的 Self-Directed Search 原版；原版测评受版权保护，请通过官方渠道获取授权。",
    ),
    formula_mode: None,
    tags: &Tag {
        info: Some(&["高考", "专业"]),
        normal: None,
        warning: None,
        error: None,
    },
    interpretation: Interpretation {
        capacity_category_interpretations: &CAPACITY_CATEGORY_INTERPRETATIONS,
        majors_matches: &MAJORS_MATCHES,
    },
    questions: QUESTIONS,
};
