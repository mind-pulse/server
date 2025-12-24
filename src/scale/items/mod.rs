// 量表模块命名尽量使用全称，避免缩写名称重复。
// 缩写严格使用官方命名。
// 前端路由可以使用自定义缩写（比如添加前缀以区别相同缩写的量表）以避免冲突。

mod beck_depression_inventory;
mod cattell_sixteen_personality_factors;
mod enneagram_personality_test;
mod eysenck_personality_questionnaire;
mod hamilton_depression_scale;
mod holland_occupational_interest;
mod holland_occupational_interest_highschool_cn;
mod neo_personality_inventory_revised;
mod self_rating_anxiety_scale;
mod self_rating_depression_scale;
mod symptom_checklist_90;
mod yale_brown_obsessive_compulsive_scale;

pub use self::{
    beck_depression_inventory::BECK_DEPRESSION_INVENTORY,
    cattell_sixteen_personality_factors::SIXTEEN_PERSONALITY_FACTORS,
    enneagram_personality_test::ENNEAGRAM_PERSONALITY_TEST,
    eysenck_personality_questionnaire::EYSENCK_PERSONALITY_QUESTIONNAIRE_REVISED_SHORT_SCALE,
    hamilton_depression_scale::HAMILTON_DEPRESSION_SCALE,
    holland_occupational_interest::HOLLAND_OCCUPATIONAL_INTEREST,
    holland_occupational_interest_highschool_cn::HOLLAND_OCCUPATIONAL_INTEREST_HIGH_SCHOOL_CN,
    neo_personality_inventory_revised::NEO_PERSONALITY_INVENTORY_REVISED,
    self_rating_anxiety_scale::SELF_RATING_ANXIETY_SCALE,
    self_rating_depression_scale::SELF_RATING_DEPRESSION_SCALE,
    symptom_checklist_90::SYMPTOM_CHECKLIST_90,
    yale_brown_obsessive_compulsive_scale::YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE,
};
