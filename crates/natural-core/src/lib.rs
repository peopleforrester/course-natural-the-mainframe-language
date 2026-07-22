// ABOUTME: Public API of the Natural teaching interpreter core: parse a source program
// ABOUTME: and drive it as a resumable state machine that yields terminal output.

mod error;
mod interp;
mod lexer;
mod parser;

pub use error::NaturalError;
pub use interp::{Interpreter, Step};

/// Run a program to completion and collect its terminal output, one entry per line.
///
/// This is the convenience driver used by tests and by the CLI. The browser build
/// drives [`Interpreter::step`] directly instead, so that it can hand control back to
/// JavaScript between statements.
pub fn run_to_lines(source: &str) -> Result<Vec<String>, NaturalError> {
    let program = parser::parse(source)?;
    let mut interp = Interpreter::new(program);
    let mut lines = Vec::new();
    while let Step::Output(line) = interp.step()? {
        lines.push(line);
    }
    Ok(lines)
}
