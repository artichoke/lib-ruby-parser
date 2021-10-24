crate::use_native_or_external!(Maybe);

mod fixture;
use bumpalo::Bump;
pub(crate) use fixture::test_file;

#[allow(non_snake_case)]
mod gen;
#[allow(non_snake_case)]
mod manual;

use crate::{
    source::{MagicComment, MagicCommentKind},
    Loc, Parser, ParserOptions, ParserResult,
};

macro_rules! fixture_file {
    ($dir:literal, $fixture:ident) => {
        #[test]
        fn $fixture() {
            let fixture_path = format!("{}/{}", $dir, stringify!($fixture));
            test_file(&fixture_path);
        }
    };
}
pub(crate) use fixture_file;

fn parse<'a>(bump: &'a Bump, input: &[u8]) -> ParserResult<'a> {
    let options = ParserOptions::new(
        bumpalo::collections::String::from_str_in("(eval)", &bump),
        Maybe::none(),
        Maybe::none(),
        false,
    );
    let parser = Parser::new(
        &bump,
        bumpalo::collections::Vec::from_iter_in(input.iter().cloned(), &bump),
        options,
    );
    parser.do_parse()
}

#[test]
fn test_magic_comment() {
    let bump = Bump::new();
    let fixture = std::fs::read("fixtures/magic_comments.rb").unwrap();
    let result = parse(&bump, &fixture);
    assert_eq!(
        result.magic_comments.as_slice(),
        &[
            MagicComment::new(
                MagicCommentKind::encoding(),
                Loc::new(2, 10),
                Loc::new(12, 17),
            ),
            MagicComment::new(
                MagicCommentKind::frozen_string_literal(),
                Loc::new(20, 41),
                Loc::new(43, 47),
            ),
            MagicComment::new(
                MagicCommentKind::encoding(),
                Loc::new(50, 56),
                Loc::new(58, 63),
            ),
            MagicComment::new(
                MagicCommentKind::shareable_constant_value(),
                Loc::new(66, 90),
                Loc::new(92, 99),
            ),
            MagicComment::new(
                MagicCommentKind::warn_indent(),
                Loc::new(102, 113),
                Loc::new(115, 119),
            ),
        ]
    );
}
