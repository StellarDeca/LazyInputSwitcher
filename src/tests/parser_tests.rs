use crate::core::*;
use crate::parser::*;

impl Cursor {
    pub fn new(row: usize, column: usize) -> Cursor {
        Cursor { row, column }
    }
}

#[derive(Debug)]
struct CommentCheck {
    row: usize,
    col: usize,
    in_comment: bool,
}
impl CommentCheck {
    fn new(row: usize, col: usize, in_comment: bool) -> CommentCheck {
        CommentCheck { row, col, in_comment }
    }
}

fn run_comment_test(lang: SupportLanguage, code: String, checks: &[CommentCheck]) {
    let mut parser = Parser::new();
    parser.add_language(lang);
    parser.build_tree(lang, &code);

    let comments = parser.get_comments(lang, &code);

    for check in checks {
        let except = if check.in_comment { "comment" } else { "code" };
        assert_eq!(
            comments.in_range(&Cursor::new(check.row, check.col), &code),
            check.in_comment,
            "{:?}: Test Failed at position ({}, {}) Except {}"
            , lang, check.row, check.col, except
        );
    }
}

#[test]
fn rust() {
    let code = r#"
// <-- 行注释
///
pub fn main() { println!("Hello World!"); }
/**
    <--- 块注释
**/a
        "#
        .to_string();
    let lang = SupportLanguage::Rust;
    let checks = [
        // 单行注释
        CommentCheck::new(1, 0, false),
        CommentCheck::new(2, 2, true),
        // 代码片段
        CommentCheck::new(3, 5, false),
        // 块注释
        CommentCheck::new(4, 0, false),
        CommentCheck::new(4, 1, true),
        CommentCheck::new(6, 2, true),
        CommentCheck::new(6, 3, false),
    ];
    run_comment_test(lang, code, &checks);
}

#[test]
fn lua() {
    let code = r#"
--  <--- 行注释
---
print("lua test")
--[[
    <--- 块注释
--]] local f = {}
    "#.to_string();
    let lang = SupportLanguage::Lua;
    let checks = [
        // 单行注释
        CommentCheck::new(1, 0, false),
        CommentCheck::new(2, 2, true),
        // 代码片段
        CommentCheck::new(3, 5, false),
        // 块注释
        CommentCheck::new(4, 0, false),
        CommentCheck::new(4, 1, true),
        CommentCheck::new(6, 3, true),
        CommentCheck::new(6, 4, false),
    ];
    run_comment_test(lang, code, &checks);
}
