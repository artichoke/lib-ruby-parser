use crate::Loc;
use crate::{DiagnosticMessage, ErrorLevel};

/// Diagnostic message that comes from the parser when there's an error or warning
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Diagnostic<'a> {
    /// Level of the diagnostic (error or warnings)
    pub level: ErrorLevel,
    /// Message of the diagnostic
    pub message: DiagnosticMessage<'a>,
    /// Location of the diagnostic
    pub loc: &'a Loc,
}

impl<'a> Diagnostic<'a> {
    /// Construncts an instance of `Diagnostic`
    pub fn new(level: ErrorLevel, message: DiagnosticMessage<'a>, loc: &'a Loc) -> Self {
        Self {
            level,
            message,
            loc,
        }
    }

    /// Returns `level` field
    pub fn level(&self) -> &ErrorLevel {
        &self.level
    }

    /// Returns `message` field
    pub fn message(&self) -> &DiagnosticMessage {
        &self.message
    }

    /// Returns `loc` field
    pub fn loc(&self) -> &Loc {
        &self.loc
    }
}
