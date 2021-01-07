use std::fmt;

/// This indicates the level of the error, to handle it differently based on the level of the raised error.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorLevel {
    /// An error that breaks the execution of the program.
    BreakingError,
    /// An error that invalidates the output of the function generating the error. So things like invalid
    /// characters, numeric literals etc.
    InvalidatingError,
    /// A warning that invalidates some strict invariants posed by the specification. Which do not necessarily
    /// prevent the code from running, but will need to be checked.
    StrictWarning,
    /// A warning that invalidates some looser defined invariants. Which are generally bad but sometimes occur
    /// due to other software packages not following the specifications to the letter.
    LooseWarning,
    /// A general warning.
    GeneralWarning,
}

impl ErrorLevel {
    /// Get the descriptor for this ErrorLevel (Error/Warning). This can be used to display to users to indicate
    /// the severity of the error.
    pub fn descriptor(&self) -> &str {
        match self {
            ErrorLevel::BreakingError => "Error",
            ErrorLevel::InvalidatingError => "Error",
            ErrorLevel::StrictWarning => "Warning",
            ErrorLevel::LooseWarning => "Warning",
            ErrorLevel::GeneralWarning => "Warning",
        }
    }
}

impl fmt::Display for ErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.descriptor())
    }
}
