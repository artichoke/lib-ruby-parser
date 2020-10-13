use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct XHeredoc {
    pub parts: Vec<Node>,

    pub heredoc_body_l: Range,
    pub heredoc_end_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for XHeredoc {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspect(&self, level: usize) -> String {
        todo!()
    }
}
