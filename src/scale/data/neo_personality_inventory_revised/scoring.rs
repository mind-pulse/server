use super::{Comparison, Operator, QuestionOption, Score, ScoringRule};

/// 按常模 T 分（M=50, SD=10）划档：档位边界为 M 的 ±0.5SD 整数倍，即每 5 个 T 分一档。
pub(super) const SCORING_RULE: ScoringRule = ScoringRule {
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
                    expression: "M+0.5*SD",
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
                    expression: "M+0.5*SD",
                    operator: Operator::G,
                }),
                Some(Comparison {
                    expression: "M+1*SD",
                    operator: Operator::LE,
                }),
            ],
        },
        Score {
            value: 8,
            comparisons: &[
                Some(Comparison {
                    expression: "M+1*SD",
                    operator: Operator::G,
                }),
                Some(Comparison {
                    expression: "M+1.5*SD",
                    operator: Operator::LE,
                }),
            ],
        },
        Score {
            value: 9,
            comparisons: &[
                Some(Comparison {
                    expression: "M+1.5*SD",
                    operator: Operator::G,
                }),
                Some(Comparison {
                    expression: "M+2*SD",
                    operator: Operator::LE,
                }),
            ],
        },
        Score {
            value: 10,
            comparisons: &[
                Some(Comparison {
                    expression: "M+2*SD",
                    operator: Operator::G,
                }),
                None,
            ],
        },
    ],
};

pub(super) const FORWARD: &[QuestionOption; 5] = &[
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

pub(super) const REVERSE: &[QuestionOption; 5] = &[
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
