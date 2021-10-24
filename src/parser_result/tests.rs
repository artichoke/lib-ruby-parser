use super::ParserResult;
use crate::source::{CommentType, MagicCommentKind, SourceLine};
use crate::{
    source::Comment, source::DecodedInput, source::MagicComment, Bytes, Diagnostic,
    DiagnosticMessage, Loc, Node, Token,
};
use crate::{ErrorLevel, LexState};

crate::use_native_or_external!(Maybe);
crate::use_native_or_external!(Vec);
crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(String);

fn ast<'a>(bump: &'a bumpalo::Bump) -> Maybe<&'a Node<'a>> {
    Maybe::some(Node::new_retry(bump, Loc::new(1, 2)))
}

fn tokens<'a>(bump: &'a bumpalo::Bump) -> Vec<'a, Token<'a>> {
    bump_vec![in bump; Token::new(
        bump,
        280,
        Bytes::new(bump, bump_vec![in bump; 97, 98, 99]),
        Loc::new(3, 4),
        LexState { value: 1 },
        LexState { value: 2 },
    )]
}

fn diagnostics<'a>(bump: &'a bumpalo::Bump) -> Vec<Diagnostic> {
    bump_vec![in bump; Diagnostic::new(
        ErrorLevel::error(),
        DiagnosticMessage::new_alias_nth_ref(),
        Loc::new(5, 6),
    )]
}

fn comments<'a>(bump: &'a bumpalo::Bump) -> Vec<Comment> {
    bump_vec![in bump; Comment::make(Loc::new(7, 8), CommentType::inline())]
}
fn magic_comments<'a>(bump: &'a bumpalo::Bump) -> Vec<MagicComment> {
    bump_vec![in bump; MagicComment::new(
        MagicCommentKind::warn_indent(),
        Loc::new(9, 10),
        Loc::new(11, 12),
    )]
}
fn input<'a>(bump: &'a bumpalo::Bump) -> DecodedInput<'a> {
    let mut input = DecodedInput::named(bump, String::from_str_in("foo", bump));
    input.set_bytes(bump_vec![in bump; 1, 2, 3]);
    input.set_lines(bump_vec![in bump; SourceLine::new(1, 2, false)]);
    input
}

fn parser_options<'a>(bump: &'a bumpalo::Bump) -> ParserResult<'a> {
    ParserResult::new(
        ast(bump),
        tokens(bump),
        diagnostics(bump),
        comments(bump),
        magic_comments(bump),
        input(bump),
    )
}

#[test]
fn test_new() {
    let bump = bumpalo::Bump::new();
    let parser_options = parser_options(&bump);
    drop(parser_options);
}

#[test]
fn test_debug() {
    let bump = bumpalo::Bump::new();
    assert_eq!(
        format!("{:?}", parser_options(&bump)),
        "ParserResult { \
ast: Some(Retry(Retry { expression_l: 1...2 })), \
tokens: [[kIN, \"abc\", 3...4]], \
diagnostics: [Diagnostic { level: error, message: AliasNthRef(AliasNthRef), loc: 5...6 }], \
comments: [Comment { location: 7...8, kind: Inline }], \
magic_comments: [MagicComment { kind: WarnIndent, key_l: 9...10, value_l: 11...12 }], \
input: DecodedInput { name: \"foo\", lines: [SourceLine { start: 1, end: 2, ends_with_eof: false }], bytes: [1, 2, 3] } \
}"
    )
}

#[test]
fn test_getters() {
    let bump = bumpalo::Bump::new();
    let parser_options = parser_options(&bump);

    assert_eq!(parser_options.ast(), &ast(&bump));
    assert_eq!(parser_options.tokens(), &tokens(&bump));
    assert_eq!(parser_options.diagnostics(), &diagnostics(&bump));
    assert_eq!(parser_options.comments(), &comments(&bump));
    assert_eq!(parser_options.magic_comments(), &magic_comments(&bump));
}
