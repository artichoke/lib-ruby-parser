crate::use_native_or_external!(Vec);
use bumpalo::Bump;

use crate::{Bytes, LexState, Loc, Token};

fn lex_state(value: i32) -> LexState {
    let mut lex_state = LexState::default();
    lex_state.set(value);
    lex_state
}

fn new_token<'a>(bump: &'a Bump) -> Token<'a> {
    Token::new(
        bump,
        1,
        Bytes::new(bump, bump_vec![in &bump; 1, 2, 3]),
        Loc::new(1, 2),
        lex_state(1),
        lex_state(2),
    )
}

#[test]
fn test_new() {
    let bump = Bump::new();
    let token = new_token(&bump);
    drop(token);
}

#[test]
fn test_token_type() {
    let bump = Bump::new();
    let token = new_token(&bump);
    assert_eq!(token.token_type(), 1)
}

#[test]
fn test_token_value() {
    let bump = Bump::new();
    let token = new_token(&bump);
    assert_eq!(
        token.token_value(),
        &Bytes::new(&bump, bump_vec![in &bump; 1, 2, 3])
    );
}

#[test]
fn test_set_token_value() {
    let bump = Bump::new();
    let mut token = new_token(&bump);
    {
        token.set_token_value(Bytes::new(&bump, bump_vec![in &bump; 4, 5, 6]));
    }
    assert_eq!(
        token.token_value(),
        &Bytes::new(&bump, bump_vec![in &bump; 4, 5, 6])
    );
}

// #[test]
// fn test_into_token_value() {
//     let bump = Bump::new();
//     let token = new_token(&bump);
//     assert_eq!(
//         token.into_token_value(),
//         Bytes::new(bump,  bump_vec![in &bump; 1, 2, 3])
//     );
// }

#[test]
fn test_loc() {
    let bump = Bump::new();
    let token = new_token(&bump);
    assert_eq!(token.loc(), &Loc::new(1, 2));
}

#[test]
fn test_lex_state_before() {
    let bump = Bump::new();
    let token = new_token(&bump);
    assert_eq!(token.lex_state_before(), lex_state(1));
}

#[test]
fn test_lex_state_after() {
    let bump = Bump::new();
    let token = new_token(&bump);
    assert_eq!(token.lex_state_after(), lex_state(2));
}
