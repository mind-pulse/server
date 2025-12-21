use serde::Serialize;

use crate::scale::{ComfortingWord, CriticalWarning, ScaleCategory};

use super::{
    HTMLElement, PlainText, PlainTexts, Question, QuestionOption, Scale, SentenceItem, Status, Tag,
    Texts,
};

#[derive(Debug, Serialize)]
struct ScoreStandard {
    total: [u8; 2],
    any: [u8; 2],
}

#[derive(Debug, Serialize)]
pub struct InterpretationItem {
    range: ScoreStandard,

    /// 标签，如"正常"、"轻度"、"中度"、"重度"
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

const INTRODUCTION: Texts = &[
    &[SentenceItem::Plain("耶鲁布朗强迫量表是美国 GOODMAN 等人根据 DSM-III-R 诊断标准而制定的专门测定强迫症状严重程度的量表，是临床上使用的评定强迫症的主要量表之一。")]
];

const INSTRUCTION: Texts = &[&[
    SentenceItem::Plain("本量表包含 "),
    SentenceItem::HTMLElement(HTMLElement::Strong("10")),
    SentenceItem::Plain(
        " 个项目，为保证调查结果的准确性，务必请您仔细阅读测试内容，依照你主要的强迫症状作答。",
    ),
]];

const INTERPRETATION: &[InterpretationItem] = &[
    InterpretationItem {
        range: ScoreStandard {
            total: [0, 5],
            any: [0, 5],
        },
        label: "正常",
        comforting_word: ComfortingWord {
            text: "您的得分在正常范围内，这是一个积极的信号。",
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
        range: ScoreStandard {
            total: [6, 15],
            any: [6, 9],
        },
        label: "轻度",
        comforting_word: ComfortingWord {
            text: "您的得分显示有轻度强迫倾向，这在很多人身上都会出现。",
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
        range: ScoreStandard {
            total: [16, 25],
            any: [10, 14],
        },
        label: "中度",
        comforting_word: ComfortingWord {
            text: "您的得分显示有中度强迫倾向，这可能需要更多关注和支持。",
            caution: Some("中度强迫并非您的错，它是一种可治疗的医疗状况。寻求帮助是勇敢的表现。"),
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
        range: ScoreStandard { total: [26, 40], any: [15, 20] },
        label: "重度",
        comforting_word: ComfortingWord {
            text: "您的得分显示有重度强迫倾向，这已经严重影响到生活。",
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
];

pub const YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE: Scale<&[InterpretationItem], Question> = Scale {
    name: "耶鲁布朗强迫症量表",
    abbreviation: "Y-BOCS",
    primary_category: ScaleCategory::Emotion,
    related_categories: Some(&[
        ScaleCategory::MentalHealth,
        ScaleCategory::Behavior,
    ]),
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: None,
    warning: None,
    formula_mode: None,
    tags: Tag{ info: Some(&["强迫"]), normal: None, warning: None, error: None },
    interpretation: INTERPRETATION,
    questions: &[
        Question {
            title: "您每天花多少时间在强迫思维上？每天强迫思维出现的频率有多高？",
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
            title: "您的强迫思维对社交、学业成就或工作能力有多大妨碍？（假如目前没有工作，则强迫思维对每天日常活动的妨碍有多大？回答此题时，请想是否有任何事情因为强迫思维而不去做或较少做）",
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
            title: "您的强迫思维给您带来多大的苦恼或困扰？",
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
            title: "您有多少努力对抗强迫思维？你是否尝试转移注意力或不去想它呢？（重点不在于是否成功转移，而在于你有多努力对抗或尝试频率有多高）",
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
            title: "您控制强迫思维的能力有多少？您停止或转移强迫思维的效果如何？（不包括通过强迫行为来停止强迫思维）",
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
            title: "您每天花多少时间在强迫行为上？每天做出强迫行为的频率有多高？",
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
                    text: "极重（多于八小时），或几乎无时无刻都有（次数多到无法计算，且一小时内很少没有多种强迫思维）",
                    point: 4,
                },
            ],
        },
        Question {
            title: "您的强迫行为对社交、学业成就或工作能力有多大妨碍？（假如目前没有工作，则强迫行为对每天日常活动的妨碍有多大？）",
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
            title: "假如被制止从事强迫行为时，您有什么感觉？您会多焦虑？",
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
            title: "您有多努力去对抗强迫行为？或尝试停止强迫行为的频率？（仅评估你有多努力对抗强迫行为或尝试频率有多高，而不在于评估您停止强迫行为的效果有多好）",
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
            title: "您控制强迫行为的能力如何？您停止强迫（仪式）行为的效果如何？（假如你很少去对抗，那就回想那些少数对抗的情境，以便回答此题。）",
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
    ]
};
