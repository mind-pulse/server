use std::ops::RangeInclusive;

use serde::Serialize;

use crate::scale::ScaleCategory;

use super::{HTMLElement, PlainText, QuestionOption, Scale, SentenceItem, Tag, Texts};

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 维度
enum Dimension {
    /// 神经质，Negative Emotionality
    N,
    /// 外向性，Extraversion
    E,
    /// 开放性，Open-Mindedness
    O,
    /// 宜人性，Agreeableness
    A,
    /// 尽责性/责任心，Conscientiousness
    C,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Subdimension {
    N(NegativeEmotionalitySubdimension),
    E(ExtraversionSubdimension),
    O(OpenMindednessSubdimension),
    A(AgreeablenessSubdimension),
    C(ConscientiousnessSubdimension),
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 神经质子维度
enum NegativeEmotionalitySubdimension {
    /// 焦虑
    N1,
    /// 愤怒、敌意
    N2,
    /// 抑郁
    N3,
    /// 自我意识/人际关系敏感
    N4,
    /// 冲动性
    N5,
    /// 脆弱性
    N6,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 开放性子维度
enum OpenMindednessSubdimension {
    /// 想象力
    O1,
    /// 审美/艺术美感
    O2,
    /// 感受丰富度
    O3,
    /// 尝鲜。体验新鲜事物，而不是熟悉事物。
    O4,
    /// 思辨。求知。
    O5,
    /// 价值观
    O6,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 外向性子维度
enum ExtraversionSubdimension {
    /// 热情性
    E1,
    /// 乐群性
    E2,
    /// 自我肯定
    E3,
    /// 活跃性
    E4,
    /// 寻求刺激
    E5,
    /// 积极情感
    E6,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 宜人性子维度
enum AgreeablenessSubdimension {
    /// 信任
    A1,
    /// 坦城
    A2,
    /// 利他
    A3,
    /// 顺从
    A4,
    /// 谦逊
    A5,
    /// 同理心
    A6,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 尽责性/责任心子维度
enum ConscientiousnessSubdimension {
    /// 能力
    C1,
    /// 条理性
    C2,
    /// 责任感
    C3,
    /// 追求成就
    C4,
    /// 自律
    C5,
    /// 审慎
    C6,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct NormData {
    m: f64,
    sd: f64,
}

#[derive(Serialize)]
struct NormGender {
    dimension_norm: &'static [DimensionNorm; 5],
    subdimension_norm: &'static [SubdimensionNorm; 30],
}

#[derive(Serialize)]
struct DimensionNorm {
    dimension: Dimension,
    data: NormData,
}

#[derive(Serialize)]
struct SubdimensionNorm {
    dimension: Subdimension,
    data: NormData,
}

#[derive(Serialize)]
struct Norm {
    male: NormGender,
    female: NormGender,
}

const NORM: Norm = Norm {
    male: NormGender {
        dimension_norm: &[
            DimensionNorm {
                dimension: Dimension::N,
                data: NormData { m: 83.8, sd: 24.41 },
            },
            DimensionNorm {
                dimension: Dimension::E,
                data: NormData {
                    m: 109.97,
                    sd: 18.27,
                },
            },
            DimensionNorm {
                dimension: Dimension::O,
                data: NormData {
                    m: 110.65,
                    sd: 15.88,
                },
            },
            DimensionNorm {
                dimension: Dimension::A,
                data: NormData { m: 113.0, sd: 14.9 },
            },
            DimensionNorm {
                dimension: Dimension::C,
                data: NormData {
                    m: 123.89,
                    sd: 20.32,
                },
            },
        ],
        subdimension_norm: &[
            // 维度 N
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
                data: NormData { m: 14.15, sd: 5.7 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
                data: NormData { m: 13.82, sd: 5.24 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
                data: NormData { m: 14.37, sd: 5.7 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
                data: NormData { m: 15.35, sd: 4.73 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
                data: NormData { m: 14.86, sd: 4.88 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
                data: NormData { m: 11.23, sd: 5.09 },
            },
            // 维度 E
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E1),
                data: NormData { m: 21.53, sd: 4.27 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E2),
                data: NormData { m: 18.69, sd: 4.65 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E3),
                data: NormData { m: 15.1, sd: 4.37 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E4),
                data: NormData { m: 18.33, sd: 4.74 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E5),
                data: NormData { m: 17.24, sd: 4.64 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E6),
                data: NormData { m: 19.07, sd: 4.85 },
            },
            // 维度 O
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O1),
                data: NormData { m: 15.19, sd: 4.19 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O2),
                data: NormData { m: 20.7, sd: 4.88 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O3),
                data: NormData { m: 19.47, sd: 3.95 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O4),
                data: NormData { m: 15.02, sd: 3.61 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O5),
                data: NormData { m: 19.62, sd: 4.98 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O6),
                data: NormData { m: 20.64, sd: 3.97 },
            },
            // 维度 A
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A1),
                data: NormData { m: 21.07, sd: 3.94 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A2),
                data: NormData { m: 19.03, sd: 5.11 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A3),
                data: NormData { m: 20.99, sd: 3.53 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A4),
                data: NormData { m: 15.33, sd: 4.64 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A5),
                data: NormData { m: 16.67, sd: 4.06 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A6),
                data: NormData { m: 19.91, sd: 3.59 },
            },
            // 维度 C
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C1),
                data: NormData { m: 20.8, sd: 4.34 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C2),
                data: NormData { m: 18.39, sd: 4.21 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C3),
                data: NormData { m: 23.47, sd: 4.22 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C4),
                data: NormData { m: 20.02, sd: 4.34 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C5),
                data: NormData { m: 20.48, sd: 4.66 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C6),
                data: NormData { m: 20.72, sd: 4.63 },
            },
        ],
    },
    female: NormGender {
        dimension_norm: &[
            DimensionNorm {
                dimension: Dimension::N,
                data: NormData {
                    m: 87.26,
                    sd: 24.57,
                },
            },
            DimensionNorm {
                dimension: Dimension::E,
                data: NormData {
                    m: 107.13,
                    sd: 18.85,
                },
            },
            DimensionNorm {
                dimension: Dimension::O,
                data: NormData {
                    m: 110.31,
                    sd: 17.26,
                },
            },
            DimensionNorm {
                dimension: Dimension::A,
                data: NormData {
                    m: 119.0,
                    sd: 15.79,
                },
            },
            DimensionNorm {
                dimension: Dimension::C,
                data: NormData {
                    m: 125.08,
                    sd: 17.67,
                },
            },
        ],
        subdimension_norm: &[
            // 维度 N
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
                data: NormData { m: 14.81, sd: 5.45 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
                data: NormData { m: 13.99, sd: 5.16 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
                data: NormData { m: 14.46, sd: 5.45 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
                data: NormData { m: 16.38, sd: 5.27 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
                data: NormData { m: 14.89, sd: 5.1 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
                data: NormData { m: 12.73, sd: 5.18 },
            },
            // 维度 E
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E1),
                data: NormData { m: 22.05, sd: 4.31 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E2),
                data: NormData { m: 18.3, sd: 4.85 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E3),
                data: NormData { m: 14.59, sd: 4.67 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E4),
                data: NormData { m: 18.01, sd: 4.93 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E5),
                data: NormData { m: 14.25, sd: 5.35 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E6),
                data: NormData { m: 18.94, sd: 5.09 },
            },
            // 维度 O
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O1),
                data: NormData { m: 15.89, sd: 4.78 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O2),
                data: NormData { m: 24.05, sd: 4.78 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O3),
                data: NormData { m: 19.59, sd: 4.32 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O4),
                data: NormData { m: 15.02, sd: 3.88 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O5),
                data: NormData { m: 17.68, sd: 5.33 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O6),
                data: NormData { m: 20.68, sd: 3.76 },
            },
            // 维度 A
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A1),
                data: NormData { m: 21.84, sd: 4.43 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A2),
                data: NormData { m: 21.54, sd: 4.97 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A3),
                data: NormData { m: 21.19, sd: 3.88 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A4),
                data: NormData { m: 16.19, sd: 4.72 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A5),
                data: NormData { m: 17.68, sd: 4.62 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A6),
                data: NormData { m: 20.56, sd: 3.76 },
            },
            // 维度 C
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C1),
                data: NormData { m: 20.44, sd: 4.33 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C2),
                data: NormData { m: 19.32, sd: 4.75 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C3),
                data: NormData { m: 24.18, sd: 3.48 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C4),
                data: NormData { m: 19.69, sd: 4.07 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C5),
                data: NormData { m: 21.01, sd: 4.03 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C6),
                data: NormData { m: 20.43, sd: 4.66 },
            },
        ],
    },
};

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum Operator {
    G,
    GE,
    L,
    LE,
}

#[derive(Serialize)]
struct Comparison {
    expression: PlainText,
    operator: Operator,
}

#[derive(Serialize)]
struct Score {
    comparisons: &'static [Option<Comparison>; 2],
    value: u8,
}

#[derive(Serialize)]
struct ScoringRule {
    range: RangeInclusive<u8>,
    low: &'static [Score; 3],
    middle: &'static [Score; 3],
    high: &'static [Score; 4],
}

const SCORING_RULE: ScoringRule = ScoringRule {
    range: 1..=10,
    low: &[
        Score {
            value: 1,
            comparisons: &[
                None,
                Some(Comparison {
                    expression: "M-2*SD",
                    operator: Operator::L,
                }),
            ],
        },
        Score {
            value: 2,
            comparisons: &[
                Some(Comparison {
                    expression: "M-2*SD",
                    operator: Operator::GE,
                }),
                Some(Comparison {
                    expression: "M-1.5*SD",
                    operator: Operator::LE,
                }),
            ],
        },
        Score {
            value: 3,
            comparisons: &[
                Some(Comparison {
                    expression: "M-1.5*SD",
                    operator: Operator::G,
                }),
                Some(Comparison {
                    expression: "M-1*SD",
                    operator: Operator::LE,
                }),
            ],
        },
    ],
    middle: &[
        Score {
            value: 4,
            comparisons: &[
                Some(Comparison {
                    expression: "M-1*SD",
                    operator: Operator::G,
                }),
                Some(Comparison {
                    expression: "M-0.5*SD",
                    operator: Operator::LE,
                }),
            ],
        },
        Score {
            value: 5,
            comparisons: &[
                Some(Comparison {
                    expression: "M-0.5*SD",
                    operator: Operator::G,
                }),
                Some(Comparison {
                    expression: "M",
                    operator: Operator::LE,
                }),
            ],
        },
        Score {
            value: 6,
            comparisons: &[
                Some(Comparison {
                    expression: "M",
                    operator: Operator::G,
                }),
                Some(Comparison {
                    expression: "M+0.3*SD",
                    operator: Operator::LE,
                }),
            ],
        },
    ],
    high: &[
        Score {
            value: 7,
            comparisons: &[
                Some(Comparison {
                    expression: "M+0.3*SD",
                    operator: Operator::G,
                }),
                Some(Comparison {
                    expression: "M+0.6*SD",
                    operator: Operator::LE,
                }),
            ],
        },
        Score {
            value: 8,
            comparisons: &[
                Some(Comparison {
                    expression: "M+0.6*SD",
                    operator: Operator::G,
                }),
                Some(Comparison {
                    expression: "M+SD",
                    operator: Operator::LE,
                }),
            ],
        },
        Score {
            value: 9,
            comparisons: &[
                Some(Comparison {
                    expression: "M+SD",
                    operator: Operator::G,
                }),
                Some(Comparison {
                    expression: "M+1.5*SD",
                    operator: Operator::LE,
                }),
            ],
        },
        Score {
            value: 10,
            comparisons: &[
                Some(Comparison {
                    expression: "M+1.5*SD",
                    operator: Operator::G,
                }),
                None,
            ],
        },
    ],
};

#[derive(Serialize)]
pub struct Question {
    title: PlainText,
    options: &'static [QuestionOption; 5],
    dimension: Dimension,
    subdimension: Subdimension,
}

const FORWARD: &[QuestionOption; 5] = &[
    QuestionOption {
        text: "完全不符合",
        point: 0,
    },
    QuestionOption {
        text: "不太符合",
        point: 1,
    },
    QuestionOption {
        text: "一般般吧",
        point: 2,
    },
    QuestionOption {
        text: "比较符合",
        point: 3,
    },
    QuestionOption {
        text: "完全符合",
        point: 4,
    },
];

const REVERSE: &[QuestionOption; 5] = &[
    QuestionOption {
        text: "完全不符合",
        point: 4,
    },
    QuestionOption {
        text: "不太符合",
        point: 3,
    },
    QuestionOption {
        text: "一般般吧",
        point: 2,
    },
    QuestionOption {
        text: "比较符合",
        point: 1,
    },
    QuestionOption {
        text: "完全符合",
        point: 0,
    },
];

#[derive(Serialize)]
struct DimensionInterpretation {
    dimension: Dimension,
    name: PlainText,
    description: PlainText,
    low: PlainText,
    high: PlainText,
    subdimension_interpretations: &'static [SubdimensionInterpretation; 6],
}

#[derive(Serialize)]
struct SubdimensionInterpretation {
    dimension: Subdimension,
    name: PlainText,
    description: PlainText,
    low: PlainText,
    high: PlainText,
}

#[derive(Serialize)]
pub struct Interpretation {
    norm: Norm,
    scoring_rule: ScoringRule,
    dimensions: &'static [DimensionInterpretation; 5],
}

const INTERPRETATION: Interpretation = Interpretation {
    norm: NORM,
    scoring_rule: SCORING_RULE,
    dimensions: &[
        DimensionInterpretation {
            name: "神经质",
            dimension: Dimension::N,
            description: "神经质反映个体情感调节过程，反映个体体验消极情绪的倾向和情绪不稳定性。",
            low: "较少烦恼，较少情绪化，比较平静。",
            high:"倾向于有心理压力，不现实的想法、过多的要求和冲动，更容易体验到诸如愤怒、焦虑、抑郁等消极的情绪。对外界刺激反应比一般人强烈，对情绪的调节、应对能力比较差，经常处于一种不良的情绪状态下。并且思维、决策、以及有效应对外部压力的能力比较差。",
            subdimension_interpretations: &[
                SubdimensionInterpretation {
                    name: "焦虑",
                    dimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
                    description: "忧虑、恐惧、容易担忧、紧张、神经过敏。",
                    low: "心态平静，放松，不容易感到害怕，不会总是担心事情可能会出问题，情绪平静、放松、稳定。",
                    high: "焦虑，容易感觉到危险和威胁，容易紧张、恐惧、担忧、不安。",
                },
                SubdimensionInterpretation {
                    name: "愤怒和敌意",
                    dimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
                    description: "反映的是体验愤怒以及有关状态（如挫折、痛苦）的倾向，测量个体体验愤怒的容易程度。",
                    low: "不容易生气、发火，友好的、脾气随和，不易动怒。",
                    high: "容易发火，在感到自己受到不公正的待遇后会充满怨恨，暴躁的、愤怒的和受挫的。",
                },
                SubdimensionInterpretation {
                    name: "抑郁",
                    dimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
                    description: "测量正常个体在体验抑郁情感时的不同倾向。",
                    low: "不容易感到悲伤、很少被遗弃感。",
                    high: "绝望的、内疚的、郁闷的、沮丧的。容易感到悲伤、被遗弃、灰心丧气。容易感到内疚、悲伤、失望和孤独。他们容易受打击，经常情绪低落。",
                },
                SubdimensionInterpretation {
                    name: "自我意识",
                    dimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
                    description: " 核心部分是害羞和尴尬情绪体验",
                    low: "在社交场合镇定、自信，不容易感到紧张、害羞。",
                    high: "太关心别人如何看待自己，害怕别人嘲笑自己，在社交场合容易感到害羞、焦虑、自卑、易尴尬。",
                },
                SubdimensionInterpretation {
                    name: "冲动性",
                    dimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
                    description: "指个体对冲动和渴望的控制。",
                    low: "自我控制的、能抵挡诱惑的。",
                    high: "在感受到强烈的诱惑时，不容易抑制，容易追求短时的满足而不考虑长期的后果。不能抵抗渴望的、草率的、爱挖苦人的、自我中心的。",
                },
                SubdimensionInterpretation {
                    name: "脆弱性",
                    dimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
                    description: "指在遭受压力时的脆弱性。",
                    low: "压力下，感到平静、自信。适应力强的、头脑清醒的、勇敢的。",
                    high: "压力下，容易感到惊慌、混乱、无助，不能应付压力。",
                },
            ],
        },
        DimensionInterpretation {
            name: "外向性",
            dimension: Dimension::E,
            description: "外向性来表示人际互动的数量和密度、对刺激的需要以及获得愉悦的能力。这个维度将社会性的、主动的、个人定向的个体和沉默的、严肃的、腼腆的、安静的人作对比。这个方面可由两个品质加以衡量：人际的卷入水平和活力水平。前者评估个体喜欢他人陪伴的程度，而后者反映了个体个人的节奏和活力水平。",
            low: "比较安静，谨慎，不喜欢与外界过多接触。他们不喜欢与人接触不能被解释为害羞或者抑郁，这仅仅是因为比起外向的人，他们不需要那么多的刺激，因此喜欢一个人独处。内向人的这种特点有时会被人误认为是傲慢或者不友好，其实一旦和他接触你经常会发现他是一个非常和善的人。",
            high: "喜欢与人接触，充满活力，经常感受到积极的情绪。他们热情，喜欢运动，喜欢刺激冒险。在一个群体当中，他们非常健谈，自信，喜欢引起别人的注意。",
            subdimension_interpretations: &[
                SubdimensionInterpretation {
                    name: "热情",
                    dimension: Subdimension::E(ExtraversionSubdimension::E1),
                    description: "它和人际亲密问题最相关。",
                    low: "虽然并不意味着冷淡、不友好，但通常会被认为是对人疏远的。",
                    high: "热情的人喜欢周围的人，经常会向他们表达积极友好的情绪。他们善于交朋友，容易和别人形成亲密的关系。好交际的、健谈的、富有情感的。",
                },
                SubdimensionInterpretation {
                    name: "乐群性",
                    dimension: Subdimension::E(ExtraversionSubdimension::E2),
                    description: "指偏爱有他人的陪伴。",
                    low: "避免人群，感觉太闹。希望有更多的时间独处，有自己的个人空间。避开人群的、喜欢独处的。",
                    high: "喜欢与人相处，喜欢人多热闹的场合。开朗的、有许多朋友的、寻求社会联系的。",
                },
                SubdimensionInterpretation {
                    name: "独断性",
                    dimension: Subdimension::E(ExtraversionSubdimension::E3),
                    description: "指一个人在社交场合表现得自信、有主见和有影响力。",
                    low: "在人群中话很少，让别人处于主导支配地位。谦逊的、腼腆的、沉默寡言的。",
                    high: "喜欢在人群中处于支配地位，指挥别人，影响别人的行为。支配的、有说服力的自信的、果断的。",
                },
                SubdimensionInterpretation {
                    name: "活跃性",
                    dimension: Subdimension::E(ExtraversionSubdimension::E4),
                    description: "指一个人在社交和非社交环境中的活跃程度。",
                    low: "在生活工作中慢节奏，悠闲的，不着急的、缓慢的、从容不迫的。",
                    high: "在生活工作中快节奏，忙碌，显得充满精力，喜欢参与很多事情，精力充沛的、快节奏的、充满活力的。",
                },
                SubdimensionInterpretation {
                    name: "寻求刺激",
                    dimension: Subdimension::E(ExtraversionSubdimension::E5),
                    description: "指一个人对刺激事物的追求。",
                    low: "避免喧嚣和吵闹，讨厌冒险，谨慎的、沉静的，对刺激不感兴趣的。",
                    high: "在缺乏刺激的情况下容易感到厌烦，喜欢喧嚣吵闹，喜欢冒险，寻求刺激、浮华、寻求强烈刺激，喜欢冒险。",
                },
                SubdimensionInterpretation {
                    name: "积极情绪",
                    dimension: Subdimension::E(ExtraversionSubdimension::E6),
                    description: "表示体验积极情绪（如喜悦、快乐、爱和兴奋）的倾向。",
                    low: "不容易感受到各种积极的情绪，但并不意味着一定会感受到各种负面情绪。低分者只是不那么容易兴奋起来。不热情的、平静的、严肃的。",
                    high: "容易感受到各种积极的情绪，如快乐、乐观、兴奋等。快乐的情绪高涨的、愉悦的、乐观的。",
                },
            ],
        },
            DimensionInterpretation {
            name: "开放性",
            dimension: Dimension::O,
            description: "开放性描述一个人的认知风格。对经验的开放性被定义为：为了自身的缘故对经验的前摄（proactive）寻求理解，以及对陌生情境的容忍和探索。这个维度将那些好奇的、新颖的、非传统的以及有创造性的个体与那些传统的、无艺术兴趣的、无分析能力的个体做比较。",
            low: "封闭性的人讲求实际，偏爱常规，比较传统和保守。",
            high: "开放性的人偏爱抽象思维，兴趣广泛。",
            subdimension_interpretations: &[
                SubdimensionInterpretation {
                    name: "想象力",
                    dimension: Subdimension::O(OpenMindednessSubdimension::O1),
                    description: "对想象开放的人有生动的想象和活跃的幻想生活。",
                    low: "理性的、现实的，实干的，更喜欢现实思考的。",
                    high: "对于他们来说，现实世界太平淡了。喜欢充满幻想，创造一个更有趣、丰富的世界。想象力丰富的、爱做白日梦的。",
                },
                SubdimensionInterpretation {
                    name: "审美",
                    dimension: Subdimension::O(OpenMindednessSubdimension::O2),
                    description: "指一个人对美感的敏感度和欣赏力。",
                    low: "对美缺乏敏感性，对艺术不感兴趣。对艺术不敏感的、不理解的。",
                    high: "欣赏自然和艺术中的美。重视审美经历的、能为艺术和美所感动的。",
                },
                SubdimensionInterpretation {
                    name: "情感丰富性",
                    dimension: Subdimension::O(OpenMindednessSubdimension::O3),
                    description: "指一个人的情感体验丰富程度。",
                    low: "较少感知到自己的情感和内心世界，也不愿意坦率地表达出来。情绪范围窄、对环境不敏感。",
                    high: "容易感知自己的情绪和内心世界。敏感的、移情的、重视自己感受的。",
                },
                SubdimensionInterpretation {
                    name: "尝新",
                    dimension: Subdimension::O(OpenMindednessSubdimension::O4),
                    description: "对新鲜事物的追求。",
                    low: "对不熟悉的事物感到有些不舒服，喜欢熟悉的环境和人。生活方式固定、喜欢熟悉的事物。",
                    high: "喜欢接触新的事物，去外面旅行、体验不同的经验。感觉千篇一律令人乏味，愿意去尝试新的事物。寻求新异和多样性、尝试新的活动。",
                },
                SubdimensionInterpretation {
                    name: "思辨",
                    dimension: Subdimension::O(OpenMindednessSubdimension::O5),
                    description: "为了他们自身的缘故而积极追求理智上的兴趣，思路开阔、愿意思考新的、非常规的观点。",
                    low: "事务的、事实定向的、不欣赏思想挑战的。",
                    high: "有求知欲的、善于分析的、理论定向的。",
                },
                SubdimensionInterpretation {
                    name: "价值观",
                    dimension: Subdimension::O(OpenMindednessSubdimension::O6),
                    description: "对权威和常规的接受度",
                    low: "喜欢遵循权威和常规带来的稳定和安全感，不会去挑战现有秩序和权威。教条的、保守的、顺从的。",
                    high: "喜欢挑战权威、常规和传统观念。在极端状态下，他们会表现出对现存规则的敌意，同情那些打破现存法律的人，喜欢混乱、冲突和无序的状态。能容忍的、宽宏大量的、不顺从的。",
                },
            ],
        },
            DimensionInterpretation {
            name: "宜人性",
            dimension: Dimension::A,
            description: "个体对其他人所持的态度，这些态度一方面包括亲近人的、有同情心的、信任他人的、宽大的、心软的，另一方面包括敌对的、愤世嫉俗的、爱摆布人的、复仇心重的、无情的。这里所说的广义的人际定向范围。宜人性代表了“爱”，对合作和人际和谐是否看重。",
            low: "把自己的利益放在别人的利益之上。本质上，他们不关心别人的利益，因此也不乐意去帮助别人。有时候，他们对别人是非常多疑的，怀疑别人的动机。",
            high: "善解人意的、友好的、慷慨大方的、乐于助人的，愿意为了别人放弃自己的利益。",
            subdimension_interpretations: &[
                SubdimensionInterpretation {
                    name: "信任",
                    dimension: Subdimension::A(AgreeablenessSubdimension::A1),
                    description: "一个人对他人的信任度。",
                    low: "认为别人是自私、危险、想占自己便宜。谨慎的、悲观的、猜忌的、铁石心肠的。",
                    high: "相信别人是诚实、可信和有良好动机的。宽恕的、信任他人的、平和的。",
                },
                SubdimensionInterpretation {
                    name: "坦诚",
                    dimension: Subdimension::A(AgreeablenessSubdimension::A2),
                    description: "指一个人坦诚、真诚和直接的程度。",
                    low: "在与人交往时往往会掩饰自己，防卫心理较重，不愿意向别人露出自己的底牌。精明的、机敏的（clever）、献媚的。",
                    high: "认为在与人交往时没有必要去掩饰，显得坦率、真诚。直接的、坦率的、坦白的、老实的。",
                },
                SubdimensionInterpretation {
                    name: "利他",
                    dimension: Subdimension::A(AgreeablenessSubdimension::A3),
                    description: "指一个人愿意帮助他人的程度。",
                    low: "不愿意帮助别人，感觉帮助别人是一种负担。自私的、愤世嫉俗的、冷酷的、势利的。",
                    high: "愿意帮助别人，感觉帮助别人是一种乐趣。热心的、心软的、温和的、慷慨的、好心的。",
                },
                SubdimensionInterpretation {
                    name: "顺从",
                    dimension: Subdimension::A(AgreeablenessSubdimension::A4),
                    description: "指一个人乐于服从他人和遵守规则。",
                    low: "不介意与人发生冲突，会为了达到自己的目的去威胁别人。倔强的、有过分要求的、刚愎自用的、铁石心肠的。",
                    high: "不喜欢与人发生冲突，为了与人相处，愿意放弃自己的立场或者否定自己的需要。恭顺的、有求必应的、好心的。",
                },
                SubdimensionInterpretation {
                    name: "谦逊",
                    dimension: Subdimension::A(AgreeablenessSubdimension::A5),
                    description: "指一个人不夸耀自己、不张扬的程度。",
                    low: "攻击的、傲慢的、爱炫耀的、粗暴的。",
                    high: "谦逊的、不摆架子的。",
                },
                SubdimensionInterpretation {
                    name: "同理心",
                    dimension: Subdimension::A(AgreeablenessSubdimension::A6),
                    description: "对他人的同情心和关心的态度。",
                    low: "对别人的痛苦没有强烈的感受，为自己的客观而感到自豪，更关心真实、公平而不是仁慈。心胸狭窄的、冷酷的、固执己见的、势利的。",
                    high: "富有同情心，容易感受到别人的悲伤，表示同情。友好的、热心的、温和的、心软的。",
                },
            ],
        },
            DimensionInterpretation {
            name: "尽责性/责任心",
            dimension: Dimension::C,
            description: "指我们控制、管理和调节自身冲动的方式，评估个体在目标导向行为上的组织、坚持和动机。它把可信赖的、讲究的个体和懒散的、马虎的个体作比较。同时反映个体自我控制的程度以及推迟需求满足的能力。",
            low: "通常比较随意、懒散、不守时。",
            high:"通常有条理、有组织、有责任心。",
            subdimension_interpretations: &[
                SubdimensionInterpretation {
                    name: "能力",
                    dimension: Subdimension::C(ConscientiousnessSubdimension::C1),
                    description: "表示对某人有能力的、明智的、深谋远虑的、高效的感觉。",
                    low: "对自己的能力不自信，不相信自己可以控制自己的工作和生活。困惑的、健忘的、愚蠢的。",
                    high: "对自己的能力自信的。高效的、一丝不苟的自信的聪明的。",
                },
                SubdimensionInterpretation {
                    name: "条理性",
                    dimension: Subdimension::C(ConscientiousnessSubdimension::C2),
                    description: "指一个人有条理、有组织和有计划的能力。",
                    low: "没有计划性和条理性，显得杂乱无章。无序的、易冲动的、粗心的。",
                    high: "具有良好的条理性，喜欢制定计划，并按规则办事。精确的、高效的、有条不紊的。",
                },
                SubdimensionInterpretation {
                    name: "责任感",
                    dimension: Subdimension::C(ConscientiousnessSubdimension::C3),
                    description: "从某方面来说，责任心意味着受良心的支配，而那是由尽责这一子维度来评估的。",
                    low: "感觉规矩、条例是一种约束。经常被别人看作是不可靠、不负责任的。懒散的、漫不经心的、不专心的。",
                    high: "有责任感，按规矩办事。可信赖的、有礼貌的、有组织的、一丝不苟的。",
                },
                SubdimensionInterpretation {
                    name: "追求成就",
                    dimension: Subdimension::C(ConscientiousnessSubdimension::C4),
                    description: "指一个人对成功的渴望和追求。",
                    low: "满足于完成基本的工作，被别人看作是懒惰的。悠闲的、爱空想的、无组织的。",
                    high: "追求成功和卓越，通常有目标感的，甚至会被别人当作工作狂。有抱负的、勤奋的、富有进取心的、坚忍不拔的。",
                },
                SubdimensionInterpretation {
                    name: "自律",
                    dimension: Subdimension::C(ConscientiousnessSubdimension::C5),
                    description: "指即使枯燥乏味或有其它干扰，也能执行任务并将其完成。",
                    low: "做事拖延，经常半途而废，遇到困难容易退缩。没有抱负的、健忘的、心不在焉的。",
                    high: "尽力完成工作和任务，克服困难，专著于自己的任务。有组织的、一丝不苟的、精力充沛的、能干的、高效的。",
                },
                SubdimensionInterpretation {
                    name: "审慎",
                    dimension: Subdimension::C(ConscientiousnessSubdimension::C6),
                    description: "行动前是否仔细考虑的倾向。",
                    low: "没有考虑后果，冲动，想到什么做什么。不成熟的、草率的、冲动的、粗心的。",
                    high: "三思而后行，不冲动。谨慎的、有逻辑性的、成熟的。",
                },
            ],
        },
    ],
};

const QUESTIONS: &[Question] = &[
    Question {
        title: "我不是一个很容易忧虑的人",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
    },
    Question {
        title: "我容易受到惊吓",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
    },
    Question {
        title: "我很少感到恐惧或焦虑",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
    },
    Question {
        title: "我常感到紧张和心神不定",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
    },
    Question {
        title: "我很少为将来的事情担忧",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
    },
    Question {
        title: "我常常担心事情可能会出错",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
    },
    Question {
        title: "我所惧怕的事较大部分人要少",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
    },
    Question {
        title: "有时候我的脑中会出现一些可怕的念头",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
    },
    Question {
        title: "我常因为别人对我做法而感到愤怒",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
    },
    Question {
        title: "我是一个性情平和的人",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
    },
    Question {
        title: "我容易激动，性情急躁",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
    },
    Question {
        title: "别人并不认为我是一个易怒或易发脾气的人",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
    },
    Question {
        title: "我常常讨厌那些我不得不与之打交道的人",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
    },
    Question {
        title: "我一般很难发怒",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
    },
    Question {
        title: "我有时感到怨恨",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
    },
    Question {
        title: "即使是小小的烦恼事也会使我感到受挫",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
    },
    Question {
        title: "我很少感觉孤独和忧郁",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
    },
    Question {
        title: "有时我感到自己毫无价值",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
    },
    Question {
        title: "我很少感到忧郁或沮丧",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
    },
    Question {
        title: "我有时会深感内疚或有一种负罪感",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
    },
    Question {
        title: "当事情出问题时，我喜欢责径自己",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
    },
    Question {
        title: "我对自己的评价很低",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
    },
    Question {
        title: "有时候我感到事情暗淡无望",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
    },
    Question {
        title: "我经常在事情不顺利时感到挫折并想因此而放弃",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
    },
    Question {
        title: "与别人交往时，我时常害怕出一些丢人现眼或笨拙的差错",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
    },
    Question {
        title: "我在社交场合很少感到不自在",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
    },
    Question {
        title: "有好几次我羞愧得只想躲起来",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
    },
    Question {
        title: "当别人嘲笑我和开我的玩笑时，我不会感到太难堪",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
    },
    Question {
        title: "我经常感到自己不如别人",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
    },
    Question {
        title: "在上司与权威面前，我不会感到不自在",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
    },
    Question {
        title: "如果我对别人说错了话或做错了事，我感到难以再面对他们",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
    },
    Question {
        title: "当我所认识的人做了傻事时，我会替他们难为情",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
    },
    Question {
        title: "我极少过多沉溺于任何事情",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
    },
    Question {
        title: "我感到难以克制我对某些事情的欲望",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
    },
    Question {
        title: "我对于抵抗诱惑并没有什么困难",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
    },
    Question {
        title: "当有我爱吃的食物时，我会吃得太多",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
    },
    Question {
        title: "我很少凭冲动行事",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
    },
    Question {
        title: "我有时会因吃得过多而弄得肠胃出毛病",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
    },
    Question {
        title: "有时我会因一时冲动而做一些令我后悔的事情",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
    },
    Question {
        title: "我向来能够控制自己的情绪",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
    },
    Question {
        title: "我经常感到无能为力",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
    },
    Question {
        title: "我觉得我有能力应付我所遇到的大部分问题",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
    },
    Question {
        title: "我在大量压力的情况下，有时感到精神象是要崩溃一样",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
    },
    Question {
        title: "我在紧急情况下仍能保持头脑冷静",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
    },
    Question {
        title: "我常常觉得难以拿定主意",
        options: FORWARD,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
    },
    Question {
        title: "在遇到危机时，我觉得我能很好地控制自己",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
    },
    Question {
        title: "当所有的事情好象都一团糟时，我仍能作出正确的决定",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
    },
    Question {
        title: "我的情绪相当稳定",
        options: REVERSE,
        dimension: Dimension::N,
        subdimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
    },
    Question {
        title: "我的确喜欢我所遇到的大部分人",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E1),
    },
    Question {
        title: "与别人聊天并不会使我得到许多的乐趣",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E1),
    },
    Question {
        title: "别人认为我是一个热情和友好的人",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E1),
    },
    Question {
        title: "我的确有些冷淡，并与别人保持距离",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E1),
    },
    Question {
        title: "我确实喜欢与别人交谈",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E1),
    },
    Question {
        title: "我感到对陌生人微笑和与之交往是一件很容易的事情",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E1),
    },
    Question {
        title: "我对朋友有深厚的感情依恋",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E1),
    },
    Question {
        title: "我时常关心与我一块工作的人们的生活情况",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E1),
    },
    Question {
        title: "我尽量避开人群",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E2),
    },
    Question {
        title: "我喜欢很多人在我身边",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E2),
    },
    Question {
        title: "我通常喜欢单独工作",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E2),
    },
    Question {
        title: "当我独处一段时间后，就会真正感到需要别人",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E2),
    },
    Question {
        title: "我喜欢做那种可由我单独干而不受别人干扰的工作",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E2),
    },
    Question {
        title: "如果要我选择，我更愿意去人多的海滩而不愿去森林中单独的小屋中度假",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E2),
    },
    Question {
        title: "我对大家聚到一起的场合感到乏味",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E2),
    },
    Question {
        title: "我喜欢人多的聚会",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E2),
    },
    Question {
        title: "我具有领导才干，喜欢表达和坚持实行自己的主张",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E3),
    },
    Question {
        title: "我有时没有尽我所能地坚持自己的权利",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E3),
    },
    Question {
        title: "我常常在我所在的团体中担负领导责任",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E3),
    },
    Question {
        title: "开会时，我通常让别人发言而自己较少开口",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E3),
    },
    Question {
        title: "碰到要作决定时，别人常来找我商量",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E3),
    },
    Question {
        title: "我宁愿我行我素也不愿成为别人的领导",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E3),
    },
    Question {
        title: "与别人谈话时，大部分时间都是我在讲话",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E3),
    },
    Question {
        title: "我觉得要控制一种局面并非易事",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E3),
    },
    Question {
        title: "我更喜欢从事慢节奏的工作和娱乐活动",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E4),
    },
    Question {
        title: "我做事情时感到精力充沛",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E4),
    },
    Question {
        title: "我做事较慢但稳重",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E4),
    },
    Question {
        title: "我常常感到精力旺盛",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E4),
    },
    Question {
        title: "我并不象其他人那样敏捷和活跃",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E4),
    },
    Question {
        title: "我似乎总是匆匆忙忙",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E4),
    },
    Question {
        title: "我的生活节奏很快",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E4),
    },
    Question {
        title: "我是一个很活跃的人",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E4),
    },
    Question {
        title: "我常常寻求刺激",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E5),
    },
    Question {
        title: "我不会喜欢去一个热闹喧嚣的大都市度假",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E5),
    },
    Question {
        title: "有时，我做某些事情只是为了寻求刺激和冒险",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E5),
    },
    Question {
        title: "我不愿意看那引起令人震惊或恐怖的电影",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E5),
    },
    Question {
        title: "我喜欢去热闹的场合",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E5),
    },
    Question {
        title: "我喜欢“过山车”的刺激",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E5),
    },
    Question {
        title: "鲜艳的色彩和华丽的款式对我具有吸引力",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E5),
    },
    Question {
        title: "喜欢成为从事某种体育或健身项目群体中的一员",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E5),
    },
    Question {
        title: "我从没有真正因为开心而跳起来过",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E6),
    },
    Question {
        title: "我有时会感到极其快乐和欣喜若狂",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E6),
    },
    Question {
        title: "我不是一个乐观主义者",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E6),
    },
    Question {
        title: "我有时会充满快乐",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E6),
    },
    Question {
        title: "我并不觉得自己特别轻松愉快和乐观开朗",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E6),
    },
    Question {
        title: "我是一个快乐的、生气勃勃的人",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E6),
    },
    Question {
        title: "我很少用“绝了”或“好极了”这类的字眼来表达我的体验",
        options: REVERSE,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E6),
    },
    Question {
        title: "我容易发笑",
        options: FORWARD,
        dimension: Dimension::E,
        subdimension: Subdimension::E(ExtraversionSubdimension::E6),
    },
    Question {
        title: "我的想象力相当丰富",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O1),
    },
    Question {
        title: "我尽量使自己的思想合乎现实而避免幻想",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O1),
    },
    Question {
        title: "我常喜欢幻想",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O1),
    },
    Question {
        title: "我不喜欢将时间浪费在白日梦中",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O1),
    },
    Question {
        title: "我喜欢沉浸在幻想及想入非非之中，并探寻它们所有的可能性，任其发展",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O1),
    },
    Question {
        title: "当我发现自己在想入非非时，通常会使自己变得忙碌或开始将注意力集中在某些工作或活动上",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O1),
    },
    Question {
        title: "小时候很少乐于玩“过家家”的游戏",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O1),
    },
    Question {
        title: "我来说，让思想无拘无束地漫游是一件很困难的事",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O1),
    },
    Question {
        title: "我十分注意事物的审美和艺术方面的特点",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O2),
    },
    Question {
        title: "当我听音乐时，我有时会完全沉浸在其中",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O2),
    },
    Question {
        title: "看芭蕾舞或现代舞蹈使我感到乏味",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O2),
    },
    Question {
        title: "我深感艺术和自然所呈现出的各种格调和姿态奇妙有趣",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O2),
    },
    Question {
        title: "我很少或不会因诗词而产生什么感触",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O2),
    },
    Question {
        title: "有些音乐总是使我那样着迷",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O2),
    },
    Question {
        title: "当我阅读一首诗或观看一件艺术品时，我有时会感到兴奋或惊喜",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O2),
    },
    Question {
        title: "与故事性的诗歌相比，我更喜欢那些强调情感与意象的诗歌",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O2),
    },
    Question {
        title: "我宁愿过变动起伏而能获得各种情感体验的生活，而不愿过那种安逸稳定而平淡的生活",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O3),
    },
    Question {
        title: "我很少有强烈的情绪体验",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O3),
    },
    Question {
        title: "与物质生活本身相比，我更注重生活的情调与情趣",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O3),
    },
    Question {
        title: "我很少会留意即刻的情绪和感受",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O3),
    },
    Question {
        title: "我体验到许多不同的情绪和感受",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O3),
    },
    Question {
        title: "我很少注意不同处境对我的心境或感受的影响",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O3),
    },
    Question {
        title: "我很容易为别人设身处地着想",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O3),
    },
    Question {
        title: "奇特的事情——如某些气味或远方的地名——能激起我很强的感受",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O3),
    },
    Question {
        title: "我做事总有一定的方式，我不喜欢改变我的做事习惯",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O4),
    },
    Question {
        title: "我觉得学习和培养新爱好是很有趣的",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O4),
    },
    Question {
        title: "一旦我找到了做事的适当方法，我便会一直照这种方法做下去",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O4),
    },
    Question {
        title: "我经常品尝我没吃过的和异地风味的食物",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O4),
    },
    Question {
        title: "我更愿意在我熟悉的环境中生活",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O4),
    },
    Question {
        title: "我似乎总是为了得到新的感觉而改变一下屋子里的环境",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O4),
    },
    Question {
        title: "与一个从未去过的地方相比，我更愿去一个我曾经去过并熟悉的地方度假",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O4),
    },
    Question {
        title: "我会沿着惯常的路去某一个地方",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O4),
    },
    Question {
        title: "我对理论和抽象的观念很感兴趣",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O5),
    },
    Question {
        title: "我觉得哲学性的争论很乏味",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O5),
    },
    Question {
        title: "我喜欢解决问题或猜谜",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O5),
    },
    Question {
        title: "当别人谈论抽象的、理论性的事物时，有时我会感到乏味",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O5),
    },
    Question {
        title: "我喜欢玩需要动脑筋的游戏或解决某种难题",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O5),
    },
    Question {
        title: "我没有兴趣思索宇宙的本质或人类的现状",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O5),
    },
    Question {
        title: "我有很强的求知欲望",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O5),
    },
    Question {
        title: "我对脑力活动或知识方面的事情有广泛的兴趣",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O5),
    },
    Question {
        title: "我认让学生听具有争论性的演讲只会混淆和误导他们的思想",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O6),
    },
    Question {
        title: "我认为法律与社会政策应该及时修改以便反映变化着的世界之所需",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O6),
    },
    Question {
        title: "我认为在道德问题上做决定时，我们应该遵从宗教权威的判断",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O6),
    },
    Question {
        title: "其它社会的人可能有与我们不同的是非观念，但对他们来说这些观念可能是正确的",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O6),
    },
    Question {
        title: "我认为忠实于自己的想法和原则比广泛听取意见更重要",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O6),
    },
    Question {
        title: "我觉得我的胸襟宽阔，并能容忍别人的生活方式",
        options: FORWARD,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O6),
    },
    Question {
        title: "我认为一个人如果到了25岁仍不知道自己有什么信念，他们准是有些问题",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O6),
    },
    Question {
        title:
            "认为以严格的纪律使小孩形成遵守规矩的品质比让他们自己形成独立的判断和行事的能力更为重要",
        options: REVERSE,
        dimension: Dimension::O,
        subdimension: Subdimension::O(OpenMindednessSubdimension::O6),
    },
    Question {
        title: "人们表面上说得漂亮，内心另有他谋",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A1),
    },
    Question {
        title: "我觉得大部分人基本上是心怀善意的",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A1),
    },
    Question {
        title: "我认为如果你任别人怎样对待你，许多人便会趁机利用你",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A1),
    },
    Question {
        title: "我认为大部分与我交往的人是诚实可信的",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A1),
    },
    Question {
        title: "当别人认为我做一些好事时，我会怀疑他们是否出于真心",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A1),
    },
    Question {
        title: "首先总是对人们抱以信任的态度",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A1),
    },
    Question {
        title: "往往把别人设想得尽可能好",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A1),
    },
    Question {
        title:
            "尽管人类社会存在着一些阴暗的东西（如战争、罪恶、欺骗），我仍然相信人性总的来说是善良的",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A1),
    },
    Question {
        title: "我并不狡猾",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A2),
    },
    Question {
        title: "如果有必要，我愿意操纵别人以得到我所想得到的",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A2),
    },
    Question {
        title: "即使是我想欺骗别人，我也做不出来",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A2),
    },
    Question {
        title: "绝对诚实是一种很差的做生意的方法",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A2),
    },
    Question {
        title: "我会特别不愿意被别人认为是一个虚伪的人",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A2),
    },
    Question {
        title: "有时我会哄骗别人做些我想要他们做的事",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A2),
    },
    Question {
        title: "我有时会恐吓或哄着别人去做我想要他们做的事",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A2),
    },
    Question {
        title: "我为自己与人打交道时精于人情世故而感到得意",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A2),
    },
    Question {
        title: "有时候，我的一些做法使别人认为我太顾自己或自我中心",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A3),
    },
    Question {
        title: "我尽量对我所碰到的每个人都很有礼貌",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A3),
    },
    Question {
        title: "我有些冷淡而有心计",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A3),
    },
    Question {
        title: "我通常尽可能深思熟虑",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A3),
    },
    Question {
        title: "别人并不觉得我慷慨",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A3),
    },
    Question {
        title: "我所认识的大部分人都喜欢我",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A3),
    },
    Question {
        title: "我认为我是一个乐善好施的人",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A3),
    },
    Question {
        title: "我常将自己的事暂搁一旁而去帮别人",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A3),
    },
    Question {
        title: "我情愿与别人合作而不愿与他们竞争",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A4),
    },
    Question {
        title: "在有必要时，我会说出一些讽刺挖苦和尖刻的话来",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A4),
    },
    Question {
        title: "即使是在理当发怒的情况下，我也会为表达我的愤怒而犹豫",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A4),
    },
    Question {
        title: "如果我不喜欢某人，我便会让对方知道",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A4),
    },
    Question {
        title: "当我被别人侮辱时，我尽量宽恕和忘记",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A4),
    },
    Question {
        title: "如果有人先打我，我会立即还击",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A4),
    },
    Question {
        title: "我有些固执，脾气较倔",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A4),
    },
    Question {
        title: "我经常与家人及同事争论",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A4),
    },
    Question {
        title: "我不会羞于夸耀自己的天分和成就",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A5),
    },
    Question {
        title: "我不喜欢谈论我自己以及我的成就",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A5),
    },
    Question {
        title: "我知道我比大多数人优秀",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A5),
    },
    Question {
        title: "我尽量谦卑恭顺",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A5),
    },
    Question {
        title: "我对自己有很高的评价",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A5),
    },
    Question {
        title: "我并没有那种比别人要优越的感觉，不管他们的条件怎么样",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A5),
    },
    Question {
        title: "较之被别人称赞，我更愿意称赞别人",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A5),
    },
    Question {
        title: "我是一个优越的人",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A5),
    },
    Question {
        title: "领导们在制定政策时，应更多地考虑民众的生活",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A6),
    },
    Question {
        title: "我头脑现实，不易动感情",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A6),
    },
    Question {
        title: "为了帮助穷人和老年人，我们怎样做也不为多",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A6),
    },
    Question {
        title: "我并不同情行乞的人",
        options: REVERSE,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A6),
    },
    Question {
        title: "在人性需要和经济因素方面，应优先考虑前者",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A6),
    },
    Question {
        title: "我认为整个人类的成员都值得尊重",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A6),
    },
    Question {
        title: "我对不如我幸运的那些人感到同情",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A6),
    },
    Question {
        title: "与“公正”相比，我更愿意别人觉得我“仁慈”",
        options: FORWARD,
        dimension: Dimension::A,
        subdimension: Subdimension::A(AgreeablenessSubdimension::A6),
    },
    Question {
        title: "以办事情慎重稳妥和合乎常理而为人所知",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C1),
    },
    Question {
        title: "我觉得与周围人比，我的能力不够强",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C1),
    },
    Question {
        title: "我尽量掌握新信息，并通常能做出明智的决定",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C1),
    },
    Question {
        title: "我常常遇到一些我没有充分准备的情況",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C1),
    },
    Question {
        title: "我为自己对事情有恰当的判断而感到自豪",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C1),
    },
    Question {
        title: "我好象什么事情也没有完全干成过",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C1),
    },
    Question {
        title: "我是一个很能干的人",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C1),
    },
    Question {
        title: "我做事有效果，也有效率",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C1),
    },
    Question {
        title: "我宁愿暂时不做决定以便留有选择余地，而不愿提前计划好一切",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C2),
    },
    Question {
        title: "我通常保持我的物件干净、整洁",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C2),
    },
    Question {
        title: "我不是一个做事情有条不紊的人",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C2),
    },
    Question {
        title: "我喜欢把东西放在固定的位置，以便我确知它们在哪里",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C2),
    },
    Question {
        title: "我好象总不能把事情安排得井井有条",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C2),
    },
    Question {
        title: "我有些挑剔和求全",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C2),
    },
    Question {
        title: "我并不是非得要把东西弄得特别干净",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C2),
    },
    Question {
        title: "我花许多时间寻找我放错了地方的东西",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C2),
    },
    Question {
        title: "我尽心尽力地执行一切分派给我的任务",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C3),
    },
    Question {
        title: "我有时没有象我应该的那样可靠和值得信赖",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C3),
    },
    Question {
        title: "我通常会很快地如数还清我的债务",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C3),
    },
    Question {
        title: "当责难临头时，我有时除了扯谎以免被罚外别无他法",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C3),
    },
    Question {
        title: "当我做出承诺后，我总是能够照着去做",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C3),
    },
    Question {
        title: "我严格遵循我的道德准则",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C3),
    },
    Question {
        title: "我做事尽量认真细致，以免重做",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C3),
    },
    Question {
        title: "除非我真的病了，我才会请一天假",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C3),
    },
    Question {
        title: "我有些随和与松懈",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C4),
    },
    Question {
        title: "我有明确的目标，并能有条不紊地朝向它而工作",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C4),
    },
    Question {
        title: "当我开始一项新自我改进计划后，我总是半途而废",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C4),
    },
    Question {
        title: "我努力工作以达到我的目标",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C4),
    },
    Question {
        title: "我并不觉得我在极力驱使自己干出一番事业",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C4),
    },
    Question {
        title: "我努力完成一切我所能做到的事情",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C4),
    },
    Question {
        title: "我尽量将我所干的一切事情都干得出色",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C4),
    },
    Question {
        title: "我象一个“工作狂”",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C4),
    },
    Question {
        title: "我善于计划时间安排要做的事情，以至能够及时完成工作",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C5),
    },
    Question {
        title: "我在安顿下来工作之前要浪费很多时间",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C5),
    },
    Question {
        title: "我是一个总是能够将事情办妥和有工作成效的人",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C5),
    },
    Question {
        title: "我很难让自己做我应该做的事",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C5),
    },
    Question {
        title: "一旦我开始一项工作，我差不多总是会将它干完",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C5),
    },
    Question {
        title: "一项工作太难时，我宁愿开始一项新的",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C5),
    },
    Question {
        title: "太多的小事情需要去做，有时我一概置之不理",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C5),
    },
    Question {
        title: "我很有自律性",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C5),
    },
    Question {
        title: "在过去的日子里，我做过一些很愚蠢的事",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C6),
    },
    Question {
        title: "我做出一个决定之前要进行周全的考虑",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C6),
    },
    Question {
        title: "有时，我是先干了再想",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C6),
    },
    Question {
        title: "我在采取行动前往往要考虑后果",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C6),
    },
    Question {
        title: "我经常凭一时冲动做事",
        options: REVERSE,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C6),
    },
    Question {
        title: "我很少做出仓促的决定",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C6),
    },
    Question {
        title: "旅行之前我要仔细做好计划",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C6),
    },
    Question {
        title: "回答问题之前，我要认真思考",
        options: FORWARD,
        dimension: Dimension::C,
        subdimension: Subdimension::C(ConscientiousnessSubdimension::C6),
    },
];

const INTRODUCTION: Texts = &[
    &[SentenceItem::Plain("心理学者研究发现大约有五种特质可以涵盖人格描述的所有方面，即大五人格，也被称之为人格的海洋。")],
    &[
        SentenceItem::Plain("大五人格问卷多用于正常人群，特别是应用于"),
        SentenceItem::HTMLElement(HTMLElement::Strong("工作绩效领域")),
        SentenceItem::Plain("。一般要求测试者具有"),
        SentenceItem::HTMLElement(HTMLElement::Strong("初中三年级以上")),
        SentenceItem::Plain("文化水平。"),
    ],
    &[SentenceItem::Plain("大五人格是西方心理学界公认的一个人格特质模型，在多年的研究中被不断验证。")],
    &[SentenceItem::Plain("大五人格测试有多个版本，本测试为句子式的 NEO-PI-R。")],
];

const INSTRUCTION: Texts = &[
    &[
        SentenceItem::Plain("本测试有 "),
        SentenceItem::HTMLElement(HTMLElement::Strong("240")),
        SentenceItem::Plain(" 题，由于题目较多，在开始测试前请保证您有充足的时间完成此测试。"),
    ],
    &[
        SentenceItem::Plain("测试的过程中要保持"),
        SentenceItem::HTMLElement(HTMLElement::Strong("环境的安静和舒适")),
        SentenceItem::Plain("，有"),
        SentenceItem::HTMLElement(HTMLElement::Strong("适当的照明和温度")),
        SentenceItem::Plain("，"),
        SentenceItem::HTMLElement(HTMLElement::Strong("排除任何可能的干扰")),
        SentenceItem::Plain("（比如有人旁观您的回答情况或者有人在室内走动等）。"),
    ],
    &[
        SentenceItem::Plain("您应以"),
        SentenceItem::HTMLElement(HTMLElement::Strong("直觉性的反应")),
        SentenceItem::Plain("依题作答，"),
        SentenceItem::HTMLElement(HTMLElement::Strong("无须迟疑不决，拖延时间")),
        SentenceItem::Plain("。"),
    ],
];

pub const NEO_PERSONALITY_INVENTORY_REVISED: Scale<Interpretation, Question> = Scale {
    name: "大五人格测试",
    abbreviation: "NEO-PI-R",
    primary_category: ScaleCategory::Personality,
    related_categories: Some(&[
        ScaleCategory::Emotion,
        ScaleCategory::MentalHealth,
        ScaleCategory::Interpersonal,
        ScaleCategory::CareerAndAcademics,
        ScaleCategory::Wellbeing,
    ]),
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: Some(&[
        "人格理论有几种，大五人格特质理论是近年来最受重用和欢迎的一个，也是最简单实用的一个，它将人格归为5种特质，这5种特质一般翻译为：外倾性、宜人性、严谨性、神经质和开放性。",
        "有学者称大五人格特质理论为“人格心理学领域的一场静悄悄的革命”，并认为大五特质理论是适合全人类的，它在过去的半个多世纪里被心理学研究者证明具有跨语言、跨文化、跨情景、跨评定者的一致性和稳定性。",
    ]),
    references: None,
    warning: Some("适用于初中三年级以上文化水平的人群。"),
    formula_mode: None,
    tags: Tag{ info: Some(&["人格"]), normal: None, warning: Some(&["初三+"]), error: None },
    interpretation: INTERPRETATION,
    questions: QUESTIONS,
};
