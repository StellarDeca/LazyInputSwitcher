
mod adapter;

use crate::core::*;
use adapter::*;
use std::collections::HashMap;
use tree_sitter::{Node, Query, QueryCursor, Range, StreamingIterator, Tree};

pub(super) struct Parser {
    adapter: Adapter,
    tree: Option<Tree>,
    parsers: HashMap<SupportLanguage, tree_sitter::Parser>,
    query: HashMap<SupportLanguage, Query>,
}
impl Parser {
    pub(super) fn new() -> Parser {
        let adapter = Adapter::new();
        let parsers = HashMap::new();
        let query = HashMap::new();
        Parser { adapter, parsers, query, tree: None }
    }

    pub(super) fn add_language(&mut self, type_: SupportLanguage) {
        let mut parser = tree_sitter::Parser::new();
        let query = self.adapter.get_comment_query(type_);

        parser.set_language(self.adapter.get_language(type_)).unwrap();
        self.parsers.insert(type_, parser);
        self.query.insert(type_, query);
    }

    pub(super) fn build_tree(&mut self, type_: SupportLanguage, code: &String) {
        // 如果tree不存在，则自动新建树
        let parser = self.parsers.get_mut(&type_).unwrap();
        self.tree = parser.parse(code.as_bytes(), None);
    }

    pub(super) fn get_comments(&mut self, type_: SupportLanguage, code: &String) -> NodesRange {
        let mut node_range = NodesRange::new();
        if let Some(tree) = &self.tree {
            let root = tree.root_node();
            let query = self.query.get(&type_).unwrap();
            let mut query_cursor = QueryCursor::new();
            let mut res = query_cursor.matches(&query, root, code.as_bytes());
            // 遍历结果，返回comment的range数组
            while let Some(m) = res.next() {
                for iter in m.captures { node_range.add_node(iter.node) };
            };
        };
        node_range
    }
}

pub(super) struct NodesRange {
    nodes_range: Vec<Range>,
}
impl NodesRange {
    fn new() -> NodesRange { NodesRange { nodes_range: vec![] } }

    fn add_node(&mut self, node: Node) {
        self.nodes_range.push(node.range())
    }

    pub(super) fn in_range(&self, cursor: &Cursor, code: &String) -> bool {
        // 判断cursor的位置是否在node节点里
        // row 为 0基 行号 column 为 行内 utf-8 字节偏移量 0 基
        let (sr, sc) = (cursor.row, cursor.column);

        fn cmp_pos(r1: usize, c1: usize, r2: usize, c2: usize) -> i8 {
            // 判断给定的r1, c1是否在r2,c2范围内
            // 范围左面返回-1,范围右面返回1,相等返回0
            if r1 < r2 { return -1 };
            if r1 > r2 { return 1 };
            if c1 < c2 { return -1 };
            if c1 > c2 { return 1 };
            0
        }
        for range in &self.nodes_range {
            let start = range.start_point;
            let end = range.end_point;
            let (rs, cs) = (start.row, start.column);
            let (re, ce) = (end.row, end.column);

            // 严格判断边界条件， 左开右闭
            // 注意TreeSitter本身范围为 左闭右开区间
            //
            // 对于cursor处于注释范围末尾时
            // 根据cursor后接字符是否为文本结束或者换行符
            // 如果为文本结束或者换行符
            // 则认为cursor处在注释中
            if cmp_pos(sr, sc, re, ce) == 0 {
                // 当cursor恰好位于结束符位置时
                // 分析 cursor 到 行末 之间的字符
                // 全部为空白字符则说明无其他有意义字符 => in comment
                // 存在非空白字符则说明存在其他有含义的字符 => not in comment
                if let Some(mut tail) = code.get(range.end_byte..) {
                    let end = tail.find('\n').unwrap_or(tail.len());
                    tail = &tail[..end];
                    if !tail.trim().is_empty() {
                        return false;
                    }
                }
                return true;
            } else if cmp_pos(sr, sc, rs, cs) > 0 && cmp_pos(sr, sc, re, ce) < 0 {
                return true;
            }
        };
        false
    }
}
