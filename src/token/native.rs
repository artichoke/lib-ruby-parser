use bumpalo::Bump;

use crate::{Bytes, LexState, Loc};

/// A token that is emitted by a lexer and consumed by a parser
#[derive(Clone)]
#[repr(C)]
pub struct Token<'a> {
    pub(crate) bump: &'a Bump,

    /// Numeric representation of the token type,
    /// e.g. 42 (for example) for tINTEGER
    token_type: i32,

    /// Value of the token,
    /// e.g "42" for 42
    token_value: Bytes<'a>,

    /// Location of the token
    loc: &'a Loc,

    /// Lex state **before** reading the token
    lex_state_before: LexState,

    /// Lex state **after** reading the token
    lex_state_after: LexState,
}

impl PartialEq for Token<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
            && self.token_value == other.token_value
            && self.loc == other.loc
            && self.lex_state_before == other.lex_state_before
            && self.lex_state_after == other.lex_state_after
    }
}

impl Eq for Token<'_> {}

impl<'a> Token<'a> {
    /// Constructor
    pub fn new(
        bump: &'a Bump,
        token_type: i32,
        token_value: Bytes<'a>,
        loc: &'a Loc,
        lex_state_before: LexState,
        lex_state_after: LexState,
    ) -> Self {
        Self {
            bump,
            token_type,
            token_value,
            loc,
            lex_state_before,
            lex_state_after,
        }
    }

    /// Returns type of the token
    pub fn token_type(&self) -> i32 {
        self.token_type
    }

    /// Returns type of the token
    pub fn token_value(&self) -> &Bytes<'a> {
        &self.token_value
    }

    /// Sets token value
    pub fn set_token_value(&mut self, token_value: Bytes<'a>) {
        self.token_value = token_value
    }

    // /// Consumes self, returns owned values of the token
    // pub fn into_token_value(self) -> Bytes<'a> {
    //     self.token_value
    // }

    /// Returns location of the token
    pub fn loc(&self) -> &Loc {
        &self.loc
    }

    /// Returns lex state **before** reading the token
    pub fn lex_state_before(&self) -> LexState {
        self.lex_state_before
    }

    /// Returns lex state **after** reading the token
    pub fn lex_state_after(&self) -> LexState {
        self.lex_state_after
    }
}
