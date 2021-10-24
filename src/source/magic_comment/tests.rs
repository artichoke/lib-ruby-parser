use bumpalo::Bump;

use super::MagicComment;
use crate::source::MagicCommentKind;
use crate::Loc;

fn new_magic_comment<'a>(bump: &'a Bump) -> MagicComment {
    MagicComment::new(
        MagicCommentKind::frozen_string_literal(),
        Loc::new(1, 2),
        Loc::new(3, 4),
    )
}

#[test]
fn test_new() {
    let bump = Bump::new();

    let magic_comment = new_magic_comment(&bump);

    assert!(magic_comment.kind().is_frozen_string_literal());
    assert_eq!(magic_comment.key_l().begin(), 1);
    assert_eq!(magic_comment.key_l().end(), 2);
    assert_eq!(magic_comment.value_l().begin(), 3);
    assert_eq!(magic_comment.value_l().end(), 4);
}

#[test]
fn test_debug() {
    let bump = Bump::new();

    let magic_comment = new_magic_comment(&bump);

    assert_eq!(
        format!("{:?}", magic_comment),
        "MagicComment { kind: FrozenStringLiteral, key_l: 1...2, value_l: 3...4 }"
    )
}

#[test]
fn test_cmp() {
    let bump = Bump::new();

    let magic_comment = new_magic_comment(&bump);

    assert_eq!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc::new(1, 2),
            Loc::new(3, 4),
        )
    );

    assert_ne!(
        magic_comment,
        MagicComment::new(MagicCommentKind::encoding(), Loc::new(1, 2), Loc::new(3, 4),)
    );

    assert_ne!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc::new(0, 2),
            Loc::new(3, 4),
        )
    );

    assert_ne!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc::new(1, 0),
            Loc::new(3, 4),
        )
    );

    assert_ne!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc::new(1, 2),
            Loc::new(0, 4),
        )
    );

    assert_ne!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc::new(1, 2),
            Loc::new(3, 0),
        )
    );
}

#[test]
fn test_clone() {
    let bump = Bump::new();

    let magic_comment = new_magic_comment(&bump);

    assert_eq!(magic_comment, magic_comment.clone())
}
