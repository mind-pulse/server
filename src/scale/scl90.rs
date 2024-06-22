use serde::Serialize;

use super::{FormulaMode, HTMLElement, Integer, OperationalRule, QuestionOption, Scale, SentenceItem, Tag, Texts};

#[derive(Debug, Serialize, Hash, Eq, PartialEq)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
enum Symptom {
    /// 驱体化
    Somatization,
    /// 强迫症状
    ObsessiveCompulsiive,
    /// 人际关系敏感
    SensitiveOfInterpersonalRelationship,
    /// 抑郁
    Despondent,
    /// 焦虑
    Anxiety,
    /// 敌对
    Hostility,
    /// 恐怖
    Phobia,
    /// 偏执
    Bigotry,
    /// 精神病性
    Psychotic,
    /// 其他
    Other,
}

#[derive(Debug, Serialize)]
pub struct Question {
    title: &'static str,
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
    comparison_operator: &'static str,
}

#[derive(Debug, Serialize)]
struct Positive {
    total: Rule,
    positive_amount: Rule,
    any_symptom_average: Rule,
}

#[derive(Debug, Serialize)]
struct SymptomInterpretation {
    name: &'static str,
    symptom: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "UPPERCASE"))]
struct Symptoms {
    somatization:SymptomInterpretation ,
    obsessive_compulsiive:SymptomInterpretation ,
    sensitive_of_interpersonal_relationship:SymptomInterpretation ,
    despondent:SymptomInterpretation ,
    anxiety:SymptomInterpretation ,
    hostility:SymptomInterpretation ,
    phobia:SymptomInterpretation ,
    bigotry:SymptomInterpretation ,
    psychotic:SymptomInterpretation ,
    other:SymptomInterpretation ,
}

#[derive(Debug, Serialize)]
pub struct Interpretation {
    positive: Positive,
    symptoms: Symptoms,
}

const INTRODUCTION: Texts = &[
    &[
        SentenceItem::Plain("《症状自评量表SCL90》是世界上"),
        SentenceItem::HTMLElement(HTMLElement::Strong("最著名")),
        SentenceItem::Plain("的心理健康测试量表之一，是当前使用最为广泛的精神障碍和心理疾病门诊检查量表，将协助您从"),
        SentenceItem::HTMLElement(HTMLElement::Strong("十个方面")),
        SentenceItem::Plain("来了解自己的心理健康程度。"),
    ]
];

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
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: None,
    references: None,
    warning: Some("本量表仅适用 16 岁以上的人群。"),
    formula_mode: Some(FormulaMode{
        operational_rule: OperationalRule::Multiply(1.25),
        integer: Some(Integer::Round),
    }),
    tags: Tag{ info: Some(&["多症状"]), normal: Some(&["自评"]), warning: Some(&["16+"]), error: None },
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
                symptom: "主要反映身体不适感，包括心血管、胃肠道、呼吸和其他系统的不适，和头痛、背痛、肌肉酸痛，以及焦虑等躯体不适表现。"
            }, 
            obsessive_compulsiive: SymptomInterpretation { 
                name: "强迫症状", 
                symptom: "主要指那些明知没有必要，但又无法摆脱的无意义的思想、冲动和行为，还有一些比较一般的认知障碍的行为征象也在这一因子中反映。"
            },
            sensitive_of_interpersonal_relationship: SymptomInterpretation { 
                name: "人际关系敏感", 
                symptom: "主要是指某些人际的不自在与自卑感，特别是与其他人相比较时更加突出。在人际交往中的自卑感，心神不安，明显的不自在，以及人际交流中的不良自我暗示，消极的期待等是这方面症状的典型原因。" }, 
            despondent: SymptomInterpretation {
                name: "抑郁", 
                symptom: "苦闷的情感与心境为代表性症状，还以生活兴趣的减退，动力缺乏，活力丧失等为特征。还表现出失望、悲观以及与抑郁相联系的认知和躯体方面的感受，另外，还包括有关死亡的思想和自杀观念。"
            }, 
            anxiety: SymptomInterpretation { 
                name: "焦虑", 
                symptom: "一般指那些烦躁，坐立不安，神经过敏，紧张以及由此产生的躯体征象，如震颤等。" 
            }, 
            hostility: SymptomInterpretation { 
                name: "敌对", 
                symptom: "主要从三方面来反映敌对的表现：思想、感情及行为。其项目包括厌烦的感觉，摔物，争论直到不可控制的脾气暴发等各方面。" 
            }, 
            phobia: SymptomInterpretation { 
                name: "恐怖", 
                symptom: "恐惧的对象包括出门旅行，空旷场地，人群或公共场所和交通工具。此外，还有社交恐怖。" 
            },
            bigotry: SymptomInterpretation { 
                name: "偏执",
                symptom: "主要指投射性思维，敌对，猜疑，妄想，被动体验和夸大等。" 
            }, 
            psychotic: SymptomInterpretation { 
                name: "精神病性", 
                symptom: "反映各式各样的急性症状和行为，即限定不严的精神病性过程的症状表现。" 
            }, 
            other: SymptomInterpretation { 
                name: "其它项目", 
                symptom: "作为附加项目或其他，作为第10个因子来处理，以便使各因子分之和等于总分。" 
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
            symptom: Symptom::ObsessiveCompulsiive,
        },
        Question {
            title: "头晕或晕倒",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "对异性的兴趣减退",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
        },
        Question {
            title: "对旁人责备求全",
            options: &OPTIONS,
            symptom: Symptom::SensitiveOfInterpersonalRelationship,
        },
        Question {
            title: "感到别人能控制自己的思想",
            options: &OPTIONS,
            symptom: Symptom::Psychotic,
        },
        Question {
            title: "责怪别人制造麻烦",
            options: &OPTIONS,
            symptom: Symptom::Bigotry,
        },
        Question {
            title: "忘性大",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsiive,
        },
        Question {
            title: "担心自己的衣饰整齐及仪态的端正",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsiive,
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
            symptom: Symptom::Phobia,
        },
        Question {
            title: "感到自己的精力下降，活动减慢",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
        },
        Question {
            title: "想结束自己的生命",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
        },
        Question {
            title: "听到旁人听不到的声音",
            options: &OPTIONS,
            symptom: Symptom::Psychotic,
        },
        Question {
            title: "发抖",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "感到大多数人都不可信任",
            options: &OPTIONS,
            symptom: Symptom::Bigotry,
        },
        Question {
            title: "胃口不好",
            options: &OPTIONS,
            symptom: Symptom::Other,
        },
        Question {
            title: "容易哭泣",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
        },
        Question {
            title: "同异性相处时感到害羞不自在",
            options: &OPTIONS,
            symptom: Symptom::SensitiveOfInterpersonalRelationship,
        },
        Question {
            title: "感到受骗，中了圈套或有人想抓住您",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
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
            symptom: Symptom::Phobia,
        },
        Question {
            title: "经常责怪自己",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
        },
        Question {
            title: "腰痛",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "感到难以完成任务",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsiive,
        },
        Question {
            title: "感到孤独",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
        },
        Question {
            title: "感到苦闷",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
        },
        Question {
            title: "过分担忧",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
        },
        Question {
            title: "对事物不感兴趣",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
        },
        Question {
            title: "感到害怕",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "您的感情容易受到伤害",
            options: &OPTIONS,
            symptom: Symptom::SensitiveOfInterpersonalRelationship,
        },
        Question {
            title: "旁人能知道自己的私下想法",
            options: &OPTIONS,
            symptom: Symptom::Psychotic,
        },
        Question {
            title: "感到别人不理解您、不同情您",
            options: &OPTIONS,
            symptom: Symptom::SensitiveOfInterpersonalRelationship,
        },
        Question {
            title: "感到人们对您不友好，不喜欢您",
            options: &OPTIONS,
            symptom: Symptom::SensitiveOfInterpersonalRelationship,
        },
        Question {
            title: "做事必须做得很慢，以保证做得正确",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsiive,
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
            symptom: Symptom::SensitiveOfInterpersonalRelationship,
        },
        Question {
            title: "肌肉酸痛",
            options: &OPTIONS,
            symptom: Symptom::Somatization,
        },
        Question {
            title: "感到有人在监视您、谈论您",
            options: &OPTIONS,
            symptom: Symptom::Bigotry,
        },
        Question {
            title: "难以入睡",
            options: &OPTIONS,
            symptom: Symptom::Other,
        },
        Question {
            title: "做事，必须反复检查",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsiive,
        },
        Question {
            title: "难以作出决定",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsiive,
        },
        Question {
            title: "怕乘电车、公共汽车、地铁或火车",
            options: &OPTIONS,
            symptom: Symptom::Phobia,
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
            symptom: Symptom::Phobia,
        },
        Question {
            title: "脑子变空了",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsiive,
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
            symptom: Symptom::Despondent,
        },
        Question {
            title: "不能集中注意力",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsiive,
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
            symptom: Symptom::Other,
        },
        Question {
            title: "吃得太多",
            options: &OPTIONS,
            symptom: Symptom::Other,
        },
        Question {
            title: "当别人看着自己或谈论自己时感到不自在",
            options: &OPTIONS,
            symptom: Symptom::SensitiveOfInterpersonalRelationship,
        },
        Question {
            title: "有一些不属于您自己的想法",
            options: &OPTIONS,
            symptom: Symptom::Psychotic,
        },
        Question {
            title: "有想打人或伤害他人的冲动",
            options: &OPTIONS,
            symptom: Symptom::Hostility,
        },
        Question {
            title: "醒得太早",
            options: &OPTIONS,
            symptom: Symptom::Other,
        },
        Question {
            title: "必须反复洗手、点数目或触摸某些东西",
            options: &OPTIONS,
            symptom: Symptom::ObsessiveCompulsiive,
        },
        Question {
            title: "睡得不稳不深",
            options: &OPTIONS,
            symptom: Symptom::Other,
        },
        Question {
            title: "有想摔坏或破坏东西的冲动",
            options: &OPTIONS,
            symptom: Symptom::Hostility,
        },
        Question {
            title: "有一些别人没有的想法或念头",
            options: &OPTIONS,
            symptom: Symptom::Bigotry,
        },
        Question {
            title: "感到对别人神经过敏",
            options: &OPTIONS,
            symptom: Symptom::SensitiveOfInterpersonalRelationship,
        },
        Question {
            title: "在商店或电影院等人多的地方感到不自在",
            options: &OPTIONS,
            symptom: Symptom::Phobia,
        },
        Question {
            title: "感到任何事情都很困难",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
        },
        Question {
            title: "一阵阵恐惧或惊恐",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "感到公共场合吃东西很不舒服",
            options: &OPTIONS,
            symptom: Symptom::SensitiveOfInterpersonalRelationship,
        },
        Question {
            title: "经常与人争论",
            options: &OPTIONS,
            symptom: Symptom::Hostility,
        },
        Question {
            title: "单独一人时神经很紧张",
            options: &OPTIONS,
            symptom: Symptom::Phobia,
        },
        Question {
            title: "别人对您的成绩没有做出恰当的评价",
            options: &OPTIONS,
            symptom: Symptom::Bigotry,
        },
        Question {
            title: "即使和别人在一起也感到孤单",
            options: &OPTIONS,
            symptom: Symptom::Psychotic,
        },
        Question {
            title: "感到坐立不安心神不定",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "感到自己没有什么价值",
            options: &OPTIONS,
            symptom: Symptom::Despondent,
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
            symptom: Symptom::Phobia,
        },
        Question {
            title: "感到别人想占自己的便宜",
            options: &OPTIONS,
            symptom: Symptom::Bigotry,
        },
        Question {
            title: "为一些有关性的想法而苦恼",
            options: &OPTIONS,
            symptom: Symptom::Psychotic,
        },
        Question {
            title: "您认为应该因为自己的过错而受到惩罚",
            options: &OPTIONS,
            symptom: Symptom::Psychotic,
        },
        Question {
            title: "感到要很快把事情做完",
            options: &OPTIONS,
            symptom: Symptom::Anxiety,
        },
        Question {
            title: "感到自己的身体有严重问题",
            options: &OPTIONS,
            symptom: Symptom::Psychotic,
        },
        Question {
            title: "从未感到和其他人很亲近",
            options: &OPTIONS,
            symptom: Symptom::Psychotic,
        },
        Question {
            title: "感到自己有罪",
            options: &OPTIONS,
            symptom: Symptom::Other,
        },
        Question {
            title: "感到自己的脑子有毛病",
            options: &OPTIONS,
            symptom: Symptom::Psychotic,
        },
    ],
};
