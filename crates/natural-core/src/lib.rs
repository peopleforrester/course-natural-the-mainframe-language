// ABOUTME: Public API of the Natural teaching interpreter core: parse a source program
// ABOUTME: and drive it as a resumable state machine that yields terminal output.

mod error;
mod interp;
mod lexer;
mod parser;
mod value;

pub use error::NaturalError;
pub use interp::{Field, Interpreter, Step};
pub use value::{Format, Value, print_width, render_field};

/// Re-exported so callers can build and compare numeric values without depending on the
/// exact decimal crate version themselves.
pub use rust_decimal::Decimal;

/// What a finished program produced: its terminal output and its final field values.
///
/// The field snapshot is not only for tests. Lesson exercises are checked by inspecting
/// the learner's variables after a run, so this is course-facing API.
#[derive(Debug, Clone)]
pub struct Outcome {
    pub lines: Vec<String>,
    fields: std::collections::BTreeMap<String, Field>,
}

impl Outcome {
    /// The value of a declared field. The name is matched without regard to case.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.fields.get(&parser::normalize(name)).map(|f| &f.value)
    }

    /// The declared format of a field. The name is matched without regard to case.
    pub fn format(&self, name: &str) -> Option<&Format> {
        self.fields.get(&parser::normalize(name)).map(|f| &f.format)
    }

    /// Every declared field name, in a stable order.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.fields.keys().map(String::as_str)
    }
}

/// Runs a program to completion, returning its output and final field values.
pub fn run(source: &str) -> Result<Outcome, NaturalError> {
    let program = parser::parse(source)?;
    let mut interp = Interpreter::new(program);
    let mut lines = Vec::new();
    while let Step::Output(line) = interp.step()? {
        lines.push(line);
    }
    Ok(Outcome {
        lines,
        fields: interp.fields().clone(),
    })
}

/// Runs a program and collects only its terminal output, one entry per line.
///
/// The browser build drives [`Interpreter::step`] directly instead, so that it can hand
/// control back to JavaScript between statements.
pub fn run_to_lines(source: &str) -> Result<Vec<String>, NaturalError> {
    run(source).map(|outcome| outcome.lines)
}
