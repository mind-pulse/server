use super::{HTMLElement, SentenceItem, Texts};

pub(super) const INTRODUCTION: Texts = &[
    &[
        SentenceItem::Plain("可用于"),
        SentenceItem::HTMLElement(HTMLElement::Strong("就业方向")),
        SentenceItem::Plain("的指导和"),
        SentenceItem::HTMLElement(HTMLElement::Strong("高考填报志愿")),
        SentenceItem::Plain("时的专业选择。"),
    ],
    &[SentenceItem::Plain(
        "由美国职业指导专家霍兰德（John Holland）根据他本人大量的职业咨询经验及其职业类型理论编制的测评工具。霍兰德认为，个人职业兴趣特性与职业之间应有一种内在的对应关系。",
    )],
];

pub(super) const INSTRUCTION: Texts = &[
    &[
        SentenceItem::Plain("本测量表将帮助你发现和确定自己的"),
        SentenceItem::HTMLElement(HTMLElement::Strong("职业兴趣")),
        SentenceItem::Plain("和"),
        SentenceItem::HTMLElement(HTMLElement::Strong("能力特长")),
        SentenceItem::Plain("，从而更好地选择适合自己的专业。"),
    ],
    &[
        SentenceItem::Plain("在测评的时候，要求在一个"),
        SentenceItem::HTMLElement(HTMLElement::Strong(
            "不受干扰的、安静的、独立的空间和平静的心态",
        )),
        SentenceItem::Plain("下进行。"),
    ],
];
