use super::{HTMLElement, SentenceItem, Texts};

pub(super) const INTRODUCTION: Texts = &[
    &[
        SentenceItem::Plain(
            "艾森克人格问卷（Eysenck Personality Questionnaire, 简称 EPQ）是英国伦敦大学心理系和精神病研究所艾森克教授编制的，归纳了三个人格的基本因素：",
        ),
        SentenceItem::HTMLElement(HTMLElement::Strong("内外向性")),
        SentenceItem::Plain("(E)、"),
        SentenceItem::HTMLElement(HTMLElement::Strong("神经质")),
        SentenceItem::Plain("(又称情绪性)(N)和"),
        SentenceItem::HTMLElement(HTMLElement::Strong("精神质")),
        SentenceItem::Plain("(又称倔强、讲求实际)(P)，以及"),
        SentenceItem::HTMLElement(HTMLElement::Strong("效度量表")),
        SentenceItem::Plain("(L，用于检测作答的真实程度)。"),
    ],
    &[SentenceItem::Plain(
        "三个人格度不但经过许多数学统计上的和行为观察方面的分析，而且也得到实验室内多种心理实验的考察，被广泛应用于医学、司法、教育和心理咨询等领域，适合各种人群测试。",
    )],
    &[SentenceItem::Plain(
        "艾森克人格问卷简式量表中国版在原量表的基础上针对中国进行了修订，在保持甚至提高信度的同时精简了部分题目。",
    )],
];

pub(super) const INSTRUCTION: Texts = &[&[
    SentenceItem::Plain("该量表共有 "),
    SentenceItem::HTMLElement(HTMLElement::Strong("48")),
    SentenceItem::Plain(
        " 个项目，请仔细阅读每一条，然后根据该句话与您自己的实际情况相符合的程度进行选择。",
    ),
]];
