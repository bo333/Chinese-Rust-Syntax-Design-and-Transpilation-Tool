use serde::{Deserialize, Serialize};

use crate::error_code::ErrorCode;
use crate::source_location::{SourceLocation, SourceRange};

/// 诊断信息严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    /// 错误：阻止转译
    Error,
    /// 警告：可继续转译
    Warning,
    /// 提示：信息性
    Hint,
}

/// 修复建议
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixSuggestion {
    /// 修复建议的中文描述
    pub message: String,
    /// 替换范围
    pub replacement_range: Option<SourceRange>,
    /// 替换文本
    pub replacement_text: Option<String>,
}

/// 诊断信息：记录转译过程中的错误、警告和提示
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// 严重程度
    pub severity: DiagnosticSeverity,
    /// 错误码（CNxxxx格式，通过ErrorCode值对象校验）
    pub error_code: ErrorCode,
    /// 中文源码位置
    pub cn_location: SourceLocation,
    /// 中文错误描述
    pub cn_message: String,
    /// 修复建议列表
    pub fix_suggestions: Vec<FixSuggestion>,
    /// 原始Rust编译器错误（仅在Rust编译错误回映射时存在）
    pub original_rust_error: Option<String>,
}