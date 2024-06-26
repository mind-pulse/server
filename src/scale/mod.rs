mod bdi;
mod epq_rsc;
mod ept;
mod h_sds;
mod hamd;
mod neo_pi_r;
mod sas;
mod scl90;
mod sds;
mod sixteen_pf;
mod y_bocs;

use serde::Serialize;

use crate::error::{MindPulseError, MindPulseResult};

pub(super) use self::bdi::BECK_DEPRESSION_RATING_SCALE;
pub(super) use self::epq_rsc::EPQ_RSC;
pub(super) use self::ept::ENNEAGRAM_PERSONALITY_TEST;
pub(super) use self::h_sds::SELF_DIRECTED_SEARCH;
pub(super) use self::hamd::HAMILTON_DEPRESSION_SCALE;
pub(super) use self::neo_pi_r::REVISED_NEOPERSONALITY_INVENTORY;
pub(super) use self::sas::SELF_RATING_ANXIETY_SCALE;
pub(super) use self::scl90::SYMPTOM_CHECKLIST_90;
pub(super) use self::sds::SELF_RATING_DEPRESSION_SCALE;
pub(super) use self::sixteen_pf::SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE;
pub(super) use self::y_bocs::YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE;

type PlainText = &'static str;
type PlainTexts = &'static [PlainText];

#[derive(Debug, Serialize, Clone)]
pub(super) struct QuestionOption {
    text: PlainText,
    point: i8,
}

#[derive(Debug, Serialize)]
pub(super) struct Question {
    title: PlainText,
    options: &'static [QuestionOption],
}

#[derive(Debug, Serialize)]
pub(super) struct InterpretationItem<I> {
    range: [I; 2],
    description: PlainText,
    #[serde(skip_serializing_if = "Option::is_none")]
    advice: Option<PlainTexts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symptom: Option<PlainTexts>,
    status: Status,
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub(super) enum OperationalRule {
    Multiply(f64),
    // Divide(f64),
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "UPPERCASE"))]
pub(super) enum Integer {
    // Floor,
    // Ceil,
    /// 四舍五入
    Round,
}

#[derive(Debug, Serialize)]
pub(super) struct FormulaMode {
    operational_rule: OperationalRule,
    integer: Option<Integer>,
}

#[derive(Debug, Serialize)]
pub(super) struct Scale<'r, I, Q> {
    name: PlainText,
    abbreviation: PlainText,
    /// 对量表的说明
    instruction: Texts,
    questions: &'r [Q],
    interpretation: I,
    /// 简介
    introduction: Texts,
    #[serde(skip_serializing_if = "Option::is_none")]
    references: Option<PlainTexts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning: Option<PlainText>,
    /// js 计算用到的表达式，和用"<SUM>"替代
    #[serde(skip_serializing_if = "Option::is_none")]
    formula_mode: Option<FormulaMode>,
    tags: Tag,
    /// 理念
    idea: Option<PlainTexts>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub(super) enum Status {
    /// 正常
    Normal,
    /// 轻度
    Mild,
    /// 中度
    Moderate,
    /// 重度
    Major,
}

#[derive(Debug, Serialize)]
pub(super) struct Tag {
    info: Option<PlainTexts>,
    normal: Option<PlainTexts>,
    warning: Option<PlainTexts>,
    error: Option<PlainTexts>,
}

/// 特征
#[derive(Debug, Serialize)]
pub(super) struct Characteristic {
    low: PlainTexts,
    high: PlainTexts,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "text", rename_all = "UPPERCASE")]
pub(super) enum HTMLElement {
    Strong(PlainText),
    //Mark(PlainText),
    //A { text: PlainText, href: PlainText },
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub(super) enum SentenceItem {
    Plain(PlainText),
    HTMLElement(HTMLElement),
}

pub(super) type Sentence = &'static [SentenceItem];
pub(super) type Texts = &'static [Sentence];

#[derive(Debug, Serialize)]
pub(super) struct ScalePath {
    name: PlainText,
    path: PlainText,
    introduction: Texts,
    warning: Option<PlainText>,
    tags: Tag,
    disabled: bool,
}

impl ScalePath {
    pub(crate) fn name(&self) -> &str {
        self.name
    }
}

pub(super) const PATHS: [ScalePath; 11] = [
    ScalePath {
        name: SELF_DIRECTED_SEARCH.name,
        path: "h_sds",
        introduction: SELF_DIRECTED_SEARCH.introduction,
        warning: SELF_DIRECTED_SEARCH.warning,
        tags: SELF_DIRECTED_SEARCH.tags,
        disabled: false,
    },
    ScalePath {
        name: REVISED_NEOPERSONALITY_INVENTORY.name,
        path: "neo_pi_r",
        introduction: REVISED_NEOPERSONALITY_INVENTORY.introduction,
        warning: REVISED_NEOPERSONALITY_INVENTORY.warning,
        tags: REVISED_NEOPERSONALITY_INVENTORY.tags,
        disabled: false,
    },
    ScalePath {
        name: SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE.name,
        path: "16pf",
        introduction: SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE.introduction,
        warning: SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE.warning,
        tags: SIXTEEN_PERSONALITY_FACTOR_QUESTIONNAIRE.tags,
        disabled: false,
    },
    ScalePath {
        name: ENNEAGRAM_PERSONALITY_TEST.name,
        path: "ept",
        introduction: ENNEAGRAM_PERSONALITY_TEST.introduction,
        warning: ENNEAGRAM_PERSONALITY_TEST.warning,
        tags: ENNEAGRAM_PERSONALITY_TEST.tags,
        disabled: false,
    },
    ScalePath {
        name: EPQ_RSC.name,
        path: "epq_rsc",
        introduction: EPQ_RSC.introduction,
        warning: EPQ_RSC.warning,
        tags: EPQ_RSC.tags,
        disabled: false,
    },
    ScalePath {
        name: SYMPTOM_CHECKLIST_90.name,
        path: "scl90",
        introduction: SYMPTOM_CHECKLIST_90.introduction,
        warning: SYMPTOM_CHECKLIST_90.warning,
        tags: SYMPTOM_CHECKLIST_90.tags,
        disabled: false,
    },
    ScalePath {
        name: BECK_DEPRESSION_RATING_SCALE.name,
        path: "bdi",
        introduction: BECK_DEPRESSION_RATING_SCALE.introduction,
        warning: BECK_DEPRESSION_RATING_SCALE.warning,
        tags: BECK_DEPRESSION_RATING_SCALE.tags,
        disabled: false,
    },
    ScalePath {
        name: SELF_RATING_DEPRESSION_SCALE.name,
        path: "sds",
        introduction: SELF_RATING_DEPRESSION_SCALE.introduction,
        warning: SELF_RATING_DEPRESSION_SCALE.warning,
        tags: SELF_RATING_DEPRESSION_SCALE.tags,
        disabled: false,
    },
    ScalePath {
        name: SELF_RATING_ANXIETY_SCALE.name,
        path: "sas",
        introduction: SELF_RATING_ANXIETY_SCALE.introduction,
        warning: SELF_RATING_ANXIETY_SCALE.warning,
        tags: SELF_RATING_ANXIETY_SCALE.tags,
        disabled: false,
    },
    ScalePath {
        name: YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE.name,
        path: "y_bocs",
        introduction: YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE.introduction,
        warning: YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE.warning,
        tags: YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE.tags,
        disabled: false,
    },
    ScalePath {
        name: HAMILTON_DEPRESSION_SCALE.name,
        path: "hamd",
        introduction: HAMILTON_DEPRESSION_SCALE.introduction,
        warning: HAMILTON_DEPRESSION_SCALE.warning,
        tags: HAMILTON_DEPRESSION_SCALE.tags,
        disabled: false,
    },
];

pub(super) fn get_scale_index_by_path(path: &str) -> MindPulseResult<usize> {
    match PATHS.iter().position(|sp| sp.path == path) {
        None => {
            error!(message = "scale 无效", scale = path);
            Err(MindPulseError::Response("无效的 scale".to_owned()))
        }
        Some(idx) => Ok(idx),
    }
}
