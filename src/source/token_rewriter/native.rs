use super::InternalTokenRewriterResult;
use crate::Token;

/// Enum of what token rewriter should do with a token.
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RewriteAction {
    /// Means "drop the token", i.e. don't return it to a parser
    Drop,

    /// Means "keep the token", i.e. return it to a parser
    Keep,
}

impl RewriteAction {
    pub(crate) fn is_drop(&self) -> bool {
        matches!(self, Self::Drop)
    }

    pub(crate) fn is_keep(&self) -> bool {
        matches!(self, Self::Keep)
    }
}

/// Enum of what token rewriter should do with the state of the lexer
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LexStateAction {
    /// Means "set the state to X"
    Set(i32),

    /// Means "keep the state unchanged"
    Keep,
}

impl LexStateAction {
    pub(crate) fn is_set(&self) -> bool {
        matches!(self, Self::Set(_))
    }

    pub(crate) fn is_keep(&self) -> bool {
        matches!(self, Self::Keep)
    }

    pub(crate) fn next_state(&self) -> i32 {
        match self {
            Self::Set(state) => *state,
            Self::Keep => panic!("Wrong variant of LexStateAction"),
        }
    }
}

/// Output of the token rewriter
#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct TokenRewriterResult<'a> {
    /// Rewritten token. Can be input token if no rewriting expected
    pub rewritten_token: &'a Token<'a>,

    /// Action to be applied on a token (keep or drop)
    pub token_action: RewriteAction,

    /// Action to be applied on lexer's state (keep as is or change)
    pub lex_state_action: LexStateAction,
}

// impl<'a> TokenRewriterResult<'a> {
//     pub(crate) fn into_internal(
//         &'a self,
//         bump: &'a bumpalo::Bump,
//     ) -> &'a InternalTokenRewriterResult<'a> {
//         let Self {
//             rewritten_token,
//             token_action,
//             lex_state_action,
//         } = self;
//         bump.alloc(InternalTokenRewriterResult {
//             rewritten_token,
//             token_action: token_action.clone(),
//             lex_state_action: lex_state_action.clone(),
//         })
//     }
// }

/// Token rewriter function
pub type TokenRewriterFn<'a> = dyn (Fn(&'a Token<'a>, &[u8]) -> TokenRewriterResult<'a>) + 'a;

/// Token rewriter struct, can be used to rewrite tokens on the fly
pub struct TokenRewriter<'a> {
    f: Box<TokenRewriterFn<'a>>,
}

impl<'a> TokenRewriter<'a> {
    /// Constructs a rewriter based on a given function
    pub fn new(f: Box<TokenRewriterFn<'a>>) -> Self {
        Self { f }
    }

    pub(crate) fn call(&self, token: &'a Token<'a>, input: &[u8]) -> TokenRewriterResult<'a> {
        let f = &*self.f;
        f(token, input)
    }
}
