use serde::{Deserialize, Serialize};

use crate::sugar_types::{PatternElementType, PatternType, SugarRuleStatus};

/// 语法糖匹配模式
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SugarPattern {
    /// 模式类型
    pub pattern_type: PatternType,
    /// 模式元素列表
    pub elements: Vec<PatternElement>,
}

/// 模式元素
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternElement {
    /// 元素类型
    pub element_type: PatternElementType,
    /// 元素值
    pub value: String,
    /// 捕获组名称
    pub capture_name: Option<String>,
}

/// 展开模板
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpansionTemplate {
    /// 模板部分列表
    pub template_parts: Vec<TemplatePart>,
}

/// 模板部分
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplatePart {
    /// 是否为捕获组引用
    pub is_capture_ref: bool,
    /// 内容（字面量文本或捕获组名称）
    pub content: String,
}

/// 语法糖规则
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SugarRule {
    /// 语法糖名称（2-8个汉字）
    pub name: String,
    /// 匹配模式
    pub match_pattern: SugarPattern,
    /// 展开模板
    pub expansion_template: ExpansionTemplate,
    /// 优先级（数值越大优先级越高）
    pub priority: i32,
    /// 展开条件（可选）
    pub expansion_condition: Option<String>,
    /// 规则状态
    pub status: SugarRuleStatus,
}