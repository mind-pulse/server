use super::{QUESTION_OPTIONS, Question, Type};

pub(super) const QUESTIONS: &[Question] = &[
    Question {
        title: "我很容易迷惑",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我不想成为一个喜欢批评的人,但很难做到",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我喜欢研究宇宙的道理,哲理",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很注意自己是否年轻,因为那是找乐子的本钱",
        r#type: Type::Enthusiast,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我喜欢独自自主,一切都靠自己",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "当我有困难时，我会试着不让人知道",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "被人误解对我而言是一件十分痛苦的事。",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "施比受会给我更大的满足感",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我常常设想最糟的结果而使自己陷入苦恼中",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我常常试探或考验朋友,伴侣的忠诚",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我看不起那些不像我一样坚强的人,有时我会用种种方式羞辱他们",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "身体上的舒适对我非常重要",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我能触碰生活中的悲伤和不幸",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "别人不能完成他的分内事,会令我失望和愤怒",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我时常拖延问题,不去解决",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我喜欢戏剧性,多彩多姿的生活",
        r#type: Type::Enthusiast,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我认为自己非常不完善",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我对感官的需求特别强烈,喜欢美食,服装,身体的触觉刺激,并纵情享乐",
        r#type: Type::Enthusiast,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "当别人请教我一些问题,我会巨细无遗地分析得很清楚",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我习惯推销自己,从不觉得难为情",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "有时我会放纵和做出*越的事",
        r#type: Type::Enthusiast,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "帮助不到别人会让我觉得痛苦",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我不喜欢人家问我广泛,笼统的问题",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "在某方面我有放纵的倾向(例如食物,药物等)",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我宁愿适应别人,包括我的伴侣,而不会反抗他们",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我最不喜欢的一件事就是虚伪",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我知错能改,但由于执著好强,周围的人还是感觉到压力",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我常觉得很多事情都很好玩,很有趣,人生真是快乐",
        r#type: Type::Enthusiast,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我有时很欣赏自己充满权威,有时又优柔寡断,依赖别人",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我习惯付出多于接受",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "面对威胁时,我一是变得焦虑,一是对抗迎面而来的危险",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我通常是等别人来接近我,而不是我去就接近他们",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我喜欢当主角,希望得到大家的注意",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "别人批评我,我也不会回应和辩解,因为我不想发生任何争执与冲突",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我有时期待别人的指导,有时却忽略别人的忠告径直去做我想做的事",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我经常忘记自己的需要",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "在重大危机中,我通常能克服我对自己的质疑与内心的焦虑",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我是一个天生的推销员,说服别人对我来说是一件轻易的事",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我不相信一个我一直都无法了解的的人",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我爱依惯例行事,不大喜欢改变",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很在乎家人,在家中表现得忠诚和包容",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我被动而优柔寡断",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很有包容力,彬彬有礼,但跟人的感情互动不深",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我沉默寡言,好象不会关心别人似的",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "当沉浸在工作或我擅长的领域时,别人会觉得我冷酷无情",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我常常保持警觉",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我不喜欢要对人尽义务的感觉",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "如果不能完美的表态,我宁愿不说",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我的计划比我实际完成的还要多",
        r#type: Type::Enthusiast,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我野心勃勃,喜欢挑战和登上高峰的经验",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我倾向于独断专行并自己解决问题",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很多时候感到被遗弃",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我常常表现得十分忧郁的样子,充满痛苦而且内向",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "初见陌生人时,我会表现得很冷漠,高傲",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我的面部表情严肃而生硬",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很飘忽,常常不知自己下一刻想要什么",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我常对自己挑剔,期望不断改善自己的缺点,以成为一个完美的人",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我感受特别深刻,并怀疑那些总是很快乐的人",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我做事有效率,也会找捷径,模仿力特强",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我讲理,重实用",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我有很强的创造天分和想象力,喜欢将事情重新整合",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我不要求得到太多的注意力",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我喜欢每件事都井然有序,但别人会认为我过分执著",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我渴望拥有完美的心灵伴侣",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我常夸耀自己,对自己的能力十分有信心",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "如果周遭的人行为太过分时,我准会让他难堪",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我外向,精力充沛,喜欢不断追求成就,这使我的自我感觉十分良好",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我是一位忠实的朋友和伙伴",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我知道如何让别人喜欢我",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很少看到别人的功劳和好处",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很容易知道别人的功劳和好处",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我嫉妒心强,喜欢跟别人比较",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我对别人做的事总是不放心,批评一番后,自己会动手再做",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "别人会说我常常带着面具做人",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "有时我会激怒对方,引来莫名其妙的吵架,其实我是想试探对方爱不爱我",
        r#type: Type::Loyalist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我会极力保护我所爱的人",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我常常可以保持兴奋的情绪",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我只喜欢与有趣的人交友,对一些闷蛋却懒得交往,即使他们看来很有深度",
        r#type: Type::Enthusiast,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我常往外跑,四处帮助别人",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "有时我会讲求效率而牺牲完美和原则",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我似乎不太懂得幽默,没有弹性",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我待人热情而有耐性",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "在人群中我时常感到害羞和不安",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我喜欢效率,讨厌拖泥带水",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "帮助别人达致快乐和成功是我重要的成就",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "付出时,别人若不欣然接纳,我便会有挫折感",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我的肢体硬邦邦的,不习惯别人热情的付出",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我对大部分的社交集会不太有兴趣,除非那是我熟识的和喜爱的人",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "很多时候我会有强烈的寂寞感",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "人们很乐意向我表白他们所遭遇的问题",
        r#type: Type::Helper,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我不但不会说甜言蜜语,而且别人会觉得我唠叨不停",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我常担心自由被剥夺,因此不爱作承诺",
        r#type: Type::Enthusiast,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我喜欢告诉别人我所做的事和所知的一切",
        r#type: Type::Achiever,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很容易认同别人为我所做的事和所知的一切",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我要求光明正大,为此不惜与人发生冲突",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很有正义感,有时会支持不利的一方",
        r#type: Type::Challenger,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我注重小节而效率不高",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我容易感到沮丧和麻木更多于愤怒",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我不喜欢那些侵略性或过度情绪化的人",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我非常情绪化,一天的喜怒哀乐多变",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我不想别人知道我的感受与想法,除非我告诉他们",
        r#type: Type::Investigator,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我喜欢刺激和紧张的关系,而不是稳定和依赖的关系",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很少用心去听别人的心情,只喜欢说说俏皮话和笑话",
        r#type: Type::Enthusiast,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我是循规蹈矩的人,秩序对我十分有意义",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我很难找到一种我真正感到被爱的关系",
        r#type: Type::Individualist,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "假如我想要结束一段关系,我不是直接告诉对方就是激怒他来让他离开我",
        r#type: Type::Reformer,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我温和平静,不自夸,不爱与人竞争",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
    Question {
        title: "我有时善良可爱,有时又粗野暴躁,很难捉摸",
        r#type: Type::Peacemaker,
        options: QUESTION_OPTIONS,
    },
];
