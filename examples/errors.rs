//! Example of how these errors look, for visual debugging

use pdbtbx::*;

fn main() {
    let errors = vec![
        PDBError::new(
            ErrorLevel::BreakingError,
            "Error01:None",
            "General text with how the error came to be.",
            Context::None,
        ),
        PDBError::new(
            ErrorLevel::BreakingError,
            "Error02:Show",
            "General text with how the error came to be.",
            Context::Show {
                line: "line with erroer".to_string(),
            },
        ),
        PDBError::new(
            ErrorLevel::BreakingError,
            "Error03:FullLine",
            "General text with how the error came to be.",
            Context::FullLine {
                linenumber: 99,
                line: "line with erroer".to_string(),
            },
        ),
        PDBError::new(
            ErrorLevel::BreakingError,
            "Error04:Line",
            "General text with how the error came to be.",
            Context::Line {
                linenumber: 100,
                line: "line with erroer".to_string(),
                offset: 14,
                length: 1,
            },
        ),
        PDBError::new(
            ErrorLevel::BreakingError,
            "Error05:Range",
            "General text with how the error came to be.",
            Context::Range {
                start_linenumber: 123,
                lines: vec![
                    "line with erroer".to_string(),
                    "and another erroer".to_string(),
                ],
                offset: 0,
            },
        ),
        PDBError::new(
            ErrorLevel::BreakingError,
            "Error06:RangeHighlights",
            "General text with how the error came to be.",
            Context::RangeHighlights {
                start_linenumber: 123,
                lines: vec![
                    "line with erroer".to_string(),
                    "and another erroer".to_string(),
                ],
                highlights: vec![(0, 14, 1), (1, 16, 1)],
            },
        ),
        PDBError::new(
            ErrorLevel::BreakingError,
            "Error06:Multiple",
            "General text with how the error came to be.",
            Context::Multiple {
                contexts: vec![
                    (
                        Some("Original file (RangeHighlights)".to_string()),
                        Context::RangeHighlights {
                            start_linenumber: 123,
                            lines: vec![
                                "line with erroer".to_string(),
                                "and another erroer".to_string(),
                            ],
                            highlights: vec![(0, 14, 1), (1, 16, 1)],
                        },
                    ),
                    (
                        Some("Lint level (Line)".to_string()),
                        Context::Line {
                            linenumber: 10,
                            line: "#[deny(spelling_mistakes)]".to_string(),
                            offset: 7,
                            length: 17,
                        },
                    ),
                    (
                        Some("Extra stuff (show)".to_string()),
                        Context::Show {
                            line: "See this!".to_string(),
                        },
                    ),
                ],
            },
        ),
    ];
    for error in errors {
        println!("{error}");
    }
}
