use lib_ruby_parser_nodes::{template::*, NodeField};

const TEMPLATE: &str = "// This file is auto-generated by {{ helper generated-by }}

use crate::nodes::*;
use crate::Node;

/// Common trait for all visitors
pub trait Visitor: Sized {
{{ each node }}<dnl>
    /// Invoked by a `Visitor` on entering into `{{ helper node-camelcase-name }}` node.
    fn on_{{ helper node-lower-name }}(&mut self, node: &{{ helper node-camelcase-name }}) {
        visit_{{ helper node-lower-name }}(self, node);
    }
{{ end }}

    /// Generic `visit` router that calls `on_<type>` under the hood
    fn visit(&mut self, node: &Node) {
        match node {
{{ each node }}<dnl>
            Node::{{ helper node-camelcase-name }}(inner) => {
                self.on_{{ helper node-lower-name }}(inner);
            }
{{ end }}
        }
    }
}

{{ each node }}<dnl>
/// Visits all children of {{ helper node-camelcase-name }} node
#[allow(unused_variables)]
pub fn visit_{{ helper node-lower-name }}<V: Visitor>(visitor: &mut V, node: &{{ helper node-camelcase-name }}) {
{{ each node-field }}<dnl>
    {{ helper visit-child }}
{{ end }}<dnl>
}
{{ end }}
";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let mut fns = crate::codegen::fns::default_fns!();

    fns.register::<NodeField, F::Helper>("visit-child", local_helpers::visit_child);

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("src/traverse/visitor/visit_gen.rs", contents).unwrap();
}

mod local_helpers {
    use lib_ruby_parser_nodes::NodeField;

    pub(crate) fn visit_child(node_field: &NodeField) -> String {
        let field_name = crate::codegen::fns::rust::node_fields::rust_field_name(node_field);

        use lib_ruby_parser_nodes::NodeFieldType::*;
        match node_field.field_type {
            Node => {
                format!("visitor.visit(&node.{});", field_name)
            }
            Nodes => {
                format!(
                    "for item in &node.{} {{ visitor.visit(item); }}",
                    field_name
                )
            }
            MaybeNode { .. } => {
                format!(
                    "if let Some(inner) = node.{}.as_ref() {{ visitor.visit(inner); }}",
                    field_name
                )
            }

            Loc | MaybeLoc | Str { .. } | MaybeStr { .. } | StringValue | U8 => {
                return format!("// skip {}", field_name)
            }
        }
    }
}
