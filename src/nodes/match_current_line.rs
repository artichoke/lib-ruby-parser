use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCurrentLine {
    pub re: Box<Node>,

    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for MatchCurrentLine {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspect(&self, level: usize) -> String {
        todo!()
    }
}
