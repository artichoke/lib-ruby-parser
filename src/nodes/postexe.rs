use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Postexe {
    pub body: Option<Box<Node>>,

    pub expression_l: Range,
    pub keyword_l: Range,
}

impl<'a> InnerNode<'a> for Postexe {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspect(&self, level: usize) -> String {
        todo!()
    }
}
