//! 量表领域模型与注册表。
//!
//! 类型按职责拆分到各自模块，`catalog` 是读取量表的唯一入口。

mod catalog;
mod category;
mod data;
mod definition;
mod interpretation;
mod question;
mod scoring;
mod summary;
mod text;

pub use catalog::{all_names, json, name, summaries};
pub use category::ScaleCategory;
pub use definition::{Scale, Tag};
pub use interpretation::{
    Characteristic, ComfortingWord, CriticalWarning, InterpretationItem, Status, SymptomGuidance,
};
pub use question::{Question, QuestionOption};
pub use scoring::{FormulaMode, Integer, OperationalRule};
pub use summary::ScaleId;
pub use text::{HTMLElement, PlainText, PlainTexts, SentenceItem, Texts};
