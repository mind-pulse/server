use serde::Serialize;

use crate::scale::category::ScaleCategory;
use crate::scale::common::{
    PlainText, Question, QuestionOption, Scale, SentenceItem, Status, Tag, Texts,
};

const INTRODUCTION: Texts = &[
    &[SentenceItem::Plain("汉密尔顿抑郁量表（Hamilton Depression Scale，HAMD）是由 Hamilton 编制，是临床上评定抑郁状态时应用得最为普遍的量表。")], 
    &[SentenceItem::Plain( "量表是 24 项版本，方法简单，标准明确，便于掌握。适用于有抑郁症状的成人。总分能够较好的反映疾病严重程度，也能很好的衡量治疗效果，是经典和被公认的抑郁评定量表。")],
];

const INSTRUCTION: Texts = &[
    &[SentenceItem::Plain("HAMD大部分项目采用0~4分的5级评分法。各级的标准为：（0）无；（1）轻度；（2）中度；（3）重度；（4）极重度。少数项目采用 0~2 分的 3 级评分法，其分级的标准为：（0）无；（1）轻~中度；（2）重度。")],
    &[SentenceItem::Plain("应由经过训练的两名评定员对被评定者进行汉密尔顿抑郁量表联合检查。一般采用交谈与观察方式，待检查结束后，两名评定员分别独立评分。若需比较治疗前后抑郁症状和病情的变化，则于入组时，评定当时或入组前一周的情况，治疗后 2-6 周，再次评定，以资比较。")],
];

#[derive(Debug, Serialize)]
pub struct InterpretationItem {
    range: [u8; 2],
    description: PlainText,
    status: Status,
}

pub const HAMILTON_DEPRESSION_SCALE: Scale<&[InterpretationItem], Question> = Scale {
    id: 10,
    name: "汉密尔顿抑郁量表",
    description: "他评量表，用于评估抑郁状态",
    abbreviation: "HAMD",
    primary_category: ScaleCategory::Emotion,
    related_categories: Some(&[ScaleCategory::MentalHealth, ScaleCategory::Somatic]),
    idea: None,
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    references: Some(&["张作记主编.《行为医学量表手册》（光盘版）[M].中华医学电子音像出版社.2005年"]),
    warning: Some("此为量表非自评，仅供心理科和精神科医生、实习生使用，如需自评抑郁状态，请使用抑郁自评量表（SDS）"),
    formula_mode: None,
    tags: &Tag{ info: None, normal: None, warning: None, error: Some(&["医用"]) },
    interpretation: &[
        InterpretationItem {
            range: [0, 8],
            description: "正常",
            status: Status::Normal,
        },
        InterpretationItem {
            range: [8, 20],
            description: "可能有抑郁症",
            status: Status::Mild,
        },
        InterpretationItem {
            range: [20, 35],
            description: "肯定有抑郁症",
            status: Status::Moderate,
        },
        InterpretationItem {
            range: [35, 76],
            description: "严重抑郁症",
            status: Status::Major,
        },
    ],
    questions: &[
        Question {
            title: "抑郁情绪",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "只在问到时才诉述",
                    point: 1,
                },
                QuestionOption {
                    text: "在言语中自发地表达",
                    point: 2,
                },
                QuestionOption {
                    text: "不用言语也可从表情、姿势、声音或欲哭中流露出这种情绪",
                    point: 3,
                },
                QuestionOption {
                    text: "病人的自发和非自发语言（表情、动作），几乎完全表现为这种情绪",
                    point: 4,
                },
            ],
        },
        Question {
            title: "有罪感",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "责备自己，感到自己已连累他人",
                    point: 1,
                },
                QuestionOption {
                    text: "认为自己犯了罪，或反复思考以往的过失和错误",
                    point: 2,
                },
                QuestionOption {
                    text: "认为目前的疾病，是对自己错误的惩罚，或有罪恶妄想",
                    point: 3,
                },
                QuestionOption {
                    text: "罪恶妄想伴有指责或威胁性幻觉",
                    point: 4,
                },
            ],
        },
        Question {
            title: "自杀",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "觉得活着没有意义",
                    point: 1,
                },
                QuestionOption {
                    text: "希望自己已经死去，或常想到与死有关的事",
                    point: 2,
                },
                QuestionOption {
                    text: "消极观念（自杀念头）",
                    point: 3,
                },
                QuestionOption {
                    text: "有严重自杀行为",
                    point: 4,
                },
            ],
        },
        Question {
            title: "入睡困难",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "主诉有时有入睡困难，即上床后半小时仍不能入睡",
                    point: 1,
                },
                QuestionOption {
                    text: "主诉每晚均有入睡困难",
                    point: 2,
                },
            ],
        },
        Question {
            title: "睡眠不深",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "睡眠浅多恶梦",
                    point: 1,
                },
                QuestionOption {
                    text: "半夜（晚上12点以前）曾醒来（不包括上厕所）",
                    point: 2,
                },
            ],
        },
        Question {
            title: "早醒",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "有早醒，比平时早醒1小时，但能重新入睡",
                    point: 1,
                },
                QuestionOption {
                    text: "早醒后无法重新入睡",
                    point: 2,
                },
            ],
        },
        Question {
            title: "工作和兴趣",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无异常",
                    point: 0,
                },
                QuestionOption {
                    text: "提问时才诉述",
                    point: 1,
                },
                QuestionOption {
                    text: "自发地直接或间接表达对活动、工作或学习失去兴趣，如感到没精打采，犹豫不决，不能坚持或需强迫自己去工作或活动",
                    point: 2,
                },
                QuestionOption {
                    text: "病室劳动或娱乐不满3小时",
                    point: 3,
                },
                QuestionOption {
                    text: "因目前的疾病而停止工作，住院患者不参加任何活动或者没有他人帮助便不能完成病室日常事务",
                    point: 4,
                },
            ],
        },
        Question {
            title: "迟缓：指思维和语言缓慢，注意力难以集中，主动性减退。",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "精神检查中发现轻度迟缓",
                    point: 1,
                },
                QuestionOption {
                    text: "精神检查中发现明显迟缓",
                    point: 2,
                },
                QuestionOption {
                    text: "精神检查进行困难",
                    point: 3,
                },
                QuestionOption {
                    text: "完全不能回答问题（木僵）",
                    point: 4,
                },
            ],
        },
        Question {
            title: "激越",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "检查时表现的有些心神不定",
                    point: 1,
                },
                QuestionOption {
                    text: "明显的心神不定或小动作多",
                    point: 2,
                },
                QuestionOption {
                    text: "不能静坐，检查中曾站立",
                    point: 3,
                },
                QuestionOption {
                    text: "搓手，咬手指，扯头发，咬嘴唇",
                    point: 4,
                },
            ],
        },
        Question {
            title: "精神性焦虑",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "问到时才诉述",
                    point: 1,
                },
                QuestionOption {
                    text: "自发地表达",
                    point: 2,
                },
                QuestionOption {
                    text: "表情和言谈流露明显忧虑",
                    point: 3,
                },
                QuestionOption {
                    text: "明显惊恐",
                    point: 4,
                },
            ],
        },
        Question {
            title: "躯体性焦虑：指焦虑的生理症状，包括口干、腹胀、腹泻、打呃、腹绞痛、心悸、头痛、过度换气和叹息、以及尿频和出汗等。",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "轻度",
                    point: 1,
                },
                QuestionOption {
                    text: "中度，有肯定的上述症状",
                    point: 2,
                },
                QuestionOption {
                    text: "重度，上述症状严重，影响生活或需加处理",
                    point: 3,
                },
                QuestionOption {
                    text: "严重影响生活和活动",
                    point: 4,
                },
            ],
        },
        Question {
            title: "胃肠道症状",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "食欲减退，但不需他人鼓励便自行进食",
                    point: 1,
                },
                QuestionOption {
                    text: "进食需他人催促或请求或需要应用泻药或助消化药",
                    point: 2,
                },
            ],
        },
        Question {
            title: "全身症状",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "四肢、背部或颈部沉重感，背痛，头痛，肌肉疼痛，全身乏力或疲倦",
                    point: 1,
                },
                QuestionOption {
                    text: "上述症状明显",
                    point: 2,
                },
            ],
        },
        Question {
            title: "性症状：指性欲减退、月经紊乱等。",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无症状，或者不能肯定，或者该项对被评者不适合。",
                    point: 0,
                },
                QuestionOption {
                    text: "轻度",
                    point: 1,
                },
                QuestionOption {
                    text: "重度",
                    point: 2,
                },
            ],
        },
        Question {
            title: "疑病",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "对身体过分关注",
                    point: 1,
                },
                QuestionOption {
                    text: "反复考虑健康问题",
                    point: 2,
                },
                QuestionOption {
                    text: "有疑病妄想",
                    point: 3,
                },
                QuestionOption {
                    text: "伴幻觉的疑病妄想",
                    point: 4,
                },
            ],
        },
        Question {
            title: "体重减轻",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "一周内体重减轻1斤以上",
                    point: 1,
                },
                QuestionOption {
                    text: "一周内体重减轻2斤以上",
                    point: 2,
                },
            ],
        },
        Question {
            title: "自知力",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "知道自己有病，表现为忧郁",
                    point: 0,
                },
                QuestionOption {
                    text: "知道自己有病，但归于伙食太差、环境问题、工作过忙、病毒感染或需要休息等",
                    point: 1,
                },
                QuestionOption {
                    text: "完全否认有病",
                    point: 2,
                },
            ],
        },
        Question {
            title: "日夜变化（如果症状在早晨或傍晚加重，先指出哪一种，然后按其变化程度评分）",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "轻度变化",
                    point: 1,
                },
                QuestionOption {
                    text: "重度变化",
                    point: 2,
                },
            ],
        },
        Question {
            title: "人格解体或现实解体：指非真实感或虚无妄想。",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "问及时才诉述",
                    point: 1,
                },
                QuestionOption {
                    text: "自发诉述",
                    point: 2,
                },
                QuestionOption {
                    text: "有虚无妄想",
                    point: 3,
                },
                QuestionOption {
                    text: "伴幻觉的虚无妄想",
                    point: 4,
                },
            ],
        },
        Question {
            title: "偏执症状",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "有猜疑",
                    point: 1,
                },
                QuestionOption {
                    text: "有关系观念",
                    point: 2,
                },
                QuestionOption {
                    text: "有关系妄想或被害妄想",
                    point: 3,
                },
                QuestionOption {
                    text: "伴有幻觉的关系妄想或被害妄想",
                    point: 4,
                },
            ],
        },
        Question {
            title: "强迫症状：指强迫思维和强迫行为。",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "问及时才诉述",
                    point: 1,
                },
                QuestionOption {
                    text: "自发诉述",
                    point: 2,
                },
            ],
        },
        Question {
            title: "能力减退感",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "仅于提问时方引出主观体验",
                    point: 1,
                },
                QuestionOption {
                    text: "病人主动表示能力减退感",
                    point: 2,
                },
                QuestionOption {
                    text: "需鼓励、指导和安慰才能完成病室日常事务或个人卫生",
                    point: 3,
                },
                QuestionOption {
                    text: "穿衣、梳洗、进食、铺床或个人卫生均需他人协助",
                    point: 4,
                },
            ],
        },
        Question {
            title: "绝望感",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "有时怀疑“情况是否会好转”，但解释后能接受。",
                    point: 1,
                },
                QuestionOption {
                    text: "持续感到“没有希望”，但解释后能接受",
                    point: 2,
                },
                QuestionOption {
                    text: "对未来感到灰心、悲观和绝望，解释后不能排除",
                    point: 3,
                },
                QuestionOption {
                    text: "自动反复诉述“我的病不会好了”或诸如此类的情况",
                    point: 4,
                },
            ],
        },
        Question {
            title: "自卑感",
            is_multiple: false,
            options: &[
                QuestionOption {
                    text: "无",
                    point: 0,
                },
                QuestionOption {
                    text: "仅在询问时诉述有自卑感（我不如他人）",
                    point: 1,
                },
                QuestionOption {
                    text: "自动诉述有自卑感（我不如他人）",
                    point: 2,
                },
                QuestionOption {
                    text: "病人主动诉述：“我一无是处”或“低人一等”，与评2分者只是程度的差别",
                    point: 3,
                },
                QuestionOption {
                    text: "自卑感达妄想的程度，例如“我是废物”或类似情况",
                    point: 4,
                },
            ],
        },
    ],
};
