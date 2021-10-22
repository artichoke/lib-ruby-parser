use bumpalo::Bump;

use super::InputError;

crate::use_native_or_external!(String);

#[test]
fn test_unsupported_encoding() {
    let bump = Bump::new();
    let err = InputError::new_unsupported_encoding(String::from_str_in("foo", &bump));

    assert!(err.is_unsupported_encoding());
    assert!(!err.is_decoding_error());

    assert_eq!(err.get_unsupported_encoding_message(), "foo");
    assert_eq!(format!("{:?}", err), "UnsupportedEncoding(\"foo\")");
}

#[test]
fn test_decoding_error() {
    let bump = Bump::new();
    let err = InputError::new_decoding_error(String::from_str_in("bar", &bump));

    assert!(err.is_decoding_error());
    assert!(!err.is_unsupported_encoding());

    assert_eq!(err.get_decoding_error_message(), "bar");
    assert_eq!(format!("{:?}", err), "DecodingError(\"bar\")");
}
