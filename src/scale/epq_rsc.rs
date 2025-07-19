use serde::Serialize;

use super::{HTMLElement, QuestionOption, Scale, SentenceItem, Tag, Texts};


#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct NormDimension {
    m: f64,
    sd: f64,
}

#[derive(Debug, Serialize)]
struct NormRange {
    range: [Option<u8>; 2],
    #[serde(rename(serialize = "P"))]
    p: NormDimension,
    #[serde(rename(serialize = "E"))]
    e: NormDimension,
    #[serde(rename(serialize = "N"))]
    n: NormDimension,
    #[serde(rename(serialize = "L"))]
    l: NormDimension,
}

#[derive(Debug, Serialize)]
struct Norm {
    male: [NormRange; 7],
    female: [NormRange; 7],
}

const NORM: Norm = Norm {
    male: [
        NormRange {
            range: [Some(16), Some(19)],
            p: NormDimension { m: 3.15, sd: 1.82 },
            e: NormDimension { m: 7.74, sd: 2.77 },
            n: NormDimension { m: 4.70, sd: 2.96 },
            l: NormDimension { m: 4.43, sd: 2.55 },
        },
        NormRange {
            range: [Some(20), Some(29)],
            p: NormDimension { m: 3.00, sd: 2.00 },
            e: NormDimension { m: 8.05, sd: 2.67 },
            n: NormDimension { m: 4.57, sd: 3.06 },
            l: NormDimension { m: 4.90, sd: 2.66 },
        },
        NormRange {
            range: [Some(30), Some(39)],
            p: NormDimension { m: 2.88, sd: 2.04 },
            e: NormDimension { m: 7.82, sd: 2.68 },
            n: NormDimension { m: 4.01, sd: 2.78 },
            l: NormDimension { m: 5.61, sd: 2.66 },
        },
        NormRange {
            range: [Some(40), Some(49)],
            p: NormDimension { m: 2.91, sd: 2.34 },
            e: NormDimension { m: 7.34, sd: 2.88 },
            n: NormDimension { m: 4.34, sd: 2.95 },
            l: NormDimension { m: 6.55, sd: 2.78 },
        },
        NormRange {
            range: [Some(50), Some(59)],
            p: NormDimension { m: 2.67, sd: 2.21 },
            e: NormDimension { m: 6.95, sd: 2.98 },
            n: NormDimension { m: 3.90, sd: 2.89 },
            l: NormDimension { m: 7.19, sd: 2.66 },
        },
        NormRange {
            range: [Some(60), Some(69)],
            p: NormDimension { m: 2.68, sd: 2.31 },
            e: NormDimension { m: 7.08, sd: 3.01 },
            n: NormDimension { m: 3.70, sd: 3.00 },
            l: NormDimension { m: 7.73, sd: 3.08 },
        },
        NormRange {
            range: [Some(70), None],
            p: NormDimension { m: 2.92, sd: 2.79 },
            e: NormDimension { m: 6.89, sd: 3.08 },
            n: NormDimension { m: 4.38, sd: 3.39 },
            l: NormDimension { m: 8.00, sd: 3.13 },
        },
    ],
    female: [
        NormRange {
            range: [Some(16), Some(19)],
            p: NormDimension { m: 2.63, sd: 1.81 },
            e: NormDimension { m: 8.13, sd: 2.58 },
            n: NormDimension { m: 4.93, sd: 2.75 },
            l: NormDimension { m: 4.86, sd: 2.43 },
        },
        NormRange {
            range: [Some(20), Some(29)],
            p: NormDimension { m: 2.68, sd: 1.82 },
            e: NormDimension { m: 7.44, sd: 2.79 },
            n: NormDimension { m: 4.81, sd: 2.95 },
            l: NormDimension { m: 5.32, sd: 2.70 },
        },
        NormRange {
            range: [Some(30), Some(39)],
            p: NormDimension { m: 2.44, sd: 1.82 },
            e: NormDimension { m: 7.50, sd: 2.87 },
            n: NormDimension { m: 4.49, sd: 2.89 },
            l: NormDimension { m: 6.64, sd: 2.76 },
        },
        NormRange {
            range: [Some(40), Some(49)],
            p: NormDimension { m: 2.55, sd: 2.30 },
            e: NormDimension { m: 7.15, sd: 2.86 },
            n: NormDimension { m: 4.44, sd: 2.95 },
            l: NormDimension { m: 7.45, sd: 2.98 },
        },
        NormRange {
            range: [Some(50), Some(59)],
            p: NormDimension { m: 2.36, sd: 1.82 },
            e: NormDimension { m: 6.92, sd: 2.90 },
            n: NormDimension { m: 4.48, sd: 2.88 },
            l: NormDimension { m: 7.73, sd: 2.68 },
        },
        NormRange {
            range: [Some(60), Some(69)],
            p: NormDimension { m: 2.51, sd: 1.98 },
            e: NormDimension { m: 7.28, sd: 2.95 },
            n: NormDimension { m: 4.44, sd: 3.12 },
            l: NormDimension { m: 7.72, sd: 2.96 },
        },
        NormRange {
            range: [Some(70), None],
            p: NormDimension { m: 2.32, sd: 1.89 },
            e: NormDimension { m: 7.28, sd: 3.48 },
            n: NormDimension { m: 4.88, sd: 3.25 },
            l: NormDimension { m: 8.84, sd: 2.58 },
        },
    ],
};

#[derive(Debug, Serialize)]
struct HighLow {
    high: &'static [&'static str],
    low: &'static [&'static str],
}

#[derive(Debug, Serialize)]
struct DimensionScoreInterpretation {
    /// 中间型，解释文本是由句子组成的文本数组。
    osculant: &'static [&'static str],
    /// 倾向型
    inclined: HighLow,
    /// 典型
    typical: HighLow,
}

#[derive(Debug, Serialize)]
struct DimensionInterpretation {
    label: &'static str,
    notice: Option<&'static str>,
    supplementary: Option<&'static [&'static str]>,
    dimension: DimensionScoreInterpretation,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct Dimensions {
    e: DimensionInterpretation,
    n: DimensionInterpretation,
    p: DimensionInterpretation,
    l: DimensionInterpretation,
}

#[derive(Debug, Serialize)]
pub struct Interpretation {
    norm: Norm,
    dimensions: Dimensions,
    temperaments: Temperaments,
    // todo: 结果解释暂时直接写在前端，需要后续移动回后端
}


#[derive(Debug, Serialize)]
enum Dimension {
    /// Extraversion/Introversion，内外倾向量表
    E,
    /// Neuroticism/Stability，情绪性量表
    N,
    /// Psychoticism/Socialisation，心理变态量表，又称精神质
    P,
    /// Lie/Social Desirability，效度量表
    L,
}

#[derive(Debug, Serialize)]
struct Temperaments {
    sanguine: &'static [&'static str],
    choleric: &'static [&'static str],
    melancholic: &'static [&'static str],
    phlegmatic: &'static [&'static str],
}

// /// 气质类型
// #[derive(Debug, Serialize)]
// enum Temperament {
//     /// 多血质
//     Sanguine,
//     /// 胆汁质
//     Choleric,
//     /// 抑郁质
//     Melancholic,
//     /// 粘液质
//     Phlegmatic,
// }

#[derive(Debug, Serialize)]
pub struct Question {
    title: &'static str,
    options: &'static [QuestionOption],
    dimension: Dimension,
}

const INTRODUCTION: Texts = &[
    &[
        SentenceItem::Plain("艾森克人格问卷（Eysenck Personality Questionnaire, 简称 EPQ）是英国伦敦大学心理系和精神病研究所艾森克教授编制的，归纳了三个人格的基本因素："),
        SentenceItem::HTMLElement(HTMLElement::Strong("内外向性")),
        SentenceItem::Plain("(E)、"),
        SentenceItem::HTMLElement(HTMLElement::Strong("神经质")),
        SentenceItem::Plain("(又称情绪性)(N)和"),
        SentenceItem::HTMLElement(HTMLElement::Strong("精神质")),
        SentenceItem::Plain("(又称倔强、讲求实际)(P)。"),
    ],
    &[
        SentenceItem::Plain("三个人格度不但经过许多数学统计上的和行为观察方面的分析，而且也得到实验室内多种心理实验的考察，被广泛应用于医学、司法、教育和心理咨询等领域，适合各种人群测试。"),
    ],
    &[
        SentenceItem::Plain("艾森克人格问卷简式量表中国版在原量表的基础上针对中国进行了修订，在保持甚至提高信度的同时精简了部分题目。"),
    ]
];

const INSTRUCTION: Texts = &[
    &[
        SentenceItem::Plain("该量表共有 "),
        SentenceItem::HTMLElement(HTMLElement::Strong("48")),
        SentenceItem::Plain(" 个项目，请仔细阅读每一条，然后根据该句话与您自己的实际情况相符合的程度进行选择。"),
    ]
];

pub const EPQ_RSC: Scale<Interpretation, Question> = Scale {
    name: "艾森克人格问卷简式量表中国版",
    abbreviation: "EPQ-RSC",
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: None,
    formula_mode: None,
    warning: Some("本量表仅适用 16 岁以上的人群。"),
    tags: Tag{ info: Some(&[ "人格"]), normal: Some(&["自评"]), warning: Some(&["16+"]), error: None },
    interpretation: Interpretation { 
        norm: NORM,
        temperaments: Temperaments {
            sanguine: &[
                "多血质是人的气质类型之一。多血质的人表现出这样的特点：容易形成有朝气、热情、活泼、爱交际、有同情心、思想灵活等品质；也容易出现变化无常、粗枝大叶、浮躁、缺乏一贯性等特点。这种人活泼、好动、敏感、反应迅速、喜欢与人交往、注意力容易转移、兴趣和情感易变换等等。这种人适宜于做要求反应迅速而灵活的工作。",
                "外向，活泼好动,善于交际；思维敏捷；容易接受新鲜事物；情绪情感容易产生也容易变化和消失，容易外露；体验不深刻等。",
                "多血质是职业多面手，专长多，能力强，精于调整、调和各类关系，有经营管理、分析设计和规划能力，会推销商品。适于经济规划、统计、设计、商业推销、节目主持、相声演员等。"
            ],
            choleric: &[
                "胆汁质（bilious temperament），人的四种性格类型之一，其特点是“情感发生迅速、强烈、持久，动作的发生也是迅速、强烈、有力。属于这一类型的人都热情，直爽，精力旺盛，脾气急躁，心境变化剧烈，易动感情，具有外倾性。",
                "反应迅速，情绪有时激烈、冲动，很外向。",
                "胆汁质的人较适合做反应迅速、动作有力、应急性强、危险性较大、难度较高而费力的工作。他们大都可能成为出色的导游员、勘探工作者、推销员、节目主持人、演讲者、外事接待员等。",
            ],
            melancholic: &[
                "抑郁质是人的一种气质类型，抑郁质特点：抑郁质的人神经类型属于弱型，他们体验情绪的方式较少，稳定的情感产生也很慢，但对情感的体验深刻、有力、持久，而且具有高度的情绪易感性。抑郁质的人为人小心谨慎，思考透彻，在困难面前容易优柔寡断。",
                "抑郁质的人一般表现为行为孤僻、不太合群、观察细致、非常敏感、表情腼腆、多愁善感、行动迟缓、优柔寡断，具有明显的内倾性。",
                "抑郁质适合的职业：校对、打字、排版、检察员、雕刻工作、刺绣工作、保管员、机要秘书、艺术工作者、哲学家、科学家等。",
            ],
            phlegmatic: &[
                "粘液质人的表现特点：粘液质相当于神经活动强而均衡的安静型。这种气质的人平静，善于克制忍让，生活有规律，不为无关事情分心，埋头苦干，有耐久力，态度持重，不卑不亢，不爱空谈，严肃认真；但不够灵活，注意力不易转移，因循守旧，对事业缺乏热情。",
                "粘液质适合的职业：外科医生、法官、管理人员、出纳员、会计、播音员、话务员、调解员、教师、人力人事管理主管、心理咨询师等。",
            ],
        },
        dimensions: Dimensions {
            e: DimensionInterpretation{
                notice: None,
                label: "内外倾向",
                supplementary: Some(&[
                    "需要注意的是，这些是一般性的趋势，而个体差异仍然存在。人格是复杂多面的，还受到其他因素的影响，如环境、个人经历和文化背景。因此，这只是一种大致的描述，具体的行为表现还需要考虑其他因素。",
                ]),
                dimension: DimensionScoreInterpretation { 
                    osculant: &[
                        "社交灵活性：在社交方面可能表现出一定的灵活性，既能够与他人交往，又能够独立行动，具有一定的自主性。",
                        "适度的社交活动：可能喜欢参与社交活动，但不会过分依赖社交，也能够独自度过一段时间。",
                        "合理的冒险倾向：可能愿意尝试新事物和冒险，但不会过于冲动，会在适当时机考虑风险和后果。",
                        "适度的能量水平：在能量和活跃度方面可能表现出适度的水平，既不过于亢奋，也不过于沉闷。",
                        "社交技能：可能具备一定的社交技能，能够与不同类型的人建立关系，但不会过分追求社交成功。",
                    ], 
                    inclined: HighLow { 
                        high: &[
                            "社交活跃：较高外向性得分的人通常更喜欢社交活动，愿意主动参与各种社交场合，与他人互动更频繁。",
                            "容易结交朋友：由于外向性较高，这些人通常更容易结交朋友，建立和维护社交网络。",
                            "喜欢刺激：对新奇和刺激的需求较高，可能更愿意尝试新事物、参与冒险活动，寻求刺激。",
                            "乐观开朗：较高外向性的人可能更倾向于乐观、开朗，能够积极面对生活的挑战。",
                        ], 
                        low: &[
                            "较为内向：较低外向性得分的个体可能更倾向于内向，可能不太主动在社交场合中参与。",
                            "独立性：有可能更喜欢独自工作或独自度过时间，不太依赖外部刺激来维持情绪。",
                            "喜欢安静的环境：较低外向性得分的人可能更喜欢安静、不那么繁忙的环境，可能更容易感到疲倦或不适应过于刺激的环境。",
                            "较少的社交活动：可能对社交活动的需求较少，不太喜欢大型社交活动，更倾向于小团体或独立活动。",
                        ] 
                    }, 
                    typical: HighLow { 
                        high: &[
                            "极度社交活跃：非常高外向性得分的人可能表现出极度的社交活跃，几乎总是喜欢与他人在一起。",
                            "喜欢领导角色：由于外向性较高，这些人可能更愿意担任领导角色，参与团队合作，与他人协作。",
                            "寻求持续刺激：非常高外向性得分的人可能不仅喜欢新奇和刺激，还可能持续地寻求这些体验。",
                            "与人为善：具有高外向性的人通常更容易与他人建立亲密关系，表现出友善和合作的态度。",
                        ], 
                        low: &[
                            "高度内向：非常低外向性得分的个体可能表现出高度的内向，可能非常少参与社交活动，更喜欢独自一人。",
                            "社交回避：可能对社交场合感到不适应或回避，可能避免参与需要大量社交互动的情境。",
                            "独立性强烈：非常低外向性得分的个体可能强烈地追求独立，可能更愿意独立完成任务，避免过多的合作和协作。",
                        ],
                    },
                },
            },
            n: DimensionInterpretation{
                label: "神经质/情绪稳定性",
                notice: Some("反映的是正常行为，与病症无关。"),
                supplementary: Some(&[
                    "需要注意的是，这些是一般性的趋势，而个体差异仍然存在。人格是复杂多面的，还受到其他因素的影响，如环境、个人经历和文化背景。因此，这只是一种大致的描述，具体的行为表现还需要考虑其他因素。",
                ]),
                dimension: DimensionScoreInterpretation { 
                    osculant: &[
                        "适应性较好： 在面对生活的起伏和压力时，情绪稳定性得分平均水平的人可能表现出相对较好的适应能力。他们可能能够更容易地处理压力源，并在面对挑战时保持相对冷静和理智。",
                        "较少情绪波动： 这些人可能不太容易受到小事的影响，也可能在日常生活中经历的情绪波动相对较小。他们可能更容易保持积极的情绪状态。",
                        "相对较少焦虑： 情绪稳定性得分平均水平的人可能相对较少表现出过度的焦虑和紧张感。他们可能更容易保持情绪平衡，不太容易陷入强烈的负面情绪状态。",
                        "适度的风险承受能力： 这些人可能在适度的风险和不确定性下表现得较为从容。他们可能不太容易过于谨慎或过于冒险。",
                    ], 
                    inclined: HighLow { 
                        high: &[
                            "情感调控能力强： 高情绪稳定性的个体通常能更好地应对压力和挫折，情绪波动较小。",
                            "较少体验焦虑和紧张： 这些人在面对压力时可能表现得更冷静，不容易被不良情绪所影响。",
                            "自信和乐观： 高情绪稳定性的人可能更倾向于乐观和自信，能够更积极地看待生活中的各种情况。",
                            "稳定的情感状态： 这类个体在情感上更加平稳，不太容易情绪波动大。",
                        ], 
                        low: &[
                            "冷静沉着： 他们可能在面对压力和困难时能够保持冷静，不容易被激怒或受到情绪波动的影响。",
                            "情绪调节能力： 具有较低情绪稳定性的人可能更擅长自我调节，能够更有效地处理负面情绪。",
                            "较少的焦虑： 相对较低得分的人可能不太容易感到焦虑或紧张，对生活中的各种变化可能更加适应。",
                        ],
                    }, 
                    typical: HighLow { 
                        high: &[
                            "冷静和沉着： 在极端情况下，非常高的情绪稳定性可能表现为极度的冷静和沉着，即使面临压力也能保持冷静。",
                            "抗压能力强： 这类个体可能对生活中的挑战和压力具有较高的抵抗力，不容易受到外界因素的干扰。",
                            "情感表达相对稳定： 非常高的情绪稳定性可能意味着情感表达相对一致，不容易受外界情绪因素的左右。",
                        ], 
                        low: &[
                            "冷漠和淡漠： 在极端情况下，非常低的情绪稳定性得分可能表现为对他人情感的冷漠和淡漠，缺乏对他人需求的关切。",
                            "易受伤： 可能对他人的批评或压力过度敏感，容易感到受伤并产生消极情绪反应。",
                            "社交障碍： 非常低的情绪稳定性得分可能与社交问题相关，因为个体可能难以处理社交压力和冲突。",
                        ],
                    }, 
                },
            },
            p: DimensionInterpretation{
                label: "精神质",
                notice: Some("反映的是正常行为，与病症无关。"),
                supplementary: Some(&[
                    "需要注意的是，这些是一般性的趋势，而个体差异仍然存在。人格是复杂多面的，还受到其他因素的影响，如环境、个人经历和文化背景。因此，这只是一种大致的描述，具体的行为表现还需要考虑其他因素。",
                ]),
                dimension: DimensionScoreInterpretation { 
                    osculant: &[
                        "平衡的情感表达：在情感稳定性上，可能表现出一定的平衡，对于压力和负面情绪的反应相对中等。",
                        "社交灵活性：在与他人的互动中可能表现出一定的灵活性，既能够独立行动，又能够与他人合作和交往。",
                        "决策的谨慎性：在决策和行为上可能相对谨慎，不会过于冲动，而且能够考虑到可能的后果。",
                        "对他人感受的关注：在关心他人感受方面可能相对均衡，既能够表达关切，又不至于过于冷漠。",
                        "适度的冷静度：在面对压力和挑战时，可能表现出一种适度的冷静和决断，而不是过于冷漠或过于激动。",
                    ], 
                    inclined: HighLow { 
                        high: &[
                            "冷漠和不关心：倾向于表现出对他人感受的冷漠和不关心，可能不太关注或理会他人的情感需求。",
                            "冲动和不稳定：可能表现出较大的冲动性，缺乏计划性和稳定性，可能在决策和行为上表现出不可预测性。",
                            "挑战性：可能具有较强的挑战性，对于权威和规则可能表现出不敬和抵触的态度，更倾向于追求刺激和冒险。",
                            "不合作和独立：可能不太愿意合作，更喜欢独立思考和行动，可能对他人的期望和规定不太敏感。",
                        ], 
                        low: &[
                            "温和和关心：较低的精神质得分可能意味着个体更为温和，更关心他人感受，可能更愿意与他人合作。",
                            "稳定和可预测：较低的精神质得分可能表示个体在情感上相对稳定，决策和行为较为谨慎和可预测。",
                            "遵从规则：较低的精神质得分可能意味着个体更愿意遵循社会规范和权威的指导，更容易与组织和团队协作。",
                            "谨慎和深思熟虑：较低的精神质得分的个体可能更倾向于深思熟虑，对于决策和行为可能更为谨慎。",
                        ],
                    }, 
                    typical: HighLow { 
                        high: &[
                            "极端冷漠和冷酷：可能表现出极端的冷漠和冷酷，对他人的需求和感受缺乏关切，可能对他人的痛苦不太敏感。",
                            "极高冲动性：在决策和行为上可能表现出极端的冲动性，可能会冲动地采取行动而不考虑后果。",
                            "极端挑战性：可能表现出极端的挑战性，对于规则和权威可能持有极端的抵触态度，可能寻求极端的刺激和冒险。",
                            "完全不合作和孤立：可能对合作和社交完全不感兴趣，更倾向于独立行动，可能表现出完全不合作的态度。",
                        ], 
                        low: &[
                            "极端关心他人：非常低的精神质得分可能表明个体对他人感受非常关切，可能表现出极端的关心和同理心。",
                            "高度稳定：在情感上可能非常稳定，很少受到外部刺激的影响，决策和行为相对可预测。",
                            "强烈的合作倾向：非常低的精神质得分可能表现出强烈的合作倾向，更愿意与他人共事，避免冲突。",
                        ],
                    }, 
                },
            },
            l: DimensionInterpretation{
                label: "效度或掩饰性",
                notice: None,
                supplementary: None,
                dimension: DimensionScoreInterpretation { 
                    osculant: &[
                        "您可能诚实地回答了问卷中的问题，没有故意隐瞒或歪曲信息。",
                        "您也有可能在测试中没有明显的试图产生社会上更为接受的回答，或者没有明显的欺骗行为。这可以被解释为测试的效度较高，您愿意以真实的方式回答问题。",
                        "在某些情况下，平均水平的L得分可能表明您的反应风格较为均衡，既没有过分的坦率也没有过分的隐瞒。这可能取决于您的个性特征和对测试的态度。",
                    ], 
                    inclined: HighLow { 
                        high: &[
                            "您可能有一定的社会欲望或者试图呈现自己更为正面的形象。",
                            "这可能是因为您在进行测试时感到一些社会压力，希望产生较为正面的印象。",
                            "注意，这并不一定意味着您在所有情境下都会有不真实的回答，但在某些方面可能会有一些社交导向的倾向。",
                        ], 
                        low: &[
                            "您在回答问题时倾向于诚实，但并没有强烈的倾向。",
                        ],
                    }, 
                    typical: HighLow { 
                        high: &[
                            "您的得分可能表明您在测试过程中采取了过于防御性的态度，可能存在试图掩盖真实感受或情况的倾向。",
                        ], 
                        low: &[
                            "您回答问题时较为坦诚，没有试图隐瞒或虚伪。",
                        ],
                    },
                },
            },
        }
    },
    questions: &[        
        Question {
            title: "你的情绪是否时起时落?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "当你看到小孩(或动物)受折磨时是否感到难受?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是个健谈的人吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "如果你说了要做什么事，是否不论此事可能如果不顺利你都总能遵守诺言?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否会无言无故地感到“很惨”?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "欠债会使你感到忧虑吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是个生气勃勃的人吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否曾贪图过超过你应得的分外之物?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是个容易被激怒的人吗?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你会服用能产生奇异或危险效果的药物吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你愿意认识陌生人吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否曾经有过明知自己做错了事却责备别人的情况?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你的感情容易受伤害吗?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否愿意按照自己的方式行事，而不愿意按照规则办事?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "在热闹的聚会中你能使自己放得开，使自己玩得开心吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你所有的习惯是否都是好的?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否时常感到“极其厌倦”?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "良好的举止和整洁对你来说很重要吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "在结交新朋友时，你经常是积极主动的吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否有过随口骂人的时候?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你认为自己是一个胆怯不安的人吗?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否认为婚姻是不合时宜的，应该废除?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你能否很容易地给一个沉闷的聚会注入活力?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你曾毁坏或丢失过别人的东西吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是个忧心忡忡的人吗?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你爱和别人合作吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "在社交场合你是否倾向于呆在不显眼的地方?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "如果在你的工作中出现了错误，你知道后会感到忧虑吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你讲过别人的坏话或脏话吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你认为自己是个神经紧张或“弦绷得过紧”的人吗?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否觉得人们为了未来有保障，而在储蓄和保险方面花费的时间太多了?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否喜欢和人们相处在一起?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "当你还是个小孩子的时候，你是否曾有过对父母耍赖或不听话的行为?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "在经历了一次令人难堪的事之后，你是否会为此烦恼很长时间?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否努力使自己对人不粗鲁?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是否喜欢在自己周围有许多热闹和令人兴奋的事情?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你曾在玩游戏时作过弊吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是否因自己的“神经过敏”而感到痛苦?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你愿意别人怕你吗?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你曾利用过别人吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你是否喜欢说笑话和谈论有趣的事?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否时常感到孤独?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否认为遵循社会规范比按照个人方式行事更好一些?",
            dimension: Dimension::P,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "在别人眼里你总是充满活力的吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你总能做到言行一致吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你是否时常被内疚感所困扰?",
            dimension: Dimension::N,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
        Question {
            title: "你有时将今天该做的事情拖到明天去做吗?",
            dimension: Dimension::L,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 0,
                },
                QuestionOption {
                    text: "不是",
                    point: 1,
                },
            ],
        },
        Question {
            title: "你能使一个聚会顺利进行下去吗?",
            dimension: Dimension::E,
            options: &[
                QuestionOption {
                    text: "是",
                    point: 1,
                },
                QuestionOption {
                    text: "不是",
                    point: 0,
                },
            ],
        },
    ]
};
