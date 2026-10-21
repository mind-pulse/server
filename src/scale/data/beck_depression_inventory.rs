use crate::scale::{
    HTMLElement, InterpretationItem, Question, QuestionOption, Scale, ScaleCategory, SentenceItem,
    Status, Tag, Texts,
};

use super::depression_guidance::{
    MAJOR_ADVICE, MAJOR_COMFORT, MAJOR_SYMPTOMS, MAJOR_WARNING, MILD_ADVICE, MILD_COMFORT,
    MILD_SYMPTOMS, MODERATE_ADVICE, MODERATE_COMFORT, MODERATE_SYMPTOMS, NORMAL_ADVICE,
    NORMAL_COMFORT,
};

const INTRODUCTION: Texts = &[&[
    SentenceItem::Plain("贝克抑郁量表第二版（BDI-II）是抑郁自评量表中"),
    SentenceItem::HTMLElement(HTMLElement::Strong("最常用的量表")),
    SentenceItem::Plain(
        "之一，按 DSM-IV 抑郁症状修订，评估最近两周的抑郁症状严重程度，共 21 组陈述。",
    ),
]];

const INSTRUCTION: Texts = &[&[
    SentenceItem::Plain("本量表包含 "),
    SentenceItem::HTMLElement(HTMLElement::Strong("21")),
    SentenceItem::Plain(" 组陈述，每组 4 句，按 "),
    SentenceItem::HTMLElement(HTMLElement::Strong("0–3")),
    SentenceItem::Plain(" 级评分。请根据你"),
    SentenceItem::HTMLElement(HTMLElement::Strong("最近两周（包括今天）")),
    SentenceItem::Plain(
        "的实际情况，从每组中选择最符合的一句；如果一组中有两句以上符合，请选择最严重的一句。",
    ),
]];

pub const BECK_DEPRESSION_INVENTORY: Scale<&[InterpretationItem<u8>], Question> = Scale {
    id: 6,
    name: "贝克抑郁量表第二版（BDI-II）",
    description: "21 个问题，评估最近两周的抑郁症状严重程度",
    primary_category: ScaleCategory::Emotion,
    related_categories: Some(&[ScaleCategory::MentalHealth, ScaleCategory::Somatic]),
    abbreviation: "BDI-II",
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: Some(&[
        "Beck, A. T., Steer, R. A., & Brown, G. K. (1996). Manual for the Beck Depression Inventory-II. San Antonio, TX: Psychological Corporation.",
        "王振, 赵旭东, 王力. (2005). 贝克抑郁量表第二版中文版的信度与效度分析. 中国心理卫生杂志, 19(6), 321-323.",
        "Beck, A. T., Ward, C. H., Mendelson, M., Mock, J., & Erbaugh, J. (1961). An inventory for measuring depression. Archives of General Psychiatry, 4(6), 561-571.",
    ]),
    warning: Some(
        "适用年龄：13 岁及以上。版权提示：本量表的正式版本受版权保护，需通过官方渠道获得授权后使用；此处数据为公开资料转写，仅供个人学习参考，不得用于商业或临床用途。本结果仅为筛查参考，不能作为诊断依据。若情绪困扰持续或影响生活，请咨询精神科医生或心理治疗师。",
    ),
    tags: &Tag {
        info: Some(&["抑郁"]),
        normal: None,
        warning: None,
        error: None,
    },
    interpretation: &[
        InterpretationItem {
            range: [0, 13],
            description: "无抑郁",
            critical_warning: None,
            comforting_word: NORMAL_COMFORT,
            advice: Some(NORMAL_ADVICE),
            symptom: None,
            status: Status::Normal,
        },
        InterpretationItem {
            range: [14, 19],
            description: "轻度抑郁",
            critical_warning: None,
            comforting_word: MILD_COMFORT,
            advice: Some(MILD_ADVICE),
            symptom: Some(MILD_SYMPTOMS),
            status: Status::Mild,
        },
        InterpretationItem {
            range: [20, 28],
            description: "中度抑郁",
            critical_warning: None,
            comforting_word: MODERATE_COMFORT,
            advice: Some(MODERATE_ADVICE),
            symptom: Some(MODERATE_SYMPTOMS),
            status: Status::Moderate,
        },
        InterpretationItem {
            range: [29, 63],
            description: "重度抑郁",
            critical_warning: Some(MAJOR_WARNING),
            comforting_word: MAJOR_COMFORT,
            advice: Some(MAJOR_ADVICE),
            symptom: Some(MAJOR_SYMPTOMS),
            status: Status::Major,
        },
    ],
    questions: &[
        Question {
            title: "悲伤",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我不觉得悲伤",
                    point: 0,
                },
                QuestionOption {
                    text: "很多时候我都感到悲伤",
                    point: 1,
                },
                QuestionOption {
                    text: "所有时间我都感到悲伤",
                    point: 2,
                },
                QuestionOption {
                    text: "我太悲伤或太难过，不堪忍受",
                    point: 3,
                },
            ],
        },
        Question {
            title: "悲观",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没有对未来失去信心",
                    point: 0,
                },
                QuestionOption {
                    text: "我比以往更加对未来没有信心",
                    point: 1,
                },
                QuestionOption {
                    text: "我感到前景黯淡",
                    point: 2,
                },
                QuestionOption {
                    text: "我觉得将来毫无希望，且只会变得更糟",
                    point: 3,
                },
            ],
        },
        Question {
            title: "失败感",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我不觉得自己是个失败者",
                    point: 0,
                },
                QuestionOption {
                    text: "我的失败比较多",
                    point: 1,
                },
                QuestionOption {
                    text: "回首往事，我看到一大堆的失败",
                    point: 2,
                },
                QuestionOption {
                    text: "我觉得自己是一个彻底的失败者",
                    point: 3,
                },
            ],
        },
        Question {
            title: "失去乐趣",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我像过去一样能从喜欢的事情中得到乐趣",
                    point: 0,
                },
                QuestionOption {
                    text: "我不能像过去一样从喜欢的事情中得到乐趣",
                    point: 1,
                },
                QuestionOption {
                    text: "我从过去喜欢的事情中获得的快乐很少",
                    point: 2,
                },
                QuestionOption {
                    text: "我完全不能从过去喜欢的事情中获得快乐",
                    point: 3,
                },
            ],
        },
        Question {
            title: "内疚感",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没有特别的内疚感",
                    point: 0,
                },
                QuestionOption {
                    text: "我对已做过或该做但没做的许多事感到内疚",
                    point: 1,
                },
                QuestionOption {
                    text: "在大部分时间里我都感到内疚",
                    point: 2,
                },
                QuestionOption {
                    text: "我任何时候都感到内疚",
                    point: 3,
                },
            ],
        },
        Question {
            title: "惩罚感",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没觉得自己在受惩罚",
                    point: 0,
                },
                QuestionOption {
                    text: "我觉得自己可能会受到惩罚",
                    point: 1,
                },
                QuestionOption {
                    text: "我觉得自己会受到惩罚",
                    point: 2,
                },
                QuestionOption {
                    text: "我觉得正在受到惩罚",
                    point: 3,
                },
            ],
        },
        Question {
            title: "自我厌恶",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我对自己的感觉同过去一样",
                    point: 0,
                },
                QuestionOption {
                    text: "我对自己丧失了信心",
                    point: 1,
                },
                QuestionOption {
                    text: "我对自己感到失望",
                    point: 2,
                },
                QuestionOption {
                    text: "我讨厌我自己",
                    point: 3,
                },
            ],
        },
        Question {
            title: "自我责备",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "与过去相比，我没有更多的责备或批评自己",
                    point: 0,
                },
                QuestionOption {
                    text: "我比过去责备自己更多",
                    point: 1,
                },
                QuestionOption {
                    text: "只要我有过失，我就责备自己",
                    point: 2,
                },
                QuestionOption {
                    text: "只要发生不好的事情，我就责备自己",
                    point: 3,
                },
            ],
        },
        Question {
            title: "自杀想法",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没有任何自杀的想法",
                    point: 0,
                },
                QuestionOption {
                    text: "我有自杀的想法，但我不会去做",
                    point: 1,
                },
                QuestionOption {
                    text: "我想自杀",
                    point: 2,
                },
                QuestionOption {
                    text: "如果有机会我就会自杀",
                    point: 3,
                },
            ],
        },
        Question {
            title: "哭泣",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "和过去比较，我哭的次数并没有增加",
                    point: 0,
                },
                QuestionOption {
                    text: "我比过去哭的多",
                    point: 1,
                },
                QuestionOption {
                    text: "现在任何小事都会让我哭",
                    point: 2,
                },
                QuestionOption {
                    text: "我想哭，但哭不出来",
                    point: 3,
                },
            ],
        },
        Question {
            title: "烦躁不安",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我现在没有比过去更加烦躁",
                    point: 0,
                },
                QuestionOption {
                    text: "我现在比过去更容易烦躁",
                    point: 1,
                },
                QuestionOption {
                    text: "我非常烦躁或不安，很难保持安静",
                    point: 2,
                },
                QuestionOption {
                    text: "我非常烦躁不安，必须不停走动或做事情",
                    point: 3,
                },
            ],
        },
        Question {
            title: "失去兴趣",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我对其他人或活动没有失去兴趣",
                    point: 0,
                },
                QuestionOption {
                    text: "和过去相比，我对其他人或事的兴趣减少了",
                    point: 1,
                },
                QuestionOption {
                    text: "我失去了对其他人或事的大部分兴趣",
                    point: 2,
                },
                QuestionOption {
                    text: "任何事情都很难引起我的兴趣",
                    point: 3,
                },
            ],
        },
        Question {
            title: "犹豫不决",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我现在能和过去一样作决定",
                    point: 0,
                },
                QuestionOption {
                    text: "我现在作决定比以前困难",
                    point: 1,
                },
                QuestionOption {
                    text: "我作决定比以前困难了很多",
                    point: 2,
                },
                QuestionOption {
                    text: "我作任何决定都很困难",
                    point: 3,
                },
            ],
        },
        Question {
            title: "无价值感",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我不觉得自己没有价值",
                    point: 0,
                },
                QuestionOption {
                    text: "我认为自己不如过去有价值或有用",
                    point: 1,
                },
                QuestionOption {
                    text: "我觉得自己不如别人有价值",
                    point: 2,
                },
                QuestionOption {
                    text: "我觉得自己毫无价值",
                    point: 3,
                },
            ],
        },
        Question {
            title: "精力下降",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我和过去一样有精力",
                    point: 0,
                },
                QuestionOption {
                    text: "我不如从前有精力",
                    point: 1,
                },
                QuestionOption {
                    text: "我没有精力做很多事情",
                    point: 2,
                },
                QuestionOption {
                    text: "我做任何事情都没有足够的精力",
                    point: 3,
                },
            ],
        },
        Question {
            title: "睡眠变化",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没有感觉到睡眠有什么变化",
                    point: 0,
                },
                QuestionOption {
                    text: "我比平时睡得多一些",
                    point: 1,
                },
                QuestionOption {
                    text: "我比平时睡得少一些",
                    point: 1,
                },
                QuestionOption {
                    text: "我比平时睡得多很多",
                    point: 2,
                },
                QuestionOption {
                    text: "我比平时睡得少很多",
                    point: 2,
                },
                QuestionOption {
                    text: "我一天大部分时间都在睡觉",
                    point: 3,
                },
                QuestionOption {
                    text: "我比平时早醒 1–2 小时，而且无法再入睡",
                    point: 3,
                },
            ],
        },
        Question {
            title: "易激惹",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我并不比过去容易发火",
                    point: 0,
                },
                QuestionOption {
                    text: "与过去相比，我比较容易发火",
                    point: 1,
                },
                QuestionOption {
                    text: "与过去相比，我非常容易发火",
                    point: 2,
                },
                QuestionOption {
                    text: "我现在随时都很容易发火",
                    point: 3,
                },
            ],
        },
        Question {
            title: "食欲变化",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没有感觉到食欲有什么变化",
                    point: 0,
                },
                QuestionOption {
                    text: "我的食欲比平时差一些",
                    point: 1,
                },
                QuestionOption {
                    text: "我的食欲比平时好一些",
                    point: 1,
                },
                QuestionOption {
                    text: "我的食欲比平时差很多",
                    point: 2,
                },
                QuestionOption {
                    text: "我的食欲比平时好很多",
                    point: 2,
                },
                QuestionOption {
                    text: "我完全没有食欲",
                    point: 3,
                },
                QuestionOption {
                    text: "我总是很想吃东西",
                    point: 3,
                },
            ],
        },
        Question {
            title: "注意力下降",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我和过去一样可以集中精神",
                    point: 0,
                },
                QuestionOption {
                    text: "我无法像过去一样集中精神",
                    point: 1,
                },
                QuestionOption {
                    text: "任何事情都很难让我长时间集中精神",
                    point: 2,
                },
                QuestionOption {
                    text: "任何事情都无法让我集中精神",
                    point: 3,
                },
            ],
        },
        Question {
            title: "疲劳",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没觉得比过去更易累或乏力",
                    point: 0,
                },
                QuestionOption {
                    text: "我比过去更容易累或乏力",
                    point: 1,
                },
                QuestionOption {
                    text: "因为太累或者太乏力，许多过去常做的事情不能做了",
                    point: 2,
                },
                QuestionOption {
                    text: "因为太累或者太乏力，大多数过去常做的事情都不能做了",
                    point: 3,
                },
            ],
        },
        Question {
            title: "性兴趣下降",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "我没觉得最近对性的兴趣有什么变化",
                    point: 0,
                },
                QuestionOption {
                    text: "我对性的兴趣比过去少了",
                    point: 1,
                },
                QuestionOption {
                    text: "现在我对性的兴趣少多了",
                    point: 2,
                },
                QuestionOption {
                    text: "我对性的兴趣已经完全丧失",
                    point: 3,
                },
            ],
        },
    ],
};

#[cfg(test)]
mod tests {
    use super::BECK_DEPRESSION_INVENTORY;

    #[test]
    fn interpretation_ranges_are_contiguous_closed_intervals() {
        let items = BECK_DEPRESSION_INVENTORY.interpretation;
        let mut expected_start: u8 = 0;

        for item in items {
            assert_eq!(item.range[0], expected_start, "区间必须连续、不重叠");
            expected_start = item.range[1] + 1;
        }
    }
}
