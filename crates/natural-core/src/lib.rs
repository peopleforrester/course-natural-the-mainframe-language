// ABOUTME: Public API of the Natural teaching interpreter core: parse a source program
// ABOUTME: and drive it as a resumable state machine that yields terminal output.

mod data;
mod error;
mod interp;
mod lexer;
mod parser;
mod value;

pub use data::{Database, Ddm, DdmField, Record};
pub use error::NaturalError;
pub use interp::{Field, InputRequest, Interpreter, Step};
pub use parser::{Program, parse as parse_program};
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
    /// Every prompt the program showed, in order. Lessons assert against these to check
    /// that an exercise asked for what it was supposed to ask for.
    pub prompts: Vec<String>,
    fields: std::collections::BTreeMap<String, Field>,
    committed: Database,
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

    /// The database as it stands after the last committed transaction.
    ///
    /// Work a program performed but never committed is deliberately absent, so a lesson
    /// can check whether the learner remembered END TRANSACTION.
    pub fn committed(&self) -> &Database {
        &self.committed
    }
}

/// Runs a program that asks for no input.
///
/// A program containing INPUT fails with [`NaturalError::InputRequired`] rather than
/// silently skipping the read. Use [`run_with_input`] for those.
pub fn run(source: &str) -> Result<Outcome, NaturalError> {
    run_with_input(source, &[])
}

/// Runs a program to completion, answering each INPUT from `inputs` in order.
pub fn run_with_input(source: &str, inputs: &[&str]) -> Result<Outcome, NaturalError> {
    let program = parser::parse(source)?;
    let mut interp = Interpreter::new(program);
    let mut lines = Vec::new();
    let mut prompts = Vec::new();
    let mut supplied = 0;

    loop {
        match interp.step()? {
            Step::Output(line) => lines.push(line),
            Step::NeedsInput(request) => {
                prompts.push(request.prompt.clone());
                let Some(value) = inputs.get(supplied) else {
                    return Err(NaturalError::InputRequired {
                        prompt: request.prompt,
                    });
                };
                supplied += 1;
                interp.provide_input(value)?;
            }
            Step::Done => break,
        }
    }

    Ok(Outcome {
        lines,
        prompts,
        fields: interp.fields().clone(),
        committed: interp.committed().clone(),
    })
}

/// Runs a program and collects only its terminal output, one entry per line.
///
/// The browser build drives [`Interpreter::step`] directly instead, so that it can hand
/// control back to JavaScript between statements.
pub fn run_to_lines(source: &str) -> Result<Vec<String>, NaturalError> {
    run(source).map(|outcome| outcome.lines)
}
