use bumpalo::Bump;

use super::Comment;
use crate::source::CommentType;
use crate::Loc;

#[test]
fn test_comment_type() {
    let bump = Bump::new();
    let comment = Comment::make(Loc::new(&bump, 1, 2), CommentType::inline());

    assert_eq!(comment.location().begin(), 1);
    assert_eq!(comment.location().end(), 2);
    assert!(comment.kind().is_inline());
}

fn comment<'a>(bump: &'a Bump) -> Comment<'a> {
    Comment::make(Loc::new(&bump, 1, 2), CommentType::document())
}

#[test]
fn test_debug() {
    let bump = Bump::new();
    assert_eq!(
        format!("{:?}", comment(&bump)),
        "Comment { location: 1...2, kind: Document }"
    )
}

#[test]
fn test_compare() {
    let bump = Bump::new();

    assert_eq!(
        Comment::make(Loc::new(&bump, 1, 2), CommentType::document()),
        comment(&bump)
    );

    assert_ne!(
        Comment::make(Loc::new(&bump, 2, 2), CommentType::document()),
        comment(&bump)
    );

    assert_ne!(
        Comment::make(Loc::new(&bump, 1, 3), CommentType::document()),
        comment(&bump)
    );

    assert_ne!(
        Comment::make(Loc::new(&bump, 1, 2), CommentType::inline()),
        comment(&bump)
    );
}

#[test]
fn test_clone() {
    let bump = Bump::new();

    let comment = comment(&bump).clone();
    assert_eq!(comment.location(), Loc::new(&bump, 1, 2));
    assert_eq!(comment.kind(), &CommentType::document());
}
