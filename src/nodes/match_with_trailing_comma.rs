use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

// TODO: remove
#[derive(Debug, Clone, PartialEq)]
pub struct MatchWithTrailingComma {
    pub match_: Box<Node>,

    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for MatchWithTrailingComma {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspect(&self, level: usize) -> String {
        todo!()
    }
}
