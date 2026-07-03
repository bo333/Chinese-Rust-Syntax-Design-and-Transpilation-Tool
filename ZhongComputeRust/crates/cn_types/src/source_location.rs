use serde::{Deserialize, Serialize};

/// 源码位置：记录中文源代码中的具体位置
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file_path: String,
    pub line: u32,
    pub column: u32,
}

/// 源码范围：记录中文源代码中的一段区域
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceRange {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

/// Rust代码位置：记录生成Rust代码中的位置
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RustLocation {
    pub line: u32,
    pub column: u32,
}