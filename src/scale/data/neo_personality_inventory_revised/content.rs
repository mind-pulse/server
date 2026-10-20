use super::{HTMLElement, SentenceItem, Texts};

pub(super) const INTRODUCTION: Texts = &[
    &[SentenceItem::Plain(
        "心理学者研究发现大约有五种特质可以涵盖人格描述的所有方面，即大五人格，也被称之为人格的海洋。",
    )],
    &[
        SentenceItem::Plain("大五人格问卷多用于正常人群，特别是应用于"),
        SentenceItem::HTMLElement(HTMLElement::Strong("工作绩效领域")),
        SentenceItem::Plain("。一般要求测试者具有"),
        SentenceItem::HTMLElement(HTMLElement::Strong("初中三年级以上")),
        SentenceItem::Plain("文化水平。"),
    ],
    &[SentenceItem::Plain(
        "大五人格是西方心理学界公认的一个人格特质模型，在多年的研究中被不断验证。",
    )],
    &[SentenceItem::Plain(
        "大五人格测试有多个版本，本测试为句子式的 NEO-PI-R。",
    )],
];

pub(super) const INSTRUCTION: Texts = &[
    &[
        SentenceItem::Plain("本测试有 "),
        SentenceItem::HTMLElement(HTMLElement::Strong("240")),
        SentenceItem::Plain(" 题，由于题目较多，在开始测试前请保证你有充足的时间完成此测试。"),
    ],
    &[
        SentenceItem::Plain("测试的过程中要保持"),
        SentenceItem::HTMLElement(HTMLElement::Strong("环境的安静和舒适")),
        SentenceItem::Plain("，有"),
        SentenceItem::HTMLElement(HTMLElement::Strong("适当的照明和温度")),
        SentenceItem::Plain("，"),
        SentenceItem::HTMLElement(HTMLElement::Strong("排除任何可能的干扰")),
        SentenceItem::Plain("（比如有人旁观你的回答情况或者有人在室内走动等）。"),
    ],
    &[
        SentenceItem::Plain("你应以"),
        SentenceItem::HTMLElement(HTMLElement::Strong("直觉性的反应")),
        SentenceItem::Plain("依题作答，"),
        SentenceItem::HTMLElement(HTMLElement::Strong("无须迟疑不决，拖延时间")),
        SentenceItem::Plain("。"),
    ],
];
