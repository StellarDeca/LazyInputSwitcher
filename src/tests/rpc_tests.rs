use crate::rpc::*;
use serde_json::json;

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

#[test]
fn to_json_message() {
    let r = AnalyzeResult {
        grammar: GrammarMode::Code,
    };
    let res = ClientResponse::new(0, true, None, Some(CommandResult::from_analyze_result(r)))
        .to_json_message();
    let res_json: serde_json::Value = serde_json::from_str(&res).unwrap();
    let mes = json!({
        "cid": 0, "success": true, "error": null, "result": { "grammar": "Code"}
    });
    assert_eq!(res_json, mes);
}
