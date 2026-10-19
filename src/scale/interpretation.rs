use serde::Serialize;

use crate::scale::text::{PlainText, PlainTexts};

/// 症状对应的自助建议
#[derive(Debug, Serialize)]
pub struct SymptomGuidance {
    /// 当该因子阳性时，可以告诉用户“你可以尝试做的事”
    pub self_help: PlainTexts,
    /// 需要提醒用户注意的信号（但不等于诊断）
    pub watch_out: PlainTexts,
    /// 何时建议寻求专业帮助（非常重要）
    pub seek_help_when: PlainTexts,
}

/// 安慰语
#[derive(Debug, Serialize)]
pub struct ComfortingWord {
    /// 安慰语
    pub text: PlainText,
    /// 强调语，存储警告信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caution: Option<PlainText>,
}

/// 极度危险提醒
#[derive(Debug, Serialize)]
pub struct CriticalWarning {
    pub title: PlainText,
    pub content: PlainText,
}

/// 分数区间的解释结果
#[derive(Debug, Serialize)]
pub struct InterpretationItem<I>
where
    I: PartialOrd + Serialize + Copy,
{
    /// 范围，左开右闭
    pub range: [I; 2],
    /// 描述
    pub description: PlainText,
    /// 极度危险提醒
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_warning: Option<CriticalWarning>,
    /// 安慰语
    pub comforting_word: ComfortingWord,
    /// 建议
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advice: Option<PlainTexts>,
    /// 症状
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symptom: Option<PlainTexts>,
    /// 状态
    pub status: Status,
}

/// 分数区间的严重程度
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum Status {
    /// 正常
    Normal,
    /// 轻度
    Mild,
    /// 中度
    Moderate,
    /// 重度
    Major,
}

/// 人格特征的两端描述
#[derive(Debug, Serialize)]
pub struct Characteristic {
    pub low: PlainTexts,
    pub high: PlainTexts,
}
