use super::{
    FormulaMode, HTMLElement, Integer, InterpretationItem, OperationalRule, Question,
    QuestionOption, Scale, SentenceItem, Status, Tag, Texts,
};

const INTRODUCTION: Texts = &[
    &[
        SentenceItem::Plain("本量表是根据Zung于1971年编制的“焦虑自评量表（Self—Rating Anxiety Scale，SAS）改编而成，可为临床心理咨询、诊断、治疗以及病理心理机制的研究提供科学依据。本测验应用范围颇广，适用于各种职业、文化阶层及年龄段的正常人或各类精神病人。"),
    ],
];

const INSTRUCTION: Texts = &[&[
    SentenceItem::Plain("本量表包含 "),
    SentenceItem::HTMLElement(HTMLElement::Strong("20")),
    SentenceItem::Plain(
        " 个项目，分为 4 级评分，为保证调查结果的准确性，务必请您仔细阅读以下内容，然后根据您",
    ),
    SentenceItem::HTMLElement(HTMLElement::Strong("最近 1 周")),
    SentenceItem::Plain(
        "的实际情况选择适当的选项，每一条文字后面有四个选项，请根据选项内容进行恰当的选择。",
    ),
]];

pub const SELF_RATING_ANXIETY_SCALE: Scale<&[InterpretationItem<i8>], Question> = Scale {
    name: "焦虑自评量表",
    abbreviation: "SAS",
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: None,
    warning: None,
    formula_mode: Some(FormulaMode{
        operational_rule: OperationalRule::Multiply(1.25),
        integer: Some(Integer::Round),
    }),
    tags: Tag{ info: Some(&["焦虑"]), normal: Some(&["自评"]), warning: None, error: None },
    interpretation: &[
        InterpretationItem{
            range: [0, 50],
            description: "正常",
            advice: None,
            symptom: None,
            status: Status::Normal,
        },
        InterpretationItem{
            range: [50, 60],
            description: "轻度焦虑",
            advice: Some(&[
                "以下几点也许对您自我调节有帮助：",
                "自我释放：找个空旷的地方，比如山间，找个人少的地方放声大喊几声，将自己不满的情绪全部释放出来。",
                "保证充足睡眠：睡眠是一定要保证充足的，如果身体得不到足够的休息，我们的身体就会显得很疲惫，精神不佳，倦怠懒言，紧接着情绪就会变得敏感，免疫力也会随之下降。",
                "自我注意力转移：找一件能吸引自己，带给自己乐趣的事情做，或者做一些运动，这对于防止胡思乱想有比较明显的帮助，同时也可增强你的适应能力。",
                "请注意，这些方法仅供参考，如果您的焦虑症状严重或持续，请及时咨询专业医生。",
            ]),
            symptom: Some(&[
                "您可能有一些身体不适，如心慌、胸闷、厌食、心跳加快、疲惫感、失眠、多梦等。",
                "您可能情绪消沉、消极、焦躁，具体行为表现如坐立不安、走来走去、注意力分散等。",
                "您可能有过分的担心和恐惧，如对于生活失去信心和兴趣，极度的厌世，不想与任何人交流，只想一个人安静地待着。",
            ]),
            status: Status::Mild,
        },
        InterpretationItem{
            range: [60, 70],
            description: "中度焦虑",
            advice: Some(&[
                "首先建议您寻求心理医生的帮助，如果暂时无法就医，可以通过以下办法缓解您的症状：",
                "深呼吸: 慢慢地吸气，然后慢慢地呼气，重复几次。",
                "放松训练: 通过肌肉松弛训练、冥想、瑜伽等方式来放松身体和心灵。",
                "运动: 运动可以帮助释放紧张情绪，增加身体的健康感。",
                "规律作息: 保持规律的作息时间，有助于身体和心灵的平衡。",
                "请注意，这些方法仅有可能缓解您的症状，只有及时就医才能彻底治愈。",
            ]),
            symptom: Some(&[
                "您可能有各种各样不同的症状表现，其中最为明显的就是躯体症状表现和情绪症状表现：",
                "您常常有心慌、胸闷气短、呼吸急促、头晕、头痛、尿频、尿急、恶心、呕吐、腹痛、腹泻等症状；",
                "您还会存在着明显的害怕和恐惧的心理，而且自己还会不由自主的出现各种各样的小动作。",
            ]),
            status: Status::Moderate,
        },
        InterpretationItem{
            range: [70, 100],
            description: "重度焦虑",
            advice: Some(&[
                "重度焦虑症的症状可能对个体的日常生活造成显著影响，因此寻求专业帮助是至关重要的。心理治疗、药物治疗或二者结合可能是有效的治疗方法。",
                "您应该立即寻求医生、心理医生或其他专业健康专家的帮助。及早的干预和治疗有助于缓解症状和提高生活质量。"
            ]),
            symptom: Some(&[
                "您可能有严重的身体、情绪和行为上的症状，下面是一些可能的表现：",
                "身体：高度紧张和肌肉紧绷，可能导致慢性的肌肉疼痛；心跳急速，可能伴随心悸；深呼吸困难，有时可能出现呼吸过快或呼吸不规律；持续的胃部不适，可能导致消化系统问题，如胃溃疡或胃肠道紊乱。",
                "情绪：持续的强烈焦虑和紧张感，常伴有强烈的害怕和恐慌感，深度的无助感和绝望感，易激动，情绪波动明显。",
                "行为：避免与焦虑有关的人、地或事物产生交集；难以集中注意力，思维混乱；失眠或睡眠质量极差；可能伴随有自伤或自杀的念头。",
                "社交回避：避免社交场合，可能由于担心被批评、被评价或引起注意。",
                "强迫症状：有时重度焦虑症可能伴随有强迫症状，如强烈的重复行为或思维。",
                "生活：对日常生活活动的参与可能受到明显影响，包括工作、学习和家庭生活。",
            ]),
            status: Status::Major,
        },
    ],
    questions: &[
        Question {
            title: "我感到比往常更加神经过敏和焦虑",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我无缘无故感到担心",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我容易心烦意乱或感到恐慌",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我感到我的身体好像被分成几块，支离破碎",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我感到事事都很顺利，不会有倒霉的事情发生",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 4,
                },
                QuestionOption {
                    text: "有时",
                    point: 3,
                },
                QuestionOption {
                    text: "经常",
                    point: 2,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 1,
                },
            ],
        },
        Question {
            title: "我的四肢抖动和震颤",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我因头痛、颈痛、背痛而烦恼",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我感到无力且容易疲劳",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我感到很平静，能安静坐下来",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 4,
                },
                QuestionOption {
                    text: "有时",
                    point: 3,
                },
                QuestionOption {
                    text: "经常",
                    point: 2,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 1,
                },
            ],
        },
        Question {
            title: "我感到我的心跳较快",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我因阵阵的眩晕而不舒服",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我有阵阵要昏倒的感觉",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我呼吸时进气和出气都不费力",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 4,
                },
                QuestionOption {
                    text: "有时",
                    point: 3,
                },
                QuestionOption {
                    text: "经常",
                    point: 2,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 1,
                },
            ],
        },
        Question {
            title: "我的手指和脚趾感到麻木和刺痛",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我因胃痛和消化不良而苦恼",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我必须时常排尿",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我的手总是很温暖而干燥",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 4,
                },
                QuestionOption {
                    text: "有时",
                    point: 3,
                },
                QuestionOption {
                    text: "经常",
                    point: 2,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 1,
                },
            ],
        },
        Question {
            title: "我觉得脸发烧发红",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
        Question {
            title: "我容易入睡，晚上休息很好",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 4,
                },
                QuestionOption {
                    text: "有时",
                    point: 3,
                },
                QuestionOption {
                    text: "经常",
                    point: 2,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 1,
                },
            ],
        },
        Question {
            title: "我做恶梦",
            options: &[
                QuestionOption {
                    text: "从无或偶尔",
                    point: 1,
                },
                QuestionOption {
                    text: "有时",
                    point: 2,
                },
                QuestionOption {
                    text: "经常",
                    point: 3,
                },
                QuestionOption {
                    text: "总是如此",
                    point: 4,
                },
            ],
        },
    ]
};
