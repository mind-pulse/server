//! 量表注册表：所有量表查询的唯一入口。
//!
//! 注册通过 `macro_rules!` 展开，避免手写多个 `match` 分支和数组。

use crate::error::{Error, Result};
use crate::scale::data::{
    BECK_DEPRESSION_INVENTORY, ENNEAGRAM_PERSONALITY_TEST,
    EYSENCK_PERSONALITY_QUESTIONNAIRE_REVISED_SHORT_SCALE, HAMILTON_DEPRESSION_SCALE,
    HOLLAND_OCCUPATIONAL_INTEREST, HOLLAND_OCCUPATIONAL_INTEREST_HIGH_SCHOOL_CN,
    NEO_PERSONALITY_INVENTORY_REVISED, SELF_RATING_ANXIETY_SCALE, SELF_RATING_DEPRESSION_SCALE,
    SIXTEEN_PERSONALITY_FACTORS, SYMPTOM_CHECKLIST_90, YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE,
};
use crate::scale::summary::{ScaleId, ScaleSummary, estimate_duration};

macro_rules! register_scales {
    (
        $(
            // 格式：常量名 => { disabled: 禁用状态 (可选，默认为 false) }
            $scale:ident $( => { disabled: $disabled:expr } )?
        ),* $(,)?
    ) => {
        /// 所有量表的概览（用于前端展示列表）
        pub fn summaries() -> Vec<ScaleSummary<'static>> {
            vec![
                $(
                    ScaleSummary {
                        id: $scale.id,
                        name: $scale.name,
                        // 自动计算时长
                        duration: estimate_duration($scale.questions.len() as u32),
                        total_questions: $scale.questions.len() as u32,
                        description: $scale.description,
                        primary_category: $scale.primary_category,
                        related_categories: $scale.related_categories,
                        introduction: $scale.introduction,
                        warning: $scale.warning,
                        tags: $scale.tags,
                        // 处理默认禁用状态
                        disabled: false $( || $disabled )?,
                    },
                )*
            ]
        }

        /// 所有量表的 (ID, 名称)，包含已禁用的量表
        pub fn all_names() -> Vec<(ScaleId, &'static str)> {
            vec![
                $( (ScaleId($scale.id), $scale.name), )*
            ]
        }

        /// 根据 ID 获取量表的完整 JSON 数据
        pub fn json(id: ScaleId) -> Result<serde_json::Value> {
            match id {
                $(
                    ScaleId(value) if value == $scale.id => {
                        serde_json::to_value(&$scale).map_err(Error::from)
                    }
                )*
                _ => Err(Error::ScaleNotFound),
            }
        }

        /// 根据 ID 获取量表名称
        pub fn name(id: ScaleId) -> Result<&'static str> {
            match id {
                $(
                    ScaleId(value) if value == $scale.id => Ok($scale.name),
                )*
                _ => Err(Error::ScaleNotFound),
            }
        }
    };
}

register_scales! {
    // 1. 普通注册
    HOLLAND_OCCUPATIONAL_INTEREST_HIGH_SCHOOL_CN,
    NEO_PERSONALITY_INVENTORY_REVISED,
    SIXTEEN_PERSONALITY_FACTORS,
    EYSENCK_PERSONALITY_QUESTIONNAIRE_REVISED_SHORT_SCALE,
    SYMPTOM_CHECKLIST_90,
    BECK_DEPRESSION_INVENTORY,
    SELF_RATING_DEPRESSION_SCALE,
    SELF_RATING_ANXIETY_SCALE,
    YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE,

    // 2. 带有特殊禁用逻辑的注册 (使用 => { disabled: ... })

    // 生产环境暂未完成的测试，仅在 Debug 模式下开启
    ENNEAGRAM_PERSONALITY_TEST => {
        disabled: cfg!(not(debug_assertions))
    },

    // 暂时强制下架的测试
    HOLLAND_OCCUPATIONAL_INTEREST => {
        disabled: true
    },

    // 他评量表，暂时不在前端显示
    HAMILTON_DEPRESSION_SCALE => {
        disabled: true
    },
}
