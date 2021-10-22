crate::use_native_or_external!(String);

/// An enum with all possible kinds of errors that can be returned
/// from a decoder
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InputError<'a> {
    /// Emitted when no custom decoder provided but input has custom encoding.
    ///
    /// You can return this error from your custom decoder if you don't support given encoding.
    UnsupportedEncoding(String<'a>),

    /// Generic error that can be emitted from a custom decoder
    DecodingError(String<'a>),
}

impl<'a> InputError<'a> {
    /// Constructs UnupportedEncoding variant
    pub fn new_unsupported_encoding(err: String<'a>) -> Self {
        Self::UnsupportedEncoding(err)
    }

    /// Constructs DecodingError variant
    pub fn new_decoding_error(err: String<'a>) -> Self {
        Self::DecodingError(err)
    }
}

#[cfg(test)]
impl<'a> InputError<'a> {
    pub(crate) fn is_unsupported_encoding(&self) -> bool {
        matches!(self, Self::UnsupportedEncoding(_))
    }

    pub(crate) fn is_decoding_error(&self) -> bool {
        matches!(self, Self::DecodingError(_))
    }

    pub(crate) fn get_unsupported_encoding_message(&self) -> &String {
        match self {
            Self::UnsupportedEncoding(message) => message,
            Self::DecodingError(_) => panic!("InputError is DecodingError"),
        }
    }

    pub(crate) fn get_decoding_error_message(&self) -> &String {
        match self {
            Self::DecodingError(message) => message,
            Self::UnsupportedEncoding(_) => panic!("InputError is UnsupportedEncoding"),
        }
    }
}
