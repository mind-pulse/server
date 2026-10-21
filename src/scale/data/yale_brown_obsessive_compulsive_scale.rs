use serde::Serialize;

use crate::scale::{
    ComfortingWord, CriticalWarning, HTMLElement, PlainText, PlainTexts, Question, QuestionOption,
    Scale, ScaleCategory, SentenceItem, Status, Tag, Texts,
};

#[derive(Debug, Serialize)]
struct ScoreStandard {
    total: [u8; 2],
}

#[derive(Debug, Serialize)]
pub struct InterpretationItem {
    range: ScoreStandard,

    /// 标签，如"正常"、"轻度"、"中度"、"重度"、"极重度"
    label: PlainText,

    /// 安慰语 - 包含核心结论和重要提醒
    comforting_word: ComfortingWord,

    /// 症状描述 - 用户可能正在经历的具体表现
    #[serde(skip_serializing_if = "Option::is_none")]
    symptoms: Option<PlainTexts>,

    /// 具体建议 - 非专业的行动指南
    #[serde(skip_serializing_if = "Option::is_none")]
    advice: Option<PlainTexts>,

    /// 极度危险提醒 - 仅用于重度情况
    #[serde(skip_serializing_if = "Option::is_none")]
    critical_warning: Option<CriticalWarning>,

    /// 状态 - 用于前端样式
    status: Status,
}

const INTRODUCTION: Texts = &[&[SentenceItem::Plain(
    "耶鲁布朗强迫量表是美国 Goodman 等人根据 DSM-III-R 诊断标准而制定的专门测定强迫症状严重程度的量表，是临床上使用的评定强迫症的主要量表之一。",
)]];

const INSTRUCTION: Texts = &[&[
    SentenceItem::Plain("本量表包含 "),
    SentenceItem::HTMLElement(HTMLElement::Strong("10")),
    SentenceItem::Plain(
        " 个项目，为保证调查结果的准确性，务必请你仔细阅读测试内容，依照你主要的强迫症状作答。",
    ),
]];

const INTERPRETATION: &[InterpretationItem] = &[
    InterpretationItem {
        range: ScoreStandard { total: [0, 7] },
        label: "正常",
        comforting_word: ComfortingWord {
            text: "你的得分在正常范围内，这是一个积极的信号。",
            caution: Some("心理状态会随时间和压力变化，继续保持对自己心理健康的关注是有益的。"),
        },
        symptoms: None,
        advice: Some(&[
            "继续保持良好的生活习惯和压力管理。",
            "如果未来感到压力大时，可以尝试散步、深呼吸等简单的放松方法。",
        ]),
        critical_warning: None,
        status: Status::Normal,
    },
    InterpretationItem {
        range: ScoreStandard { total: [8, 15] },
        label: "轻度",
        comforting_word: ComfortingWord {
            text: "你的得分提示有一些轻度的强迫倾向，这并不少见。",
            caution: Some("这些感觉在压力大时更常见，并不代表你有严重问题，但值得关注。"),
        },
        symptoms: Some(&[
            "偶尔会反复思考某些想法",
            "可能有一些小的习惯或检查行为",
            "大多数时候能够正常生活和工作",
        ]),
        advice: Some(&[
            "尝试记录什么时候这些感觉会出现，了解自己的压力源。",
            "可以学习一些放松技巧，比如冥想或渐进式肌肉放松。",
            "如果这些感觉持续困扰你，可以考虑和专业心理工作者聊一聊。",
        ]),
        critical_warning: None,
        status: Status::Mild,
    },
    InterpretationItem {
        range: ScoreStandard { total: [16, 23] },
        label: "中度",
        comforting_word: ComfortingWord {
            text: "你的得分显示有中度强迫倾向，这可能需要更多关注和支持。",
            caution: Some(
                "这类困扰并不代表你已经确诊，它很常见、也可以改善。寻求帮助是勇敢的表现。",
            ),
        },
        symptoms: Some(&[
            "重复的想法或行为已经明显影响日常效率",
            "可能需要花较多时间处理这些感觉",
            "有时会感到疲惫或烦躁",
        ]),
        advice: Some(&[
            "建议联系心理咨询师或心理医生进行专业评估。",
            "可以请家人或朋友支持你寻求帮助。",
            "专业的帮助可以教你有效的方法来应对这些困扰。",
        ]),
        critical_warning: None,
        status: Status::Moderate,
    },
    InterpretationItem {
        range: ScoreStandard { total: [24, 31] },
        label: "重度",
        comforting_word: ComfortingWord {
            text: "你的得分处在偏高的区间，说明这些想法或行为可能已占用不少时间和精力，值得尽快找专业人士评估。",
            caution: Some("重度强迫是可以治疗的医疗状况。寻求专业帮助是改善的第一步。"),
        },
        symptoms: Some(&[
            "这些感觉几乎每天都在影响你",
            "需要大量时间来处理这些想法或行为",
            "已经影响到了基本的生活安排",
        ]),
        advice: Some(&[
            "建议尽快联系心理医生或精神科医生进行评估。",
            "专业的治疗和支持可以帮助你改善这种情况。",
            "你可以从预约一次专业的评估开始。",
        ]),
        critical_warning: Some(CriticalWarning {
            title: "重要提醒",
            content: "如果这些困扰让你感到非常痛苦，或者有伤害自己的想法，请立即联系专业医疗人员或拨打心理援助热线。",
        }),
        status: Status::Major,
    },
    InterpretationItem {
        range: ScoreStandard { total: [32, 40] },
        label: "极重度",
        comforting_word: ComfortingWord {
            text: "你的得分处在最高区间，说明这些想法或行为可能已经占据了大部分时间和精力。",
            caution: Some(
                "这不代表你已经确诊，但它提示你值得尽快获得专业支持。寻求帮助是勇敢的表现。",
            ),
        },
        symptoms: Some(&[
            "这些感觉几乎每天都在影响你，持续时间很长",
            "需要花费大量时间处理这些想法或行为",
            "已经明显影响到日常生活、工作或人际关系",
        ]),
        advice: Some(&[
            "建议尽快联系精神科医生或心理治疗师进行评估。",
            "如果愿意，可以请信任的家人或朋友陪你一起就医。",
            "在等待就诊期间，尽量保持规律的作息和基本的日常活动。",
        ]),
        critical_warning: Some(CriticalWarning {
            title: "重要提醒",
            content: "如果这些困扰让你感到非常痛苦，或者有伤害自己的想法，请立即联系专业医疗人员或拨打全国统一心理援助热线 12356。",
        }),
        status: Status::Extreme,
    },
];

pub const YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE: Scale<&[InterpretationItem], Question> = Scale {
    id: 9,
    name: "耶鲁-布朗强迫量表",
    description: "国际通用的强迫症状严重程度评估量表（自评改编版），10题评估困扰程度",
    abbreviation: "Y-BOCS",
    primary_category: ScaleCategory::Emotion,
    related_categories: Some(&[ScaleCategory::MentalHealth, ScaleCategory::Behavior]),
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: Some(&[
        "Goodman, W. K., Price, L. H., Rasmussen, S. A., Mazure, C., Fleischmann, R. L., Hill, C. L., Heninger, G. R., & Charney, D. S. (1989). The Yale-Brown Obsessive Compulsive Scale: I. Development, use, and reliability. Archives of General Psychiatry, 46(11), 1006-1011.",
    ]),
    warning: Some(
        "本量表原为临床医生他评工具，此处为自评改编版，结果仅供自我了解与就医参考，不能作为诊断依据。 使用提示：本量表原始版本可公开用于研究，正式使用请遵循原始发布方的使用条款；此处仅供个人学习参考，不得用于商业或临床诊断。",
    ),
    tags: &Tag {
        info: Some(&["强迫"]),
        normal: None,
        warning: None,
        error: None,
    },
    interpretation: INTERPRETATION,
    questions: &[
        Question {
            title: "你每天花多少时间在强迫思维上？每天强迫思维出现的频率有多高？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "完全无强迫思维",
                    point: 0,
                },
                QuestionOption {
                    text: "轻微（少于一小时），或偶尔有（一天不超过8次）",
                    point: 1,
                },
                QuestionOption {
                    text: "中度（一至三小时），或常常有（一天超过8次，但一天大部分时间没有强迫思维）",
                    point: 2,
                },
                QuestionOption {
                    text: "重度（多于三小时但不超过八小时），或频率非常高（一天超过8次，且一天大部分时间有强迫思维）",
                    point: 3,
                },
                QuestionOption {
                    text: "极重（多于八小时），或几乎无时无刻都有（次数多到无法计算，且一小时内很少没有多种强迫思维）",
                    point: 4,
                },
            ],
        },
        Question {
            title: "你的强迫思维对社交、学业成就或工作能力有多大妨碍？（假如目前没有工作，则强迫思维对每天日常活动的妨碍有多大？回答此题时，请想是否有任何事情因为强迫思维而不去做或较少做）",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "不受妨碍",
                    point: 0,
                },
                QuestionOption {
                    text: "轻微。稍微妨碍社交或工作活动，但整体表现并无大碍。",
                    point: 1,
                },
                QuestionOption {
                    text: "中度。确实妨碍社交或工作活动，但仍可应付。",
                    point: 2,
                },
                QuestionOption {
                    text: "重度。导致社交或工作表现的障碍。",
                    point: 3,
                },
                QuestionOption {
                    text: "极度。无能力应付社交或工作。",
                    point: 4,
                },
            ],
        },
        Question {
            title: "你的强迫思维给你带来多大的苦恼或困扰？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "没有",
                    point: 0,
                },
                QuestionOption {
                    text: "轻微。不会太烦人。",
                    point: 1,
                },
                QuestionOption {
                    text: "中度。觉得很烦，但尚可应付。",
                    point: 2,
                },
                QuestionOption {
                    text: "重度。非常烦人。",
                    point: 3,
                },
                QuestionOption {
                    text: "极重。几乎一直持续且令人丧志地苦恼。",
                    point: 4,
                },
            ],
        },
        Question {
            title: "你有多少努力对抗强迫思维？你是否尝试转移注意力或不去想它呢？（重点不在于是否成功转移，而在于你有多努力对抗或尝试频率有多高）",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "一直不断地努力与之对抗（或症状很轻微，不需要积极地对抗）",
                    point: 0,
                },
                QuestionOption {
                    text: "大部分时间都试图与之对抗（超过一半的时间我都试图与之对抗）",
                    point: 1,
                },
                QuestionOption {
                    text: "用些许努力去对抗。",
                    point: 2,
                },
                QuestionOption {
                    text: "屈服于所有的强迫思维，未试图控制，但仍有些不甘心。",
                    point: 3,
                },
                QuestionOption {
                    text: "完全愿意屈服于强迫思维。",
                    point: 4,
                },
            ],
        },
        Question {
            title: "你控制强迫思维的能力有多少？你停止或转移强迫思维的效果如何？（不包括通过强迫行为来停止强迫思维）",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "完全控制。我可以完全控制。",
                    point: 0,
                },
                QuestionOption {
                    text: "大多能控制。只要花些力气与注意力，即能停止或转移强迫思维。",
                    point: 1,
                },
                QuestionOption {
                    text: "中等程度控制。“有时”能停止或转移强迫思维。",
                    point: 2,
                },
                QuestionOption {
                    text: "控制力弱。很少能成功地停止或消除强迫思维，只能转移。",
                    point: 3,
                },
                QuestionOption {
                    text: "无法控制。完全不能自主，连转移一下强迫思维的能力都没有。",
                    point: 4,
                },
            ],
        },
        Question {
            title: "你每天花多少时间在强迫行为上？每天做出强迫行为的频率有多高？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "完全无强迫行为",
                    point: 0,
                },
                QuestionOption {
                    text: "轻微（少于一小时），或偶尔有（一天不超过8次）",
                    point: 1,
                },
                QuestionOption {
                    text: "中度（一至三小时），或常常有（一天超过8次，但一天大部分时间没有强迫行为）",
                    point: 2,
                },
                QuestionOption {
                    text: "重度（多于三小时但不超过八小时），或频率非常高（一天超过8次，且一天大部分时间有强迫行为）",
                    point: 3,
                },
                QuestionOption {
                    text: "极重（多于八小时），或几乎无时无刻都有（次数多到无法计算，且一小时内很少没有多种强迫行为）",
                    point: 4,
                },
            ],
        },
        Question {
            title: "你的强迫行为对社交、学业成就或工作能力有多大妨碍？（假如目前没有工作，则强迫行为对每天日常活动的妨碍有多大？）",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "不受妨碍",
                    point: 0,
                },
                QuestionOption {
                    text: "轻微。稍微妨碍社交或工作活动，但整体表现并无大碍。",
                    point: 1,
                },
                QuestionOption {
                    text: "中度。确实妨碍社交或工作活动，但仍可应付。",
                    point: 2,
                },
                QuestionOption {
                    text: "重度。导致社交或工作表现的障碍。",
                    point: 3,
                },
                QuestionOption {
                    text: "极度。无能力应付社交或工作。",
                    point: 4,
                },
            ],
        },
        Question {
            title: "假如被制止从事强迫行为时，你有什么感觉？你会多焦虑？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "没有焦虑",
                    point: 0,
                },
                QuestionOption {
                    text: "轻微。假如强迫行为被阻止，只是稍微焦虑。",
                    point: 1,
                },
                QuestionOption {
                    text: "中度。假如强迫行为被阻止，会有中等程度的焦虑，但是仍可以应付。",
                    point: 2,
                },
                QuestionOption {
                    text: "严重。假如强迫行为被阻止，会明显且困扰地增加焦虑。",
                    point: 3,
                },
                QuestionOption {
                    text: "极度。假如有任何需要改变强迫行为的处置时，会导致极度地焦虑。",
                    point: 4,
                },
            ],
        },
        Question {
            title: "你有多努力去对抗强迫行为？或尝试停止强迫行为的频率？（仅评估你有多努力对抗强迫行为或尝试频率有多高，而不在于评估你停止强迫行为的效果有多好）",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "一直不断地努力与之对抗（或症状很轻微，不需要积极地对抗）",
                    point: 0,
                },
                QuestionOption {
                    text: "大部分时间都试图与之对抗（超过一半的时间我都试图与之对抗）",
                    point: 1,
                },
                QuestionOption {
                    text: "用些许努力去对抗。",
                    point: 2,
                },
                QuestionOption {
                    text: "屈服于所有的强迫行为，未试图控制，但仍有些不甘心。",
                    point: 3,
                },
                QuestionOption {
                    text: "完全愿意屈服于强迫行为。",
                    point: 4,
                },
            ],
        },
        Question {
            title: "你控制强迫行为的能力如何？你停止强迫（仪式）行为的效果如何？（假如你很少去对抗，那就回想那些少数对抗的情境，以便回答此题。）",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "完全控制。我可以完全控制。",
                    point: 0,
                },
                QuestionOption {
                    text: "大多能控制。只要花些力气与注意力，即能停止强迫行为。",
                    point: 1,
                },
                QuestionOption {
                    text: "中等程度控制。“有时”控制强迫行为，有些困难。",
                    point: 2,
                },
                QuestionOption {
                    text: "控制力弱。只能忍耐耽搁一下时间，但最终还是必须完成强迫行为。",
                    point: 3,
                },
                QuestionOption {
                    text: "完全无法控制。连耽搁一下的能力都没有。",
                    point: 4,
                },
            ],
        },
    ],
};

#[cfg(test)]
mod tests {
    use super::INTERPRETATION;
    use crate::scale::Status;

    #[test]
    fn total_ranges_partition_0_to_40() {
        let mut expected_start = 0u8;

        for item in INTERPRETATION {
            assert_eq!(
                item.range.total[0], expected_start,
                "总分区间必须连续、不重叠"
            );
            expected_start = item.range.total[1] + 1;
        }

        assert_eq!(expected_start, 41, "最高一档必须覆盖满分 40");
    }

    #[test]
    fn has_five_bands_and_extreme_top() {
        assert_eq!(INTERPRETATION.len(), 5);

        let extreme = INTERPRETATION.last().unwrap();
        assert_eq!(extreme.label, "极重度");
        assert_eq!(extreme.range.total, [32, 40]);
        assert!(matches!(extreme.status, Status::Extreme));
    }
}
