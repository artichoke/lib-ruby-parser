crate::use_native_or_external!(Vec);
use bumpalo::Bump;

use crate::Bytes;

#[derive(Debug, Clone)]
pub(crate) struct TokenBuf<'a> {
    bump: &'a Bump,
    pub(crate) bytes: Bytes<'a>,
}

impl<'a> TokenBuf<'a> {
    pub(crate) fn new(bump: &'a Bump, bytes: &[u8]) -> Self {
        Self {
            bump,
            bytes: Bytes::new(bump, Vec::from_iter_in(bytes.iter().cloned(), bump)),
        }
    }

    pub(crate) fn take(&mut self) -> Self {
        let mut out = Self::new(self.bump, &[]);
        std::mem::swap(self, &mut out);
        out
    }

    pub(crate) fn push(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub(crate) fn append(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.push(*byte)
        }
    }

    pub(crate) fn prepend(&mut self, part: &[u8]) {
        let mut tmp = Vec::from_iter_in(part.iter().cloned(), self.bump);
        tmp.extend(self.bytes.as_raw().iter());
        self.bytes.set_raw(tmp.into());
    }

    pub(crate) fn borrow_string(&self) -> Result<&str, &[u8]> {
        match std::str::from_utf8(self.bytes.as_raw()) {
            Ok(s) => Ok(s),
            Err(_) => Err(self.bytes.as_raw()),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) fn clear(&mut self) {
        self.bytes.clear()
    }

    pub(crate) fn default(bump: &'a Bump) -> Self {
        Self {
            bump,
            bytes: Bytes::new(bump, Vec::new_in(bump)),
        }
    }
}

impl PartialEq<str> for TokenBuf<'_> {
    fn eq(&self, other: &str) -> bool {
        self.bytes.as_raw().as_slice() == other.as_bytes()
    }
}
