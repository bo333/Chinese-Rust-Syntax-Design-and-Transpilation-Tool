use std::path::Path;

use cn_types::mapping_registry::MappingRegistry;
use thiserror::Error;

/// 配置加载错误类型
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("配置文件不存在: {path}，使用内置默认配置")]
    NotFound { path: String },

    #[error("配置格式错误: {message}")]
    FormatError { message: String },

    #[error("映射规范版本不一致: 期望 {expected}, 实际 {actual}")]
    VersionMismatch { expected: String, actual: String },

    #[error("自定义映射冲突: {message}")]
    MappingConflict { message: String },

    #[error("强制规范缺失: {missing_items:?}")]
    MandatorySpecMissing { missing_items: Vec<String> },

    #[error("强制规范被篡改: {tampered_items:?}")]
    MandatorySpecTampered { tampered_items: Vec<String> },

    #[error("向后兼容性违反: {message}")]
    BackwardIncompatibility { message: String },

    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML解析错误: {0}")]
    TomlError(#[from] toml::de::Error),
}

/// 一致性违反项
#[derive(Debug, Clone)]
pub struct ConsistencyViolation {
    pub violation_type: ViolationType,
    pub description: String,
}

/// 一致性违反类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationType {
    /// 关键词多义映射
    KeywordAmbiguity,
    /// 类型映射不确定
    TypeIndeterminacy,
    /// 语法糖优先级冲突
    SugarPriorityConflict,
}

/// 配置加载器trait
pub trait ConfigLoader {
    /// 从TOML文件加载映射规范配置
    fn load_from_file(config_path: &Path) -> Result<MappingRegistry, ConfigError>;

    /// 从字符串加载映射规范配置
    fn load_from_str(config_content: &str) -> Result<MappingRegistry, ConfigError>;

    /// 构建内置强制规范映射注册表
    fn builtin(version: &str) -> MappingRegistry;

    /// 注册配置变更回调（用于热加载）
    fn register_change_callback(callback: Box<dyn Fn(MappingRegistry)>);

    /// 校验映射规范的一致性和完整性
    fn validate_consistency(registry: &MappingRegistry) -> Result<(), Vec<ConsistencyViolation>>;

    /// 校验内置强制规范完整性
    fn validate_mandatory_spec(registry: &MappingRegistry) -> Result<(), ConfigError>;

    /// 校验向后兼容性
    fn validate_backward_compatibility(
        old_registry: &MappingRegistry,
        new_registry: &MappingRegistry,
    ) -> Result<(), ConfigError>;
}