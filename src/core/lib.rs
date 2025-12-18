//! 全局公用的 结构体 与 枚举
use rust_embed::RustEmbed;
use std::fmt::Display;
use serde::{Serialize, Deserialize};

/// 表示输入法当前的模式状态。
///
/// 这是跨平台的统一枚举，用于描述输入法是否处于用户母语/非英文模式
/// 或是英文模式。
///
/// **用法示例:**
/// ```rust
/// use crate::core::InputMethodMode;
///
/// let mode = InputMethodMode::Native;
/// assert_eq!(mode.to_string(), "native");
/// assert_eq!(InputMethodMode::default(), InputMethodMode::English);
/// ```
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum InputMethodMode {
    /// 母语或非英文输入模式
    Native,
    /// 英文输入模式
    English,
}
impl Display for InputMethodMode {
    /// 格式化输出
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InputMethodMode::Native => write!(f, "native"),
            InputMethodMode::English => write!(f, "english"),
        }
    }
}
impl InputMethodMode {
    pub fn from_str(mode: String) -> Result<InputMethodMode, serde_json::Error> {
        serde_json::from_str(&mode)
    }
}

/// 表示当前支持的编程语言。
///
/// 主要用于配置或通知客户端当前编辑器/进程正在使用的语言环境。
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum SupportLanguage {
    Rust,
    Python,
    Lua,
    C,
    Java,
    JavaScript,
}
impl SupportLanguage {
    /// SupportLanguage为可 哈希的，提高哈希表可读性
    /// 
    /// 尝试将字符串转换为 [`SupportLanguage`] 枚举。
    ///
    /// 转换是大小写不敏感的。
    ///
    /// ```rust
    /// use crate::core::InputMethodMode;
    ///
    /// assert_eq!(SupportLanguage::from_string("Rust".to_string()), Some(SupportLanguage::Rust));
    /// assert!(SupportLanguage::from_string("python".to_string()).is_none());
    /// ```
    pub fn from_string(s: &String) -> Option<SupportLanguage> {
        let lang = s.to_lowercase();
        if lang == "rust" {
            Some(SupportLanguage::Rust)
        } else if lang == "python" {
            Some(SupportLanguage::Python)
        } else if lang == "lua" {
            Some(SupportLanguage::Lua)
        } else if lang == "c" {
            Some(SupportLanguage::C)
        } else if lang == "java" {
            Some(SupportLanguage::Java)
        } else if lang == "javascript" {
            Some(SupportLanguage::JavaScript)
        }
        else {
            None
        }
    }

    /// 将枚举转换为对应的小写字符串。
    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap().trim_matches('"').to_lowercase()
    }
}

/// 表示文本编辑器中的光标位置
/// row 为 0基 行号 column 为 行内 utf-8 字节偏移量 0 基
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Cursor {
    /// 光标所在的行号 0基
    pub row: usize,
    /// 行内 utf-8 字节偏移量 0基
    pub column: usize,
}

/// 静态资源文件打包
/// 把 static 文件夹中的静态资源打包进 可执行文件中
/// 通过API访问且无额外开销
#[derive(RustEmbed)]
#[folder = r"src/static/TreeSitterQuery/"]
pub struct StaticTreeSitterQuery;
