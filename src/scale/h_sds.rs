use serde::Serialize;

use crate::scale::ScaleCategory;

use super::{HTMLElement, PlainText, QuestionOption, Scale, SentenceItem, Tag, Texts};

/// 问题类型
#[derive(Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
enum QuestionType {
    /// 感兴趣的事
    InterestedEvent,
    /// 擅长的事
    GoodAt,
    /// 喜欢的职业
    LovedJob,
    /// 能力类型
    CapacityCategory,
}

/// 人格类型
#[derive(Serialize)]
enum CapacityCategory {
    R,
    A,
    I,
    S,
    E,
    C,
}

#[derive(Serialize)]
pub struct Question {
    title: PlainText,
    question_type: QuestionType,
    capacity_category: CapacityCategory,
    options: &'static [QuestionOption],
    is_multiple: bool,
}

#[derive(Serialize)]
struct CapacityCategoryInterpretation {
    capacity_category: CapacityCategory,
    name: PlainText,
    /// 人格特征
    personality_trait: PlainText,
    /// 职业特征
    occupational_stigma: PlainText,
}

const CAPACITY_CATEGORY_INTERPRETATIONS: [CapacityCategoryInterpretation; 6] = [
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::R,
        name: "现实型",
        personality_trait: "“安分随流、直率坦诚、实事求是、循规蹈矩、坚忍不拔、勤劳节俭”是对这类人较好的描述。他们的动手能力较强，喜欢与机器、工具打交道，喜欢实际操作，做事喜欢遵循一定的规则。他们做人很现实，不是个理想主义者，并且追求安定、舒适的生活。但是。这类人通常表达能力不强，不善与人交际，思想较保守，对新鲜事物不太感兴趣，情感体验也不太丰富，他们的生活也许缺少一些情趣。",
        occupational_stigma: "需要进行明确、具体分工的，并有一定程序要求的技术型、技能型工作。如机械制造、建筑、渔业、野外工作、工程技术等职业。（木匠、农民、操作X光的技师、工程师、飞机机械师、鱼类和野生动物专家、自动化技师、机械工（车工、钳工等）、电工、无线电报务员、火车司机、长途公共汽车司机、机械制图员、修理机器、电器师）",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::I,
        name: "研究型",
        personality_trait: "也叫探索型。这个类型的人对自然现象和自然规律很感兴趣，喜欢同观念而不是同人或事务打交道，对工作有着极大的热情。他们的思维逻辑性较强，善于通过分析思考解决面临的难题，但并不一定实现具体的操作。喜欢面对疑问和不懈的挑战，不愿循规蹈矩，总是渴望创新。他们为人慎重而敏感，追求的是内在自我价值的实现，而非物质生活的质量。他们将自己描绘成“分析型的、内省的、独立的、好奇心强烈的和含蓄的”。",
        occupational_stigma: "善于通过观察和科学分析进行系统的创造性活动，一般的研究对象侧重于自然科学而不是社会科学方面。适合这类人的职业有生物学、物理学。气象学、天文学等自然科学方面的科学工作者，化学技师、实验研究人员，建筑设计师等，计算机程序设计员、工程师等。（气象学者、生物学者、天文学家、药剂师、动物学者、化学家、科学报刊编辑、地质学者、植物学者、物理学者、数学家、实验员、科研人员、科技作者）",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::A,
        name: "艺术型",
        personality_trait: "艺术型的人有很强的自我表现欲，对自己十分自信，喜欢通过新颖的设计引起别人情感上的共呜。他们的想象力很丰富，创造力很强，喜欢凭直觉做出判断，感情丰富，容易冲动，甚至可以为追求心中的理想抛弃一切。但他们的生活也许是杂乱无章、缺少秩序的。艺术型的人可以描述为“独立不倚、创新求异、热衷表现、激情洋溢、感情丰富和理想主义”。",
        occupational_stigma: "善于通过非系统化的、自由的活动进行艺术表现，但精细的操作能力较差。相应职业有演员、影视工作人员、画家、歌唱家、音乐演奏家、诗人、作家，工艺美术设计人员等。（室内装饰专家、图书管理专家、摄影师、音乐教师、作家、演员、记者、诗人、作曲家、编剧、雕刻家、漫画家）",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::S,
        name: "社会型",
        personality_trait: "社会型的人有较强的社会责任感和人道主义倾向，社会适应能力较强。他们善于与人交往，喜欢周围有别人存在，对别人的事很有兴趣，乐于帮助别人解决难题。这种人喜欢与人而不是与事物打交道。“助人为乐、有责任心、热情、开朗、友好、善良、易于合作”是对他们较好的描述。这类人喜欢社会交往性工作，如教师、医生、护士、公关人员、营销人员等。",
        occupational_stigma: "从事更多时间与人打交道的说服、教育和治疗工作。如教师、护士、心理学工作者，社会活动家等。（社会学者、导游、福利机构工作者、咨询人员、社会工作者、社会科学教师、学校领导、精神病工作者、公共保健护士）",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::E,
        name: "企业型",
        personality_trait: "也叫事业型或管理型。这个类型的人充满自信，喜欢竞争和冒险。好成为领导者，好支配他人，善辞令，好与人争辩，总试图让别人接受自己的观点。他们不愿从事精细工作，不喜欢需要长期复杂思维的工作。不愿被人支配，不易与人合作。在别人眼中，他们是“敢做敢为的、信心百倍的、乐观的、冲动的、自我显示的、精力旺盛的”。",
        occupational_stigma: "适于从事需要胆略、冒风险和承担责任的活动。主要指管理、决策方面的工作。如经理、推销员、电视节目主持人、政治家等。（推销员、进货员、商品批发员、旅馆经理、饭店经理、广告宣传员、调度员、律师、政治家、零售商）",
    },
    CapacityCategoryInterpretation {
        capacity_category: CapacityCategory::C,
        name: "传统型",
        personality_trait: "传统型的人喜欢有秩序的、安稳的生活，做事有计划；乐于执行上级派下来的任务；讲求精确，不愿冒险；想象力和创造力较差。下面的形容词可以很好的描述他们“循规蹈矩、踏实稳当、忠实可靠、顺从听话”。他们与现实型的人区别在于他们对花大量体力和脑力的活动不感兴趣。",
        occupational_stigma: "传统型的人适于从事的工作有以下特点：严格按照固定的规则和方法进行的重复性、习惯性的活动，能够较快的见到自己的劳动成果，需要一定的自控能力。具体职业有会计、录入员、图书管理员、审计员、出纳员、秘书、邮递员、税务员、统计员等等。（记账员、法庭速记员、成本估算员、税务员、核算员、打字员、办公室职员、计算机操作员）",
    },
];

#[derive(Serialize)]
struct CareerInformation {
    code: PlainText,
    information: PlainText,
}

const CAREER_INFORMATION: [CareerInformation; 72] = [
    CareerInformation {
        code: "RIA",
        information: "牙科技术员、陶工、建筑设计员、模型工、细木工、制作链条人员。",
    },
    CareerInformation {
        code: "RIS",
        information: "厨师、林务员、跳水员、潜水员、染色员、电器修理、眼镜制作、电工、纺织机器装配工、服务员、装玻璃工人、发电厂工人、焊接工。",
    },
    CareerInformation {
        code: "RIE",
        information: "建筑和桥梁工程、环境工程、航空工程、公路工程、电力工程、信号工程、电话工程、一般机械工程、自动工程、矿业工程、海洋工程、交通工程技术人员、制图员、家政经济人员、计量员、农民、农场工人、农业机械操作、清洁工、无线电修理、汽车修理、手表修理、管工、线路装配工、工具仓库管理员。",
    },
    CareerInformation {
        code: "RIC",
        information: "船上工作人员、接待员、杂志保管员、牙医助手、制帽工、磨坊工、石匠、机器制造、机车(火车头)制造、农业机器装配、汽车装配工、缝纫机装配工、钟表装配和检验、电动器具装配、鞋匠、锁匠、货物检验员、电梯机修工、托儿所所长、钢琴调音员、装配工、印刷工、建筑钢铁工作、卡车司机。",
    },
    CareerInformation {
        code: "RAI",
        information: "手工雕刻、玻璃雕刻、制作模型人员、家具木工、制作皮革品、手工绣花、手工钩针纺织、排字工作、印刷工作、图画雕刻、装订工。",
    },
    CareerInformation {
        code: "RSE",
        information: "消防员、交通巡警、警察、门卫、理发师、房间清洁工、屠夫、锻工、开凿工人、管道安装工、出租汽车驾驶员、货物搬运工、送报员、勘探员、娱乐场所的服务员、起卸机操作工、灭害虫者、电梯操作工、厨房助手。",
    },
    CareerInformation {
        code: "RSI",
        information: "纺织工、编织工、农业学校教师、某些职业课程教师(诸如艺术、商业、技术、工艺课程)、雨衣上胶工。",
    },
    CareerInformation {
        code: "REC",
        information: "抄水表员、保姆、实验室动物饲养员、动物管理员。",
    },
    CareerInformation {
        code: "REI",
        information: "轮船船长、航海领航员、大副、试管实验员。",
    },
    CareerInformation {
        code: "RES",
        information: "旅馆服务员、家畜饲养员、渔民、渔网修补工、水手长、收割机操作工、搬运行李工人、公园服务员、救  生员、登山导游、火车工程技术员、建筑工作、铺轨工人。",
    },
    CareerInformation {
        code: "RCI",
        information: "测量员、勘测员、仪表操作者、农业工程技术、化学工程技师、民用工程技师、石油工程技师、资料室管理员、探矿工、煅烧工、烧窖工、矿工、保养工、磨床工、取样工、样品检验员、纺纱工、炮手、漂洗工、电焊工、锯木工、刨床工、制帽工、手工缝纫工、油漆工、染色工、按摩工、木匠、农民建筑工作、电影放映员、勘测员助手。",
    },
    CareerInformation {
        code: "RCS",
        information: "公共汽车驾驶员、一等水手、游泳池服务员、裁缝、建筑工作、石匠、烟囱修建工、混凝土工、电话修理工、爆炸手、邮递员、矿工、裱糊工人、纺纱工。",
    },
    CareerInformation {
        code: "RCE",
        information: "打井工、吊车驾驶员、农场工人、邮件分类员、铲车司机、拖拉机司机。",
    },
    CareerInformation {
        code: "IAS",
        information: "普通经济学家、农场经济学家、财政经济学家、国际贸易经济学家、实验心理学家、工程心理学家、心理学家、哲学家、内科医生、数学家。",
    },
    CareerInformation {
        code: "IAR",
        information: "人类学家、天文学家、化学/物理学家、医学病理、动物标本剥制者、化石修复者、艺术品管理者。",
    },
    CareerInformation {
        code: "ISE",
        information: "营养学家、饮食顾问、火灾检查员、邮政服务检查员。",
    },
    CareerInformation {
        code: "ISC",
        information: "侦察员、播音室/电视修理服务员、验尸室人员、编目录者、医学实验定技师、调查研究者。",
    },
    CareerInformation {
        code: "ISR",
        information: "水生生物学者，昆虫学者、微生物学家、配镜师、矫正视力者、细菌学家、牙科医生、骨科医生。",
    },
    CareerInformation {
        code: "ISA",
        information: "实验心理学家、普通心理学家、发展心理学家、教育心理学家、社会心理学家、临床心理学家、目标学家、皮肤病学家、精神病学家、妇产科医师、眼科医生、五官科医生、医学实验室技术专家、民航医务人员、护士。",
    },
    CareerInformation {
        code: "IES",
        information: "细菌学家、生理学家、化学专家、地质专家、地理物理学专家、纺织技术专家、医院药剂师、工业药剂师、药房营业员。",
    },
    CareerInformation {
        code: "IEC",
        information: "档案保管员、保险统计员。",
    },
    CareerInformation {
        code: "ICR",
        information: "质量检验技术员、地质学技师、工程师、法官、图书馆技术辅导员、计算机操作员、医院听诊员、家禽检查员。",
    },
    CareerInformation {
        code: "IRA",
        information: "地理学家、地质学家、声学物理学家、矿物学家、古生物学家、石油学家、地震学家、声学物理学家、原子和分子物理学家、电学和磁学物理学家、气象学家、设计审核员、人口统计学家、数学统计学家、外科医生、城市规划家、气象员。",
    },
    CareerInformation {
        code: "IRS",
        information: "流体物理学家、物理海洋学家、等离子体物理学家、农业科学家、动物学家、食品科学家、园艺学家、植物学家、细菌学家、解剖学家、动物病理学家、作物病理学家、药物学家、生物化学家、生物物理学家、细胞生物学家、临床化学家、遗传学家、分子生物学家、质量控制工程师、地理学家、兽医、放射性治疗技师。",
    },
    CareerInformation {
        code: "IRE",
        information: "化验员、化学工程师、纺织工程师、食品技师、渔业技术专家、材料和测试工程师、电气工程师、土木工程师、航空工程师、行政官员、冶金专家、原子核工程师、陶瓷工程师、地质工程师、电力工程量、口腔科医生、牙科医生。",
    },
    CareerInformation {
        code: "IRC",
        information: "飞机领航员、飞行员、物理实验室技师、文献检查员、农业技术专家、动植物技术专家、生物技师、油管检查员、工商业规划者、矿藏安全检查员、纺织品检验员、照相机修理者、工程技术员、编计算程序者、工具设计者、仪器维修工。",
    },
    CareerInformation {
        code: "CRI",
        information: "簿记员、会计、记时员、铸造机操作工、打字员、按键操作工、复印机操作工。",
    },
    CareerInformation {
        code: "CRS",
        information: "仓库保管员、档案管理员、缝纫工、讲述员、收款人。",
    },
    CareerInformation {
        code: "CRE",
        information: "标价员、实验室工作者、广告管理员、自动打字机操作员、电动机装配工、缝纫机操作工。",
    },
    CareerInformation {
        code: "CIS",
        information: "记账员、顾客服务员、报刊发行员、土地测量员、保险公司职员、会计师、估价员、邮政检查员、外贸检查员。",
    },
    CareerInformation {
        code: "CIE",
        information: "打字员、统计员、支票记录员、订货员、校对员、办公室工作人员。",
    },
    CareerInformation {
        code: "CIR",
        information: "校对员、工程职员、海底电报员、检修计划员、发扳员。",
    },
    CareerInformation {
        code: "CSE",
        information: "接待员、通讯员、电话接线员、卖票员、旅馆服务员、私人职员、商学教师、旅游办事员。",
    },
    CareerInformation {
        code: "CSR",
        information: "运货代理商、铁路职员、交通检查员、办公室通信员、薄记员、出纳员、银行财务职员。",
    },
    CareerInformation {
        code: "CSA",
        information: "秘书、图书管理员、办公室办事员。",
    },
    CareerInformation {
        code: "CER",
        information: "邮递员、数据处理员、办公室办事员。",
    },
    CareerInformation {
        code: "CEI",
        information: "推销员、经济分析家。",
    },
    CareerInformation {
        code: "CES",
        information: "银行会计、记账员、法人秘书、速记员、法院报告人。",
    },
    CareerInformation {
        code: "ECI",
        information: "银行行长、审计员、信用管理员、地产管理员、商业管理员。",
    },
    CareerInformation {
        code: "ECS",
        information: "信用办事员、保险人员、各类进货员、海关服务经理、售货员，购买员、会计。",
    },
    CareerInformation {
        code: "ERI",
        information: "建筑物管理员、工业工程师、农场管理员、护士长、农业经营管理人员。",
    },
    CareerInformation {
        code: "ERS",
        information: "仓库管理员、房屋管理员、货栈监督管理员。",
    },
    CareerInformation {
        code: "ERC",
        information: "邮政局长、渔船船长、机械操作领班、木工领班、瓦工领班、驾驶员领班。",
    },
    CareerInformation {
        code: "EIR",
        information: "科学、技术和有关周期出版物的管理员。",
    },
    CareerInformation {
        code: "EIC",
        information: "专利代理人、鉴定人、运输服务检查员、安全检查员、废品收购人员。",
    },
    CareerInformation {
        code: "EIS",
        information: "警官、侦察员、交通检验员、安全咨询员、合同管理者、商人。",
    },
    CareerInformation {
        code: "EAS",
        information: "法官、律师、公证人。",
    },
    CareerInformation {
        code: "EAR",
        information: "展览室管理员、舞台管理员、播音员、训兽员。",
    },
    CareerInformation {
        code: "ESC",
        information: "理发师、裁判员、政府行政管理员、财政管理员、I程管理员、职业病防治、售货员、商业经理、办公室主任、人事负责人、调度员。",
    },
    CareerInformation {
        code: "ESR",
        information: "家具售货员、书店售货员、公共汽车驾驶员、日用品售货员、护士长、自然科学和工程的行政领导。",
    },
    CareerInformation {
        code: "ESI",
        information: "博物馆管理员、图书馆管理员、古迹管理员、饮食业经理、地区安全服务管理员、技术服务咨询者、超级市场管理员、零售商品店店员、批发商、出租汽车服务站调度。",
    },
    CareerInformation {
        code: "ESA",
        information: "博物馆馆长、报刊管理员、音乐器材售货员、广告商售画营业员、导游、(轮船或班机上的)事务长、飞机上的服务员、船员、法官、律师。",
    },
    CareerInformation {
        code: "ASE",
        information: "戏剧导演、舞蹈教师、广告撰稿人，报刊、专栏作者、记者、演员、英语翻译。",
    },
    CareerInformation {
        code: "ASI",
        information: "音乐教师、乐器教师、美术教师、管弦乐指挥，合唱队指挥、歌星、演奏家、哲学家、作家、广告经理、时装模特。",
    },
    CareerInformation {
        code: "AER",
        information: "新闻摄影师、电视摄影师、艺术指导、录音指导、丑角演员、魔术师、木偶戏演员、骑士、跳水员。",
    },
    CareerInformation {
        code: "AEI",
        information: "音乐指挥、舞台指导、电影导演。",
    },
    CareerInformation {
        code: "AES",
        information: "流行歌手、舞蹈演员、电影导演、广播节目主持人、舞蹈教师、口技表演者、喜剧演员、模特。",
    },
    CareerInformation {
        code: "AIS",
        information: "画家、剧作家、编辑、评论家、时装艺术大师、新闻摄影师、男演员、文学作者。",
    },
    CareerInformation {
        code: "AIE",
        information: "花匠、皮衣设计师、工业产品设计师、剪影艺术家、复制雕刻品大师。",
    },
    CareerInformation {
        code: "AIR",
        information: "建筑师、画家、摄影师、绘图员、环境美化工、园艺师、雕刻家、包装设计师、陶器设计师、绣花工、漫画工。",
    },
    CareerInformation {
        code: "SEC",
        information: "社会活动家、退伍军人服务官员、工商会事务代表、教育咨询者、宿舍管理员、旅馆经理、饮食服务管理员。",
    },
    CareerInformation {
        code: "SER",
        information: "体育教练、游泳指导。",
    },
    CareerInformation {
        code: "SEI",
        information: "大学校长、学院院长、医院行政管理员、历史学家、家政经济学家、职业学校教师、资料员。",
    },
    CareerInformation {
        code: "SEA",
        information: "娱乐活动管理员、国外服务办事员、社会服务助理、一般咨询者、宗教教育工作者。",
    },
    CareerInformation {
        code: "SCE",
        information: "部长助理、福利机构职员、生产协调人、环境卫生管理人员、戏院经理、餐馆经理、售票员。",
    },
    CareerInformation {
        code: "SRI",
        information: "外科医师助手、医院服务员。",
    },
    CareerInformation {
        code: "SRE",
        information: "体育教师、职业病治疗者、体育教练、专业运动员、房管员、儿童家庭教师、警察、引座员、传达员、保姆。",
    },
    CareerInformation {
        code: "SRC",
        information: "护理员、护理助理、医院勤杂工、理发师、学校儿童服务人员。",
    },
    CareerInformation {
        code: "SIA",
        information: "社会学家、心理咨询者、学校心理学家、政治科学家、大学或学院的系主任、大学或学院的教育学教师、大学农业教师、大学工程和建筑课程的教师、大学法律教师、大学数学、医学、物理、社会科学和生命科学的教师、研究生助教、成人教育教师。",
    },
    CareerInformation {
        code: "SIE",
        information: "营养学家、饮食学家、海关检查员、安全检查员、税务稽查员、校长。",
    },
    CareerInformation {
        code: "SIC",
        information: "描图员、兽医助手、诊所助理、体检检查员、监督缓刑犯的工作者、娱乐指导者、咨询人员、社会科学教师。",
    },
    CareerInformation {
        code: "SIR",
        information: "理疗员、救护队工作人员、手足病医生、职业病治疗助手。",
    },
];

#[derive(Serialize)]
pub struct Interpretation {
    capacity_category_interpretations: &'static [CapacityCategoryInterpretation],
    career_information: &'static [CareerInformation],
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
        SentenceItem::Plain("，从而更好地做出职业的决择。"),
    ],
    &[
        SentenceItem::Plain("在测评的时候，要求在一个"),
        SentenceItem::HTMLElement(HTMLElement::Strong(
            "不受干扰的、安静的、独立的空间和平静的心态",
        )),
        SentenceItem::Plain("下进行。"),
    ],
];

pub const SELF_DIRECTED_SEARCH: Scale<Interpretation, Question> = Scale {
    name: "霍兰德职业兴趣测评",
    primary_category: ScaleCategory::CareerAndAcademics,
    related_categories: Some(&[
        ScaleCategory::Personality,
        ScaleCategory::AttitudeAndValues,
    ]),
    abbreviation: "SDS", // 与抑郁自评量表缩写相同
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: Some(&[
        "霍兰德认为，个人职业兴趣特性与职业之间应有一种内在的对应关系。根据兴趣的不同，人格可分为研究型（I）、艺术型（A）、社会型（S）、企业型（E）、传统型（C）、现实型（R）六个维度，每个人的性格都是这六个维度的不同程度组合。",
        "霍兰德的职业兴趣理论主要从兴趣的角度出发来探索职业指导的问题。他明确提出了职业兴趣的人格观，使人们对职业兴趣的认识有了质的变化。霍兰德的职业兴趣理论反映了他长期专注于职业指导的实践经历，他把对职业环境的研究与对职业兴趣个体差异的研究有机地结合起来，而在霍兰德的职业兴趣类型理论提出之前，二者的研究是相对独立进行的。",
        "兴趣测试和能力测试的结合在职业指导和职业咨询的实际操作中起到了促进作用。",
    ]),
    references: None,
    warning: None,
    formula_mode: None,
    tags: Tag{ info: Some(&["职业", "高考", "就业", "专业"]), normal: Some(&["自评"]), warning: None, error: None },
    interpretation: Interpretation { capacity_category_interpretations: &CAPACITY_CATEGORY_INTERPRETATIONS, career_information: &CAREER_INFORMATION },
    questions: &[
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
            ]
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
                    text: "薄记人员",
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
    ],
};
