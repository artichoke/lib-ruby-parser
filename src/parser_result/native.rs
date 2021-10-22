use crate::source::Comment;
use crate::source::DecodedInput;
use crate::source::MagicComment;
use crate::Diagnostic;
use crate::Node;
use crate::Token;

/// Combination of all data that `Parser` can give you
#[derive(Debug)]
#[repr(C)]
pub struct ParserResult<'a> {
    /// Abstract Syntax Tree that was constructed from you code.
    /// Contains `None` if the code gives no AST nodes
    pub ast: Option<&'a Node<'a>>,

    /// List of tokens returned by a Lexer and consumed by a Parser.
    /// Empty unless ParserOptions::record_tokens is set to true.
    pub tokens: bumpalo::collections::Vec<'a, &'a Token<'a>>,

    /// List of all diagnostics (errors and warings) that have been
    /// recorded during lexing and parsing
    pub diagnostics: bumpalo::collections::Vec<'a, &'a Diagnostic>,

    /// List of comments extracted from the source code.
    pub comments: bumpalo::collections::Vec<'a, &'a Comment>,

    /// List of magic comments extracted from the source code.
    pub magic_comments: bumpalo::collections::Vec<'a, &'a MagicComment>,

    /// Input that was used for parsing.
    ///
    /// Note: this input is not necessary the same byte array that
    /// you passed to Parser::parse. If encoding of the input is
    /// not `UTF-8` or `ASCII-8BIT/BINARY` Parser invokes `decoder`
    /// that usually produces a different sequence of bytes.
    ///
    /// Pass **this** data to `Loc::source`, otherwise you'll get
    /// incorrect source ranges.
    pub input: DecodedInput<'a>,
}

impl<'a> ParserResult<'a> {
    pub(crate) fn new(
        ast: Option<&'a Node<'a>>,
        tokens: bumpalo::collections::Vec<'a, &'a Token<'a>>,
        diagnostics: bumpalo::collections::Vec<'a, &'a Diagnostic>,
        comments: bumpalo::collections::Vec<'a, &'a Comment>,
        magic_comments: bumpalo::collections::Vec<'a, &'a MagicComment>,
        input: DecodedInput,
    ) -> Self {
        Self {
            ast,
            tokens,
            diagnostics,
            comments,
            magic_comments,
            input,
        }
    }

    /// Returns `ast` attribute
    pub fn ast(&self) -> &Option<&'a Node<'a>> {
        &self.ast
    }
    /// Returns `tokens` attribute
    pub fn tokens(&self) -> &bumpalo::collections::Vec<'a, &'a Token<'a>> {
        &self.tokens
    }
    /// Returns `diagnostics` attribute
    pub fn diagnostics(&self) -> &bumpalo::collections::Vec<'a, &'a Diagnostic> {
        &self.diagnostics
    }
    /// Returns `comments` attribute
    pub fn comments(&self) -> &bumpalo::collections::Vec<'a, &'a Comment> {
        &self.comments
    }
    /// Returns `magic_comments` attribute
    pub fn magic_comments(&self) -> &bumpalo::collections::Vec<'a, &'a MagicComment> {
        &self.magic_comments
    }
    /// Returns `input` attribute
    pub fn input(&self) -> &DecodedInput {
        &self.input
    }
}
