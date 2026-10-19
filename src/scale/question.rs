use serde::Serialize;

use crate::scale::text::PlainText;

/// 单个可选项
#[derive(Debug, Serialize, Clone)]
pub struct QuestionOption {
    pub text: PlainText,
    pub point: i8,
}

/// 通用题目
#[derive(Debug, Serialize)]
pub struct Question {
    pub title: PlainText,
    pub is_multiple: bool,
    pub options: &'static [QuestionOption],
}
