use crate::codegen::c::helpers;

fn contents() -> String {
    let nodes = lib_ruby_parser_nodes::nodes();

    format!(
        "// This file is autogenerated by {generator}

#include \"nodes.h\"
#include \"impl_blob.h\"
#include <stdio.h>
#include <stdlib.h>

{impl_blobs}
IMPL_BLOB(Node);
IMPL_BLOB(NodeList);

// drop variant fns
{drop_variant_fns}

void drop_node(Node *node)
{{
    {drop_node}
}}

void drop_maybe_node_ptr(Node **node)
{{
    Node *ptr = *node;
    if (ptr) {{
        drop_node(ptr);
        free(ptr);
    }}
}}

void drop_node_ptr(Node **node)
{{
    Node *ptr = *node;
    drop_node(ptr);
    free(ptr);
}}

void lib_ruby_parser__internal__containers__list__of_nodes__drop(NodeList *);
void drop_node_list(NodeList *node_list)
{{
    lib_ruby_parser__internal__containers__list__of_nodes__drop(node_list);
}}
",
        generator = file!(),
        impl_blobs = nodes.map(&impl_blob).join("\n"),
        drop_variant_fns = nodes.map(&drop_variant_fn).join("\n"),
        drop_node = drop_node(&nodes)
    )
}

pub(crate) fn codegen() {
    std::fs::write("external/c/nodes.c", contents()).unwrap();
}

fn impl_blob(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "IMPL_BLOB({struct_name});",
        struct_name = node.camelcase_name
    )
}

fn drop_variant_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    let drop_fields = node
        .fields
        .map(&|field| {
            let fn_name = match field.field_type {
                lib_ruby_parser_nodes::NodeFieldType::Node => "drop_node_ptr",
                lib_ruby_parser_nodes::NodeFieldType::Nodes => "drop_node_list",
                lib_ruby_parser_nodes::NodeFieldType::MaybeNode { .. } => "drop_maybe_node_ptr",
                lib_ruby_parser_nodes::NodeFieldType::Loc => "drop_loc",
                lib_ruby_parser_nodes::NodeFieldType::MaybeLoc => "drop_maybe_loc",

                lib_ruby_parser_nodes::NodeFieldType::Str { .. } => "drop_string_ptr",

                lib_ruby_parser_nodes::NodeFieldType::MaybeStr { .. } => "drop_maybe_string_ptr",
                lib_ruby_parser_nodes::NodeFieldType::StringValue => "drop_bytes",
                lib_ruby_parser_nodes::NodeFieldType::U8 => "drop_byte",
            };

            format!(
                "{fn_name}(&(variant->{field_name}));",
                fn_name = fn_name,
                field_name = helpers::nodes::fields::field_name(field)
            )
        })
        .join("\n    ");

    format!(
        "void drop_node_{lower}({struct_name} *variant) {{
    {drop_fields}
}}",
        struct_name = node.camelcase_name,
        drop_fields = drop_fields,
        lower = node.lower_name()
    )
}

fn drop_node(nodes: &lib_ruby_parser_nodes::NodeList) -> String {
    let branches = nodes
        .map(&|node| {
            format!(
                "case {tag_name}:
            drop_node_{lower}(&(node->as.{member_name}));
            break;",
                tag_name = helpers::nodes::enum_variant_name(node),
                lower = node.lower_name(),
                member_name = helpers::nodes::union_member_name(node)
            )
        })
        .join("\n        ");

    format!(
        "switch (node->tag) {{
        {branches}
    }}",
        branches = branches
    )
}
