#[cfg(test)]
pub(crate) mod dummy_decoder {
    use crate::source::{Decoder, DecoderResult, InputError};
    crate::use_native_or_external!(Vec);
    crate::use_native_or_external!(String);

    pub(crate) fn decoded_output<'a>(bump: &'a Bump) -> Vec<'a, u8> {
        crate::use_native_or_external!(Vec);

        bump_vec![in bump; b'3', b'+', b'3']
    }

    pub(crate) fn decoding_error<'a>(bump: &'a Bump) -> InputError<'a> {
        InputError::new_decoding_error(String::from_str_in("foo", bump))
    }

    #[cfg(feature = "compile-with-external-structures")]
    mod implementation {
        use super::{decoded_output, decoding_error};
        use crate::blobs::{Blob, HasBlob};
        use crate::source::{Decoder, DecoderResult};

        type ExternDecodeFn = extern "C" fn() -> Blob<DecoderResult>;

        extern "C" {
            fn lib_ruby_parser__testing__decoder__new(f: ExternDecodeFn) -> Blob<Decoder>;
        }

        extern "C" fn decode_ok() -> Blob<DecoderResult> {
            DecoderResult::new_ok(decoded_output()).into_blob()
        }

        extern "C" fn decode_err() -> Blob<DecoderResult> {
            DecoderResult::new_err(decoding_error()).into_blob()
        }

        pub(crate) fn ok_decoder() -> Decoder {
            Decoder::from_blob(unsafe { lib_ruby_parser__testing__decoder__new(decode_ok) })
        }

        pub(crate) fn err_decoder() -> Decoder {
            Decoder::from_blob(unsafe { lib_ruby_parser__testing__decoder__new(decode_err) })
        }
    }

    #[cfg(not(feature = "compile-with-external-structures"))]
    mod implementation {
        use bumpalo::Bump;

        use super::{decoded_output, decoding_error};
        use crate::source::{Decoder, DecoderResult};
        crate::use_native_or_external!(Vec);
        crate::use_native_or_external!(String);

        fn decode_ok<'a>(
            bump: &'a Bump,
            _encoding: String<'a>,
            _input: Vec<'a, u8>,
        ) -> DecoderResult<'a> {
            DecoderResult::Ok(decoded_output(bump))
        }

        fn decode_err<'a>(
            bump: &'a Bump,
            _encoding: String,
            _input: Vec<'a, u8>,
        ) -> DecoderResult<'a> {
            DecoderResult::Err(decoding_error(bump))
        }

        pub(crate) fn ok_decoder<'a>(bump: &'a Bump) -> Decoder<'a> {
            Decoder::new(Box::new(move |encoding, input| {
                decode_ok(bump, encoding, input)
            }))
        }

        pub(crate) fn err_decoder<'a>(bump: &'a Bump) -> Decoder<'a> {
            Decoder::new(Box::new(move |encoding, input| {
                decode_err(bump, encoding, input)
            }))
        }
    }

    use bumpalo::Bump;
    pub(crate) use implementation::{err_decoder, ok_decoder};

    pub(crate) fn call_dummy_decoder<'a>(
        bump: &'a Bump,
        decoder: Decoder<'a>,
    ) -> DecoderResult<'a> {
        // it's dummy, so encoding/input doesn't matter
        let encoding = String::from_str_in("UTF-8", &bump);
        let input = bump_vec![in &bump; b'2', b'+', b'2'];

        decoder.call(encoding, input)
    }
}
