crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(Vec);
crate::use_native_or_external!(Maybe);

use crate::builder::{ArgsType, PKwLabel};
use crate::str_term::StrTerm;
use crate::Node;
use crate::Token;

impl<'a> Node<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a mut Node<'a> {
        match value {
            ParseValue::Node(value) => value,
            other => unreachable!("expected Node, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod BoxedNode {
    use super::{Node, ParseValue, Ptr};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> &'a mut Node {
        match value {
            ParseValue::Node(value) => value,
            other => unreachable!("expected BoxedNode, got {:?}", other),
        }
    }
}

impl<'a> Token<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> &'a mut Token {
        match value {
            ParseValue::Token(value) => value,
            other => unreachable!("expected Token<'a>, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod NodeList {
    use super::{Node, ParseValue, Vec};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Vec<'a, &'a mut Node<'a>> {
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

#[derive(Debug)]
pub(crate) struct Superclass<'a> {
    pub(crate) lt_t: Maybe<&'a mut Token<'a>>,
    pub(crate) value: Maybe<&'a mut Node<'a>>,
}
impl<'a> Superclass<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> Superclass<'a> {
        match value {
            ParseValue::Superclass(value) => value,
            other => unreachable!("expected Superclass, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Ensure<'a> {
    pub(crate) ensure_t: &'a mut Token<'a>,
    pub(crate) body: Maybe<&'a mut Node<'a>>,
}
#[allow(non_snake_case)]
pub(crate) mod OptEnsure {
    use super::{Ensure, ParseValue};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Option<Ensure<'a>> {
        match value {
            ParseValue::OptEnsure(value) => value,
            other => unreachable!("expected OptEnsure, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Else<'a> {
    pub(crate) else_t: &'a mut Token<'a>,
    pub(crate) body: Maybe<&'a mut Node<'a>>,
}
#[allow(non_snake_case)]
pub(crate) mod OptElse {
    use super::{Else, ParseValue};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Option<Else<'a>> {
        match value {
            ParseValue::OptElse(value) => value,
            other => unreachable!("expected OptElse, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ExcVar<'a> {
    pub(crate) assoc_t: Maybe<&'a mut Token<'a>>,
    pub(crate) exc_var: Maybe<&'a mut Node<'a>>,
}
impl<'a> ExcVar<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> ExcVar<'a> {
        match value {
            ParseValue::ExcVar(value) => value,
            other => unreachable!("expected ExcVar, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct IfTail<'a> {
    pub(crate) keyword_t: Maybe<&'a mut Token<'a>>,
    pub(crate) body: Maybe<&'a mut Node<'a>>,
}
impl<'a> IfTail<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> IfTail<'a> {
        match value {
            ParseValue::IfTail(value) => value,
            other => unreachable!("expected IfTail, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ExprValueDo<'a> {
    pub(crate) value: &'a mut Node<'a>,
    pub(crate) do_t: &'a mut Token<'a>,
}
impl<'a> ExprValueDo<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> ExprValueDo<'a> {
        match value {
            ParseValue::ExprValueDo(value) => value,
            other => unreachable!("expected ExprValueDo, got {:?}", other),
        }
    }
}

impl<'a> PKwLabel<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> PKwLabel<'a> {
        match value {
            ParseValue::PKwLabel(value) => value,
            other => unreachable!("expected PKwLabel, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BraceBody<'a> {
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a mut Node<'a>>,
}
impl<'a> BraceBody<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> BraceBody<'a> {
        match value {
            ParseValue::BraceBody(value) => value,
            other => unreachable!("expected BraceBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CmdBraceBlock<'a> {
    pub(crate) begin_t: &'a mut Token<'a>,
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a mut Node<'a>>,
    pub(crate) end_t: &'a mut Token<'a>,
}
impl<'a> CmdBraceBlock<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> CmdBraceBlock<'a> {
        match value {
            ParseValue::CmdBraceBlock(value) => value,
            other => unreachable!("expected CmdBraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ParenArgs<'a> {
    pub(crate) begin_t: &'a mut Token<'a>,
    pub(crate) args: Vec<'a, &'a mut Node<'a>>,
    pub(crate) end_t: &'a mut Token<'a>,
}
impl<'a> ParenArgs<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> ParenArgs<'a> {
        match value {
            ParseValue::ParenArgs(value) => value,
            other => unreachable!("expected ParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct OptParenArgs<'a> {
    pub(crate) begin_t: Maybe<&'a mut Token<'a>>,
    pub(crate) args: Vec<'a, &'a mut Node<'a>>,
    pub(crate) end_t: Maybe<&'a mut Token<'a>>,
}
impl<'a> OptParenArgs<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> OptParenArgs {
        match value {
            ParseValue::OptParenArgs(value) => value,
            other => unreachable!("expected OptParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BeginBlock<'a> {
    pub(crate) begin_t: &'a mut Token<'a>,
    pub(crate) body: Maybe<&'a mut Node<'a>>,
    pub(crate) end_t: &'a mut Token<'a>,
}
impl<'a> BeginBlock<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> BeginBlock {
        match value {
            ParseValue::BeginBlock(value) => value,
            other => unreachable!("expected BeginBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LambdaBody<'a> {
    pub(crate) begin_t: &'a mut Token<'a>,
    pub(crate) body: Maybe<&'a mut Node<'a>>,
    pub(crate) end_t: &'a mut Token<'a>,
}
impl<'a> LambdaBody<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> LambdaBody<'a> {
        match value {
            ParseValue::LambdaBody(value) => value,
            other => unreachable!("expected LambdaBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DoBlock<'a> {
    pub(crate) begin_t: &'a mut Token<'a>,
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a mut Node<'a>>,
    pub(crate) end_t: &'a mut Token<'a>,
}
impl<'a> DoBlock<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> DoBlock<'a> {
        match value {
            ParseValue::DoBlock(value) => value,
            other => unreachable!("expected DoBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BraceBlock<'a> {
    pub(crate) begin_t: &'a mut Token<'a>,
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a mut Node<'a>>,
    pub(crate) end_t: &'a mut Token<'a>,
}
impl<'a> BraceBlock<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> BraceBlock<'a> {
        match value {
            ParseValue::BraceBlock(value) => value,
            other => unreachable!("expected BraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DefsHead<'a> {
    pub(crate) def_t: &'a mut Token<'a>,
    pub(crate) definee: &'a mut Node<'a>,
    pub(crate) dot_t: &'a mut Token<'a>,
    pub(crate) name_t: &'a mut Token<'a>,
}
impl<'a> DefsHead<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> DefsHead<'a> {
        match value {
            ParseValue::DefsHead(value) => value,
            other => unreachable!("expected DefsHead, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DefnHead<'a> {
    pub(crate) def_t: &'a mut Token<'a>,
    pub(crate) name_t: &'a mut Token<'a>,
}
impl<'a> DefnHead<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> DefnHead<'a> {
        match value {
            ParseValue::DefnHead(value) => value,
            other => unreachable!("expected DefnHead, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Cases<'a> {
    pub(crate) when_bodies: Vec<'a, &'a mut Node<'a>>,
    pub(crate) opt_else: Option<Else<'a>>,
}
impl<'a> Cases<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> Cases<'a> {
        match value {
            ParseValue::Cases(value) => value,
            other => unreachable!("expected Cases, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CaseBody<'a> {
    pub(crate) when_bodies: Vec<'a, &'a mut Node<'a>>,
    pub(crate) opt_else: Option<Else<'a>>,
}
impl<'a> CaseBody<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> CaseBody<'a> {
        match value {
            ParseValue::CaseBody(value) => value,
            other => unreachable!("expected CaseBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PCases<'a> {
    pub(crate) in_bodies: Vec<'a, &'a mut Node<'a>>,
    pub(crate) opt_else: Option<Else<'a>>,
}
impl<'a> PCases<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> PCases<'a> {
        match value {
            ParseValue::PCases(value) => value,
            other => unreachable!("expected PCases, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PCaseBody<'a> {
    pub(crate) in_bodies: Vec<'a, &'a mut Node<'a>>,
    pub(crate) opt_else: Option<Else<'a>>,
}
impl<'a> PCaseBody<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> PCaseBody<'a> {
        match value {
            ParseValue::PCaseBody(value) => value,
            other => unreachable!("expected PCaseBody, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeNode {
    use super::{Node, ParseValue, PtrAPI};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Option<&'a mut Node<'a>> {
        match value {
            ParseValue::MaybeNode(value) => value,
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeBoxedNode {
    use super::{Maybe, Node, ParseValue, Ptr};

    pub(crate) fn from<'a>(value: ParseValue<'a>) -> Maybe<&'a mut Node<'a>> {
        match value {
            ParseValue::MaybeNode(value) => value,
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DoBody<'a> {
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a mut Node<'a>>,
}
impl<'a> DoBody<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> DoBody<'a> {
        match value {
            ParseValue::DoBody(value) => value,
            other => unreachable!("expected DoBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PTopExpr<'a> {
    pub(crate) pattern: &'a mut Node<'a>,
    pub(crate) guard: Maybe<&'a mut Node<'a>>,
}
impl<'a> PTopExpr<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> PTopExpr<'a> {
        match value {
            ParseValue::PTopExpr(value) => value,
            other => unreachable!("expected PTopExpr, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MatchPatternWithTrailingComma<'a> {
    pub(crate) elements: Vec<'a, &'a mut Node<'a>>,
    pub(crate) trailing_comma: Maybe<&'a mut Token<'a>>,
}
impl<'a> MatchPatternWithTrailingComma<'a> {
    pub(crate) fn from(value: ParseValue<'a>) -> MatchPatternWithTrailingComma<'a> {
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
    Token(&'a mut Token<'a>),
    TokenList(Vec<'a, &'a mut Token<'a>>),
    Node(&'a mut Node<'a>),
    NodeList(Vec<'a, &'a mut Node<'a>>),
    Bool(bool),
    MaybeStrTerm(Option<&'a StrTerm<'a>>),
    Num(i32),

    /* For custom superclass rule */
    Superclass(Superclass<'a>),

    /* For custom opt_ensure rule */
    OptEnsure(Option<Ensure<'a>>),

    /* For custom opt_else rule */
    OptElse(Option<Else<'a>>),

    /* For custom exc_var rule */
    ExcVar(ExcVar<'a>),

    /* For custom if_tail rule */
    IfTail(IfTail<'a>),

    /* For custom expr_value_do rule */
    ExprValueDo(ExprValueDo<'a>),

    /* For custom p_kw_label rule */
    PKwLabel(PKwLabel<'a>),

    /* For custom brace_body rule */
    BraceBody(BraceBody<'a>),

    /* For custom cmd_brace_block rule */
    CmdBraceBlock(CmdBraceBlock<'a>),

    /* For custom paren_args rule  */
    ParenArgs(ParenArgs<'a>),

    /* For custom opt_paren_args rule  */
    OptParenArgs(OptParenArgs<'a>),

    /* For custom lambda_body rule  */
    LambdaBody(LambdaBody<'a>),

    /* For custom do_block rule  */
    DoBlock(DoBlock<'a>),

    /* For custom brace_block rule  */
    BraceBlock(BraceBlock<'a>),

    /* For custom defs_head rule */
    DefsHead(DefsHead<'a>),

    /* For custom defn_head rule */
    DefnHead(DefnHead<'a>),

    /* For custom begin_block rule  */
    BeginBlock(BeginBlock<'a>),

    /* For custom cases rule */
    Cases(Cases<'a>),

    /* For custom case_body rule */
    CaseBody(CaseBody<'a>),

    /* For custom p_cases rule */
    PCases(PCases<'a>),

    /* For custom p_case_body rule */
    PCaseBody(PCaseBody<'a>),

    /* For custom compstmt rule */
    MaybeNode(Maybe<&'a mut Node<'a>>),

    /* For custom do_body rule */
    DoBody(DoBody<'a>),

    /* For custom p_top_expr rule */
    PTopExpr(PTopExpr<'a>),

    /* For pattern matching patterns with trailing comma */
    MatchPatternWithTrailingComma(MatchPatternWithTrailingComma<'a>),
}

impl<'a> ParseValue<'a> {
    pub(crate) fn from_token(token: &'a mut Token<'a>) -> Self {
        Self::Token(token)
    }

    pub(crate) fn new_superclass(value: Superclass<'a>) -> Self {
        Self::Superclass(value)
    }
    pub(crate) fn new_opt_ensure(value: Option<Ensure<'a>>) -> Self {
        Self::OptEnsure(value)
    }
    pub(crate) fn new_opt_else(value: Option<Else<'a>>) -> Self {
        Self::OptElse(value)
    }
    pub(crate) fn new_exc_var(value: ExcVar<'a>) -> Self {
        Self::ExcVar(value)
    }
    pub(crate) fn new_if_tail(value: IfTail<'a>) -> Self {
        Self::IfTail(value)
    }
    pub(crate) fn new_expr_value_do(value: ExprValueDo<'a>) -> Self {
        Self::ExprValueDo(value)
    }
    pub(crate) fn new_p_kw_label(value: PKwLabel<'a>) -> Self {
        Self::PKwLabel(value)
    }
    pub(crate) fn new_brace_body(value: BraceBody<'a>) -> Self {
        Self::BraceBody(value)
    }
    pub(crate) fn new_cmd_brace_block(value: CmdBraceBlock<'a>) -> Self {
        Self::CmdBraceBlock(value)
    }
    pub(crate) fn new_paren_args(value: ParenArgs<'a>) -> Self {
        Self::ParenArgs(value)
    }
    pub(crate) fn new_opt_paren_args(value: OptParenArgs<'a>) -> Self {
        Self::OptParenArgs(value)
    }
    pub(crate) fn new_lambda_body(value: LambdaBody<'a>) -> Self {
        Self::LambdaBody(value)
    }
    pub(crate) fn new_do_block(value: DoBlock<'a>) -> Self {
        Self::DoBlock(value)
    }
    pub(crate) fn new_brace_block(value: BraceBlock<'a>) -> Self {
        Self::BraceBlock(value)
    }
    pub(crate) fn new_defs_head(value: DefsHead<'a>) -> Self {
        Self::DefsHead(value)
    }
    pub(crate) fn new_defn_head(value: DefnHead<'a>) -> Self {
        Self::DefnHead(value)
    }
    pub(crate) fn new_begin_block(value: BeginBlock<'a>) -> Self {
        Self::BeginBlock(value)
    }
    pub(crate) fn new_cases(value: Cases<'a>) -> Self {
        Self::Cases(value)
    }
    pub(crate) fn new_case_body(value: CaseBody<'a>) -> Self {
        Self::CaseBody(value)
    }
    pub(crate) fn new_p_cases(value: PCases<'a>) -> Self {
        Self::PCases(value)
    }
    pub(crate) fn new_p_case_body(value: PCaseBody<'a>) -> Self {
        Self::PCaseBody(value)
    }
    pub(crate) fn new_do_body(value: DoBody<'a>) -> Self {
        Self::DoBody(value)
    }
    pub(crate) fn new_p_top_expr(value: PTopExpr<'a>) -> Self {
        Self::PTopExpr(value)
    }
    pub(crate) fn new_match_pattern_with_trailing_comma(
        value: MatchPatternWithTrailingComma<'a>,
    ) -> Self {
        Self::MatchPatternWithTrailingComma(value)
    }
    pub(crate) fn new_none() -> Self {
        Self::None
    }
    pub(crate) fn new_node(node: &'a mut Node<'a>) -> Self {
        Self::Node(node)
    }
    pub(crate) fn new_maybe_node(maybe_node: Option<&'a mut Node<'a>>) -> Self {
        Self::MaybeNode(maybe_node)
    }
    pub(crate) fn new_node_list(node_list: Vec<'a, &'a mut Node<'a>>) -> Self {
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
    pub(crate) fn new_token(token: &'a mut Token<'a>) -> Self {
        Self::Token(token)
    }
    pub(crate) fn new_token_list(token_list: Vec<'a, &'a mut Token<'a>>) -> Self {
        Self::TokenList(token_list)
    }
}

impl Default for ParseValue<'_> {
    fn default() -> Self {
        Self::Stolen
    }
}
