use super::{CapacityCategory, CapacityCategoryInterpretation};

pub(super) const CAPACITY_CATEGORY_INTERPRETATIONS: [CapacityCategoryInterpretation; 6] = [
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::R,
        name: "现实型",
        personality_trait: "这个类型的人喜欢使用工具、机器操作，偏爱具体任务而非抽象思想。务实、踏实，体力好，更喜欢与物打交道。喜欢以物质、机械、动物、工作等为对象，从事有规律、清晰、有序、系统的工作。喜欢动手实践解决问题。",
        occupational_stigma: "适合这类人的专业有工学门类专业（机械类、电气类、电子信息类等）、农学门类专业（植物生产类、动物生产类、动物医学类等）、医学门类专业（临床医学类、口腔医学类、中医学类等）。",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::I,
        name: "研究型",
        personality_trait: "也叫探索型。这个类型的人擅长思考和观察，喜欢理论分析和独立研究，求知欲强、逻辑清晰、独立自主、不轻信结论，善于提出疑问，通过逻辑和证据来验证想法，喜欢研究物，抽象思维，不喜欢研究人，更喜欢与概念、数据、理论打交道。",
        occupational_stigma: "适合这类人的专业有理学门类专业（数学类、物理学类、化学类、生物学类等）、科研方向的医学门类专业（基础医学类、临床医学类、药学类、法医学类等）、哲学门类专业、经济学门类专业（经济学类、财政学类、金融学类）。",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::A,
        name: "艺术型",
        personality_trait: "这个类型的人充满创造力与想象力，喜欢通过自我表达、创作来展现个人想法和价值观，厌恶重复、僵化的工作，追求美感与独特性，对色彩、声音、文字形式等有敏锐的感受力，注重作品的审美价值和原创性，情感丰富与直觉性强，倾向于依赖情感、直觉和想象来处理问题，渴望自由与独立。",
        occupational_stigma: "适合这类人的专业有艺术学门类专业（音乐与舞蹈学类、戏剧与影视学类、美术学类、设计学类等）、文学门类专业（中国语言文学类、外国语言文学类、新闻传播学类）、工学门类专业（建筑类）",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::S,
        name: "社会型",
        personality_trait: "这个类型的人天生喜欢帮助、教导、服务他人，并能从中获得巨大的满足感和价值感，善于倾听、理解、沟通和协调，能够敏锐地察觉他人的情绪和需求，心肠软、有爱心，愿意为他人或社会福祉付出努力，更喜欢团队合作、人际互动，而不是独自面对机器、数据和工具。",
        occupational_stigma: "适合这类人的专业有教育学门类专业（教育学类、体育学类）、医学门类专业（临床医学类、口腔医学类、中医学类等）、法学门类专业（法学类、社会学类、马克思主义理论类等）、管理学门类专业（工商管理类、公共管理类）。",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::E,
        name: "企业型",
        personality_trait: "也叫事业型或管理型。这个类型的人喜欢竞争，追求成就，渴望获得领导地位和声望，精力充沛，善于言辞，乐于推销自己的观点、产品甚至自己，不畏惧风险，愿意为可能的高回报做出决策并承担后果，更喜欢通过控制、说服他人来达成组织或个人的目标。",
        occupational_stigma: "适合这类人的专业有经济学门类专业（经济学类、财政学类、金融学类等）、管理学门类专业（管理科学与工程类、工商管理类、公共管理类等）、法学门类专业（法学类、政治学类、社会学类）。",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::C,
        name: "传统型",
        personality_trait: "也叫常规型。这个类型的人喜欢结构清晰、流程明确、秩序井然的环境和工作任务，讨厌混乱和不确定性，能够高效、准确地进行数据处理、文书归档等需要耐心和细心的工作，并以零错误为荣，享受可预测、稳定的工作内容，不喜欢频繁的变化和冒险，更喜欢独立处理文件、数字、系统，而不是进行大量的社交、说服或创意活动。",
        occupational_stigma: "适合这类人的专业有管理学门类专业（管理科学与工程类、工商管理类、公共管理类等）、经济学门类专业（经济学类、财政学类、金融学类等）、理学门类专业（数学类、统计学类、心理学类）、文学门类专业（中国语言文学类、外国语言文学类、新闻传播学类）。",
    },
];
