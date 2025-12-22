use serde::Serialize;

use crate::scale::{PlainText, PlainTexts, ScaleCategory, SymptomGuidance};

use super::{
    FormulaMode, HTMLElement, Integer, OperationalRule, QuestionOption, Scale, SentenceItem, Tag,
    Texts,
};

#[derive(Debug, Serialize, Hash, Eq, PartialEq)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
enum Symptom {
    /// 躯体化
    Somatization,
    /// 强迫症状
    ObsessiveCompulsive,
    /// 人际关系敏感
    InterpersonalSensitivity,
    /// 抑郁
    Depression,
    /// 焦虑
    Anxiety,
    /// 敌对
    Hostility,
    /// 恐怖
    PhobicAnxiety,
    /// 偏执
    ParanoidIdeation,
    /// 精神病性
    Psychoticism,
    /// 其他
    Others,
}

#[derive(Debug, Serialize)]
pub struct Question {
    title: PlainText,
    options: &'static [QuestionOption],
    symptom: Symptom,
}

const OPTIONS: [QuestionOption; 5] = [
    QuestionOption {
        text: "从无",
        point: 1,
    },
    QuestionOption {
        text: "很轻",
        point: 2,
    },
    QuestionOption {
        text: "中等",
        point: 3,
    },
    QuestionOption {
        text: "偏重",
        point: 4,
    },
    QuestionOption {
        text: "严重",
        point: 5,
    },
];

#[derive(Debug, Serialize)]
struct Rule {
    value: u8,
    comparison_operator: PlainText,
}

#[derive(Debug, Serialize)]
struct Positive {
    total: Rule,
    positive_amount: Rule,
    any_symptom_average: Rule,
}

#[derive(Debug, Serialize)]
pub struct SymptomInterpretation {
    /// 中文名称
    name: PlainText,
    /// 专业术语
    terminology: PlainText,
    /// 描述
    description: PlainText,
    /// 阳性描述
    positive: PlainTexts,
    /// 阴性描述
    negative: PlainText,
    /// 建议
    guidance: SymptomGuidance,
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "UPPERCASE"))]
struct Symptoms {
    somatization: SymptomInterpretation,
    obsessive_compulsive: SymptomInterpretation,
    interpersonal_sensitivity: SymptomInterpretation,
    depression: SymptomInterpretation,
    anxiety: SymptomInterpretation,
    hostility: SymptomInterpretation,
    phobic_anxiety: SymptomInterpretation,
    paranoid_ideation: SymptomInterpretation,
    psychoticism: SymptomInterpretation,
    others: SymptomInterpretation,
}

#[derive(Debug, Serialize)]
pub struct Interpretation {
    positive: Positive,
    symptoms: Symptoms,
}

const INTRODUCTION: Texts = &[&[
    SentenceItem::Plain("《SCL90症状自评量表》是全球使用"),
    SentenceItem::HTMLElement(HTMLElement::Strong("最广泛")),
    SentenceItem::Plain("的心理健康评估工具之一，帮助您从"),
    SentenceItem::HTMLElement(HTMLElement::Strong("情绪、压力、人际关系等10个维度")),
    SentenceItem::Plain("全面了解自己的心理状态。它常用于专业初筛，也能为您提供自我觉察的参考。"),
]];

const INSTRUCTION: Texts = &[
    &[
        SentenceItem::Plain("该量表共有 "),
        SentenceItem::HTMLElement(HTMLElement::Strong("90")),
        SentenceItem::Plain(" 个项目，包含有较广泛的精神病症状学内容，从"),
        SentenceItem::HTMLElement(HTMLElement::Strong("感觉、情感、思维、意识、行为")),
        SentenceItem::Plain("直至"),
        SentenceItem::HTMLElement(HTMLElement::Strong("生活习惯、人际关系、饮食睡眠")),
        SentenceItem::Plain("等，均有涉及，并采用 10 个因子分别反映 10 个方面的心理症状情况。")
    ],
    &[
        SentenceItem::Plain("以下问题中列出了有些人可能有的症状或问题，请仔细阅读每一条，然后根据该句话与您自己的实际情况相符合的程度（"),
        SentenceItem::HTMLElement(HTMLElement::Strong("最近一个星期或现在")),
        SentenceItem::Plain("）进行选择。"),
    ],
];

const INTERPRETATION: Interpretation = Interpretation {
    positive: Positive {
        total: Rule {
            value: 160, comparison_operator: ">="
        },
        positive_amount: Rule {
            value: 43, comparison_operator: ">"
        },
        any_symptom_average: Rule {
            value: 2, comparison_operator: ">"
        }
    },
    symptoms: Symptoms {
        somatization: SymptomInterpretation {
            name: "躯体化",
            terminology: "Somatization",
            description: "指心理上的压力、烦恼或情绪问题，没有直接表现为心情不好，反而转化成了身体上的不舒服。通常表现多样，但去医院做检查，常常发现不了明确的生理疾病。",
            positive: &[
                "反复或持续的身体疼痛（如头痛、背痛、肌肉酸痛）。",
                "肠胃不适，比如没来由的胃痛、胀气、腹泻或便秘。",
                "心慌、胸闷，或者感觉呼吸不畅。",
                "经常感到头晕、眼花或乏力。",
                "睡眠问题，如失眠、多梦、醒后不解乏。",
            ],
            negative: "身体感受基本正常，偶尔的不适（如累了酸痛、饿了的胃痛）能明确找到原因，并且不影响日常生活。",
            guidance: SymptomGuidance {
                self_help: &[
                    "在医生已排除明显生理问题的前提下，可以尝试记录身体不适出现的时间、情境和当时的情绪状态。",
                    "给自己安排规律的休息和放松时间，例如轻度运动、散步或深呼吸练习。",
                    "留意近期是否存在持续的心理压力或情绪压抑，身体有时会替情绪“说话”。",
                ],
                watch_out: &[
                    "身体不适是否频繁出现，且越来越影响工作、学习或日常活动。",
                    "是否因为担心身体问题而反复就医或过度关注身体感觉。",
                ],
                seek_help_when: &[
                    "身体症状持续数周以上，且明显影响生活质量。",
                    "身体不适伴随明显焦虑或情绪困扰，自己难以缓解。",
                ],
            }
        },
        obsessive_compulsive: SymptomInterpretation {
            name: "强迫症状",
            terminology: "Obsessive-Compulsive Disorder (OCD)",
            description: "指头脑中反复冒出一些自己不想要的想法，或者忍不住想做某些重复动作。自己也知道这些想法或行为没必要，但就是控制不住，不做就会非常焦虑。",
            positive: &[
                "总控制不住地想一些事（如是否锁门、是否脏、是否犯错）。",
                "有强烈的冲动想去做某个特定动作（如触摸、排列）。",
                "重复性的行为（如反复洗手、检查、计数）。",
                "做事必须按照某种固定的顺序或规则，否则就不安心。",
                "过分关注细节、对称或整洁。",
                "害怕自己会失控，做出伤害自己或别人的事。",
            ],
            negative: "思维灵活，能专注在当下重要的事情上。日常习惯让生活有序，但不会因为这些习惯感到强烈的焦虑或被控制。",
            guidance: SymptomGuidance {
                self_help: &[
                    "尝试觉察哪些想法或行为是出于“缓解焦虑”，而不是实际需要。",
                    "在安全的前提下，逐步减少某些重复行为的频率，观察焦虑是否会自然下降。",
                    "提醒自己：有想法并不等于会付诸行动，想法本身并不危险。",
                ],
                watch_out: &[
                    "强迫行为是否占用了大量时间，影响效率或生活节奏。",
                    "是否因为无法完成某些行为而感到极度焦虑或崩溃。",
                ],
                seek_help_when: &[
                    "强迫想法或行为已严重干扰工作、学习或人际关系。",
                    "自己多次尝试调整但效果有限，焦虑感持续加重。",
                ],
            }
        },
        interpersonal_sensitivity: SymptomInterpretation {
            name: "人际关系敏感",
            terminology: "Interpersonal Sensitivity",
            description: "指在和人打交道时特别容易感到紧张、不自信。可能会非常在意别人的看法，别人的一句话、一个眼神就能让你琢磨半天，担心对方是不是不喜欢自己，从而想要回避社交。",
            positive: &[
                "与人相处时感到紧张、不自在。",
                "害怕在别人面前出丑或被批评。",
                "总觉得别人在看轻、拒绝或误解自己。",
                "社交结束后，反复回想和检查自己的表现。",
                "很难对别人说“不”或表达不同意见。",
                "倾向于回避聚会、会议等需要社交的场合。",
            ],
            negative: "在大多数社交场合能感到自然放松，可以比较自信地表达自己。会在意他人的评价，但不会过度纠结或因此严重影响心情。",
            guidance: SymptomGuidance {
                self_help: &[
                    "在社交后反思时，可以尝试区分“事实”和“自己的猜测”。",
                    "练习用更宽松的标准看待自己的表现，允许不完美存在。",
                    "从低压力的社交情境开始，逐步增加互动的频率和深度。",
                ],
                watch_out: &[
                    "是否长期回避社交，或在人际中持续感到自卑和紧张。",
                    "是否过度依赖他人的评价来确认自我价值。",
                ],
                seek_help_when: &[
                    "社交焦虑已明显限制正常的人际交往或职业发展。",
                    "长期感到被否定、被排斥，且难以自行调整。",
                ],
            }
        },
        depression: SymptomInterpretation {
            name: "抑郁",
            terminology: "Depression",
            description: "指的是一种持续较长时间（如连续两周以上）的低落情绪。感觉开心不起来，对以前喜欢的事情也提不起兴趣，整个人容易累，想法容易变得悲观、消极。这种状态已经影响到了日常的工作、学习或生活。",
            positive: &[
                "几乎每天大部分时间都情绪低落、悲伤或空虚。",
                "对几乎所有活动的兴趣或乐趣都明显减退。",
                "精力不足，非常容易疲劳。",
                "食欲和体重发生明显变化（暴增或剧减）。",
                "睡眠困扰（失眠、早醒、嗜睡）。",
                "思考、集中注意力或做决定变得困难。",
                "感觉自己没有价值，或有过度的、不合理的自责。",
                "反复想到死亡，或有自杀念头。",
            ],
            negative: "情绪总体平稳，有正常的喜怒哀乐。遇到挫折会低落，但能较快恢复。对生活保有基本的热情和期待。",
            guidance: SymptomGuidance {
                self_help: &[
                    "尽量保持基本的生活规律，如作息、饮食和日常活动。",
                    "即使提不起兴趣，也可以从非常小的事情开始行动，而不是等状态变好。",
                    "尝试把情绪写下来，帮助自己理清内心的感受。",
                ],
                watch_out: &[
                    "情绪低落是否持续两周以上，且没有明显好转。",
                    "是否明显减少了与他人的联系或对生活的投入。",
                ],
                seek_help_when: &[
                    "持续感到绝望、无助，或生活功能明显下降。",
                    "出现反复想到死亡或自伤的念头（请尽快寻求专业帮助）。",
                ],
            }
        },
        anxiety: SymptomInterpretation {
            name: "焦虑",
            terminology: "Anxiety",
            description: "指的是一种过度的、难以控制的紧张和担心。不仅心里总想着“万一……怎么办”，身体也处于紧绷状态（如心慌、肌肉紧张）。这种担忧常常和现实中的问题不成比例。",
            positive: &[
                "持续感到紧张、不安，很难放松下来。",
                "对未来可能发生的不好的事情过度思虑。",
                "身体紧绷感，如肩颈僵硬、紧咬牙关。",
                "心慌、心跳加快、手抖或出汗。",
                "坐立不安，静不下来。",
                "很容易被一点声响或小事惊吓到。",
            ],
            negative: "在日常生活压力下，能保持相对平静。会对真正有风险的事情担心，但不会整天被各种“万一”所困，大多数时间身心能放松。",
            guidance: SymptomGuidance {
                self_help: &[
                    "当感到焦虑时，尝试把注意力拉回当下，比如关注呼吸或身体感觉。",
                    "将担忧具体化，区分“可以控制的”和“暂时无法控制的”。",
                    "减少咖啡因、熬夜等可能加重紧张感的生活因素。",
                ],
                watch_out: &[
                    "焦虑是否几乎每天存在，且难以放松。",
                    "是否因为担忧而影响睡眠、专注力或决策能力。",
                ],
                seek_help_when: &[
                    "焦虑情绪长期存在，且明显干扰生活。",
                    "伴随频繁的惊恐感或强烈的身体不适。",
                ],
            }
        },
        hostility: SymptomInterpretation {
            name: "敌对",
            terminology: "Hostility",
            description: "指的是一种对他人的、持续的负面态度，表现为容易烦躁、发怒、怨恨，甚至有想争吵或破坏的冲动。这种情绪的强烈程度和持续时间可能超出一般情况。",
            positive: &[
                "脾气变得急躁，一点小事就容易发火。",
                "内心常感到愤愤不平，对他人怀有怨恨。",
                "总想与人争论，习惯性反驳别人。",
                "有摔东西、破坏物品的冲动。",
                "容易把别人的行为往坏处想。",
                "很难原谅别人的过错。",
            ],
            negative: "能恰当地表达不满和不同意见。大多数时候能保持冷静、客观，面对冲突时能以沟通解决问题为主，不易被激怒。",
            guidance: SymptomGuidance {
                self_help: &[
                    "在情绪上来时，尝试暂时离开冲突场景，给自己冷静的空间。",
                    "觉察愤怒背后是否存在委屈、压力或未被满足的需求。",
                    "通过运动或书写等方式释放积累的情绪能量。",
                ],
                watch_out: &[
                    "是否频繁因小事发怒，并在事后感到后悔。",
                    "愤怒是否已影响人际关系或工作表现。",
                ],
                seek_help_when: &[
                    "情绪爆发难以控制，或出现攻击冲动。",
                    "愤怒情绪长期积累，自己难以缓解。",
                ],
            }
        },
        phobic_anxiety: SymptomInterpretation {
            name: "恐怖",
            terminology: "Phobic Anxiety",
            description: "指的是对某些特定的场所、事物或情境（如高处、人群、封闭空间）产生一种强烈的、不合理的恐惧。这种恐惧感非常真实，以至于你会极力去回避那些场合，这可能会明显干扰你的正常生活。",
            positive: &[
                "对场所的恐惧：如害怕空旷地、电梯、飞机、人群密集处等，并因此回避。",
                "对情境的恐惧：如害怕当众说话、社交聚会、看医生打针等，并因此回避。",
                "对事物的恐惧：如害怕特定动物、鲜血、黑暗等，并因此回避。",
            ],
            negative: "对日常环境和活动没有不合理的、强烈的恐惧感。即使有些情境让人紧张或不适应（如公开演讲），也能在准备后去面对，不会因此完全回避。",
            guidance: SymptomGuidance {
                self_help: &[
                    "识别自己回避的具体情境，区分危险程度和真实风险。",
                    "从压力较小的情境开始，逐步尝试接触而非完全回避。",
                    "在面对恐惧前，提前做好心理准备和放松练习。",
                ],
                watch_out: &[
                    "回避行为是否不断扩大，影响生活范围。",
                    "是否因为恐惧而放弃重要活动或机会。",
                ],
                seek_help_when: &[
                    "恐惧反应强烈且持续存在，难以自行面对。",
                    "回避行为已严重限制生活和出行。",
                ],
            }
        },
        paranoid_ideation: SymptomInterpretation {
            name: "偏执",
            terminology: "Paranoid Ideation",
            description: "指的是一种过度的、缺乏事实依据的怀疑和不信任。容易觉得别人对你有恶意、想伤害或欺骗你（例如，总觉得同事在背后针对你）。这种想法可能导致人际关系紧张。",
            positive: &[
                "总感觉别人对自己不怀好意或意图伤害。",
                "容易觉得自己被利用、欺骗或背叛。",
                "过度警觉，总觉得别人在议论或关注自己。",
                "很难相信他人，总是怀疑别人的动机。",
                "容易把别人中性的言行解读出恶意。",
                "对一些自认的“冒犯”耿耿于怀，难以放下。",
            ],
            negative: "能保持合理的警惕，但不会无故猜疑。能基本信任熟悉的人，并能相对客观地看待事情，不会轻易认定别人有恶意。",
            guidance: SymptomGuidance {
                self_help: &[
                    "当产生怀疑时，尝试寻找多种可能解释，而不只停留在最负面的推断。",
                    "把想法与可信任的人交流，有助于获得不同视角。",
                    "提醒自己：感受是真实的，但解释未必完全准确。",
                ],
                watch_out: &[
                    "怀疑和不信任是否持续加重，影响人际关系。",
                    "是否难以接受他人的解释或安慰。",
                ],
                seek_help_when: &[
                    "长期处于高度警觉和不安状态。",
                    "怀疑想法明显影响社交或工作功能。",
                ],
            }
        },
        psychoticism: SymptomInterpretation {
            name: "精神病性",
            terminology: "Psychoticism",
            description: "这里指的是一些与绝大多数人不同的、独特的感知或思维体验，这些体验在当事人看来非常真实，但可能与客观现实不一致。需要特别强调的是，这不一定意味着患有精神疾病，许多人在巨大压力或睡眠剥夺时也可能有短暂、轻微的类似体验。",
            positive: &[
                "感觉周围世界变得不真实，或者自己像在梦中。",
                "在清醒时，听到、看到或感觉到并不存在的东西。",
                "坚信一些在他人看来非常古怪、不符合事实的念头（如被特殊力量控制）。",
                "觉得自己的思想被广播、被插入或被偷走。",
                "从普通事件中解读出只针对自己的特殊含义（如觉得电视新闻在向自己发送秘密信息）。",
                "思维或说话方式变得混乱，让别人难以理解。",
            ],
            negative: "思维和感知与现实环境基本一致。想法连贯合理，能清晰地区分内心的想象和外部真实世界。",
            guidance: SymptomGuidance {
                self_help: &[
                    "关注近期的睡眠、压力和生活节奏，极端疲劳可能加重异常体验。",
                    "尽量保持与现实环境的连接，例如规律作息和日常交流。",
                    "不要独自承受困扰，及时向信任的人表达感受。",
                ],
                watch_out: &[
                    "感知或想法的异常是否变得频繁或强烈。",
                    "是否开始难以区分现实与内心体验。",
                ],
                seek_help_when: &[
                    "出现持续的幻觉、妄想或思维混乱。",
                    "这些体验已明显影响判断、行为或安全。",
                ],
            }
        },
        others: SymptomInterpretation {
            name: "其它（睡眠/食欲/自我价值）",
            terminology: "Others",
            description: "包含一些广泛影响心理状态的生活层面问题，如睡眠、食欲、自我价值感等。",
            positive: &[
                "显著的睡眠问题：难以入睡、易醒、早醒。",
                "食欲的显著变化：吃不下或暴饮暴食。",
                "强烈的、过度的自责和内疚感。",
                "感觉自己是个负担，让家人失望。",
                "感到非常孤独，与世隔绝。",
                "做任何决定（无论大小）都异常困难。",
                "对自己极度不满意，觉得自己很失败。",
                "感觉生活没有意义、没有目标。",
            ],
            negative: "睡眠和食欲基本规律、稳定。对自己有基本的接纳和满意度，能处理日常事务和决策，与外界保持一定的、令人舒适的联结感。",
            guidance: SymptomGuidance {
                self_help: &[
                    "优先关注最基础的生活需求，如睡眠、饮食和休息。",
                    "对自己保持相对宽容，避免用过于苛刻的标准评价自己。",
                    "尝试与他人保持最低限度的联系，减少孤立感。",
                ],
                watch_out: &[
                    "睡眠或食欲问题是否持续存在且加重。",
                    "自我否定和无价值感是否反复出现。",
                ],
                seek_help_when: &[
                    "基本生活节奏被严重打乱。",
                    "长期感到孤独、无意义或情绪低落。",
                ],
            }
        }
    },
};

pub const SYMPTOM_CHECKLIST_90: Scale<Interpretation, Question> = Scale {
    name: "症状自评量表",
    abbreviation: "SCL-90",
    primary_category: ScaleCategory::MentalHealth,
    related_categories: Some(&[
        ScaleCategory::Emotion,
        ScaleCategory::Somatic,
        ScaleCategory::Interpersonal,
        ScaleCategory::Behavior,
    ]),
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: None,
    warning: Some("本量表仅适用 16 岁以上的人群。"),
    formula_mode: Some(FormulaMode {
        operational_rule: OperationalRule::Multiply(1.25),
        integer: Some(Integer::Round),
    }),
    tags: Tag {
        info: Some(&["多症状"]),
        normal: None,
        warning: Some(&["16+"]),
        error: None,
    },
    interpretation: INTERPRETATION,
    questions: &[
        Question {
            title: "头痛",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "神经过敏，心中不踏实",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "头脑中有不必要的想法或字句盘旋",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsive,
        },
        Question {
            title: "头晕或晕倒",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "对异性的兴趣减退",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "对旁人责备求全",
            options: &OPTIONS,
            symptom: Symptom::InterpersonalSensitivity,
        },
        Question {
            title: "感到别人能控制自己的思想",
            options: &OPTIONS,
            symptom: Symptom::Psychoticism,
        },
        Question {
            title: "责怪别人制造麻烦",
            options: &OPTIONS,
            symptom: Symptom::ParanoidIdeation,
        },
        Question {
            title: "忘性大",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsive,
        },
        Question {
            title: "担心自己的衣饰整齐及仪态的端正",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsive,
        },
        Question {
            title: "容易烦恼和激动",
            options: &OPTIONS,
            symptom: Symptom::Hostility,
        },
        Question {
            title: "胸痛",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "害怕空旷的场所或街道",
            options: &OPTIONS,
            symptom: Symptom::PhobicAnxiety,
        },
        Question {
            title: "感到自己的精力下降，活动减慢",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "想结束自己的生命",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "听到旁人听不到的声音",
            options: &OPTIONS,
            symptom: Symptom::Psychoticism,
        },
        Question {
            title: "发抖",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "感到大多数人都不可信任",
            options: &OPTIONS,
            symptom: Symptom::ParanoidIdeation,
        },
        Question {
            title: "胃口不好",
            options: &OPTIONS,
            symptom: Symptom::Others,
        },
        Question {
            title: "容易哭泣",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "同异性相处时感到害羞不自在",
            options: &OPTIONS,
            symptom: Symptom::InterpersonalSensitivity,
        },
        Question {
            title: "感到受骗，中了圈套或有人想抓住您",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "无缘无故地突然感到害怕",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "自己不能控制地大发脾气",
            options: &OPTIONS,
            symptom: Symptom::Hostility,
        },
        Question {
            title: "怕单独出门",
            options: &OPTIONS,
            symptom: Symptom::PhobicAnxiety,
        },
        Question {
            title: "经常责怪自己",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "腰痛",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "感到难以完成任务",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsive,
        },
        Question {
            title: "感到孤独",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "感到苦闷",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "过分担忧",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "对事物不感兴趣",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "感到害怕",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "您的感情容易受到伤害",
            options: &OPTIONS,
            symptom: Symptom::InterpersonalSensitivity,
        },
        Question {
            title: "旁人能知道自己的私下想法",
            options: &OPTIONS,
            symptom: Symptom::Psychoticism,
        },
        Question {
            title: "感到别人不理解您、不同情您",
            options: &OPTIONS,
            symptom: Symptom::InterpersonalSensitivity,
        },
        Question {
            title: "感到人们对您不友好，不喜欢您",
            options: &OPTIONS,
            symptom: Symptom::InterpersonalSensitivity,
        },
        Question {
            title: "做事必须做得很慢，以保证做得正确",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsive,
        },
        Question {
            title: "心跳得很厉害",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "恶心或胃部不舒服",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "感到比不上他人",
            options: &OPTIONS,
            symptom: Symptom::InterpersonalSensitivity,
        },
        Question {
            title: "肌肉酸痛",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "感到有人在监视您、谈论您",
            options: &OPTIONS,
            symptom: Symptom::ParanoidIdeation,
        },
        Question {
            title: "难以入睡",
            options: &OPTIONS,
            symptom: Symptom::Others,
        },
        Question {
            title: "做事，必须反复检查",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsive,
        },
        Question {
            title: "难以作出决定",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsive,
        },
        Question {
            title: "怕乘电车、公共汽车、地铁或火车",
            options: &OPTIONS,
            symptom: Symptom::PhobicAnxiety,
        },
        Question {
            title: "呼吸有困难",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "一阵阵发冷或发热",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "因为感到害怕而避开某些东西、场合或活动",
            options: &OPTIONS,
            symptom: Symptom::PhobicAnxiety,
        },
        Question {
            title: "脑子变空了",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsive,
        },
        Question {
            title: "身体发麻或刺痛",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "喉咙有梗塞感",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "感到前途没有希望",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "不能集中注意力",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsive,
        },
        Question {
            title: "感到身体的某一部分软弱无力",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "感到紧张或容易紧张",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "感到手或脚发重",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "想到死亡的事",
            options: &OPTIONS,
            symptom: Symptom::Others,
        },
        Question {
            title: "吃得太多",
            options: &OPTIONS,
            symptom: Symptom::Others,
        },
        Question {
            title: "当别人看着自己或谈论自己时感到不自在",
            options: &OPTIONS,
            symptom: Symptom::InterpersonalSensitivity,
        },
        Question {
            title: "有一些不属于您自己的想法",
            options: &OPTIONS,
            symptom: Symptom::Psychoticism,
        },
        Question {
            title: "有想打人或伤害他人的冲动",
            options: &OPTIONS,
            symptom: Symptom::Hostility,
        },
        Question {
            title: "醒得太早",
            options: &OPTIONS,
            symptom: Symptom::Others,
        },
        Question {
            title: "必须反复洗手、点数目或触摸某些东西",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsive,
        },
        Question {
            title: "睡得不稳不深",
            options: &OPTIONS,
            symptom: Symptom::Others,
        },
        Question {
            title: "有想摔坏或破坏东西的冲动",
            options: &OPTIONS,
            symptom: Symptom::Hostility,
        },
        Question {
            title: "有一些别人没有的想法或念头",
            options: &OPTIONS,
            symptom: Symptom::ParanoidIdeation,
        },
        Question {
            title: "感到对别人神经过敏",
            options: &OPTIONS,
            symptom: Symptom::InterpersonalSensitivity,
        },
        Question {
            title: "在商店或电影院等人多的地方感到不自在",
            options: &OPTIONS,
            symptom: Symptom::PhobicAnxiety,
        },
        Question {
            title: "感到任何事情都很困难",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "一阵阵恐惧或惊恐",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "感到公共场合吃东西很不舒服",
            options: &OPTIONS,
            symptom: Symptom::InterpersonalSensitivity,
        },
        Question {
            title: "经常与人争论",
            options: &OPTIONS,
            symptom: Symptom::Hostility,
        },
        Question {
            title: "单独一人时神经很紧张",
            options: &OPTIONS,
            symptom: Symptom::PhobicAnxiety,
        },
        Question {
            title: "别人对您的成绩没有做出恰当的评价",
            options: &OPTIONS,
            symptom: Symptom::ParanoidIdeation,
        },
        Question {
            title: "即使和别人在一起也感到孤单",
            options: &OPTIONS,
            symptom: Symptom::Psychoticism,
        },
        Question {
            title: "感到坐立不安心神不定",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "感到自己没有什么价值",
            options: &OPTIONS,
            symptom: Symptom::Depression,
        },
        Question {
            title: "感到熟悉的东西变成陌生或不像真的",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "大叫或摔东西",
            options: &OPTIONS,
            symptom: Symptom::Hostility,
        },
        Question {
            title: "害怕会在公共场合晕倒",
            options: &OPTIONS,
            symptom: Symptom::PhobicAnxiety,
        },
        Question {
            title: "感到别人想占自己的便宜",
            options: &OPTIONS,
            symptom: Symptom::ParanoidIdeation,
        },
        Question {
            title: "为一些有关性的想法而苦恼",
            options: &OPTIONS,
            symptom: Symptom::Psychoticism,
        },
        Question {
            title: "您认为应该因为自己的过错而受到惩罚",
            options: &OPTIONS,
            symptom: Symptom::Psychoticism,
        },
        Question {
            title: "感到要很快把事情做完",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "感到自己的身体有严重问题",
            options: &OPTIONS,
            symptom: Symptom::Psychoticism,
        },
        Question {
            title: "从未感到和其他人很亲近",
            options: &OPTIONS,
            symptom: Symptom::Psychoticism,
        },
        Question {
            title: "感到自己有罪",
            options: &OPTIONS,
            symptom: Symptom::Others,
        },
        Question {
            title: "感到自己的脑子有毛病",
            options: &OPTIONS,
            symptom: Symptom::Psychoticism,
        },
    ],
};
