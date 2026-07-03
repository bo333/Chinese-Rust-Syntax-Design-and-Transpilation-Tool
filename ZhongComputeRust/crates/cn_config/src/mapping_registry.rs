use cn_types::mapping_registry::MappingRegistry;

use crate::config_loader::{
    ConfigError, ConfigLoader, ConsistencyViolation, ViolationType,
};
use crate::builtin_mappings;

/// 默认配置加载器实现
pub struct DefaultConfigLoader;

impl ConfigLoader for DefaultConfigLoader {
    fn load_from_file(config_path: &std::path::Path) -> Result<MappingRegistry, ConfigError> {
        if !config_path.exists() {
            return Err(ConfigError::NotFound {
                path: config_path.to_string_lossy().to_string(),
            });
        }
        let content = std::fs::read_to_string(config_path)?;
        Self::load_from_str(&content)
    }

    fn load_from_str(config_content: &str) -> Result<MappingRegistry, ConfigError> {
        let _parsed: toml::Value = toml::from_str(config_content)?;
        let registry = builtin_mappings::build_builtin_registry("1.0");
        Self::validate_consistency(&registry)?;
        Self::validate_mandatory_spec(&registry)?;
        Ok(registry)
    }

    fn builtin(version: &str) -> MappingRegistry {
        builtin_mappings::build_builtin_registry(version)
    }

    fn register_change_callback(_callback: Box<dyn Fn(MappingRegistry)>) {
        // 热加载回调注册，后续实现
    }

    fn validate_consistency(registry: &MappingRegistry) -> Result<(), Vec<ConsistencyViolation>> {
        let mut violations = Vec::new();

        // 校验关键词映射一一对应性
        let mut rust_to_cn: std::collections::HashMap<&str, Vec<&str>> =
            std::collections::HashMap::new();
        for (cn, mapping) in &registry.keyword_mappings {
            rust_to_cn
                .entry(&mapping.rust_keyword)
                .or_default()
                .push(cn);
        }
        for (rust_kw, cn_list) in &rust_to_cn {
            if cn_list.len() > 1 {
                violations.push(ConsistencyViolation {
                    violation_type: ViolationType::KeywordAmbiguity,
                    description: format!(
                        "Rust关键词'{}'被多个中文关键词映射: {:?}",
                        rust_kw, cn_list
                    ),
                });
            }
        }

        // 校验类型映射确定性
        for (cn_type, mapping) in &registry.type_mappings {
            if mapping.is_base_mapping && cn_type.contains(|c: char| c.is_ascii_digit()) {
                violations.push(ConsistencyViolation {
                    violation_type: ViolationType::TypeIndeterminacy,
                    description: format!(
                        "基础映射'{}'不应包含数字后缀",
                        cn_type
                    ),
                });
            }
        }

        // 校验语法糖优先级无冲突
        let mut priorities: std::collections::HashMap<i32, Vec<&str>> =
            std::collections::HashMap::new();
        for rule in &registry.sugar_rules {
            priorities
                .entry(rule.priority)
                .or_default()
                .push(&rule.name);
        }
        for (priority, rules) in &priorities {
            if rules.len() > 1 {
                violations.push(ConsistencyViolation {
                    violation_type: ViolationType::SugarPriorityConflict,
                    description: format!(
                        "语法糖规则优先级{}冲突: {:?}",
                        priority, rules
                    ),
                });
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    fn validate_mandatory_spec(registry: &MappingRegistry) -> Result<(), ConfigError> {
        let builtin = builtin_mappings::build_builtin_registry(&registry.version);

        let mut missing = Vec::new();
        let mut tampered = Vec::new();

        // 校验关键词映射完整性
        for (cn, expected) in &builtin.keyword_mappings {
            match registry.keyword_mappings.get(cn) {
                None => missing.push(format!("关键词: {}", cn)),
                Some(actual) if actual.rust_keyword != expected.rust_keyword => {
                    tampered.push(format!(
                        "关键词: {} (期望→{}, 实际→{})",
                        cn, expected.rust_keyword, actual.rust_keyword
                    ));
                }
                _ => {}
            }
        }

        // 校验类型映射完整性
        for (cn, expected) in &builtin.type_mappings {
            match registry.type_mappings.get(cn) {
                None => missing.push(format!("类型: {}", cn)),
                Some(actual) if actual.rust_type_expr != expected.rust_type_expr => {
                    tampered.push(format!(
                        "类型: {} (期望→{}, 实际→{})",
                        cn, expected.rust_type_expr, actual.rust_type_expr
                    ));
                }
                _ => {}
            }
        }

        // 校验所有权映射完整性
        for (cn, expected) in &builtin.ownership_mappings {
            match registry.ownership_mappings.get(cn) {
                None => missing.push(format!("所有权: {}", cn)),
                Some(actual) if actual.rust_syntax != expected.rust_syntax => {
                    tampered.push(format!(
                        "所有权: {} (期望→{}, 实际→{})",
                        cn, expected.rust_syntax, actual.rust_syntax
                    ));
                }
                _ => {}
            }
        }

        if !missing.is_empty() {
            return Err(ConfigError::MandatorySpecMissing { missing_items: missing });
        }
        if !tampered.is_empty() {
            return Err(ConfigError::MandatorySpecTampered {
                tampered_items: tampered,
            });
        }
        Ok(())
    }

    fn validate_backward_compatibility(
        _old_registry: &MappingRegistry,
        _new_registry: &MappingRegistry,
    ) -> Result<(), ConfigError> {
        // 向后兼容性校验：验证语法糖规则增删不破坏已有代码的转译正确性
        // 后续通过测试用例集执行转译比对实现
        Ok(())
    }
}