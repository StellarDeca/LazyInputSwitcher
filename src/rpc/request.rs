//! #### 客户端请求样式
//! ```json
//! {
//!     cid: u16, // 客户端ID，用于标识客户端身份 值为0时自动分配cid
//!
//!     // Exit 时服务端将会结束自身的运行，服务端一段时间无客户端连接也会自动退出
//!     // Switch 时 将会执行语法分析 与输入法自动切换
//!     // Analyze 时 仅执行 语法分析
//!     // MethodOnly 时 仅执行输入法切换
//!     command: Exit, Switcher, Analyze, MethodOnly
//!
//!     /// 按照命令类型区分 Analyze 参数
//!     params: {
//!         code: String,  // 原始代码
//!
//!         // 代码类型,注意首字母大写
//!         // 名称应与 crate::core::SupportLanguage 枚举中保持一致
//!         language: String,
//!
//!         // 光标位置 UTF-16 字符位置, 0基
//!         cursor: {
//!             row: usize,
//!             column: usize
//!         }
//!     },
//!
//!     /// MethodOnly 参数
//!     params: {
//!         mode: Native / English,  // 目标输入法, 首字母大写
//!     },
//!
//!     /// Switch 参数
//!     params: {
//!         code: String,  // 原始代码
//!
//!         // 代码类型,注意首字母大写
//!         // 名称应与 crate::core::SupportLanguage 枚举中保持一致
//!         language: String,
//!
//!         // 光标位置 UTF-16 字符位置, 0基
//!         cursor: {
//!             row: usize,
//!             column: usize
//!         }
//!     },
//!
//!     /// Exit 参数
//!     params: {
//!         // 无参数, 空的 一对花括号
//!     },
//! }
//! ```

use crate::core::Cursor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum CommandMode {
    Analyze,
    MethodOnly,
    Switch,
    Exit,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AnalyzeParams {
    pub(crate) code: String,
    pub(crate) language: String,
    pub(crate) cursor: Cursor,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MethodOnlyParams {
    pub(crate) mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SwitchParams {
    pub(crate) code: String,
    pub(crate) language: String,
    pub(crate) cursor: Cursor,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub(crate) struct CommandParams {
    pub(crate) params: serde_json::Value,
}
impl CommandParams {
    pub(crate) fn to_analyze_params(self) -> Result<AnalyzeParams, serde_json::Error> {
        serde_json::from_value(self.params)
    }

    pub(crate) fn to_method_only_params(self) -> Result<MethodOnlyParams, serde_json::Error> {
        serde_json::from_value(self.params)
    }

    pub(crate) fn to_switch_params(self) -> Result<SwitchParams, serde_json::Error> {
        serde_json::from_value(self.params)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ClientRequest {
    pub(crate) cid: u16,
    pub(crate) command: CommandMode,
    pub(crate) params: CommandParams,
}
impl ClientRequest {
    pub(crate) fn from_json_message(json_string: String) -> Result<ClientRequest, String> {
        // 使用 serde_json::from_str 函数
        match serde_json::from_str(&json_string) {
            Ok(request) => Ok(request),
            Err(json_error) => Err(json_error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_analyze_params() {

        let json_string = r#"
            {
                "cid": 1,
                "command": "Analyze",
                "params": {
                    "code": "let x = 1;",
                    "language": "Rust",
                    "cursor": { "row": 0, "column": 5 }
                }
            }
        "#;

        let req: ClientRequest = serde_json::from_str(json_string).unwrap();
        let analyze: AnalyzeParams = req.params.to_analyze_params().unwrap();
        assert_eq!(analyze.code, "let x = 1;");
        assert_eq!(analyze.language, "Rust");
        assert_eq!(analyze.cursor.row, 0);
        assert_eq!(analyze.cursor.column, 5);
    }
}
