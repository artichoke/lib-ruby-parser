// This file is autogenerated by codegen/rust/node_file.liquid

use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::Loc;
use crate::Node;

/// Represents a `case` statement (for pattern matching see `CaseMatch` node)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Case {
    /// Expression given to `case`, `Int("1")` for `case 1; end`
    /// `None` for code like
    ///
    /// ```text
    /// case
    /// when pattern
    /// end
    /// ```
    pub expr: Option<Box<Node>>,

    /// A list of `When` nodes (each has `patterns` and `body`)
    pub when_bodies: Vec<Node>,

    /// Body of the `else` branch, `None` if there's no `else` branch
    pub else_body: Option<Box<Node>>,

    /// Location of the `case` keyword
    ///
    /// ```text
    /// case 1; end
    /// ~~~~
    /// ```
    pub keyword_l: Loc,

    /// Location of the `else` keyword
    ///
    /// ```text
    /// case 1; else; end
    ///         ~~~~
    /// ```
    ///
    /// `None` if there's no `else` branch
    pub else_l: Option<Loc>,

    /// Location of the `end` keyword
    ///
    /// ```text
    /// case 1; end
    ///         ~~~
    /// ```
    pub end_l: Loc,

    /// Location of the full expression
    ///
    /// ```text
    /// case 1; end
    /// ~~~~~~~~~~~
    /// ```
    pub expression_l: Loc,

}

impl InnerNode for Case {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_maybe_node_or_nil(&self.expr);
        result.push_nodes(&self.when_bodies);
        result.push_maybe_node_or_nil(&self.else_body);
        
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "case"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        if let Some(node) = self.expr.as_ref() { node.inner_ref().print_with_locs() }
        for node in self.when_bodies.iter() { node.inner_ref().print_with_locs(); }
        if let Some(node) = self.else_body.as_ref() { node.inner_ref().print_with_locs() }
        self.keyword_l.print("keyword");
        if let Some(loc) = self.else_l.as_ref() { loc.print("else") }
        self.end_l.print("end");
        self.expression_l.print("expression");
        
    }
}
