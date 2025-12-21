use serde::Serialize;

use crate::scale::ScaleCategory;

use super::{
    Characteristic, HTMLElement, PlainText, QuestionOption, Scale, SentenceItem, Tag, Texts,
};

#[derive(Serialize)]
enum Factor {
    /// 乐群性
    A,
    /// 聪慧性
    B,
    /// 稳定性
    C,
    /// 恃强性
    E,
    /// 兴奋性
    F,
    /// 有恒性
    G,
    /// 敢为性
    H,
    /// 敏感性
    I,
    /// 怀疑性
    L,
    /// 幻想性
    M,
    /// 世故性
    N,
    /// 忧虑性
    O,
    /// 实验性
    Q1,
    /// 独立性
    Q2,
    /// 自律性
    Q3,
    /// 紧张性
    Q4,
}

#[derive(Serialize)]
pub struct Question {
    title: PlainText,
    options: &'static [QuestionOption],
    factor: Option<Factor>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Range {
    Array([u8; 2]),
    Value(u8),
}

type Ranges = &'static [Range; 10];

#[derive(Serialize)]
struct NormItem {
    ranges: Ranges,
    mean: f32,
    standard_deviation: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct Norm {
    a: NormItem,
    b: NormItem,
    c: NormItem,
    e: NormItem,
    f: NormItem,
    g: NormItem,
    h: NormItem,
    i: NormItem,
    l: NormItem,
    m: NormItem,
    n: NormItem,
    o: NormItem,
    q1: NormItem,
    q2: NormItem,
    q3: NormItem,
    q4: NormItem,
}

const NORM: Norm = Norm {
    a: NormItem {
        ranges: &[
            Range::Array([0, 1]),
            Range::Array([2, 3]),
            Range::Array([4, 5]),
            Range::Value(6),
            Range::Array([7, 8]),
            Range::Array([9, 11]),
            Range::Array([12, 13]),
            Range::Value(14),
            Range::Array([15, 16]),
            Range::Array([17, 20]),
        ],
        mean: 9.06,
        standard_deviation: 3.40,
    },
    b: NormItem {
        ranges: &[
            Range::Array([0, 3]),
            Range::Value(4),
            Range::Value(5),
            Range::Value(6),
            Range::Value(7),
            Range::Value(8),
            Range::Value(9),
            Range::Value(10),
            Range::Value(11),
            Range::Array([12, 13]),
        ],
        mean: 7.65,
        standard_deviation: 1.60,
    },
    c: NormItem {
        ranges: &[
            Range::Array([0, 5]),
            Range::Array([6, 7]),
            Range::Array([8, 9]),
            Range::Array([10, 11]),
            Range::Array([12, 13]),
            Range::Array([14, 16]),
            Range::Array([17, 18]),
            Range::Array([19, 20]),
            Range::Array([21, 22]),
            Range::Array([23, 26]),
        ],
        mean: 14.08,
        standard_deviation: 4.11,
    },
    e: NormItem {
        ranges: &[
            Range::Array([0, 2]),
            Range::Array([3, 4]),
            Range::Value(5),
            Range::Array([6, 7]),
            Range::Array([8, 9]),
            Range::Array([10, 12]),
            Range::Array([13, 14]),
            Range::Array([15, 16]),
            Range::Array([17, 18]),
            Range::Array([19, 26]),
        ],
        mean: 9.82,
        standard_deviation: 3.50,
    },
    f: NormItem {
        ranges: &[
            Range::Array([0, 3]),
            Range::Value(4),
            Range::Array([5, 6]),
            Range::Value(7),
            Range::Array([8, 9]),
            Range::Array([10, 12]),
            Range::Array([13, 14]),
            Range::Array([15, 16]),
            Range::Array([17, 18]),
            Range::Array([19, 26]),
        ],
        mean: 10.69,
        standard_deviation: 3.84,
    },
    g: NormItem {
        ranges: &[
            Range::Array([0, 5]),
            Range::Array([6, 7]),
            Range::Array([8, 9]),
            Range::Value(10),
            Range::Array([11, 12]),
            Range::Array([13, 14]),
            Range::Array([15, 16]),
            Range::Value(17),
            Range::Value(18),
            Range::Array([19, 20]),
        ],
        mean: 1.69,
        standard_deviation: 2.85,
    },
    h: NormItem {
        ranges: &[
            Range::Array([0, 1]),
            Range::Value(2),
            Range::Value(3),
            Range::Array([4, 6]),
            Range::Array([7, 8]),
            Range::Array([9, 11]),
            Range::Array([12, 14]),
            Range::Array([15, 16]),
            Range::Array([17, 19]),
            Range::Array([20, 26]),
        ],
        mean: 8.76,
        standard_deviation: 4.95,
    },
    i: NormItem {
        ranges: &[
            Range::Array([0, 5]),
            Range::Value(6),
            Range::Array([7, 8]),
            Range::Value(9),
            Range::Array([10, 11]),
            Range::Array([12, 13]),
            Range::Value(14),
            Range::Array([15, 16]),
            Range::Value(17),
            Range::Array([18, 19]),
        ],
        mean: 11.42,
        standard_deviation: 2.87,
    },
    l: NormItem {
        ranges: &[
            Range::Array([0, 3]),
            Range::Array([4, 5]),
            Range::Value(6),
            Range::Array([7, 8]),
            Range::Array([9, 10]),
            Range::Array([11, 12]),
            Range::Value(13),
            Range::Array([14, 15]),
            Range::Value(16),
            Range::Array([17, 20]),
        ],
        mean: 10.25,
        standard_deviation: 3.05,
    },
    m: NormItem {
        ranges: &[
            Range::Array([0, 5]),
            Range::Array([6, 7]),
            Range::Array([8, 9]),
            Range::Array([10, 11]),
            Range::Array([12, 13]),
            Range::Array([14, 15]),
            Range::Array([16, 17]),
            Range::Array([18, 19]),
            Range::Value(20),
            Range::Array([21, 26]),
        ],
        mean: 13.27,
        standard_deviation: 3.39,
    },
    n: NormItem {
        ranges: &[
            Range::Array([0, 2]),
            Range::Value(3),
            Range::Value(4),
            Range::Array([5, 6]),
            Range::Array([7, 8]),
            Range::Array([9, 10]),
            Range::Value(11),
            Range::Array([12, 13]),
            Range::Value(14),
            Range::Array([15, 20]),
        ],
        mean: 8.21,
        standard_deviation: 2.67,
    },
    o: NormItem {
        ranges: &[
            Range::Array([0, 2]),
            Range::Array([3, 4]),
            Range::Array([5, 6]),
            Range::Array([7, 8]),
            Range::Array([9, 10]),
            Range::Array([11, 12]),
            Range::Array([13, 14]),
            Range::Array([15, 16]),
            Range::Array([17, 18]),
            Range::Array([19, 26]),
        ],
        mean: 10.42,
        standard_deviation: 3.79,
    },
    q1: NormItem {
        ranges: &[
            Range::Array([0, 4]),
            Range::Value(5),
            Range::Array([6, 7]),
            Range::Value(8),
            Range::Array([9, 10]),
            Range::Array([11, 12]),
            Range::Value(13),
            Range::Value(14),
            Range::Value(15),
            Range::Array([16, 20]),
        ],
        mean: 10.15,
        standard_deviation: 2.54,
    },
    q2: NormItem {
        ranges: &[
            Range::Array([0, 5]),
            Range::Array([6, 7]),
            Range::Value(8),
            Range::Array([9, 10]),
            Range::Array([11, 12]),
            Range::Array([13, 14]),
            Range::Value(15),
            Range::Array([16, 17]),
            Range::Value(18),
            Range::Array([19, 20]),
        ],
        mean: 12.26,
        standard_deviation: 2.88,
    },
    q3: NormItem {
        ranges: &[
            Range::Array([0, 4]),
            Range::Array([5, 6]),
            Range::Array([7, 8]),
            Range::Array([9, 10]),
            Range::Array([11, 12]),
            Range::Array([13, 14]),
            Range::Value(15),
            Range::Array([16, 17]),
            Range::Value(18),
            Range::Array([19, 20]),
        ],
        mean: 12.21,
        standard_deviation: 3.41,
    },
    q4: NormItem {
        ranges: &[
            Range::Array([0, 2]),
            Range::Array([3, 4]),
            Range::Array([5, 6]),
            Range::Array([7, 8]),
            Range::Array([9, 11]),
            Range::Array([12, 14]),
            Range::Array([15, 16]),
            Range::Array([17, 19]),
            Range::Array([20, 31]),
            Range::Array([22, 26]),
        ],
        mean: 11.46,
        standard_deviation: 4.79,
    },
};

#[derive(Serialize)]
struct FirstPersonalityFactor {
    factor: Factor,
    name: PlainText,
    characteristic: Characteristic,
    occupations: PlainText,
}

#[derive(Serialize)]
struct SecondPersonalityFactor {
    key: PlainText,
    name: PlainText,
    expression: PlainText,
    characteristic: Characteristic,
}

#[derive(Serialize)]
pub struct Interpretation {
    norm: Norm,
    normal_range: [u8; 2],
    first_personality_factor: &'static [FirstPersonalityFactor; 16],
    second_personality_factor: &'static [SecondPersonalityFactor; 4],
}

const INTRODUCTION: Texts = &[&[
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

const INSTRUCTION: Texts = &[
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

const INTERPRETATION: Interpretation = Interpretation {
    normal_range: [3, 8],
    norm: NORM,
    first_personality_factor: &[
        FirstPersonalityFactor {
            factor: Factor::A,
            name: "乐群性",
            characteristic: Characteristic {
                low: &[
                    "性格偏内向、沉默，不擅长也不喜欢主动与人交往，显得有些冷漠。标准分低于3分的人通常比较固执，坚持自己的想法，不轻易妥协，做事追求高标准、严谨认真，更愿意独自完成工作，关注事情本身而非人际关系，偶尔会对他人或事情细节过于挑剔。",
                ],
                high: &[
                    "性格外向、热情，喜欢和人打交道。标准分超过8分的人，待人亲切友善，合作和适应能力很强，愿意和他人协作完成工作，也喜欢参与、组织各类集体活动；为人大度不斤斤计较，能坦然接受他人的批评建议，即使是初次见面的人，也能很快熟络起来。",
                ],
            },
            occupations: "教师、销售、人力资源、公关等需要频繁与人沟通协作的职业者，通常乐群性偏高；而从事物理研究、电气工程师、数据分析、精密仪器研发等需要专注独立工作的职业者，乐群性往往偏低。前者需要频繁处理人际互动中的各类问题，保持积极的沟通状态；后者则需要高度专注、冷静严谨的工作状态，才能完成高精度的工作要求。",
        },
        FirstPersonalityFactor {
            factor: Factor::B,
            name: "聪慧性",
            characteristic: Characteristic {
                low: &[
                    "抽象思维能力较弱，学习和理解新知识的速度偏慢，不太擅长举一反三、触类旁通。需要注意的是，得分低并不代表“愚笨”，可能是因为情绪状态不稳定、注意力不集中，或是对所接触的领域缺乏兴趣，而非智力本身的问题。",
                ],
                high: &[
                    "头脑灵活，知识面较广，擅长抽象思考和逻辑分析。得分高的人学习能力强，理解和接受新知识快，思维敏捷且判断准确；通常受教育程度较高，身心状态也更稳定，高得分也能反映出良好的认知功能。",
                ],
            },
            occupations: "科研、金融分析、法律、医生等需要较强逻辑思维和学习能力的职业，通常要求聪慧性偏高；而从事前台文员、基础数据录入、后勤保障等重复性、流程化工作的人，若聪慧性过高，可能会因觉得工作内容单调乏味，难以长期保持工作热情。",
        },
        FirstPersonalityFactor {
            factor: Factor::C,
            name: "稳定性",
            characteristic: Characteristic {
                low: &[
                    "情绪容易波动，遇到不顺心的事易烦躁、焦虑。得分低的人面对生活中的挫折和阻碍时，心态不够平和，容易被外界环境影响，出现心神不宁、急躁易怒的情况；长期下来可能会感到身心疲惫，甚至出现失眠、多梦等睡眠问题。需要注意的是，低得分仅反映情绪调节能力较弱，并非等同于患有精神或神经类疾病。",
                ],
                high: &[
                    "情绪稳定、心智成熟，能理性面对生活中的各种问题。得分高的人遇到困难时沉着冷静，处理事情有魄力，也能带动团队的士气；不过有时也可能因为暂时无法解决某些难题，会主动调整心态、自我宽慰。",
                ],
            },
            occupations: "教师、机械工程师、销售、消防员、客服、医护人员等需要频繁应对突发状况、处理各类问题的职业，通常需要较高的情绪稳定性；而作家、自由撰稿人、邮递员、环卫工人等工作节奏相对自主的职业，即便情绪稳定性偏低，对工作的影响也较小。",
        },
        FirstPersonalityFactor {
            factor: Factor::E,
            name: "恃强性",
            characteristic: Characteristic {
                low: &[
                    "性格谦逊、随和，愿意配合他人的想法和安排，不喜欢争执。得分低的人往往行事温顺，容易迁就他人，有时会缺乏自信，即便自身条件不错，也可能产生“不如别人”的消极感受；这种心态只是自信心不足的表现，并非精神疾病的特征。",
                ],
                high: &[
                    "性格强势、独立，有自己的主见且不愿轻易妥协。得分高的人自信心强，甚至有些自我肯定过度，做事果断但有时会显得武断；不愿被他人支配，甚至会主动挑战权威或试图主导他人。",
                ],
            },
            occupations: "企业管理者、创业者、团队负责人、消防员、飞行员等需要独立决策、敢于担当的职业，通常恃强性偏高；而行政助理、客服、幼教等需要耐心配合他人的职业，恃强性往往偏低。人格特征与性别无必然关联，不存在“男性普遍高于女性”的情况。",
        },
        FirstPersonalityFactor {
            factor: Factor::F,
            name: "兴奋性",
            characteristic: Characteristic {
                low: &[
                    "性格严肃、冷静，话不多，做事谨慎克制。得分低的人行为偏拘谨，习惯先思考再表达，不轻易发表意见，偶尔会显得消极、情绪偏低；有时会因过度深思熟虑而显得固执，不过在工作中通常认真负责、值得信赖。",
                ],
                high: &[
                    "性格活泼开朗，心态放松，随遇而安。得分高的人乐观健谈，对人对事充满热情，情感表达直接；但有时容易冲动，做事缺乏周密考虑，行为偶尔会显得随性多变。",
                ],
            },
            occupations: "企业行政管理人员、市场营销、主播、公关、竞选候选人等需要快速调动情绪、善于沟通感染他人的职业，通常兴奋性偏高；而实验技术员、质检人员、数据分析员等需要保持冷静专注的职业，兴奋性偏低也不影响工作表现。",
        },
        FirstPersonalityFactor {
            factor: Factor::G,
            name: "有恒性",
            characteristic: Characteristic {
                low: &[
                    "做事更看重实际效果，不太拘泥于规则和长远目标。得分低的人缺乏长期坚持的动力，对规则和责任的重视度偏低，更倾向于灵活解决问题，偶尔会为了达成短期目标忽视规则，但并非刻意违法违规；这类人解决实际问题时效率较高，不喜欢在形式上浪费时间。",
                ],
                high: &[
                    "有责任心，做事有始有终，坚守原则和底线。得分高的人认真细致，对自己的行为有明确的是非判断，无论工作还是生活都能坚持既定目标；交友也更倾向于踏实努力的人，相对不喜欢过于浮夸、爱开玩笑的类型。",
                ],
            },
            occupations: "社团负责人、企业管理者、警察、教师、医护人员等需要高度责任感和规则意识的职业，有恒性通常偏高；而犯罪人员往往无视规则和责任，有恒性得分极低。对于销售、创业等需要灵活变通的职业，有恒性偏低未必是劣势，反而能更快适应变化。",
        },
        FirstPersonalityFactor {
            factor: Factor::H,
            name: "敢为性",
            characteristic: Characteristic {
                low: &[
                    "性格腼腆、缺乏自信，在陌生环境或人群中容易感到紧张不自在。得分低的人不擅长当众发言，也不愿主动和陌生人交流，面对新机会时习惯观望，有时会因过度关注自身感受，忽略周围的重要信息和活动。",
                ],
                high: &[
                    "勇敢果断，敢于尝试新事物，不怕冒险。得分高的人表达直接、不怯场，面对困难能保持坚韧的态度；但有时会因过于大胆而忽略细节，导致不必要的失误，也可能因为精力旺盛，显得过于主动（比如对他人过度热情）。",
                ],
            },
            occupations: "敢为性通常会随着人生阅历的增加有所提升。消防员、飞行员、探险从业者、销售、创业者等需要敢于冒险、快速决策的职业，敢为性偏高；而行政文员、档案管理员、后台运营等偏向安稳、流程化的职业，敢为性往往偏低。团队负责人通常需要一定的敢为性，但并非越高越好，需结合谨慎性。",
        },
        FirstPersonalityFactor {
            factor: Factor::I,
            name: "敏感性",
            characteristic: Characteristic {
                low: &[
                    "性格理智、务实，做事靠自己，不轻易被情绪左右。得分低的人处理问题时客观冷静，有主见且独立，注重实际效果；但有时会过于理性，显得不够共情，让人觉得有些“冷漠”。",
                ],
                high: &[
                    "心思细腻、情感丰富，容易被外界事物打动。得分高的人内心柔软，喜欢艺术、富有想象力；但有时会过于感性，做事缺乏耐心和持久力，不喜欢繁琐、体力型的工作，也不太能接受过于直白粗俗的表达方式；在团队中若想法过于理想化，可能会影响整体工作效率。敏感性与性别无关，“女性化”是刻板印象。",
                ],
            },
            occupations: "室内设计师、音乐家、艺术家、文案策划、心理咨询师等需要共情能力和想象力的职业，敏感性通常偏高；而电气工程师、外科医生、统计师、机械维修师等需要理性客观、专注实操的职业，敏感性往往偏低。",
        },
        FirstPersonalityFactor {
            factor: Factor::L,
            name: "怀疑性",
            characteristic: Characteristic {
                low: &[
                    "性格随和、容易信任他人，不喜欢猜忌和竞争。得分低的人待人真诚，愿意配合他人，善于体谅对方的感受，在团队中能友好协作。",
                ],
                high: &[
                    "性格多疑，有自己的主见且不易改变。得分高的人对他人的动机容易产生怀疑，与人相处时会比较计较得失，有时会忽略他人的利益和感受；但这种特征也能让他们更谨慎地判断事物，不易被欺骗。",
                ],
            },
            occupations: "在团队中，怀疑性低的人更愿意以集体利益为先，是可靠的合作者；但怀疑性过高则容易猜忌他人，影响团队协作，反而不利于事情推进。机械工程师、技术工人、养老护理员、医护人员等需要信任和协作的职业，怀疑性通常偏低；而警察、风控专员、审计、行政管理人员等需要警惕性和判断力的职业，怀疑性往往偏高。",
        },
        FirstPersonalityFactor {
            factor: Factor::M,
            name: "幻想性",
            characteristic: Characteristic {
                low: &[
                    "性格务实、循规蹈矩，做决策前会充分考虑现实条件。得分低的人做事稳妥、不莽撞，遇到紧急情况能保持冷静；但有时会过于看重实际，缺乏想象力，生活和工作中显得比较单调乏味。",
                ],
                high: &[
                    "富有想象力，思维跳脱，不喜欢被常规束缚。得分高的人创造力强，做事更跟随自己的兴趣和想法；但有时会忽略生活和工作中的细节，想法过于理想化甚至冲动，容易被他人误解为“不切实际”。",
                ],
            },
            occupations: "艺术家、作家、创意策划、科研人员（尤其是基础研究）等需要创造力和想象力的职业，幻想性通常偏高；而会计、物流管理、建筑施工、客服等需要脚踏实地、注重细节的职业，幻想性往往偏低。",
        },
        FirstPersonalityFactor {
            factor: Factor::N,
            name: "世故性",
            characteristic: Characteristic {
                low: &[
                    "性格坦率、单纯，心里想什么就说什么，不擅长掩饰自己。得分低的人想法简单，做事容易凭感觉，待人友善不争抢，容易满足；但有时会因过于直白显得幼稚、不懂变通，甚至让人觉得不够周到。",
                ],
                high: &[
                    "为人精明、处事老练，懂得根据场合调整自己的言行。得分高的人能冷静客观地分析问题，待人接物得体周到；但有时会显得过于圆滑，甚至让人觉得有些“精明过头”。",
                ],
            },
            occupations: "科学家、工程师、飞行员、企业管理者、商务谈判人员等需要冷静分析、处事周全的职业，世故性通常偏高；而护士、幼教、社工、心理咨询师等需要真诚共情的职业，世故性往往偏低——这类职业更看重坦率和同理心，过高的世故性反而容易让人觉得缺乏真诚。",
        },
        FirstPersonalityFactor {
            factor: Factor::O,
            name: "忧虑性",
            characteristic: Characteristic {
                low: &[
                    "心态乐观、自信，对自己解决问题的能力有把握。得分低的人沉着稳定，有安全感，能很好地适应社会环境；但有时会因过于自信，忽略他人的感受，缺乏共情，从而引起别人的不满。",
                ],
                high: &[
                    "容易感到焦虑、担忧，遇事总往坏处想。得分高的人常常觉得生活中有很多不如意，情绪容易低落悲观，有患得患失的心态，也会因自我怀疑而不敢主动与人交往。需要注意的是，高得分仅反映情绪上的忧虑倾向，并非等同于患有神经或精神类疾病。",
                ],
            },
            occupations: "从事餐饮服务员等服务行业基层岗位、基础行政文员等压力大且发展空间有限的职业，或是长期处于职业瓶颈期的文字工作者（如作家、编辑），忧虑性可能偏高；而职业运动员、电气工程师、消防员、护士等需要稳定心态和抗压能力的职业，忧虑性通常偏低。",
        },
        FirstPersonalityFactor {
            factor: Factor::Q1,
            name: "实验性",
            characteristic: Characteristic {
                low: &[
                    "偏向保守，认可传统的观念和做事方式。得分低的人愿意接受经过时间检验的权威观点，不喜欢轻易尝试新事物、新方法，对新思想和变化的接受度低；但这也意味着他们做事更稳妥，不易出错，并非“老顽固”或“落伍”。",
                ],
                high: &[
                    "思想开放，喜欢挑战传统和现有规则。得分高的人愿意重新审视既有的理论和做法，乐于接受前沿的思想和行为方式，不轻易下结论；知识面广，愿意尝试新事物来丰富自己的经历。",
                ],
            },
            occupations: "企业创新岗、科研人员、政策研究人员、前沿领域创业者等需要突破思维定式的职业，实验性通常偏高；而护士、技术工人、传统行业从业者等需要遵循既定流程和规范的职业，实验性往往偏低。实验性低仅反映思维偏向保守，与是否患有精神疾病无关。",
        },
        FirstPersonalityFactor {
            factor: Factor::Q2,
            name: "独立性",
            characteristic: Characteristic {
                low: &[
                    "习惯依赖他人，做事喜欢和人协作，不愿独自承担。得分低的人容易放弃自己的想法去迎合他人，需要得到团队的认可来建立自信；但这并不代表他们真的喜欢社交，只是更害怕独自面对问题。",
                ],
                high: &[
                    "独立自强，有主见，能自己做决策并承担后果。得分高的人习惯独自完成工作计划，不依赖他人，也不会被外界的评价左右；不喜欢控制别人，也不在意是否能获得所有人的好感，更看重内心的自洽。",
                ],
            },
            occupations: "科学家、企业高管、自由职业者、独立顾问等需要独立决策、自主推进工作的职业，独立性通常偏高；而行政助理、客服、团队助理等需要配合他人工作的职业，独立性偏低也能胜任。独立性低不代表缺乏随机应变能力，只是更擅长协作而非独断。",
        },
        FirstPersonalityFactor {
            factor: Factor::Q3,
            name: "自律性",
            characteristic: Characteristic {
                low: &[
                    "自我约束能力较弱，做事容易随心所欲，不注重规则和他人感受。得分低的人难以克制自己的情绪和行为，既不遵守既定的礼仪规范，也很少考虑别人的需求，内心容易陷入矛盾，生活和工作中也可能出现适应不良的情况。",
                ],
                high: &[
                    "自我要求高，自律性强，能很好地控制自己的情绪和行为。得分高的人言行一致，做事有原则，既能保持自尊，也能赢得他人的尊重；但有时会因过于坚持自己的原则，显得有些固执。",
                ],
            },
            occupations: "企业管理者、团队负责人、军人、生产班组长等需要自我约束并带领团队遵守规则的职业，自律性通常偏高；电气工程师、生产一线员工等需要严格遵守操作规范的职业，高自律性也有助于工作的顺利开展。",
        },
        FirstPersonalityFactor {
            factor: Factor::Q4,
            name: "紧张性",
            characteristic: Characteristic {
                low: &[
                    "心态平和、放松，不喜欢给自己太大压力。得分低的人容易满足，能保持内心的平静；但有时会因过于安逸，缺乏进取心和行动力，显得有些懒散。",
                ],
                high: &[
                    "容易感到紧张、焦虑，心态难以放松。得分高的人缺乏耐心，常常心神不宁，即使休息也难以缓解疲惫；对自己和周围的人、事缺乏信心，生活中总处于紧绷状态，容易有焦虑感。",
                ],
            },
            occupations: "从事餐饮服务员、流水线工人等工作强度大、发展空间有限，或是觉得自身能力未得到发挥的人，紧张性可能偏高；而教师、自由职业者、园艺师等工作节奏相对舒缓的职业，紧张性通常偏低。",
        },
    ],
    second_personality_factor: &[
        SecondPersonalityFactor {
            key: "X1",
            name: "适应与焦虑型",
            expression: "(38+2*L+3*O+4*Q4-2*C-2*H-2*Q2)/10",
            characteristic: Characteristic {
                low: &[
                    "生活和工作状态整体适应良好，心态满足，能完成自己在意的目标和事情。但有时面对难度较大的工作，可能缺乏坚持下去的毅力，容易知难而退，不愿付出过多努力。",
                ],
                high: &[
                    "对当前的生活、工作状态感到不满，容易产生焦虑情绪；长期处于这种状态，不仅会影响心情，还可能导致睡眠、食欲等身体方面的不适。",
                ],
            },
        },
        SecondPersonalityFactor {
            key: "X2",
            name: "内向与外向型",
            expression: "(2*A+3*E+4*F+5*H-2*Q2-11)/10",
            characteristic: Characteristic {
                low: &[
                    "性格偏内向，更习惯独处，在人际交往中会有所克制，不喜欢过度社交。这种特质适合从事科研、精密仪器操作、文字编辑等需要专注、细致的工作。",
                ],
                high: &[
                    "性格偏外向，开朗健谈，擅长与人交往，不喜欢被拘束。这种特质适合从事销售、公关、市场营销、商务谈判等需要频繁沟通的工作。",
                ],
            },
        },
        SecondPersonalityFactor {
            key: "X3",
            name: "感情用事与安详机警型",
            expression: "(77+2*C+2*E+2*F+2*N-4*A-6*I-2*M)/10",
            characteristic: Characteristic {
                low: &[
                    "情感丰富细腻，但容易被情绪困扰，自信心不足，偶尔会情绪低落。对生活中的小细节很敏感，性格温和，注重生活品质；但做决策时会反复思考、顾虑过多，容易犹豫不决。",
                ],
                high: &[
                    "有强烈的事业心，做事果断、有魄力，充满进取精神，精力充沛且行动迅速。但容易忽略生活中的细节，只关注核心问题；有时会因考虑不周、过于冲动而贸然行事，不计后果。"
                ],
            },
        },
        SecondPersonalityFactor {
            key: "X4",
            name: "怯懦与果敢型",
            expression: "(4*E+3*M+4*Q1+4*Q2-3*A-2*G)/10",
            characteristic: Characteristic {
                low: &[
                    "性格偏怯懦，习惯顺从他人，做事依赖别人的安排，缺乏主动性。为了获得他人的认可，容易事事迁就，不敢坚持自己的想法，显得被动。",
                ],
                high: &[
                    "性格果敢、独立，有魄力，敢于表达自己的想法和立场。会主动寻找能发挥自身能力的机会，展现自己的创造力；但有时可能会表现出较强的攻击性，过于追求自身利益。"
                ],
            },
        },
    ],
};

pub const SIXTEEN_PERSONALITY_FACTORS: Scale<Interpretation, Question> = Scale {
    name: "卡特尔16种人格因素问卷",
    abbreviation: "16PF",
    primary_category: ScaleCategory::Personality,
    related_categories: Some(&[
        ScaleCategory::CognitionAndAbility,
        ScaleCategory::Interpersonal,
        ScaleCategory::Wellbeing,
    ]),
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: Some(&[
        "卡特尔认为人格的基本结构元素是特质。特质的种类很多，有人类共同的特质，有各人独有的特质。有的特质决定于遗传，有的决定于环境；有的与动机有关，有的则与能力和气质有关。若从向度来分，可分为四种向度。",
        "（1）表面特质与根源特质",
        "表面特质是指一群看起来似乎聚在一起的特征或行为，即可以观察到的各种行为表现。它们之间是具有相关性的。根源特质是行为的最终根源和原因。它们是堆砌成人格的砖块。每一个根源特质控制着一簇表面特质。透过对许多表面特质的因素分析便可找到它们所属的根源特质。", 
        "（2）能力特质、气质特质与动力特质", 
        "能力特质与认知和思维有关，在16PF中主要由智慧因素（B因素）表示，决定工作的效率。行为的情绪、情感方面则表明了气质和风格的特质。动力特质与行为的意志和动机方面有关。", 
        "（3）个别特质和共同特质", 
        "卡特尔赞同阿尔波特的观点，认为人类存在着所有社会成员共同具有的特质（共同特质）和个体独有的特质，即个别特质（指表面特质）。虽有共同特质，但共同特质在各个成员身上的强度却各不相同（指根源特质）。", 
        "（4）体质特质和环境塑造特质", 
        "卡特尔认为16PF中有些特质是由遗传决定的，称为体质根源特质，而有些特质来源于经验，因此称为环境塑造特质。卡特尔认为在人格的成长和发展中遗传与环境都有影响。他十分重视遗传的重要性，曾试图决定每一根源特质的特殊遗传成分。", 
        "卡特尔在其人格的解释性理论构想的基础上编制了16种人格因素问卷，从16个方面描述个体的人格特征。这16个因素或分量表的名称和符号分别是：乐群性（A）、聪慧性（B）、稳定性（C）、恃强性（E）、兴奋性（F）、有恒性（G）、敢为性（H）、敏感性（I）、怀疑性（L）、幻想性（M）、世故性（N）、忧虑性（O）、实验性（Q1）、独立性（Q2）、自律性（Q3）、紧张性（Q4）。有关这16个因素的说明可详见测验指导书。",
    ]),
    references: None,
    formula_mode: None,
    warning: Some("本量表仅适用 16 岁以上的人群。"),
    tags: Tag{ info: Some(&[ "人格"]), normal: None, warning: Some(&["16+"]), error: None },
    interpretation: INTERPRETATION,
    questions: &[
        Question {
            title: "我很明了本测试的说明：",
            factor: None,
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 0,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我对本测试的每一个问题，都能做到诚实地回答：",
            factor: None,
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 0,
                },
                QuestionOption {
                    text: "不同意",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果我有机会的话，我愿意：",
            factor: Some(Factor::A),
            options: &[
                QuestionOption {
                    text: "到一个繁华的城市去旅行",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "浏览清静的山区",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我有能力应付各种困难：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "即使是关在铁笼里的猛兽，我见了也会感到惴惴不安：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我总是不敢大胆批评别人的言行：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "有时如此",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我的思想似乎：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "比较先进",
                    point: 2,
                },
                QuestionOption {
                    text: "一般",
                    point: 1,
                },
                QuestionOption {
                    text: "比较保守",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我不擅长说笑话，讲有趣的事：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "当我见到邻居或新友争吵时，我总是：",
            factor: Some(Factor::G),
            options: &[
                QuestionOption {
                    text: "任其自己解决",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "予以劝解",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在群众集会时，我：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "谈吐自如",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "保持沉默",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我愿意做一个：",
            factor: Some(Factor::I),
            options: &[
                QuestionOption {
                    text: "建筑工程师",
                    point: 0,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "社会科学研究者",
                    point: 2,
                },
            ],
        },
        Question {
            title: "阅读时，我喜欢选读：",
            factor: Some(Factor::I),
            options: &[
                QuestionOption {
                    text: "自然科学书籍",
                    point: 0,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "政治理论书籍",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我认为很多人都有些心理不正常，只是他们不愿承认：",
            factor: Some(Factor::L),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我希望我的爱人擅长交际，无须具有文艺才能：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "对于性情急躁、爱发脾气的人，我仍能以礼相待：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "受人侍奉时我常常局促不安：",
            factor: Some(Factor::N),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在从事体力或脑力劳动之后，我总是需要有比别人更多的休息时间，才能保持工作效率：",
            factor: Some(Factor::N),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "半夜醒来，我常常为种种不安而不能入睡：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "常常如此",
                    point: 2,
                },
                QuestionOption {
                    text: "有时如此",
                    point: 1,
                },
                QuestionOption {
                    text: "极少如此",
                    point: 0,
                },
            ],
        },
        Question {
            title: "事情进行得不顺利时，我常常急得涕泪交流：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "常常如此",
                    point: 0,
                },
                QuestionOption {
                    text: "有时如此",
                    point: 1,
                },
                QuestionOption {
                    text: "极少如此",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我以为只要双方同意可离婚，可以不受传统观念的束缚：",
            factor: Some(Factor::Q1),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我对人或物的兴趣都很容易改变：",
            factor: Some(Factor::Q1),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "工作中，我愿意：",
            factor: Some(Factor::Q2),
            options: &[
                QuestionOption {
                    text: "和别人合作",
                    point: 0,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "自己单独进行",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我常常无缘无故地自言自语：",
            factor: Some(Factor::Q3),
            options: &[
                QuestionOption {
                    text: "常常如此",
                    point: 0,
                },
                QuestionOption {
                    text: "偶尔如此",
                    point: 1,
                },
                QuestionOption {
                    text: "从不如此",
                    point: 2,
                },
            ],
        },
        Question {
            title: "无论是工作、饮食或外出游览，我总是：",
            factor: Some(Factor::Q3),
            options: &[
                QuestionOption {
                    text: "匆匆忙忙不能尽兴",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "从容不迫",
                    point: 2,
                },
            ],
        },
        Question {
            title: "又使我怀疑别人是否对我的言行真正有兴趣：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果我在工厂里工作，我愿做：",
            factor: Some(Factor::A),
            options: &[
                QuestionOption {
                    text: "技术科的工作",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "宣传科的工作",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在阅读时我愿阅读：",
            factor: Some(Factor::A),
            options: &[
                QuestionOption {
                    text: "有关太空旅行的书籍",
                    point: 0,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "有关家庭教育的书籍",
                    point: 2,
                },
            ],
        },
        Question {
            title: "本题后面列出三个单词，哪个与其他两个单词不同类",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "狗",
                    point: 0,
                },
                QuestionOption {
                    text: "石头",
                    point: 1,
                },
                QuestionOption {
                    text: "牛",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果我能到一个新的环境，我要：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "把生活安排得和从前不一样",
                    point: 0,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "和从前一样",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在一生中，我总觉得我能达到我所预期的目标：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "当我说谎时总觉得内心羞愧不敢正视对方：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "假使我手里拿着一把装着子弹的手枪，我必须把子弹拿出来才能安心：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "多数人认为我是一个说话风趣的人：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果人们知道我内心的成见，他们会大吃一惊：",
            factor: Some(Factor::G),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在公共场合，如果我突然成为大家注意的中心，就会感到局促不安：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我总喜欢参加规模庞大的晚会或集会：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在学科中，我喜欢：",
            factor: Some(Factor::I),
            options: &[
                QuestionOption {
                    text: "音乐",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "手工劳动",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我常常怀疑那些出乎我意料的对我过于友善的人的动机是否诚实：",
            factor: Some(Factor::L),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我愿意把我的生活安排得像一个：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "艺术家",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "会计师",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我认为目前所需要的是：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "多出现一些改造世界的理想家",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "脚踏实地的实干家",
                    point: 0,
                },
            ],
        },
        Question {
            title: "有时候我觉得我需要剧烈的体力劳动：",
            factor: Some(Factor::N),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我愿意跟有教养的人来往而不愿意同粗鲁的人交往：",
            factor: Some(Factor::N),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在处理一些必须凭借智慧的事务中：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "我的亲人表现得比一般人差",
                    point: 2,
                },
                QuestionOption {
                    text: "普通",
                    point: 1,
                },
                QuestionOption {
                    text: "我的亲人表现得超人一等",
                    point: 0,
                },
            ],
        },
        Question {
            title: "当领导召见我时，我：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "觉得可以趁机提出建议",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "总怀疑自己做错事",
                    point: 2,
                },
            ],
        },
        Question {
            title: "如果待遇优厚，我愿意做护理精神病人的工作：",
            factor: Some(Factor::Q1),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "读报时，我喜欢读：",
            factor: Some(Factor::Q1),
            options: &[
                QuestionOption {
                    text: "当今世界的基本问题",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "地方新闻",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在接受困难任务时，我总是：",
            factor: Some(Factor::Q2),
            options: &[
                QuestionOption {
                    text: "有独立完成的信心",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "希望有别人帮助和指导",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在游览时，我宁愿观看一个画家的写生，也不愿听大家的辩论：",
            factor: Some(Factor::Q3),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我的神经脆弱，稍有点刺激就会战栗：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "时常如此",
                    point: 2,
                },
                QuestionOption {
                    text: "有时如此",
                    point: 1,
                },
                QuestionOption {
                    text: "从不如此",
                    point: 0,
                },
            ],
        },
        Question {
            title: "早晨起来，常常感到疲乏不堪：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果待遇相同，我愿选做：",
            factor: Some(Factor::A),
            options: &[
                QuestionOption {
                    text: "森林管理员",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "中小学教员",
                    point: 2,
                },
            ],
        },
        Question {
            title: "每逢过年过节或亲友结婚时，我：",
            factor: Some(Factor::A),
            options: &[
                QuestionOption {
                    text: "喜欢赠送礼品",
                    point: 2,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "不愿相互送礼",
                    point: 0,
                },
            ],
        },
        Question {
            title: "本题列有三个数字，哪个数字与其他两个数字不同类：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "5",
                    point: 0,
                },
                QuestionOption {
                    text: "2",
                    point: 1,
                },
                QuestionOption {
                    text: "7",
                    point: 0,
                },
            ],
        },
        Question {
            title: "猫和鱼就像牛和：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "牛奶",
                    point: 0,
                },
                QuestionOption {
                    text: "木材",
                    point: 1,
                },
                QuestionOption {
                    text: "盐",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我在小学时敬佩的老师，到现在仍然值得我敬佩：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我觉得我确实有一些别人所不及的优良品质：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "根据我的能力，即使让我做一些平凡的工作，我也会安心的：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我喜欢看电影或参加其他娱乐活动的次数：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "比一般人多",
                    point: 2,
                },
                QuestionOption {
                    text: "和一般人相同",
                    point: 1,
                },
                QuestionOption {
                    text: "比一般人少",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我喜欢从事需要精密技术的工作：",
            factor: Some(Factor::G),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在有威望有地位的人面前，我总是较为局促谨慎：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "对于我来说在大众面前演讲或表演，是一件难事：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我愿意：",
            factor: Some(Factor::I),
            options: &[
                QuestionOption {
                    text: "指挥几个人工作",
                    point: 0,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "和同志们一起工作",
                    point: 2,
                },
            ],
        },
        Question {
            title: "即使我做了一件让别人笑话的事，我也能坦然处之：",
            factor: Some(Factor::L),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我认为没有人会幸灾乐祸地希望我遇到困难：",
            factor: Some(Factor::L),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "一个人应该考虑人生的真正意义：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我喜欢去处理被别人弄得一塌糊涂的工作：",
            factor: Some(Factor::N),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "当我非常高兴时，总有一种“好景不长”的感受：",
            factor: Some(Factor::N),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在一般困难情境中，我总能保持乐观：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "迁居是一件极不愉快的事：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在年轻的时候，当我和父母的意见不同时：",
            factor: Some(Factor::Q1),
            options: &[
                QuestionOption {
                    text: "保留自己的意见",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "接受父母的意见",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我希望把我的家庭：",
            factor: Some(Factor::Q2),
            options: &[
                QuestionOption {
                    text: "建设成适合自身活动和娱乐的地方",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "成为邻里交往活动的一部分",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我解决问题时，多借助于：",
            factor: Some(Factor::Q2),
            options: &[
                QuestionOption {
                    text: "个人独立思考",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "和别人互相讨论",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在需要当机立断时，我总是：",
            factor: Some(Factor::Q3),
            options: &[
                QuestionOption {
                    text: "镇静地运用理智",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "常常紧张兴奋",
                    point: 0,
                },
            ],
        },
        Question {
            title: "最近在一两件事情上，我觉得我是无辜受累的：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我善于控制我的表情：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "如果待遇相同，我愿做一个：",
            factor: Some(Factor::A),
            options: &[
                QuestionOption {
                    text: "化学研究工作者",
                    point: 0,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "旅行社经理",
                    point: 2,
                },
            ],
        },
        Question {
            title: "以“惊讶”与“新奇”搭配为例，认为“惧怕”与（ ）搭配：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "勇敢",
                    point: 0,
                },
                QuestionOption {
                    text: "焦虑",
                    point: 1,
                },
                QuestionOption {
                    text: "恐怖",
                    point: 2,
                },
            ],
        },
        Question {
            title: "本题后面列出三个分数，哪一个数与其他两个分数不同类：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "3/7",
                    point: 0,
                },
                QuestionOption {
                    text: "3/9",
                    point: 1,
                },
                QuestionOption {
                    text: "3/11",
                    point: 2,
                },
            ],
        },
        Question {
            title: "不知为什么，有些人总是回避或冷淡我：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我虽然好意待人，但常常得不到好报：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我不喜欢争强好胜的人：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "和一般人相比，我的朋友的确太少：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "不在万不得已的情况下，我总是回避参加应酬性的活动：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我认为对领导逢迎得当比工作表现更重要：",
            factor: Some(Factor::G),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "参加竞赛时，我总是看重竞赛的活动，而不计较其成败：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "总是如此",
                    point: 0,
                },
                QuestionOption {
                    text: "一般如此",
                    point: 1,
                },
                QuestionOption {
                    text: "偶然如此",
                    point: 2,
                },
            ],
        },
        Question {
            title: "按照我个人的意愿，我希望做的工作是：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "有固定而可靠的工资收入",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "工资高低应随我的工作表现而随时调整",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我愿意阅读：",
            factor: Some(Factor::I),
            options: &[
                QuestionOption {
                    text: "军事与政治的实事记载",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "富有情感的幻想的作品",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我认为有许多人之所以不敢犯罪，其主要原因是怕被惩罚：",
            factor: Some(Factor::L),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我的父母从来不严格要求我事事顺从：",
            factor: Some(Factor::L),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "“百折不挠，再接再厉”的精神常常被人们所忽略：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "当有人对我发火时，我总是：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "设法使他镇静下来",
                    point: 2,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "自己也会发起火来",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我希望人们都要友好相处：",
            factor: Some(Factor::N),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "不论是在极高的屋顶上，还是在极深的隧道中，我很少感到胆怯不安：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "只要没有过错，不管别人怎么说，我总能心安理得：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我认为凡是无法用理智来解决的问题，有时就不得不靠强权处理：",
            factor: Some(Factor::Q1),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我在年轻的时候，和异性朋友交往：",
            factor: Some(Factor::Q2),
            options: &[
                QuestionOption {
                    text: "较多",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "较别人少",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我在社团活动中，是一个活跃分子：",
            factor: Some(Factor::Q2),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在人声嘈杂中，我仍能不受干扰，专心工作：",
            factor: Some(Factor::Q3),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在某些心境下，我常常因为困惑陷入空想而将工作搁置下来：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我很少用难堪的语言去刺伤别人的感情：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果让我选择，我宁愿选做：",
            factor: Some(Factor::A),
            options: &[
                QuestionOption {
                    text: "列车员",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "描图员",
                    point: 0,
                },
            ],
        },
        Question {
            title: "“理不胜词”的意思是：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "理不如词",
                    point: 2,
                },
                QuestionOption {
                    text: "理多而词少",
                    point: 1,
                },
                QuestionOption {
                    text: "辞藻华丽而理不足",
                    point: 1,
                },
            ],
        },
        Question {
            title: "以“铁锹”与“挖掘”搭配为例，我认为“刀子”与（   ）搭配：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "琢磨",
                    point: 2,
                },
                QuestionOption {
                    text: "切割",
                    point: 1,
                },
                QuestionOption {
                    text: "铲除",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我在大街上，常常避开我所不愿意打招呼的人：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "极不如此",
                    point: 2,
                },
                QuestionOption {
                    text: "偶然如此",
                    point: 1,
                },
                QuestionOption {
                    text: "有时如此",
                    point: 0,
                },
            ],
        },
        Question {
            title: "当我聚精会神地听音乐时，假使有人在旁边高谈阔论：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "我仍能专心听音乐",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不能专心而感到恼怒",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在课堂上，如果我的意见与老师不同，我常常：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "保持沉默",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "表明自己的看法",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我单独跟异性谈话时，总显得不自然：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我在待人接物方面，的确不太成功：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不完全这样",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "每当做一件困难工作时，我总是：",
            factor: Some(Factor::G),
            options: &[
                QuestionOption {
                    text: "预先做好准备",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "相信到时候总会有办法解决的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在我结交的朋友中，男女各占一半：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我在结交朋友方面：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "结识很多的人",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "维持几个深交的朋友",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我愿意做一个社会科学家，而不愿做一个机械工程师：",
            factor: Some(Factor::I),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果我发现别人的缺点，我常常不顾一切地提出指责：",
            factor: Some(Factor::L),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我喜欢设法影响和我一起工作的同事，使他们能协助我达到所计划的目的：",
            factor: Some(Factor::L),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我喜欢做音乐，或跳舞，或新闻采访等工作：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "当人们表扬我的时候，我总觉得羞愧窘促：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我认为一个国家最需要解决的问题是：",
            factor: Some(Factor::N),
            options: &[
                QuestionOption {
                    text: "政治问题",
                    point: 2,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "道德问题",
                    point: 0,
                },
            ],
        },
        Question {
            title: "有时我会无故地产生一种面临大祸的恐惧：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "有时如此",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我在童年时，害怕黑暗的次数：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "很多",
                    point: 2,
                },
                QuestionOption {
                    text: "不太多",
                    point: 1,
                },
                QuestionOption {
                    text: "几乎没有",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在闲暇的时候，我喜欢：",
            factor: Some(Factor::Q1),
            options: &[
                QuestionOption {
                    text: "看一部历史性的探险小说",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "读一本科学性的幻想小说",
                    point: 2,
                },
            ],
        },
        Question {
            title: "当人们批评我古怪不正常时，我：",
            factor: Some(Factor::Q2),
            options: &[
                QuestionOption {
                    text: "非常气恼",
                    point: 0,
                },
                QuestionOption {
                    text: "有些气恼",
                    point: 1,
                },
                QuestionOption {
                    text: "无所谓",
                    point: 2,
                },
            ],
        },
        Question {
            title: "当来到一个新城市里找地址时，我常常：",
            factor: Some(Factor::Q2),
            options: &[
                QuestionOption {
                    text: "找人问路",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "参考地图",
                    point: 2,
                },
            ],
        },
        Question {
            title: "当朋友声明她要在家休息时，我总是设法怂恿她同我一起到外面去玩：",
            factor: Some(Factor::Q3),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在就寝时，我常常：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "不易入睡",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "极易入睡",
                    point: 0,
                },
            ],
        },
        Question {
            title: "有人烦扰我时，我：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "能不露声色",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "总要说给别人听，以泄愤怒",
                    point: 2,
                },
            ],
        },
        Question {
            title: "如果待遇相同，我愿做一个：",
            factor: Some(Factor::A),
            options: &[
                QuestionOption {
                    text: "律师",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "航海员",
                    point: 0,
                },
            ],
        },
        Question {
            title: "“时间变成了永恒”这是比喻：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "时间过得快",
                    point: 0,
                },
                QuestionOption {
                    text: "忘了时间",
                    point: 0,
                },
                QuestionOption {
                    text: "光阴一去不复返",
                    point: 1,
                },
            ],
        },
        Question {
            title: "本题后的哪一项应接在“×0000××00×××”的后面：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "×0×",
                    point: 0,
                },
                QuestionOption {
                    text: "00×",
                    point: 1,
                },
                QuestionOption {
                    text: "0××",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我不论到什么地方，都能清楚地辨别方向：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我热爱我所学的专业和所从事的工作：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果我急于想借朋友的东西，而朋友又不在家时，我认为不告而取也没有关系：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我喜欢给朋友讲述一些我个人有趣的经历：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我宁愿做一个：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "演员",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "建筑师",
                    point: 0,
                },
            ],
        },
        Question {
            title: "业余时间，我总是做好安排，不使时间浪费：",
            factor: Some(Factor::G),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在和别人交往中，我常常会无缘无故地产生一种自卑感：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "和不熟识的人交谈，对我来说：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "毫不困难",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "是一件难事",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我所喜欢的音乐是：",
            factor: Some(Factor::I),
            options: &[
                QuestionOption {
                    text: "轻松活泼的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "富有感情的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我爱想入非非：",
            factor: Some(Factor::I),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我认为未来20年的世界局势，定将好转：",
            factor: Some(Factor::L),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在童年时，我喜欢阅读：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "神话幻想故事",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "战争故事",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我向来对机械、汽车等发生兴趣：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "即使让我做一个缓刑释放的罪犯的管理人，我也会把工作搞得很好：",
            factor: Some(Factor::N),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我仅仅被认为是一个能够苦干而稍有成就的人而已",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "就是在不顺利的情况下，我仍能保持精神振奋：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我认为节制生育是解决经济与和平问题的重要条件：",
            factor: Some(Factor::Q1),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在工作中，我喜欢独自筹划，不愿受别人干涉：",
            factor: Some(Factor::Q2),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "尽管有的同志和我的意见不和，但仍能跟他搞好团结：",
            factor: Some(Factor::Q3),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我在工作和学习上，总是使自己不粗心大意，不忽略细节：",
            factor: Some(Factor::Q3),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在和人争辩或险遭事故后，我常常表现出震颤、筋疲力尽、不能安心工作：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "未经医生处方，我是从不乱吃药的：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "根据我个人的兴趣，我愿意参加：",
            factor: Some(Factor::A),
            options: &[
                QuestionOption {
                    text: "摄影组织活动",
                    point: 0,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "文娱队活动",
                    point: 2,
                },
            ],
        },
        Question {
            title: "以“星火”与“燎原”搭配为例，我认为“姑息”与（   ）搭配：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "同情",
                    point: 0,
                },
                QuestionOption {
                    text: "养奸",
                    point: 1,
                },
                QuestionOption {
                    text: "纵容",
                    point: 0,
                },
            ],
        },
        Question {
            title: "“钟表”与“时间”的关系犹如“裁缝”与（   ）的关系：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "服装",
                    point: 0,
                },
                QuestionOption {
                    text: "剪刀",
                    point: 0,
                },
                QuestionOption {
                    text: "布料",
                    point: 1,
                },
            ],
        },
        Question {
            title: "生动的梦境，常常干扰我的睡眠：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "经常如此",
                    point: 0,
                },
                QuestionOption {
                    text: "偶然如此",
                    point: 1,
                },
                QuestionOption {
                    text: "从不如此",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我爱打抱不平：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果我要到一个新城市，我将要：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "到处闲逛",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "避免去不安全的地方",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我爱穿朴素的衣服，不愿穿华丽的服装：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我认为安静的娱乐远远胜过热闹的宴会：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我明知自己有缺点，但不愿接受别人的批评：",
            factor: Some(Factor::G),
            options: &[
                QuestionOption {
                    text: "偶然如此",
                    point: 0,
                },
                QuestionOption {
                    text: "极少如此",
                    point: 1,
                },
                QuestionOption {
                    text: "从不如此",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我总是把“是，非，善，恶”作为处理问题的原则：",
            factor: Some(Factor::G),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "当我工作时，我不喜欢有许多人在旁边参观：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我认为，侮辱那些即使有错误但有文化教养的人，如医生、教师等也是不应该的：",
            factor: Some(Factor::I),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在各种课程中，我喜欢：",
            factor: Some(Factor::I),
            options: &[
                QuestionOption {
                    text: "语文",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "数学",
                    point: 0,
                },
            ],
        },
        Question {
            title: "那些自以为是、道貌岸然的人使我生气：",
            factor: Some(Factor::L),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "和循规蹈矩的人交谈：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "很有兴趣，并有所",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "他们的思想简单，使我太厌烦",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我喜欢：",
            factor: Some(Factor::M),
            options: &[
                QuestionOption {
                    text: "有几个有时对我很苛求但富有感情的朋友",
                    point: 0,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不受别人的干扰",
                    point: 2,
                },
            ],
        },
        Question {
            title: "如果征求我的意见，我赞同：",
            factor: Some(Factor::N),
            options: &[
                QuestionOption {
                    text: "切实制止精神病患者和智能低下的人生育",
                    point: 2,
                },
                QuestionOption {
                    text: "不确定",
                    point: 1,
                },
                QuestionOption {
                    text: "杀人犯必须判处死刑",
                    point: 0,
                },
            ],
        },
        Question {
            title: "有时我会无缘无故地感到沮丧，痛哭：",
            factor: Some(Factor::O),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "当和立场相反的人争辩时，我主张：",
            factor: Some(Factor::Q1),
            options: &[
                QuestionOption {
                    text: "尽量找出基本概念的差异",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "彼此让步",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我一向重感情而不重理智，因而我的观点常常动摇不定：",
            factor: Some(Factor::Q1),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "我的学习多赖于：",
            factor: Some(Factor::Q2),
            options: &[
                QuestionOption {
                    text: "阅读书刊",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "参加集体讨论",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我宁愿选择一个工资较高的工作，不在乎是否有保障，而不愿做工资低的固定工作：",
            factor: Some(Factor::Q3),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "在参加讨论时，我总是能把握自己的立场：",
            factor: Some(Factor::Q3),
            options: &[
                QuestionOption {
                    text: "经常如此",
                    point: 2,
                },
                QuestionOption {
                    text: "一般如此",
                    point: 1,
                },
                QuestionOption {
                    text: "必要时才如此",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我常常被一些无所谓的小事所烦扰：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我宁愿住在嘈杂的闹市区，而不愿住在僻静的地区：",
            factor: Some(Factor::Q4),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 2,
                },
            ],
        },
        Question {
            title: "下列工作如果任我挑选的话，我愿做：",
            factor: Some(Factor::A),
            options: &[
                QuestionOption {
                    text: "少先队辅导员",
                    point: 2,
                },
                QuestionOption {
                    text: "不太确定",
                    point: 1,
                },
                QuestionOption {
                    text: "修表工作",
                    point: 0,
                },
            ],
        },
        Question {
            title: "一人（   ）事，人人受累：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "偾",
                    point: 1,
                },
                QuestionOption {
                    text: "愤",
                    point: 0,
                },
                QuestionOption {
                    text: "喷",
                    point: 0,
                },
            ],
        },
        Question {
            title: "望子成龙的家长往往（ ）苗助长：",
            factor: Some(Factor::B),
            options: &[
                QuestionOption {
                    text: "揠",
                    point: 1,
                },
                QuestionOption {
                    text: "堰",
                    point: 0,
                },
                QuestionOption {
                    text: "偃",
                    point: 0,
                },
            ],
        },
        Question {
            title: "气候的变化并不影响我的情绪：",
            factor: Some(Factor::C),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "因为我对一切问题都有一些见解，所以大家都认为我是一个有头脑的人：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我讲话的声音：",
            factor: Some(Factor::E),
            options: &[
                QuestionOption {
                    text: "洪亮",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "低沉",
                    point: 0,
                },
            ],
        },
        Question {
            title: "一般人都认为我是一个活跃热情的人：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我喜欢做出差机会较多的工作：",
            factor: Some(Factor::F),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我做事严格，力求把事情办得尽善尽美：",
            factor: Some(Factor::G),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在取回或归还所借的东西时，我总是仔细检查，看是否保持原样：",
            factor: Some(Factor::G),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "介于 A、C 之间",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我通常是精力充沛，忙碌多事：",
            factor: Some(Factor::H),
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 2,
                },
                QuestionOption {
                    text: "不一定",
                    point: 1,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
        Question {
            title: "我确信我没有遗漏或漫不经心回答上面的任何问题：",
            factor: None,
            options: &[
                QuestionOption {
                    text: "是的",
                    point: 0,
                },
                QuestionOption {
                    text: "不一定",
                    point: 0,
                },
                QuestionOption {
                    text: "不是的",
                    point: 0,
                },
            ],
        },
    ],
};
