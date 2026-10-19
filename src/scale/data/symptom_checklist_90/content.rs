use super::{HTMLElement, SentenceItem, Texts};

pub(super) const INTRODUCTION: Texts = &[&[
    SentenceItem::Plain("《SCL-90症状自评量表》是全球使用"),
    SentenceItem::HTMLElement(HTMLElement::Strong("最广泛")),
    SentenceItem::Plain("的心理健康评估工具之一，帮助你从"),
    SentenceItem::HTMLElement(HTMLElement::Strong("情绪、压力、人际关系等9个症状维度")),
    SentenceItem::Plain("全面了解自己的心理状态。它常用于专业初筛，也能为你提供自我觉察的参考。"),
]];

pub(super) const INSTRUCTION: Texts = &[
    &[
        SentenceItem::Plain("该量表共有 "),
        SentenceItem::HTMLElement(HTMLElement::Strong("90")),
        SentenceItem::Plain(" 个项目，包含较广泛的心理与精神症状，从"),
        SentenceItem::HTMLElement(HTMLElement::Strong("感觉、情感、思维、意识、行为")),
        SentenceItem::Plain("直至"),
        SentenceItem::HTMLElement(HTMLElement::Strong("生活习惯、人际关系、饮食睡眠")),
        SentenceItem::Plain(
            "等，均有涉及，并采用 9 个因子分别反映 9 个方面的心理症状情况，另有 7 个附加项目单独归类。",
        ),
    ],
    &[
        SentenceItem::Plain(
            "以下问题中列出了有些人可能有的症状或问题，请仔细阅读每一条，然后根据该句话与你自己的实际情况相符合的程度（",
        ),
        SentenceItem::HTMLElement(HTMLElement::Strong("最近一个星期或现在")),
        SentenceItem::Plain("）进行选择。"),
    ],
];
