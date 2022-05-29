// This file is autogenerated by codegen/rust/node_file.liquid

use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::Loc;
use crate::Node;

/// Represents an array pattern used in pattern matching
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ArrayPattern {
    /// A list of elements
    pub elements: Vec<Node>,

    /// Location of the open bracket
    ///
    /// ```text
    /// [1, ^a, 3 => foo]
    /// ~
    /// ```
    ///
    /// `None` for pattern like `1, 2` without brackets
    pub begin_l: Option<Loc>,

    /// Location of the closing bracket
    ///
    /// ```text
    /// [1, ^a, 3 => foo]
    ///                 ~
    /// ```
    ///
    /// `None` for pattern like `1, 2` without brackets
    pub end_l: Option<Loc>,

    /// Location of the full expression
    ///
    /// ```text
    /// [1, ^a, 3 => foo]
    /// ~~~~~~~~~~~~~~~~~
    /// ```
    pub expression_l: Loc,

}

impl InnerNode for ArrayPattern {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_nodes(&self.elements);
        
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "array_pattern"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        for node in self.elements.iter() { node.inner_ref().print_with_locs(); }
        if let Some(loc) = self.begin_l.as_ref() { loc.print("begin") }
        if let Some(loc) = self.end_l.as_ref() { loc.print("end") }
        self.expression_l.print("expression");
        
    }
}
