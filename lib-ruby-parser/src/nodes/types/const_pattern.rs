// This file is autogenerated by codegen/rust/node_file.liquid

use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::Loc;
use crate::Node;

/// Const pattern used in pattern matching (e.g. `in A(1, 2)`)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ConstPattern {
    /// Constant that is used, `Const("Foo")` for `in For(42)`
    pub const_: Box<Node>,

    /// Inner part of the constant pattern
    ///
    /// `ArrayPattern(vec![ Int("1") ])` for `Foo(1)`
    pub pattern: Box<Node>,

    /// Location of the open parenthesis
    ///
    /// ```text
    /// case 1; in Foo(42); end
    ///               ~
    /// ```
    pub begin_l: Loc,

    /// Location of the closing parenthesis
    ///
    /// ```text
    /// case 1; in Foo(42); end
    ///                  ~
    /// ```
    pub end_l: Loc,

    /// Location of the full expression
    ///
    /// ```text
    /// case 1; in Foo(42); end
    ///            ~~~~~~~
    /// ```
    pub expression_l: Loc,

}

impl InnerNode for ConstPattern {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.const_);
        result.push_node(&self.pattern);
        
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "const_pattern"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.const_.inner_ref().print_with_locs();
        self.pattern.inner_ref().print_with_locs();
        self.begin_l.print("begin");
        self.end_l.print("end");
        self.expression_l.print("expression");
        
    }
}
