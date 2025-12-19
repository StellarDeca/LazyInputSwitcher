use crate::core::*;
use crate::parser::*;

impl Cursor {
    pub fn new(row: usize, column: usize) -> Cursor {
        Cursor { row, column }
    }
}

#[test]
fn test() {
    let code = &r#"
// <-- 行注释
pub fn main() { println!("Hello World!"); }
/**
    <--- 块注释
**/
        "#
    .to_string();
    let mut parser = Parser::new();
    let lang = SupportLanguage::Rust;
    parser.add_language(lang);
    parser.update_tree(lang, code);
    let res = parser.get_comments(lang, code);
    // 判断 TS 解析是否正常, 检查边界条件 与 内部条件 文档注释同样视为单行注释 或 块注释
    // 单行注释
    assert_eq!(res.in_range(&Cursor::new(1, 0)), false);
    assert_eq!(res.in_range(&Cursor::new(1, 1)), true);
    // 代码片段
    assert_eq!(res.in_range(&Cursor::new(2, 5)), false);
    // 块注释
    assert_eq!(res.in_range(&Cursor::new(3, 0)), false);
    assert_eq!(res.in_range(&Cursor::new(3, 1)), true);
    assert_eq!(res.in_range(&Cursor::new(5, 2)), true);
    assert_eq!(res.in_range(&Cursor::new(5, 3)), false);
}
