use crate::scale::ScaleCategory;

use super::{
    ComfortingWord, CriticalWarning, HTMLElement, InterpretationItem, Question, QuestionOption,
    Scale, SentenceItem, Status, Tag, Texts,
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
    name: "贝克抑郁自评量表",    
    primary_category: ScaleCategory::Emotion,
    related_categories: Some(&[ScaleCategory::MentalHealth, ScaleCategory::Somatic]),
    abbreviation: "BDI",
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: None,
    warning: None,
    formula_mode: None,
    tags: Tag{ info: Some(&["抑郁"]), normal: None, warning: None, error: None },
    interpretation: &[
        InterpretationItem{
            range: [0, 5],
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
            range: [5, 8],
            description: "轻度抑郁",
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
                "常见症状：早晨心情沉重、体重轻微下降、头脑昏沉、偶尔觉得自己没用。",
                "有时会出现：食欲下降、做事感到吃力、对未来感到迷茫、难以做决定、觉得生活乏味、对以往喜欢的事物兴趣减少。",
                "偶尔发生：情绪低落、想哭、便秘、感到疲劳。",
            ]),
            status: Status::Mild,
        },
        InterpretationItem{
            range: [8, 16],
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
                "情绪持续低落：经常感到沮丧、悲伤或无助。",
                "兴趣减退：对以前喜欢的活动失去热情，做事缺乏动力。",
                "睡眠变化：失眠、早醒或睡眠过多。",
                "食欲改变：食欲明显下降或暴饮暴食。",
                "精力不足：总是疲劳，即使简单事务也感觉费力。",
                "注意力下降：难以集中精神，工作或学习效率降低。",
                "负面思维：容易自责、自我批评，对未来悲观。",
                "体重波动：体重可能明显增加或减少。",
                "身体不适：可能出现头痛、胃痛等，但没有明确生理原因。",
                "社交回避：不想与人交往，感觉与他人疏远。",
            ]),
            status: Status::Moderate,
        },
        InterpretationItem{
            range: [16, 40],
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
                "情绪方面：持续深切的悲伤、绝望感；对所有活动失去兴趣；情绪易怒或麻木；严重自责；失眠或整天嗜睡；极度疲劳。",
                "身体方面：频繁头痛、胃痛或肌肉疼痛；食欲剧变导致体重明显增减；注意力难以集中；性欲减退。",
                "认知方面：强烈的无用感或负罪感；决策困难；记忆力变差；无法感受快乐。",
                "社交方面：完全回避社交；对他人冷漠；言语和行动变得迟缓。",
            ]),
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
    ]
};
