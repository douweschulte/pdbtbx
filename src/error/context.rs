use std::fmt;

/// A struct to define the context of an error message
#[derive(Debug, Clone)]
pub enum Context {
    /// When no context can be given
    None,
    /// When only a line (eg filename) can be shown
    Show { line: String },
    /// When a full line is faulty and no special position can be annotated
    FullLine { linenumber: usize, line: String },
    /// When a special position can be annotated on a line
    Line {
        linenumber: usize,
        line: String,
        offset: usize,
        length: usize,
    },
}

impl Context {
    /// Creates a new context when no context can be given
    pub fn none() -> Context {
        Context::None
    }

    /// Creates a new context when only a line (eg filename) can be shown
    pub fn show(line: &str) -> Context {
        Context::Show {
            line: line.to_string(),
        }
    }

    /// Creates a new context when a full line is faulty and no special position can be annotated
    pub fn full_line(linenumber: usize, line: &str) -> Context {
        Context::FullLine {
            linenumber,
            line: line.to_owned(),
        }
    }

    /// Creates a new context when a special position can be annotated on a line
    pub fn line(linenumber: usize, line: &str, offset: usize, length: usize) -> Context {
        Context::Line {
            linenumber,
            line: line.to_string(),
            offset,
            length,
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Context::None => write!(f, ""),
            Context::Show { line } => write!(f, "\n     |\n     | {}\n     |\n", line),
            Context::FullLine { linenumber, line } => {
                write!(f, "\n     |\n{:<4} | {}\n     |\n", linenumber, line)
            }
            Context::Line {
                linenumber,
                line,
                offset,
                length,
            } => write!(
                f,
                "\n     |\n{:<4} | {}\n     | {}{}\n",
                linenumber,
                line,
                " ".repeat(*offset),
                "^".repeat(*length)
            ),
        }
    }
}
