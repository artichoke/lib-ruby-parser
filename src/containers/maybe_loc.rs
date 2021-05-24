use crate::Loc;

#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    use super::Loc;

    /// Rust-compatible nullable pointer
    pub type MaybeLoc = Option<Loc>;

    use super::MaybeLocSome;
    impl MaybeLocSome for MaybeLoc {
        fn some(loc: Loc) -> Self {
            Some(loc)
        }
    }

    use super::MaybeLocNone;
    impl MaybeLocNone for MaybeLoc {
        fn none() -> Self {
            None
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod c {
    use crate::Loc;

    /// C-compatible Option<Loc>
    #[repr(C)]
    #[derive(PartialEq)]
    pub enum MaybeLoc {
        /// Equivalent of Option::Some
        Some(Loc),

        /// Equivalent of Option::None
        None,
    }

    impl std::fmt::Debug for MaybeLoc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.as_ref(), f)
        }
    }

    impl Clone for MaybeLoc {
        fn clone(&self) -> Self {
            match self.as_ref() {
                Some(loc) => Self::some(loc.clone()),
                None => Self::none(),
            }
        }
    }

    use super::MaybeLocSome;
    impl MaybeLocSome for MaybeLoc {
        fn some(loc: Loc) -> Self {
            Self::Some(loc)
        }
    }

    use super::MaybeLocNone;
    impl MaybeLocNone for MaybeLoc {
        fn none() -> Self {
            Self::None
        }
    }

    impl MaybeLoc {
        /// Equivalent of Option::or_else
        pub fn or_else<F>(self, f: F) -> Self
        where
            F: FnOnce() -> Self,
        {
            if self.is_none() {
                f()
            } else {
                self
            }
        }

        /// Equivalent of Option::unwrap
        pub fn unwrap(self) -> Loc {
            match self {
                MaybeLoc::Some(loc) => loc,
                MaybeLoc::None => panic!("failed to unwrap MaybeLoc::None"),
            }
        }

        /// Equivalent of Option::unwrap_or_else
        pub fn unwrap_or_else<F>(self, f: F) -> Loc
        where
            F: FnOnce() -> Loc,
        {
            match self {
                MaybeLoc::Some(loc) => loc,
                MaybeLoc::None => f(),
            }
        }

        /// Equivalent of Option::expect
        pub fn expect(self, message: &str) -> Loc {
            match self {
                MaybeLoc::Some(loc) => loc,
                MaybeLoc::None => panic!("{}", message),
            }
        }

        /// Equivalent of Option::map
        pub fn map<F>(self, f: F) -> Self
        where
            F: FnOnce(Loc) -> Loc,
        {
            match self {
                MaybeLoc::Some(loc) => Self::Some(f(loc)),
                MaybeLoc::None => Self::None,
            }
        }

        /// Equivalent of Option::is_none
        pub fn is_none(&self) -> bool {
            matches!(self, MaybeLoc::None)
        }

        /// Equivalent of Option::is_some
        pub fn is_some(&self) -> bool {
            matches!(self, MaybeLoc::Some(_))
        }

        /// Equivalent of Option::as_ref
        pub fn as_ref(&self) -> Option<&Loc> {
            match self {
                MaybeLoc::Some(loc) => Some(loc),
                MaybeLoc::None => None,
            }
        }
    }

    impl From<Loc> for MaybeLoc {
        fn from(loc: Loc) -> Self {
            Self::Some(loc)
        }
    }
}

pub(crate) trait MaybeLocSome {
    fn some(value: Loc) -> Self
    where
        Self: Sized;
}

pub(crate) trait MaybeLocNone {
    fn none() -> Self
    where
        Self: Sized;
}
