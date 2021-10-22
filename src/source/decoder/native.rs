use crate::source::DecoderResult;
use crate::source::InputError;
crate::use_native_or_external!(String);
crate::use_native_or_external!(Vec);

/// Decoder is what is used if input source has encoding
/// that is not supported out of the box.
///
/// Supported encoding are:
/// 1. UTF-8
/// 2. ASCII-8BIT (or BINARY, it's an alias)
///
/// So if your source looks like this:
///
/// ```text
/// # encoding: koi8-r
/// \xFF = 42
/// ```
///
/// you need to provide a decoder that converts this byte sequence
/// into UTF-8 bytes.
///
/// Decoding function
///
/// Takes encoding name and initial input as arguments
/// and returns `Ok(decoded)` vector of bytes or `Err(error)` that will be returned
/// in the `ParserResult::diagnostics` vector.
pub type DecoderFn<'a> = dyn (Fn(String<'a>, Vec<'a, u8>) -> DecoderResult<'a>) + 'a;

/// Custom decoder, a wrapper around a function
pub struct Decoder<'a> {
    f: Box<DecoderFn<'a>>,
}

impl<'a> Decoder<'a> {
    /// Constructs a rewriter based on a given function
    pub fn new(f: Box<DecoderFn<'a>>) -> Self {
        Self { f }
    }

    pub(crate) fn call(&self, encoding: String<'a>, input: Vec<'a, u8>) -> DecoderResult<'a> {
        let f = &*self.f;
        f(encoding, input)
    }
}

impl std::fmt::Debug for Decoder<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder").finish()
    }
}

pub fn decode_input<'a>(
    input: Vec<'a, u8>,
    enc: String<'a>,
    decoder: &mut Option<Decoder<'a>>,
) -> DecoderResult<'a> {
    match enc.to_uppercase().as_str() {
        "UTF-8" | "ASCII-8BIT" | "BINARY" => {
            return DecoderResult::Ok(input.into());
        }
        _ => {
            if let Some(f) = decoder.as_mut() {
                f.call(enc, input)
            } else {
                DecoderResult::Err(InputError::UnsupportedEncoding(enc))
            }
        }
    }
}
