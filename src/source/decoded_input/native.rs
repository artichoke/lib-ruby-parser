use crate::source::SourceLine;
crate::use_native_or_external!(String);
crate::use_native_or_external!(Vec);

/// Decoded input
#[repr(C)]
pub struct DecodedInput<'a> {
    pub(crate) bump: &'a bumpalo::Bump,

    /// Name of the input
    pub name: String<'a>,

    /// Lines list
    pub lines: Vec<'a, SourceLine>,

    /// Decoded bytes
    pub bytes: Vec<'a, u8>,
}

impl std::fmt::Debug for DecodedInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecodedInput")
            .field("name", &self.name)
            .field("lines", &self.lines)
            .field("bytes", &self.bytes)
            .finish()
    }
}

impl<'a> DecodedInput<'a> {
    /// Constructs empty DecodedInput with given name
    pub fn named<Name>(bump: &'a bumpalo::Bump, name: Name) -> Self
    where
        Name: Into<String<'a>>,
    {
        Self {
            bump,
            name: name.into(),
            lines: Vec::new_in(bump),
            bytes: Vec::new_in(bump),
        }
    }

    pub(crate) fn name(&self) -> &String {
        &self.name
    }
    pub(crate) fn lines(&self) -> &Vec<'a, SourceLine> {
        &self.lines
    }
    pub(crate) fn bytes(&'a self) -> &'a Vec<'a, u8> {
        &self.bytes
    }

    #[allow(dead_code)]
    pub(crate) fn set_name(&mut self, name: String<'a>) {
        self.name = name;
    }
    pub(crate) fn set_lines(&mut self, lines: Vec<'a, SourceLine>) {
        self.lines = lines;
    }
    pub(crate) fn set_bytes(&mut self, bytes: Vec<'a, u8>) {
        self.bytes = bytes;
    }

    /// Converts itself into owned vector of bytes
    pub fn into_bytes(self) -> Vec<'a, u8> {
        self.bytes
    }

    pub(crate) fn take_bytes(&mut self) -> Vec<'a, u8> {
        let mut bytes = Vec::<'a, u8>::new_in(self.bump);
        std::mem::swap(&mut self.bytes, &mut bytes);
        bytes
    }
}
