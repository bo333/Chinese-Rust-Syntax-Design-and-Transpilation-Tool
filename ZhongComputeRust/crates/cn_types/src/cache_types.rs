use serde::{Deserialize, Serialize};

use crate::pipeline_types::Fingerprint;
use crate::source_map::SourceMap;

/// 增量转译缓存键
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CacheKey {
    /// 文件路径
    pub file_path: String,
    /// 映射规范版本
    pub spec_version: String,
}

/// 增量转译缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Token序列
    pub tokens: Vec<String>,
    /// 中文语法树（序列化形式）
    pub cn_ast: String,
    /// Rust语法树（序列化形式）
    pub rust_ast: String,
    /// 映射对照表
    pub source_map: SourceMap,
    /// AST节点指纹映射表
    pub fingerprints: Vec<(u64, Fingerprint)>,
    /// 映射规范版本
    pub spec_version: String,
}

/// 安全策略配置
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityPolicyConfig {
    /// 是否允许源码显式引入的unsafe块
    pub allow_unsafe_from_source: bool,
    /// 是否禁止网络请求
    pub deny_network_access: bool,
    /// 是否仅允许输出文件写入
    pub deny_file_write_except_output: bool,
    /// 转译超时阈值（毫秒）
    pub max_transpile_timeout_ms: u64,
}