use crate::scale::category::ScaleCategory;
use crate::scale::common::{
    ComfortingWord, CriticalWarning, FormulaMode, HTMLElement, Integer, InterpretationItem,
    OperationalRule, Question, QuestionOption, Scale, SentenceItem, Status, Tag, Texts,
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

pub const SELF_RATING_ANXIETY_SCALE: Scale<&[InterpretationItem<u8>], Question> = Scale {
    id: 8,
    name: "焦虑自评量表",
    description: "用科学方式读懂自己的情绪，为内心的平静提供清晰指引",
    abbreviation: "SAS",
    primary_category: ScaleCategory::Emotion,
    related_categories: Some(&[ScaleCategory::MentalHealth]),
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: None,
    warning: None,
    formula_mode: Some(FormulaMode{
        operational_rule: OperationalRule::Multiply(1.25),
        integer: Some(Integer::Round),
    }),
    tags: &Tag{ info: Some(&["焦虑", "压力", "失眠"]), normal: None, warning: None, error: None },
    interpretation: &[
        InterpretationItem{
            range: [0, 50],
            description: "正常",
            critical_warning: None,
            comforting_word: ComfortingWord {
                text: "很高兴看到你拥有比较平稳的心态！这说明你最近在情绪调节和生活平衡上做得不错。就像定期体检一样，偶尔关注自己的心理状态，是爱自己的重要方式。",
                caution: None,
            },
            advice: Some(&["请继续保持这份内心的平和，你积极的生活态度不仅能滋养自己，也能像阳光一样温暖身边的人。"]),
            symptom: None,
            status: Status::Normal,
        },
        InterpretationItem{
            range: [50, 60],
            description: "轻度焦虑",            
            critical_warning: None,
            comforting_word: ComfortingWord {
                text: "你可能正感受到一些焦虑情绪的波动，这是生活中常见的反应，很多人都会经历。",
                caution: None,
            },
            advice: Some(&[
                "给情绪一个出口：找个安全私密的空间（比如自己的房间、车内），大声播放一首喜欢的歌并跟着唱，或者写日记、画画，把纷乱的想法“倒”出来。",
                "呵护你的睡眠：试着建立规律的作息，睡前一小时远离手机。充足的睡眠是情绪的“稳定器”，休息好了，我们会更有精力面对挑战。",
                "转移注意力：主动做一些能带来小确幸的事，比如散步、做一道菜、拼图。当注意力投入在具体的事情上，胡思乱想的空间就变小了。",
            ]),
            symptom: Some(&[
                "身体上：可能会比较容易感到疲劳、失眠，或有心慌、胸闷等紧张感。",
                "情绪上：有时会感到烦躁、坐立不安，或对一些事情过分担心。",
                "想法上：偶尔会想一个人静静，对事物的兴趣可能有所下降。",
            ]),
            status: Status::Mild,
        },
        InterpretationItem{
            range: [60, 70],
            description: "中度焦虑",            
            critical_warning: None,
            comforting_word: ComfortingWord {
                text: "焦虑情绪对你目前的影响已经比较明显，这段时间你可能感觉比较辛苦，情绪上的困扰已经带来了一些实实在在的影响。",
                caution: Some("请记住，感到焦虑不是你的错，也不是软弱的表现。这恰恰说明你正在努力应对一些压力，而寻求改变本身就是一种勇气。"),
            },
            advice: Some(&[
                "即时舒缓技巧：尝试“478呼吸法”（吸气4秒，屏息7秒，呼气8秒），快速平复生理上的紧张感。",
                "学习放松训练：每天抽出10分钟，跟随引导音频进行正念冥想或渐进式肌肉放松，像锻炼肌肉一样锻炼你的“放松能力”。",
                "让身体动起来：规律的运动（如快走、瑜伽）是天然的焦虑缓解剂，能有效释放压力，提升情绪。",
                "建立生活节奏：尽量固定每天吃饭、睡觉、活动的时间，稳定的生活框架能带来安全感。",
            ]),
            symptom: Some(&[
                "身体反应更频繁：可能会经常感到头晕、头痛、肠胃不适，或肌肉紧绷。",
                "情绪消耗较大：紧张、害怕的感觉比较强烈，甚至有时不知道为什么就心慌起来。",
                "行为变化：可能会下意识地避免一些场合，或发现自己很难集中注意力。",
            ]),
            status: Status::Moderate,
        },
        InterpretationItem{
            range: [70, 100],
            description: "重度焦虑",
            critical_warning: Some(CriticalWarning {
                title:"需要立即寻求帮助",
                content:"重度焦虑需要专业医疗干预，请立即联系精神科医生或前往医院，进行全面的评估。"
            }),
            comforting_word: ComfortingWord {
                text: "看到这个结果，你或许会感到害怕或无助。请首先知道：你并不孤单，这种情况是可以被有效治疗的。",
                caution: Some("强烈的焦虑感就像心灵的重感冒，它需要专业医生的“诊断”和“处方”。主动寻求帮助，是此刻你能为自己做的最强大、最正确的事情。"),
            },
            advice: Some(&[
                "立即联系专业人士：建议优先考虑前往医院的精神科或心理科，进行全面的评估。也可以寻求有资质的心理咨询师/治疗师的帮助。",
                "你不是负担：将你的感受告诉你信任的家人或朋友，获得他们的支持。你的痛苦是真实的，值得被认真对待。",
                "安全第一：如果脑海中出现伤害自己或他人的念头，请立即拨打心理援助热线（24小时全国生命危机干预热线：400-161-9995），或前往最近的医院急诊室。保护你的生命安全永远是第一位。"
            ]),
            // 重度焦虑尽量减少症状描述，避免增加用户的焦虑
            symptom: Some(&[
                "身体长期处于“警报”状态，比如心慌、胸闷、肌肉紧绷疼痛或严重的睡眠问题。",
                "情绪被强烈的紧张、恐惧或无助感淹没，难以放松，甚至影响基本的思考与注意力。",
                "日常生活、工作或社交变得异常困难，可能会产生回避行为，或出现一些自己无法控制的担忧与念头。",
            ]),
            status: Status::Major,
        },
    ],
    questions: &[
        Question {
            title: "我感到比往常更加神经过敏和焦虑",
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
            is_multiple: false,
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
