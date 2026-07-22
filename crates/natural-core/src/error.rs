// ABOUTME: Natural-level diagnostics. Every message names the Natural concept the
// ABOUTME: learner got wrong, never a parser internal, because errors are teaching surfaces.

use std::fmt;

/// An error a learner can act on. Variants are deliberately coarse and concept-shaped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NaturalError {
    /// The program never terminated with END.
    MissingEnd,
    /// A quoted literal ran to the end of the line without a closing quote.
    UnterminatedString { line: usize },
    /// The statement keyword is not one this interpreter knows.
    UnknownStatement { name: String, line: usize },
}

impl fmt::Display for NaturalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NaturalError::MissingEnd => write!(
                f,
                "This program has no END statement. Every Natural program must finish with END."
            ),
            NaturalError::UnterminatedString { line } => write!(
                f,
                "Line {line}: a text literal was opened with a quote but never closed. \
                 Close it with a matching quote, and write a quote inside text by doubling it."
            ),
            NaturalError::UnknownStatement { name, line } => write!(
                f,
                "Line {line}: '{name}' is not a Natural statement this course knows yet."
            ),
        }
    }
}

impl std::error::Error for NaturalError {}
