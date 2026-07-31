// ABOUTME: Executes a parsed program as a resumable state machine driven by an explicit
// ABOUTME: program counter, never Rust recursion, so it can pause for INPUT and resume.

use crate::error::NaturalError;
use crate::parser::{Condition, Operand, Program, Statement, WriteItem};
use crate::value::{Format, Value, coerce, render_field};
use rust_decimal::Decimal;
use std::collections::BTreeMap;
use std::str::FromStr;

/// What the interpreter is waiting for while suspended.
///
/// This is deliberately a struct rather than a bare prompt string. A Natural map read is a
/// yield point exactly like a line-mode INPUT, so what gets suspended is ultimately a
/// screen. Growing this struct (fields, attributes, cursor position) must not require
/// changing the [`Step`] enum or the resume protocol. See
/// `research/08-mainframe-emulators-3270.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputRequest {
    /// Text shown before reading. In Tier 2 this becomes a rendered screen.
    pub prompt: String,
    /// The field the value will be assigned to.
    pub field: String,
}

/// One observable effect of advancing the interpreter.
///
/// `step` returns to the caller between statements and keeps all execution state in the
/// [`Interpreter`] struct rather than on the Rust call stack, so the browser can hand
/// control back to JavaScript and resume later. See docs/gotchas-rust-wasm.md.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Output(String),
    /// The program is suspended. Supply a value with [`Interpreter::provide_input`].
    NeedsInput(InputRequest),
    Done,
}

/// An INPUT statement partway through being satisfied.
///
/// One INPUT may read several fields, so the interpreter suspends once per field and
/// remembers its position here rather than on the call stack.
#[derive(Debug, Clone)]
struct PendingInput {
    prompt: Option<String>,
    targets: Vec<(String, usize)>,
    next: usize,
}

/// A declared field: what it may hold, and what it currently holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub format: Format,
    pub value: Value,
}

/// How many statements a program may execute before it is assumed to be stuck.
///
/// This is a product requirement, not a tuning knob: lessons run in the learner's own
/// browser tab, so an unbounded REPEAT must fail with a teaching error rather than freeze
/// the page. Tier 1 programs execute a few hundred statements at most, so the default is
/// several orders of magnitude clear of legitimate work.
pub const DEFAULT_STEP_LIMIT: usize = 1_000_000;

pub struct Interpreter {
    program: Program,
    fields: BTreeMap<String, Field>,
    pc: usize,
    pending_input: Option<PendingInput>,
    steps: usize,
    step_limit: usize,
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
            pending_input: None,
            steps: 0,
            step_limit: DEFAULT_STEP_LIMIT,
        }
    }

    /// Overrides the runaway-loop cap. Lower it to keep tests fast; raise it only for a
    /// lesson that genuinely needs more work than the default allows.
    pub fn with_step_limit(mut self, limit: usize) -> Self {
        self.step_limit = limit;
        self
    }

    pub fn step_limit(&self) -> usize {
        self.step_limit
    }

    pub fn fields(&self) -> &BTreeMap<String, Field> {
        &self.fields
    }

    /// Advances until the next observable effect. Returns [`Step::Done`] once the program
    /// is exhausted.
    ///
    /// Statements that produce no output advance the loop rather than calling `step`
    /// again, because statement execution must never recurse on the Rust call stack. A
    /// recursive evaluator could not be paused, and INPUT requires pausing.
    pub fn step(&mut self) -> Result<Step, NaturalError> {
        loop {
            // A partially satisfied INPUT is resumed before the program counter moves, so
            // calling step again without supplying a value asks again rather than
            // silently skipping the field.
            if let Some(pending) = &self.pending_input {
                if let Some(request) = pending.request() {
                    return Ok(Step::NeedsInput(request));
                }
                self.pending_input = None;
            }

            let Some(statement) = self.program.statements.get(self.pc).cloned() else {
                return Ok(Step::Done);
            };
            self.pc += 1;

            // Counting executed statements catches a runaway loop of any shape, including
            // one built from jumps the learner wrote by hand, which counting iterations of
            // a particular construct would miss.
            self.steps += 1;
            if self.steps > self.step_limit {
                return Err(NaturalError::RunawayLoop {
                    limit: self.step_limit,
                });
            }

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
                    // Padding BETWEEN elements is documented and verified, so it is built
                    // above and never touched. Trailing blanks at end of line are a
                    // different question: verification could not establish either way
                    // whether Natural emits them, and most documentation examples show
                    // them absent. They are invisible in a terminal, so this interpreter
                    // trims them as a deliberate course convention. See
                    // research/verification/v07-output-formatting.md.
                    return Ok(Step::Output(parts.join(" ").trim_end().to_string()));
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

                // Blocks are compiled to jumps, so control flow is just an assignment to
                // the program counter. Nothing recurses, so a suspension can happen
                // anywhere, including inside a branch.
                Statement::IfFalseJump {
                    condition,
                    target,
                    line,
                } => {
                    if !self.evaluate(&condition, line)? {
                        self.pc = target;
                    }
                }

                Statement::IfTrueJump {
                    condition,
                    target,
                    line,
                } => {
                    if self.evaluate(&condition, line)? {
                        self.pc = target;
                    }
                }

                Statement::Jump { target } => self.pc = target,

                Statement::ForInit {
                    var,
                    from,
                    to,
                    exit,
                    line,
                } => {
                    let start = self.resolve(&from)?;
                    let format = self.format_of(&var, line)?;
                    let coerced = coerce(start, &format, &var, line)?;
                    self.assign(&var, coerced, line)?;
                    if !self.control_still_in_range(&var, &to, line)? {
                        self.pc = exit;
                    }
                }

                Statement::ForNext { var, to, top, line } => {
                    let current = self.numeric_field(&var, line)?;
                    let format = self.format_of(&var, line)?;
                    let next = coerce(Value::Number(current + Decimal::ONE), &format, &var, line)?;
                    self.assign(&var, next, line)?;
                    if self.control_still_in_range(&var, &to, line)? {
                        self.pc = top;
                    }
                }

                Statement::Input { prompt, targets } => {
                    // Fail fast on an undeclared field rather than suspending and only
                    // discovering the problem after the learner has typed something.
                    for (name, line) in &targets {
                        self.format_of(name, *line)?;
                    }
                    self.pending_input = Some(PendingInput {
                        prompt,
                        targets,
                        next: 0,
                    });
                }
            }
        }
    }

    /// Supplies the value the program is waiting for and clears the suspension.
    ///
    /// The text is converted according to the target field's declared format, so the same
    /// length, precision, and overflow rules that govern MOVE also govern INPUT.
    pub fn provide_input(&mut self, text: &str) -> Result<(), NaturalError> {
        let Some(pending) = &self.pending_input else {
            return Err(NaturalError::NotWaitingForInput);
        };
        let Some((name, line)) = pending.targets.get(pending.next).cloned() else {
            return Err(NaturalError::NotWaitingForInput);
        };

        let format = self.format_of(&name, line)?;
        let value = parse_input_value(text, &format, &name, line)?;
        let coerced = coerce(value, &format, &name, line)?;
        self.assign(&name, coerced, line)?;

        if let Some(pending) = &mut self.pending_input {
            pending.next += 1;
        }
        Ok(())
    }

    /// Evaluates a comparison. Values must be of the same kind, because silently coercing
    /// text and numbers would let a learner write a comparison that quietly never matches.
    fn evaluate(&self, condition: &Condition, line: usize) -> Result<bool, NaturalError> {
        let left = self.resolve(&condition.left)?;
        let right = self.resolve(&condition.right)?;

        let ordering = match (&left, &right) {
            (Value::Number(a), Value::Number(b)) => a.cmp(b),
            // Natural pads the shorter operand with blanks, so trailing blanks never
            // affect the result. Comparing the stored text gives the same answer.
            (Value::Alpha(a), Value::Alpha(b)) => a.trim_end().cmp(b.trim_end()),
            (Value::Logical(a), Value::Logical(b)) => a.cmp(b),
            _ => {
                return Err(NaturalError::IncomparableValues {
                    left: left.describe_kind().to_string(),
                    right: right.describe_kind().to_string(),
                    line,
                });
            }
        };
        Ok(condition.op.holds(ordering))
    }

    /// True while a FOR control field has not yet passed its upper bound.
    fn control_still_in_range(
        &self,
        var: &str,
        to: &Operand,
        line: usize,
    ) -> Result<bool, NaturalError> {
        let current = self.numeric_field(var, line)?;
        let Value::Number(limit) = self.resolve(to)? else {
            return Err(NaturalError::IncomparableValues {
                left: "a number".to_string(),
                right: "text".to_string(),
                line,
            });
        };
        Ok(current <= limit)
    }

    /// Reads a field that a loop requires to be numeric.
    fn numeric_field(&self, name: &str, line: usize) -> Result<Decimal, NaturalError> {
        match self.fields.get(name).map(|f| &f.value) {
            Some(Value::Number(n)) => Ok(*n),
            Some(other) => Err(NaturalError::IncomparableValues {
                left: other.describe_kind().to_string(),
                right: "a number".to_string(),
                line,
            }),
            None => Err(NaturalError::UndeclaredVariable {
                name: name.to_string(),
                line,
            }),
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

impl PendingInput {
    /// The request for the field currently being read, or None once all are satisfied.
    fn request(&self) -> Option<InputRequest> {
        let (name, _) = self.targets.get(self.next)?;
        Some(InputRequest {
            // A literal prompt applies to the statement; without one the field name is
            // shown. Line-mode prompt rendering is a course convention, because real
            // Natural presents a map rather than prompting field by field.
            prompt: self.prompt.clone().unwrap_or_else(|| name.clone()),
            field: name.clone(),
        })
    }
}

/// Converts a line of learner input into a value of the target field's format.
fn parse_input_value(
    text: &str,
    format: &Format,
    name: &str,
    line: usize,
) -> Result<Value, NaturalError> {
    let trimmed = text.trim();
    let invalid = |expected: &str| NaturalError::InvalidInput {
        text: text.to_string(),
        name: name.to_string(),
        expected: expected.to_string(),
        line,
    };

    match format {
        Format::Alpha { .. } => Ok(Value::Alpha(text.to_string())),
        Format::Logical => match trimmed.to_ascii_uppercase().as_str() {
            "TRUE" | "T" | "Y" | "YES" => Ok(Value::Logical(true)),
            "FALSE" | "F" | "N" | "NO" => Ok(Value::Logical(false)),
            _ => Err(invalid("true or false")),
        },
        Format::Numeric { .. } | Format::Packed { .. } | Format::Integer { .. } => {
            Decimal::from_str(trimmed)
                .map(Value::Number)
                .map_err(|_| invalid("a number"))
        }
    }
}
