use super::{Factor, Question, QuestionOption};

pub(super) const QUESTIONS: &[Question] = &[
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
];
