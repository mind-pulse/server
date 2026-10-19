use super::{Dimension, Question, QuestionOption};

pub(super) const QUESTIONS: &[Question] = &[
    Question {
        title: "你的情绪是否时起时落?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "当你看到小孩(或动物)受折磨时是否感到难受?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你是个健谈的人吗?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "如果你说了要做什么事，是否不论此事可能如果不顺利你都总能遵守诺言?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否会无言无故地感到“很惨”?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "欠债会使你感到忧虑吗?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你是个生气勃勃的人吗?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否曾贪图过超过你应得的分外之物?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你是个容易被激怒的人吗?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你会服用能产生奇异或危险效果的药物吗?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你愿意认识陌生人吗?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否曾经有过明知自己做错了事却责备别人的情况?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你的感情容易受伤害吗?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否愿意按照自己的方式行事，而不愿意按照规则办事?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "在热闹的聚会中你能使自己放得开，使自己玩得开心吗?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你所有的习惯是否都是好的?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否时常感到“极其厌倦”?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "良好的举止和整洁对你来说很重要吗?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "在结交新朋友时，你经常是积极主动的吗?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否有过随口骂人的时候?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你认为自己是一个胆怯不安的人吗?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否认为婚姻是不合时宜的，应该废除?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你能否很容易地给一个沉闷的聚会注入活力?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你曾毁坏或丢失过别人的东西吗?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你是个忧心忡忡的人吗?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你爱和别人合作吗?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "在社交场合你是否倾向于呆在不显眼的地方?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "如果在你的工作中出现了错误，你知道后会感到忧虑吗?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你讲过别人的坏话或脏话吗?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你认为自己是个神经紧张或“弦绷得过紧”的人吗?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否觉得人们为了未来有保障，而在储蓄和保险方面花费的时间太多了?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否喜欢和人们相处在一起?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "当你还是个小孩子的时候，你是否曾有过对父母耍赖或不听话的行为?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "在经历了一次令人难堪的事之后，你是否会为此烦恼很长时间?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否努力使自己对人不粗鲁?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你是否喜欢在自己周围有许多热闹和令人兴奋的事情?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你曾在玩游戏时作过弊吗?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你是否因自己的“神经过敏”而感到痛苦?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你愿意别人怕你吗?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你曾利用过别人吗?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你是否喜欢说笑话和谈论有趣的事?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否时常感到孤独?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否认为遵循社会规范比按照个人方式行事更好一些?",
        dimension: Dimension::P,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "在别人眼里你总是充满活力的吗?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你总能做到言行一致吗?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你是否时常被内疚感所困扰?",
        dimension: Dimension::N,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
    Question {
        title: "你有时将今天该做的事情拖到明天去做吗?",
        dimension: Dimension::L,
        options: &[
            QuestionOption {
                text: "是",
                point: 0,
            },
            QuestionOption {
                text: "不是",
                point: 1,
            },
        ],
    },
    Question {
        title: "你能使一个聚会顺利进行下去吗?",
        dimension: Dimension::E,
        options: &[
            QuestionOption {
                text: "是",
                point: 1,
            },
            QuestionOption {
                text: "不是",
                point: 0,
            },
        ],
    },
];
