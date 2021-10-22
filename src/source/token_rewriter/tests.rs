crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(Vec);

use bumpalo::Bump;

use crate::source::token_rewriter::InternalTokenRewriterResult;

use super::{TokenRewriter, TokenRewriterResult};
use crate::Bytes;
use crate::LexState;
use crate::Loc;
use crate::Token;

const INITIAL_TOKEN_ID: i32 = 310;
const REWRITTEN_TOKEN_ID: i32 = 300;

fn rewritten_token<'a>(bump: &'a Bump) -> &'a Token<'a> {
    bump.alloc(Token::new(
        bump,
        REWRITTEN_TOKEN_ID,
        Bytes::new(
            bump,
            bump_vec![in bump; b'r', b'e', b'w', b'r', b'i', b't', b't', b'e', b'n'],
        ),
        Loc::new(bump, 1, 2),
        LexState { value: 1 },
        LexState { value: 2 },
    ))
}

#[cfg(feature = "compile-with-external-structures")]
mod dummy_rewriter {
    use super::rewritten_token;
    use super::{Ptr, Token};
    use crate::blobs::{Blob, HasBlob};
    use crate::source::token_rewriter::TokenRewriter;

    extern "C" {
        fn lib_ruby_parser__testing__token_rewriter__new_keep(
            token_f: extern "C" fn() -> Ptr<Token>,
        ) -> Blob<TokenRewriter>;
        fn lib_ruby_parser__testing__token_rewriter__new_drop(
            token_f: extern "C" fn() -> Ptr<Token>,
        ) -> Blob<TokenRewriter>;
        fn lib_ruby_parser__testing__token_rewriter__new_rewrite(
            token_f: extern "C" fn() -> Ptr<Token>,
        ) -> Blob<TokenRewriter>;
    }

    extern "C" fn token_f() -> Ptr<Token> {
        rewritten_token()
    }

    pub(crate) fn dummy_decoder_keep() -> TokenRewriter {
        TokenRewriter::from_blob(unsafe {
            lib_ruby_parser__testing__token_rewriter__new_keep(token_f)
        })
    }

    pub(crate) fn dummy_decoder_drop() -> TokenRewriter {
        TokenRewriter::from_blob(unsafe {
            lib_ruby_parser__testing__token_rewriter__new_drop(token_f)
        })
    }

    pub(crate) fn dummy_decoder_rewrite() -> TokenRewriter {
        TokenRewriter::from_blob(unsafe {
            lib_ruby_parser__testing__token_rewriter__new_rewrite(token_f)
        })
    }
}

#[cfg(not(feature = "compile-with-external-structures"))]
mod dummy_rewriter {
    use super::rewritten_token;
    use crate::source::token_rewriter::{
        LexStateAction, RewriteAction, TokenRewriter, TokenRewriterResult,
    };

    pub(crate) fn dummy_decoder_keep<'a>() -> TokenRewriter<'a> {
        TokenRewriter::new(Box::new(|token, _input| TokenRewriterResult {
            rewritten_token: token,
            token_action: RewriteAction::Keep,
            lex_state_action: LexStateAction::Keep,
        }))
    }
    pub(crate) fn dummy_decoder_drop<'a>() -> TokenRewriter<'a> {
        TokenRewriter::new(Box::new(|token, _input| TokenRewriterResult {
            rewritten_token: token,
            token_action: RewriteAction::Drop,
            lex_state_action: LexStateAction::Keep,
        }))
    }
    pub(crate) fn dummy_decoder_rewrite<'a>(bump: &'a bumpalo::Bump) -> TokenRewriter<'a> {
        TokenRewriter::new(Box::new(move |_token, _input| TokenRewriterResult {
            rewritten_token: rewritten_token(bump),
            token_action: RewriteAction::Keep,
            lex_state_action: LexStateAction::Keep,
        }))
    }
}

fn call_dummy_rewriter<'a>(
    bump: &'a Bump,
    rewriter: TokenRewriter<'a>,
    input: &'a [u8],
) -> TokenRewriterResult<'a> {
    // it's dummy, so encoding/input doesn't matter
    let token = bump.alloc(Token::new(
        &bump,
        INITIAL_TOKEN_ID,
        Bytes::new(
            &bump,
            bump_vec![in &bump; b'i', b'n', b'i', b't', b'i', b'a', b'l'],
        ),
        Loc::new(bump, 1, 2),
        LexState { value: 1 },
        LexState { value: 2 },
    ));

    rewriter.call(token, input)
}

#[test]
fn test_keep() {
    let bump = Bump::new();

    let input = bump_vec![in &bump; b'2', b'+', b'2'];
    let result = call_dummy_rewriter(
        &bump,
        dummy_rewriter::dummy_decoder_keep(),
        input.as_slice(),
    );
    let result = result.into_internal(&bump);

    assert_eq!(result.rewritten_token.token_type(), INITIAL_TOKEN_ID);
    assert_eq!(
        result.rewritten_token.token_value(),
        &Bytes::new(
            &bump,
            bump_vec!(in &bump; b'i', b'n', b'i', b't', b'i', b'a', b'l')
        )
    );
    assert!(result.token_action.is_keep());
}

#[test]
fn test_drop() {
    let bump = bumpalo::Bump::new();

    let input = bump_vec![in &bump; b'2', b'+', b'2'];
    let result = call_dummy_rewriter(
        &bump,
        dummy_rewriter::dummy_decoder_drop(),
        input.as_slice(),
    );
    let result = result.into_internal(&bump);

    assert!(result.token_action.is_drop());
}

#[test]
fn test_rewrite() {
    let bump = Bump::new();

    let input = bump_vec![in &bump; b'2', b'+', b'2'];
    let result = call_dummy_rewriter(
        &bump,
        dummy_rewriter::dummy_decoder_rewrite(&bump),
        input.as_slice(),
    );
    let result = result.into_internal(&bump);

    assert_eq!(result.rewritten_token.token_type(), REWRITTEN_TOKEN_ID);
    assert_eq!(
        result.rewritten_token.token_value(),
        &Bytes::new(
            &bump,
            bump_vec![in &bump; b'r', b'e', b'w', b'r', b'i', b't', b't', b'e', b'n']
        )
    );
    assert!(result.token_action.is_keep());
}
