use crate::source::InputError;
crate::use_native_or_external!(Vec);

/// Result that is returned from decoding function
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DecoderResult<'a> {
    /// Ok + decoded bytes
    Ok(Vec<'a, u8>),

    /// Err + reason
    Err(InputError<'a>),
}

impl<'a> DecoderResult<'a> {
    /// Constructs `Ok` variant
    pub fn new_ok(output: Vec<'a, u8>) -> Self {
        Self::Ok(output)
    }

    /// Constructs `Err` variant
    pub fn new_err(err: InputError<'a>) -> Self {
        Self::Err(err)
    }

    pub(crate) fn into_result(self) -> Result<Vec<'a, u8>, InputError<'a>> {
        match self {
            Self::Ok(value) => Ok(value),
            Self::Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
impl<'a> DecoderResult<'a> {
    pub(crate) fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    pub(crate) fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    pub(crate) fn as_ok(&self) -> &Vec<'a, u8> {
        match &self {
            Self::Ok(ok) => ok,
            Self::Err(_) => panic!("DecoderResult is Err"),
        }
    }

    pub(crate) fn as_err(&self) -> &InputError {
        match &self {
            Self::Err(err) => err,
            Self::Ok(_) => panic!("DecoderResult is Ok"),
        }
    }
}
