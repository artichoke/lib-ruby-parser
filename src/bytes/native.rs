use bumpalo::Bump;

/// Representation of a byte sequence
#[derive(Clone)]
#[repr(C)]
pub struct Bytes<'a> {
    pub(crate) bump: &'a Bump,

    /// Raw vector of bytes
    pub raw: bumpalo::collections::Vec<'a, u8>,
}

impl PartialEq for Bytes<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Eq for Bytes<'_> {}

impl std::fmt::Debug for Bytes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bytes").field("raw", &self.raw).finish()
    }
}

// impl Default for Bytes<'_> {
//     fn default() -> Self {
//         Self::new(vec![])
//     }
// }

impl<'a> Bytes<'a> {
    /// Constructs Bytes based on a given vector
    pub fn new(bump: &'a Bump, raw: bumpalo::collections::Vec<'a, u8>) -> Bytes<'a> {
        Self { bump, raw }
    }

    pub(crate) fn prepend(&mut self, part: &[u8]) {
        let initial = self.raw.split_off(0);
        self.raw.clear();
        self.raw.extend_from_slice(part);
        self.raw.extend_from_slice(initial.as_slice());
    }

    /// Returns a reference to inner data
    pub fn as_raw(&self) -> &bumpalo::collections::Vec<'a, u8> {
        &self.raw
    }

    /// "Unwraps" self and returns inner data
    pub fn into_raw(self) -> bumpalo::collections::Vec<'a, u8> {
        self.raw
    }

    /// Replaces inner data with given Vec
    pub fn set_raw(&mut self, raw: bumpalo::collections::Vec<'a, u8>) {
        self.raw = raw
    }

    /// Appends a byte
    pub fn push(&mut self, item: u8) {
        self.raw.push(item);
    }

    pub(crate) fn take(&mut self) -> Self {
        Self {
            raw: self.raw.split_off(0),
            bump: self.bump,
        }
    }
}

impl std::ops::Index<usize> for Bytes<'_> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.raw.index(index)
    }
}
