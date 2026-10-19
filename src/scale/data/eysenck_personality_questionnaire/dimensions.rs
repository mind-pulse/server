use super::{DimensionInterpretation, HighLow, ScoreInterpret, ScoreInterpretItem};

/// 人格维度 - 内外倾向（E）
pub(super) const E_DIMENSION: DimensionInterpretation = DimensionInterpretation {
    label: "内外倾向",
    notice: None,
    core_explain: "这个维度看你“精力来源”：外向的人靠社交充电，内向的人靠独处充电，没有好坏之分",
    supplementary: Some(
        "人格是复杂的，即使外向的人也需要独处，内向的人也能做好社交，关键是适配自己的节奏",
    ),
    score_interpret: ScoreInterpret {
        medium: &[
            ScoreInterpretItem {
                title: "核心特点",
                content: "既喜欢跟人玩，也能独处，精力分配比较均衡",
            },
            ScoreInterpretItem {
                title: "实用建议",
                content: "学习/工作可以“社交+独处”结合（比如上午小组讨论，下午独自整理）",
            },
        ],
        moderate: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "喜欢社交，精力靠跟人相处充电，做事偏主动",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "用小组讨论、边讲边学的方式，避免长期独自死记硬背",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "多参加社交活动，但每天留10分钟独处，避免精力透支",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选团队协作、对外沟通的岗位（销售、运营），避开长期独处的工作",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "喜欢独处，精力靠安静环境充电，做事偏谨慎",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "选安静的环境学习，用精读、整理笔记的方式，避开频繁小组讨论",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "少参加人多的聚会，跟1-2个朋友相处就好，别勉强自己社交",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选独立工作、专注思考的岗位（科研、设计），避开频繁对外沟通的工作",
                },
            ],
        },
        extreme: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "几乎离不开社交，一独处就觉得无聊，做事特别主动",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "刻意留独处时间（比如睡前看书），避免过度依赖外界认可",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "别因为太想社交，忽略自己的真实感受",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "非常不喜欢社交，甚至回避多人场合，独处时最放松",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "必要社交前做心理建设（比如提前想好想说的话），别完全封闭自己",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "别因为怕社交，错过重要的机会（比如面试、合作）",
                },
            ],
        },
    },
};

/// 人格维度 - 神经质/情绪稳定性（N）
pub(super) const N_DIMENSION: DimensionInterpretation = DimensionInterpretation {
    label: "情绪性（神经质）",
    notice: Some("这是正常人格特质，和“精神病/心理疾病”无关！"),
    core_explain: "这个维度看你“情绪波动程度”：高分=情绪容易波动（敏感），低分=情绪稳定（冷静），无好坏",
    supplementary: Some("情绪波动大的人更能感知细节，情绪稳定的人更能抗压，只是适配不同场景"),
    score_interpret: ScoreInterpret {
        medium: &[
            ScoreInterpretItem {
                title: "核心特点",
                content: "情绪不算特别稳定，但也不会轻易崩溃，能应对日常压力",
            },
            ScoreInterpretItem {
                title: "实用建议",
                content: "压力大时做3次深呼吸，别立刻做决定",
            },
        ],
        moderate: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "情绪容易波动，一点小事可能开心/难过很久，对细节特别敏感",
                },
                ScoreInterpretItem {
                    title: "学习建议",
                    content: "情绪不好时先停学，做5分钟拉伸，别带着情绪做题",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "写日记梳理情绪，别憋在心里；少刷负面信息，避免情绪内耗",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选能发挥“敏感优势”的岗位（文案、设计、用户调研），避开高压应急的工作",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "情绪特别稳定，很少生气/难过，面对压力也能冷静",
                },
                ScoreInterpretItem {
                    title: "学习建议",
                    content: "可以偶尔逼自己“感性一点”，比如多思考题目背后的逻辑，别只看答案",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "多关注身边人的情绪，别因为自己冷静，忽略别人的感受",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选高压、需要决策的岗位（管理、应急救援），避开需要细腻感知的工作",
                },
            ],
        },
        extreme: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "情绪极易波动，容易焦虑、紧张，一点小事就想很多",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "每天花10分钟冥想，把担心的事写下来，逐个解决，别想太多",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "别因为情绪不好，否定自己的能力",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "情绪过于稳定，甚至有点“冷漠”，对开心/难过的事反应淡",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "刻意体验情绪（比如看一部感人的电影），别让自己变得太麻木",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "别因为自己冷静，忽略身边人的情感需求",
                },
            ],
        },
    },
};

/// 人格维度 - 精神质（P）
pub(super) const P_DIMENSION: DimensionInterpretation = DimensionInterpretation {
    label: "倔强性（精神质）",
    notice: Some("这是正常人格特质，和“精神病/心理疾病”无关！"),
    core_explain: "这个维度测量“倔强性/精神质”：高分者往往更独立、不拘泥规则、对他人感受关注较少、敢于冒险；低分者更温和、合作、有同理心。它反映正常人格差异，与精神疾病无关",
    supplementary: Some(
        "高分者的果断与独立性适合需要开拓的情境，低分者的亲和与配合适合需要协作的情境",
    ),
    score_interpret: ScoreInterpret {
        medium: &[
            ScoreInterpretItem {
                title: "核心特点",
                content: "既愿意配合别人，也有自己的想法，不会完全盲从",
            },
            ScoreInterpretItem {
                title: "实用建议",
                content: "团队中可以先听别人的意见，再说出自己的想法，兼顾协作和独立",
            },
        ],
        moderate: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "特别独立，不喜欢被规则束缚，做事有自己的节奏，不太在意别人的看法",
                },
                ScoreInterpretItem {
                    title: "学习建议",
                    content: "按自己的方法学习，别照搬别人的计划；遇到不同意见，先听再反驳",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "可以保留自己的个性，但别故意跟人对着干，避免冲突",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选需要创新、独立决策的岗位（创业、科研、设计），避开需要严格遵守规则的工作",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "特别温和，愿意配合别人，在意身边人的感受，不喜欢冲突",
                },
                ScoreInterpretItem {
                    title: "学习建议",
                    content: "别因为怕别人不高兴，不敢提自己的问题；可以主动跟同学交流想法",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "保留自己的底线，别一味迁就别人，避免委屈自己",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选需要团队协作、沟通的岗位（人力资源、客服、教师），避开需要独断决策的工作",
                },
            ],
        },
        extreme: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "极度独立，甚至有点“自我”，完全不按规则来，不在意别人的评价",
                },
                ScoreInterpretItem {
                    title: "学习建议",
                    content: "按自己的方法学习，别照搬别人的计划；遇到不同意见，先听再反驳",
                },
                ScoreInterpretItem {
                    title: "工作建议",
                    content: "选需要创新、独立决策的岗位（创业、科研、设计），避开需要严格遵守规则的工作",
                },
                ScoreInterpretItem {
                    title: "生活建议",
                    content: "可以保留自己的个性，但别故意跟人对着干，避免冲突",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "适当考虑别人的感受，别因为太自我，影响人际关系",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "创新也要兼顾实际，别脱离团队/规则空谈想法",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "极度温和，甚至有点“没主见”，总是迁就别人，害怕冲突",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "刻意练习说“不”，比如不想帮别人做事时，直接说明原因",
                },
                ScoreInterpretItem {
                    title: "避坑提醒",
                    content: "别因为迁就别人，放弃自己的目标和想法",
                },
            ],
        },
    },
};

/// 人格维度 - 效度/掩饰性（L）
pub(super) const L_DIMENSION: DimensionInterpretation = DimensionInterpretation {
    label: "掩饰性",
    notice: None,
    core_explain: "这个维度看你“回答是否真实”：高分=可能想展现自己的好形象，低分=回答更坦诚，仅参考测试效度",
    supplementary: None,
    score_interpret: ScoreInterpret {
        medium: &[
            ScoreInterpretItem {
                title: "核心特点",
                content: "回答比较真实，没有刻意隐瞒，也没有刻意表现自己",
            },
            ScoreInterpretItem {
                title: "实用建议",
                content: "测试结果有参考价值，可以结合自己的实际情况看",
            },
        ],
        moderate: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "可能想展现自己“更好”的一面，回答时有所隐瞒",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "可以重新做一次测试，按真实想法回答，结果会更准",
                },
                ScoreInterpretItem {
                    title: "提醒",
                    content: "这不是“不诚实”，只是想给人留好印象，很正常",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "回答特别坦诚，完全按自己的真实想法来，没有隐瞒",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "测试结果参考性高，可以重点看自己的人格倾向",
                },
            ],
        },
        extreme: HighLow {
            high: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "回答时过度掩饰，可能完全没按真实想法来",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "隔几天再做一次测试，放松心态，别刻意“装好人”，结果才有用",
                },
            ],
            low: &[
                ScoreInterpretItem {
                    title: "核心特点",
                    content: "回答极度坦诚，甚至有点“口无遮拦”",
                },
                ScoreInterpretItem {
                    title: "实用建议",
                    content: "测试结果很真实，但也要结合实际，别完全按测试下结论",
                },
            ],
        },
    },
};
