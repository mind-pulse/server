use serde::Serialize;

use super::{Scale, Tag, Text, Characteristic, QuestionOption};

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
    title: Text,
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
    name: Text,
    characteristic: Characteristic,
    occupations: Text,
}

#[derive(Serialize)]
struct SecondPersonalityFactor {
    key: Text,
    name: Text,
    expression: Text,
    characteristic: Characteristic,
}

#[derive(Serialize)]
pub struct Interpretation {
    norm: Norm,
    normal_range: [u8; 2],
    first_personality_factor: &'static [FirstPersonalityFactor; 16],
    second_personality_factor: &'static [SecondPersonalityFactor; 4],
}

pub const SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE: Scale<Interpretation, Question> = Scale {
    name: "卡特尔16种人格因素问卷",
    abbreviation: "16PF",
    introduction: &[
        "本测验是美国伊利诺州立大学人格及能力研究所卡特尔（Catell）教授编制的。卡特尔根据自己的人格特质理论，运用因素分析方法编制了这一测验。",
        "卡特尔认为：人的行为之所以具有一致性和规律性就是因为每一个人都具有根源特质。为了测量4501个用来描述人类行为的词汇，从中选定171项特征名称，让大学生应用这些名称对同学进行行为评定，因素分析后最终得到16种人格特质。卡特尔认为这16种特质代表着人格组织的基本构成。"
    ],
    instruction: &[
        "本测验共有187道题目，都是有关个人的兴趣和态度等问题。每个人对这些问题是会有不同看法的，回答也是不同的，因而对问题如何回答，并没有对与不对之分，只是表明你对这些问题的态度。请你要尽量表达个人的意见，不要有顾虑。",
        "应当记住的是：",
        "1．每一测题只能选择一个答案。",
        "2．不可漏掉任何题目。",
        "3．尽量不选择B答案。",
        "4．本测验不计时间，但应凭自己的直觉反应进行作答，不要迟疑不决，拖延时间。一定要在一个小时以内完成整个测验。",
        "5．有些题目你可能从未思考过，或者感到不太容易回答。对于这样的题目，同样要求你做出一种倾向性的选择。"
    ],
    idea: Some(&[
        "卡特尔认为人格的基本结构元素是特质。特质的种类很多，有人类共同的特质，有各人独有的特质。有的特质决定于遗传，有的决定于环境；有的与动机有关，有的则与能力和气质有关。若从向度来分，可分为四种向度。",
        "（1）表面特质与根源特质",
        "表面特质是指一群看起来似乎聚在一起的特征或行为，即可以观察到的各种行为表现。它们之间是具有相关性的。根源特质是行为的最终根源和原因。它们是堆砌成人格的砖块。每一个根源特质控制着一簇表面特质。透过对许多表面特质的因素分析便可找到它们所属的根源特质。", 
        "（2）能力特质、气质特质与动力特质", 
        "能力特质与认知和思维有关，在16PF中主要由智慧因素(B因素)表示，决定工作的效率。行为的情绪、情感方面则表明了气质和风格的特质。动力特质与行为的意志和动机方面有关。", 
        "（3）个别特质和共同特质", 
        "卡特尔赞同阿尔波特的观点，认为人类存在着所有社会成员共同具有的特质(共同特质)和个体独有的特质，即个别特质(指表面特质)。虽有共同特质，但共同特质在各个成员身上的强度却各不相同(指根源特质)。", 
        "（4）体质特质和环境塑造特质", 
        "卡特尔认为16PF中有些特质是由遗传决定的，称为体质根源特质，而有些特质来源于经验，因此称为环境塑造特质。卡特尔认为在人格的成长和发展中遗传与环境都有影响。他十分重视遗传的重要性，曾试图决定每一根源特质的特殊遗传成分。", 
        "卡特尔在其人格的解释性理论构想的基础上编制了16种人格因素问卷，从16个方面描述个体的人格特征。这16个因素或分量表的名称和符号分别是：乐群性(A)、聪慧性(B)、稳定性(C)、恃强性(E)、兴奋性(F)、有恒性(G)、敢为性(H)、敏感性(I)、怀疑性(L)、幻想性(M)、世故性(N)、忧虑性(O)、实验性(Q1)、独立性(Q2)、自律性(Q3)、紧张性(Q4)。有关这16个因素的说明可详见测验指导书。",
    ]),
    references: None,
    formula_mode: None,
    warning: Some("本量表仅适用 16 岁以上的人群。"),
    tags: Tag{ info: Some(&[ "人格"]), normal: Some(&["自评"]), warning: Some(&["16+"]), error: None },
    interpretation: Interpretation { 
        normal_range: [3, 8],
        norm: NORM, 
        first_personality_factor: &[
            FirstPersonalityFactor {
                factor: Factor::A,
                name: "乐群性",
                characteristic: Characteristic {
                    low: &[
                        "缄默，孤独，冷漠。标准分低于3者通常固执，对人冷漠，落落寡合，喜欢吹毛求疵，宁愿独自工作，对事而不对人，不轻易放弃自己的主见，为人做事的标准常常很高。严谨而不苟且。",
                    ],
                    high: &[
                        "外向，热情，乐群。标准分高过8者，通常和蔼可亲，与人相处，合作与适应的能力特强。喜欢和别人共同工作，参加或组织各种社团活动，不斤斤计较，容易接受别人的批评。萍水相逢也可以一见如故。",
                    ],
                },
                occupations: "教师和推销员多系高A，而物理学家和电机工程师则多系低A。前者需要时时应付人与人之间的复杂情绪或行为问题，而仍然能够保证其乐观的态度；后者则必须极端地冷静严肃与正确，才能圆满地完成任务。",
            },
            FirstPersonalityFactor {
                factor: Factor::B,
                name: "聪慧性",
                characteristic: Characteristic {
                    low: &[
                        "思想迟钝，学识浅薄，抽象思考能力弱。低者通常学习与了解能力不强，不能“举一隅而以三隅反”。迟钝的原因可能由于情绪不稳定，心理病态或失常所致。",
                    ],
                    high: &[
                        "聪明，富有才识，善于抽象思考。高者通常学习能力强，思考敏捷正确，教育、文化水准高，个人心身状态健康。机警者多有高B，高B反映心理机能的正常。",
                    ],
                },
                occupations: "专业训练需要高B，但从事例行职务的人，如打字员、电话生、家庭主妇等，则因高B而对例行琐务发生厌恶，不能久安其职。",
            },
            FirstPersonalityFactor {
                factor: Factor::C,
                name: "稳定性",
                characteristic: Characteristic {
                    low: &[
                        "情绪激动，易生烦恼。低者通常不能以“逆来顺受”的态度应付生活上所遭遇的阻扰和挫折，容易受环境的支配而心神动摇不定。不能面对现实，时时会暴躁不安、心身疲乏，甚至于失眠、噩梦、恐怖等症状。所有神经病人和精神病人都属低C。",
                    ],
                    high: &[
                        "情绪稳定而成熟，能面对现实。高者通常以沉着的态度应付现实各项问题。行动充满魄力。能振奋勇气，维持团队的精神。有时高C也可能由于不能彻底解决许多生活难题，而不得不自我宽解。",
                    ],
                },
                occupations: "教师、机械工程师、推销员、救火队员等，凡需要应付日常生活各种难题者应有高C。但是凡能随心所欲安排自己工作进度的人，如作家、邮差或清道工等，则虽系低C，尚无大碍。",
            },
            FirstPersonalityFactor {
                factor: Factor::E,
                name: "恃强性",
                characteristic: Characteristic {
                    low: &[
                        "谦逊，顺从，通融，恭顺。低者通常行为温顺，迎合别人的意旨，也可能因为希望可遇而不可求，即使处在十全十美的境地，而有“事事不如人”之感，许多精神病人都有这样消极的心情。",
                    ],
                    high: &[
                        "好强固执，独立积极。高者通常自视甚高，自以为是。可能非常地武断，而时常驾驭不及他的人和反抗权势者。",
                    ],
                },
                occupations: "一般来说，领袖以及有地位有成就的人多属高E。救火队员和飞行员的因素E高。男人较女人高。",
            },
            FirstPersonalityFactor {
                factor: Factor::F,
                name: "兴奋性",
                characteristic: Characteristic {
                    low: &[
                        "严肃，谨慎，冷静，寡言。低者通常行动拘谨，内省而不轻易发言，较消极、忧郁。有时候可能过分深思熟虑，又近乎骄傲自满。在职责上，他常是认真而可靠的工作人员。",
                    ],
                    high: &[
                        "轻松兴奋，随遇而安。高者通常活泼、愉快、健谈，对人对事热心而富有感情。但是有时也可能会冲动，以致行为变化莫测。",
                    ],
                },
                occupations: "行政主管人员多有高F。竞选人必有高F，才能够获得选民的爱戴，实验技术人员则不必有高F。",
            },
            FirstPersonalityFactor {
                factor: Factor::G,
                name: "有恒性",
                characteristic: Characteristic {
                    low: &[
                        "苟且敷衍，缺乏奉公守法的精神。低者通常缺乏较高的目标和理想，对于人群及社会没有绝对的责任感，甚至于有时不惜执法犯法，不择手段已达到某一目的。但他常能有效地解决实际问题，而无须浪费时间和精力。",
                    ],
                    high: &[
                        "持恒负责，做事尽职。高者通常细心周到，有始有终。是非善恶是他的行为指针。所结交的朋友多是努力苦干的人，而不十分欣赏诙谐有趣的人。",
                    ],
                },
                occupations: "各种社团组织的领袖需要高G。业务管理和警察具有极高的因素G。任性纵欲，放火杀人的罪犯，因素G极低。",
            },
            FirstPersonalityFactor {
                factor: Factor::H,
                name: "敢为性",
                characteristic: Characteristic {
                    low: &[
                        "畏怯退缩，缺乏自信心。低者通常在人群中羞怯，有不自然的姿态，有强烈的自卑感。拙于发言，更不愿和陌生人交谈。凡是采取观望的态度，有时由于过分的自我意识而忽视了社会环境中的重要事物与活动。",
                    ],
                    high: &[
                        "冒险敢为，少有顾忌。高者通常不掩饰，不畏缩，有敢做敢为的精神，使他能经历艰辛而保持刚毅的一面。有时可能太粗心大意，忽视细节，遭受无谓的打击与挫折。可能无聊多事，喜欢向异性殷勤卖力。",
                    ],
                },
                occupations: "因素H常随年龄而增强。救火队员和飞行员有高H，事务员多是低H。团队领导人必具有高H。",
            },
            FirstPersonalityFactor {
                factor: Factor::I,
                name: "敏感性",
                characteristic: Characteristic {
                    low: &[
                        "理智的，着重现实，自恃其力。低者常以客观、坚强、独立的态度处理当前的问题。重视文化修养，可能过分冷酷无情。",
                    ],
                    high: &[
                        "敏感，感情用事。高者通常心肠软，易受感动，较女性化，爱好艺术，富于幻想。有时过分不切实际，缺乏耐心和恒心，不喜欢接近粗俗的人和做笨重的工作。在团体活动中，不着实际的看法与行为常常减低了团队的工作效率。",
                    ],
                },
                occupations: "室内设计师、音乐家、艺人、女人属高I，而工程师、外科医生、统计师等则多低I。",
            },
            FirstPersonalityFactor {
                factor: Factor::L,
                name: "怀疑性",
                characteristic: Characteristic {
                    low: &[
                        "信赖随和，易与人相处。通常无猜忌，不与人角逐竞争，顺应合作，善于体贴人。",
                    ],
                    high: &[
                        "怀疑，刚愎，固执己见。通常怀疑、不信任别人，与人相处常常斤斤计较，不顾及到别人的利益。",
                    ],
                },
                occupations: "在团体活动中，低L是以团体福利为前提的忠实分子，因素L过分高者常常成事不足、败事有余。工程师、机工、精神病护理员多是低L，而行政人员和警察常是高L。",
            },
            FirstPersonalityFactor {
                factor: Factor::M,
                name: "幻想性",
                characteristic: Characteristic {
                    low: &[
                        "现实，合乎成规，力求妥善合理。通常先要斟酌现实条件，而后决定取舍，不鲁莽从事。在紧要关头时，也能保持镇静，有时可能过分重视现实，为人索然寡趣。",
                    ],
                    high: &[
                        "幻想的，狂放不羁。高者通常忽视生活的细节，只以本身的动机、当时兴趣等主观因素为行为的出发点。可能富有创造力，有时也过分不务实际，近乎冲动，因而容易被人误解及奚落。",
                    ],
                },
                occupations: "艺术家、作家及从事研究者多有高M。低M多选择需要实际、机警、脚踏实地的工作。",
            },
            FirstPersonalityFactor {
                factor: Factor::N,
                name: "世故性",
                characteristic: Characteristic {
                    low: &[
                        "坦白，直率，天真。低者通常思想简单，感情用事。与人无争，与世无杵，自许，心满意足。但有时显得幼稚、粗鲁、笨拙，似乎缺乏教养。",
                    ],
                    high: &[
                        "精明能干，世故。高者通常处事老练，行为得体。能冷静分析一切，近乎狡猾。对于一切事物的看法是理智、客观的。",
                    ],
                },
                occupations: "科学家、工程师、飞行员多是高N，牧师神父、护士等多是低N。牧师的因素N不应太高，低N使他不苛求责难，能容忍同情信徒的缺点。",
            },
            FirstPersonalityFactor {
                factor: Factor::O,
                name: "忧虑性",
                characteristic: Characteristic {
                    low: &[
                        "乐群，沉着，有自信心。低者通常有信心，不轻易动摇，信任自己有应付问题的能力，有安全感，能适应世俗。有时因为缺乏同情，而引发别人的反感与恶意。",
                    ],
                    high: &[
                        "忧虑抑郁，烦恼自扰。高者通常觉得世道艰辛，人生不如意事常八九，甚至沮丧悲观，时时有患得患失之感。自觉不容于人，也缺乏和人接近的勇气。各种神经病和精神病人都有高O。",
                    ],
                },
                occupations: "年老的女招待，低级办事员，以及终生碌碌未尽如意的作家、编辑等有高O。职业运动员、电机工、救火队员、护士等多是低O。",
            },
            FirstPersonalityFactor {
                factor: Factor::Q1,
                name: "实验性",
                characteristic: Characteristic {
                    low: &[
                        "保守的，尊重传统观念与行为标准。低者通常无条件地接受社会中许多相沿已久而有权威性的见解，不愿尝试探求新的境界。常常激烈地反对新思想以及一切新的变动。在政治与宗教信仰上，墨守成规，可能被称为老顽固或时代的落伍者。",
                    ],
                    high: &[
                        "自由的，批评激进，不拘泥现实。高者通常喜欢考验一切现有的理论与实施，而予以新的评价，不轻易判断是非，企图了解较前卫的思想与行为。可能广见多闻。愿意充实自己的生活经验。",
                    ],
                },
                occupations: "行政主管人员、前卫的政治家、科学家都必须具有高Q1，护士、牧师神父与许多受高深教育的技工等则多有低Q1，神经病人的因素Q1也比较低。",
            },
            FirstPersonalityFactor {
                factor: Factor::Q2,
                name: "独立性",
                characteristic: Characteristic {
                    low: &[
                        "依赖，随群附和。低者通常宁欲与人共同工作，而不愿独立孤行。常常放弃个人的主见而附和取得别人的好感，需要团体的支持以维持其自信心，却并非真正的乐群者。",
                    ],
                    high: &[
                        "自立自强，当机立断。高者通常能够自作主张，独自完成自己的工作计划，不依赖别人，也不受社会舆论的约束，同样也无意控制或支配别人，不嫌恶人，但是也不需要别人的好感。",
                    ],
                },
                occupations: "科学家、行政主管人员等多有高Q2，低Q2者不能胜任需要有随机应变能力的职务。",
            },
            FirstPersonalityFactor {
                factor: Factor::Q3,
                name: "自律性",
                characteristic: Characteristic {
                    low: &[
                        "矛盾冲突，不顾大体。低者通常既不能克制自己，又不能尊重礼俗，更不愿考虑别人的需要，充满矛盾却无法解决。生活适应有问题者多低Q3。",
                    ],
                    high: &[
                        "知己知彼，自律严谨。高者通常言行一致，能够合理地支配自己的感情行动。为人处世，总能保持其自尊心，赢得别人的尊重，有时却不免太固执己见。",
                    ],
                },
                occupations: "高Q3者多具有领袖能力的才干，主管人员、电机工和生产部门的成功也需要高Q3。",
            },
            FirstPersonalityFactor {
                factor: Factor::Q4,
                name: "紧张性",
                characteristic: Characteristic {
                    low: &[
                        "心平气和，闲散宁静，低者通常知足常乐，保持内心的平衡。也可能过分疏懒，缺乏进取心。",
                    ],
                    high: &[
                        "紧张闲扰，激动挣扎。高者通常缺乏耐心，心神不安，态度兴奋。时常感觉疲乏，又无法彻底摆脱以求宁静。在社群中，他对人与事都缺乏信心。每日生活战战兢兢，不能自给。",
                    ],
                },
                occupations: "未能在生活或职业中发挥本身才智潜能的人多具有高Q4，如餐店招待、家庭主妇等。",
            },
        ],
        second_personality_factor: &[
            SecondPersonalityFactor {
                key: "X1",
                name: "适应与焦虑型",
                expression: "(38+2*L+3*O+4*Q4-2*C-2*H-2*Q2)/10",
                characteristic: Characteristic { 
                    low: &[
                        "生活适应顺利，通常感到心满意足，能做到所期望的及自认为重要的事情。也可能对困难的工作缺乏毅力，遇事有知难而退、不肯奋斗努力的倾向。",
                    ],
                    high: &[
                        "对自己生活和工作状态感到不满意。可能会使心情受到破坏和影响身体健康。",
                    ],
                },
            },
            SecondPersonalityFactor {
                key: "X2",
                name: "内向与外向型",
                expression: "(2*A+3*E+4*F+5*H-2*Q2-11)/10",
                characteristic: Characteristic { 
                    low: &[
                        "内倾，趋于胆小，自足，在与别人接触中常采取克制态度，有利于从事精细工作。",
                    ], 
                    high: &[
                        "外倾，开朗，善于交际，不受拘束，有利于从事贸易工作。",
                    ],
                },
            },
            SecondPersonalityFactor {
                key: "X3",
                name: "感情用事与安详机警型",
                expression: "(77+2*C+2*E+2*F+2*N-4*A-6*I-2*M)/10",
                characteristic: Characteristic {
                    low: &[
                        "情感丰富而感到困扰不安，可能是缺乏信心，颓丧的类型，对生活中的细节较为含蓄敏感，性格温和，讲究生活艺术，采取行动前再三思考，顾虑太多。",
                    ], 
                    high: &[
                        "富有事业心，果断，刚毅，有进取精神，精力充沛，行动迅速，但常忽视生活上的细节，只对明显的事物注意，有时会考虑不周，不计后果，贸然行事。"
                    ],
                },
            },
            SecondPersonalityFactor {
                key: "X4",
                name: "怯懦与果敢型",
                expression: "(4*E+3*M+4*Q1+4*Q2-3*A-2*G)/10",
                characteristic: Characteristic { 
                    low: &[
                        "怯懦，顺从，依赖别人，纯洁，个性被动，受人驱使而不能独立，为获取别人的欢心会事事迁就。",
                    ],
                    high: &[
                        "果断，独立，有气魄，有攻击性倾向，通常会主动寻找可以施展这种行为的环境或机会，以充分表现自己的独创能力，并从中取得利益。"
                    ],
                },
            },
        ],
    },
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
