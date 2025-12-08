use serde::Serialize;

use crate::scale::ScaleCategory;

use super::{HTMLElement, PlainText, PlainTexts, QuestionOption, Scale, SentenceItem, Tag, Texts};

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum Type {
    /// 完美主义者
    Reformer,
    /// 全爱自助型
    Helper,
    /// 成就型
    Achiever,
    /// 自我型
    Individualist,
    /// 智慧型
    Investigator,
    /// 忠诚型
    Loyalist,
    /// 活跃型
    Enthusiast,
    /// 领袖型
    Challenger,
    /// 和平型
    Peacemaker,
}


#[derive(Debug, Serialize)]
pub struct Question {
    title: PlainText,
    r#type: Type,
    options: [QuestionOption; 2],
}

const QUESTION_OPTIONS: [QuestionOption; 2] = [
    QuestionOption {
        point: 1,
        text: "是",
    },
    QuestionOption {
        point: 0,
        text: "否",
    },
];

#[derive(Debug, Serialize)]
struct Character {
    keyword: PlainText,
    description: PlainText,
}

#[derive(Debug, Serialize)]
pub struct TypeInterpretation {
    r#type: Type,
    desire_trait: PlainText,
    basic_confusion: PlainText,
    main_features: PlainText,
    main_traits: PlainText,
    lifestyle: PlainText,
    relationships: PlainText,
    characters: &'static [Character],
    basic_fear: PlainText,
    basic_desire: PlainText,
}

#[derive(Debug, Serialize)]
pub struct Interpretation {
    type_interpretations: &'static [TypeInterpretation],
    dialog: PlainTexts,
}

const INTRODUCTION: Texts = &[
    &[SentenceItem::Plain("九型人格测试问卷将有助于你更好地了解自身的优势和弱点，并知道在何种情形下你的行动将更为有效。同时，你还可以通过测评结论知道他人是如何看待他们自己的，以及相互间又是如何相处影响的。")],
    &[SentenceItem::Plain("九型人格测试属于一种自我测试。九型人格测试主要用于帮助你有效地掌握个人的行为习惯，测试中所回答的问题答案没有好与坏之分、没有正确与错误之别，它仅反映你自己的个性和你的世界观。")],
];

const INSTRUCTION: Texts = &[
    &[
        SentenceItem::Plain("本测试包含 "),
        SentenceItem::HTMLElement(HTMLElement::Strong("108")),
        SentenceItem::Plain(" 个项目，在符合你情况的题目下选择是，不符合的选择否。"),
    ],
];


pub const ENNEAGRAM_PERSONALITY_TEST: Scale<Interpretation, Question> = Scale {
    name: "九型人格测试",
    primary_category: ScaleCategory::Personality,
    related_categories: Some(&[        
        ScaleCategory::Emotion,
        ScaleCategory::Interpersonal,
        ScaleCategory::AttitudeAndValues,
    ]),
    abbreviation: "EPT",
    introduction: INTRODUCTION,
    instruction: INSTRUCTION,
    idea: Some(&[
        "人格被分为九型，你必然属于其中一型。而这个型就是你的基本人格型态。",
        "九型人格不仅仅是一种精妙的性格分析工具，更主要的是为个人修养、自我提升和历练提供更深入的洞察力。与当今其它性格分类法不同，九型人格揭示了人们内在最深层的价值观和注意力焦点，它不受表面的外在行为的变化所影响。它可以让人真正地知己知彼；可以帮助人们明白自己的个性，从而完全接纳自己的短处、活出自己的长处；可以让人明白其它不同人的个性类型，从而懂得如何与不同的人交往沟通及融洽相处，与别人建立更真挚、和谐的合作伙伴关系。",
        "虽然人的基本性格型态是不会改变，但是某一型的典型描述，却不见全然符合某一个人，原因正是上面说过的：人们为了顺应成长环境、社会文化，他们在安定或压力的情况下，有可能出现一些差异。而必须强调的是：每一个人的成长环境都是独一无二，所以同类型人之间可能有许多共同点，但却也各自拥有一些属于自己最特殊的特质。",
        "九型人格中，没有哪一型是全然属于“男人的型”，而哪一型又是“女人专属”。",
        "没有哪一型比较好，哪一型比较差的绝对价值观，事实上，每一型的人都各有其优缺点。",
    ]),
    references: None,
    warning: Some("此测试未被心理学界广泛认可，不能正确反映出测试者的人格情况。"),
    formula_mode: None,
    tags: Tag{ info: Some(&[ "人格"]), normal: Some(&["自评"]), warning: None, error: None },
    interpretation: Interpretation { 
        dialog: &[
            "人格最健康的时候，随时有人格整合的可能，例如：第 9 型的出现了第 3 型的特征，由原本的内向保守，变得充满活力，基本欲望得到满足，基本恐惧隐藏。健康的人格令人活出真我，心理平衡、充分发挥自己的潜能和能力对社会作出贡献。",
            "一般的时候基本欲望和基本恐惧浮现，由自我取代了真我，自我过分膨胀，自我防卫机制出现，心理变得不平衡，容易与人发生冲突，为了满足基本欲望，可以不惜一切伤害人，也很容易屈服在社会的阴影下，人格的优点未能充分发挥。不健康的人格，自我防卫机制失灵，可以导致人格陷落，如第五型人出现了第七型的缺点如失控、信口开河和生活奢侈等。严重的会导致精神病，甚至自毁。当人格于极健康和不健康的时候，是有整合和陷落的现象，这会导致错误地判断人的基本人格，尤其是极健康的时候。",
        ],
        type_interpretations:  &[
            TypeInterpretation {
                r#type: Type::Reformer,
                desire_trait: "追求不断进步",
                basic_confusion: "我若不完美，就没有人会爱我。",
                main_features: "原则性、不易妥协、常说“应该”及“不应该”、黑白分明、对自己和别人要求甚高、追求完美、不断改进、感情世界薄弱；希望把每件事都做得尽善尽美，希望自己或是这个世界都更进步。时时刻刻反省自己是否犯错，也会纠正别人的错。",
                main_traits: "忍耐、有毅力、守承诺、贯彻始终、爱家顾家、守法、有影响力的领袖、喜欢控制、光明磊落。",
                lifestyle: "爱劝勉教导，逃避表达忿怒，相信自己每天有干不完的事。",
                relationships: "你是典型的完美主义者，显浅易明。正因为你事事追求完美，你很少讲出称赞的说话，很多时只有批评，无论是对自己，或是对身边的人也是！又因为你对自己的超超高标准，你给自己很大压力，会很难放松自己去尽情的玩、开心的笑！",
                characters: &[
                    Character {
                        keyword: "愤怒、不满",
                        description: "属于第一型的你，相信常常这感觉，对吧？你们常有愤怒、不满的感觉都是源自你们超高的生活要求。当遇到什么不顺意时，就很容易感到忿怒、不满，觉得事情不应该这样发生……这种情绪不单是对自己，还有对周围的环境和人，都是一样，因为你对他们一样带有超高的要求。但要注意，作为你的朋友，要承受你的嬲怒情绪，的确不是容易，也会造成压力，所以要多加注意啊！",
                    },
                    Character {
                        keyword: "失望、沮丧",
                        description: "同样因为你们事事追求完美的态度，让你们在生活里常常感到碰钉子、不如意。除了是对外发泄愤怒情绪，其实在内心不断经历挫败，不断经历失望。这些情绪对你们并不健康，必须积极处理。最根源的方法不是让自己做得更出色，而是调节对每事每情的看法，轻松面对！ 其他类型上面说的有的 事正确的",
                    },
                ],
                basic_fear: "怕自己做错，变坏，被腐败",
                basic_desire: "希望自己是对的，好的，贞洁的，有诚信的",
            },
            TypeInterpretation {
                r#type: Type::Helper,
                desire_trait: "追求服待",
                basic_confusion: "我若不帮助人，就没有人会爱我。",
                main_features: "渴望别人的爱或良好关系、甘愿迁就他人、以人为本、要别人觉得需要自己、常忽略自己 ；很在意别人的感情和需要，十分热心，愿意付出爱给别人，看到别人满足地接受他们的爱，才会觉得自己活得有价值。",
                main_traits: "温和友善、随和 、绝不直接表达需要，婉转含蓄 、好好先生/小姐 、慷慨大方 、乐善好施",
                lifestyle: "爱报告事实，逃避被帮助，忙于助人，否认问题存在。",
                relationships: "助人型（Helper）顾名思义，你很喜欢帮人，而且主动，慷慨大方！虽然你对别人的需要很敏锐，但却很多时忽略了自己的需要。在你来说，满足别人的需要比满足自己的需要更重要，所以你很少向人提出请求。这样说来，你的自我并不强，很多时候要靠帮助别人来肯定自己。",
                characters: &[
                    Character {
                        keyword: "自豪、骄傲",
                        description: "第二型的你，是否觉得这个形容很不贴切？觉得很惊奇？其实，一向表现得助人为快乐之本的你，是通过热心帮助人去肯定自己，要朋友接纳欣赏自己。所以当有朋友找你们帮助，你自是开心不已，也会有自豪和骄傲之感，因为在过程中你得到肯定和满足。",
                    },
                    Character {
                        keyword: "占有、控制",
                        description: "正因为帮人得到这么多的满足，你们很想继续这样的关下去，这个很正常！可是，当你们「投资」下越多时间和心力，你希望得到的回报更多。很有可能，你们会很希望朋友会很「attach」（依赖 依靠 仰赖附和 迷恋）你，甚至是只「attach」你一个，事事对你说，跟你分享。这便反映在你内心的占有欲，若然朋友非这样对你，便会很失望，觉得他们背叛了你。甚或，你可能会对他们施加压力，以控制他们。这里当然不是说每个第二型都是这样子，但当我们状态不佳，心情不太好时，的确有机会出现以上倾向。多点留意自己的情绪反应，有助控制及改善！",
                    },
                ],
                basic_fear: "不被爱，不被需要",
                basic_desire: "感受爱的存在",
            },
            TypeInterpretation {
                r#type: Type::Achiever,
                desire_trait: "追求成果",
                basic_confusion: "我若没有成就，就没有人会爱我。",
                main_features: "强烈好胜心，喜欢认威，常与别人比较，以成就衡量自己的价值高低，着重形象，工作狂，惧怕表达内心感受 ；希望能够得到大家的肯定。是个野心家，不断地追求有效，希望与众不同，受到别人的注目、羡慕，成为众人的焦点。",
                main_traits: "自信、活力充沛 、风趣幽默 、满有把握 、处世圆滑 、积极进取 、美丽形象",
                lifestyle: "爱数说自己成就，逃避失败，按着长远目标过活。",
                relationships: "成就型（Achiever）的你精力充沛，总是动力过人，因为你有很强的争胜欲望！你喜欢接受挑战，会把你自己的价值与成就连成一线。成就型的你会全心全意去追求一个目标，因为你相信「天下没有不可能的事」。动力十足的你，最好做leader带领其他人啦！",
                characters: &[
                    Character {
                        keyword: "自恋、炫耀",
                        description: "第三型的人倾向看自己成为颇大、颇重要的，所以有一点点的自恋、自我膨胀。所以你们都会把自己最好的一面给友人看，甚至极端时，会在朋友面前撒谎，以求「保持」自己在朋友心目中的形象。很多时，第三型真正的实力往往没有那么强，因为他们的表达实有一点点夸张。",
                    },
                    Character {
                        keyword: "害怕亲密",
                        description: "第三型的你很害怕亲密关系，不是说你们会没有朋友，只是当关系进深的时候，你可能会因怕真面目被看见而避开、逃掉。所以，亲密/好朋友关系对第三型说并不容易建立，因为他们害怕被人看见自己的真面目，也因此很难开放自己与人坦诚交往。第三型的你好胜心颇强，通常认为自己不能在朋友面前「认衰」，所以会表现得「很棒很棒」的，但世界上没有一个人是十全十美的完人啊！当能容许自己以真面目视人，你的生活将很快乐！",
                    },
                ],
                basic_fear: "没有成就，一事无成",
                basic_desire: "感觉有价值，被接受",
            },
            TypeInterpretation {
                r#type: Type::Individualist,
                desire_trait: "追求独特",
                basic_confusion: "我若不是独特的，就没有人会爱我。",
                main_features: "情绪化，追求浪漫，惧怕被人拒绝，觉得别人不明白自己， 占有欲强，我行我素生活风格：爱讲不开心的事，易忧郁、妒忌，生活追寻感觉好；很珍惜自己的爱和情感，所以想好好地滋养它们，并用最美、最特殊的方式来表达。他们想创造出独一无二、与众不同的形象和作品，所以不停地自我察觉、自我反省，以及自我探索。",
                main_traits: "易受情绪影响、倾向追求不寻常、艺术性而富有意义的事物 、多幻想，认为死亡、苦难、悲剧才是极具价值和真实的生命 、对美感的敏锐可见于独特的衣着，及对布置环境的品味显出他的独特性、极具创造力、过分情绪化、容易沮丧或消沉 、常觉生命是一个悲剧 、对人若即若离，怕亲密的关系令人发现自己不完美就会离他而去。",
                lifestyle: "爱讲不开心的事，易忧郁、妒忌，生活追寻感觉好。",
                relationships: "自我型（Individualist）：曾否有人跟你说，你有艺术家的脾气？这个自我型就正正是艺术家的性格-多愁善感及想像力丰富，会常沉醉于自己的想像世界里。另一方面，由于你是感情主导的人，有些工作你不喜欢就可能不会做，不会考虑责任的问题。",
                characters: &[
                    Character {
                        keyword: "嫉妒、比较",
                        description: "自我型的你们其实都有点「艺术家脾气」，对吧！自怜、觉得自己与其他人不一样、喜欢沉醉于自己的想象世界……很多时，第四型的表现会比较抽离，都是因为跟身边人比较，觉得自己不同，其他人不会明白，又觉得其他人都拥有很多你们没有的东西，所以在现实的社交圈子里很难得到满足。",
                    },
                    Character {
                        keyword: "自我沉醉、自怜",
                        description: "由于从现实生活中得不到满足，自我型的朋友都会在幻想里建构自己的世界，制造一些moody（adj. 抑郁的 感伤的；情绪多变的 喜怒无常的）的环境，好让自己的情绪得以发泄出来。不过，这样一来，自我型的人都显得比较情绪化，令其他人更不能明白你们，更孤立起来。所以你们要小心，不要让自己过分脱节啊！",
                    },
                ],
                basic_fear: "没有独特的自我感受或存在意义",
                basic_desire: "寻找自我，在内在经验中找到自我认同",
            },
            TypeInterpretation {
                r#type: Type::Investigator,
                desire_trait: "追求知识",
                basic_confusion: "我若没有知识，就没有人会爱我。",
                main_features: "冷眼看世界，抽离情感,喜欢思考分析，要知很多，但缺乏行动，对物质生活要求不高，喜欢精神生活，不善表达内心感受 ；想藉由获取更多的知识，来了解环境，面对周遭的事物。他们想找出事情的脉络与原理，做为行动的准则。有了知识，他们才敢行动，也才会有安全感。",
                main_traits: "温文儒雅 、有学问 、条理分明 、表达含蓄 、拙於词令 、沉默内向 、冷漠疏离 、欠缺活力 、反应缓慢",
                lifestyle: "爱观察、批评，把自己抽离，每天有看不完的书。",
                relationships: "理智型（Thinker）的你是个很冷静的人，总想跟身边的人和事保持一段距离，也不会让情绪左右。很多时候，你都会先做旁观者，后才投入参与。另外，你也需要充分的私人空间和高度的私隐，否则你会觉得很焦虑，不安定！你也很有机会成为专家，例如电脑啦，漫画啦，时装啦，因为你对知识是非常热爱的！",
                characters: &[
                    Character {
                        keyword: "好辨、抽离",
                        description: "思想型的人常常观察身边的事，却很少参与，所以感情投入也很少。还有，他们好辨，很执着，却少有「辨输」的空间和量度。对知识的执着固然重要，但经验生活中所得的体会也非常可贵，希望你们取得平衡，得到最多！",
                    },
                ],
                basic_fear: "无能，无知",
                basic_desire: "能干，知识丰富",
            },
            TypeInterpretation {
                r#type: Type::Loyalist,
                desire_trait: "追求忠心",
                basic_confusion: "我若不顺从，就没有人会爱我。",
                main_features: "做事小心谨慎，不轻易相信别人，多疑虑,喜欢群体生活，为别人做事尽心尽力，不喜欢受人注视，安于现状，不喜转换新环境；相信权威、跟随权威的引导行事，然而另一方面又容易反权威，性格充满矛盾。他们的团体意识很强，需要亲密感，需要被喜爱、被接纳并得到安全的保障。",
                main_traits: "忠诚 、警觉 、谨慎 、机智 、务实 、守规 、纪律维持者。",
                lifestyle: "爱平和讨论，惧怕权威，传统可给予安全感，害怕成就、逃避问题。",
                relationships: "忠诚型（Loyalist）的你会是一个很好员工，因为你很忠心尽责。安全感对你都很重要，因为当遇到新的人和事，都会令你产生恐惧、不安的感觉。基于这种恐惧不安，凡事你都会作最坏打算，换句话说，你为人都比较悲观，也较易去逃避了事。",
                characters: &[
                    Character {
                        keyword: "害怕、忧虑、犹豫",
                        description: "忠诚型的你们表现得忠诚，是因为你们害怕，对很多事情皆忧虑，很多时都向坏处打算，所以做人很谨慎。同一原因，由于害怕做错决定，所以当面对抉择的时候，你们大都显得很犹疑，心大心细。适当的忧虑能保护我们，但若过份忧虑则会阻碍我们前行！留意留意！",
                    },
                ],
                basic_fear: "得不到支援及指引，单凭一己的能力不能兼顾全部",
                basic_desire: "得到支援及安全感",
            },
            TypeInterpretation {
                r#type: Type::Enthusiast,
                desire_trait: "追求快乐",
                basic_confusion: "我若不带来欢乐，就没有人会爱我。",
                main_features: "乐观,要新鲜感，追上潮流，不喜承受压力，怕负面情绪；想过愉快的生活，想创新、自娱娱人，渴望过比较享受的生活，把人间的不美好化为乌有。他们喜欢投入经验快乐及情绪高昂的世界，所以他们总是不断地寻找快乐、经验快乐。",
                main_traits: "快乐热心 、不停活动 、不停获取 、怕严肃认真的事情 、多才多艺 、对玩乐的事非常熟悉亦会花精力钻研 、不惜任何代价只要快乐 、嬉笑怒骂的方式对人对事。",
                lifestyle: "爱讲自己经验，喜欢制造开心，人生有太多开心的事情等着他。",
                relationships: "活跃型（Adventurer）的你，就是如此这般：乐观、精力充沛、迷人、好动、贪新鲜、五时花六时变……「最近要玩得开心」就是你的生活哲学！你们很需要生活有新鲜感，所以很不喜欢被束缚、被控制。你的活力是玩的活力，又跟第三型的成就型又有所不同，相信你们是活动搅手，玩极唔厌！",
                characters: &[
                    Character {
                        keyword: "不耐烦、冲动、上瘾",
                        description: "好玩、享乐主义行头的活跃型，做事欠缺耐性，因为你们都很怕闷。不耐烦之余，也很易冲动行事，因第七型的朋友做事鲜有周详计划，很讲即庆，想做就去做！但你们必须要小心，就算遇上一种玩意、兴趣你十分喜欢，也得学习不要沉迷下去！始终要顾及自己的身体及其他事情啊！(我这样说是因为第七型的人比其他型的人更上烟瘾、毒瘾、赌瘾、或者打机瘾等等，小心！)",
                    },
                ],
                basic_fear: "被剥削，被困于痛苦中",
                basic_desire: "追求快乐、满足、得偿如愿",
            },
            TypeInterpretation {
                r#type: Type::Challenger,
                desire_trait: "追求权力",
                basic_confusion: "我若没有权力，就没有人会爱我。",
                main_features: "追求权力，讲求实力，不靠他人，有正义感,要话事，喜欢做大事；是绝对的行动派，一碰到问题便马上采取行动去解决。想要独立自主，一切靠自己，依照自己的能力做事，要建设前不惜先破坏，想带领大家走向公平、正义。",
                main_traits: "具攻击性、自我中心 、轻视懦弱、尊重强人 、为受压迫者挺身而出 、冲动 、有什麼不满意即场发作 、主观 、直觉。",
                lifestyle: "爱命令，说话大声、有威严，报复心理、爱辩论，靠意志来掌管生活。",
                relationships: "领袖型（Leader）：很多领袖都有以下特质：豪爽、不拘小节、自视甚高、遇强越强、关心正义、公平。你们清楚自己的目标，并努力前进。由于不愿被人控制，且具有一定的支配力，所以你们很有潜质做领袖带领大家。由于你们都较好胜，有时候会对人有点攻击性，让人感到压力。",
                characters: &[
                    Character {
                        keyword: "侵略、挑战、反叛",
                        description: "第八型的你通常身兼领袖身份，可以有权力全权安排，也可指挥他人。由于你们的动力较强，有时会予人侵略之感，而这个也是你本身的动力源头，你很有争胜及控制的欲望，但却要小心运用，不要用之伤害别人！此外，你专向难度及规范挑战，就是「明知山有虎，偏向虎山行」的任性。所以很可能，妈妈叫你不要做的东西，你偏不听；老师要你学的，你偏扮傻……你会是这样子吗？要是真的话，会是于你有益吗？",
                    },
                ],
                basic_fear: "被认为软弱、被人伤害、控制、侵犯",
                basic_desire: "自己决定生命的路向，捍卫本身的利益，做强者",
            },
            TypeInterpretation {
                r#type: Type::Peacemaker,
                desire_trait: "追求和平",
                basic_confusion: "我若不和善，就没有人会爱我。",
                main_features: "须花长时间作决定，难于拒绝他人，不懂宣泄愤怒；显得十分温和，不喜欢与人起冲突，不自夸、不爱出风头，个性淡薄。想要和人和谐相处，避开所有的冲突与紧张，希望事物能维持美好的现状。忽视会让自己不愉快的事物，并尽可能让自己保持平稳、平静。",
                main_traits: "温和友善 、忍耐、随和 、怕竞争 、无法集中注意力，有时像梦游 、不到最后一分钟不会完工 、非常倚赖别人的提醒 、注意力集中在细节、次要的事 、对大多数事物没有多大的兴趣 、不喜欢被人支配 、绝不直接表达不满，只是阳奉阴违。",
                lifestyle: "爱调和，做事缓慢，易懒惰、抑压，生活追寻舒服。",
                relationships: "和平型（Peacemaker）：在很多情况，你们都是和平使者，善解人意，随和。你们很容易了解别人，却不是太清楚自己想要什么，会显得优柔寡断。相对地说，你们的主见会比较少，宁愿配合其他人的安排，做一个很好的支持者，所以你是比较被动的。",
                characters: &[
                    Character {
                        keyword: "怕羞、怕事、懒惰",
                        description: "和平型的你与世无争，渴望人人能和平共处，很怕引起冲突，是不显眼的一个。由于从不试图突出自己，你们会比较怕羞、怕事，也很容易有躲懒的意欲，因为你喜爱和平，不喜爱辛劳，所以你也不会PUSH自己！若你心想干一番大事，则要好好鞭策自己啦！",
                    },
                ],
                basic_fear: "转变与压力，失去平衡",
                basic_desire: "维系内在的平静及安稳",
            },
        ],
    },
    questions: &[
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
    ]
};
