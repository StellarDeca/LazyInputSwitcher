use crate::core::SupportLanguage;
use std::collections::HashMap;
use std::path::PathBuf;
use tree_sitter::{Language, Query};

pub(super) struct Adapter {
    language: HashMap<SupportLanguage, Language>,
}
impl Adapter {
    pub(super) fn new() -> Adapter {
        // 封装统一引用
        use tree_sitter_c::LANGUAGE as c_;
        use tree_sitter_lua::LANGUAGE as lua_;
        use tree_sitter_python::LANGUAGE as python_;
        use tree_sitter_rust::LANGUAGE as rust_;

        let mut language: HashMap<SupportLanguage, Language> = HashMap::new();
        language.insert(SupportLanguage::Rust, rust_.into());
        language.insert(SupportLanguage::Python, python_.into());
        language.insert(SupportLanguage::Lua, lua_.into());
        language.insert(SupportLanguage::C, c_.into());

        Adapter { language }
    }

    pub(super) fn get_language(&self, type_: SupportLanguage) -> &Language {
        let res = self.language.get(&type_);
        res.unwrap()
    }

    pub(super) fn get_comment_query(&self, type_: SupportLanguage) -> Query {
        // 构造 query 文件路径并动态加载
        let relative_path = format!("/src/static/TreeSitterQuery/{}.scm", type_.to_string());
        let mut absolute_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        absolute_path.push(&relative_path.trim_start_matches('/'));

        let query_code = std::fs::read_to_string(absolute_path).unwrap();
        Query::new(self.get_language(type_), query_code.as_str()).unwrap()
    }
}
