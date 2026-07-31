// ABOUTME: Natural-level diagnostics. Every message names the Natural concept the
// ABOUTME: learner got wrong, never a parser internal, because errors are teaching surfaces.

use thiserror::Error;

/// An error a learner can act on. Variants are deliberately concept-shaped rather than
/// mirroring the interpreter's internal structure.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NaturalError {
    #[error("This program has no END statement. Every Natural program must finish with END.")]
    MissingEnd,

    #[error(
        "Line {line}: a text literal was opened with a quote but never closed. \
         Close it with a matching quote, and write a quote inside text by doubling it."
    )]
    UnterminatedString { line: usize },

    #[error("Line {line}: '{name}' is not a Natural statement this course knows yet.")]
    UnknownStatement { name: String, line: usize },

    #[error(
        "Line {line}: DEFINE DATA must be the first statement in the program. \
         Move the data block above your other statements."
    )]
    DefineDataNotFirst { line: usize },

    #[error("The DEFINE DATA block was never closed. Add END-DEFINE after your last field.")]
    MissingEndDefine,

    #[error(
        "Line {line}: '{name}' has not been declared. \
         Add it to the DEFINE DATA block before you use it."
    )]
    UndeclaredVariable { name: String, line: usize },

    #[error("Line {line}: '{name}' is already declared. Each field needs its own name.")]
    DuplicateVariable { name: String, line: usize },

    #[error("Line {line}: {detail}")]
    InvalidFormat { detail: String, line: usize },

    #[error(
        "Line {line}: the value {value} does not fit in '{name}', which allows \
         {int_digits} position(s) before the decimal point."
    )]
    NumericOverflow {
        name: String,
        value: String,
        int_digits: u32,
        line: usize,
    },

    #[error("Line {line}: cannot assign {source_kind} to '{name}', which is {target_kind}.")]
    IncompatibleAssignment {
        name: String,
        source_kind: String,
        target_kind: String,
        line: usize,
    },

    #[error("Line {line}: {feature} is not available yet in this course build.")]
    NotYetSupported { feature: String, line: usize },

    #[error("Line {line}: '{text}' is not a valid value for '{name}', which holds {expected}.")]
    InvalidInput {
        text: String,
        name: String,
        expected: String,
        line: usize,
    },

    #[error("This program asks for input ({prompt}), so it cannot be run without supplying any.")]
    InputRequired { prompt: String },

    #[error("The program is not waiting for input right now.")]
    NotWaitingForInput,
}
