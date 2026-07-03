use cn_types::mapping_models::{KeywordMapping, OwnershipMapping, TypeMapping};
use cn_types::mapping_registry::MappingRegistry;
use cn_types::sugar_models::SugarRule;

/// 映射规则查询接口
pub trait MappingQuery {
    /// 查询中文关键词对应的Rust关键词
    fn lookup_keyword(&self, cn_keyword: &str) -> Option<&KeywordMapping>;

    /// 反向查询：Rust关键词→中文关键词
    fn lookup_keyword_reverse(&self, rust_keyword: &str) -> Option<&KeywordMapping>;

    /// 查询中文类型名对应的Rust类型
    fn lookup_type(&self, cn_type_name: &str) -> Option<&TypeMapping>;

    /// 反向查询：Rust类型→中文类型名
    fn lookup_type_reverse(&self, rust_type_expr: &str) -> Option<&TypeMapping>;

    /// 查询语法糖规则
    fn lookup_sugar_rule(&self, sugar_name: &str) -> Option<&SugarRule>;

    /// 查询所有权语法映射
    fn lookup_ownership(&self, cn_ownership_syntax: &str) -> Option<&OwnershipMapping>;

    /// 获取当前映射规范版本
    fn get_version(&self) -> &str;
}

impl MappingQuery for MappingRegistry {
    fn lookup_keyword(&self, cn_keyword: &str) -> Option<&KeywordMapping> {
        self.keyword_mappings.get(cn_keyword)
    }

    fn lookup_keyword_reverse(&self, rust_keyword: &str) -> Option<&KeywordMapping> {
        self.reverse_keyword_index
            .get(rust_keyword)
            .and_then(|cn_kw| self.keyword_mappings.get(cn_kw))
    }

    fn lookup_type(&self, cn_type_name: &str) -> Option<&TypeMapping> {
        self.type_mappings.get(cn_type_name)
    }

    fn lookup_type_reverse(&self, rust_type_expr: &str) -> Option<&TypeMapping> {
        self.reverse_type_index
            .get(rust_type_expr)
            .and_then(|cn_type| self.type_mappings.get(cn_type))
    }

    fn lookup_sugar_rule(&self, sugar_name: &str) -> Option<&SugarRule> {
        self.sugar_rules.iter().find(|r| r.name == sugar_name)
    }

    fn lookup_ownership(&self, cn_ownership_syntax: &str) -> Option<&OwnershipMapping> {
        self.ownership_mappings.get(cn_ownership_syntax)
    }

    fn get_version(&self) -> &str {
        &self.version
    }
}