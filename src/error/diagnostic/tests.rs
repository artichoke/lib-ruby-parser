use bumpalo::Bump;

use super::Diagnostic;
use crate::{DiagnosticMessage, ErrorLevel, Loc};
crate::use_native_or_external!(Vec);
crate::use_native_or_external!(String);

fn new_diagnostic<'a>(bump: &'a Bump) -> Diagnostic<'a> {
    Diagnostic::new(
        ErrorLevel::error(),
        DiagnosticMessage::new_alias_nth_ref(),
        Loc::new(1, 2),
    )
}

#[test]
fn test_new() {
    let bump = Bump::new();

    let diagnostic = new_diagnostic(&bump);
    drop(diagnostic)
}

#[test]
fn test_get_level() {
    let bump = Bump::new();
    assert!(new_diagnostic(&bump).level().is_error())
}

#[test]
fn test_get_message() {
    let bump = Bump::new();
    assert!(new_diagnostic(&bump).message().is_alias_nth_ref())
}

#[test]
fn test_get_loc() {
    let bump = Bump::new();
    let diagnostic = new_diagnostic(&bump);
    assert_eq!(diagnostic.loc().begin(), 1);
    assert_eq!(diagnostic.loc().end(), 2)
}

#[test]
fn test_renders() {
    let bump = Bump::new();
    let source = "line 1\nvery long line 2\n";
    let mut input =
        crate::source::DecodedInput::named(&bump, String::from_str_in("(test_render)", &bump));
    input.update_bytes(Vec::from_iter_in(source.bytes(), &bump));

    let error = Diagnostic::new(
        ErrorLevel::warning(),
        DiagnosticMessage::new_fraction_after_numeric(),
        Loc::new(8, 12),
    );

    assert_eq!(
        error
            .render(&bump, &input)
            .expect("failed to render diagnostic"),
        vec![
            "(test_render):2:1: warning: unexpected fraction part after numeric literal",
            "(test_render):2: very long line 2",
            "(test_render):2:  ^~~~"
        ]
        .join("\n")
    );
}
