use crate::containers::Ptr;
use crate::Loc;

/// An enum of all magic comment kinds
#[derive(Debug, Clone, PartialEq)]
pub enum MagicCommentKind {
    /// `# encoding: ... comment`
    Encoding,

    /// `# frozen_string_literal: true/false` comment
    FrozenStringLiteral,

    /// `# warn_ident: true/false` comment
    WarnIndent,

    /// `# shareable_constant_value: ...` comment
    ShareableContstantValue,
}

/// Representation of a magic comment in Ruby
#[derive(Debug, Clone, PartialEq)]
pub struct MagicComment {
    /// Kind of a magic comment
    pub kind: MagicCommentKind,

    /// Location of the "key":
    ///
    /// ```text
    /// # encoding: utf-8
    ///   ~~~~~~~~
    /// ```
    pub key_l: Ptr<Loc>,

    /// Location of the "value":
    ///
    /// ```text
    /// # encoding: utf-8
    ///             ~~~~~
    /// ```
    pub value_l: Ptr<Loc>,
}

impl MagicComment {
    /// Constructor
    pub fn new(kind: MagicCommentKind, key_l: Ptr<Loc>, value_l: Ptr<Loc>) -> Self {
        Self {
            kind,
            key_l,
            value_l,
        }
    }
}
