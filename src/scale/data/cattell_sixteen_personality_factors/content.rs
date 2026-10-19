use super::{HTMLElement, SentenceItem, Texts};

pub(super) const INTRODUCTION: Texts = &[&[
    SentenceItem::Plain("从"),
    SentenceItem::HTMLElement(HTMLElement::Strong(
        "乐群、聪慧、自律、独立、敏感、冒险、怀疑",
    )),
    SentenceItem::Plain("等 "),
    SentenceItem::HTMLElement(HTMLElement::Strong("16")),
    SentenceItem::Plain(" 个相对独立的人格特点对人进行描绘，并可以了解应试者在"),
    SentenceItem::HTMLElement(HTMLElement::Strong("环境适应、专业成就和心理健康")),
    SentenceItem::Plain("等方面的表现。在人事管理中，16PF 能够预测应试者的"),
    SentenceItem::HTMLElement(HTMLElement::Strong("工作稳定性、工作效率和压力承受能力")),
    SentenceItem::Plain("等。可广泛应用于"),
    SentenceItem::HTMLElement(HTMLElement::Strong("心理咨询、人员选拔和职业指导")),
    SentenceItem::Plain("的各个环节，为人事决策和人事诊断提供个人心理素质的参考依据。"),
]];

pub(super) const INSTRUCTION: Texts = &[
    &[
        SentenceItem::Plain("本测验共有 "),
        SentenceItem::HTMLElement(HTMLElement::Strong("187")),
        SentenceItem::Plain(" 道题目，都是有关个人的"),
        SentenceItem::HTMLElement(HTMLElement::Strong("兴趣和态度")),
        SentenceItem::Plain(
            "等问题。每个人对这些问题是会有不同看法的，回答也是不同的，因而对问题如何回答，并",
        ),
        SentenceItem::HTMLElement(HTMLElement::Strong("没有对与不对之分")),
        SentenceItem::Plain("，只是表明你对这些问题的态度。请你要尽量表达个人的意见，"),
        SentenceItem::HTMLElement(HTMLElement::Strong("不要有顾虑")),
        SentenceItem::Plain("。"),
    ],
    &[SentenceItem::Plain("应当记住的是：")],
    &[SentenceItem::Plain("1．每一测题只能选择一个答案。")],
    &[SentenceItem::Plain("2．不可漏掉任何题目。")],
    &[SentenceItem::Plain("3．尽量不选择 B 答案。")],
    &[
        SentenceItem::Plain(
            "4．本测验不计时间，但应凭自己的直觉反应进行作答，不要迟疑不决，拖延时间。一定要在",
        ),
        SentenceItem::HTMLElement(HTMLElement::Strong("一个小时以内")),
        SentenceItem::Plain("完成整个测验。"),
    ],
    &[
        SentenceItem::Plain(
            "5．有些题目你可能从未思考过，或者感到不太容易回答。对于这样的题目，同样要求你做出一种",
        ),
        SentenceItem::HTMLElement(HTMLElement::Strong("倾向性")),
        SentenceItem::Plain("的选择。"),
    ],
];
