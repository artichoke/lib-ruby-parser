crate::use_native_or_external!(Vec);
crate::use_native_or_external!(String);

use bumpalo::Bump;

use super::DecodedInput;
use crate::source::SourceLine;

#[test]
fn test_new() {
    let bump = Bump::new();
    let decoded = DecodedInput::named(&bump, String::from_str_in("foo", &bump));
    assert_eq!(decoded.name(), "foo");
}

fn decoded_input<'a>(bump: &'a Bump) -> DecodedInput<'a> {
    let mut decoded = DecodedInput::named(&bump, String::from_str_in("foo", &bump));
    decoded.set_bytes(bump_vec![in &bump; 1, 2, 3]);
    decoded.set_lines(bump_vec![in &bump; SourceLine::new(1, 2, true)]);
    decoded
}

#[test]
fn test_settter() {
    let bump = Bump::new();
    let decoded = decoded_input(&bump);

    assert_eq!(decoded.bytes(), &bump_vec![in &bump; 1, 2, 3]);
    assert_eq!(
        decoded.lines(),
        &bump_vec![in &bump; SourceLine::new(1, 2, true)]
    );
}

#[test]
fn test_debug() {
    let bump = Bump::new();
    let decoded = decoded_input(&bump);

    assert_eq!(
        format!("{:?}", decoded),
        "DecodedInput { name: \"foo\", lines: [SourceLine { start: 1, end: 2, ends_with_eof: true }], bytes: [1, 2, 3] }"
    );
}

#[test]
fn test_take_bytes() {
    let bump = Bump::new();
    let mut decoded = decoded_input(&bump);

    assert_eq!(decoded.take_bytes(), bump_vec![in &bump; 1, 2, 3]);
    assert_eq!(decoded.take_bytes(), bump_vec![in &bump; ]);
}

#[test]
fn test_into_bytes() {
    let bump = Bump::new();
    let decoded = decoded_input(&bump);

    assert_eq!(decoded.into_bytes(), bump_vec![in &bump; 1, 2, 3]);
}
