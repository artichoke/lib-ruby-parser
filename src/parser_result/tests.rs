use super::ParserResult;
use crate::source::{CommentType, MagicCommentKind, SourceLine};
use crate::{
    source::Comment, source::DecodedInput, source::MagicComment, Bytes, Diagnostic,
    DiagnosticMessage, Loc, Node, Token,
};
use crate::{ErrorLevel, LexState};

crate::use_native_or_external!(Maybe);
crate::use_native_or_external!(List);
crate::use_native_or_external!(Ptr);

fn ast<'a>(bump: &'a bumpalo::Bump) -> Maybe<&'a Node<'a>> {
    Maybe::some(Ptr::new(Node::new_retry(Loc::new(1, 2))))
}

fn tokens<'a>(bump: &'a bumpalo::Bump) -> List<Token<'a>> {
    list![Token::new(
        280,
        Bytes::new(list![97, 98, 99]),
        Loc::new(3, 4),
        LexState { value: 1 },
        LexState { value: 2 },
    )]
}

fn diagnostics<'a>(bump: &'a bumpalo::Bump) -> List<Diagnostic> {
    list![Diagnostic::new(
        ErrorLevel::error(),
        DiagnosticMessage::new_alias_nth_ref(),
        Loc::new(5, 6),
    )]
}

fn comments<'a>(bump: &'a bumpalo::Bump) -> List<Comment> {
    list![Comment::make(Loc::new(7, 8), CommentType::inline())]
}
fn magic_comments<'a>(bump: &'a bumpalo::Bump) -> List<MagicComment> {
    list![MagicComment::new(
        MagicCommentKind::warn_indent(),
        Loc::new(9, 10),
        Loc::new(11, 12),
    )]
}
fn input<'a>(bump: &'a bumpalo::Bump) -> DecodedInput<'a> {
    let mut input = DecodedInput::named("foo");
    input.set_bytes(list![1, 2, 3]);
    input.set_lines(list![SourceLine::new(1, 2, false)]);
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
