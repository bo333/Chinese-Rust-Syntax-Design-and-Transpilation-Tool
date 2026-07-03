use cn_types::mapping_registry::MappingRegistry;

/// 版本管理器
#[derive(Debug, Clone)]
pub struct VersionManager {
    current_version: String,
}

impl VersionManager {
    pub fn new(version: &str) -> Self {
        Self {
            current_version: version.to_string(),
        }
    }

    /// 获取当前版本号
    pub fn current_version(&self) -> &str {
        &self.current_version
    }

    /// 检查版本兼容性
    pub fn check_compatibility(&self, source_version: &str) -> VersionCompatibility {
        let current_major = self.parse_major(&self.current_version);
        let source_major = self.parse_major(source_version);

        if current_major != source_major {
            VersionCompatibility::Incompatible {
                expected: self.current_version.clone(),
                actual: source_version.to_string(),
            }
        } else {
            VersionCompatibility::Compatible
        }
    }

    fn parse_major(&self, version: &str) -> u32 {
        version
            .split('.')
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }
}

/// 版本兼容性检查结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionCompatibility {
    Compatible,
    Incompatible { expected: String, actual: String },
}