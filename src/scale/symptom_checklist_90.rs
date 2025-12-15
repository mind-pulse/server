use serde::Serialize;

use crate::scale::{PlainText, PlainTexts, ScaleCategory};

use super::{
    FormulaMode, HTMLElement, Integer, OperationalRule, QuestionOption, Scale, SentenceItem, Tag,
    Texts,
};

#[derive(Debug, Serialize, Hash, Eq, PartialEq)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
enum Symptom {
    /// 驱体化
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
struct SymptomInterpretation {
    name: PlainText,
    description: PlainText,
    positive: PlainTexts,
    negative: PlainText,
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
    formula_mode: Some(FormulaMode{
        operational_rule: OperationalRule::Multiply(1.25),
        integer: Some(Integer::Round),
    }),
    tags: Tag{ info: Some(&["多症状"]), normal: None, warning: Some(&["16+"]), error: None },
    interpretation: Interpretation { 
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
                description: "指心理上的压力、烦恼或情绪问题，没有直接表现为心情不好，反而转化成了身体上的不舒服。通常表现多样，但去医院做检查，常常发现不了明确的生理疾病。",
                positive: &[
                    "反复或持续的身体疼痛（如头痛、背痛、肌肉酸痛）。",
                    "肠胃不适，比如没来由的胃痛、胀气、腹泻或便秘。",
                    "心慌、胸闷，或者感觉呼吸不畅。",
                    "经常感到头晕、眼花或乏力。",
                    "睡眠问题，如失眠、多梦、醒后不解乏。",
                ],
                negative: "身体感受基本正常，偶尔的不适（如累了酸痛、饿了的胃痛）能明确找到原因，并且不影响日常生活。"
            }, 
            obsessive_compulsive: SymptomInterpretation {
                name: "强迫症状", 
                description: "指头脑中反复冒出一些自己不想要的想法，或者忍不住想做某些重复动作。自己也知道这些想法或行为没必要，但就是控制不住，不做就会非常焦虑。",
                positive: &[
                    "总控制不住地想一些事（如是否锁门、是否脏、是否犯错）。",
                    "有强烈的冲动想去做某个特定动作（如触摸、排列）。",
                    "重复性的行为（如反复洗手、检查、计数）。",
                    "做事必须按照某种固定的顺序或规则，否则就不安心。",
                    "过分关注细节、对称或整洁。",
                    "害怕自己会失控，做出伤害自己或别人的事。",
                ],
                negative: "思维灵活，能专注在当下重要的事情上。日常习惯让生活有序，但不会因为这些习惯感到强烈的焦虑或被控制。"
            },
            interpersonal_sensitivity: SymptomInterpretation { 
                name: "人际关系敏感", 
                description: "指在和人打交道时特别容易感到紧张、不自信。可能会非常在意别人的看法，别人的一句话、一个眼神就能让你琢磨半天，担心对方是不是不喜欢自己，从而想要回避社交。",
                positive: &[
                    "与人相处时感到紧张、不自在。",
                    "害怕在别人面前出丑或被批评。",
                    "总觉得别人在看轻、拒绝或误解自己。",
                    "社交结束后，反复回想和检查自己的表现。",
                    "很难对别人说“不”或表达不同意见。",
                    "倾向于回避聚会、会议等需要社交的场合。",
                ],
                negative: "在大多数社交场合能感到自然放松，可以比较自信地表达自己。会在意他人的评价，但不会过度纠结或因此严重影响心情。"
            }, 
            depression: SymptomInterpretation {
                name: "抑郁", 
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
                negative: "情绪总体平稳，有正常的喜怒哀乐。遇到挫折会低落，但能较快恢复。对生活保有基本的热情和期待。"
            }, 
            anxiety: SymptomInterpretation {
                name: "焦虑", 
                description: "指的是一种过度的、难以控制的紧张和担心。不仅心里总想着“万一……怎么办”，身体也处于紧绷状态（如心慌、肌肉紧张）。这种担忧常常和现实中的问题不成比例。",
                positive: &[
                    "持续感到紧张、不安，很难放松下来。",
                    "对未来可能发生的不好的事情过度思虑。",
                    "身体紧绷感，如肩颈僵硬、紧咬牙关。",
                    "心慌、心跳加快、手抖或出汗。",
                    "坐立不安，静不下来。",
                    "很容易被一点声响或小事惊吓到。",
                ],
                negative: "在日常生活压力下，能保持相对平静。会对真正有风险的事情担心，但不会整天被各种“万一”所困，大多数时间身心能放松。"
            }, 
            hostility: SymptomInterpretation {
                name: "敌对", 
                description: "指的是一种对他人的、持续的负面态度，表现为容易烦躁、发怒、怨恨，甚至有想争吵或破坏的冲动。这种情绪的强烈程度和持续时间可能超出一般情况。",
                positive: &[
                    "脾气变得急躁，一点小事就容易发火。",
                    "内心常感到愤愤不平，对他人怀有怨恨。",
                    "总想与人争论，习惯性反驳别人。",
                    "有摔东西、破坏物品的冲动。",
                    "容易把别人的行为往坏处想。",
                    "很难原谅别人的过错。",
                ],
                negative: "能恰当地表达不满和不同意见。大多数时候能保持冷静、客观，面对冲突时能以沟通解决问题为主，不易被激怒。"
            }, 
            phobic_anxiety: SymptomInterpretation { 
                name: "恐怖", 
                description: "指的是对某些特定的场所、事物或情境（如高处、人群、封闭空间）产生一种强烈的、不合理的恐惧。这种恐惧感非常真实，以至于你会极力去回避那些场合，这可能会明显干扰你的正常生活。",
                positive: &[
                    "对场所的恐惧：如害怕空旷地、电梯、飞机、人群密集处等，并因此回避。",
                    "对情境的恐惧：如害怕当众说话、社交聚会、看医生打针等，并因此回避。",
                    "对事物的恐惧：如害怕特定动物、鲜血、黑暗等，并因此回避。",
                ],
                negative: "对日常环境和活动没有不合理的、强烈的恐惧感。即使有些情境让人紧张或不适应（如公开演讲），也能在准备后去面对，不会因此完全回避。"
            },
            paranoid_ideation: SymptomInterpretation { 
                name: "偏执",
                description: "指的是一种过度的、缺乏事实依据的怀疑和不信任。容易觉得别人对你有恶意、想伤害或欺骗你（例如，总觉得同事在背后针对你）。这种想法可能导致人际关系紧张。",
                positive: &[
                    "总感觉别人对自己不怀好意或意图伤害。",
                    "容易觉得自己被利用、欺骗或背叛。",
                    "过度警觉，总觉得别人在议论或关注自己。",
                    "很难相信他人，总是怀疑别人的动机。",
                    "容易把别人中性的言行解读出恶意。",
                    "对一些自认的“冒犯”耿耿于怀，难以放下。",
                ],
                negative: "能保持合理的警惕，但不会无故猜疑。能基本信任熟悉的人，并能相对客观地看待事情，不会轻易认定别人有恶意。"
            }, 
            psychoticism: SymptomInterpretation { 
                name: "精神病性", 
                description: "这里指的是一些与绝大多数人不同的、独特的感知或思维体验，这些体验在当事人看来非常真实，但可能与客观现实不一致。需要特别强调的是，这不一定意味着患有精神疾病，许多人在巨大压力或睡眠剥夺时也可能有短暂、轻微的类似体验。",
                positive: &[
                    "感觉周围世界变得不真实，或者自己像在梦中。",
                    "在清醒时，听到、看到或感觉到并不存在的东西。",
                    "坚信一些在他人看来非常古怪、不符合事实的念头（如被特殊力量控制）。",
                    "觉得自己的思想被广播、被插入或被偷走。",
                    "从普通事件中解读出只针对自己的特殊含义（如觉得电视新闻在向自己发送秘密信息）。",
                    "思维或说话方式变得混乱，让别人难以理解。",
                ],
                negative: "思维和感知与现实环境基本一致。想法连贯合理，能清晰地区分内心的想象和外部真实世界。"
            }, 
            others: SymptomInterpretation { 
                name: "其它", 
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
                negative: "睡眠和食欲基本规律、稳定。对自己有基本的接纳和满意度，能处理日常事务和决策，与外界保持一定的、令人舒适的联结感。"
            }
        },
    },
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
