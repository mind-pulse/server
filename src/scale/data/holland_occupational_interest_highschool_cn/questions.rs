use super::super::holland_occupational_interest::{CapacityCategory, Question, QuestionType};
use crate::scale::QuestionOption;

pub(super) const QUESTIONS: &[Question] = &[
    Question {
        title: "（可多选或不选）选择你感兴趣的校园科技活动：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你擅长的科技实践能力：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你向往的工科/技术类专业方向：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你感兴趣的校园艺术活动：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你擅长的艺术创作能力：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你向往的艺术/文创类专业方向：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你感兴趣的学术研究活动：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你擅长的学术研究能力：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你向往的理科/科研类专业方向：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你感兴趣的校园社会活动：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你擅长的社会实践能力：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你向往的教育/服务类专业方向：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你感兴趣的校园领导活动：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你擅长的领导组织能力：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你向往的经管/管理类专业方向：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你感兴趣的校园事务管理活动：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你擅长的事务管理能力：",
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
        ],
    },
    Question {
        title: "（可多选或不选）选择你向往的财会/文职类专业方向：",
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
        title: "满分7分的话，你觉得自己的智能设备/手工工具操作能力在什么水平：",
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
        title: "满分7分的话，你觉得自己的体育技能/体能素质在什么水平：",
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
        title: "满分7分的话，你觉得自己的学科探究/小课题研究能力在什么水平：",
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
        title: "满分7分的话，你觉得自己的数学运算/逻辑推理能力在什么水平：",
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
        title: "满分7分的话，你觉得自己的绘画/写作/设计等艺术创作能力在什么水平：",
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
        title: "满分7分的话，你觉得自己的乐器演奏/乐感等音乐技能在什么水平：",
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
        title: "满分7分的话，你觉得自己的知识点讲解/观点表达能力在什么水平：",
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
        title: "满分7分的话，你觉得自己的同学交往/团队协作等交际技能在什么水平：",
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
        title: "满分7分的话，你觉得自己的校园义卖/活动推广等沟通洽谈能力在什么水平：",
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
        title: "满分7分的话，你觉得自己的小组活动/班级事务等领导组织能力在什么水平：",
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
        title: "满分7分的话，你觉得自己的作业整理/活动筹备等事务执行能力在什么水平：",
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
        title: "满分7分的话，你觉得自己的办公软件/资料统计等办公技能在什么水平：",
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
