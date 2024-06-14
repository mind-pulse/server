use serde::Serialize;

use super::{Question, QuestionOption, Scale, Status, Tag, Text, Texts};

#[derive(Debug, Serialize)]
struct ScoreStandard {
    total: [u8; 2],
    any: [u8; 2],
}

#[derive(Debug, Serialize)]
pub struct InterpretationItem {
    range: ScoreStandard,
    label: Text,
    description: Option<Texts>,
    advice: Option<Texts>,
    status: Status,
}

pub const YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE: Scale<&[InterpretationItem], Question> = Scale {
    name: "耶鲁布朗强迫症量表",
    abbreviation: "Y-BOCS",
    introduction: &["耶鲁布朗强迫量表是美国GOODMAN等人根据DSM-III-R诊断标准而制定的专门测定强迫症状严重程度的量表；是临床上使用的评定强迫症的主要量表之一。"],
    instruction: &["本量表包含 10 个项目，为保证调查结果的准确性，务必请您仔细阅读测试内容，依照你主要的强迫症状作答。"],
    idea: None,
    references: None,
    warning: None,
    formula_mode: None,
    tags: Tag{ info: Some(&["强迫"]), normal: Some(&["自评"]), warning: None, error: None },
    interpretation: &[
        InterpretationItem {
            range: ScoreStandard { 
                total: [0, 5], 
                any: [0, 5],
            },
            label: "无强迫思维和行为",
            description: None,
            advice: Some(&["请继续保持积极阳光的生活态度，面对人生，相信你会遇到很多惊喜，同时也能给他人带来快乐！"]),
            status: Status::Normal,
        },
        InterpretationItem {
            range: ScoreStandard { 
                total: [6, 15], 
                any: [6, 9],
            },
            label: "轻度",
            description: Some(&[
                "您可能有轻度的强迫症，症状已经对您的生活、学习或职业开始造成一定的影响。",
                "您的症状会随着环境和情绪的变化不断的波动，如果不能尽早的解决，很容易会朝着严重的程度发展、泛化。"
            ]),
            advice: Some(&["此时是治疗效果最理想的时期，建议尽早治疗。"]),
            status: Status::Mild,
        },
        InterpretationItem {
            range: ScoreStandard { 
                total: [16, 25], 
                any: [10, 14],
            },
            label: "中度",
            description: Some(&[
                "您可能有中度的强迫症，症状的频率或严重程度已经对生活、学习或职业造成明显的障碍。",
                "完成您的工作或学习任务对您来说比较困难，甚至在没有出现有效的改善前，您可能产生抑郁症状，甚至出现自杀念头。"
            ]),
            advice: Some(&["必须接受心理治疗或者药物治疗。"]),
            status: Status::Moderate,
        },
        InterpretationItem {
            range: ScoreStandard { 
                total: [26, 40], 
                any: [15, 20],
            },
            label: "重度",
            description: Some(&[
                "您的强迫症状已经非常严重，完全无法完成工作或学习任务，无法屡行您的社会角色功能，甚至连衣食住行等生活功能都无法进行。",
                "您可能已经无法出门，将自己禁锢家中，无时无刻都有强迫思考，无时无刻都在执行强迫行为。"
            ]),
            advice: Some(&["此时极易发展出抑郁症状，通常需要强制治疗。"]),
            status: Status::Major,
        },
    ],
    questions: &[
        Question {
            title: "您每天花多少时间在强迫思维上？每天强迫思维出现的频率有多高？",
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
                    text: "中度。假如强迫行为被阻止，会有中等程度的焦虑，但是仍可以应付。 　　3=严重。假如强迫行为被阻止，会明显且困扰地增加焦虑。",
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
