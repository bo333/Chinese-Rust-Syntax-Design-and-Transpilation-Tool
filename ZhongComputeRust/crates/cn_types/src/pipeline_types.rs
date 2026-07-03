use serde::{Deserialize, Serialize};

use crate::ast_node_type::{CnAstNodeType, RustAstNodeType};
use crate::mapping_models::KeywordMapping;
use crate::source_location::{RustLocation, SourceLocation};
use crate::token_type::CnTokenType;

/// 词法分析Token
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    /// Token类型
    pub token_type: CnTokenType,
    /// 文本内容
    pub text: String,
    /// 源码位置
    pub location: SourceLocation,
    /// 关键词映射信息（仅关键词Token存在）
    pub keyword_mapping: Option<KeywordMapping>,
}

/// 显式消歧语义标注
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticAnnotation {
    /// 显式类型标注
    pub explicit_type: Option<String>,
    /// 显式生命周期标注
    pub explicit_lifetime: Option<String>,
    /// 消歧提示
    pub disambiguation_hint: Option<String>,
}

/// AST节点唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AstNodeId(pub u64);

/// 中文语法树
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CnAst {
    /// 根节点
    pub root: CnAstNode,
}

/// 中文语法树节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CnAstNode {
    /// 节点类型
    pub node_type: CnAstNodeType,
    /// 子节点
    pub children: Vec<CnAstNode>,
    /// 源码位置
    pub location: SourceLocation,
    /// 匹配的语法糖规则标识
    pub sugar_rule_id: Option<String>,
    /// 语义标注（显式消歧信息）
    pub semantic_annotation: Option<SemanticAnnotation>,
    /// 节点指纹（用于增量转译）
    pub fingerprint: Option<Fingerprint>,
    /// 节点唯一标识
    pub id: AstNodeId,
}

/// Rust语法树
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RustAst {
    /// 根节点
    pub root: RustAstNode,
}

/// Rust语法树节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RustAstNode {
    /// 节点类型
    pub node_type: RustAstNodeType,
    /// 子节点
    pub children: Vec<RustAstNode>,
    /// Rust代码位置
    pub location: RustLocation,
}

/// AST节点指纹（用于增量转译）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fingerprint {
    /// 内容哈希
    pub hash: u64,
    /// 计算时间戳
    pub computed_at: u64,
}