crate::use_native_or_external!(String);
crate::use_native_or_external!(Maybe);

use crate::source::token_rewriter::TokenRewriter;
use crate::source::Decoder;

#[repr(C)]
pub(crate) struct InternalParserOptions<'a> {
    pub(crate) buffer_name: String<'a>,
    pub(crate) decoder: Maybe<Decoder<'a>>,
    pub(crate) token_rewriter: Maybe<TokenRewriter<'a>>,
    pub(crate) record_tokens: bool,
}
