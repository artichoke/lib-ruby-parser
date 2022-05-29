// This file is autogenerated by codegen/rust/node_file.liquid

use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::Loc;
use crate::Node;

/// Represents an `unless` guard used in pattern matching (i.e. `in pattern unless guard`)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct UnlessGuard {
    /// Condition of the guard, `Lvar("foo")` in `in pattern unless guard`
    pub cond: Box<Node>,

    /// Location of the `unless` keyword
    ///
    /// ```text
    /// case foo; in pattern unless cond; end
    ///                      ~~~~~~
    /// ```
    pub keyword_l: Loc,

    /// Location of the full expression
    ///
    /// ```text
    /// case foo; in pattern unless cond; end
    ///                      ~~~~~~~~~~~
    /// ```
    pub expression_l: Loc,

}

impl InnerNode for UnlessGuard {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.cond);
        
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "unless_guard"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.cond.inner_ref().print_with_locs();
        self.keyword_l.print("keyword");
        self.expression_l.print("expression");
        
    }
}
