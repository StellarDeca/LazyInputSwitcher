//! #### 服务端响应样式（Response）
//!
//! 服务端对每一次客户端请求返回一个 Response 对象，用于告知当前请求的处理结果、状态、以及相关语法分析和输入法切换信息。
//!
//! ```json
//! {
//!     cid: u16,                 // 服务端确认的客户端 ID（与请求一致）
//!     success: bool,            // 请求是否成功（true 表示操作成功）
//!     error: Null / String      // 当 success = false 时, 此字段包含轻轻执行错误原因 成功则为 Null
//!     result: Null / {}         // 当 success = true 时，此字段包含执行结果 不成功则为 Null
//!
//!     /// Analyze 请求结果
//!     result: {
//!         grammar: Comment / Code
//!     }
//!
//!     /// ModeOnly 请求结果
//!     result: {
//!         method: Native / English
//!     }
//!
//!     /// Switch 请求结果
//!     result: {
//!         grammar : Comment / Code,
//!         method: Native / English,
//!     }
//!
//!     /// Exit 无请求结果, 服务器将断开网络连接之后结束自身
//! }
//! ```

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum GrammarMode {
    Code,
    Comment,
}
impl GrammarMode {
    pub(crate) fn from_bool(in_comment: bool) -> GrammarMode {
        if in_comment { GrammarMode::Comment } else { GrammarMode::Code }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AnalyzeResult {
    pub(crate) grammar: GrammarMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MethodOnlyResult {
    pub(crate) method: crate::core::InputMethodMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SwitchResult {
    pub(crate) grammar: GrammarMode,
    pub(crate) method: crate::core::InputMethodMode,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub(crate) struct CommandResult {
    pub(crate) result: serde_json::Value,
}
impl CommandResult {
    pub(crate) fn from_analyze_result(result: AnalyzeResult) -> CommandResult {
        CommandResult { result: serde_json::to_value(&result).unwrap() }
    }

    pub(crate) fn from_method_only_result(result: MethodOnlyResult) -> CommandResult {
        CommandResult { result: serde_json::to_value(&result).unwrap() }
    }

    pub(crate) fn from_switch_result(result: SwitchResult) -> CommandResult {
        CommandResult { result: serde_json::to_value(&result).unwrap() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ClientResponse {
    cid: u16,
    success: bool,
    error: Option<String>,
    result: Option<CommandResult>,
}
impl ClientResponse {
    pub(crate) fn new(cid: u16, success: bool, error: Option<String>, result: Option<CommandResult>) -> ClientResponse {
        ClientResponse { cid, success, error, result }
    }

    pub(crate) fn to_json_message(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;
    #[test]
    fn to_json_message() {
        let r = AnalyzeResult { grammar: GrammarMode::Code };
        let res = ClientResponse::new(
            0, true, None, Some(CommandResult::from_analyze_result(r))
        ).to_json_message();
        let res_json: serde_json::Value = serde_json::from_str(&res).unwrap();
        let mes = json!({
            "cid": 0, "success": true, "error": null, "result": { "grammar": "Code"}
        });
        assert_eq!(res_json, mes);
    }
}
