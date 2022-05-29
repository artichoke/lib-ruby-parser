// This file is autogenerated by codegen/rust/node_file.liquid

use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::Loc;
use crate::Node;

/// Represents local variable assignment (i.e. `foo = 42`)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Lvasgn {
    /// Name of the local variable
    pub name: String,

    /// Value that is assigned to a local variable
    pub value: Option<Box<Node>>,

    /// Location of the local variable name
    ///
    /// ```text
    /// foo = 42
    /// ~~~
    /// ```
    pub name_l: Loc,

    /// Location of the `=` operator
    ///
    /// ```text
    /// foo = 42
    ///     ~
    /// ```
    ///
    /// `None` if local variable assignment is a part of the multi-assignment.
    /// In such case `value` is a part of the `Masgn` node.
    pub operator_l: Option<Loc>,

    /// Location of the full expression
    ///
    /// ```text
    /// foo = 42
    /// ~~~~~~~~
    /// ```
    pub expression_l: Loc,

}

impl InnerNode for Lvasgn {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_str(&self.name);
        result.push_maybe_node(&self.value);
        
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "lvasgn"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        if let Some(node) = self.value.as_ref() { node.inner_ref().print_with_locs() }
        self.name_l.print("name");
        if let Some(loc) = self.operator_l.as_ref() { loc.print("operator") }
        self.expression_l.print("expression");
        
    }
}
