// This file is autogenerated by codegen/rust/node_file.liquid

use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::Loc;
use crate::Node;

/// Represents a wildcard pattern used in pattern matching (i.e. `in *foo`)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MatchRest {
    /// Name of the variable name
    ///
    /// `None` if there's no name (i.e. `in *`)
    pub name: Option<Box<Node>>,

    /// Location of the `*` operator
    ///
    /// ```text
    /// case foo; in *bar; end
    ///              ~
    /// ```
    pub operator_l: Loc,

    /// Location of the `*` operator
    ///
    /// ```text
    /// case foo; in *bar; end
    ///              ~~~~
    /// ```
    pub expression_l: Loc,

}

impl InnerNode for MatchRest {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_maybe_node(&self.name);
        
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "match_rest"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        if let Some(node) = self.name.as_ref() { node.inner_ref().print_with_locs() }
        self.operator_l.print("operator");
        self.expression_l.print("expression");
        
    }
}
