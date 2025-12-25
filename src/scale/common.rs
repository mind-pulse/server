use serde::Serialize;

use crate::scale::category::ScaleCategory;

pub type PlainText = &'static str;
pub type PlainTexts = &'static [PlainText];

#[derive(Debug, Serialize)]
pub struct SymptomGuidance {
    /// 当该因子阳性时，可以告诉用户“你可以尝试做的事”
    pub self_help: PlainTexts,

    /// 需要提醒用户注意的信号（但不等于诊断）
    pub watch_out: PlainTexts,

    /// 何时建议寻求专业帮助（非常重要）
    pub seek_help_when: PlainTexts,
}

#[derive(Debug, Serialize, Clone)]
pub struct QuestionOption {
    pub text: PlainText,
    pub point: i8,
}

#[derive(Debug, Serialize)]
pub struct Question {
    pub title: PlainText,
    pub is_multiple: bool,
    pub options: &'static [QuestionOption],
}

#[derive(Debug, Serialize)]
pub struct ComfortingWord {
    /// 安慰语
    pub text: PlainText,
    /// 强调语，存储警告信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caution: Option<PlainText>,
}

#[derive(Debug, Serialize)]
pub struct CriticalWarning {
    pub title: PlainText,
    pub content: PlainText,
}

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

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum OperationalRule {
    Multiply(f64),
    // Divide(f64),
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "UPPERCASE"))]
pub enum Integer {
    // Floor,
    // Ceil,
    /// 四舍五入
    Round,
}

#[derive(Debug, Serialize)]
pub struct FormulaMode {
    pub operational_rule: OperationalRule,
    pub integer: Option<Integer>,
}

#[derive(Debug, Serialize)]
pub struct Scale<'r, I, Q> {
    /// 量表唯一标识
    pub id: u16,
    /// 名称
    pub name: PlainText,
    /// 一句话描述
    pub description: PlainText,
    /// 分类
    pub primary_category: ScaleCategory,
    /// 相关分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_categories: Option<&'r [ScaleCategory]>,
    /// 简称
    pub abbreviation: PlainText,

    /// 对量表的说明
    pub instruction: Texts,
    /// 问题列表
    pub questions: &'r [Q],
    /// 解释
    pub interpretation: I,

    /// 简介
    pub introduction: Texts,
    /// 参考文献
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<PlainTexts>,
    /// 警告
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<PlainText>,
    /// js 计算用到的表达式，和用"<SUM>"替代
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula_mode: Option<FormulaMode>,
    /// 理念
    pub idea: Option<PlainTexts>,

    /// 标签
    pub tags: &'r Tag,
}

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

#[derive(Debug, Serialize)]
pub struct Tag {
    pub info: Option<PlainTexts>,
    pub normal: Option<PlainTexts>,
    pub warning: Option<PlainTexts>,
    pub error: Option<PlainTexts>,
}

/// 特征
#[derive(Debug, Serialize)]
pub struct Characteristic {
    pub low: PlainTexts,
    pub high: PlainTexts,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "text", rename_all = "UPPERCASE")]
pub enum HTMLElement {
    Strong(PlainText),
    //Mark(PlainText),
    //A { text: PlainText, href: PlainText },
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SentenceItem {
    Plain(PlainText),
    HTMLElement(HTMLElement),
}

pub type Sentence = &'static [SentenceItem];
pub type Texts = &'static [Sentence];

#[derive(Debug, Serialize)]
pub struct ScaleListItem<'r> {
    pub id: u16,
    /// 名称
    pub name: PlainText,
    /// 预计测试时长（分钟）
    pub duration: [u32; 2],
    /// 问题总数
    pub total_questions: u32,
    /// 分类
    pub primary_category: ScaleCategory,
    /// 相关分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_categories: Option<&'r [ScaleCategory]>,
    /// 一句话描述
    pub description: PlainText,
    /// 简介
    pub introduction: Texts,
    /// 警告
    pub warning: Option<PlainText>,
    /// 标签
    pub tags: &'r Tag,
    /// 是否禁用
    pub disabled: bool,
}

impl ScaleListItem<'_> {
    pub(crate) fn name(&self) -> &str {
        self.name
    }

    pub(crate) fn id(&self) -> u16 {
        self.id
    }
}
