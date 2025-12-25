mod category;
mod common;
mod items;

use crate::error::{MindPulseError, MindPulseResult};

pub use self::items::{
    BECK_DEPRESSION_INVENTORY, ENNEAGRAM_PERSONALITY_TEST,
    EYSENCK_PERSONALITY_QUESTIONNAIRE_REVISED_SHORT_SCALE, HAMILTON_DEPRESSION_SCALE,
    HOLLAND_OCCUPATIONAL_INTEREST, HOLLAND_OCCUPATIONAL_INTEREST_HIGH_SCHOOL_CN,
    NEO_PERSONALITY_INVENTORY_REVISED, SELF_RATING_ANXIETY_SCALE, SELF_RATING_DEPRESSION_SCALE,
    SIXTEEN_PERSONALITY_FACTORS, SYMPTOM_CHECKLIST_90, YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE,
};

const SECONDS_PER_QUESTION_MIN: u32 = 10;
const SECONDS_PER_QUESTION_MAX: u32 = 15;

/// 计算预计测试时长（分钟）
///
/// 每个问题耗时 10-15秒 秒为标准计算
const fn estimated_duration(total_questions: u32) -> [u32; 2] {
    let min = total_questions * SECONDS_PER_QUESTION_MIN / 60;
    let max = total_questions * SECONDS_PER_QUESTION_MAX / 60;
    // 确保至少显示 1 分钟，避免出现 [0, 0]
    if max < 1 {
        [1, 2]
    } else {
        [min, max]
    }
}

// 6. 辅助函数：仅获取量表名称
pub fn get_scale_name_by_id(id: u16) -> MindPulseResult<&'static str> {
    get_scale_info_by_id(id).map(|(_, name, _)| name)
}

/// 该宏用于注册量表，自动生成列表、查找函数和 JSON 序列化逻辑。
/// 避免手动维护多个 match 分支和数组。
macro_rules! register_scales {
    (
        $(
            // 格式：常量名 => { 禁用状态 (可选，默认为 false) }
            $scale:ident $( => { disabled: $disabled:expr } )?
        ),* $(,)?
    ) => {
        use crate::scale::common::ScaleListItem;

        /// 所有量表的列表概览（用于前端展示列表）
        pub const LIST: &[ScaleListItem] = &[
            $(
                ScaleListItem {
                    id: $scale.id,
                    name: $scale.name,
                    // 自动计算时长
                    duration: estimated_duration($scale.questions.len() as u32),
                    total_questions: $scale.questions.len() as u32,
                    description: $scale.description,
                    primary_category: $scale.primary_category,
                    related_categories: $scale.related_categories,
                    introduction: $scale.introduction,
                    warning: $scale.warning,
                    tags: &$scale.tags,
                    // 处理默认禁用状态
                    // 解释：如果 $disabled 存在，展开为 "false || $disabled"
                    // 如果不存在，展开为 "false"
                    disabled: false $( || $disabled )?,
                }
            ),*
        ];

        /// 根据 ID 获取量表的完整 JSON 数据
        pub fn get_scale_json_by_id(id: u16) -> MindPulseResult<serde_json::Value> {
            match id {
                $(
                    val if val == $scale.id => {
                        serde_json::to_value(&$scale)
                            .map_err(|e| MindPulseError::Response(format!("序列化失败: {}", e)))
                    }
                )*
                _ => Err(MindPulseError::Response("无效的量表 ID".to_owned())),
            }
        }

        /// 根据 ID 获取量表基本信息 (ID, Name, QuestionCount)
        pub fn get_scale_info_by_id(id: u16) -> MindPulseResult<(u16, &'static str, usize)> {
            match id {
                $(
                    val if val == $scale.id => Ok(($scale.id, $scale.name, $scale.questions.len())),
                )*
                _ => Err(MindPulseError::Response("无效的量表 ID".to_owned())),
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
