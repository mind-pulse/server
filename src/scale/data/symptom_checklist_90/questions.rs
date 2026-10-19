use super::{OPTIONS, Question, Symptom};

pub(super) const QUESTIONS: &[Question] = &[
    Question {
        title: "头痛",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "神经过敏，心中不踏实",
        options: &OPTIONS,
        symptom: Symptom::Anxiety,
    },
    Question {
        title: "头脑中有不必要的想法或字句盘旋",
        options: &OPTIONS,
        symptom: Symptom::ObsessiveCompulsive,
    },
    Question {
        title: "头晕或晕倒",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "对异性的兴趣减退",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "对旁人责备求全",
        options: &OPTIONS,
        symptom: Symptom::InterpersonalSensitivity,
    },
    Question {
        title: "感到别人能控制自己的思想",
        options: &OPTIONS,
        symptom: Symptom::Psychoticism,
    },
    Question {
        title: "责怪别人制造麻烦",
        options: &OPTIONS,
        symptom: Symptom::ParanoidIdeation,
    },
    Question {
        title: "忘性大",
        options: &OPTIONS,
        symptom: Symptom::ObsessiveCompulsive,
    },
    Question {
        title: "担心自己的衣饰整齐及仪态的端正",
        options: &OPTIONS,
        symptom: Symptom::ObsessiveCompulsive,
    },
    Question {
        title: "容易烦恼和激动",
        options: &OPTIONS,
        symptom: Symptom::Hostility,
    },
    Question {
        title: "胸痛",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "害怕空旷的场所或街道",
        options: &OPTIONS,
        symptom: Symptom::PhobicAnxiety,
    },
    Question {
        title: "感到自己的精力下降，活动减慢",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "想结束自己的生命",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "听到旁人听不到的声音",
        options: &OPTIONS,
        symptom: Symptom::Psychoticism,
    },
    Question {
        title: "发抖",
        options: &OPTIONS,
        symptom: Symptom::Anxiety,
    },
    Question {
        title: "感到大多数人都不可信任",
        options: &OPTIONS,
        symptom: Symptom::ParanoidIdeation,
    },
    Question {
        title: "胃口不好",
        options: &OPTIONS,
        symptom: Symptom::Others,
    },
    Question {
        title: "容易哭泣",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "同异性相处时感到害羞不自在",
        options: &OPTIONS,
        symptom: Symptom::InterpersonalSensitivity,
    },
    Question {
        title: "感到受骗，中了圈套或有人想抓住你",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "无缘无故地突然感到害怕",
        options: &OPTIONS,
        symptom: Symptom::Anxiety,
    },
    Question {
        title: "自己不能控制地大发脾气",
        options: &OPTIONS,
        symptom: Symptom::Hostility,
    },
    Question {
        title: "怕单独出门",
        options: &OPTIONS,
        symptom: Symptom::PhobicAnxiety,
    },
    Question {
        title: "经常责怪自己",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "腰痛",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "感到难以完成任务",
        options: &OPTIONS,
        symptom: Symptom::ObsessiveCompulsive,
    },
    Question {
        title: "感到孤独",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "感到苦闷",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "过分担忧",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "对事物不感兴趣",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "感到害怕",
        options: &OPTIONS,
        symptom: Symptom::Anxiety,
    },
    Question {
        title: "你的感情容易受到伤害",
        options: &OPTIONS,
        symptom: Symptom::InterpersonalSensitivity,
    },
    Question {
        title: "旁人能知道自己的私下想法",
        options: &OPTIONS,
        symptom: Symptom::Psychoticism,
    },
    Question {
        title: "感到别人不理解你、不同情你",
        options: &OPTIONS,
        symptom: Symptom::InterpersonalSensitivity,
    },
    Question {
        title: "感到人们对你不友好，不喜欢你",
        options: &OPTIONS,
        symptom: Symptom::InterpersonalSensitivity,
    },
    Question {
        title: "做事必须做得很慢，以保证做得正确",
        options: &OPTIONS,
        symptom: Symptom::ObsessiveCompulsive,
    },
    Question {
        title: "心跳得很厉害",
        options: &OPTIONS,
        symptom: Symptom::Anxiety,
    },
    Question {
        title: "恶心或胃部不舒服",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "感到比不上他人",
        options: &OPTIONS,
        symptom: Symptom::InterpersonalSensitivity,
    },
    Question {
        title: "肌肉酸痛",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "感到有人在监视你、谈论你",
        options: &OPTIONS,
        symptom: Symptom::ParanoidIdeation,
    },
    Question {
        title: "难以入睡",
        options: &OPTIONS,
        symptom: Symptom::Others,
    },
    Question {
        title: "做事，必须反复检查",
        options: &OPTIONS,
        symptom: Symptom::ObsessiveCompulsive,
    },
    Question {
        title: "难以作出决定",
        options: &OPTIONS,
        symptom: Symptom::ObsessiveCompulsive,
    },
    Question {
        title: "怕乘电车、公共汽车、地铁或火车",
        options: &OPTIONS,
        symptom: Symptom::PhobicAnxiety,
    },
    Question {
        title: "呼吸有困难",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "一阵阵发冷或发热",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "因为感到害怕而避开某些东西、场合或活动",
        options: &OPTIONS,
        symptom: Symptom::PhobicAnxiety,
    },
    Question {
        title: "脑子变空了",
        options: &OPTIONS,
        symptom: Symptom::ObsessiveCompulsive,
    },
    Question {
        title: "身体发麻或刺痛",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "喉咙有梗塞感",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "感到前途没有希望",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "不能集中注意力",
        options: &OPTIONS,
        symptom: Symptom::ObsessiveCompulsive,
    },
    Question {
        title: "感到身体的某一部分软弱无力",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "感到紧张或容易紧张",
        options: &OPTIONS,
        symptom: Symptom::Anxiety,
    },
    Question {
        title: "感到手或脚发重",
        options: &OPTIONS,
        symptom: Symptom::Somatization,
    },
    Question {
        title: "想到死亡的事",
        options: &OPTIONS,
        symptom: Symptom::Others,
    },
    Question {
        title: "吃得太多",
        options: &OPTIONS,
        symptom: Symptom::Others,
    },
    Question {
        title: "当别人看着自己或谈论自己时感到不自在",
        options: &OPTIONS,
        symptom: Symptom::InterpersonalSensitivity,
    },
    Question {
        title: "有一些不属于你自己的想法",
        options: &OPTIONS,
        symptom: Symptom::Psychoticism,
    },
    Question {
        title: "有想打人或伤害他人的冲动",
        options: &OPTIONS,
        symptom: Symptom::Hostility,
    },
    Question {
        title: "醒得太早",
        options: &OPTIONS,
        symptom: Symptom::Others,
    },
    Question {
        title: "必须反复洗手、点数目或触摸某些东西",
        options: &OPTIONS,
        symptom: Symptom::ObsessiveCompulsive,
    },
    Question {
        title: "睡得不稳不深",
        options: &OPTIONS,
        symptom: Symptom::Others,
    },
    Question {
        title: "有想摔坏或破坏东西的冲动",
        options: &OPTIONS,
        symptom: Symptom::Hostility,
    },
    Question {
        title: "有一些别人没有的想法或念头",
        options: &OPTIONS,
        symptom: Symptom::ParanoidIdeation,
    },
    Question {
        title: "感到对别人神经过敏",
        options: &OPTIONS,
        symptom: Symptom::InterpersonalSensitivity,
    },
    Question {
        title: "在商店或电影院等人多的地方感到不自在",
        options: &OPTIONS,
        symptom: Symptom::PhobicAnxiety,
    },
    Question {
        title: "感到任何事情都很困难",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "一阵阵恐惧或惊恐",
        options: &OPTIONS,
        symptom: Symptom::Anxiety,
    },
    Question {
        title: "感到公共场合吃东西很不舒服",
        options: &OPTIONS,
        symptom: Symptom::InterpersonalSensitivity,
    },
    Question {
        title: "经常与人争论",
        options: &OPTIONS,
        symptom: Symptom::Hostility,
    },
    Question {
        title: "单独一人时神经很紧张",
        options: &OPTIONS,
        symptom: Symptom::PhobicAnxiety,
    },
    Question {
        title: "别人对你的成绩没有做出恰当的评价",
        options: &OPTIONS,
        symptom: Symptom::ParanoidIdeation,
    },
    Question {
        title: "即使和别人在一起也感到孤单",
        options: &OPTIONS,
        symptom: Symptom::Psychoticism,
    },
    Question {
        title: "感到坐立不安心神不定",
        options: &OPTIONS,
        symptom: Symptom::Anxiety,
    },
    Question {
        title: "感到自己没有什么价值",
        options: &OPTIONS,
        symptom: Symptom::Depression,
    },
    Question {
        title: "感到熟悉的东西变成陌生或不像真的",
        options: &OPTIONS,
        symptom: Symptom::Anxiety,
    },
    Question {
        title: "大叫或摔东西",
        options: &OPTIONS,
        symptom: Symptom::Hostility,
    },
    Question {
        title: "害怕会在公共场合晕倒",
        options: &OPTIONS,
        symptom: Symptom::PhobicAnxiety,
    },
    Question {
        title: "感到别人想占自己的便宜",
        options: &OPTIONS,
        symptom: Symptom::ParanoidIdeation,
    },
    Question {
        title: "为一些有关性的想法而苦恼",
        options: &OPTIONS,
        symptom: Symptom::Psychoticism,
    },
    Question {
        title: "你认为应该因为自己的过错而受到惩罚",
        options: &OPTIONS,
        symptom: Symptom::Psychoticism,
    },
    Question {
        title: "感到要很快把事情做完",
        options: &OPTIONS,
        symptom: Symptom::Anxiety,
    },
    Question {
        title: "感到自己的身体有严重问题",
        options: &OPTIONS,
        symptom: Symptom::Psychoticism,
    },
    Question {
        title: "从未感到和其他人很亲近",
        options: &OPTIONS,
        symptom: Symptom::Psychoticism,
    },
    Question {
        title: "感到自己有罪",
        options: &OPTIONS,
        symptom: Symptom::Others,
    },
    Question {
        title: "感到自己的脑子有毛病",
        options: &OPTIONS,
        symptom: Symptom::Psychoticism,
    },
];
