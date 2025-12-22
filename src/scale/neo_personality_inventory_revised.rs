use std::ops::RangeInclusive;

use serde::Serialize;

use crate::scale::ScaleCategory;

use super::{HTMLElement, PlainText, QuestionOption, Scale, SentenceItem, Tag, Texts};

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// 维度
enum Dimension {
    /// 神经质/情绪稳定性，Neuroticism (Negative Emotionality)
    N,
    /// 外向性，Extraversion
    E,
    /// 开放性，Openness to Experience (Intellect/Imagination)
    O,
    /// 宜人性，Agreeableness
    A,
    /// 尽责性，Conscientiousness
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
    terminology: PlainText,
    description: PlainText,
    analysis: &'static [PlainText; 10],
    subdimension_interpretations: &'static [SubdimensionInterpretation; 6],
}

#[derive(Serialize)]
struct SubdimensionInterpretation {
    dimension: Subdimension,
    name: PlainText,
    description: PlainText,
    analysis: &'static [PlainText; 10],
}

#[derive(Serialize)]
pub struct Interpretation {
    norm: Norm,
    scoring_rule: ScoringRule,
    dimensions: &'static [DimensionInterpretation; 5],
}

// ===================== 神经质（N）子维度解释 =====================
const N_SUBDIMENSIONS: [SubdimensionInterpretation; 6] = [
    // N1 焦虑
    SubdimensionInterpretation {
        dimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
        name: "焦虑",
        description: "对未来、未知或负面事件的担忧程度",
        analysis: &[
            "几乎从不焦虑，即使面临重大事件也异常镇定",
            "很少感到焦虑，能够平静应对大多数压力情境",
            "偶尔会焦虑，但能很快平复，不常胡思乱想",
            "适度焦虑，在重要事情前会担心但能控制",
            "中等焦虑水平，对未知有一定担忧但通常合理",
            "比常人更易焦虑，会为各种可能性做准备",
            "经常感到焦虑，对小事也容易担心紧张",
            "高焦虑，常预想最坏情况，影响生活决策",
            "极高频度焦虑，时刻担忧各种可能风险",
            "持续极端焦虑，难以放松，常被恐惧支配",
        ],
    },
    // N2 愤怒、敌意
    SubdimensionInterpretation {
        dimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
        name: "愤怒与敌意",
        description: "容易生气、产生敌意或对他人不满的程度",
        analysis: &[
            "几乎从不生气，极其宽容平和",
            "很少发脾气，对人极为宽容耐心",
            "偶有不满但不轻易发怒，能理性表达",
            "会生气但能控制，不常表现出敌意",
            "正常愤怒水平，受挑衅时会合理反击",
            "比常人更易恼怒，对不公平更敏感",
            "经常生气，容易对他人产生不满",
            "高愤怒倾向，常有敌意情绪，难宽容",
            "极易暴怒，常怀敌意，易与人冲突",
            "极端易怒，敌意极强，几乎无法控制愤怒",
        ],
    },
    // N3 抑郁
    SubdimensionInterpretation {
        dimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
        name: "抑郁",
        description: "感到情绪低落、悲观或失去兴趣的程度",
        analysis: &[
            "极其乐观，几乎从不情绪低落",
            "非常积极，很少感到悲伤或失去兴趣",
            "偶尔情绪低落但很快恢复，整体乐观",
            "适度悲观情绪，遇到挫折会低落但能走出",
            "正常抑郁倾向，有高低起伏但总体平衡",
            "比常人更易抑郁，常反思负面可能性",
            "经常情绪低落，对许多事提不起兴趣",
            "高频度抑郁，常感悲伤无望，恢复缓慢",
            "深度抑郁倾向，长期情绪低落影响生活",
            "极端抑郁，持续强烈悲伤，完全丧失兴趣",
        ],
    },
    // N4 自我意识/人际关系敏感
    SubdimensionInterpretation {
        dimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
        name: "自我意识/人际敏感",
        description: "在意他人看法、容易感到尴尬或被评价的程度",
        analysis: &[
            "完全不在意他人评价，社交中极度自在",
            "极少在意别人看法，社交轻松自然",
            "偶会在意评价但不困扰，社交基本自在",
            "适度关注他人看法，会因此调整行为",
            "正常敏感度，希望被认可但不过度在意",
            "比常人更在意评价，社交中略显拘谨",
            "非常在意他人眼光，常担心被负面评价",
            "高度敏感，易感尴尬，社交压力很大",
            "极度在意他人看法，几乎无法放松社交",
            "完全被他人评价支配，社交中极度焦虑",
        ],
    },
    // N5 冲动性
    SubdimensionInterpretation {
        dimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
        name: "冲动性",
        description: "做事是否容易冲动、缺乏思考的程度",
        analysis: &[
            "极其谨慎，凡事深思熟虑后才行动",
            "非常谨慎，很少做出冲动决定",
            "通常谨慎，偶有冲动但影响不大",
            "适度冲动，有时会凭直觉做决定",
            "正常冲动水平，平衡思考与直觉",
            "比常人更易冲动，常凭感觉决策",
            "经常冲动行事，事后常反思决策",
            "高度冲动，很少考虑后果就行动",
            "极度冲动，几乎不思考就做决定",
            "完全冲动主导，行为完全缺乏规划",
        ],
    },
    // N6 脆弱性
    SubdimensionInterpretation {
        dimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
        name: "脆弱性",
        description: "面对压力或挫折时的抗压能力",
        analysis: &[
            "极其坚韧，面对巨大压力也从容应对",
            "非常抗压，困难面前保持冷静高效",
            "抗压能力较强，能处理大多数挫折",
            "适度抗压，遇到困难需要一些时间恢复",
            "正常脆弱性，有压力反应但能应对",
            "比常人更易被压力影响，需要支持",
            "抗压能力较弱，压力下效率明显下降",
            "高度脆弱，遇到挫折容易情绪崩溃",
            "极其脆弱，微小压力就可能导致崩溃",
            "完全无法承受压力，需要持续外部支持",
        ],
    },
];

// ===================== 外向性（E）子维度解释 =====================
const E_SUBDIMENSIONS: [SubdimensionInterpretation; 6] = [
    // E1 热情
    SubdimensionInterpretation {
        dimension: Subdimension::E(ExtraversionSubdimension::E1),
        name: "热情",
        description: "对待他人友好、温暖和亲近的程度",
        analysis: &[
            "极其冷淡疏远，几乎不表达情感",
            "非常冷淡，很少表现出热情或友好",
            "略显冷淡，但基本礼貌",
            "适度热情，对熟人友好但不过度",
            "正常热情水平，能友好待人",
            "比常人更热情，容易表达友好",
            "非常热情，经常表现出温暖和亲近",
            "高度热情，对大多数人非常友好温暖",
            "极其热情，几乎对所有人表达强烈友好",
            "极端热情，时刻充满情感表达和亲近感",
        ],
    },
    // E2 乐群性
    SubdimensionInterpretation {
        dimension: Subdimension::E(ExtraversionSubdimension::E2),
        name: "乐群性",
        description: "喜欢与他人相处和参与社交活动的程度",
        analysis: &[
            "极度孤僻，强烈避免社交活动",
            "非常孤僻，更喜欢独处",
            "略显孤僻，社交有限但能参与",
            "适度合群，有选择地参与社交",
            "正常合群水平，享受适度社交",
            "比常人更合群，喜欢社交活动",
            "非常合群，经常参与各种社交",
            "高度合群，几乎所有时间都想与人相处",
            "极其合群，无法忍受孤独",
            "极端合群，完全依赖社交活动获得能量",
        ],
    },
    // E3 自信
    SubdimensionInterpretation {
        dimension: Subdimension::E(ExtraversionSubdimension::E3),
        name: "自信",
        description: "在社交场合中自信和主导的程度",
        analysis: &[
            "极其不自信，社交中完全被动",
            "非常不自信，避免成为关注焦点",
            "略显不自信，但能在熟悉环境表达",
            "适度自信，能在舒适区主导",
            "正常自信水平，平衡主动与倾听",
            "比常人更自信，常主动发言",
            "非常自信，在社交中表现主导",
            "高度自信，几乎总是掌控社交场合",
            "极其自信，强烈的存在感和影响力",
            "极端自信，在任何场合都完全主导",
        ],
    },
    // E4 活跃性
    SubdimensionInterpretation {
        dimension: Subdimension::E(ExtraversionSubdimension::E4),
        name: "活跃性",
        description: "日常生活中的精力和活动水平",
        analysis: &[
            "极其低活力，总是感到疲惫无力",
            "非常低活力，活动量很小",
            "活力较低，需要大量休息",
            "适度活力，能完成日常活动",
            "正常活力水平，保持日常节奏",
            "比常人更有活力，喜欢保持忙碌",
            "非常活跃，精力充沛",
            "高度活跃，几乎总是充满能量",
            "极其活跃，难以静下来休息",
            "极端活跃，时刻精力旺盛不停歇",
        ],
    },
    // E5 寻求刺激
    SubdimensionInterpretation {
        dimension: Subdimension::E(ExtraversionSubdimension::E5),
        name: "寻求刺激",
        description: "追求兴奋、刺激和新奇体验的程度",
        analysis: &[
            "极其规避刺激，强烈偏好稳定常规",
            "非常规避刺激，喜欢可预测环境",
            "略显规避刺激，偶尔尝试新事物",
            "适度寻求刺激，平衡常规与新奇",
            "正常刺激寻求水平，适度尝试新体验",
            "比常人更爱刺激，常寻求新鲜感",
            "非常爱刺激，经常尝试冒险活动",
            "高度寻求刺激，生活充满新奇体验",
            "极其爱刺激，几乎无法忍受常规",
            "极端寻求刺激，持续追求高风险体验",
        ],
    },
    // E6 积极情绪
    SubdimensionInterpretation {
        dimension: Subdimension::E(ExtraversionSubdimension::E6),
        name: "积极情绪",
        description: "体验到快乐、乐观和愉快情绪的频率",
        analysis: &[
            "极其少积极情绪，几乎从不感到快乐",
            "非常少积极情绪，很少表现快乐",
            "略显低落，但偶尔感到愉快",
            "适度积极情绪，有正常的快乐体验",
            "正常积极情绪水平，经常感到愉快",
            "比常人更积极，经常快乐乐观",
            "非常积极，几乎总是心情愉快",
            "高度积极，持续体验到强烈快乐",
            "极其积极，时刻充满愉悦感",
            "极端积极，几乎无法抑制的快乐情绪",
        ],
    },
];

// ===================== 开放性（O）子维度解释 =====================
const O_SUBDIMENSIONS: [SubdimensionInterpretation; 6] = [
    // O1 想象力
    SubdimensionInterpretation {
        dimension: Subdimension::O(OpenMindednessSubdimension::O1),
        name: "想象力",
        description: "拥有丰富想象力和内心世界的程度",
        analysis: &[
            "极其缺乏想象，完全现实导向",
            "非常缺乏想象，很少幻想",
            "想象力有限，主要关注现实",
            "适度想象力，偶尔幻想但不沉迷",
            "正常想象水平，平衡现实与想象",
            "比常人更有想象力，常沉浸幻想",
            "非常富有想象力，拥有丰富内心世界",
            "高度想象力，经常创造复杂幻想",
            "极其富有想象力，几乎活在幻想中",
            "极端想象力，完全被内心世界主导",
        ],
    },
    // O2 审美/艺术美感
    SubdimensionInterpretation {
        dimension: Subdimension::O(OpenMindednessSubdimension::O2),
        name: "审美",
        description: "对艺术、美感和审美体验的欣赏程度",
        analysis: &[
            "完全无视艺术和美，无审美兴趣",
            "非常不关注审美，很少欣赏艺术",
            "审美兴趣有限，只关注实用",
            "适度审美欣赏，偶尔关注艺术",
            "正常审美水平，能欣赏常见美",
            "比常人更爱美，常欣赏艺术",
            "非常注重审美，经常参与艺术活动",
            "高度审美敏感，强烈艺术鉴赏力",
            "极其注重审美，生活围绕美展开",
            "极端审美追求，完全被美感支配",
        ],
    },
    // O3 感受丰富度
    SubdimensionInterpretation {
        dimension: Subdimension::O(OpenMindednessSubdimension::O3),
        name: "感受丰富度",
        description: "对自身和他人情感的觉察与体验深度",
        analysis: &[
            "极其情感淡漠，几乎不关注感受",
            "非常情感淡漠，很少体验深刻情感",
            "情感体验有限，主要关注表面",
            "适度感受丰富，能体验基本情感",
            "正常情感深度，能理解复杂感受",
            "比常人感受更丰富，情感体验深",
            "非常感受丰富，经常深入体验情感",
            "高度情感敏感，强烈共情能力",
            "极其感受丰富，时刻体验深刻情感",
            "极端情感深度，完全被感受淹没",
        ],
    },
    // O4 尝鲜
    SubdimensionInterpretation {
        dimension: Subdimension::O(OpenMindednessSubdimension::O4),
        name: "尝鲜",
        description: "愿意尝试新事物、新体验的程度",
        analysis: &[
            "极其抗拒新事物，强烈偏好熟悉",
            "非常抗拒新事物，很少尝试新体验",
            "略显保守，但偶尔尝试小变化",
            "适度开放，平衡熟悉与新鲜",
            "正常尝新水平，适度尝试新事物",
            "比常人更爱尝新，常寻求新体验",
            "非常爱尝新，经常尝试不同事物",
            "高度开放，生活充满新体验",
            "极其爱尝新，几乎无法忍受常规",
            "极端尝新追求，持续寻找全新体验",
        ],
    },
    // O5 思辨/求知
    SubdimensionInterpretation {
        dimension: Subdimension::O(OpenMindednessSubdimension::O5),
        name: "思辨",
        description: "对抽象概念、哲学问题的兴趣程度",
        analysis: &[
            "完全无思辨兴趣，只关注具体事实",
            "非常缺乏思辨，很少思考抽象问题",
            "思辨有限，主要在实用层面思考",
            "适度思辨，偶尔思考抽象问题",
            "正常思辨水平，能进行抽象思考",
            "比常人更爱思辨，常思考深层问题",
            "非常爱思辨，经常探讨哲学概念",
            "高度思辨，深度探索抽象理论",
            "极其爱思辨，几乎只思考抽象问题",
            "极端思辨追求，完全沉浸抽象世界",
        ],
    },
    // O6 价值观
    SubdimensionInterpretation {
        dimension: Subdimension::O(OpenMindednessSubdimension::O6),
        name: "价值观",
        description: "对社会规范和传统价值观的挑战程度",
        analysis: &[
            "极其传统保守，严格遵守社会规范",
            "非常传统，很少质疑现有价值观",
            "略显保守，但能接受小变化",
            "适度开放，平衡传统与创新",
            "正常开放水平，有选择地接受新观念",
            "比常人更开放，常质疑传统观念",
            "非常开放，经常挑战社会规范",
            "高度开放，强烈反传统倾向",
            "极其开放，几乎完全拒绝传统",
            "极端开放，彻底颠覆所有传统价值观",
        ],
    },
];

// ===================== 宜人性（A）子维度解释 =====================
const A_SUBDIMENSIONS: [SubdimensionInterpretation; 6] = [
    // A1 信任
    SubdimensionInterpretation {
        dimension: Subdimension::A(AgreeablenessSubdimension::A1),
        name: "信任",
        description: "对他人的信任程度，是否容易怀疑他人",
        analysis: &[
            "极其多疑，几乎不相信任何人",
            "非常多疑，总是怀疑他人动机",
            "略显多疑，需要时间建立信任",
            "适度信任，平衡信任与谨慎",
            "正常信任水平，通常相信他人善意",
            "比常人更信任，容易相信别人",
            "非常信任，通常认为他人诚实",
            "高度信任，几乎总是相信他人",
            "极其信任，很难怀疑任何人",
            "极端信任，完全天真轻信他人",
        ],
    },
    // A2 坦诚
    SubdimensionInterpretation {
        dimension: Subdimension::A(AgreeablenessSubdimension::A2),
        name: "坦诚",
        description: "待人是否真诚、坦率，是否愿意表露真实想法",
        analysis: &[
            "极其不坦诚，总是隐藏真实想法",
            "非常不坦诚，很少表达真实感受",
            "略显保留，选择性表达想法",
            "适度坦诚，在舒适区表达真实",
            "正常坦诚水平，通常诚实但考虑他人感受",
            "比常人更坦诚，经常直言不讳",
            "非常坦诚，几乎总是表达真实想法",
            "高度坦诚，完全诚实不考虑后果",
            "极其坦诚，毫无保留表达一切",
            "极端坦诚，完全不顾及他人感受的直率",
        ],
    },
    // A3 利他
    SubdimensionInterpretation {
        dimension: Subdimension::A(AgreeablenessSubdimension::A3),
        name: "利他",
        description: "愿意为他人付出、帮助他人的程度",
        analysis: &[
            "极其自私，几乎从不帮助他人",
            "非常自私，很少考虑他人需要",
            "略显自私，但会帮助亲近的人",
            "适度利他，平衡自我与他人需要",
            "正常利他水平，愿意帮助他人",
            "比常人更利他，经常主动帮助",
            "非常利他，总是考虑他人福祉",
            "高度利他，几乎总把他人放第一位",
            "极其利他，完全忽视自己需要",
            "极端利他，完全自我牺牲倾向",
        ],
    },
    // A4 顺从
    SubdimensionInterpretation {
        dimension: Subdimension::A(AgreeablenessSubdimension::A4),
        name: "顺从",
        description: "是否愿意妥协、配合他人，避免冲突的程度",
        analysis: &[
            "极其不顺从，总是坚持自己观点",
            "非常不顺从，很少妥协",
            "略显固执，但能在重要事情妥协",
            "适度顺从，平衡坚持与妥协",
            "正常顺从水平，合理让步避免冲突",
            "比常人更顺从，经常让步妥协",
            "非常顺从，几乎总是避免冲突",
            "高度顺从，总是顺从他人意愿",
            "极其顺从，完全不敢表达反对",
            "极端顺从，完全丧失自我意愿",
        ],
    },
    // A5 谦逊
    SubdimensionInterpretation {
        dimension: Subdimension::A(AgreeablenessSubdimension::A5),
        name: "谦逊",
        description: "是否低调、不自夸，看待自己成就的态度",
        analysis: &[
            "极其自负，总是夸耀自己成就",
            "非常自负，很少承认他人优点",
            "略显自负，但能适当谦逊",
            "适度谦逊，平衡自信与谦虚",
            "正常谦逊水平，适当表现但不张扬",
            "比常人更谦逊，经常贬低自己",
            "非常谦逊，总是低调不张扬",
            "高度谦逊，几乎从不接受表扬",
            "极其谦逊，完全否定自己价值",
            "极端谦逊，病态的自卑和贬低",
        ],
    },
    // A6 同理心
    SubdimensionInterpretation {
        dimension: Subdimension::A(AgreeablenessSubdimension::A6),
        name: "同理心",
        description: "理解和感受他人情绪的能力",
        analysis: &[
            "完全缺乏同理心，无法理解他人感受",
            "非常缺乏同理心，很少考虑他人情绪",
            "同理心有限，主要理解表面情绪",
            "适度同理心，能理解基本情感",
            "正常同理心水平，能体会他人感受",
            "比常人更有同理心，常感同身受",
            "非常强的同理心，深刻理解他人情绪",
            "高度同理心，几乎能感受他人一切",
            "极其强的同理心，常被他人情绪淹没",
            "极端同理心，完全失去自我情绪边界",
        ],
    },
];

// ===================== 尽责性（C）子维度解释 =====================
const C_SUBDIMENSIONS: [SubdimensionInterpretation; 6] = [
    // C1 能力
    SubdimensionInterpretation {
        dimension: Subdimension::C(ConscientiousnessSubdimension::C1),
        name: "能力",
        description: "对自己能力的信心和做事的胜任力",
        analysis: &[
            "完全无能感，认为自己什么都做不好",
            "非常低能力感，缺乏完成任务信心",
            "能力感有限，只对熟悉任务有信心",
            "适度能力感，对多数任务有信心",
            "正常能力感，合理评估自己能力",
            "比常人更有能力感，常自信能胜任",
            "非常强的能力感，相信自己能做好一切",
            "高度能力感，几乎从不怀疑自己能力",
            "极其强的能力感，过度自信倾向",
            "极端能力感，完全不现实的自我高估",
        ],
    },
    // C2 条理性
    SubdimensionInterpretation {
        dimension: Subdimension::C(ConscientiousnessSubdimension::C2),
        name: "条理性",
        description: "做事是否有计划、有条理，生活和工作是否整洁",
        analysis: &[
            "完全无序，生活工作极度混乱",
            "非常无序，很少有计划安排",
            "略显无序，但能维持基本秩序",
            "适度条理，重要事情会计划",
            "正常条理水平，保持基本组织性",
            "比常人更有条理，喜欢计划一切",
            "非常条理，生活工作高度组织化",
            "高度条理，严格遵循计划安排",
            "极其条理，无法忍受任何无序",
            "极端条理，病态的秩序和控制需求",
        ],
    },
    // C3 责任感
    SubdimensionInterpretation {
        dimension: Subdimension::C(ConscientiousnessSubdimension::C3),
        name: "责任感",
        description: "对自己的承诺和任务负责的程度",
        analysis: &[
            "完全无责任感，从不履行承诺",
            "非常缺乏责任感，经常逃避责任",
            "责任感有限，只对重要承诺负责",
            "适度责任感，通常会履行承诺",
            "正常责任感水平，认真对待责任",
            "比常人更有责任感，总是尽力负责",
            "非常强的责任感，视承诺为必须",
            "高度责任感，承担过多责任",
            "极其强的责任感，无法拒绝任何责任",
            "极端责任感，病态的责任负担",
        ],
    },
    // C4 成就追求
    SubdimensionInterpretation {
        dimension: Subdimension::C(ConscientiousnessSubdimension::C4),
        name: "成就追求",
        description: "对成就的渴望和努力程度",
        analysis: &[
            "完全无成就动机，满足于最低标准",
            "非常低成就动机，很少追求卓越",
            "成就动机有限，只在意重要目标",
            "适度成就追求，在关键领域努力",
            "正常成就动机，合理追求成功",
            "比常人更追求成就，常设高目标",
            "非常强的成就动机，总是追求卓越",
            "高度成就追求，几乎所有事都追求最好",
            "极其强的成就动机，无法接受不完美",
            "极端成就追求，病态的完美主义",
        ],
    },
    // C5 自律
    SubdimensionInterpretation {
        dimension: Subdimension::C(ConscientiousnessSubdimension::C5),
        name: "自律",
        description: "坚持完成任务、克服困难的能力",
        analysis: &[
            "完全无自律，几乎从不坚持完成任务",
            "非常缺乏自律，经常半途而废",
            "自律有限，只对感兴趣事坚持",
            "适度自律，能在监督下完成任务",
            "正常自律水平，通常能坚持到底",
            "比常人更自律，有很强意志力",
            "非常强的自律，总能克服困难完成",
            "高度自律，严格自我要求和控制",
            "极其强的自律，近乎苛刻的自我控制",
            "极端自律，完全压抑本能的自我控制",
        ],
    },
    // C6 审慎
    SubdimensionInterpretation {
        dimension: Subdimension::C(ConscientiousnessSubdimension::C6),
        name: "审慎",
        description: "做事是否谨慎、考虑周全的程度",
        analysis: &[
            "完全冲动，从不思考后果",
            "非常冲动，很少仔细考虑",
            "略显冲动，但重大决定会思考",
            "适度审慎，平衡思考与行动",
            "正常审慎水平，通常会思考后果",
            "比常人更审慎，总是深思熟虑",
            "非常审慎，行动前仔细分析一切",
            "高度审慎，过度分析导致犹豫",
            "极其审慎，几乎无法做出决定",
            "极端审慎，完全的分析瘫痪",
        ],
    },
];

// ===================== 核心维度解释 =====================
const DIMENSIONS: &[DimensionInterpretation; 5] = &[
    // 神经质（N）
    DimensionInterpretation {
        dimension: Dimension::N,
        name: "神经质/情绪稳定性",
        terminology: "Neuroticism",
        description: "衡量一个人情绪的稳定性和抗压能力",
        analysis: &[
            "情绪极其稳定，几乎不会感到焦虑、愤怒或抑郁，抗压能力超强，面对任何挫折都能保持冷静",
            "情绪很稳定，很少有负面情绪，只有极端情况才会感到不安，能快速调整心态",
            "情绪较稳定，偶尔有负面情绪，但持续时间短，能自我调节，不影响正常生活",
            "情绪稳定性中等，遇到压力会有负面情绪，但大部分时候能控制，需要一定时间恢复",
            "情绪稳定性一般，负面情绪出现频率适中，需要他人或自我调节才能恢复",
            "情绪稍不稳定，经常有负面情绪，容易焦虑或生气，恢复时间较长",
            "情绪较不稳定，负面情绪频繁，容易陷入焦虑、抑郁，影响日常生活",
            "情绪很不稳定，大部分时间被负面情绪困扰，抗压能力差，容易情绪崩溃",
            "情绪极不稳定，长期处于负面情绪中，焦虑、愤怒、抑郁交替出现，需要外界帮助",
            "情绪完全失控，时刻被强烈的负面情绪支配，无法自我调节，严重影响生活",
        ],
        subdimension_interpretations: &N_SUBDIMENSIONS,
    },
    // 外向性（E）
    DimensionInterpretation {
        dimension: Dimension::E,
        name: "外向性",
        terminology: "Extraversion",
        description: "衡量一个人是否喜欢社交、内向或者外向的程度",
        analysis: &[
            "极度内向，完全从独处中获取能量，社交会感到极度疲惫，几乎不参加社交活动",
            "非常内向，喜欢独处，只有必要时才社交，社交后需要大量时间恢复",
            "比较内向，偏好独处，但能应付必要的社交，不主动发起社交",
            "轻微内向，独处和社交都能接受，独处时更放松，社交后需要短暂休息",
            "内外向平衡，既能享受独处，也能享受社交，能量来源均衡",
            "轻微外向，喜欢社交，独处时间过长会感到无聊，社交后精力充沛",
            "比较外向，主动发起社交，喜欢和人相处，独处时间不宜过长",
            "非常外向，极度喜欢社交，从社交中获得大量能量，独处会感到孤独",
            "极度外向，几乎离不开社交，独处时会感到不适，需要不断和人互动",
            "完全外向，社交是主要的生活方式，一刻也不想独处，永远需要有人陪伴",
        ],
        subdimension_interpretations: &E_SUBDIMENSIONS,
    },
    // 开放性（O）
    DimensionInterpretation {
        dimension: Dimension::O,
        name: "开放性",
        terminology: "Openness to Experience",
        description: "衡量一个人对新事物、新想法的接受程度和思维的开放程度",
        analysis: &[
            "极度保守，只喜欢熟悉的事物，拒绝任何新想法和新体验，思维固定不变",
            "非常保守，偏好传统和熟悉的事物，对新事物有强烈的抵触情绪",
            "比较保守，接受少量主流的新事物，大部分时候喜欢按部就班",
            "轻微保守，基本接受现状，偶尔尝试小的改变，不喜欢大的变化",
            "开放性中等，既接受传统也接受新事物，不极端，能平衡保守和创新",
            "轻微开放，愿意尝试新事物，对新想法有好奇心，偶尔突破常规",
            "比较开放，喜欢新想法和新体验，主动尝试不同的生活方式",
            "非常开放，极度好奇，乐于接受各种新奇的想法和体验，思维活跃",
            "极度开放，完全拥抱新鲜事物，不断追求创新，思维不受任何限制",
            "完全开放，视变化为常态，主动创造新体验，思维没有边界",
        ],
        subdimension_interpretations: &O_SUBDIMENSIONS,
    },
    // 宜人性（A）
    DimensionInterpretation {
        dimension: Dimension::A,
        name: "宜人性",
        terminology: "Agreeableness",
        description: "衡量一个人是否友善、合作、愿意为他人考虑",
        analysis: &[
            "极度自我，完全以自我为中心，不考虑他人感受，容易和人产生冲突",
            "非常自我，优先考虑自己的利益，很少关心他人，合作意愿极低",
            "比较自我，大部分时候考虑自己，偶尔会关心他人，合作需要条件",
            "轻微自我，以自我为主，必要时会为他人考虑，合作意愿一般",
            "宜人性中等，既考虑自己也考虑他人，能平衡自我和他人的需求",
            "轻微宜人，愿意为他人考虑，合作意愿较强，偶尔会坚持自我",
            "比较宜人，非常愿意合作，经常为他人着想，很少和人冲突",
            "非常宜人，极度友善，总是优先考虑他人，愿意妥协和付出",
            "极度宜人，完全以他人为中心，无底线迁就他人，避免任何冲突",
            "完全宜人，利他主义者，愿意牺牲自己的利益满足他人，从不计较得失",
        ],
        subdimension_interpretations: &A_SUBDIMENSIONS,
    },
    // 尽责性（C）
    DimensionInterpretation {
        dimension: Dimension::C,
        name: "尽责性/责任心",
        terminology: "Conscientiousness",
        description: "衡量一个人是否有组织、有计划、负责任",
        analysis: &[
            "极度随意，完全没有计划，缺乏责任感，做事随心所欲，经常违约",
            "非常随意，很少做计划，责任感差，经常拖延，难以完成承诺",
            "比较随意，偶尔做计划，责任感一般，容易拖延和放弃",
            "轻微随意，有基本的计划，能完成主要任务，偶尔会拖延",
            "尽责性中等，有计划且能执行，责任感适中，偶尔会有疏漏",
            "轻微尽责，做事有计划，责任感较强，能按时完成任务",
            "比较尽责，高度自律，有详细的计划，责任感强，很少出错",
            "非常尽责，极度自律，凡事都有规划，责任感极强，追求完美",
            "极度尽责，完美主义者，计划详尽，绝不拖延，对自己要求极高",
            "完全尽责，把责任视为生命，任何事都做到极致，从不允许失误",
        ],
        subdimension_interpretations: &C_SUBDIMENSIONS,
    },
];

const INTERPRETATION: Interpretation = Interpretation {
    norm: NORM,
    scoring_rule: SCORING_RULE,
    dimensions: DIMENSIONS,
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
