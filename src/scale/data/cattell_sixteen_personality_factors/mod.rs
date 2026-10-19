use serde::Serialize;

use crate::scale::{
    Characteristic, HTMLElement, PlainText, QuestionOption, Scale, ScaleCategory, SentenceItem,
    Tag, Texts,
};

mod questions;

use self::questions::QUESTIONS;

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

/// 16PF常模
///
/// 来源不权威，仅供参考：<http://old.lifeweek.com.cn//2013/0115/39671_6.shtml>
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
            Range::Array([20, 21]),
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
                    "有强烈的事业心，做事果断、有魄力，充满进取精神，精力充沛且行动迅速。但容易忽略生活中的细节，只关注核心问题；有时会因考虑不周、过于冲动而贸然行事，不计后果。",
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
                    "性格果敢、独立，有魄力，敢于表达自己的想法和立场。会主动寻找能发挥自身能力的机会，展现自己的创造力；但有时可能会表现出较强的攻击性，过于追求自身利益。",
                ],
            },
        },
    ],
};

pub const SIXTEEN_PERSONALITY_FACTORS: Scale<Interpretation, Question> = Scale {
    id: 2,
    name: "卡特尔16种人格因素问卷",
    description: "16个维度，一眼看懂你独一无二的人格特征",
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
        "本测试采用心理学家卡特尔的经典人格理论框架。要理解这个测试，了解三个关键概念会很有帮助：",
        "1. 特质是基石：人格并非不可捉摸，它由许多稳定的“特质”组成。有些特质更多受遗传影响，有些则更受环境影响。",
        "2. 从表面到根源：我们日常表现出的行为（表面特质），往往源于内心更深层的性格根源（根源特质）。本问卷旨在测量这些更稳定的根源特质",
        "3. 十六维地图：通过科学分析，卡特尔将这些根源特质归纳为16个核心因素，涵盖了你为人处世的风格（如乐群性、世故性）、情绪模式（如稳定性、紧张性）、以及动力与自律（如有恒性、自律性）等主要方面。",
        "接下来，你将通过回答一系列问题，在这张16维人格地图上找到自己的坐标。",
    ]),
    references: None,
    formula_mode: None,
    warning: Some("本量表仅适用 16 岁以上的人群。"),
    tags: &Tag {
        info: Some(&["人格"]),
        normal: None,
        warning: Some(&["16+"]),
        error: None,
    },
    interpretation: INTERPRETATION,
    questions: QUESTIONS,
};
