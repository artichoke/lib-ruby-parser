// This file is autogenerated by codegen/rust/node_file.liquid

use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::Loc;
use crate::Node;

/// Represents a hash pattern used in pattern matching (i.e. `in { a: 1 }`)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct HashPattern {
    /// A list of inner patterns
    pub elements: Vec<Node>,

    /// Location of the open parenthesis
    ///
    /// ```text
    /// case foo; in { a: 1 }; end
    ///              ~
    /// ```
    ///
    /// `None` if there are no parentheses
    pub begin_l: Option<Loc>,

    /// Location of the open parenthesis
    ///
    /// ```text
    /// case foo; in { a: 1 }; end
    ///                     ~
    /// ```
    ///
    /// `None` if there are no parentheses
    pub end_l: Option<Loc>,

    /// Location of the full expression
    ///
    /// ```text
    /// case foo; in { a: 1 }; end
    ///              ~~~~~~~~
    /// ```
    pub expression_l: Loc,

}

impl InnerNode for HashPattern {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_nodes(&self.elements);
        
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "hash_pattern"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        for node in self.elements.iter() { node.inner_ref().print_with_locs(); }
        if let Some(loc) = self.begin_l.as_ref() { loc.print("begin") }
        if let Some(loc) = self.end_l.as_ref() { loc.print("end") }
        self.expression_l.print("expression");
        
    }
}
