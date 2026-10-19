use crate::scale::{
    HTMLElement, InterpretationItem, Question, QuestionOption, Scale, ScaleCategory, SentenceItem,
    Status, Tag, Texts,
};

use super::depression_guidance::{
    MAJOR_ADVICE, MAJOR_COMFORT, MAJOR_SYMPTOMS, MAJOR_WARNING, MILD_ADVICE, MILD_COMFORT,
    MILD_SYMPTOMS, MODERATE_ADVICE, MODERATE_COMFORT, MODERATE_SYMPTOMS, NORMAL_ADVICE,
    NORMAL_COMFORT,
};

const INTRODUCTION: Texts = &[&[
    SentenceItem::Plain("贝克抑郁自评量表 BDI（BDI-II 是第二版）是抑郁自评量表中"),
    SentenceItem::HTMLElement(HTMLElement::Strong("最著名的量表")),
    SentenceItem::Plain("之一，被广泛运用于抑郁心理测试筛查。"),
]];

const INSTRUCTION: Texts = &[&[
    SentenceItem::Plain("本量表包含 "),
    SentenceItem::HTMLElement(HTMLElement::Strong("13")),
    SentenceItem::Plain(" 个项目，分为 "),
    SentenceItem::HTMLElement(HTMLElement::Strong("4")),
    SentenceItem::Plain(" 级评分，为保证调查结果的准确性，务必请您仔细阅读以下内容，然后根据您"),
    SentenceItem::HTMLElement(HTMLElement::Strong("最近 1 周")),
    SentenceItem::Plain(
        "的实际情况选择适当的选项，每一条文字后面有四个选项，请根据选项内容进行恰当的选择。",
    ),
]];

pub const BECK_DEPRESSION_INVENTORY: Scale<&[InterpretationItem<u8>], Question> = Scale {
    id: 6,
    name: "贝克抑郁自评量表",
    description: "13个问题帮你温和地看清自己的情绪状态",
    primary_category: ScaleCategory::Emotion,
    related_categories: Some(&[ScaleCategory::MentalHealth, ScaleCategory::Somatic]),
    abbreviation: "BDI",
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: None,
    warning: None,
    formula_mode: None,
    tags: &Tag {
        info: Some(&["抑郁"]),
        normal: None,
        warning: None,
        error: None,
    },
    interpretation: &[
        InterpretationItem {
            range: [0, 5],
            description: "正常范围",
            critical_warning: None,
            comforting_word: NORMAL_COMFORT,
            advice: Some(NORMAL_ADVICE),
            symptom: None,
            status: Status::Normal,
        },
        InterpretationItem {
            range: [5, 8],
            description: "轻度抑郁",
            critical_warning: None,
            comforting_word: MILD_COMFORT,
            advice: Some(MILD_ADVICE),
            symptom: Some(MILD_SYMPTOMS),
            status: Status::Mild,
        },
        InterpretationItem {
            range: [8, 16],
            description: "中度抑郁",
            critical_warning: None,
            comforting_word: MODERATE_COMFORT,
            advice: Some(MODERATE_ADVICE),
            symptom: Some(MODERATE_SYMPTOMS),
            status: Status::Moderate,
        },
        InterpretationItem {
            range: [16, 40],
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
            title: "以下情况最符合你的是",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我不感到忧郁",
                    point: 0,
                },
                QuestionOption {
                    text: "我感到忧郁或沮丧",
                    point: 1,
                },
                QuestionOption {
                    text: "我整天忧郁，无法摆脱",
                    point: 2,
                },
                QuestionOption {
                    text: "我十分忧郁，已经承受不住",
                    point: 3,
                },
            ],
        },
        Question {
            title: "你对未来抱有什么态度？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我对未来并不感到悲观失望",
                    point: 0,
                },
                QuestionOption {
                    text: "我感到前途不太乐观",
                    point: 1,
                },
                QuestionOption {
                    text: "我感到我对前途不抱希望",
                    point: 2,
                },
                QuestionOption {
                    text: "我感到今后毫无希望，不可能有所好转",
                    point: 3,
                },
            ],
        },
        Question {
            title: "你是如何看待失败的感觉？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我并无失败的感觉",
                    point: 0,
                },
                QuestionOption {
                    text: "我觉得和大多数人相比我是失败的",
                    point: 1,
                },
                QuestionOption {
                    text: "回顾我的一生，我觉得那是一连串的失败",
                    point: 2,
                },
                QuestionOption {
                    text: "我觉得我是个彻底失败的人",
                    point: 3,
                },
            ],
        },
        Question {
            title: "你对生活的满意度如何？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我并不觉得我有什么不满意",
                    point: 0,
                },
                QuestionOption {
                    text: "我觉得我不能像平时那样享受生活",
                    point: 1,
                },
                QuestionOption {
                    text: "任何事情都不能使我感到满意一些",
                    point: 2,
                },
                QuestionOption {
                    text: "我对所有的事情都不满意",
                    point: 3,
                },
            ],
        },
        Question {
            title: "你的内疚感有多深？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没有特殊的内疚感",
                    point: 0,
                },
                QuestionOption {
                    text: "我有时感到内疚或觉得自己没价值",
                    point: 1,
                },
                QuestionOption {
                    text: "我感到非常内疚",
                    point: 2,
                },
                QuestionOption {
                    text: "我觉得自己非常坏，一钱不值",
                    point: 3,
                },
            ],
        },
        Question {
            title: "你是否会对自己感到失望？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没有对自己感到失望",
                    point: 0,
                },
                QuestionOption {
                    text: "我对自己感到失望",
                    point: 1,
                },
                QuestionOption {
                    text: "我讨厌自己",
                    point: 2,
                },
                QuestionOption {
                    text: "我憎恨自己",
                    point: 3,
                },
            ],
        },
        Question {
            title: "你会有想要伤害自己的想法吗？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没有要伤害自己的想法",
                    point: 0,
                },
                QuestionOption {
                    text: "我感到还是死掉的好",
                    point: 1,
                },
                QuestionOption {
                    text: "我考虑过自杀",
                    point: 2,
                },
                QuestionOption {
                    text: "如果有机会，我还会杀了自己",
                    point: 3,
                },
            ],
        },
        Question {
            title: "你是否失去与他人交往的兴趣？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没失去和他人交往的兴趣",
                    point: 0,
                },
                QuestionOption {
                    text: "和平时相比，我和他人交往的兴趣有所减退",
                    point: 1,
                },
                QuestionOption {
                    text: "我已失去大部分和人交往的兴趣，我对他们没有感情",
                    point: 2,
                },
                QuestionOption {
                    text: "我对他人全无兴趣，也完全不理睬别人",
                    point: 3,
                },
            ],
        },
        Question {
            title: "做决定对你来说，是否感到困难？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我能像平时一样做出决断",
                    point: 0,
                },
                QuestionOption {
                    text: "我尝试避免做决定",
                    point: 1,
                },
                QuestionOption {
                    text: "对我而言，做出决断十分困难",
                    point: 2,
                },
                QuestionOption {
                    text: "我无法做出任何决断",
                    point: 3,
                },
            ],
        },
        Question {
            title: "与过去相比，你是否对你的形象不自信？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我觉得我的形象一点也不比过去糟",
                    point: 0,
                },
                QuestionOption {
                    text: "我担心我看起来老了，不吸引人了",
                    point: 1,
                },
                QuestionOption {
                    text: "我觉得我的外表肯定变了，变得不具吸引力",
                    point: 2,
                },
                QuestionOption {
                    text: "我觉得我的形象丑陋不堪且讨人厌",
                    point: 3,
                },
            ],
        },
        Question {
            title: "你对工作抱有何种态度？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我能像平时那样工作",
                    point: 0,
                },
                QuestionOption {
                    text: "我做事时，要额外地努力才能开始",
                    point: 1,
                },
                QuestionOption {
                    text: "我必须努力迫使自己，方能干事",
                    point: 2,
                },
                QuestionOption {
                    text: "我完全不能做事情",
                    point: 3,
                },
            ],
        },
        Question {
            title: "和以往相比，你是否会很容易就感到疲倦？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "和以往相比，我并不容易疲倦",
                    point: 0,
                },
                QuestionOption {
                    text: "我比过去容易觉得疲倦",
                    point: 1,
                },
                QuestionOption {
                    text: "我做任何事都感到疲倦",
                    point: 2,
                },
                QuestionOption {
                    text: "我太易疲倦了，不能干任何事",
                    point: 3,
                },
            ],
        },
        Question {
            title: "与过去相比，你的胃口如何？",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我的胃口不比过去差",
                    point: 0,
                },
                QuestionOption {
                    text: "我的胃口没有过去那样好",
                    point: 1,
                },
                QuestionOption {
                    text: "现在我的胃口比过去差多了",
                    point: 2,
                },
                QuestionOption {
                    text: "我一点食欲都没有",
                    point: 3,
                },
            ],
        },
    ],
};
