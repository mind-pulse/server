use crate::scale::ScaleCategory;

use super::{
    ComfortingWord, CriticalWarning, FormulaMode, HTMLElement, Integer, InterpretationItem,
    OperationalRule, Question, QuestionOption, Scale, SentenceItem, Status, Tag, Texts,
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

pub const SELF_RATING_DEPRESSION_SCALE: Scale<&[InterpretationItem<u8>], Question> = Scale {
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
            description: "正常范围",
            critical_warning: None,
            comforting_word: ComfortingWord {
                text: "您的得分在正常范围内，这表明您当前的心理状态比较健康。",
                caution: None,
            },
            advice: Some(&["请继续保持积极的生活态度，关注生活中的小美好，您的阳光心态也能为他人带来温暖。"]),
            symptom: None,
            status: Status::Normal,
        },
        InterpretationItem{
            range: [53, 63],
            description: "轻度抑郁倾向",
            critical_warning: None,
            comforting_word: ComfortingWord {
                text: "如果您的得分在这个范围，请别太担心，很多人都会经历类似的情绪波动。",
                caution: None,
            },
            advice: Some(&[
                "与他人倾诉：和信任的亲友分享您的感受，寻求理解和支持，倾诉本身就能减轻心理负担。",
                "规律生活作息：尽量保证充足睡眠、均衡饮食和适度运动，这有助于稳定身心状态。",
                "寻求专业支持：如果这些情绪持续较长时间或影响生活，考虑咨询心理医生或心理咨询师，他们能提供更个性化的帮助。",
                "设定小目标：将大任务分解成小步骤，每完成一点就给自己肯定，慢慢积累信心和成就感。",
                "学习放松方法：尝试深呼吸、冥想或轻度伸展，帮助缓解紧张和焦虑。",
                "保持社交连接：即使情绪低落，也尽量与朋友家人保持联系，减少孤独感。",
                "培养兴趣爱好：花时间做自己喜欢的事，哪怕只是听音乐、散步，让生活多一些乐趣。",
                "减轻压力负担：合理规划工作生活，学会拒绝过度任务，给自己留出休息时间。",
            ]),
            symptom: Some(&[
                "· 常见症状：早晨心情沉重、体重轻微下降、头脑昏沉、偶尔觉得自己没用。",
                "· 有时会出现：食欲下降、做事感到吃力、对未来感到迷茫、难以做决定、觉得生活乏味、对以往喜欢的事物兴趣减少。",
                "· 偶尔发生：情绪低落、想哭、便秘、感到疲劳。",
            ]),
            status: Status::Mild,
        },
        InterpretationItem{
            range: [63, 73],
            description: "中度抑郁",
            critical_warning: None,
            comforting_word: ComfortingWord {
                text: "您的得分显示有中度抑郁倾向，这可能需要更多关注和支持。",
                caution: Some("中度抑郁并非您的错，它是一种可治疗的医疗状况。寻求帮助是勇敢的表现。"),
            },
            advice: Some(&[
                "建立支持网络：主动与亲友沟通，让他们了解您的状态，获得陪伴和理解。",
                "维持健康习惯：注意规律饮食、适度运动和睡眠，这些小习惯对心理有积极影响。",
                "设定可行目标：从微小、具体的任务开始，比如每天散步10分钟，逐步恢复掌控感。",
                "学习应对技巧：练习识别负面思维，用更平衡的想法替代，比如告诉自己“这只是暂时的困难”。",
                "参与社交活动：即使感觉困难，也尝试参加一些轻松的小聚会或线上交流，避免孤立自己。",
            ]),
            symptom: Some(&[
                "· 情绪持续低落：经常感到沮丧、悲伤或无助。",
                "· 兴趣减退：对以前喜欢的活动失去热情，做事缺乏动力。",
                "· 睡眠变化：失眠、早醒或睡眠过多。",
                "· 食欲改变：食欲明显下降或暴饮暴食。",
                "· 精力不足：总是疲劳，即使简单事务也感觉费力。",
                "· 注意力下降：难以集中精神，工作或学习效率降低。",
                "· 负面思维：容易自责、自我批评，对未来悲观。",
                "· 体重波动：体重可能明显增加或减少。",
                "· 身体不适：可能出现头痛、胃痛等，但没有明确生理原因。",
                "· 社交回避：不想与人交往，感觉与他人疏远。",
            ]),
            status: Status::Moderate,
        },
        InterpretationItem{
            range: [73, 100],
            description: "重度抑郁",
            critical_warning: Some(CriticalWarning {
                title:"需要立即寻求帮助",
                content:"重度抑郁需要专业医疗干预，请立即联系精神科医生或前往医院。"
            }),
            comforting_word: ComfortingWord {
                text: "您的得分表明可能有重度抑郁倾向，这需要立即的专业干预。",
                caution: Some("重度抑郁并非您的错，它是一种可治疗的医疗状况。寻求帮助是勇敢的表现。"),
            },
            advice: Some(&[
                "立即行动：尽快前往精神专科医院或综合医院精神科就诊，让专业医生进行评估。",
                "考虑全面治疗：医生可能会建议药物治疗、心理治疗或住院治疗，请信任专业方案。",
                "不要独自承受：告诉家人或朋友您的情况，寻求他们的陪伴和支持。",
            ]),
            symptom: Some(&[
                "· 情绪方面：持续深切的悲伤、绝望感；对所有活动失去兴趣；情绪易怒或麻木；严重自责；失眠或整天嗜睡；极度疲劳。",
                "· 身体方面：频繁头痛、胃痛或肌肉疼痛；食欲剧变导致体重明显增减；注意力难以集中；性欲减退。",
                "· 认知方面：强烈的无用感或负罪感；决策困难；记忆力变差；无法感受快乐。",
                "· 社交方面：完全回避社交；对他人冷漠；言语和行动变得迟缓。",
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
