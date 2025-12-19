use context_error::ErrorKind;

use crate::StrictnessLevel;
use std::fmt;

/// This indicates the level of the error, to handle it differently based on the level of the raised error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ErrorLevel {
    /// An error that breaks the execution of the program.
    #[default]
    BreakingError,
    /// An error that invalidates the output of the function generating the error. This concerns things like invalid
    /// characters, numeric literals etc.
    InvalidatingError,
    /// A warning that invalidates some strict invariants posed by the specification. These do not necessarily
    /// prevent the code from running, but will need to be checked.
    StrictWarning,
    /// A warning that invalidates some looser defined invariants. These are generally bad but sometimes occur
    /// due to other software packages not following the specifications to the letter.
    LooseWarning,
    /// A general warning.
    GeneralWarning,
}

impl ErrorKind for ErrorLevel {
    type Settings = StrictnessLevel;

    fn descriptor(&self) -> &'static str {
        match self {
            Self::BreakingError => "BreakingError",
            Self::InvalidatingError => "InvalidatingError",
            Self::StrictWarning => "StrictWarning",
            Self::LooseWarning => "LooseWarning",
            Self::GeneralWarning => "GeneralWarning",
        }
    }

    fn is_error(&self, settings: Self::Settings) -> bool {
        match settings {
            StrictnessLevel::Strict => true,
            StrictnessLevel::Medium => !matches!(self, Self::GeneralWarning),
            StrictnessLevel::Loose => !matches!(self, Self::GeneralWarning | Self::LooseWarning),
        }
    }

    fn ignored(&self, _settings: Self::Settings) -> bool {
        false
    }
}

impl fmt::Display for ErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.descriptor())
    }
}
