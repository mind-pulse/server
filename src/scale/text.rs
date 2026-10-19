use serde::Serialize;

/// 量表文案中的纯文本（编译期常量）
pub type PlainText = &'static str;

/// 一组纯文本
pub type PlainTexts = &'static [PlainText];

/// 文案中可用于强调的 HTML 元素
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "text", rename_all = "UPPERCASE")]
pub enum HTMLElement {
    Strong(PlainText),
}

/// 组成一句话的最小单元：纯文本或 HTML 元素
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SentenceItem {
    Plain(PlainText),
    HTMLElement(HTMLElement),
}

/// 一句话
pub type Sentence = &'static [SentenceItem];

/// 一段文案（由多句话组成）
pub type Texts = &'static [Sentence];
