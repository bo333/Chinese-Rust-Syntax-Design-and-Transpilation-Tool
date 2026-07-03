use serde::{Deserialize, Serialize};

/// 关键词映射：记录单个中文关键词到Rust关键词的映射关系
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeywordMapping {
    /// 中文关键词（1-4个汉字）
    pub cn_keyword: String,
    /// Rust关键词
    pub rust_keyword: String,
    /// 语义说明（不超过50字）
    pub semantic_description: String,
    /// 是否为保留关键词（禁止用作标识符）
    pub is_reserved: bool,
    /// 是否为内置强制规范（不可变更）
    pub is_builtin: bool,
    /// 映射条目所属版本号（格式"主版本.次版本"）
    pub version: String,
}

/// 类型映射：记录中文类型名到Rust类型表达式的映射关系
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeMapping {
    /// 中文类型名（2-6个汉字）
    pub cn_type_name: String,
    /// Rust类型表达式
    pub rust_type_expr: String,
    /// 是否为基础确定性映射（如"整数"→i32为基础映射，"整数64"→i64为显式映射）
    pub is_base_mapping: bool,
    /// 泛型参数映射
    pub generic_params: Vec<GenericParamMapping>,
    /// 约束条件
    pub constraints: Vec<String>,
}

/// 泛型参数映射
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericParamMapping {
    /// 中文参数名
    pub cn_param_name: String,
    /// Rust参数名
    pub rust_param_name: String,
    /// 约束条件
    pub constraint: Option<String>,
}

/// 所有权语法映射
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipMapping {
    /// 中文语法
    pub cn_syntax: String,
    /// Rust语法
    pub rust_syntax: String,
    /// 语义说明
    pub semantic_description: String,
}