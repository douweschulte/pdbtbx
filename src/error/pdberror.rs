use super::Context;
use super::ErrorLevel;
use crate::StrictnessLevel;
use std::cmp::Ordering;
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
        short_desc: impl std::string::ToString,
        long_descr: impl std::string::ToString,
        context: Context,
    ) -> PDBError {
        PDBError {
            level,
            short_description: short_desc.to_string(),
            long_description: long_descr.to_string(),
            context,
        }
    }

    /// The level of the error
    pub const fn level(&self) -> ErrorLevel {
        self.level
    }

    /// Tests if this errors is breaking with the given strictness level
    pub fn fails(&self, level: StrictnessLevel) -> bool {
        self.level.fails(level)
    }

    /// Gives the short description or title for this error
    pub fn short_description(&self) -> &str {
        &self.short_description
    }

    /// Gives the long description for this error
    pub fn long_description(&self) -> &str {
        &self.long_description
    }

    /// Gives the context for this error
    pub const fn context(&self) -> &Context {
        &self.context
    }
}

impl fmt::Debug for PDBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}{}\n{}\n",
            self.level, self.short_description, self.context, self.long_description
        )
    }
}

impl fmt::Display for PDBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}{}\n{}\n",
            self.level, self.short_description, self.context, self.long_description
        )
    }
}

impl error::Error for PDBError {}

impl PartialOrd for PDBError {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PDBError {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level)
    }
}

#[cfg(test)]
#[allow(clippy::print_stdout)]
mod tests {
    use super::*;
    use crate::Position;

    #[test]
    fn create_empty_error() {
        let a = PDBError::new(ErrorLevel::GeneralWarning, "test", "test", Context::none());
        println!("{}", a);
        assert_eq!(format!("{}", a), "GeneralWarning: test\ntest\n");
        assert_eq!(a.level(), ErrorLevel::GeneralWarning);
        assert!(!a.fails(StrictnessLevel::Loose));
    }

    #[test]
    fn create_full_line_error() {
        let a = PDBError::new(
            ErrorLevel::StrictWarning,
            "test",
            "test",
            Context::full_line(1, "testing line"),
        );
        println!("{}", a);
        assert_eq!(
            format!("{}", a),
            "StrictWarning: test\n  ╷\n1 │ testing line\n  ╵\ntest\n"
        );
        assert_eq!(a.level(), ErrorLevel::StrictWarning);
        assert!(a.fails(StrictnessLevel::Strict));
    }

    #[test]
    fn create_range_error() {
        let pos1 = Position {
            text: "hello world\nthis is a multiline\npiece of teXt",
            line: 1,
            column: 0,
        };
        let pos2 = Position {
            text: "",
            line: 4,
            column: 13,
        };
        let a = PDBError::new(
            ErrorLevel::LooseWarning,
            "test",
            "test error",
            Context::range(&pos1, &pos2),
        );
        println!("{}", a);
        assert_eq!(format!("{}", a), "LooseWarning: test\n  ╷\n1 │ hello world\n2 │ this is a multiline\n3 │ piece of teXt\n  ╵\ntest error\n");
        assert_eq!(a.level(), ErrorLevel::LooseWarning);
        assert!(a.fails(StrictnessLevel::Strict));
        assert_eq!(pos2.text, "");
        assert_eq!(pos2.line, 4);
        assert_eq!(pos2.column, 13);
    }

    #[test]
    fn ordering_and_equality() {
        let a = PDBError::new(ErrorLevel::GeneralWarning, "test", "test", Context::none());
        let b = PDBError::new(ErrorLevel::LooseWarning, "test", "test", Context::none());
        let c = PDBError::new(ErrorLevel::LooseWarning, "test", "test", Context::none());
        let d = PDBError::new(ErrorLevel::BreakingError, "test", "test", Context::none());
        assert_ne!(a, b);
        assert_eq!(b, c);
        assert_ne!(c, d);
        assert!(a > b);
        assert!(c > d);
        assert!(c < a);
    }
}
