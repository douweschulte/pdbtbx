use std::fmt;

/// A struct to define the context of an error message
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Context {
    /// When no context can be given
    None,
    /// When only a line (eg filename) can be shown
    Show {
        /// The line to be shown to the user (eg filename)
        line: String,
    },
    /// When a full line is faulty and no special position can be annotated
    FullLine {
        /// The line number to recognise where the error is located
        linenumber: usize,
        /// The line to show the issue itself
        line: String,
    },
    /// When a special position can be annotated on a line.
    /// ```text
    ///      |
    /// 104  | ATOM      O  N   MET A   1      27.251  24.447   2.594  1.00 11.79           N
    ///      |        ^^^^
    ///        <-   -><-->
    /// ```
    /// The first space (annotated by `<-`, `->`) is the offset, in this case 7. The
    /// second space is the length, in this case 4.
    Line {
        /// The line number to recognise where the error is located.
        linenumber: usize,
        /// The line to show the issue itself.
        line: String,
        /// The offset of the special position to be annotated.
        offset: usize,
        /// The length of the special position to be annotated.
        length: usize,
    },
    /// To show multiple lines where an error occurred.
    Range {
        /// The linenumber of the first line
        start_linenumber: usize,
        /// The lines to show
        lines: Vec<String>,
        /// The possible offset of the first line, will be padded with spaces
        offset: usize,
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

    /// Creates a new context to highlight a certain position
    #[allow(clippy::unwrap_used)]
    pub fn position(pos: &Position) -> Context {
        if pos.text == "" {
            Context::Line {
                linenumber: pos.line,
                line: "".to_string(),
                offset: 0,
                length: 3,
            }
        } else {
            Context::Line {
                linenumber: pos.line,
                line: pos.text.lines().into_iter().next().unwrap().to_string(),
                offset: 0,
                length: 3,
            }
        }
    }

    /// Creates a new context from a start and end point within a single file
    pub fn range(start: &Position, end: &Position) -> Context {
        if start.line == end.line {
            Context::Line {
                linenumber: start.line,
                line: start.text[..(end.column - start.column)].to_string(),
                offset: start.column,
                length: end.column - start.column,
            }
        } else {
            Context::Range {
                start_linenumber: start.line,
                lines: start.text[..(end.text.len() - start.text.len())]
                    .to_string()
                    .lines()
                    .into_iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>(),
                offset: start.column,
            }
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
            Context::Range {
                start_linenumber,
                lines,
                offset,
            } => {
                write!(f, "\"     |").expect("Fault in writing to output");
                let mut number = *start_linenumber;
                write!(f, "\n{:<4} | {}{}", number, " ".repeat(*offset), lines[0])
                    .expect("Fault in writing to output");
                for line in lines.iter().skip(1) {
                    number += 1;
                    write!(f, "\n{:<4} | {}", number, line).expect("Fault in writing to output");
                }
                write!(f, "\"     |")
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// A position in a file for use in parsing/lexing
pub struct Position<'a> {
    /// The remaining text (as ref so no copies)
    pub text: &'a str,
    /// The current linenumber
    pub line: usize,
    /// The current column number
    pub column: usize,
}
