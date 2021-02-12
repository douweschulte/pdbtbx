use super::Context;
use super::ErrorLevel;
use crate::StrictnessLevel;
use std::error;
use std::fmt;

/// An error surfacing while handling a PDB
#[derive(PartialEq, Clone, Eq)]
pub struct PDBError {
    /// The level of the error, defining how it should be handled
    level: ErrorLevel,
    /// A short description of the error, generally used as title line
    short_description: String,
    /// A longer description of the error, presented below the context to give more information and helpful feedback
    long_description: String,
    /// The context, in the most general sense this produces output which leads the user to the right place in the code or file
    context: Context,
}

impl PDBError {
    /// Create a new PDBError
    ///
    /// ## Arguments
    /// * `level` - The level of the error, defining how it should be handled
    /// * `short_desc` - A short description of the error, generally used as title line
    /// * `long_desc` -  A longer description of the error, presented below the context to give more information and helpful feedback
    /// * `context` - The context, in the most general sense this produces output which leads the user to the right place in the code or file
    pub fn new(
        level: ErrorLevel,
        short_desc: &str,
        long_descr: &str,
        context: Context,
    ) -> PDBError {
        PDBError {
            level,
            short_description: short_desc.to_owned(),
            long_description: long_descr.to_owned(),
            context,
        }
    }

    /// The level of the error
    pub fn level(&self) -> ErrorLevel {
        self.level
    }

    /// Tests if this errors is breaking with the given strictness level
    pub fn fails(&self, level: StrictnessLevel) -> bool {
        self.level.fails(level)
    }
}

impl fmt::Debug for PDBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}\n{}\n{}\n",
            self.level, self.short_description, self.context, self.long_description
        )
    }
}

impl fmt::Display for PDBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}\n{}\n{}\n",
            self.level, self.short_description, self.context, self.long_description
        )
    }
}

impl error::Error for PDBError {}
