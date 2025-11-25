use super::LanguageAdapter;
use tree_sitter::{Language, Query};
use tree_sitter_c::LANGUAGE as c_;

const QUERY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/TreeSitterQuery/c.scm"
));

pub(super) struct Adapter;

impl LanguageAdapter for Adapter {
    fn get_language(&self) -> Language {
        c_.into()
    }

    fn get_comment_query(&self) -> Query {
        Query::new(&c_.into(), QUERY).unwrap()
    }
}
