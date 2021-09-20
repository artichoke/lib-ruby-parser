use crate::blobs::{Blob, HasBlob};
use crate::containers::{ExternalList as List, ExternalMaybe as Maybe, ExternalPtr as Ptr};

use crate::source::Comment;
use crate::source::DecodedInput;
use crate::source::MagicComment;
use crate::Diagnostic;
use crate::Node;
use crate::Token;

/// Combination of all data that `Parser` can give you
#[repr(C)]
pub struct ParserResult {
    pub(crate) blob: Blob<ParserResult>,
}

extern "C" {
    fn lib_ruby_parser__external__parser_result__new(
        ast: Blob<Maybe<Ptr<Node>>>,
        tokens: Blob<List<Token>>,
        diagnostics: Blob<List<Diagnostic>>,
        comments: Blob<List<Comment>>,
        magic_comments: Blob<List<MagicComment>>,
        input: Blob<DecodedInput>,
    ) -> Blob<ParserResult>;
    fn lib_ruby_parser__external__parser_result__drop(blob: *mut Blob<ParserResult>);
    fn lib_ruby_parser__external__parser_result__get_ast(
        blob: *const Blob<ParserResult>,
    ) -> *const Blob<Maybe<Ptr<Node>>>;
    fn lib_ruby_parser__external__parser_result__get_tokens(
        blob: *const Blob<ParserResult>,
    ) -> *const Blob<List<Token>>;
    fn lib_ruby_parser__external__parser_result__get_diagnostics(
        blob: *const Blob<ParserResult>,
    ) -> *const Blob<List<Diagnostic>>;
    fn lib_ruby_parser__external__parser_result__get_comments(
        blob: *const Blob<ParserResult>,
    ) -> *const Blob<List<Comment>>;
    fn lib_ruby_parser__external__parser_result__get_magic_comments(
        blob: *const Blob<ParserResult>,
    ) -> *const Blob<List<MagicComment>>;
    fn lib_ruby_parser__external__parser_result__get_input(
        blob: *const Blob<ParserResult>,
    ) -> *const Blob<DecodedInput>;
}

impl Drop for ParserResult {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__parser_result__drop(&mut self.blob) }
    }
}

impl ParserResult {
    pub(crate) fn new(
        ast: Maybe<Ptr<Node>>,
        tokens: List<Token>,
        diagnostics: List<Diagnostic>,
        comments: List<Comment>,
        magic_comments: List<MagicComment>,
        input: DecodedInput,
    ) -> Self {
        let blob = unsafe {
            lib_ruby_parser__external__parser_result__new(
                ast.into_blob(),
                tokens.into_blob(),
                diagnostics.into_blob(),
                comments.into_blob(),
                magic_comments.into_blob(),
                input.into_blob(),
            )
        };
        Self { blob }
    }

    /// Returns `ast` attribute
    pub fn ast(&self) -> &Maybe<Ptr<Node>> {
        unsafe {
            (lib_ruby_parser__external__parser_result__get_ast(&self.blob)
                as *const Maybe<Ptr<Node>>)
                .as_ref()
                .unwrap()
        }
    }
    /// Returns `tokens` attribute
    pub fn tokens(&self) -> &List<Token> {
        unsafe {
            (lib_ruby_parser__external__parser_result__get_tokens(&self.blob) as *const List<Token>)
                .as_ref()
                .unwrap()
        }
    }
    /// Returns `diagnostics` attribute
    pub fn diagnostics(&self) -> &List<Diagnostic> {
        unsafe {
            (lib_ruby_parser__external__parser_result__get_diagnostics(&self.blob)
                as *const List<Diagnostic>)
                .as_ref()
                .unwrap()
        }
    }
    /// Returns `comments` attribute
    pub fn comments(&self) -> &List<Comment> {
        unsafe {
            (lib_ruby_parser__external__parser_result__get_comments(&self.blob)
                as *const List<Comment>)
                .as_ref()
                .unwrap()
        }
    }
    /// Returns `magic_comments` attribute
    pub fn magic_comments(&self) -> &List<MagicComment> {
        unsafe {
            (lib_ruby_parser__external__parser_result__get_magic_comments(&self.blob)
                as *const List<MagicComment>)
                .as_ref()
                .unwrap()
        }
    }
    /// Returns `input` attribute
    pub fn input(&self) -> &DecodedInput {
        unsafe {
            (lib_ruby_parser__external__parser_result__get_input(&self.blob) as *const DecodedInput)
                .as_ref()
                .unwrap()
        }
    }
}

impl std::fmt::Debug for ParserResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParserResult")
            .field("ast", self.ast())
            .field("tokens", self.tokens())
            .field("diagnostics", self.diagnostics())
            .field("comments", self.comments())
            .field("magic_comments", self.magic_comments())
            .field("input", self.input())
            .finish()
    }
}
