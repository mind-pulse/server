use crate::scale::ScaleCategory;

use super::{
    FormulaMode, HTMLElement, Integer, InterpretationItem, OperationalRule, Question,
    QuestionOption, Scale, SentenceItem, Status, Tag, Texts,
};

const INTRODUCTION: Texts = &[&[
    SentenceItem::Plain("抑郁自评量表（Self-Rating Depression Scale，SDS）是由美国杜克大学医学院的 William W. K. Zung 于 1965 年编制的，是目前应用"),
    SentenceItem::HTMLElement(HTMLElement::Strong("最广泛")),
    SentenceItem::Plain("的抑郁自评量表之一，能有效地反映抑郁状态的有关症状及其严重程度和变化情况。"),
]];

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

pub const SELF_RATING_DEPRESSION_SCALE: Scale<&[InterpretationItem<i8>], Question> = Scale {
    name: "抑郁自评量表",
    abbreviation: "SDS",
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
    tags: Tag{ info: Some(&["抑郁"]), normal: None, warning: None, error: None },
    interpretation: &[
        InterpretationItem{
            range: [0, 53],
            description: "正常",
            advice: Some(&["请继续保持积极阳光的生活态度，面对人生，相信你会遇到很多惊喜，同时也能给他人带来快乐！"]),
            symptom: None,
            status: Status::Normal,
        },
        InterpretationItem{
            range: [53, 63],
            description: "轻度抑郁",
            advice: Some(&[
                "与他人交流： 与亲朋好友分享感受，寻求支持和理解，有时候倾诉可以减轻心理负担。",
                "建立规律的生活： 确保有充足的睡眠，合理的饮食，以及规律的锻炼。这有助于维持身体和心理的平衡。",
                "寻求专业帮助： 如果感觉自己无法应对或情况持续时间较长，考虑咨询心理医生、心理治疗师或其他专业人士的帮助。",
                "设定小目标： 将大目标分解为小目标，逐步实现，有助于提高自信心和成就感。",
                "学习放松技巧： 学习深呼吸、渐进性肌肉松弛等放松技巧，有助于缓解焦虑和紧张感。",
                "保持社交活动： 虽然抑郁可能让人不愿意参与社交活动，但与他人保持联系可以减轻孤独感。",
                "寻找喜好和爱好： 发掘并培养一些喜欢的活动，这有助于提高生活质量并增加乐趣。",
                "避免过度负担： 合理安排工作和生活，避免过度压力，学会拒绝过多的任务。",
                "需要强调的是，这些建议可能对某些人有效，但并不是适用于所有情况。如果情况严重或持续，请寻求专业帮助。专业的心理医生可以根据具体情况提供更为个体化和有效的建议。",
            ]),
            symptom: Some(&[
                "早晨心情沉重、体重减轻、头脑不清楚、感到自己无用。",
                "您有时：饭量下降、感到做事困难、觉得未来没有希望、觉得难以下决定、生活没有意义、不喜爱自己平时喜爱的东西。",
                "您偶尔：感到情绪沮丧、郁闷、要哭或想哭、便秘、感到疲劳。",
            ]),
            status: Status::Mild,
        },
        InterpretationItem{
            range: [63, 73],
            description: "中度抑郁",
            advice: Some(&[
                "首先建议您寻求心理医生、心理治疗师或精神医生的帮助，同时可以通过以下办法缓解症状：",
                "建立支持系统：与亲友分享感受，寻找理解和支持。",
                "保持健康的生活方式：注意饮食、运动和睡眠，这对心理健康有积极影响。",
                "制定目标：设立小目标，逐步实现，有助于增加成就感。",
                "学习应对技巧：学会处理负面思维，采用正面的应对方式。",
                "参与社交活动：虽然可能感到困难，但尽量参与社交活动，避免孤立。",
            ]),
            symptom: Some(&[
                "您可能存在以下症状：",
                "情绪低落：持续的沮丧、悲伤、无望感。",
                "失去兴趣：对平常喜欢的活动失去兴趣，缺乏动力参与社交活动或爱好。",
                "睡眠问题：可能是失眠、早醒或过度睡眠。",
                "食欲改变：可能是食欲减退或过度进食。",
                "能量下降：感觉疲劳、无力，即使做小事也感觉很吃力。",
                "注意力难以集中：难以集中注意力，工作或学业表现可能下降。",
                "负面思维：常常有负面的自我评价、自责，对未来持悲观态度。",
                "体重变化：可能伴随体重的明显增加或减少。",
                "身体症状：如头痛、胃痛等生理不适，没有明显的生理原因。",
                "社交隔离：感觉与他人疏远，不愿与人交往。",
            ]),
            status: Status::Moderate,
        },
        InterpretationItem{
            range: [73, 100],
            description: "重度抑郁",
            advice: Some(&[
                "对于重度抑郁症，专业治疗是至关重要的，强烈建议您去精神专科医院门诊寻求专业帮助，住院治疗。",
            ]),
            symptom: Some(&[
                "您可能有一系列身体、情绪和认知方面的症状，以下是一些可能的表现：",
                "情绪方面：持续的沮丧和悲伤感；失去兴趣或乐趣对任何活动的兴趣；情绪波动，容易激动或烦躁；自责感和无望感；失眠或过度睡眠；疲劳和能量不足。",
                "身体方面：头痛、胃痛、肌肉疼痛等不适感；食欲变化，可能导致体重减轻或增加；注意力和集中力下降；性欲减退。",
                "认知方面：负面的自我评价和自卑感；集中力和决策能力下降；记忆力减退；无法享受正常的活动。",
                "社交方面：社交退缩，避免与他人交往；对他人的兴趣减少；对待他人的态度变得更加消极。",
            ]),
            status: Status::Major,
        },
    ],
    questions: &[
        Question {
            title: "我感到情绪沮丧，郁闷。",
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
            title: "我感到早晨心情最好。",
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
            title: "我要哭或想哭。",
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
            title: "我夜间睡眠不好。",
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
            title: "我吃饭像平常一样多。",
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
            title: "我的性功能正常。",
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
            title: "我感到体重减轻。",
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
            title: "我为便秘烦恼。",
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
            title: "我的心跳比平时快。",
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
            title: "我无故感到疲乏。",
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
            title: "我的头脑像平常一样清楚。",
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
            title: "我做事情像平常一样不感到困难。",
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
            title: "我坐卧难安，难以保持平静。",
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
            title: "我对未来感到有希望。",
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
            title: "我比平时更容易激怒。",
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
            title: "我觉得决定什么事很容易。",
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
            title: "我感到自己是有用的和不可缺少的人。",
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
            title: "我的生活很有意思。",
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
            title: "假若我死了，别人会过得更好。",
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
            title: "我仍旧喜欢自己平时喜欢的东西。",
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
    ]
};
