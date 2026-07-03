use serde::{Deserialize, Serialize};

/// 中文Token类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CnTokenType {
    /// 关键词
    Keyword,
    /// 标识符
    Identifier,
    /// 字面量
    Literal,
    /// 运算符
    Operator,
    /// 分隔符
    Delimiter,
    /// 注释
    Comment,
}

