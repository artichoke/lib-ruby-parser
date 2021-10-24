crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(Vec);
crate::use_native_or_external!(Maybe);

use crate::builder::{ArgsType, PKwLabel};
use crate::str_term::StrTerm;
use crate::Node;
use crate::Token;

impl<'a> Node<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a Node<'a> {
        match value {
            ParseValue::Node(value) => value,
            other => unreachable!("expected Node, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod BoxedNode {
    use super::{Node, ParseValue, Ptr};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> &'a Node {
        match value {
            ParseValue::Node(value) => value,
            other => unreachable!("expected BoxedNode, got {:?}", other),
        }
    }
}

impl<'a> Token<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a Token {
        match value {
            ParseValue::Token(value) => value,
            other => unreachable!("expected Token<'a>, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod NodeList {
    use super::{Node, ParseValue, Vec};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Vec<'a, &'a Node<'a>> {
        match value {
            ParseValue::NodeList(value) => value,
            other => unreachable!("expected NodeList, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod Bool {
    use super::ParseValue;

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> bool {
        match value {
            ParseValue::Bool(value) => value,
            other => unreachable!("expected Bool, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeStrTerm {
    use super::{ParseValue, StrTerm};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Option<&'a StrTerm<'a>> {
        match value {
            ParseValue::MaybeStrTerm(value) => value,
            other => unreachable!("expected MaybeStrTerm, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod Num {
    use super::ParseValue;

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> i32 {
        match value {
            ParseValue::Num(value) => value,
            other => unreachable!("expected Num, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Superclass<'a> {
    pub(crate) lt_t: Maybe<&'a Token<'a>>,
    pub(crate) value: Maybe<&'a Node<'a>>,
}
impl<'a> Superclass<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a Superclass<'a> {
        match value {
            ParseValue::Superclass(value) => value,
            other => unreachable!("expected Superclass, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Ensure<'a> {
    pub(crate) ensure_t: &'a Token<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
}
#[allow(non_snake_case)]
pub(crate) mod OptEnsure {
    use super::{Ensure, ParseValue};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Option<&'a Ensure<'a>> {
        match value {
            ParseValue::OptEnsure(value) => value,
            other => unreachable!("expected OptEnsure, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Else<'a> {
    pub(crate) else_t: &'a Token<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
}
#[allow(non_snake_case)]
pub(crate) mod OptElse {
    use super::{Else, ParseValue};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Option<&'a Else<'a>> {
        match value {
            ParseValue::OptElse(value) => value,
            other => unreachable!("expected OptElse, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ExcVar<'a> {
    pub(crate) assoc_t: Maybe<&'a Token<'a>>,
    pub(crate) exc_var: Maybe<&'a Node<'a>>,
}
impl<'a> ExcVar<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a ExcVar<'a> {
        match value {
            ParseValue::ExcVar(value) => value,
            other => unreachable!("expected ExcVar, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct IfTail<'a> {
    pub(crate) keyword_t: Maybe<&'a Token<'a>>,
    pub(crate) body: Maybe<&'a Node<'a>>,
}
impl<'a> IfTail<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a IfTail<'a> {
        match value {
            ParseValue::IfTail(value) => value,
            other => unreachable!("expected IfTail, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ExprValueDo<'a> {
    pub(crate) value: &'a Node<'a>,
    pub(crate) do_t: &'a Token<'a>,
}
impl<'a> ExprValueDo<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a ExprValueDo<'a> {
        match value {
            ParseValue::ExprValueDo(value) => value,
            other => unreachable!("expected ExprValueDo, got {:?}", other),
        }
    }
}

impl<'a> PKwLabel<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a PKwLabel<'a> {
        match value {
            ParseValue::PKwLabel(value) => value,
            other => unreachable!("expected PKwLabel, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BraceBody<'a> {
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
}
impl<'a> BraceBody<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a BraceBody<'a> {
        match value {
            ParseValue::BraceBody(value) => value,
            other => unreachable!("expected BraceBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CmdBraceBlock<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> CmdBraceBlock<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a CmdBraceBlock<'a> {
        match value {
            ParseValue::CmdBraceBlock(value) => value,
            other => unreachable!("expected CmdBraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ParenArgs<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) args: Vec<'a, &'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> ParenArgs<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a ParenArgs<'a> {
        match value {
            ParseValue::ParenArgs(value) => value,
            other => unreachable!("expected ParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct OptParenArgs<'a> {
    pub(crate) begin_t: Maybe<&'a Token<'a>>,
    pub(crate) args: Vec<'a, &'a Node<'a>>,
    pub(crate) end_t: Maybe<&'a Token<'a>>,
}
impl<'a> OptParenArgs<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a OptParenArgs {
        match value {
            ParseValue::OptParenArgs(value) => value,
            other => unreachable!("expected OptParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BeginBlock<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> BeginBlock<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a BeginBlock {
        match value {
            ParseValue::BeginBlock(value) => value,
            other => unreachable!("expected BeginBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LambdaBody<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> LambdaBody<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a LambdaBody<'a> {
        match value {
            ParseValue::LambdaBody(value) => value,
            other => unreachable!("expected LambdaBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DoBlock<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> DoBlock<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a DoBlock<'a> {
        match value {
            ParseValue::DoBlock(value) => value,
            other => unreachable!("expected DoBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BraceBlock<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> BraceBlock<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a BraceBlock<'a> {
        match value {
            ParseValue::BraceBlock(value) => value,
            other => unreachable!("expected BraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DefsHead<'a> {
    pub(crate) def_t: &'a Token<'a>,
    pub(crate) definee: &'a Node<'a>,
    pub(crate) dot_t: &'a Token<'a>,
    pub(crate) name_t: &'a Token<'a>,
}
impl<'a> DefsHead<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a DefsHead<'a> {
        match value {
            ParseValue::DefsHead(value) => value,
            other => unreachable!("expected DefsHead, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DefnHead<'a> {
    pub(crate) def_t: &'a Token<'a>,
    pub(crate) name_t: &'a Token<'a>,
}
impl<'a> DefnHead<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a DefnHead<'a> {
        match value {
            ParseValue::DefnHead(value) => value,
            other => unreachable!("expected DefnHead, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Cases<'a> {
    pub(crate) when_bodies: Vec<'a, &'a Node<'a>>,
    pub(crate) opt_else: Option<&'a Else<'a>>,
}
impl<'a> Cases<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a Cases<'a> {
        match value {
            ParseValue::Cases(value) => value,
            other => unreachable!("expected Cases, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CaseBody<'a> {
    pub(crate) when_bodies: Vec<'a, &'a Node<'a>>,
    pub(crate) opt_else: Option<&'a Else<'a>>,
}
impl<'a> CaseBody<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a CaseBody<'a> {
        match value {
            ParseValue::CaseBody(value) => value,
            other => unreachable!("expected CaseBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PCases<'a> {
    pub(crate) in_bodies: Vec<'a, &'a Node<'a>>,
    pub(crate) opt_else: Option<&'a Else<'a>>,
}
impl<'a> PCases<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a PCases<'a> {
        match value {
            ParseValue::PCases(value) => value,
            other => unreachable!("expected PCases, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PCaseBody<'a> {
    pub(crate) in_bodies: Vec<'a, &'a Node<'a>>,
    pub(crate) opt_else: Option<&'a Else<'a>>,
}
impl<'a> PCaseBody<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a PCaseBody<'a> {
        match value {
            ParseValue::PCaseBody(value) => value,
            other => unreachable!("expected PCaseBody, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeNode {
    use super::{Node, ParseValue, PtrAPI};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Option<&'a Node<'a>> {
        match value {
            ParseValue::MaybeNode(value) => value,
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeBoxedNode {
    use super::{Maybe, Node, ParseValue, Ptr};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Maybe<&'a Node<'a>> {
        match value {
            ParseValue::MaybeNode(value) => value,
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DoBody<'a> {
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
}
impl<'a> DoBody<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a DoBody<'a> {
        match value {
            ParseValue::DoBody(value) => value,
            other => unreachable!("expected DoBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PTopExpr<'a> {
    pub(crate) pattern: &'a Node<'a>,
    pub(crate) guard: Maybe<&'a Node<'a>>,
}
impl<'a> PTopExpr<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a PTopExpr<'a> {
        match value {
            ParseValue::PTopExpr(value) => value,
            other => unreachable!("expected PTopExpr, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MatchPatternWithTrailingComma<'a> {
    pub(crate) elements: Vec<'a, &'a Node<'a>>,
    pub(crate) trailing_comma: Maybe<&'a Token<'a>>,
}
impl<'a> MatchPatternWithTrailingComma<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a MatchPatternWithTrailingComma<'a> {
        match value {
            ParseValue::MatchPatternWithTrailingComma(value) => value,
            other => unreachable!("expected MatchPatternWithTrailingComma, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ParseValue<'a> {
    Stolen,
    Uninitialized,
    None,
    Token(&'a Token<'a>),
    TokenList(Vec<'a, &'a Token<'a>>),
    Node(&'a Node<'a>),
    NodeList(Vec<'a, &'a Node<'a>>),
    Bool(bool),
    MaybeStrTerm(Option<&'a StrTerm<'a>>),
    Num(i32),

    /* For custom superclass rule */
    Superclass(&'a Superclass<'a>),

    /* For custom opt_ensure rule */
    OptEnsure(Option<&'a Ensure<'a>>),

    /* For custom opt_else rule */
    OptElse(Option<&'a Else<'a>>),

    /* For custom exc_var rule */
    ExcVar(&'a ExcVar<'a>),

    /* For custom if_tail rule */
    IfTail(&'a IfTail<'a>),

    /* For custom expr_value_do rule */
    ExprValueDo(&'a ExprValueDo<'a>),

    /* For custom p_kw_label rule */
    PKwLabel(&'a PKwLabel<'a>),

    /* For custom brace_body rule */
    BraceBody(&'a BraceBody<'a>),

    /* For custom cmd_brace_block rule */
    CmdBraceBlock(&'a CmdBraceBlock<'a>),

    /* For custom paren_args rule  */
    ParenArgs(&'a ParenArgs<'a>),

    /* For custom opt_paren_args rule  */
    OptParenArgs(&'a OptParenArgs<'a>),

    /* For custom lambda_body rule  */
    LambdaBody(&'a LambdaBody<'a>),

    /* For custom do_block rule  */
    DoBlock(&'a DoBlock<'a>),

    /* For custom brace_block rule  */
    BraceBlock(&'a BraceBlock<'a>),

    /* For custom defs_head rule */
    DefsHead(&'a DefsHead<'a>),

    /* For custom defn_head rule */
    DefnHead(&'a DefnHead<'a>),

    /* For custom begin_block rule  */
    BeginBlock(&'a BeginBlock<'a>),

    /* For custom cases rule */
    Cases(&'a Cases<'a>),

    /* For custom case_body rule */
    CaseBody(&'a CaseBody<'a>),

    /* For custom p_cases rule */
    PCases(&'a PCases<'a>),

    /* For custom p_case_body rule */
    PCaseBody(&'a PCaseBody<'a>),

    /* For custom compstmt rule */
    MaybeNode(Maybe<&'a Node<'a>>),

    /* For custom do_body rule */
    DoBody(&'a DoBody<'a>),

    /* For custom p_top_expr rule */
    PTopExpr(&'a PTopExpr<'a>),

    /* For pattern matching patterns with trailing comma */
    MatchPatternWithTrailingComma(&'a MatchPatternWithTrailingComma<'a>),
}

impl<'a> ParseValue<'a> {
    pub(crate) fn from_token(token: &'a Token<'a>) -> Self {
        Self::Token(token)
    }

    pub(crate) fn new_superclass(value: &'a Superclass<'a>) -> Self {
        Self::Superclass(value)
    }
    pub(crate) fn new_opt_ensure(value: Option<&'a Ensure<'a>>) -> Self {
        Self::OptEnsure(value)
    }
    pub(crate) fn new_opt_else(value: Option<&'a Else<'a>>) -> Self {
        Self::OptElse(value)
    }
    pub(crate) fn new_exc_var(value: &'a ExcVar<'a>) -> Self {
        Self::ExcVar(value)
    }
    pub(crate) fn new_if_tail(value: &'a IfTail<'a>) -> Self {
        Self::IfTail(value)
    }
    pub(crate) fn new_expr_value_do(value: &'a ExprValueDo<'a>) -> Self {
        Self::ExprValueDo(value)
    }
    pub(crate) fn new_p_kw_label(value: &'a PKwLabel<'a>) -> Self {
        Self::PKwLabel(value)
    }
    pub(crate) fn new_brace_body(value: &'a BraceBody<'a>) -> Self {
        Self::BraceBody(value)
    }
    pub(crate) fn new_cmd_brace_block(value: &'a CmdBraceBlock<'a>) -> Self {
        Self::CmdBraceBlock(value)
    }
    pub(crate) fn new_paren_args(value: &'a ParenArgs<'a>) -> Self {
        Self::ParenArgs(value)
    }
    pub(crate) fn new_opt_paren_args(value: &'a OptParenArgs<'a>) -> Self {
        Self::OptParenArgs(value)
    }
    pub(crate) fn new_lambda_body(value: &'a LambdaBody<'a>) -> Self {
        Self::LambdaBody(value)
    }
    pub(crate) fn new_do_block(value: &'a DoBlock<'a>) -> Self {
        Self::DoBlock(value)
    }
    pub(crate) fn new_brace_block(value: &'a BraceBlock<'a>) -> Self {
        Self::BraceBlock(value)
    }
    pub(crate) fn new_defs_head(value: &'a DefsHead<'a>) -> Self {
        Self::DefsHead(value)
    }
    pub(crate) fn new_defn_head(value: &'a DefnHead<'a>) -> Self {
        Self::DefnHead(value)
    }
    pub(crate) fn new_begin_block(value: &'a BeginBlock<'a>) -> Self {
        Self::BeginBlock(value)
    }
    pub(crate) fn new_cases(value: &'a Cases<'a>) -> Self {
        Self::Cases(value)
    }
    pub(crate) fn new_case_body(value: &'a CaseBody<'a>) -> Self {
        Self::CaseBody(value)
    }
    pub(crate) fn new_p_cases(value: &'a PCases<'a>) -> Self {
        Self::PCases(value)
    }
    pub(crate) fn new_p_case_body(value: &'a PCaseBody<'a>) -> Self {
        Self::PCaseBody(value)
    }
    pub(crate) fn new_do_body(value: &'a DoBody<'a>) -> Self {
        Self::DoBody(value)
    }
    pub(crate) fn new_p_top_expr(value: &'a PTopExpr<'a>) -> Self {
        Self::PTopExpr(value)
    }
    pub(crate) fn new_match_pattern_with_trailing_comma(
        value: &'a MatchPatternWithTrailingComma<'a>,
    ) -> Self {
        Self::MatchPatternWithTrailingComma(value)
    }
    pub(crate) fn new_none() -> Self {
        Self::None
    }
    pub(crate) fn new_node(node: &'a Node<'a>) -> Self {
        Self::Node(node)
    }
    pub(crate) fn new_maybe_node(maybe_node: Option<&'a Node<'a>>) -> Self {
        Self::MaybeNode(maybe_node)
    }
    pub(crate) fn new_node_list(node_list: Vec<'a, &'a Node<'a>>) -> Self {
        Self::NodeList(node_list)
    }
    pub(crate) fn new_bool(value: bool) -> Self {
        Self::Bool(value)
    }
    pub(crate) fn new_num(value: i32) -> Self {
        Self::Num(value)
    }
    pub(crate) fn new_maybe_str_term(maybe_str_term: Option<&'a StrTerm<'a>>) -> Self {
        Self::MaybeStrTerm(maybe_str_term)
    }
    pub(crate) fn new_token(token: &'a Token<'a>) -> Self {
        Self::Token(token)
    }
    pub(crate) fn new_token_list(token_list: Vec<'a, &'a Token<'a>>) -> Self {
        Self::TokenList(token_list)
    }
}

impl Default for ParseValue<'_> {
    fn default() -> Self {
        Self::Stolen
    }
}

#[test]
fn test_parse_value_size() {
    let size = std::mem::size_of::<ParseValue>();
    println!("size = {}", size);

    println!(
        "sizeof vec = {}",
        std::mem::size_of::<Vec<'_, &'_ Token<'_>>>()
    );

    println!(
        "sizeof maybe(strterm) = {}",
        std::mem::size_of::<Option<StrTerm<'_>>>()
    );
    assert_eq!(size, 42);
}
