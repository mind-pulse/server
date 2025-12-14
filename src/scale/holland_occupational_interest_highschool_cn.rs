//! 此为开源版霍兰德职业兴趣高中生专用中文版本，专业来自2024年教育部公开专业目录文件，因专业数量多，这里只能硬编码一部分。
//!
//! 商业版使用高级算法实现，会对所有专业进行智能推荐组合。

use serde::Serialize;

use crate::scale::{PlainTexts, ScaleCategory};

use super::holland_occupational_interest::{
    CapacityCategory, CapacityCategoryInterpretation, Question, QuestionType,
};
use super::{HTMLElement, PlainText, QuestionOption, Scale, SentenceItem, Tag, Texts};

const CAPACITY_CATEGORY_INTERPRETATIONS: [CapacityCategoryInterpretation; 6] = [
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::R,
        name: "现实型",
        personality_trait: "这个类型的人喜欢使用工具、机器操作，偏爱具体任务而非抽象思想。务实、踏实，体力好，更喜欢与物打交道。喜欢以物质、机械、动物、工作等为对象，从事有规律、清晰、有序、系统的工作​。喜欢动手实践解决问题。",
        occupational_stigma: "适合这类人的专业有工学门类专业（机械类、电气类、电子信息类等）​、农学门类专业（植物生产类、动物生产类、动物医学类等）、医学门类专业（临床医学类、口腔医学类、中医学类等）。",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::I,
        name: "研究型",
        personality_trait: "也叫探索型。这个类型的人擅长思考和观察，喜欢理论分析和独立研究，求知欲强、逻辑清晰、独立自主、不轻信结论，善于提出疑问，通过逻辑和证据来验证想法​，喜欢研究物，抽象思维，不喜欢研究人​，更喜欢与概念、数据、理论打交道​。",
        occupational_stigma: "适合这类人的专业有理学门类专业（数学类、物理学类、化学类、生物学类等）、科研方向的医学门类专业（基础医学类、临床医学类、药学类、法医学类等）、哲学门类专业、经济学门类专业（经济学类、财政学类、金融学类）。",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::A,
        name: "艺术型",
        personality_trait: "这个类型的人充满创造力与想象力，喜欢通过自我表达、创作来展现个人想法和价值观​
，厌恶重复、僵化的工作，追求美感与独特性​，对色彩、声音、文字形式等有敏锐的感受力​，注重作品的审美价值和原创性，情感丰富与直觉性强​，倾向于依赖情感、直觉和想象来处理问题​，渴望自由与独立。",
        occupational_stigma: "适合这类人的专业有艺术学门类专业​（音乐与舞蹈学类、戏剧与影视学类、美术学类等）、文学门类专业​（中国语言文学类、外国语言文学类、新闻传播学类）、工学门类专业​（建筑类、设计学类）",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::S,
        name: "社会型",
        personality_trait: "这个类型的人天生喜欢帮助、教导、服务他人，并能从中获得巨大的满足感和价值感​，善于倾听、理解、沟通和协调，能够敏锐地察觉他人的情绪和需求​，心肠软、有爱心，愿意为他人或社会福祉付出努力，更喜欢团队合作、人际互动，而不是独自面对机器、数据和工具。",
        occupational_stigma: "适合这类人的专业有教育学门类专业​（教育学类、体育学类）、医学门类专业​（临床医学类、口腔医学类、中医学类等）、法学门类专业​（法学类、社会学类、马克思主义理论类等）、管理学门类专业​（工商管理类、公共管理类、社会学类）。",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::E,
        name: "企业型",
        personality_trait: "也叫事业型或管理型。这个类型的人喜欢竞争，追求成就，渴望获得领导地位和声望​，精力充沛，善于言辞，乐于推销自己的观点、产品甚至自己​，不畏惧风险，愿意为可能的高回报做出决策并承担后果​，更喜欢通过控制、说服他人来达成组织或个人的目标​。",
        occupational_stigma: "适合这类人的专业有经济学门类专业​（经济学类、财政学类、金融学类等）、管理学门类专业​（管理科学与工程类、工商管理类、公共管理类等）、法学门类专业​（法学类、政治学类、社会学类）。",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::C,
        name: "传统型",
        personality_trait: "也叫常规型。这个类型的人喜欢结构清晰、流程明确、秩序井然的环境和工作任务​，讨厌混乱和不确定性，能够高效、准确地进行数据处理、文书归档等需要耐心和细心的工作​，并以零错误为荣，享受可预测、稳定的工作内容​，不喜欢频繁的变化和冒险​，更喜欢独立处理文件、数字、系统，而不是进行大量的社交、说服或创意活动。",
        occupational_stigma: "适合这类人的专业有管理学门类专业​（管理科学与工程类、工商管理类、公共管理类等）、经济学门类专业​（经济学类、财政学类、金融学类等）、理学门类专业​（数学类、统计学类、心理学类）、文学门类专业​（中国语言文学类、外国语言文学类、新闻传播学类）。",
    },
];

#[derive(Serialize)]
struct MajorsMatch {
    /// 代码标识，如 "RIA"
    code: PlainText,

    /// 代码含义说明，如 "R+I+A：实操 + 研究 + 艺术"
    meaning: PlainText,

    /// 核心专业列表
    core_majors: PlainTexts,

    /// 次优专业列表
    secondary_majors: PlainTexts,

    /// 适配逻辑描述
    logic: PlainText,
}

const MAJORS_MATCHES: [MajorsMatch; 72] = [
    // R：实操型主导
    MajorsMatch {
        code: "RIA",
        meaning: "实操 + 研究 + 艺术",
        core_majors: &["冶金工程", "纺织工程", "金属材料工程", "船舶与海洋工程", "海洋技术", "林学"],
        secondary_majors: &["工业设计", "建筑设计", "珠宝首饰设计与工艺"],
        logic: "需动手操作（如材料加工、船舶建造）、技术研究（如材料性能优化）、美学设计（如船体外观、纺织图案）。",
    },
    MajorsMatch {
        code: "RIS",
        meaning: "实操 + 研究 + 社会服务",
        core_majors: &[
            "冶金工程",
            "金属材料工程",
            "热能与动力工程",
            "电气工程及其自动化",
            "计算机科学与技术",
            "交通运输",
            "环境工程",
            "油气储运工程",
            "海洋技术",
            "医学检验",
            "针灸推拿学",
            "法医学",
            "林学",
            "动物科学",
            "港口航道与海岸工程",
            "化学工程与工艺",
            "电子信息科学与技术",
        ],
        secondary_majors: &["康复治疗学", "动物医学"],
        logic: "既需实操（如电力运维、医学检验操作），又需研究（如环境治理技术、动物疫病分析），还需服务他人（如康复治疗、兽医诊疗）。",
    },
    MajorsMatch {
        code: "RIE",
        meaning: "实操 + 研究 + 企业管理",
        core_majors: &[
            "冶金工程",
            "金属材料工程",
            "电气工程及其自动化",
            "测控技术与仪器",
            "油气储运工程",
            "船舶与海洋工程",
            "轻化工程",
            "海洋技术",
            "采矿工程",
            "动物科学",
        ],
        secondary_majors: &["工程管理", "工程造价"],
        logic: "侧重工程实操（如采矿、船舶装配）、技术研究（如测控技术优化、油气储运安全）、企业生产管理（如工程进度把控）。",
    },
    MajorsMatch {
        code: "RIC",
        meaning: "实操 + 研究 + 常规事务",
        core_majors: &["信息管理与信息系统", "冶金工程", "金属材料工程", "过程装备与控制工程","热能与动力工程","电气工程及其自动化","通信工程","计算机科学与技术","交通运输","教育技术学","植物保护学","动物医学","机械设计制造及其自动化","测控技术与仪器","土木工程","环境工程","油气储运工程","船舶与海洋工程","轻化工程","化学类","地理信息系统","海洋技术","医学检验","法医学","港口航道与海岸工程"],
        secondary_majors: &["物流工程","工业工程"],
        logic: "需实操（如机械装配、设备测控）、研究（如通信技术、植物病虫害分析）、标准化管理（如生产流程记录、检验数据归档）。",
    },
    MajorsMatch {
        code: "RAI",
        meaning: "实操 + 艺术 + 研究",
        core_majors: &["园艺学", "音乐学", "美术学", "艺术设计", "纺织工程"],
        secondary_majors: &["风景园林", "产品设计"],
        logic: "侧重艺术实操（如园艺造型、纺织图案设计）、美学研究（如艺术风格创新）、技术支撑（如园艺植物培育、材料工艺优化）。",
    },
    MajorsMatch {
        code: "RSE",
        meaning: "实操 + 社会服务 + 企业管理",
        core_majors: &[
            "油气储运工程",
            "动物科学",
            "体育学类（如体育教育、运动训练）",
            "广播电视编导",
        ],
        secondary_majors: &["物业管理", "旅游管理"],
        logic: "需实操（如油气运输操作、体育训练指导）、社会服务（如动物防疫、游客接待）、企业运营（如场馆管理、节目制作统筹）。",
    },
    MajorsMatch {
        code: "RSI",
        meaning: "实操 + 社会服务 + 研究",
        core_majors: &[
            "环境工程",
            "油气储运工程",
            "针灸推拿学",
            "动物科学",
            "港口航道与海岸工程",
            "城市规划",
        ],
        secondary_majors: &["公共卫生与预防医学", "农业资源与环境"],
        logic: "需实操（如环境监测、针灸治疗）、服务社会（如公共卫生防护、农业技术推广）、技术研究（如污染治理方案、作物品种改良）。",
    },
    MajorsMatch {
        code: "REC",
        meaning: "实操 + 企业管理 + 常规事务",
        core_majors: &[
            "油气储运工程",
            "物业管理",
            "物流管理",
            "工程管理",
            "交通管理",
        ],
        secondary_majors: &["财务管理（工程方向）", "市场营销（技术产品方向）"],
        logic: "需实操（如物流仓储操作、工程现场管理）、企业运营（如物业调度、物流统筹）、事务规范（如费用核算、数据记录）。",
    },
    MajorsMatch {
        code: "REI",
        meaning: "实操 + 企业管理 + 研究",
        core_majors: &[
            "油气储运工程",
            "动物科学",
            "农林牧渔业经营管理",
            "航运管理",
        ],
        secondary_majors: &["农业工程", "海洋工程"],
        logic: "需实操（如农林生产操作、船舶驾驶）、企业管理（如农场运营、航运调度）、技术研究（如作物高产技术、船舶安全优化）。",
    },
    MajorsMatch {
        code: "RES",
        meaning: "实操 + 企业管理 + 社会服务",
        core_majors: &["油气储运工程","动物科学","物流管理","旅游管理","海事管理","农业机械化及其自动化","体育教育"],
        secondary_majors: &["酒店管理","会展经济与管理"],
        logic: "需实操（如农业机械操作、体育器材使用）、企业运营（如酒店调度、物流配送）、社会服务（如游客引导、动物疫病防治）。",
    },
    MajorsMatch {
        code: "RCI",
        meaning: "实操 + 常规事务 + 研究",
        core_majors: &["临床医学","建筑学","麻醉学","医学影像学","农学","教育技术学","勘查技术与工程","建筑环境与设备工程","动物医学","土木工程","环境工程","油气储运工程","航海技术","地理信息系统","港口航道与海岸工程"],
        secondary_majors: &["测绘工程","遥感科学与技术"],
        logic: "需实操（如临床诊疗,工程勘查）,事务规范（如病历记录,测绘数据归档）,技术研究（如疾病机制,工程技术优化）。",
    },
    MajorsMatch {
        code: "RCS",
        meaning: "实操 + 常规事务 + 社会服务",
        core_majors: &["临床医学","建筑学","勘查技术与工程","建筑环境与设备工程","环境工程","油气储运工程","航海技术","港口航道与海岸工程","给排水科学与工程"],
        secondary_majors: &["护理学","社会工作（社区技术服务方向）"],
        logic: "需实操（如建筑施工,给排水维护）,事务规范（如施工记录,服务档案）,社会服务（如社区医疗,居民生活保障）。",
    },
    MajorsMatch {
        code: "RCE",
        meaning: "实操 + 常规事务 + 企业管理",
        core_majors: &["勘查技术与工程","油气储运工程","航海技术","工程管理","测绘工程","土木工程","机械工程"],
        secondary_majors: &["工程造价","房地产开发与管理"],
        logic: "需实操（如测绘操作,机械维修）,事务规范（如工程核算,数据统计）,企业管理（如项目成本控制,施工进度统筹）。",
    },

    // I：研究型主导
    MajorsMatch {
        code: "IAS",
        meaning: "研究 + 艺术 + 社会服务",
        core_majors: &["汉语言文学","工业设计","建筑学","城市规划","环境科学"],
        secondary_majors: &["心理学（艺术治疗方向）","传播学（文化研究方向）"],
        logic: "侧重理论研究（如文学分析、城市规划原理）、艺术创意（如建筑设计、文化传播策划）、社会服务（如心理疏导、环境教育）。",
    },
    MajorsMatch {
        code: "IAR",
        meaning: "研究 + 艺术 + 实操",
        core_majors: &["纺织工程","园林工程","建筑学","考古学","地质工程"],
        secondary_majors: &["文物保护技术","数字媒体技术"],
        logic: "需科研（如地质研究、文物修复理论）、艺术创作（如园林设计、文物复原）、实操（如地质勘探、纺织工艺）。",
    },
    MajorsMatch {
        code: "ISE",
        meaning: "研究 + 社会服务 + 企业管理",
        core_majors: &["资源环境与城乡规划管理","经济学","国际经济与贸易","社会学","地理学","环境科学"],
        secondary_majors: &["公共事业管理","健康服务与管理"],
        logic: "需科研（如环境评估、经济分析）、社会服务（如公共政策制定、健康管理）、企业运营（如项目统筹、贸易策划）。",
    },
    MajorsMatch {
        code: "ISC",
        meaning: "研究 + 社会服务 + 常规事务",
        core_majors: &["汉语言文学","药学","教育学","哲学","国际经济与贸易","社会学","政治学与行政学","地理科学","环境科学"],
        secondary_majors: &["图书馆学","档案学"],
        logic: "需科研（如教育理论、药物研发）、社会服务（如教学、健康咨询）、事务规范（如文献整理、数据归档）。",
    },
    MajorsMatch {
        code: "ISR",
        meaning: "研究 + 社会服务 + 实操",
        core_majors: &["生物学","药学","经济学","交通工程","地质学","中医学","应用物理学"],
        secondary_majors: &["医学检验技术","康复治疗学"],
        logic: "需科研（如生物机制、地质结构）、社会服务（如医疗、交通规划）、实操（如检验操作、地质勘探）。",
    },
    MajorsMatch {
        code: "ISA",
        meaning: "研究 + 社会服务 + 艺术",
        core_majors: &["汉语言文学","环境科学","护理学","医疗器械工程","工业心理学","管理心理学"],
        secondary_majors: &["音乐治疗学","艺术教育"],
        logic: "需科研（如心理机制、环境影响）、社会服务（如护理、心理咨询）、艺术表达（如音乐治疗、艺术教学）。",
    },
    MajorsMatch {
        code: "IES",
        meaning: "研究 + 企业管理 + 社会服务",
        core_majors: &["水利水电工程","经济学","资源环境与城乡规划管理"],
        secondary_majors: &["环境工程（企业环保方向）","公共管理（项目运营方向）"],
        logic: "需科研（如水利技术、环境治理）、企业管理（如项目运营、资源调配）、社会服务（如公共水利、健康保障）。",
    },
    MajorsMatch {
        code: "IEC",
        meaning: "研究 + 企业管理 + 常规事务",
        core_majors: &["水利水电工程","计算机科学与技术","软件工程","财政学","财务管理","应用数学"],
        secondary_majors: &["金融工程","数据科学与大数据技术"],
        logic: "需科研（如算法研发、财政政策）、企业管理（如金融风控、项目统筹）、事务规范（如财务核算、数据统计）。",
    },
    MajorsMatch {
        code: "ICR",
        meaning: "研究 + 常规事务 + 实操",
        core_majors: &["药学","制药工程","交通工程","地质学","地球物理学","大气科学","中医学","基础医学","自动化","水文与水资源工程","统计学"],
        secondary_majors: &["测控技术与仪器（科研检测方向）","生物医学工程"],
        logic: "需科研（如药物研发、地质分析）、事务规范（如数据记录、质量控制）、实操（如实验操作、设备调试）。",
    },
    MajorsMatch {
        code: "IRA",
        meaning: "研究 + 实操 + 艺术",
        core_majors: &["地理学","地质学","声学物理","矿物学","古生物学","石油地质","地震学","原子物理","电气磁学","气象学","城市规划"],
        secondary_majors: &["建筑设计（科研方向）","数字地质建模"],
        logic: "需科研（如地质结构、气象规律）、实操（如地质勘探、气象观测）、艺术设计（如城市布局、地质模型可视化）。",
    },
    MajorsMatch {
        code: "IRS",
        meaning: "研究 + 实操 + 社会服务",
        core_majors: &["流体物理","物理海洋学","等离子体物理","农业科学","动物学","食品科学","园艺学","植物学","细菌学","解剖学","动物病理学","作物病理学","药物学","生物化学","生物物理学","细胞生物学","临床化学","遗传学","分子生物学","质量控制工程","地理学","兽医学","放射治疗技术"],
        secondary_majors: &["食品安全与检测","植物保护"],
        logic: "需科研（如生物机制、食品安全）、实操（如实验操作、作物防治）、社会服务（如兽医诊疗、食品安全保障）。",
    },
    MajorsMatch {
        code: "IRE",
        meaning: "研究 + 实操 + 企业管理",
        core_majors: &["化学工程与工艺","纺织工程","食品科学与工程","渔业资源","材料科学与工程","电气工程及其自动化","土木工程","航空航天工程","冶金工程","核工程与核技术","陶瓷工程","地质工程","电力工程","口腔医学"],
        secondary_majors: &["工业工程（技术管理方向）","航空维修工程"],
        logic: "需科研（如材料研发、航空技术）、实操（如化工生产、设备维修）、企业管理（如生产统筹、项目运营）。",
    },
    MajorsMatch {
        code: "IRC",
        meaning: "研究 + 实操 + 常规事务",
        core_majors: &["航空航天工程","飞行技术","物理学（实验方向）","农学","动植物科学","生物工程","石油工程","工商管理（工业方向）","纺织工程","计算机科学与技术（硬件方向）","工具设计","仪器科学与技术"],
        secondary_majors: &["质量工程","标准化工程"],
        logic: "需科研（如飞行技术、生物工程）、实操（如飞机驾驶、设备维修）、事务规范（如质量检验、数据记录）。",
    },

    // A：艺术型主导
    MajorsMatch {
        code: "ASE",
        meaning: "艺术 + 社会服务 + 企业管理",
        core_majors: &["戏剧影视导演","舞蹈学","广告学","汉语言文学（创意写作方向）","新闻学","英语（翻译 / 播音方向）","广播电视编导"],
        secondary_majors: &["文化产业管理","公共艺术"],
        logic: "需艺术创作（如导演、广告设计）、社会服务（如文化传播、语言服务）、企业管理（如项目统筹、节目运营）。",
    },
    MajorsMatch {
        code: "ASI",
        meaning: "艺术 + 社会服务 + 研究",
        core_majors: &["音乐学","音乐表演","美术学","艺术教育","舞蹈学","戏剧学","汉语言文学（文学创作方向）","广告学","服装设计与工程"],
        secondary_majors: &["艺术史论（应用方向）","心理学（艺术治疗方向）"],
        logic: "需艺术表达（如音乐演奏、绘画）、社会服务（如艺术教学、心理疏导）、理论研究（如艺术风格、教育方法）。",
    },
    MajorsMatch {
        code: "AER",
        meaning: "艺术 + 企业管理 + 实操",
        core_majors: &["摄影","广播电视编导（摄像方向）","艺术设计（舞台方向）","录音艺术","戏剧影视美术设计"],
        secondary_majors: &["影视摄影与制作","灯光设计"],
        logic: "需艺术创作（如摄影、舞台设计）、企业管理（如项目运营、团队协调）、实操（如摄像、灯光调试）。",
    },
    MajorsMatch {
        code: "AEI",
        meaning: "艺术 + 企业管理 + 研究",
        core_majors: &["音乐指挥","戏剧影视导演","电影学（创作方向）","艺术管理"],
        secondary_majors: &["影视制片管理","文化创意产业管理"],
        logic: "需艺术创作（如指挥、导演）、企业管理（如剧组统筹、项目运营）、理论研究（如艺术市场、观众心理）。",
    },
    MajorsMatch {
        code: "AES",
        meaning: "艺术 + 企业管理 + 社会服务",
        core_majors: &["音乐表演（流行方向）","舞蹈表演","戏剧影视导演","广播电视编导（主持方向）","表演","艺术教育"],
        secondary_majors: &["公共文化服务","旅游演艺策划"],
        logic: "需艺术表达（如演唱、表演）、企业管理（如演出统筹、节目运营）、社会服务（如文化传播、艺术普及）。",
    },
    MajorsMatch {
        code: "AIS",
        meaning: "艺术 + 研究 + 社会服务",
        core_majors: &["绘画","戏剧影视文学","编辑出版学","艺术评论","服装设计与工程","视觉传达设计","摄影（艺术方向）","汉语言文学（文学创作方向）"],
        secondary_majors: &["艺术治疗","文化遗产保护"],
        logic: "需艺术创作（如绘画、写作）、理论研究（如艺术评论、文化分析）、社会服务（如编辑出版、文化传播）。",
    },
    MajorsMatch {
        code: "AIE",
        meaning: "艺术 + 研究 + 企业管理",
        core_majors: &["园艺（景观方向）","服装设计与工程","工业设计","产品设计","雕刻艺术与家具设计"],
        secondary_majors: &["艺术衍生品设计","文创产品开发"],
        logic: "需艺术创作（如景观设计、产品造型）、理论研究（如材料工艺、市场需求）、企业管理（如产品生产、品牌运营）。",
    },
    MajorsMatch {
        code: "AIR",
        meaning: "艺术 + 研究 + 实操",
        core_majors: &["建筑学","绘画","摄影","地理信息科学（可视化方向）","环境设计","雕塑","产品设计","陶瓷艺术设计","服装与服饰设计","动画"],
        secondary_majors: &["数字媒体艺术","文物修复"],
        logic: "需艺术创作（如建筑设计、绘画）、理论研究（如空间美学、材料工艺）、实操（如模型制作、文物修复）。",
    },

    // S：社会型主导
    MajorsMatch {
        code: "SEC",
        meaning: "服务 + 企业管理 + 常规事务",
        core_majors: &["社会工作","公共事业管理","工商管理（人力资源方向）","旅游管理","酒店管理","会展经济与管理"],
        secondary_majors: &["行政管理","档案学（政务服务方向）"],
        logic: "需社会服务（如福利帮扶、游客接待）、企业管理（如酒店运营、会展统筹）、事务规范（如档案整理、数据统计）。",
    },
    MajorsMatch {
        code: "SER",
        meaning: "服务 + 企业管理 + 实操",
        core_majors: &["体育教育","运动训练","社会体育指导与管理","休闲体育"],
        secondary_majors: &["体育康复","场馆运营管理"],
        logic: "需社会服务（如体育教学、康复指导）、企业管理（如场馆运营、训练统筹）、实操（如运动指导、设备维护）。",
    },
    MajorsMatch {
        code: "SEI",
        meaning: "服务 + 企业管理 + 研究",
        core_majors: &["教育学（教育管理方向）","公共事业管理（医院 / 高校方向）","历史学（文化遗产方向）","家政学","职业技术教育"],
        secondary_majors: &["教育经济与管理","健康管理"],
        logic: "需社会服务（如教育、医疗管理）、企业管理（如高校运营、医院统筹）、理论研究（如教育政策、健康科学）。",
    },
    MajorsMatch {
        code: "SEA",
        meaning: "服务 + 企业管理 + 艺术",
        core_majors: &["文化产业管理","旅游管理（文化旅游方向）","公共艺术","社会工作（艺术治疗方向）"],
        secondary_majors: &["会展艺术与技术","休闲服务与管理"],
        logic: "需社会服务（如文化传播、艺术治疗）、企业管理（如会展运营、旅游统筹）、艺术创作（如公共艺术、活动策划）。",
    },
    MajorsMatch {
        code: "SCE",
        meaning: "服务 + 常规事务 + 企业管理",
        core_majors: &["行政管理","公共事业管理","秘书学","工商管理（行政方向）","旅游管理（政务接待方向）"],
        secondary_majors: &["档案学（政务方向）","人力资源管理（行政事务方向）"],
        logic: "需社会服务（如政务办理、福利帮扶）、事务规范（如文件处理、档案管理）、企业管理（如行政统筹、人员调度）。",
    },
    MajorsMatch {
        code: "SRI",
        meaning: "服务 + 实操 + 研究",
        core_majors: &["护理学","临床医学（全科方向）","康复治疗学","医学检验技术","动物医学（宠物医疗方向）"],
        secondary_majors: &["公共卫生与预防医学","针灸推拿学"],
        logic: "需社会服务（如医疗护理、宠物诊疗）、实操（如护理操作、检验技术）、理论研究（如疾病机制、康复方案）。",
    },
    MajorsMatch {
        code: "SRE",
        meaning: "服务 + 实操 + 企业管理",
        core_majors: &["体育教育","学前教育","初等教育","社会工作（社区服务方向）","护理学（社区护理方向）"],
        secondary_majors: &["家政学","社区管理与服务"],
        logic: "需社会服务（如教学、社区护理）、实操（如体育训练、幼儿照料）、企业管理（如班级管理、社区运营）。",
    },
    MajorsMatch {
        code: "SRC",
        meaning: "服务 + 实操 + 常规事务",
        core_majors: &["护理学（基础护理方向）","社会工作（社区实务方向）","学前教育（保育方向）","公共卫生管理"],
        secondary_majors: &["老年服务与管理","家政服务与管理"],
        logic: "需社会服务（如老人护理、社区服务）、实操（如护理操作、理发技术）、事务规范（如服务记录、档案管理）。",
    },
    MajorsMatch {
        code: "SIA",
        meaning: "服务 + 研究 + 艺术",
        core_majors: &["心理学（临床 / 教育方向）","教育学（艺术教育方向）","社会学（文化研究方向）","汉语言文学（教育方向）","历史学（文博方向）"],
        secondary_majors: &["艺术治疗学","文化遗产保护"],
        logic: "需社会服务（如心理咨询、教育）、理论研究（如心理机制、文化分析）、艺术表达（如艺术治疗、文博展示）。",
    },
    MajorsMatch {
        code: "SIE",
        meaning: "服务 + 研究 + 企业管理",
        core_majors: &["营养学","食品卫生与营养学","行政管理（卫生方向）","公共事业管理（食品安全方向）"],
        secondary_majors: &["健康服务与管理","市场监管"],
        logic: "需社会服务（如营养指导、公共卫生）、理论研究（如食品营养、健康科学）、企业管理（如健康机构运营、监管统筹）。",
    },
    MajorsMatch {
        code: "SIC",
        meaning: "服务 + 研究 + 常规事务",
        core_majors: &["教育学（教育技术方向）","心理学（应用方向）","图书馆学","档案学","社会工作（研究实务方向）"],
        secondary_majors: &["数据科学（社会服务方向）","信息资源管理"],
        logic: "需社会服务（如教育、咨询）、理论研究（如教育技术、心理科学）、事务规范（如文献整理、数据归档）。",
    },
    MajorsMatch {
        code: "SIR",
        meaning: "服务 + 研究 + 实操",
        core_majors: &["康复治疗学","护理学（康复方向）","针灸推拿学","医学技术（康复方向）","动物医学（临床方向）"],
        secondary_majors: &["物理治疗","作业治疗"],
        logic: "需社会服务（如康复治疗、兽医诊疗）、理论研究（如康复机制、疾病治疗）、实操（如理疗操作、动物手术）。",
    },

    // E：企业型主导
    MajorsMatch {
        code: "ECI",
        meaning: "管理 + 常规事务 + 研究",
        core_majors: &["金融学","会计学","工商管理（财务管理方向）","经济学（金融方向）","国际经济与贸易"],
        secondary_majors: &["金融工程","保险学"],
        logic: "需企业管理（如银行运营、财务统筹）、事务规范（如会计核算、审计）、理论研究（如金融市场、经济分析）。",
    },
    MajorsMatch {
        code: "ECS",
        meaning: "管理 + 常规事务 + 服务",
        core_majors: &["会计学","财务管理","工商管理（市场营销方向）","国际经济与贸易","社会工作（企业 CSR 方向）"],
        secondary_majors: &["人力资源管理（薪酬方向）","市场营销（客户服务方向）"],
        logic: "需企业管理（如销售统筹、客户管理）、事务规范（如财务核算、档案）、社会服务（如客户服务、CSR）。",
    },
    MajorsMatch {
        code: "ERI",
        meaning: "管理 + 实操 + 研究",
        core_majors: &["工程管理","工商管理（工业方向）","农业经济管理","护理学（护理管理方向）","环境工程（管理方向）"],
        secondary_majors: &["物业管理（工程方向）","能源经济"],
        logic: "需企业管理（如工程统筹、农场运营）、实操（如工程现场、农业生产）、理论研究（如工程技术、农业经济）。",
    },
    MajorsMatch {
        code: "ERS",
        meaning: "管理 + 实操 + 服务",
        core_majors: &["物流管理","物业管理","旅游管理（景区运营方向）","工商管理（运营方向）"],
        secondary_majors: &["会展经济与管理","酒店管理"],
        logic: "需企业管理（如物流统筹、物业运营）、实操（如仓储操作、设施维护）、社会服务（如游客接待、业主服务）。",
    },
    MajorsMatch {
        code: "ERC",
        meaning: "管理 + 实操 + 规范",
        core_majors: &["工程管理","工程造价","工商管理（生产方向）","物流管理（供应链方向）","交通运输（管理方向）"],
        secondary_majors: &["房地产开发与管理","工业工程"],
        logic: "需企业管理（如工程运营、生产统筹）、实操（如施工、物流操作）、事务规范（如成本核算、流程记录）。",
    },
    MajorsMatch {
        code: "EIR",
        meaning: "管理 + 研究 + 实操",
        core_majors: &["科技成果转化管理","工商管理（科技方向）","信息管理与信息系统（IT 项目方向）","生物工程（产业方向）"],
        secondary_majors: &["知识产权管理","技术经济及管理"],
        logic: "需企业管理（如科技项目运营、IT 统筹）、理论研究（如技术研发、成果转化）、实操（如项目落地、技术调试）。",
    },
    MajorsMatch {
        code: "EIC",
        meaning: "管理 + 研究 + 规范",
        core_majors: &["金融科技","大数据管理与应用","会计学（大数据方向）","经济学（量化方向）"],
        secondary_majors: &["保险精算","商业分析"],
        logic: "需企业管理（如金融科技运营、数据分析统筹）、理论研究（如量化模型、数据科学）、事务规范（如精算核算、数据归档）。",
    },
    MajorsMatch {
        code: "EIS",
        meaning: "管理 + 研究 + 服务",
        core_majors: &["公共事业管理（科技服务方向）","健康服务与管理","教育经济与管理","社会工作（企业服务方向）"],
        secondary_majors: &["医疗健康管理","科技咨询"],
        logic: "需企业管理（如健康机构运营、教育统筹）、理论研究（如健康科学、教育政策）、社会服务（如健康咨询、教育服务）。",
    },
    MajorsMatch {
        code: "EAS",
        meaning: "管理 + 艺术 + 服务",
        core_majors: &["文化产业管理","艺术管理","旅游管理（文化艺术方向）","市场营销（艺术产品方向）"],
        secondary_majors: &["会展艺术与技术","公共文化管理"],
        logic: "需企业管理（如文化项目运营、艺术市场统筹）、艺术创意（如活动策划、产品设计）、社会服务（如文化传播、法律咨询）。",
    },
    MajorsMatch {
        code: "EAR",
        meaning: "管理 + 艺术 + 实操",
        core_majors: &["会展经济与管理（艺术方向）","旅游管理（景观运营方向）","服装设计与工程（品牌方向）","工业设计（品牌管理方向）"],
        secondary_majors: &["家具设计与制造（品牌方向）","珠宝首饰设计与工艺（运营方向）"],
        logic: "需企业管理（如会展运营、品牌统筹）、艺术创意（如舞台设计、产品造型）、实操（如展览搭建、工艺制作）。",
    },
    MajorsMatch {
        code: "ESC",
        meaning: "管理 + 服务 + 规范",
        core_majors: &["人力资源管理","行政管理","公共事业管理","工商管理（行政方向）","社会工作（企业服务方向）"],
        secondary_majors: &["劳动关系","档案学（企业方向）"],
        logic: "需企业管理（如人员调度、行政统筹）、社会服务（如员工关怀、政务服务）、事务规范（如档案管理、流程记录）。",
    },
    MajorsMatch {
        code: "ESR",
        meaning: "管理 + 服务 + 实操",
        core_majors: &["体育经济与管理","旅游管理（户外方向）","物业管理（社区服务方向）","工商管理（运营方向）"],
        secondary_majors: &["休闲体育管理","社区管理与服务"],
        logic: "需企业管理（如体育赛事运营、社区统筹）、社会服务（如游客接待、居民服务）、实操（如赛事执行、设施维护）。",
    },
    MajorsMatch {
        code: "ESI",
        meaning: "管理 + 服务 + 研究",
        core_majors: &["教育经济与管理","健康服务与管理","公共事业管理（研究方向）","社会工作（政策研究方向）"],
        secondary_majors: &["图书馆学（管理方向）","科技咨询"],
        logic: "需企业管理（如高校运营、健康机构统筹）、社会服务（如教育、健康咨询）、理论研究（如教育政策、健康科学）。",
    },
    MajorsMatch {
        code: "ESA",
        meaning: "管理 + 服务 + 艺术",
        core_majors: &["文化产业管理（公共文化方向）","旅游管理（文旅方向）","艺术教育（管理方向）","社会工作（艺术治疗方向）"],
        secondary_majors: &["公共艺术管理","会展艺术与技术"],
        logic: "需企业管理（如文旅项目运营、艺术机构统筹）、社会服务（如文化传播、艺术治疗）、艺术创意（如展览设计、活动策划）。",
    },

    // C：常规型主导
        MajorsMatch {
        code: "CRI",
        meaning: "事务 + 实操 + 研究",
        core_majors: &["会计学","财务管理","信息管理与信息系统","工商管理（财务方向）","统计学（应用方向）"],
        secondary_majors: &["审计学","数据科学与大数据技术（财务方向）"],
        logic: "需事务规范（如会计核算、数据记录）、实操（如设备操作、财务软件使用）、理论研究（如统计分析、财务模型）。",
    },
    MajorsMatch {
        code: "CRS",
        meaning: "事务 + 实操 + 服务",
        core_majors: &["行政管理","档案学","社会工作（事务方向）","公共事业管理（行政方向）"],
        secondary_majors: &["人力资源管理（事务方向）","秘书学"],
        logic: "需事务规范（如档案管理、文件处理）、实操（如设备使用、接待）、社会服务（如客户沟通、居民服务）。",
    },
    MajorsMatch {
        code: "CRE",
        meaning: "事务 + 实操 + 管理",
        core_majors: &["工程管理（造价方向）","会计学（工程方向）","物流管理（仓储方向）","工商管理（运营方向）"],
        secondary_majors: &["工程造价","仓储管理"],
        logic: "需事务规范（如工程核算、仓储记录）、实操（如设备操作、装配）、企业管理（如生产统筹、物流调度）。",
    },
    MajorsMatch {
        code: "CIS",
        meaning: "事务 + 研究 + 服务",
        core_majors: &["会计学","财务管理","统计学（社会方向）","信息管理与信息系统","社会工作（数据方向）"],
        secondary_majors: &["数据科学（社会服务方向）","金融统计"],
        logic: "需事务规范（如会计核算、数据记录）、理论研究（如统计分析、信息科学）、社会服务（如客户咨询、数据服务）。",
    },
    MajorsMatch {
        code: "CIE",
        meaning: "事务 + 研究 + 管理",
        core_majors: &["金融学（统计方向）","会计学（大数据方向）","工商管理（数据分析方向）","经济学（统计方向）"],
        secondary_majors: &["商业分析","保险精算"],
        logic: "需事务规范（如财务核算、数据记录）、理论研究（如量化分析、统计模型）、企业管理（如财务统筹、数据分析管理）。",
    },
    MajorsMatch {
        code: "CIR",
        meaning: "事务 + 研究 + 实操",
        core_majors: &["信息管理与信息系统（IT 运维方向）","统计学（实验方向）","会计学（电算化方向）","测控技术与仪器（检测方向）"],
        secondary_majors: &["数据科学（实验方向）","质量工程"],
        logic: "需事务规范（如数据记录、检修档案）、理论研究（如统计分析、检测技术）、实操（如设备检测、IT 运维）。",
    },
    MajorsMatch {
        code: "CSE",
        meaning: "事务 + 服务 + 管理",
        core_majors: &["行政管理","人力资源管理（事务方向）","旅游管理（行政方向）","工商管理（行政方向）"],
        secondary_majors: &["秘书学","会展经济与管理（行政方向）"],
        logic: "需事务规范（如文件处理、票务记录）、社会服务（如客户接待、游客服务）、企业管理（如行政统筹、活动调度）。",
    },
    MajorsMatch {
        code: "CSR",
        meaning: "事务 + 服务 + 实操",
        core_majors: &["行政管理（社区方向）","社会工作（事务方向）","护理学（行政方向）","旅游管理（服务方向）"],
        secondary_majors: &["社区管理与服务","家政学（行政方向）"],
        logic: "需事务规范（如货运记录、交通档案）、社会服务（如社区服务、游客接待）、实操（如货运协调、交通检查）。",
    },
    MajorsMatch {
        code: "CSA",
        meaning: "事务 + 服务 + 艺术",
        core_majors: &["行政管理（文化方向）","档案学（文博方向）","社会工作（艺术服务方向）","文化产业管理（事务方向）"],
        secondary_majors: &["公共文化服务","会展艺术与技术（事务方向）"],
        logic: "需事务规范（如文博档案、文件处理）、社会服务（如文化服务、读者服务）、艺术创意（如展览布置、文化活动）。",
    },
    MajorsMatch {
        code: "CER",
        meaning: "事务 + 管理 + 实操",
        core_majors: &["会计学（企业方向）","财务管理（运营方向）","工商管理（生产方向）","物流管理（供应链方向）"],
        secondary_majors: &["市场营销（渠道方向）","生产管理"],
        logic: "需事务规范（如财务核算、数据记录）、企业管理（如生产统筹、供应链调度）、实操（如物流操作、生产执行）。",
    },
    MajorsMatch {
        code: "CEI",
        meaning: "事务 + 管理 + 研究",
        core_majors: &["金融学（公司金融方向）","会计学（财务分析方向）","工商管理（战略方向）","经济学（产业方向）"],
        secondary_majors: &["投资学","产业经济学"],
        logic: "需事务规范（如财务核算、投资记录）、企业管理（如公司运营、投资统筹）、理论研究（如产业分析、投资模型）。",
    },
    MajorsMatch {
        code: "CES",
        meaning: "事务 + 管理 + 服务",
        core_majors: &["会计学（审计方向）","财务管理（税务方向）","工商管理（合规方向）","社会工作（企业 CSR 方向）"],
        secondary_majors: &["税务","劳动关系"],
        logic: "需事务规范（如审计记录、税务申报）、企业管理（如合规管理、CSR 统筹）、社会服务（如税务咨询、员工关怀）。",
    },
];

#[derive(Serialize)]
pub struct Interpretation {
    capacity_category_interpretations: &'static [CapacityCategoryInterpretation],
    majors_matches: &'static [MajorsMatch],
}

const INTRODUCTION: Texts = &[
    &[
        SentenceItem::Plain("可用于"),
        SentenceItem::HTMLElement(HTMLElement::Strong("就业方向")),
        SentenceItem::Plain("的指导和"),
        SentenceItem::HTMLElement(HTMLElement::Strong("高考填报志愿")),
        SentenceItem::Plain("时的专业选择。"),
    ],
    &[SentenceItem::Plain("由美国职业指导专家霍兰德（John Holland）根据他本人大量的职业咨询经验及其职业类型理论编制的测评工具。霍兰德认为，个人职业兴趣特性与职业之间应有一种内在的对应关系。")],
];

const INSTRUCTION: Texts = &[
    &[
        SentenceItem::Plain("本测量表将帮助你发现和确定自己的"),
        SentenceItem::HTMLElement(HTMLElement::Strong("职业兴趣")),
        SentenceItem::Plain("和"),
        SentenceItem::HTMLElement(HTMLElement::Strong("能力特长")),
        SentenceItem::Plain("，从而更好地选择适合自己的专业。"),
    ],
    &[
        SentenceItem::Plain("在测评的时候，要求在一个"),
        SentenceItem::HTMLElement(HTMLElement::Strong(
            "不受干扰的、安静的、独立的空间和平静的心态",
        )),
        SentenceItem::Plain("下进行。"),
    ],
];

pub const HOLLAND_OCCUPATIONAL_INTEREST_HIGH_SCHOOL_CN: Scale<Interpretation, Question> = Scale {
    name: "霍兰德职业兴趣测评（高中版）",
    primary_category: ScaleCategory::CareerAndAcademics,
    related_categories: Some(&[
        ScaleCategory::Personality,
        ScaleCategory::AttitudeAndValues,
    ]),
    abbreviation: "SDS", // 与抑郁自评量表缩写相同
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: Some(&[
        "兴趣是最好的专业向导。美国心理学家霍兰德提出，每个人的兴趣特点都可以归纳为六个类型：研究型（I）、艺术型（A）、社会型（S）、企业型（E）、传统型（C）、现实型（R）。你的性格可能是这六种类型的混合，但通常会有两三个特别突出的类型，而这恰恰对应着不同类型的大学专业。",
        "这个理论的核心就是：帮你找到“喜欢的”和“适合的”专业之间的交集。过去，大家选专业可能只看分数或就业前景，而霍兰德则强调兴趣才是最关键的方向标。他把“个人特点”和“专业环境”联系起来，让我们明白：选专业不只是选一个学科，更是选择一种适合自己特质的学习方式和未来发展路径。",
        "所以，在填报志愿前，不妨做个职业兴趣测试，再结合自己的学科能力，这样能更清晰地找到既感兴趣又学得好的专业方向，让大学生活更有目标感和动力。",
    ]),
    references: None,
    warning: None,
    formula_mode: None,
    tags: Tag{ info: Some(&["高考", "专业"]), normal: None, warning: None, error: None },
    interpretation: Interpretation { capacity_category_interpretations: &CAPACITY_CATEGORY_INTERPRETATIONS, majors_matches: &MAJORS_MATCHES },
    questions: &[
        Question {
            title: "（可多选或不选）选择您感兴趣的校园科技活动：",
            question_type: QuestionType::InterestedEvent,
            capacity_category: CapacityCategory::R,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "装配或调试家用智能小电器（如智能台灯、蓝牙音箱）",
                    point: 1,
                },
                QuestionOption {
                    text: "动手修理自行车或组装滑板",
                    point: 1,
                },
                QuestionOption {
                    text: "用木板/3D打印笔制作小模型（如笔筒、摆件）",
                    point: 1,
                },
                QuestionOption {
                    text: "学习驾驶家用模拟赛车或无人机",
                    point: 1,
                },
                QuestionOption {
                    text: "操作简易机床或3D打印机制作小零件",
                    point: 1,
                },
                QuestionOption {
                    text: "参加校园木工/金工技术兴趣班",
                    point: 1,
                },
                QuestionOption {
                    text: "学习CAD制图或手绘机械草图",
                    point: 1,
                },
                QuestionOption {
                    text: "尝试组装电脑主机或更换硬件",
                    point: 1,
                },
                QuestionOption {
                    text: "参加机器人/单片机编程与组装学习班",
                    point: 1,
                },
                QuestionOption {
                    text: "拆解并复原旧家电（如老式收音机）研究原理",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您擅长的科技实践能力：",
            question_type: QuestionType::GoodAt,
            capacity_category: CapacityCategory::R,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "能熟练使用手电钻、美工刀等手工工具制作小物件",
                    point: 1,
                },
                QuestionOption {
                    text: "会用万用表检测电路故障或判断电器通断",
                    point: 1,
                },
                QuestionOption {
                    text: "能独立修理自行车爆胎、链条脱落等小故障",
                    point: 1,
                },
                QuestionOption {
                    text: "会使用缝纫机缝补衣物或3D打印机制作模型",
                    point: 1,
                },
                QuestionOption {
                    text: "能给小家具/木艺品刷漆并做到均匀美观",
                    point: 1,
                },
                QuestionOption {
                    text: "能看懂简易的房屋户型图或机械零件图纸",
                    point: 1,
                },
                QuestionOption {
                    text: "能修理台灯、充电器等简单的小电器",
                    point: 1,
                },
                QuestionOption {
                    text: "能修补松动的桌椅或损坏的木质文具",
                    point: 1,
                },
                QuestionOption {
                    text: "能通过教程修复耳机接触不良、音箱无声音等问题",
                    point: 1,
                },
                QuestionOption {
                    text: "能疏通堵塞的洗手池或更换水龙头垫片",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您向往的工科/技术类专业方向：",
            question_type: QuestionType::LovedJob,
            capacity_category: CapacityCategory::R,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "智能制造工程",
                    point: 1,
                },
                QuestionOption {
                    text: "野生动物保护与利用",
                    point: 1,
                },
                QuestionOption {
                    text: "汽车服务工程",
                    point: 1,
                },
                QuestionOption {
                    text: "木材科学与工程",
                    point: 1,
                },
                QuestionOption {
                    text: "测绘工程",
                    point: 1,
                },
                QuestionOption {
                    text: "通信工程",
                    point: 1,
                },
                QuestionOption {
                    text: "园艺专业",
                    point: 1,
                },
                QuestionOption {
                    text: "交通运输",
                    point: 1,
                },
                QuestionOption {
                    text: "电气工程及其自动化",
                    point: 1,
                },
                QuestionOption {
                    text: "轨道交通信号与控制",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您感兴趣的校园艺术活动：",
            question_type: QuestionType::InterestedEvent,
            capacity_category: CapacityCategory::A,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "用数位板进行插画/漫画创作或手绘板报",
                    point: 1,
                },
                QuestionOption {
                    text: "参演校园话剧、校园歌手大赛或班级文艺汇演",
                    point: 1,
                },
                QuestionOption {
                    text: "设计班级活动海报或规划宿舍软装布局",
                    point: 1,
                },
                QuestionOption {
                    text: "练习吉他/钢琴等乐器或加入校园乐队",
                    point: 1,
                },
                QuestionOption {
                    text: "去剧院看话剧/音乐会或线上欣赏优质演出",
                    point: 1,
                },
                QuestionOption {
                    text: "阅读青年文学小说/剧本或写短篇故事",
                    point: 1,
                },
                QuestionOption {
                    text: "用手机/相机进行校园纪实摄影创作",
                    point: 1,
                },
                QuestionOption {
                    text: "尝试写校园主题的小诗或歌词",
                    point: 1,
                },
                QuestionOption {
                    text: "参加学校美术/音乐兴趣社团或校外艺术培训班",
                    point: 1,
                },
                QuestionOption {
                    text: "练习硬笔书法或国风字体设计",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您擅长的艺术创作能力：",
            question_type: QuestionType::GoodAt,
            capacity_category: CapacityCategory::A,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "能熟练演奏至少一种乐器并完成完整曲目",
                    point: 1,
                },
                QuestionOption {
                    text: "能参与班级合唱并找准声部、跟上节奏",
                    point: 1,
                },
                QuestionOption {
                    text: "能独立完成歌曲独唱或乐器独奏表演",
                    point: 1,
                },
                QuestionOption {
                    text: "能在校园话剧中扮演角色并自然表达情绪",
                    point: 1,
                },
                QuestionOption {
                    text: "能为班级活动写简单的主题口号或短旋律",
                    point: 1,
                },
                QuestionOption {
                    text: "会跳流行舞/民族舞并能编排简单的小片段",
                    point: 1,
                },
                QuestionOption {
                    text: "能画出结构完整的插画或写出工整的书法作品",
                    point: 1,
                },
                QuestionOption {
                    text: "能做简单的黏土手工、剪纸或橡皮章雕刻",
                    point: 1,
                },
                QuestionOption {
                    text: "能设计班级活动海报或搭配出协调的穿搭",
                    point: 1,
                },
                QuestionOption {
                    text: "能写出语句通顺、有感染力的作文或活动发言稿",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您向往的艺术/文创类专业方向：",
            question_type: QuestionType::LovedJob,
            capacity_category: CapacityCategory::A,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "音乐指挥与表演",
                    point: 1,
                },
                QuestionOption {
                    text: "乐器演奏",
                    point: 1,
                },
                QuestionOption {
                    text: "汉语言文学（创意写作方向）",
                    point: 1,
                },
                QuestionOption {
                    text: "摄影专业",
                    point: 1,
                },
                QuestionOption {
                    text: "新闻学",
                    point: 1,
                },
                QuestionOption {
                    text: "美术学/书法学",
                    point: 1,
                },
                QuestionOption {
                    text: "音乐表演",
                    point: 1,
                },
                QuestionOption {
                    text: "作曲与作曲技术理论",
                    point: 1,
                },
                QuestionOption {
                    text: "表演专业",
                    point: 1,
                },
                QuestionOption {
                    text: "播音与主持艺术",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您感兴趣的学术研究活动：",
            question_type: QuestionType::InterestedEvent,
            capacity_category: CapacityCategory::I,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "阅读《环球科学》《新发现》等科普杂志或学科拓展读物",
                    point: 1,
                },
                QuestionOption {
                    text: "在学校实验室完成生物/化学小实验",
                    point: 1,
                },
                QuestionOption {
                    text: "研究家庭盆栽的品种改良或无土栽培方法",
                    point: 1,
                },
                QuestionOption {
                    text: "调查奶茶饮品的糖分含量或家用清洁剂的成分",
                    point: 1,
                },
                QuestionOption {
                    text: "针对校园热点问题（如垃圾分类）做小课题研究",
                    point: 1,
                },
                QuestionOption {
                    text: "玩数独、逻辑推理桌游或解数学竞赛拓展题",
                    point: 1,
                },
                QuestionOption {
                    text: "主动探究物理课本外的拓展知识点（如量子力学入门）",
                    point: 1,
                },
                QuestionOption {
                    text: "尝试在家做趣味化学小实验（如自制叶脉书签）",
                    point: 1,
                },
                QuestionOption {
                    text: "用几何知识设计校园简易建筑模型",
                    point: 1,
                },
                QuestionOption {
                    text: "观察并记录校园植物的生长周期与习性",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您擅长的学术研究能力：",
            question_type: QuestionType::GoodAt,
            capacity_category: CapacityCategory::I,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "能分清常见电子元件（电阻/电容）的作用并简单应用",
                    point: 1,
                },
                QuestionOption {
                    text: "能列举出5种以上高蛋白食物并说明适用场景",
                    point: 1,
                },
                QuestionOption {
                    text: "能理解核裂变的基本原理和应用领域",
                    point: 1,
                },
                QuestionOption {
                    text: "会用计算器、函数表解决复杂的数学计算问题",
                    point: 1,
                },
                QuestionOption {
                    text: "能规范使用显微镜观察细胞或微生物样本",
                    point: 1,
                },
                QuestionOption {
                    text: "能在夜空中识别北斗七星、猎户座等常见星座",
                    point: 1,
                },
                QuestionOption {
                    text: "能自主确定研究主题并完成简单的调研方案",
                    point: 1,
                },
                QuestionOption {
                    text: "能解释汽水冒泡、铁生锈等日常化学现象的原理",
                    point: 1,
                },
                QuestionOption {
                    text: "能理解人造卫星绕地飞行的基本物理逻辑",
                    point: 1,
                },
                QuestionOption {
                    text: "曾参加校内/区级学科竞赛或科技小论文评比",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您向往的理科/科研类专业方向：",
            question_type: QuestionType::LovedJob,
            capacity_category: CapacityCategory::I,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "大气科学/天文学",
                    point: 1,
                },
                QuestionOption {
                    text: "生物科学",
                    point: 1,
                },
                QuestionOption {
                    text: "医学检验技术",
                    point: 1,
                },
                QuestionOption {
                    text: "人类学",
                    point: 1,
                },
                QuestionOption {
                    text: "动物科学",
                    point: 1,
                },
                QuestionOption {
                    text: "化学",
                    point: 1,
                },
                QuestionOption {
                    text: "数学与应用数学",
                    point: 1,
                },
                QuestionOption {
                    text: "编辑出版学（科技方向）",
                    point: 1,
                },
                QuestionOption {
                    text: "地质学",
                    point: 1,
                },
                QuestionOption {
                    text: "物理学",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您感兴趣的校园社会活动：",
            question_type: QuestionType::InterestedEvent,
            capacity_category: CapacityCategory::S,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "参与学校或班级组织的志愿活动（如敬老院慰问）",
                    point: 1,
                },
                QuestionOption {
                    text: "加入校园社团（如志愿者协会、辩论社）并积极参与活动",
                    point: 1,
                },
                QuestionOption {
                    text: "主动帮助同学解答学习难题或调解小矛盾",
                    point: 1,
                },
                QuestionOption {
                    text: "协助老师照顾班级里的低年级学弟学妹",
                    point: 1,
                },
                QuestionOption {
                    text: "参加班级联欢会、校庆晚会等集体活动的组织筹备",
                    point: 1,
                },
                QuestionOption {
                    text: "和同学一起参加研学旅行或户外团建",
                    point: 1,
                },
                QuestionOption {
                    text: "阅读青少年心理相关书籍，想了解同伴相处技巧",
                    point: 1,
                },
                QuestionOption {
                    text: "参与校园辩论赛或学科知识讲座的组织工作",
                    point: 1,
                },
                QuestionOption {
                    text: "报名校运会志愿者或班级体育赛事的后勤保障",
                    point: 1,
                },
                QuestionOption {
                    text: "主动认识不同班级/年级的同学并建立友谊",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您擅长的社会实践能力：",
            question_type: QuestionType::GoodAt,
            capacity_category: CapacityCategory::S,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "能把复杂的知识点用通俗的语言讲给同学听",
                    point: 1,
                },
                QuestionOption {
                    text: "经常主动报名学校或社区的公益志愿活动",
                    point: 1,
                },
                QuestionOption {
                    text: "能和不同性格的同学合作完成小组任务",
                    point: 1,
                },
                QuestionOption {
                    text: "能和老师/学长学姐顺畅沟通并请教问题",
                    point: 1,
                },
                QuestionOption {
                    text: "会策划小型聚会并妥善招待来访的同学/亲友",
                    point: 1,
                },
                QuestionOption {
                    text: "能给低年级同学辅导基础学科知识并让其理解",
                    point: 1,
                },
                QuestionOption {
                    text: "能安排班级小型活动的流程并协调好时间人员",
                    point: 1,
                },
                QuestionOption {
                    text: "能敏锐察觉同学的情绪变化并给予安慰",
                    point: 1,
                },
                QuestionOption {
                    text: "会简单的伤口包扎或照顾感冒的同学",
                    point: 1,
                },
                QuestionOption {
                    text: "能统筹社团小型活动的物资、场地和人员",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您向往的教育/服务类专业方向：",
            question_type: QuestionType::LovedJob,
            capacity_category: CapacityCategory::S,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "公共事业管理",
                    point: 1,
                },
                QuestionOption {
                    text: "教育学（师范类）",
                    point: 1,
                },
                QuestionOption {
                    text: "应用心理学（临床方向）",
                    point: 1,
                },
                QuestionOption {
                    text: "社会工作",
                    point: 1,
                },
                QuestionOption {
                    text: "体育教育",
                    point: 1,
                },
                QuestionOption {
                    text: "公共事业管理（福利方向）",
                    point: 1,
                },
                QuestionOption {
                    text: "心理咨询",
                    point: 1,
                },
                QuestionOption {
                    text: "思想政治教育",
                    point: 1,
                },
                QuestionOption {
                    text: "旅游管理",
                    point: 1,
                },
                QuestionOption {
                    text: "行政管理",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您感兴趣的校园领导活动：",
            question_type: QuestionType::InterestedEvent,
            capacity_category: CapacityCategory::E,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "组织同学参与公益义卖并动员更多人参与",
                    point: 1,
                },
                QuestionOption {
                    text: "在校园跳蚤市场摆摊售卖闲置物品",
                    point: 1,
                },
                QuestionOption {
                    text: "和同学讨论校园热点议题或模拟班级政策提案",
                    point: 1,
                },
                QuestionOption {
                    text: "牵头策划班级活动并组织同学开会分配任务",
                    point: 1,
                },
                QuestionOption {
                    text: "用自己的想法说服同学支持班级活动方案",
                    point: 1,
                },
                QuestionOption {
                    text: "在学生会/社团中担任干事或部长等职务",
                    point: 1,
                },
                QuestionOption {
                    text: "协助老师检查班级卫生/纪律并给出改进建议",
                    point: 1,
                },
                QuestionOption {
                    text: "主动结识学校优秀学长学姐或社团负责人",
                    point: 1,
                },
                QuestionOption {
                    text: "带领小组完成社会实践课题并汇报成果",
                    point: 1,
                },
                QuestionOption {
                    text: "参与学校模拟政协/模拟联合国等实践活动",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您擅长的领导组织能力：",
            question_type: QuestionType::GoodAt,
            capacity_category: CapacityCategory::E,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "曾担任班干部且能把班级事务管理得井井有条",
                    point: 1,
                },
                QuestionOption {
                    text: "能指导小组同学分工并监督任务完成进度",
                    point: 1,
                },
                QuestionOption {
                    text: "做事情有热情，能带动周围同学一起投入",
                    point: 1,
                },
                QuestionOption {
                    text: "能通过沟通调动同学积极性，完成集体目标",
                    point: 1,
                },
                QuestionOption {
                    text: "曾成功在跳蚤市场卖出闲置物品或推销小物件",
                    point: 1,
                },
                QuestionOption {
                    text: "曾作为社团负责人组织过至少一次社团活动",
                    point: 1,
                },
                QuestionOption {
                    text: "敢向老师/学校提出班级管理或活动的改进建议",
                    point: 1,
                },
                QuestionOption {
                    text: "有组建学习小组/兴趣社团并运营的想法和能力",
                    point: 1,
                },
                QuestionOption {
                    text: "了解班级/社团管理的基本方法并能实践应用",
                    point: 1,
                },
                QuestionOption {
                    text: "能清晰表达观点并在辩论中说服他人认同",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您向往的经管/管理类专业方向：",
            question_type: QuestionType::LovedJob,
            capacity_category: CapacityCategory::E,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "工商管理",
                    point: 1,
                },
                QuestionOption {
                    text: "广播电视编导",
                    point: 1,
                },
                QuestionOption {
                    text: "企业管理",
                    point: 1,
                },
                QuestionOption {
                    text: "市场营销",
                    point: 1,
                },
                QuestionOption {
                    text: "房地产开发与管理",
                    point: 1,
                },
                QuestionOption {
                    text: "广告学",
                    point: 1,
                },
                QuestionOption {
                    text: "体育经济与管理",
                    point: 1,
                },
                QuestionOption {
                    text: "销售管理",
                    point: 1,
                },
                QuestionOption {
                    text: "创业管理",
                    point: 1,
                },
                QuestionOption {
                    text: "管理科学",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您感兴趣的校园事务管理活动：",
            question_type: QuestionType::InterestedEvent,
            capacity_category: CapacityCategory::C,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "整理自己的书桌、课本并做好分类收纳",
                    point: 1,
                },
                QuestionOption {
                    text: "帮老师抄写班级通知或整理同学作业",
                    point: 1,
                },
                QuestionOption {
                    text: "为班级活动撰写总结报告或活动方案",
                    point: 1,
                },
                QuestionOption {
                    text: "记录自己的零花钱收支并做月度预算",
                    point: 1,
                },
                QuestionOption {
                    text: "参加校园办公软件（Word/Excel）技能培训班",
                    point: 1,
                },
                QuestionOption {
                    text: "学习基础的文秘礼仪或班级事务管理技巧",
                    point: 1,
                },
                QuestionOption {
                    text: "了解基础会计知识并尝试做班级活动经费台账",
                    point: 1,
                },
                QuestionOption {
                    text: "学习用表格整理学科知识点或错题集",
                    point: 1,
                },
                QuestionOption {
                    text: "协助班主任整理班级档案、同学学籍信息",
                    point: 1,
                },
                QuestionOption {
                    text: "给亲友写节日贺卡或班级活动邀请函",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您擅长的事务管理能力：",
            question_type: QuestionType::GoodAt,
            capacity_category: CapacityCategory::C,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "能熟练使用输入法快速准确地录入中文文字",
                    point: 1,
                },
                QuestionOption {
                    text: "会用打印机/复印机完成资料复印或文件装订",
                    point: 1,
                },
                QuestionOption {
                    text: "能快速记录课堂重点或老师的口头通知",
                    point: 1,
                },
                QuestionOption {
                    text: "能分类整理课本、试卷和学习资料并定期归档",
                    point: 1,
                },
                QuestionOption {
                    text: "能高效完成老师安排的表格填写、数据统计等事务",
                    point: 1,
                },
                QuestionOption {
                    text: "会用算盘或计算器快速完成账目计算",
                    point: 1,
                },
                QuestionOption {
                    text: "能在短时间内整理好班级活动的大量文件资料",
                    point: 1,
                },
                QuestionOption {
                    text: "能熟练使用Excel做数据表格或PPT做简单汇报",
                    point: 1,
                },
                QuestionOption {
                    text: "能按要求搜集整理学科资料或活动数据",
                    point: 1,
                },
                QuestionOption {
                    text: "能为班级活动制定合理的经费预算并控制开支",
                    point: 1,
                },
            ]
        },
        Question {
            title: "（可多选或不选）选择您向往的财会/文职类专业方向：",
            question_type: QuestionType::LovedJob,
            capacity_category: CapacityCategory::C,
            is_multiple: true,
            options: &[
                QuestionOption {
                    text: "会计学",
                    point: 1,
                },
                QuestionOption {
                    text: "金融工程（银行方向）",
                    point: 1,
                },
                QuestionOption {
                    text: "税收学",
                    point: 1,
                },
                QuestionOption {
                    text: "信息管理与信息系统",
                    point: 1,
                },
                QuestionOption {
                    text: "财务管理",
                    point: 1,
                },
                QuestionOption {
                    text: "工程管理（成本方向）",
                    point: 1,
                },
                QuestionOption {
                    text: "档案学",
                    point: 1,
                },
                QuestionOption {
                    text: "数字媒体技术（办公方向）",
                    point: 1,
                },
                QuestionOption {
                    text: "法学（司法文书方向）",
                    point: 1,
                },
                QuestionOption {
                    text: "统计学",
                    point: 1,
                },
            ],
        },
        Question {
            title: "满分7分的话，您觉得自己的智能设备/手工工具操作能力在什么水平：",
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
            title: "满分7分的话，您觉得自己的体育技能/体能素质在什么水平：",
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
            title: "满分7分的话，您觉得自己的学科探究/小课题研究能力在什么水平：",
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
            title: "满分7分的话，您觉得自己的数学运算/逻辑推理能力在什么水平：",
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
            title: "满分7分的话，您觉得自己的绘画/写作/设计等艺术创作能力在什么水平：",
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
            title: "满分7分的话，您觉得自己的乐器演奏/乐感等音乐技能在什么水平：",
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
            title: "满分7分的话，您觉得自己的知识点讲解/观点表达能力在什么水平：",
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
            title: "满分7分的话，您觉得自己的同学交往/团队协作等交际技能在什么水平：",
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
            title: "满分7分的话，您觉得自己的校园义卖/活动推广等沟通洽谈能力在什么水平：",
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
            title: "满分7分的话，您觉得自己的小组活动/班级事务等领导组织能力在什么水平：",
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
            title: "满分7分的话，您觉得自己的作业整理/活动筹备等事务执行能力在什么水平：",
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
            title: "满分7分的话，您觉得自己的办公软件/资料统计等办公技能在什么水平：",
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
    ],
};
