use super::{CapacityCategory, Question, QuestionOption, QuestionType};

pub(super) const QUESTIONS: &[Question] = &[
    Question {
        title: "（可多选或不选）选择您所感兴趣的活动：",
        question_type: QuestionType::InterestedEvent,
        capacity_category: CapacityCategory::R,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "装配修理电器或玩具",
                point: 1,
            },
            QuestionOption {
                text: "修理自行车",
                point: 1,
            },
            QuestionOption {
                text: "用木头做东西",
                point: 1,
            },
            QuestionOption {
                text: "开汽车或摩托车",
                point: 1,
            },
            QuestionOption {
                text: "用机器做东西",
                point: 1,
            },
            QuestionOption {
                text: "参加木工技术学习班",
                point: 1,
            },
            QuestionOption {
                text: "参加制图描图学习班",
                point: 1,
            },
            QuestionOption {
                text: "驾驶卡车或拖拉机",
                point: 1,
            },
            QuestionOption {
                text: "参加机械和电气学习班",
                point: 1,
            },
            QuestionOption {
                text: "装配修理机器",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所感兴趣的活动：",
        question_type: QuestionType::InterestedEvent,
        capacity_category: CapacityCategory::A,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "素描/制图或绘画",
                point: 1,
            },
            QuestionOption {
                text: "参加话剧/戏剧",
                point: 1,
            },
            QuestionOption {
                text: "设计家具/布置室内",
                point: 1,
            },
            QuestionOption {
                text: "练习乐器/参加乐队",
                point: 1,
            },
            QuestionOption {
                text: "欣赏音乐或戏剧",
                point: 1,
            },
            QuestionOption {
                text: "看小说/读剧本",
                point: 1,
            },
            QuestionOption {
                text: "从事摄影创作",
                point: 1,
            },
            QuestionOption {
                text: "写诗或吟诗",
                point: 1,
            },
            QuestionOption {
                text: "进艺术（美术/音乐）培训班",
                point: 1,
            },
            QuestionOption {
                text: "练习书法",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所感兴趣的活动：",
        question_type: QuestionType::InterestedEvent,
        capacity_category: CapacityCategory::I,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "读科技图书或杂志",
                point: 1,
            },
            QuestionOption {
                text: "在实验室工作",
                point: 1,
            },
            QuestionOption {
                text: "改良水果品种，培育新的水果",
                point: 1,
            },
            QuestionOption {
                text: "调查了解土和金属等物质的成份",
                point: 1,
            },
            QuestionOption {
                text: "研究自己选择的特殊问题",
                point: 1,
            },
            QuestionOption {
                text: "解算术或数学游戏",
                point: 1,
            },
            QuestionOption {
                text: "物理课",
                point: 1,
            },
            QuestionOption {
                text: "化学课",
                point: 1,
            },
            QuestionOption {
                text: "几何课",
                point: 1,
            },
            QuestionOption {
                text: "生物课",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所感兴趣的活动：",
        question_type: QuestionType::InterestedEvent,
        capacity_category: CapacityCategory::S,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "或单位组织的正式活动",
                point: 1,
            },
            QuestionOption {
                text: "参加某个社会团体或俱乐部活动",
                point: 1,
            },
            QuestionOption {
                text: "帮助别人解决困难",
                point: 1,
            },
            QuestionOption {
                text: "照顾儿童",
                point: 1,
            },
            QuestionOption {
                text: "出席晚会、联欢会、茶话会",
                point: 1,
            },
            QuestionOption {
                text: "和大家一起出去郊游",
                point: 1,
            },
            QuestionOption {
                text: "想获得关于心理方面的知识",
                point: 1,
            },
            QuestionOption {
                text: "参加讲座会或辩论会",
                point: 1,
            },
            QuestionOption {
                text: "观看或参加体育比赛和运动会",
                point: 1,
            },
            QuestionOption {
                text: "结交新朋友",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所感兴趣的活动：",
        question_type: QuestionType::InterestedEvent,
        capacity_category: CapacityCategory::E,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "鼓动他人",
                point: 1,
            },
            QuestionOption {
                text: "卖东西",
                point: 1,
            },
            QuestionOption {
                text: "谈论政治",
                point: 1,
            },
            QuestionOption {
                text: "制定计划、参加会议",
                point: 1,
            },
            QuestionOption {
                text: "以自己的意志影响别人的行为",
                point: 1,
            },
            QuestionOption {
                text: "在社会团体中担任职务",
                point: 1,
            },
            QuestionOption {
                text: "检查与评价别人的工作",
                point: 1,
            },
            QuestionOption {
                text: "结交名流",
                point: 1,
            },
            QuestionOption {
                text: "指导有某种目标的团体",
                point: 1,
            },
            QuestionOption {
                text: "参与政治活动",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所感兴趣的活动：",
        question_type: QuestionType::InterestedEvent,
        capacity_category: CapacityCategory::C,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "整理好桌面与房间",
                point: 1,
            },
            QuestionOption {
                text: "抄写文件和信件",
                point: 1,
            },
            QuestionOption {
                text: "为领导写报告或公务信函",
                point: 1,
            },
            QuestionOption {
                text: "检查个人收支情况",
                point: 1,
            },
            QuestionOption {
                text: "打字培训班",
                point: 1,
            },
            QuestionOption {
                text: "参加算盘、文秘等实务培训",
                point: 1,
            },
            QuestionOption {
                text: "参加商业会计培训班",
                point: 1,
            },
            QuestionOption {
                text: "参加情报处理培训班",
                point: 1,
            },
            QuestionOption {
                text: "整理信件、报告、记录等",
                point: 1,
            },
            QuestionOption {
                text: "写商业贸易信",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所擅长的事：",
        question_type: QuestionType::GoodAt,
        capacity_category: CapacityCategory::R,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "能使用电锯/电钻和锉刀等木工工具",
                point: 1,
            },
            QuestionOption {
                text: "知道万用电表的使用方法",
                point: 1,
            },
            QuestionOption {
                text: "能够修理自行车或其它机械",
                point: 1,
            },
            QuestionOption {
                text: "能够使用电钻订、磨床或缝纫机",
                point: 1,
            },
            QuestionOption {
                text: "能给家具和木制品刷漆",
                point: 1,
            },
            QuestionOption {
                text: "能看建筑设计图",
                point: 1,
            },
            QuestionOption {
                text: "能够修理简单的电气用品",
                point: 1,
            },
            QuestionOption {
                text: "能修理家具",
                point: 1,
            },
            QuestionOption {
                text: "能修理收录机",
                point: 1,
            },
            QuestionOption {
                text: "能简单地修理水管",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所擅长的事：",
        question_type: QuestionType::GoodAt,
        capacity_category: CapacityCategory::A,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "能演奏乐器",
                point: 1,
            },
            QuestionOption {
                text: "能参加二部或四部合唱",
                point: 1,
            },
            QuestionOption {
                text: "独唱或独奏",
                point: 1,
            },
            QuestionOption {
                text: "扮演剧中角色",
                point: 1,
            },
            QuestionOption {
                text: "能创作简单的乐曲",
                point: 1,
            },
            QuestionOption {
                text: "会跳舞",
                point: 1,
            },
            QuestionOption {
                text: "能绘画、素描或书法",
                point: 1,
            },
            QuestionOption {
                text: "能雕刻、剪纸或泥塑",
                point: 1,
            },
            QuestionOption {
                text: "能设计板报、服装或家具",
                point: 1,
            },
            QuestionOption {
                text: "能写一手好文章",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所擅长的事：",
        question_type: QuestionType::GoodAt,
        capacity_category: CapacityCategory::I,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "懂得真空管或晶体管的作用",
                point: 1,
            },
            QuestionOption {
                text: "能够列举三种蛋白质多的食品",
                point: 1,
            },
            QuestionOption {
                text: "理解铀的裂变",
                point: 1,
            },
            QuestionOption {
                text: "能用计算尺、计算器、对数表",
                point: 1,
            },
            QuestionOption {
                text: "会使用显微镜",
                point: 1,
            },
            QuestionOption {
                text: "能找到三个星座",
                point: 1,
            },
            QuestionOption {
                text: "能独立进行调查研究",
                point: 1,
            },
            QuestionOption {
                text: "能解释简单的化学",
                point: 1,
            },
            QuestionOption {
                text: "能理解人造卫星为什么不落地",
                point: 1,
            },
            QuestionOption {
                text: "经常参加学术的会议",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所擅长的事：",
        question_type: QuestionType::GoodAt,
        capacity_category: CapacityCategory::S,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "有向各种人说明解释的能力",
                point: 1,
            },
            QuestionOption {
                text: "常参加社会福利活动",
                point: 1,
            },
            QuestionOption {
                text: "能和大家一起友好相处地工作",
                point: 1,
            },
            QuestionOption {
                text: "善于与年长者相处",
                point: 1,
            },
            QuestionOption {
                text: "会邀请人、招待人",
                point: 1,
            },
            QuestionOption {
                text: "能简单易懂地教育儿童",
                point: 1,
            },
            QuestionOption {
                text: "能安排会议等活动顺序",
                point: 1,
            },
            QuestionOption {
                text: "善于体察人心和帮助他人",
                point: 1,
            },
            QuestionOption {
                text: "帮助护理病人和伤员",
                point: 1,
            },
            QuestionOption {
                text: "安排社团组织的各种事务",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所擅长的事：",
        question_type: QuestionType::GoodAt,
        capacity_category: CapacityCategory::E,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "担任过学生干部并且干得不错",
                point: 1,
            },
            QuestionOption {
                text: "工作上能指导和监督他人",
                point: 1,
            },
            QuestionOption {
                text: "做事充满活力和热情",
                point: 1,
            },
            QuestionOption {
                text: "有效利用自身的做法调动他人",
                point: 1,
            },
            QuestionOption {
                text: "销售能力强",
                point: 1,
            },
            QuestionOption {
                text: "曾作为俱乐部或社团的负责人",
                point: 1,
            },
            QuestionOption {
                text: "向领导提出建议或反映意见",
                point: 1,
            },
            QuestionOption {
                text: "有开创事业的能力",
                point: 1,
            },
            QuestionOption {
                text: "知道怎样成为一个优秀的领导者",
                point: 1,
            },
            QuestionOption {
                text: "健谈善辩",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您所擅长的事：",
        question_type: QuestionType::GoodAt,
        capacity_category: CapacityCategory::C,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "会熟练的打印中文",
                point: 1,
            },
            QuestionOption {
                text: "会用外文打字机或复印机",
                point: 1,
            },
            QuestionOption {
                text: "能快速记笔记和抄写文章",
                point: 1,
            },
            QuestionOption {
                text: "善于整理保管文件和资料",
                point: 1,
            },
            QuestionOption {
                text: "善于从事事务性的工作",
                point: 1,
            },
            QuestionOption {
                text: "会用算盘",
                point: 1,
            },
            QuestionOption {
                text: "能在短时间内分类和处理大量文件",
                point: 1,
            },
            QuestionOption {
                text: "能使用计算机",
                point: 1,
            },
            QuestionOption {
                text: "能搜集数据",
                point: 1,
            },
            QuestionOption {
                text: "善于为自己或集体做财务预算表",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您喜欢的职业：",
        question_type: QuestionType::LovedJob,
        capacity_category: CapacityCategory::R,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "飞机机械师",
                point: 1,
            },
            QuestionOption {
                text: "野生动物专家",
                point: 1,
            },
            QuestionOption {
                text: "汽车维修工",
                point: 1,
            },
            QuestionOption {
                text: "木匠",
                point: 1,
            },
            QuestionOption {
                text: "测量工程师",
                point: 1,
            },
            QuestionOption {
                text: "无线电报务员",
                point: 1,
            },
            QuestionOption {
                text: "园艺师",
                point: 1,
            },
            QuestionOption {
                text: "长途公共汽车司机",
                point: 1,
            },
            QuestionOption {
                text: "电工",
                point: 1,
            },
            QuestionOption {
                text: "火车司机",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您喜欢的职业：",
        question_type: QuestionType::LovedJob,
        capacity_category: CapacityCategory::A,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "乐队指挥",
                point: 1,
            },
            QuestionOption {
                text: "演奏家",
                point: 1,
            },
            QuestionOption {
                text: "作家",
                point: 1,
            },
            QuestionOption {
                text: "摄影家",
                point: 1,
            },
            QuestionOption {
                text: "记者",
                point: 1,
            },
            QuestionOption {
                text: "画家、书法家",
                point: 1,
            },
            QuestionOption {
                text: "歌唱家",
                point: 1,
            },
            QuestionOption {
                text: "作曲家",
                point: 1,
            },
            QuestionOption {
                text: "电影电视演员",
                point: 1,
            },
            QuestionOption {
                text: "电视节目主持人",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您喜欢的职业：",
        question_type: QuestionType::LovedJob,
        capacity_category: CapacityCategory::I,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "气象学或天文学者",
                point: 1,
            },
            QuestionOption {
                text: "生物学者",
                point: 1,
            },
            QuestionOption {
                text: "医学实验室的技术人员",
                point: 1,
            },
            QuestionOption {
                text: "人类学者",
                point: 1,
            },
            QuestionOption {
                text: "动物学者",
                point: 1,
            },
            QuestionOption {
                text: "化学者",
                point: 1,
            },
            QuestionOption {
                text: "教学者",
                point: 1,
            },
            QuestionOption {
                text: "科学杂志的编辑或作家",
                point: 1,
            },
            QuestionOption {
                text: "地质学者",
                point: 1,
            },
            QuestionOption {
                text: "物理学者",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您喜欢的职业：",
        question_type: QuestionType::LovedJob,
        capacity_category: CapacityCategory::S,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "街道、工会或妇联干部",
                point: 1,
            },
            QuestionOption {
                text: "小学、中学教师",
                point: 1,
            },
            QuestionOption {
                text: "精神病医生",
                point: 1,
            },
            QuestionOption {
                text: "婚姻介绍所工作人员",
                point: 1,
            },
            QuestionOption {
                text: "体育教练",
                point: 1,
            },
            QuestionOption {
                text: "福利机构负责人",
                point: 1,
            },
            QuestionOption {
                text: "心理咨询员",
                point: 1,
            },
            QuestionOption {
                text: "共青团干部",
                point: 1,
            },
            QuestionOption {
                text: "导游",
                point: 1,
            },
            QuestionOption {
                text: "国家机关工作人员",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您喜欢的职业：",
        question_type: QuestionType::LovedJob,
        capacity_category: CapacityCategory::E,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "厂长",
                point: 1,
            },
            QuestionOption {
                text: "电视片编制人",
                point: 1,
            },
            QuestionOption {
                text: "公司经理",
                point: 1,
            },
            QuestionOption {
                text: "销售员",
                point: 1,
            },
            QuestionOption {
                text: "不动产推销员",
                point: 1,
            },
            QuestionOption {
                text: "广告部长",
                point: 1,
            },
            QuestionOption {
                text: "体育活动主办者",
                point: 1,
            },
            QuestionOption {
                text: "销售部长",
                point: 1,
            },
            QuestionOption {
                text: "个体工商业者",
                point: 1,
            },
            QuestionOption {
                text: "企业管理咨询人员",
                point: 1,
            },
        ],
    },
    Question {
        title: "（可多选或不选）选择您喜欢的职业：",
        question_type: QuestionType::LovedJob,
        capacity_category: CapacityCategory::C,
        is_multiple: true,
        options: &[
            QuestionOption {
                text: "会计师",
                point: 1,
            },
            QuestionOption {
                text: "银行出纳员",
                point: 1,
            },
            QuestionOption {
                text: "税收管理员",
                point: 1,
            },
            QuestionOption {
                text: "计算机操作员",
                point: 1,
            },
            QuestionOption {
                text: "簿记人员",
                point: 1,
            },
            QuestionOption {
                text: "成本核算员",
                point: 1,
            },
            QuestionOption {
                text: "文书档案管理员",
                point: 1,
            },
            QuestionOption {
                text: "打字员",
                point: 1,
            },
            QuestionOption {
                text: "法庭书记员",
                point: 1,
            },
            QuestionOption {
                text: "人员普查登记员",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的机械操作能力在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::R,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的体育技能在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::R,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的科学研究能力在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::I,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的数学技能在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::I,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的艺术创作能力在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::A,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的音乐技能在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::A,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的解释表达能力在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::S,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的交际技能在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::S,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的商业洽谈能力在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::E,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的领导技能在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::E,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的事务执行能力在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::C,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
    Question {
        title: "满分 7 分的话，您觉得您的办公技能在什么水平：",
        question_type: QuestionType::CapacityCategory,
        capacity_category: CapacityCategory::C,
        is_multiple: false,
        options: &[
            QuestionOption {
                text: "7",
                point: 7,
            },
            QuestionOption {
                text: "6",
                point: 6,
            },
            QuestionOption {
                text: "5",
                point: 5,
            },
            QuestionOption {
                text: "4",
                point: 4,
            },
            QuestionOption {
                text: "3",
                point: 3,
            },
            QuestionOption {
                text: "2",
                point: 2,
            },
            QuestionOption {
                text: "1",
                point: 1,
            },
        ],
    },
];
