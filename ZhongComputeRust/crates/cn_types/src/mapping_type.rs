use serde::{Deserialize, Serialize};

/// 映射对照表条目的映射类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MappingType {
    /// 关键词映射
    Keyword,
    /// 类型名映射
    TypeName,
    /// 语法糖映射
    Sugar,
    /// 控制流映射
    ControlFlow,
    /// 其他映射
    Other,
}