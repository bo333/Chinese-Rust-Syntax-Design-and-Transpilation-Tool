use serde::{Deserialize, Serialize};

use crate::mapping_type::MappingType;
use crate::pipeline_types::AstNodeId;

/// 映射对照表
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMap {
    /// 映射条目列表
    pub entries: Vec<SourceMapEntry>,
    /// 中文源文件路径
    pub cn_file_path: String,
    /// Rust代码文件路径
    pub rust_file_path: String,
}

/// 映射对照表条目
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMapEntry {
    /// 中文源码行号
    pub cn_line: u32,
    /// 中文源码列号
    pub cn_column: u32,
    /// Rust代码行号
    pub rust_line: u32,
    /// Rust代码列号
    pub rust_column: u32,
    /// 映射类型
    pub mapping_type: MappingType,
    /// 是否为确定性映射（所有合法映射此值必须为true）
    pub is_deterministic: bool,
}

/// 映射记录：记录CN-AST到Rust-AST的映射关系
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingRecord {
    /// 中文AST节点ID
    pub cn_node_id: AstNodeId,
    /// Rust AST节点ID
    pub rust_node_id: AstNodeId,
    /// 映射规则标识
    pub mapping_rule_id: String,
    /// 是否为确定性映射
    pub is_deterministic: bool,
}