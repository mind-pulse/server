use super::{
    HTMLElement, InterpretationItem, Question, QuestionOption, Scale, SentenceItem, Status, Tag,
    Texts,
};

const INTRODUCTION: Texts = &[&[
    SentenceItem::Plain("贝克抑郁自评量表BDI（BDI-II是第二版）是抑郁自评量表中"),
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

pub const BECK_DEPRESSION_RATING_SCALE: Scale<&[InterpretationItem<i8>], Question> = Scale {
    name: "贝克抑郁自评量表",
    abbreviation: "BDI",
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: Some(&["钱铭怡等. 艾森克人格问卷简式量表中国版(EPQ-RSC)的修订. 心理学报. 2000"]),
    warning: None,
    formula_mode: None,
    tags: Tag{ info: Some(&["抑郁"]), normal: Some(&["自评"]), warning: None, error: None },
    interpretation: &[
        InterpretationItem{
            range: [0, 5],
            description: "正常",
            advice: Some(&["请继续保持积极阳光的生活态度，面对人生，相信你会遇到很多惊喜，同时也能给他人带来快乐！"]),
            symptom: None,
            status: Status::Normal,
        },
        InterpretationItem{
            range: [5, 8],
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
            range: [8, 16],
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
            range: [16, 40],
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
            title: "以下情况最符合你的是",
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
