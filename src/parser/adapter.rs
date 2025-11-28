use crate::core::{SupportLanguage, StaticTreeSitterQuery as STSQuery};
use std::collections::HashMap;
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
        use tree_sitter_java::LANGUAGE as java_;
        use tree_sitter_javascript::LANGUAGE as javascript_;

        let mut language: HashMap<SupportLanguage, Language> = HashMap::new();
        language.insert(SupportLanguage::Rust, rust_.into());
        language.insert(SupportLanguage::Python, python_.into());
        language.insert(SupportLanguage::Lua, lua_.into());
        language.insert(SupportLanguage::C, c_.into());
        language.insert(SupportLanguage::Java, java_.into());
        language.insert(SupportLanguage::JavaScript, javascript_.into());

        Adapter { language }
    }

    pub(super) fn get_language(&self, type_: SupportLanguage) -> &Language {
        let res = self.language.get(&type_);
        res.unwrap()
    }

    pub(super) fn get_comment_query(&self, type_: SupportLanguage) -> Query {
        // 加载 query 文件并 初始化 Query
        let query_file = STSQuery::get(&format!("{}.scm", type_.to_string())).unwrap();
        let query_code = std::str::from_utf8(&query_file.data).unwrap();
        Query::new(self.get_language(type_), query_code).unwrap()
    }
}
