crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(Vec);
crate::use_native_or_external!(Maybe);

use crate::builder::{ArgsType, PKwLabel};
use crate::str_term::StrTerm;
use crate::Node;
use crate::Token;

impl<'a> Node<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> &'a Node<'a> {
        match value {
            ParseValue::Node(value) => *value,
            other => unreachable!("expected Node, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod BoxedNode {
    use super::{Node, ParseValue, Ptr};

    pub(crate) fn from<'a>(value: &'a ParseValue<'a>) -> &'a Node {
        match value {
            ParseValue::Node(value) => value,
            other => unreachable!("expected BoxedNode, got {:?}", other),
        }
    }
}

impl<'a> Token<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> &'a Token {
        match value {
            ParseValue::Token(value) => value,
            other => unreachable!("expected Token<'a>, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod NodeList {
    use super::{Node, ParseValue, Vec};

    pub(crate) fn from<'a>(value: &'a ParseValue<'a>) -> Vec<Node<'a>> {
        match value {
            ParseValue::NodeList(value) => value,
            other => unreachable!("expected NodeList, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod Bool {
    use super::ParseValue;

    pub(crate) fn from<'a>(value: &'a ParseValue<'a>) -> bool {
        match value {
            ParseValue::Bool(value) => value,
            other => unreachable!("expected Bool, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeStrTerm {
    use super::{ParseValue, StrTerm};

    pub(crate) fn from<'a>(value: &'a ParseValue<'a>) -> Option<Box<StrTerm>> {
        match value {
            ParseValue::MaybeStrTerm(value) => value,
            other => unreachable!("expected MaybeStrTerm, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod Num {
    use super::ParseValue;

    pub(crate) fn from<'a>(value: &'a ParseValue<'a>) -> i32 {
        match value {
            ParseValue::Num(value) => value,
            other => unreachable!("expected Num, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Superclass<'a> {
    pub(crate) lt_t: Maybe<&'a Token<'a>>,
    pub(crate) value: Maybe<&'a Node<'a>>,
}
impl<'a> Superclass<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> Superclass<'a> {
        match value {
            ParseValue::Superclass(value) => *value,
            other => unreachable!("expected Superclass, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Ensure<'a> {
    pub(crate) ensure_t: &'a Token<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
}
#[allow(non_snake_case)]
pub(crate) mod OptEnsure {
    use super::{Ensure, ParseValue};

    pub(crate) fn from<'a>(value: &'a ParseValue<'a>) -> Option<Ensure<'a>> {
        match value {
            ParseValue::OptEnsure(value) => value.map(|v| *v),
            other => unreachable!("expected OptEnsure, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Else<'a> {
    pub(crate) else_t: &'a Token<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
}
#[allow(non_snake_case)]
pub(crate) mod OptElse {
    use super::{Else, ParseValue};

    pub(crate) fn from<'a>(value: &'a ParseValue<'a>) -> Option<Else<'a>> {
        match value {
            ParseValue::OptElse(value) => value.map(|v| *v),
            other => unreachable!("expected OptElse, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExcVar<'a> {
    pub(crate) assoc_t: Maybe<&'a Token<'a>>,
    pub(crate) exc_var: Maybe<&'a Node<'a>>,
}
impl<'a> ExcVar<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> ExcVar<'a> {
        match value {
            ParseValue::ExcVar(value) => *value,
            other => unreachable!("expected ExcVar, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct IfTail<'a> {
    pub(crate) keyword_t: Maybe<&'a Token<'a>>,
    pub(crate) body: Maybe<&'a Node<'a>>,
}
impl<'a> IfTail<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> IfTail<'a> {
        match value {
            ParseValue::IfTail(value) => *value,
            other => unreachable!("expected IfTail, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExprValueDo<'a> {
    pub(crate) value: &'a Node<'a>,
    pub(crate) do_t: &'a Token<'a>,
}
impl<'a> ExprValueDo<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> ExprValueDo<'a> {
        match value {
            ParseValue::ExprValueDo(value) => *value,
            other => unreachable!("expected ExprValueDo, got {:?}", other),
        }
    }
}

impl<'a> PKwLabel<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> PKwLabel<'a> {
        match value {
            ParseValue::PKwLabel(value) => *value,
            other => unreachable!("expected PKwLabel, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BraceBody<'a> {
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
}
impl<'a> BraceBody<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> BraceBody<'a> {
        match value {
            ParseValue::BraceBody(value) => *value,
            other => unreachable!("expected BraceBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CmdBraceBlock<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> CmdBraceBlock<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> CmdBraceBlock<'a> {
        match value {
            ParseValue::CmdBraceBlock(value) => *value,
            other => unreachable!("expected CmdBraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ParenArgs<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) args: List<'a, &'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> ParenArgs<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> ParenArgs<'a> {
        match value {
            ParseValue::ParenArgs(value) => *value,
            other => unreachable!("expected ParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OptParenArgs<'a> {
    pub(crate) begin_t: Maybe<&'a Token<'a>>,
    pub(crate) args: List<'a, &'a Node<'a>>,
    pub(crate) end_t: Maybe<&'a Token<'a>>,
}
impl<'a> OptParenArgs<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> OptParenArgs {
        match value {
            ParseValue::OptParenArgs(value) => *value,
            other => unreachable!("expected OptParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BeginBlock<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> BeginBlock<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> BeginBlock {
        match value {
            ParseValue::BeginBlock(value) => *value,
            other => unreachable!("expected BeginBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LambdaBody<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> LambdaBody<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> LambdaBody<'a> {
        match value {
            ParseValue::LambdaBody(value) => *value,
            other => unreachable!("expected LambdaBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DoBlock<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> DoBlock<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> DoBlock<'a> {
        match value {
            ParseValue::DoBlock(value) => *value,
            other => unreachable!("expected DoBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BraceBlock<'a> {
    pub(crate) begin_t: &'a Token<'a>,
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
    pub(crate) end_t: &'a Token<'a>,
}
impl<'a> BraceBlock<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> BraceBlock<'a> {
        match value {
            ParseValue::BraceBlock(value) => *value,
            other => unreachable!("expected BraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefsHead<'a> {
    pub(crate) def_t: &'a Token<'a>,
    pub(crate) definee: &'a Node<'a>,
    pub(crate) dot_t: &'a Token<'a>,
    pub(crate) name_t: &'a Token<'a>,
}
impl<'a> DefsHead<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> DefsHead<'a> {
        match value {
            ParseValue::DefsHead(value) => *value,
            other => unreachable!("expected DefsHead, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefnHead<'a> {
    pub(crate) def_t: &'a Token<'a>,
    pub(crate) name_t: &'a Token<'a>,
}
impl<'a> DefnHead<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> DefnHead<'a> {
        match value {
            ParseValue::DefnHead(value) => *value,
            other => unreachable!("expected DefnHead, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Cases<'a> {
    pub(crate) when_bodies: List<'a, &'a Node<'a>>,
    pub(crate) opt_else: Option<Else<'a>>,
}
impl<'a> Cases<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> Cases<'a> {
        match value {
            ParseValue::Cases(value) => *value,
            other => unreachable!("expected Cases, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CaseBody<'a> {
    pub(crate) when_bodies: List<'a, &'a Node<'a>>,
    pub(crate) opt_else: Option<Else<'a>>,
}
impl<'a> CaseBody<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> CaseBody<'a> {
        match value {
            ParseValue::CaseBody(value) => *value,
            other => unreachable!("expected CaseBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PCases<'a> {
    pub(crate) in_bodies: List<'a, &'a Node<'a>>,
    pub(crate) opt_else: Option<Else<'a>>,
}
impl<'a> PCases<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> PCases<'a> {
        match value {
            ParseValue::PCases(value) => *value,
            other => unreachable!("expected PCases, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PCaseBody<'a> {
    pub(crate) in_bodies: List<'a, &'a Node<'a>>,
    pub(crate) opt_else: Option<Else<'a>>,
}
impl<'a> PCaseBody<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> PCaseBody<'a> {
        match value {
            ParseValue::PCaseBody(value) => *value,
            other => unreachable!("expected PCaseBody, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeNode {
    use super::{Node, ParseValue, PtrAPI};

    pub(crate) fn from<'a>(value: &'a ParseValue<'a>) -> Option<&'a Node<'a>> {
        match value {
            ParseValue::MaybeNode(value) => {
                if value.is_some() {
                    Some(value.unwrap().unptr())
                } else {
                    None
                }
            }
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeBoxedNode {
    use super::{Maybe, Node, ParseValue, Ptr};

    pub(crate) fn from<'a>(value: &'a ParseValue<'a>) -> Maybe<&'a Node<'a>> {
        match value {
            ParseValue::MaybeNode(value) => value,
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DoBody<'a> {
    pub(crate) args_type: ArgsType<'a>,
    pub(crate) body: Maybe<&'a Node<'a>>,
}
impl<'a> DoBody<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> DoBody<'a> {
        match value {
            ParseValue::DoBody(value) => *value,
            other => unreachable!("expected DoBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PTopExpr<'a> {
    pub(crate) pattern: &'a Node<'a>,
    pub(crate) guard: Maybe<&'a Node<'a>>,
}
impl<'a> PTopExpr<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> PTopExpr<'a> {
        match value {
            ParseValue::PTopExpr(value) => *value,
            other => unreachable!("expected PTopExpr, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MatchPatternWithTrailingComma<'a> {
    pub(crate) elements: List<'a, &'a Node<'a>>,
    pub(crate) trailing_comma: Maybe<&'a Token<'a>>,
}
impl<'a> MatchPatternWithTrailingComma<'a> {
    pub(crate) fn from(value: &'a ParseValue<'a>) -> MatchPatternWithTrailingComma<'a> {
        match value {
            ParseValue::MatchPatternWithTrailingComma(value) => *value,
            other => unreachable!("expected MatchPatternWithTrailingComma, got {:?}", other),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum ParseValue<'a> {
    Stolen,
    Uninitialized,
    None,
    Token(&'a Token<'a>),
    TokenList(List<'a, &'a Token<'a>>),
    Node(&'a Node<'a>),
    NodeList(List<'a, &'a Node<'a>>),
    Bool(bool),
    MaybeStrTerm(Option<&'a StrTerm>),
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
    pub(crate) fn from_token(bump: &'a bumpalo::Bump, token: &'a Token<'a>) -> &'a Self {
        bump.alloc(Self::Token(token))
    }

    pub(crate) fn new_superclass(bump: &'a bumpalo::Bump, value: &'a Superclass) -> &'a Self {
        bump.alloc(Self::Superclass(Box::new(value)))
    }
    pub(crate) fn new_opt_ensure(
        bump: &'a bumpalo::Bump,
        value: Option<&'a Ensure<'a>>,
    ) -> &'a Self {
        bump.alloc(Self::OptEnsure(value))
    }
    pub(crate) fn new_opt_else(bump: &'a bumpalo::Bump, value: Option<&'a Else<'a>>) -> &'a Self {
        bump.alloc(Self::OptElse(value))
    }
    pub(crate) fn new_exc_var(bump: &'a bumpalo::Bump, value: &'a ExcVar) -> &'a Self {
        bump.alloc(Self::ExcVar(Box::new(value)))
    }
    pub(crate) fn new_if_tail(bump: &'a bumpalo::Bump, value: &'a IfTail) -> &'a Self {
        bump.alloc(Self::IfTail(Box::new(value)))
    }
    pub(crate) fn new_expr_value_do(bump: &'a bumpalo::Bump, value: &'a ExprValueDo) -> &'a Self {
        bump.alloc(Self::ExprValueDo(Box::new(value)))
    }
    pub(crate) fn new_p_kw_label(bump: &'a bumpalo::Bump, value: &'a PKwLabel) -> &'a Self {
        bump.alloc(Self::PKwLabel(Box::new(value)))
    }
    pub(crate) fn new_brace_body(bump: &'a bumpalo::Bump, value: &'a BraceBody) -> &'a Self {
        bump.alloc(Self::BraceBody(Box::new(value)))
    }
    pub(crate) fn new_cmd_brace_block(
        bump: &'a bumpalo::Bump,
        value: &'a CmdBraceBlock,
    ) -> &'a Self {
        bump.alloc(Self::CmdBraceBlock(Box::new(value)))
    }
    pub(crate) fn new_paren_args(bump: &'a bumpalo::Bump, value: &'a ParenArgs) -> &'a Self {
        bump.alloc(Self::ParenArgs(Box::new(value)))
    }
    pub(crate) fn new_opt_paren_args(bump: &'a bumpalo::Bump, value: &'a OptParenArgs) -> &'a Self {
        bump.alloc(Self::OptParenArgs(Box::new(value)))
    }
    pub(crate) fn new_lambda_body(bump: &'a bumpalo::Bump, value: &'a LambdaBody) -> &'a Self {
        bump.alloc(Self::LambdaBody(Box::new(value)))
    }
    pub(crate) fn new_do_block(bump: &'a bumpalo::Bump, value: &'a DoBlock) -> &'a Self {
        bump.alloc(Self::DoBlock(Box::new(value)))
    }
    pub(crate) fn new_brace_block(bump: &'a bumpalo::Bump, value: &'a BraceBlock) -> &'a Self {
        bump.alloc(Self::BraceBlock(Box::new(value)))
    }
    pub(crate) fn new_defs_head(bump: &'a bumpalo::Bump, value: &'a DefsHead) -> &'a Self {
        bump.alloc(Self::DefsHead(Box::new(value)))
    }
    pub(crate) fn new_defn_head(bump: &'a bumpalo::Bump, value: &'a DefnHead) -> &'a Self {
        bump.alloc(Self::DefnHead(Box::new(value)))
    }
    pub(crate) fn new_begin_block(bump: &'a bumpalo::Bump, value: &'a BeginBlock) -> &'a Self {
        bump.alloc(Self::BeginBlock(Box::new(value)))
    }
    pub(crate) fn new_cases(bump: &'a bumpalo::Bump, value: &'a Cases) -> &'a Self {
        bump.alloc(Self::Cases(Box::new(value)))
    }
    pub(crate) fn new_case_body(bump: &'a bumpalo::Bump, value: &'a CaseBody) -> &'a Self {
        bump.alloc(Self::CaseBody(Box::new(value)))
    }
    pub(crate) fn new_p_cases(bump: &'a bumpalo::Bump, value: &'a PCases) -> &'a Self {
        bump.alloc(Self::PCases(Box::new(value)))
    }
    pub(crate) fn new_p_case_body(bump: &'a bumpalo::Bump, value: &'a PCaseBody) -> &'a Self {
        bump.alloc(Self::PCaseBody(Box::new(value)))
    }
    pub(crate) fn new_do_body(bump: &'a bumpalo::Bump, value: &'a DoBody) -> &'a Self {
        bump.alloc(Self::DoBody(Box::new(value)))
    }
    pub(crate) fn new_p_top_expr(bump: &'a bumpalo::Bump, value: &'a PTopExpr) -> &'a Self {
        bump.alloc(Self::PTopExpr(Box::new(value)))
    }
    pub(crate) fn new_match_pattern_with_trailing_comma(
        bump: &'a bumpalo::Bump,
        value: MatchPatternWithTrailingComma,
    ) -> &'a Self {
        bump.alloc(Self::MatchPatternWithTrailingComma(Box::new(value)))
    }
    pub(crate) fn new_none(bump: &'a bumpalo::Bump) -> &'a Self {
        bump.alloc(Self::None)
    }
    pub(crate) fn new_node(bump: &'a bumpalo::Bump, node: &'a Node<'a>) -> &'a Self {
        bump.alloc(Self::Node(node))
    }
    pub(crate) fn new_maybe_node(
        bump: &'a bumpalo::Bump,
        maybe_node: Option<&'a Node<'a>>,
    ) -> &'a Self {
        bump.alloc(Self::MaybeNode(maybe_node))
    }
    pub(crate) fn new_node_list(
        bump: &'a bumpalo::Bump,
        node_list: List<'a, &'a Node<'a>>,
    ) -> &'a Self {
        bump.alloc(Self::NodeList(node_list))
    }
    pub(crate) fn new_bool(bump: &'a bumpalo::Bump, value: bool) -> &'a Self {
        bump.alloc(Self::Bool(value))
    }
    pub(crate) fn new_num(bump: &'a bumpalo::Bump, value: i32) -> &'a Self {
        bump.alloc(Self::Num(value))
    }
    pub(crate) fn new_maybe_str_term(
        bump: &'a bumpalo::Bump,
        maybe_str_term: Option<&'a StrTerm>,
    ) -> &'a Self {
        bump.alloc(Self::MaybeStrTerm(maybe_str_term))
    }
    pub(crate) fn new_token(bump: &'a bumpalo::Bump, token: &'a Token) -> &'a Self {
        bump.alloc(Self::Token(token))
    }
    pub(crate) fn new_token_list(
        bump: &'a bumpalo::Bump,
        token_list: List<'a, &'a Token<'a>>,
    ) -> &'a Self {
        bump.alloc(Self::TokenList(token_list))
    }
}

impl Default for ParseValue<'_> {
    fn default() -> Self {
        Self::Stolen
    }
}
