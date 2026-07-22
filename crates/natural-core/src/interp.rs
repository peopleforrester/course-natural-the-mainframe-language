// ABOUTME: Executes a parsed program as a resumable state machine driven by an explicit
// ABOUTME: program counter, never Rust recursion, so it can pause for INPUT in Tier 2.

use crate::error::NaturalError;
use crate::parser::{Program, Statement};

/// One observable effect of advancing the interpreter.
///
/// A `NeedsInput` variant joins this enum when INPUT arrives in Tier 2. The design is
/// already shaped for it: `step` returns to the caller between statements and keeps all
/// execution state in this struct rather than on the Rust call stack, so the browser can
/// hand control back to JavaScript and resume later. See docs/gotchas-rust-wasm.md.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Output(String),
    Done,
}

pub struct Interpreter {
    program: Program,
    pc: usize,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Self { program, pc: 0 }
    }

    /// Advances by one statement. Returns [`Step::Done`] once the program is exhausted.
    pub fn step(&mut self) -> Result<Step, NaturalError> {
        let Some(statement) = self.program.statements.get(self.pc) else {
            return Ok(Step::Done);
        };
        self.pc += 1;

        match statement {
            // WRITE emits its operands separated by a single space. A bare WRITE emits
            // a blank line, which is how Natural programs space their output.
            Statement::Write { operands } => Ok(Step::Output(operands.join(" "))),
        }
    }
}
