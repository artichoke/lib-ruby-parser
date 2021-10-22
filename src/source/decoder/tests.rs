crate::use_native_or_external!(Vec);

use bumpalo::Bump;

use super::shared::dummy_decoder::*;
use crate::source::DecoderResult;

#[test]
fn test_decoder_ok() {
    let bump = Bump::new();
    assert_eq!(
        call_dummy_decoder(&bump, ok_decoder(&bump)),
        DecoderResult::new_ok(decoded_output(&bump))
    );
}

#[test]
fn test_decoder_err() {
    let bump = Bump::new();
    assert_eq!(
        call_dummy_decoder(&bump, err_decoder(&bump)),
        DecoderResult::new_err(decoding_error(&bump))
    );
}
