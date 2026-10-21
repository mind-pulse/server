use serde::Serialize;

use crate::scale::category::ScaleCategory;
use crate::scale::text::{PlainText, PlainTexts, Texts};

/// 量表标签
#[derive(Debug, Serialize)]
pub struct Tag {
    pub info: Option<PlainTexts>,
    pub normal: Option<PlainTexts>,
    pub warning: Option<PlainTexts>,
    pub error: Option<PlainTexts>,
}

/// 一份量表的完整定义
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
    /// 理念
    pub idea: Option<PlainTexts>,
    /// 标签
    pub tags: &'r Tag,
}
