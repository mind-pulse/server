use crate::scale::{
    FormulaMode, HTMLElement, Integer, InterpretationItem, OperationalRule, Question,
    QuestionOption, Scale, ScaleCategory, SentenceItem, Status, Tag, Texts,
};

use super::depression_guidance::{
    MAJOR_ADVICE, MAJOR_COMFORT, MAJOR_SYMPTOMS, MAJOR_WARNING, MILD_ADVICE, MILD_COMFORT,
    MILD_SYMPTOMS, MODERATE_ADVICE, MODERATE_COMFORT, MODERATE_SYMPTOMS, NORMAL_ADVICE,
    NORMAL_COMFORT,
};

const INTRODUCTION: Texts = &[&[
    SentenceItem::Plain(
        "抑郁自评量表（Self-Rating Depression Scale，SDS）是由美国杜克大学医学院的 William W. K. Zung 于 1965 年编制的，是目前应用",
    ),
    SentenceItem::HTMLElement(HTMLElement::Strong("最广泛")),
    SentenceItem::Plain(
        "的抑郁自评量表之一，能有效地反映抑郁状态的有关症状及其严重程度和变化情况。",
    ),
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

pub const SELF_RATING_DEPRESSION_SCALE: Scale<&[InterpretationItem<u8>], Question> = Scale {
    id: 7,
    name: "抑郁自评量表",
    description: "快速自测情绪状态，用20个问题看清你是否被抑郁悄悄困扰",
    abbreviation: "SDS",
    primary_category: ScaleCategory::Emotion,
    related_categories: Some(&[ScaleCategory::MentalHealth]),
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: Some(&[
        "Zung, W. W. K. (1965). A self-rating depression scale. Archives of General Psychiatry, 12(1), 63-70.",
    ]),
    warning: None,
    formula_mode: Some(FormulaMode {
        operational_rule: OperationalRule::Multiply(1.25),
        integer: Some(Integer::Round),
    }),
    tags: &Tag {
        info: Some(&["抑郁"]),
        normal: None,
        warning: None,
        error: None,
    },
    interpretation: &[
        InterpretationItem {
            range: [0, 53],
            description: "正常范围",
            critical_warning: None,
            comforting_word: NORMAL_COMFORT,
            advice: Some(NORMAL_ADVICE),
            symptom: None,
            status: Status::Normal,
        },
        InterpretationItem {
            range: [53, 63],
            description: "轻度抑郁倾向",
            critical_warning: None,
            comforting_word: MILD_COMFORT,
            advice: Some(MILD_ADVICE),
            symptom: Some(MILD_SYMPTOMS),
            status: Status::Mild,
        },
        InterpretationItem {
            range: [63, 73],
            description: "中度抑郁",
            critical_warning: None,
            comforting_word: MODERATE_COMFORT,
            advice: Some(MODERATE_ADVICE),
            symptom: Some(MODERATE_SYMPTOMS),
            status: Status::Moderate,
        },
        InterpretationItem {
            range: [73, 100],
            description: "重度抑郁",
            critical_warning: Some(MAJOR_WARNING),
            comforting_word: MAJOR_COMFORT,
            advice: Some(MAJOR_ADVICE),
            symptom: Some(MAJOR_SYMPTOMS),
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
    ],
};
