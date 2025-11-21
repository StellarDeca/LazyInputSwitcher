use super::LanguageAdapter;
use tree_sitter::{Language, Query};
use tree_sitter_python::LANGUAGE as python_;

const QUERY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/TreeSitterQuery/python.scm"
));

pub(super) struct Adapter;

impl LanguageAdapter for Adapter {
    fn get_language(&self) -> Language {
        python_.into()
    }

    fn get_comment_query(&self) -> Query {
        Query::new(&python_.into(), QUERY).unwrap()
    }
}
