use serde::{Deserialize, Serialize};

/// 错误码值对象：强制校验格式为"CNxxxx"（四位数字）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ErrorCode {
    code: String,
}

impl ErrorCode {
    /// 创建错误码，校验格式必须为"CN"后跟四位数字
    pub fn new(code: &str) -> Result<Self, ErrorCodeFormatError> {
        if code.len() != 6 {
            return Err(ErrorCodeFormatError {
                code: code.to_string(),
                reason: "错误码长度必须为6个字符（CN+四位数字）".to_string(),
            });
        }
        if !code.starts_with("CN") {
            return Err(ErrorCodeFormatError {
                code: code.to_string(),
                reason: "错误码必须以'CN'开头".to_string(),
            });
        }
        if !code[2..].chars().all(|c| c.is_ascii_digit()) {
            return Err(ErrorCodeFormatError {
                code: code.to_string(),
                reason: "错误码'CN'后必须为四位数字".to_string(),
            });
        }
        Ok(Self {
            code: code.to_string(),
        })
    }

    /// 获取错误码字符串引用
    pub fn as_str(&self) -> &str {
        &self.code
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl AsRef<str> for ErrorCode {
    fn as_ref(&self) -> &str {
        &self.code
    }
}

/// 错误码格式校验失败
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorCodeFormatError {
    pub code: String,
    pub reason: String,
}

impl std::fmt::Display for ErrorCodeFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "错误码格式校验失败: '{}' - {}", self.code, self.reason)
    }
}

impl std::error::Error for ErrorCodeFormatError {}

// ========== 词法分析错误码（CN0001-CN0004） ==========

/// CN0001: 文件编码异常
pub const CN0001: &str = "CN0001";
/// CN0002: 未识别字符
pub const CN0002: &str = "CN0002";
/// CN0003: 保留字冲突
pub const CN0003: &str = "CN0003";
/// CN0004: 未定义关键词
pub const CN0004: &str = "CN0004";

// ========== 语法解析错误码（CN0100-CN0101） ==========

/// CN0100: 语法结构不完整
pub const CN0100: &str = "CN0100";
/// CN0101: 语法糖匹配歧义
pub const CN0101: &str = "CN0101";

// ========== 语法糖展开错误码（CN0200-CN0202） ==========

/// CN0200: 语法糖嵌套冲突
pub const CN0200: &str = "CN0200";
/// CN0201: 展开结果非法
pub const CN0201: &str = "CN0201";
/// CN0202: 正交性违规
pub const CN0202: &str = "CN0202";

// ========== 语义映射错误码（CN0300-CN0308） ==========

/// CN0300: 映射缺失
pub const CN0300: &str = "CN0300";
/// CN0301: 类型映射歧义
pub const CN0301: &str = "CN0301";
/// CN0302: 生命周期推断失败
pub const CN0302: &str = "CN0302";
/// CN0303: 确定性违反
pub const CN0303: &str = "CN0303";
/// CN0304: 循环歧义
pub const CN0304: &str = "CN0304";
/// CN0305: 模式匹配映射失败
pub const CN0305: &str = "CN0305";
/// CN0306: Trait映射失败
pub const CN0306: &str = "CN0306";
/// CN0307: 泛型映射失败
pub const CN0307: &str = "CN0307";
/// CN0308: 生命周期映射失败
pub const CN0308: &str = "CN0308";

// ========== 代码生成错误码（CN0400-CN0402） ==========

/// CN0400: 格式化失败
pub const CN0400: &str = "CN0400";
/// CN0401: 映射对照表不一致
pub const CN0401: &str = "CN0401";
/// CN0402: 非源码引入的unsafe块
pub const CN0402: &str = "CN0402";

// ========== 诊断翻译错误码（CN0500-CN0501） ==========

/// CN0500: 翻译词典缺失
pub const CN0500: &str = "CN0500";
/// CN0501: 源映射缺失
pub const CN0501: &str = "CN0501";

// ========== 配置错误码（CN0600-CN0606） ==========

/// CN0600: 配置文件不存在
pub const CN0600: &str = "CN0600";
/// CN0601: 配置格式错误
pub const CN0601: &str = "CN0601";
/// CN0602: 映射规范版本不一致
pub const CN0602: &str = "CN0602";
/// CN0603: 自定义映射冲突
pub const CN0603: &str = "CN0603";
/// CN0604: 强制规范缺失
pub const CN0604: &str = "CN0604";
/// CN0605: 强制规范被篡改
pub const CN0605: &str = "CN0605";
/// CN0606: 向后兼容性违反
pub const CN0606: &str = "CN0606";

// ========== 增量转译错误码（CN0700-CN0702） ==========

/// CN0700: 缓存不存在
pub const CN0700: &str = "CN0700";
/// CN0701: 缓存损坏
pub const CN0701: &str = "CN0701";
/// CN0702: 映射规范版本变更
pub const CN0702: &str = "CN0702";

// ========== 等价性验证错误码（CN0800-CN0802） ==========

/// CN0800: 文本不等价
pub const CN0800: &str = "CN0800";
/// CN0801: 编译不等价
pub const CN0801: &str = "CN0801";
/// CN0802: 语义不等价
pub const CN0802: &str = "CN0802";

// ========== 迁移错误码（CN0900-CN0901） ==========

/// CN0900: 无法自动迁移
pub const CN0900: &str = "CN0900";
/// CN0901: 迁移后转译失败
pub const CN0901: &str = "CN0901";

// ========== LSP错误码（CN1000-CN1001） ==========

/// CN1000: LSP连接异常
pub const CN1000: &str = "CN1000";
/// CN1001: 增量转译超时
pub const CN1001: &str = "CN1001";

// ========== 安全策略错误码（CN1100） ==========

/// CN1100: 安全策略违反
pub const CN1100: &str = "CN1100";