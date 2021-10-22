use lib_ruby_parser_nodes::template::*;

const TEMPLATE: &str = "// This file is auto-generated by {{ helper generated-by }}

use super::variants::*;

/// Enum of all possible diagnostic message (both warnings and errors)
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DiagnosticMessage<'a> {
{{ each message }}<dnl>
    {{ helper message-camelcase-name }}({{ helper message-camelcase-name }}{{ helper message-generic-lifetime }}),
{{ end }}<dnl>
}
";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let fns = crate::codegen::fns::default_fns!();

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("src/error/message/native/enum_.rs", contents).unwrap();
}
