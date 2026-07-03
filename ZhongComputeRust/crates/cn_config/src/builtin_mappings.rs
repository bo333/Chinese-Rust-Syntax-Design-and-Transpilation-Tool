use cn_types::mapping_models::{KeywordMapping, OwnershipMapping, TypeMapping};
use cn_types::mapping_registry::MappingRegistry;

/// 构建内置强制规范映射注册表
pub fn build_builtin_registry(version: &str) -> MappingRegistry {
    let mut registry = MappingRegistry::new(version);

    register_builtin_keywords(&mut registry);
    register_builtin_types(&mut registry);
    register_builtin_ownership(&mut registry);

    registry
}

/// 注册内置33项关键词映射
fn register_builtin_keywords(registry: &mut MappingRegistry) {
    let keywords = vec![
        ("函数", "fn", "函数定义"),
        ("让", "let", "变量绑定"),
        ("可变", "mut", "可变性修饰"),
        ("如果", "if", "条件判断"),
        ("否则", "else", "否则分支"),
        ("匹配", "match", "模式匹配"),
        ("循环", "loop", "无限循环"),
        ("当", "while", "条件循环"),
        ("遍历", "for", "迭代循环"),
        ("在", "in", "迭代范围"),
        ("返回", "return", "函数返回"),
        ("结构体", "struct", "结构体定义"),
        ("枚举", "enum", "枚举定义"),
        ("特征", "trait", "特征定义"),
        ("实现", "impl", "特征/类型实现"),
        ("公开", "pub", "可见性修饰"),
        ("使用", "use", "导入声明"),
        ("模块", "mod", "模块定义"),
        ("自身", "self", "自身引用"),
        ("超集", "super", "父模块引用"),
        ("外部", "extern", "外部链接"),
        ("不安全", "unsafe", "不安全块"),
        ("类型别名", "type", "类型别名"),
        ("其中", "where", "约束子句"),
        ("异步", "async", "异步修饰"),
        ("等待", "await", "异步等待"),
        ("移动", "move", "闭包捕获"),
        ("引用", "ref", "引用绑定"),
        ("静态", "static", "静态项"),
        ("常量", "const", "常量定义"),
        ("真", "true", "布尔真值"),
        ("假", "false", "布尔假值"),
        ("作为", "as", "类型转换"),
    ];

    for (cn, rust, desc) in keywords {
        let mapping = KeywordMapping {
            cn_keyword: cn.to_string(),
            rust_keyword: rust.to_string(),
            semantic_description: desc.to_string(),
            is_reserved: true,
            is_builtin: true,
            version: "1.0".to_string(),
        };
        let _ = registry.register_keyword(mapping);
    }
}

/// 注册内置27项类型名映射
fn register_builtin_types(registry: &mut MappingRegistry) {
    let types = vec![
        ("整数", "i32", true),
        ("整数8", "i8", false),
        ("整数16", "i16", false),
        ("整数32", "i32", false),
        ("整数64", "i64", false),
        ("整数尺寸", "isize", false),
        ("正整数", "u32", true),
        ("正整数8", "u8", false),
        ("正整数16", "u16", false),
        ("正整数32", "u32", false),
        ("正整数64", "u64", false),
        ("正整数尺寸", "usize", false),
        ("浮点数", "f64", true),
        ("浮点数32", "f32", false),
        ("浮点数64", "f64", false),
        ("布尔", "bool", true),
        ("字符", "char", true),
        ("字符串", "String", true),
        ("字符串切片", "&str", true),
        ("元组", "(T1, T2, ...)", true),
        ("数组", "[T; N]", true),
        ("向量", "Vec<T>", true),
        ("哈希映射", "HashMap<K, V>", true),
        ("哈希集合", "HashSet<T>", true),
        ("选项", "Option<T>", true),
        ("结果", "Result<T, E>", true),
        ("无", "()", true),
    ];

    for (cn, rust, is_base) in types {
        let mapping = TypeMapping {
            cn_type_name: cn.to_string(),
            rust_type_expr: rust.to_string(),
            is_base_mapping: is_base,
            generic_params: Vec::new(),
            constraints: Vec::new(),
        };
        let _ = registry.register_type(mapping);
    }
}

/// 注册内置4项所有权语法映射
fn register_builtin_ownership(registry: &mut MappingRegistry) {
    let ownerships = vec![
        ("借用", "&", "不可变借用"),
        ("可变借用", "&mut", "可变借用"),
        ("解引用", "*", "解引用操作"),
        ("生命周期", "'a", "生命周期标注"),
    ];

    for (cn, rust, desc) in ownerships {
        let mapping = OwnershipMapping {
            cn_syntax: cn.to_string(),
            rust_syntax: rust.to_string(),
            semantic_description: desc.to_string(),
        };
        let _ = registry.register_ownership(mapping);
    }
}