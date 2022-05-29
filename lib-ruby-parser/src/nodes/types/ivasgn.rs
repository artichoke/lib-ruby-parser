// This file is autogenerated by codegen/rust/node_file.liquid

use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::Loc;
use crate::Node;

/// Represents instance variable assignment (i.e `@foo = 42`)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Ivasgn {
    /// Name of the instance variable, `String("@foo")` in `@foo = 42`
    pub name: String,

    /// Value that is assigned to instance variable.
    ///
    /// `None` if instance variable assignment is a part of the multi-assignment.
    /// In such case `value` is a part of the `Masgn` node.
    pub value: Option<Box<Node>>,

    /// Location of the instance variable name.
    ///
    /// ```text
    /// @foo = 1
    /// ~~~~
    /// ```
    pub name_l: Loc,

    /// Location of the `=` operator.
    ///
    /// ```text
    /// @foo = 1
    ///      ~
    /// ```
    ///
    /// `None` if instance variable assignment is a part of the multi-assignment.
    /// In such case `value` is a part of the `Masgn` node.
    pub operator_l: Option<Loc>,

    /// Location of the full expression
    ///
    /// ```text
    /// @foo = 42
    /// ~~~~~~~~~
    /// ```
    pub expression_l: Loc,

}

impl InnerNode for Ivasgn {
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
        "ivasgn"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        if let Some(node) = self.value.as_ref() { node.inner_ref().print_with_locs() }
        self.name_l.print("name");
        if let Some(loc) = self.operator_l.as_ref() { loc.print("operator") }
        self.expression_l.print("expression");
        
    }
}
