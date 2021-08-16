use crate::codegen::rust::nodes::helpers::node_field_name;

fn contents() -> String {
    format!(
        "// This file is autogenerated by {generator}

use super::LocName;
use lib_ruby_parser::Node;

#[cfg(feature = \"compile-with-external-structures\")]
use lib_ruby_parser::containers::ExternalMaybeLoc;
#[cfg(feature = \"compile-with-external-structures\")]
type MaybeLoc = ExternalMaybeLoc;
#[cfg(not(feature = \"compile-with-external-structures\"))]
use lib_ruby_parser::Loc;
#[cfg(not(feature = \"compile-with-external-structures\"))]
type MaybeLoc = Option<Loc>;

impl LocName {{
    {loc_getters}

    pub(crate) fn get(&self, node: &Node) -> MaybeLoc {{
        match self {{
            {loc_branches}
        }}
    }}
}}
",
        generator = file!(),
        loc_getters = map_loc(&loc_getter).join("\n\n    "),
        loc_branches = map_loc(&loc_branch).join("\n            ")
    )
}

pub(crate) fn codegen() {
    std::fs::write("tests/loc_matcher/loc_name_gen.rs", contents()).unwrap();
}

#[derive(Debug)]
pub enum LocName {
    Begin,
    End,
    Expression,
    Keyword,
    Name,
    Assignment,
    Colon,
    DoubleColon,
    Else,
    HeredocBody,
    Operator,
    Selector,
    Assoc,
    Question,
    HeredocEnd,
}

impl LocName {
    fn to_str(&self) -> &'static str {
        match self {
            LocName::Begin => "begin_l",
            LocName::End => "end_l",
            LocName::Expression => "expression_l",
            LocName::Keyword => "keyword_l",
            LocName::Name => "name_l",
            LocName::Assignment => "assignment_l",
            LocName::Colon => "colon_l",
            LocName::DoubleColon => "double_colon_l",
            LocName::Else => "else_l",
            LocName::HeredocBody => "heredoc_body_l",
            LocName::Operator => "operator_l",
            LocName::Selector => "selector_l",
            LocName::Assoc => "assoc_l",
            LocName::Question => "question_l",
            LocName::HeredocEnd => "heredoc_end_l",
        }
    }
}

const LOC_NAMES: &[&'static LocName] = &[
    &LocName::Begin,
    &LocName::End,
    &LocName::Expression,
    &LocName::Keyword,
    &LocName::Name,
    &LocName::Assignment,
    &LocName::Colon,
    &LocName::DoubleColon,
    &LocName::Else,
    &LocName::HeredocBody,
    &LocName::Operator,
    &LocName::Selector,
    &LocName::Assoc,
    &LocName::Question,
    &LocName::HeredocEnd,
];

fn map_loc(f: &dyn Fn(&LocName) -> String) -> Vec<String> {
    LOC_NAMES.iter().map(|l| f(*l)).collect()
}

fn loc_getter(loc_name: &LocName) -> String {
    let mut variants = vec![];

    for node in lib_ruby_parser_nodes::nodes().0.iter() {
        for field in node.fields.0.iter() {
            if node_field_name(field) == loc_name.to_str() {
                match field.field_type {
                    lib_ruby_parser_nodes::NodeFieldType::Loc => {
                        variants.push((node.clone(), false))
                    }
                    lib_ruby_parser_nodes::NodeFieldType::MaybeLoc => {
                        variants.push((node.clone(), true))
                    }
                    _ => {}
                }
            }
        }
    }

    let statements: Vec<String> = variants
        .into_iter()
        .map(|(node, nullable)| {
            let get_loc = if nullable {
                format!(
                    "return inner.get_{loc_name}().clone()",
                    loc_name = loc_name.to_str()
                )
            } else {
                format!(
                    "return inner.get_{loc_name}().clone().into()",
                    loc_name = loc_name.to_str()
                )
            };

            format!(
                "if let Some(inner) = node.as_{lower_node_name}() {{
            {get_loc}
        }}",
                lower_node_name = node.lower_name(),
                get_loc = get_loc
            )
        })
        .collect();

    let statements = statements.join(" else ");

    format!(
        "fn get_{loc_name}(node: &Node) -> MaybeLoc {{
        {statements}
        else {{
            panic!(\"node {{}} doesn't support {loc_name} loc\", node.str_type())
        }}
    }}",
        loc_name = loc_name.to_str(),
        statements = statements
    )
}

fn loc_branch(loc_name: &LocName) -> String {
    let camelcase_loc_name = format!("{:?}", loc_name);
    format!(
        "LocName::{camelcase_loc_name} => Self::get_{loc_name}(node),",
        camelcase_loc_name = camelcase_loc_name,
        loc_name = loc_name.to_str()
    )
}
