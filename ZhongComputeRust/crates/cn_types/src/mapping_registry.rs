use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::mapping_models::{KeywordMapping, OwnershipMapping, TypeMapping};
use crate::sugar_models::SugarRule;

/// 映射规则注册表：映射规范的运行时表示
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingRegistry {
    /// 关键词映射表（中文关键词→映射信息）
    pub keyword_mappings: HashMap<String, KeywordMapping>,
    /// 类型映射表（中文类型名→映射信息）
    pub type_mappings: HashMap<String, TypeMapping>,
    /// 语法糖规则集
    pub sugar_rules: Vec<SugarRule>,
    /// 所有权语法映射表
    pub ownership_mappings: HashMap<String, OwnershipMapping>,
    /// 映射规范版本号
    pub version: String,
    /// 反向关键词映射索引（Rust关键词→中文关键词）
    pub reverse_keyword_index: HashMap<String, String>,
    /// 反向类型映射索引（Rust类型表达式→中文类型名）
    pub reverse_type_index: HashMap<String, String>,
}

impl MappingRegistry {
    /// 创建空的映射注册表
    pub fn new(version: &str) -> Self {
        Self {
            keyword_mappings: HashMap::new(),
            type_mappings: HashMap::new(),
            sugar_rules: Vec::new(),
            ownership_mappings: HashMap::new(),
            version: version.to_string(),
            reverse_keyword_index: HashMap::new(),
            reverse_type_index: HashMap::new(),
        }
    }

    /// 注册关键词映射，自动构建反向索引
    pub fn register_keyword(&mut self, mapping: KeywordMapping) -> Result<(), String> {
        if self.keyword_mappings.contains_key(&mapping.cn_keyword) {
            return Err(format!("关键词映射冲突: '{}' 已存在", mapping.cn_keyword));
        }
        self.reverse_keyword_index
            .insert(mapping.rust_keyword.clone(), mapping.cn_keyword.clone());
        self.keyword_mappings
            .insert(mapping.cn_keyword.clone(), mapping);
        Ok(())
    }

    /// 注册类型映射，自动构建反向索引
    pub fn register_type(&mut self, mapping: TypeMapping) -> Result<(), String> {
        if self.type_mappings.contains_key(&mapping.cn_type_name) {
            return Err(format!("类型映射冲突: '{}' 已存在", mapping.cn_type_name));
        }
        self.reverse_type_index
            .insert(mapping.rust_type_expr.clone(), mapping.cn_type_name.clone());
        self.type_mappings
            .insert(mapping.cn_type_name.clone(), mapping);
        Ok(())
    }

    /// 注册所有权语法映射
    pub fn register_ownership(&mut self, mapping: OwnershipMapping) -> Result<(), String> {
        if self.ownership_mappings.contains_key(&mapping.cn_syntax) {
            return Err(format!("所有权映射冲突: '{}' 已存在", mapping.cn_syntax));
        }
        self.ownership_mappings
            .insert(mapping.cn_syntax.clone(), mapping);
        Ok(())
    }

    /// 注册语法糖规则
    pub fn register_sugar_rule(&mut self, rule: SugarRule) -> Result<(), String> {
        if self.sugar_rules.iter().any(|r| r.name == rule.name) {
            return Err(format!("语法糖规则冲突: '{}' 已存在", rule.name));
        }
        self.sugar_rules.push(rule);
        Ok(())
    }
}