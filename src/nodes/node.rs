use crate::Loc;
use crate::Node;

impl<'a> Node<'a> {
    /// Returs a whitequark/parser -like representation of `self`.
    ///
    /// Used in tests and example scripts
    pub fn inspect(&'a self, indent: usize) -> String {
        self.inner_ref().inspect(indent)
    }

    // pub fn expression(&'a self) -> &'a Loc {
    //     self.inner_ref().expression()
    // }

    /// Returns a whitequark/parser -like node name.
    ///
    /// Used in tests and example scripts
    pub fn str_type(&'a self) -> &'static str {
        self.inner_ref().str_type()
    }

    /// Prints itself + location information
    pub fn print_with_locs(&'a self) {
        self.inner_ref().print_with_locs()
    }
}
