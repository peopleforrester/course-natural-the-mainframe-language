// ABOUTME: Executes a parsed program as a resumable state machine driven by an explicit
// ABOUTME: program counter, never Rust recursion, so it can pause for INPUT in Tier 2.

use crate::error::NaturalError;
use crate::parser::{Operand, Program, Statement, WriteItem};
use crate::value::{Format, Value, coerce, render_field};
use std::collections::BTreeMap;

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

/// A declared field: what it may hold, and what it currently holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub format: Format,
    pub value: Value,
}

pub struct Interpreter {
    program: Program,
    fields: BTreeMap<String, Field>,
    pc: usize,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        let mut fields = BTreeMap::new();
        for declaration in &program.declarations {
            fields.insert(
                declaration.name.clone(),
                Field {
                    value: declaration.format.default_value(),
                    format: declaration.format.clone(),
                },
            );
        }
        Self {
            program,
            fields,
            pc: 0,
        }
    }

    pub fn fields(&self) -> &BTreeMap<String, Field> {
        &self.fields
    }

    /// Advances until the next observable effect. Returns [`Step::Done`] once the program
    /// is exhausted.
    ///
    /// Statements that produce no output advance the loop rather than calling `step`
    /// again, because statement execution must never recurse on the Rust call stack. A
    /// recursive evaluator could not be paused, and pausing is what INPUT will require.
    pub fn step(&mut self) -> Result<Step, NaturalError> {
        loop {
            let Some(statement) = self.program.statements.get(self.pc).cloned() else {
                return Ok(Step::Done);
            };
            self.pc += 1;

            match statement {
                // WRITE separates consecutive elements with exactly one blank. Literals are
                // verbatim; fields are padded to their full print width.
                Statement::Write { items } => {
                    let mut parts = Vec::with_capacity(items.len());
                    for item in &items {
                        parts.push(match item {
                            WriteItem::Literal(text) => text.clone(),
                            WriteItem::Field { name, line } => {
                                let field = self.fields.get(name).ok_or_else(|| {
                                    NaturalError::UndeclaredVariable {
                                        name: name.clone(),
                                        line: *line,
                                    }
                                })?;
                                render_field(&field.value, &field.format)
                            }
                        });
                    }
                    return Ok(Step::Output(parts.join(" ")));
                }

                Statement::Move {
                    source,
                    target,
                    line,
                } => {
                    let value = self.resolve(&source)?;
                    let format = self.format_of(&target, line)?;
                    let coerced = coerce(value, &format, &target, line)?;
                    self.assign(&target, coerced, line)?;
                }

                Statement::Reset { targets } => {
                    for (name, line) in targets {
                        let format = self.format_of(&name, line)?;
                        self.assign(&name, format.default_value(), line)?;
                    }
                }
            }
        }
    }

    fn resolve(&self, operand: &Operand) -> Result<Value, NaturalError> {
        match operand {
            Operand::Literal(value) => Ok(value.clone()),
            Operand::Variable { name, line } => self
                .fields
                .get(name)
                .map(|f| f.value.clone())
                .ok_or_else(|| NaturalError::UndeclaredVariable {
                    name: name.clone(),
                    line: *line,
                }),
        }
    }

    fn format_of(&self, name: &str, line: usize) -> Result<Format, NaturalError> {
        self.fields
            .get(name)
            .map(|f| f.format.clone())
            .ok_or_else(|| NaturalError::UndeclaredVariable {
                name: name.to_string(),
                line,
            })
    }

    fn assign(&mut self, name: &str, value: Value, line: usize) -> Result<(), NaturalError> {
        match self.fields.get_mut(name) {
            Some(field) => {
                field.value = value;
                Ok(())
            }
            None => Err(NaturalError::UndeclaredVariable {
                name: name.to_string(),
                line,
            }),
        }
    }
}
