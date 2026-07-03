use serde::{Deserialize, Serialize};

/// 中文AST节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CnAstNodeType {
    /// 函数定义
    FnDef,
    /// 变量绑定
    LetBinding,
    /// 控制流
    ControlFlow,
    /// 模式匹配
    PatternMatch,
    /// 结构体定义
    StructDef,
    /// 枚举定义
    EnumDef,
    /// 特征定义
    TraitDef,
    /// 特征实现
    TraitImpl,
    /// 泛型函数
    GenericFn,
    /// 泛型结构体
    GenericStruct,
    /// 泛型枚举
    GenericEnum,
    /// 泛型约束（其中/where）
    GenericConstraint,
    /// 生命周期标注
    LifetimeAnnotation,
    /// 所有权借用
    OwnershipBorrow,
    /// 模块定义
    ModuleDef,
    /// 语法糖
    Sugar,
    /// 表达式
    Expr,
    /// 语句
    Stmt,
}

/// Rust AST节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RustAstNodeType {
    Fn,
    Let,
    If,
    Match,
    Loop,
    While,
    For,
    Struct,
    Enum,
    Trait,
    Impl,
    GenericFn,
    GenericStruct,
    GenericEnum,
    WhereClause,
    Lifetime,
    Borrow,
    Mod,
    Expr,
    Stmt,
}