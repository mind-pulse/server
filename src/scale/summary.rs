use serde::Serialize;

use crate::scale::category::ScaleCategory;
use crate::scale::definition::Tag;
use crate::scale::text::{PlainText, Texts};

/// 量表 ID 的强类型包装
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScaleId(pub u16);

const SECONDS_PER_QUESTION_MIN: u32 = 10;
const SECONDS_PER_QUESTION_MAX: u32 = 15;

/// 计算预计测试时长（分钟）
///
/// 每个问题耗时 10-15 秒为标准计算
pub const fn estimate_duration(total_questions: u32) -> [u32; 2] {
    let min = total_questions * SECONDS_PER_QUESTION_MIN / 60;
    let max = total_questions * SECONDS_PER_QUESTION_MAX / 60;
    // 确保至少显示 1 分钟，避免出现 [0, 0]
    if max < 1 { [1, 2] } else { [min, max] }
}

/// 列表页使用的量表概览
#[derive(Debug, Serialize)]
pub struct ScaleSummary<'r> {
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
