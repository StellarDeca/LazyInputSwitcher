use crate::core::{Cursor, SupportLanguage};
use crate::parser::Parser;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(lang: SupportLanguage, code: &str, row: usize, col: usize) {
        let mut parser = Parser::new();
        let code_str = code.to_string();
        parser.add_language(lang);
        parser.build_tree(lang, &code_str);

        let comments = parser.get_comments(lang, &code_str);
        let cursor = Cursor { row, column: col };

        // 只要运行不崩溃即代表通过
        let _result = comments.in_range(&cursor, &code_str);
    }

    #[test]
    fn test_rust() {
        run_test(SupportLanguage::Rust, "// Rust comment\nfn main() {}", 0, 5);
    }

    #[test]
    fn test_python() {
        run_test(
            SupportLanguage::Python,
            "# Python comment\ndef main(): pass",
            0,
            5,
        );
    }

    #[test]
    fn test_lua() {
        run_test(
            SupportLanguage::Lua,
            "-- Lua comment\nfunction f() end",
            0,
            5,
        );
    }

    #[test]
    fn test_c() {
        run_test(SupportLanguage::C, "/* C comment */\nint main() {}", 0, 5);
    }

    #[test]
    fn test_java() {
        run_test(
            SupportLanguage::Java,
            "// Java comment\nclass Main {}",
            0,
            5,
        );
    }

    #[test]
    fn test_javascript() {
        run_test(
            SupportLanguage::JavaScript,
            "// JS comment\nfunction f() {}",
            0,
            5,
        );
    }

    #[test]
    fn test_kotlin() {
        run_test(
            SupportLanguage::Kotlin,
            "// Kotlin comment\nfun main() {}",
            0,
            5,
        );
    }

    #[test]
    fn test_typescript() {
        run_test(
            SupportLanguage::TypeScript,
            "// TS comment\nlet x = 1;",
            0,
            5,
        );
    }

    #[test]
    fn test_cpp() {
        run_test(SupportLanguage::Cpp, "// C++ comment\nint main() {}", 0, 5);
    }

    #[test]
    fn test_go() {
        run_test(SupportLanguage::Go, "// Go comment\npackage main", 0, 5);
    }

    #[test]
    fn test_bash() {
        run_test(SupportLanguage::Bash, "# Bash comment\necho hello", 0, 5);
    }

    #[test]
    fn test_sql() {
        run_test(
            SupportLanguage::Sql,
            "-- SQL comment\nSELECT * FROM users",
            0,
            5,
        );
    }

    #[test]
    fn test_php() {
        run_test(SupportLanguage::Php, "<?php // PHP comment", 0, 10);
    }

    #[test]
    fn test_csharp() {
        run_test(
            SupportLanguage::CSharp,
            "// C# comment\nclass Program {}",
            0,
            5,
        );
    }
}
