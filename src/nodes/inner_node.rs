crate::use_native_or_external!(Maybe);
crate::use_native_or_external!(Ptr);
// crate::use_native_or_external!(String);
// crate::use_native_or_external!(List);

use crate::Bytes;
use crate::Loc;
use crate::Node;

pub trait InnerNode<'a>: std::fmt::Debug {
    fn expression(&'a self) -> &Loc;
    fn str_type(&'a self) -> &'static str;
    fn inspected_children(&'a self, indent: usize) -> Vec<String>;

    fn inspect(&'a self, indent: usize) -> String {
        let indented = "  ".repeat(indent);
        let mut sexp = format!("{}s(:{}", indented, self.str_type());

        for child in self.inspected_children(indent) {
            sexp.push_str(&child);
        }

        sexp.push(')');

        sexp
    }

    fn print_with_locs(&'a self);
}

pub(crate) struct InspectVec {
    indent: usize,
    strings: Vec<String>,
}

impl InspectVec {
    pub(crate) fn new(indent: usize) -> Self {
        Self {
            indent,
            strings: vec![],
        }
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        self.strings.push(format!(", {:?}", string));
    }

    pub(crate) fn push_raw_str(&mut self, string: &str) {
        self.strings.push(format!(", {}", string));
    }

    pub(crate) fn push_maybe_str(&mut self, string: &Maybe<bumpalo::collections::String>) {
        if let Some(string) = string.as_ref() {
            self.strings.push(format!(", {:?}", string));
        }
    }

    pub(crate) fn push_nil(&mut self) {
        self.strings.push(", nil".to_string());
    }

    pub(crate) fn push_u8(&mut self, n: &u8) {
        self.strings.push(format!(", {}", n))
    }

    pub(crate) fn push_node<'a>(&mut self, node: &'a Node<'a>) {
        self.strings
            .push(format!(",\n{}", node.inspect(self.indent + 1)))
    }

    pub(crate) fn push_maybe_node<'a>(&mut self, node: &'a Maybe<&'a mut Node<'a>>) {
        if let Some(node) = node.as_ref() {
            self.push_node(node)
        }
    }

    pub(crate) fn push_regex_options<'a>(&mut self, node: &'a Maybe<&'a mut Node<'a>>) {
        if let Some(node) = node.as_ref() {
            self.push_node(node)
        } else {
            self.strings.push(format!(
                ",\n{}{}",
                "  ".repeat(self.indent + 1),
                "s(:regopt)"
            ))
        }
    }

    pub(crate) fn push_maybe_node_or_nil<'a>(&mut self, node: &'a Maybe<&'a mut Node<'a>>) {
        if let Some(node) = node.as_ref() {
            self.push_node(node)
        } else {
            self.push_nil()
        }
    }

    pub(crate) fn push_nodes<'a>(
        &mut self,
        nodes: &'a bumpalo::collections::Vec<&'a mut Node<'a>>,
    ) {
        for node in nodes.iter() {
            self.push_node(node)
        }
    }

    pub(crate) fn push_chars(&mut self, chars: &Maybe<bumpalo::collections::String>) {
        if let Some(chars) = chars.as_ref() {
            for c in chars.as_str().chars() {
                self.push_str(&String::from(format!("{}", c)));
            }
        }
    }

    pub(crate) fn push_string_value(&mut self, bytes: &Bytes) {
        self.push_str(&bytes.to_string_lossy())
    }

    pub(crate) fn strings(&mut self) -> Vec<String> {
        std::mem::take(&mut self.strings)
    }
}
