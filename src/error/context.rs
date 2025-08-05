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
    pub const fn none() -> Self {
        Self::None
    }

    /// Creates a new context when only a line (eg filename) can be shown
    pub fn show(line: impl ToString) -> Self {
        Self::Show {
            line: line.to_string(),
        }
    }

    /// Creates a new context when a full line is faulty and no special position can be annotated
    pub fn full_line(linenumber: usize, line: impl ToString) -> Self {
        Self::FullLine {
            linenumber,
            line: line.to_string(),
        }
    }

    /// Creates a new context when a special position can be annotated on a line
    pub fn line(linenumber: usize, line: impl ToString, offset: usize, length: usize) -> Self {
        Self::Line {
            linenumber,
            line: line.to_string(),
            offset,
            length,
        }
    }

    /// Creates a new context to highlight a certain position
    #[allow(clippy::unwrap_used)]
    pub fn position(pos: &Position<'_>) -> Self {
        if pos.text.is_empty() {
            Self::Line {
                linenumber: pos.line,
                line: String::new(),
                offset: 0,
                length: 3,
            }
        } else {
            Self::Line {
                linenumber: pos.line,
                line: pos.text.lines().next().unwrap().to_string(),
                offset: 0,
                length: 3,
            }
        }
    }

    /// Creates a new context from a start and end point within a single file
    pub fn range(start: &Position<'_>, end: &Position<'_>) -> Self {
        if start.line == end.line {
            Self::Line {
                linenumber: start.line,
                line: start.text[..(end.column - start.column)].to_string(),
                offset: start.column,
                length: end.column - start.column,
            }
        } else {
            Self::Range {
                start_linenumber: start.line,
                lines: start
                    .text
                    .lines()
                    .take(end.line - start.line)
                    .map(ToString::to_string)
                    .collect::<Vec<String>>(),
                offset: start.column,
            }
        }
    }

    /// Display this context, with an optional note after the context.
    fn display(&self, f: &mut fmt::Formatter<'_>, note: Option<&str>) -> fmt::Result {
        let mut tail = true; // End with a tailing line ╵
        #[allow(
            clippy::cast_sign_loss,
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation
        )]
        let get_margin = |n| ((n + 1) as f64).log10().max(1.0).ceil() as usize;
        let margin = match self {
            Self::None => 0,
            Self::Show { .. } => 2,
            Self::FullLine { linenumber: n, .. } => get_margin(*n),
            Self::Line { linenumber: n, .. } => get_margin(*n),
            Self::Range {
                start_linenumber: n,
                lines: l,
                ..
            } => get_margin(n + l.len()),
            Self::RangeHighlights {
                start_linenumber: n,
                lines: l,
                ..
            } => get_margin(n + l.len()),
            Self::Multiple { .. } => 0,
        };
        match self {
            Self::None => {
                return Ok(());
            }
            Self::Show { line } => {
                write!(f, "\n{:pad$} ╷\n{:pad$} │ {}", "", "", line, pad = margin)?
            }
            Self::FullLine { linenumber, line } => write!(
                f,
                "\n{:pad$} ╷\n{:<pad$} │ {}",
                "",
                linenumber,
                line,
                pad = margin
            )?,
            Self::Line {
                linenumber,
                line,
                offset,
                length,
            } => write!(
                f,
                "\n{:pad$} ╷\n{:<pad$} │ {}\n{:pad$} · {}{}",
                "",
                linenumber,
                line,
                "",
                " ".repeat(*offset),
                "─".repeat(*length),
                pad = margin
            )?,
            Self::Range {
                start_linenumber,
                lines,
                offset,
            } => {
                write!(f, "\n{:pad$} ╷", "", pad = margin)?;
                let mut number = *start_linenumber;
                write!(
                    f,
                    "\n{:<pad$} │ {}{}",
                    number,
                    " ".repeat(*offset),
                    lines[0],
                    pad = margin
                )?;
                for line in lines.iter().skip(1) {
                    number += 1;
                    write!(f, "\n{number:<margin$} │ {line}")?;
                }
            }
            Self::RangeHighlights {
                start_linenumber,
                lines,
                highlights,
            } => {
                write!(f, "\n{:pad$} ╷", "", pad = margin)?;
                let mut number = *start_linenumber;
                let mut highlights_peek = highlights.iter().peekable();
                #[allow(unused)]
                for (index, line) in lines.iter().enumerate() {
                    number += 1;
                    write!(f, "\n{number:<margin$} │ {line}")?;
                    let mut first = true;
                    let mut last_offset = 0;
                    while let Some(high) = highlights_peek.peek() {
                        if high.0 > index {
                            break;
                        }
                        if let Some(high) = highlights_peek.next() {
                            if first {
                                write!(f, "\n{:pad$} · ", "", pad = margin)?;
                                first = false;
                            }
                            if last_offset < high.1 {
                                write!(
                                    f,
                                    "{}{}",
                                    " ".repeat(high.1 - last_offset),
                                    "─".repeat(high.2)
                                )?;
                                last_offset = high.1 + high.2;
                            } else {
                                eprintln!("A highlight in a range error message is detected to overlap with a previous highlight, it is skipped.");
                                // Panicking on error gave the following very intense error message (in test code):
                                // `thread panicked while panicking. aborting. ... (exit code: 0xc000001d, STATUS_ILLEGAL_INSTRUCTION)`
                                // To prevent other people from panicking upon seeing this error message this error is not raised currently.
                            }
                        }
                    }
                }
            }
            Self::Multiple { contexts } => {
                for (note, context) in contexts {
                    context.display(f, note.as_deref())?;
                }
                tail = false;
            }
        }
        // Last line
        if let Some(note) = note {
            write!(f, "\n{:pad$} ╰{}", "", note, pad = margin)
        } else if tail {
            write!(f, "\n{:pad$} ╵", "", pad = margin)
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, None)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// A position in a file for use in parsing/lexing
pub struct Position<'a> {
    /// The remaining text (as ref so no copies)
    pub text: &'a str,
    /// The current linenumber
    pub line: usize,
    /// The current column number
    pub column: usize,
}
