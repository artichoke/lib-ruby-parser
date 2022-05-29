// This file is autogenerated by codegen/rust/node_file.liquid

use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::Loc;
use crate::Node;

/// Represents ternary `if` statement (i.e. `cond ? if_true : if_false`)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct IfTernary {
    /// Condition of the `if` statement
    pub cond: Box<Node>,

    /// True-branch
    pub if_true: Box<Node>,

    /// True-branch
    pub if_false: Box<Node>,

    /// Location of the `?` operator
    ///
    /// ```text
    /// cond ? if_true : if_false
    ///      ~
    /// ```
    pub question_l: Loc,

    /// Location of the `:` operator
    ///
    /// ```text
    /// cond ? if_true : if_false
    ///                ~
    /// ```
    pub colon_l: Loc,

    /// Location of the full expression
    ///
    /// ```text
    /// cond ? if_true : if_false
    /// ~~~~~~~~~~~~~~~~~~~~~~~~~
    /// ```
    pub expression_l: Loc,

}

impl InnerNode for IfTernary {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.cond);
        result.push_node(&self.if_true);
        result.push_node(&self.if_false);
        
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "if"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.cond.inner_ref().print_with_locs();
        self.if_true.inner_ref().print_with_locs();
        self.if_false.inner_ref().print_with_locs();
        self.question_l.print("question");
        self.colon_l.print("colon");
        self.expression_l.print("expression");
        
    }
}
