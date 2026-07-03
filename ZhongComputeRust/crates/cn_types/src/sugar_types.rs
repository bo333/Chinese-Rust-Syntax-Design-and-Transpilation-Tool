use serde::{Deserialize, Serialize};

/// 语法糖规则状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SugarRuleStatus {
    /// 草案
    Draft,
    /// 实验
    Experimental,
    /// 稳定（仅稳定状态的规则参与转译）
    Stable,
    /// 废弃
    Deprecated,
}

/// 语法糖匹配模式类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatternType {
    /// 顺序匹配
    Sequence,
    /// 选择匹配
    Alternative,
    /// 重复匹配
    Repetition,
}

/// 模式元素类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatternElementType {
    /// 字面量
    Literal,
    /// 关键词
    Keyword,
    /// 标识符
    Identifier,
    /// 捕获组
    Capture,
}

/// 歧义消解策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisambiguationStrategy {
    /// 最长匹配
    LongestMatch,
    /// 显式标注
    ExplicitAnnotation,
    /// 消解失败
    Fail,
}

/// 等价性验证层级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerificationLevel {
    /// 文本等价
    TextEquivalence,
    /// 编译等价
    CompileEquivalence,
    /// 语义等价
    SemanticEquivalence,
}