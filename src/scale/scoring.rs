use serde::Serialize;

/// 分数运算规则
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum OperationalRule {
    Multiply(f64),
}

/// 取整规则
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "UPPERCASE"))]
pub enum Integer {
    /// 四舍五入（保留供其他量表使用）
    #[allow(dead_code)]
    Round,
    /// 取整数部分（向下取整）
    Floor,
}

/// 前端计算用到的公式
#[derive(Debug, Serialize)]
pub struct FormulaMode {
    pub operational_rule: OperationalRule,
    pub integer: Option<Integer>,
}
