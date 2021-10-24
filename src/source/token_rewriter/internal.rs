crate::use_native_or_external!(Ptr);

use super::{LexStateAction, RewriteAction};
use crate::Token;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct InternalTokenRewriterResult<'a> {
    pub(crate) rewritten_token: &'a mut Token<'a>,
    pub(crate) token_action: RewriteAction,
    pub(crate) lex_state_action: LexStateAction,
}
