use super::LanguageAdapter;
use tree_sitter::{Language, Query};
use tree_sitter_lua::LANGUAGE as lua_;

const QUERY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/TreeSitterQuery/lua.scm"
));

pub(super) struct Adapter;

impl LanguageAdapter for Adapter {
    fn get_language(&self) -> Language {
        lua_.into()
    }

    fn get_comment_query(&self) -> Query {
        Query::new(&lua_.into(), QUERY).unwrap()
    }
}
