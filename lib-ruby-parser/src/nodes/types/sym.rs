// This file is autogenerated by codegen/rust/node_file.liquid

use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::Loc;
use crate::Bytes;

/// Represents a plain symbol literal (i.e. `:foo`)
///
/// Note that `:` in `{ foo: bar }` belongs to a `pair` node.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Sym {
    /// Value of the symbol literal
    ///
    /// Note that it's a `StringValue`, not a `String`.
    /// The reason is that you can get UTF-8 incompatible strings
    /// from a valid UTF-8 source using escape sequences like `"\xFF"`
    ///
    /// These "\", "x", "F", "F" chars are valid separately, but together
    /// they construct a char with code = 255 that is invalid for UTF-8.
    ///
    /// You can use `to_string_lossy` or `to_string` methods to get a raw symbol value.
    pub name: Bytes,

    /// Location of the symbol begin
    ///
    /// ```text
    /// :foo
    /// ~
    /// ```
    ///
    /// `None` if symbol is a label (`{ foo: 1 }`) or a part of the symbols array (`%i[foo bar baz]`)
    pub begin_l: Option<Loc>,

    /// Location of the symbol end
    ///
    /// ```text
    /// { 'foo': 1 }
    ///        ~
    /// ```
    ///
    /// `None` if symbol is **not** a string label (`:foo`) or a part of the symbols array (`%i[foo bar baz]`)
    pub end_l: Option<Loc>,

    /// Location of the full expression
    ///
    /// ```text
    /// :foo
    /// ~~~~
    ///
    /// { foo: 1 }
    ///   ~~~~
    ///
    /// %i[foo]
    ///    ~~~
    /// ```
    pub expression_l: Loc,

}

impl InnerNode for Sym {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_string_value(&self.name);
        
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "sym"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        if let Some(loc) = self.begin_l.as_ref() { loc.print("begin") }
        if let Some(loc) = self.end_l.as_ref() { loc.print("end") }
        self.expression_l.print("expression");
        
    }
}
