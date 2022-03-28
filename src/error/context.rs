use std::fmt;

/// A struct to define the context of an error message
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Context {
    /// When no context can be given
    None,
    /// When only a line (e.g. in a file) can be shown
    Show {
        /// The line to be shown to the user (e.g. filename)
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
    /// To show multiple lines where an error occurred.
    RangeHighlights {
        /// The linenumber of the first line
        start_linenumber: usize,
        /// The lines to show
        lines: Vec<String>,
        /// Highlights defined by the line (relative to the set of lines given), start column in that line and length of highlight
        highlights: Vec<(usize, usize, usize)>,
    },
    /// To show multiple contexts
    Multiple {
        /// The contexts to show
        contexts: Vec<(Option<String>, Context)>,
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
    pub fn position(pos: &Position<'_>) -> Context {
        if pos.text.is_empty() {
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
    pub fn range(start: &Position<'_>, end: &Position<'_>) -> Context {
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
                lines: start
                    .text
                    .lines()
                    .into_iter()
                    .take(end.line - start.line)
                    .map(ToString::to_string)
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
                write!(f, "\n     |")?;
                let mut number = *start_linenumber;
                write!(f, "\n{:<4} | {}{}", number, " ".repeat(*offset), lines[0])?;
                for line in lines.iter().skip(1) {
                    number += 1;
                    write!(f, "\n{:<4} | {}", number, line)?;
                }
                write!(f, "\n     |")
            }
            Context::RangeHighlights {
                start_linenumber,
                lines,
                highlights,
            } => {
                write!(f, "\n     |")?;
                let mut number = *start_linenumber;
                let mut highlights_peek = highlights.iter().peekable();
                #[allow(unused)]
                for (index, line) in lines.iter().enumerate() {
                    number += 1;
                    write!(f, "\n{:<4} | {}", number, line)?;
                    let mut first = true;
                    let mut last_offset = 0;
                    while let Some(high) = highlights_peek.peek() {
                        if high.0 > index {
                            break;
                        }
                        if let Some(high) = highlights_peek.next() {
                            if first {
                                write!(f, "\n     | ")?;
                                first = false;
                            }
                            if last_offset < high.1 {
                                write!(
                                    f,
                                    "{}{}",
                                    " ".repeat(high.1 - last_offset),
                                    "^".repeat(high.2)
                                )?;
                                last_offset = high.1 + high.2;
                            } else {
                                println!("A highlight in a range error message is detected to overlap with a previous highlight, it is skipped.");
                                // Panicking on error gave the following very intense error message (in test code):
                                // `thread panicked while panicking. aborting. ... (exit code: 0xc000001d, STATUS_ILLEGAL_INSTRUCTION)`
                                // To prevent other people from panicking upon seeing this error message this error is not raised currently.
                            }
                        }
                    }
                }
                write!(f, "\n     |")
            }
            Context::Multiple { contexts } => {
                for (note, context) in contexts {
                    writeln!(f, "{}", context)?;
                    if let Some(note) = note {
                        writeln!(f, "={}", note)?;
                    }
                }
                Ok(())
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
