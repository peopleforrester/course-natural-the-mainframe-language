// ABOUTME: Executes a parsed program as a resumable state machine driven by an explicit
// ABOUTME: program counter, never Rust recursion, so it can pause for INPUT and resume.

use crate::data::Database;
use crate::error::NaturalError;
use crate::parser::{ArithOp, Condition, Expr, Operand, Program, Statement, WriteItem};
use crate::value::{Format, Value, coerce, print_width, render_field};
use rust_decimal::{Decimal, RoundingStrategy};
use std::collections::{BTreeMap, VecDeque};
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
    /// Lines a single statement produced, drained one per `step` call. DISPLAY emits a
    /// header, an underline, a blank line, and a data row on its first execution.
    pending_output: VecDeque<String>,
    /// DISPLAY generates its headers once per report, not once per row.
    header_emitted: bool,
    /// The sample database, rebuilt per interpreter so every lesson run starts clean.
    database: Database,
    /// Which DDM fields each declared view exposes.
    views: BTreeMap<String, ViewBinding>,
    /// One cursor per READ loop, keyed by that loop's ReadInit index so nested and
    /// repeated reads never share position.
    cursors: BTreeMap<usize, ReadCursor>,
    /// A view-binding problem found while constructing, surfaced on the first `step` so
    /// that construction can stay infallible.
    init_error: Option<NaturalError>,
    steps: usize,
    step_limit: usize,
}

/// A declared view resolved against the database.
#[derive(Debug, Clone)]
struct ViewBinding {
    ddm: String,
    fields: Vec<String>,
}

/// A READ loop's resolved record set and position within it.
#[derive(Debug, Clone)]
struct ReadCursor {
    records: Vec<usize>,
    next: usize,
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
            database: Database::sample(),
            views: BTreeMap::new(),
            cursors: BTreeMap::new(),
            pc: 0,
            pending_input: None,
            pending_output: VecDeque::new(),
            header_emitted: false,
            init_error: None,
            steps: 0,
            step_limit: DEFAULT_STEP_LIMIT,
        }
        .bind_views()
    }

    /// Resolves every VIEW OF declaration against the database and seeds its fields.
    ///
    /// A view's fields take their format from the DDM rather than from the program, so this
    /// is where a misspelled file or field is caught. The error is held until the first
    /// `step` so that construction stays infallible for callers.
    fn bind_views(mut self) -> Self {
        let views = self.program.views.clone();
        for view in views {
            let Some(ddm) = self.database.ddm(&view.ddm) else {
                self.init_error = Some(NaturalError::UnknownDdm {
                    name: view.ddm.clone(),
                    line: view.line,
                });
                return self;
            };
            let mut names = Vec::with_capacity(view.fields.len());
            for (field_name, line) in &view.fields {
                let Some(definition) = ddm.field(field_name) else {
                    self.init_error = Some(NaturalError::UnknownDdmField {
                        name: field_name.clone(),
                        ddm: ddm.name.clone(),
                        line: *line,
                    });
                    return self;
                };
                // The view buffer exists before any record is read, holding format defaults.
                self.fields.insert(
                    field_name.clone(),
                    Field {
                        value: definition.format.default_value(),
                        format: definition.format.clone(),
                    },
                );
                names.push(field_name.clone());
            }
            self.views.insert(
                view.name.clone(),
                ViewBinding {
                    ddm: view.ddm.clone(),
                    fields: names,
                },
            );
        }
        self
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
        if let Some(error) = self.init_error.take() {
            return Err(error);
        }
        loop {
            // Lines already produced are handed over one at a time before anything else
            // happens, so a multi-line statement cannot interleave with a suspension.
            if let Some(line) = self.pending_output.pop_front() {
                return Ok(Step::Output(line));
            }

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
                    self.pending_output
                        .push_back(parts.join(" ").trim_end().to_string());
                }

                Statement::Display { fields, line } => {
                    let columns = self.display_columns(&fields, line)?;
                    if !self.header_emitted {
                        self.header_emitted = true;
                        self.pending_output.push_back(join_columns(
                            columns.iter().map(|c| center(&c.header, c.width)),
                        ));
                        self.pending_output
                            .push_back(join_columns(columns.iter().map(|c| "-".repeat(c.width))));
                        // Natural always generates exactly one blank line between the
                        // underlining and the data.
                        self.pending_output.push_back(String::new());
                    }
                    let row = join_columns(columns.iter().map(|c| {
                        // A field narrower than its header sits at the left of the column;
                        // the value's own justification happens inside the field width.
                        let mut cell = c.rendered.clone();
                        while cell.chars().count() < c.width {
                            cell.push(' ');
                        }
                        cell
                    }));
                    self.pending_output.push_back(row);
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

                Statement::Compute {
                    target,
                    expr,
                    rounded,
                    line,
                } => {
                    let result = self.evaluate_expr(&expr, line)?;
                    let format = self.format_of(&target, line)?;
                    let scaled = apply_scale(result, &format, rounded);
                    let coerced = coerce(Value::Number(scaled), &format, &target, line)?;
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

                // A READ is a loop exactly as FOR and REPEAT are, and it uses the same jump
                // machinery. The only addition is a cursor over the resolved record set.
                Statement::ReadInit {
                    view,
                    by,
                    limit,
                    key,
                    exit,
                    line,
                } => {
                    let binding = self.view_binding(&view, line)?;
                    let ddm_name = binding.ddm.clone();
                    if let Some(descriptor) = &by {
                        let ddm = self
                            .database
                            .ddm(&ddm_name)
                            .expect("the binding resolved this DDM");
                        if ddm.field(descriptor).is_none() {
                            return Err(NaturalError::UnknownDdmField {
                                name: descriptor.clone(),
                                ddm: ddm_name,
                                line,
                            });
                        }
                    }
                    let mut records = self.database.order(by.as_deref());
                    if let Some(limit) = limit {
                        records.truncate(limit);
                    }
                    let cursor = ReadCursor { records, next: 0 };
                    self.cursors.insert(key, cursor);
                    if !self.advance_cursor(key, &view, line)? {
                        self.pc = exit;
                    }
                }

                Statement::ReadNext {
                    view,
                    key,
                    top,
                    line,
                } => {
                    if self.advance_cursor(key, &view, line)? {
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

    fn view_binding(&self, name: &str, line: usize) -> Result<ViewBinding, NaturalError> {
        self.views
            .get(name)
            .cloned()
            .ok_or_else(|| NaturalError::UnknownView {
                name: name.to_string(),
                line,
            })
    }

    /// Moves a READ cursor to its next record and copies that record into the view buffer.
    ///
    /// Copying into the same field map every other statement already uses is what lets
    /// WRITE, DISPLAY, IF, and COMPUTE work on database fields with no special handling.
    fn advance_cursor(
        &mut self,
        key: usize,
        view: &str,
        line: usize,
    ) -> Result<bool, NaturalError> {
        let binding = self.view_binding(view, line)?;
        let Some(cursor) = self.cursors.get_mut(&key) else {
            return Ok(false);
        };
        let Some(record) = cursor.records.get(cursor.next).copied() else {
            return Ok(false);
        };
        cursor.next += 1;

        for field_name in &binding.fields {
            if let Some(value) = self.database.value(record, field_name)
                && let Some(field) = self.fields.get_mut(field_name)
            {
                field.value = value;
            }
        }
        Ok(true)
    }

    /// Builds the column layout for one DISPLAY statement.
    ///
    /// Column width is the greater of the field's print width and its header width, and the
    /// header for a user-defined variable is the variable name including its leading `#`.
    /// Both rules are quoted from the DISPLAY statement reference in
    /// `research/07-output-formatting-semantics.md`.
    ///
    /// Natural fixes the headers from the FIRST DISPLAY statement at compile time. This
    /// interpreter fixes them at the first DISPLAY executed, which is equivalent for the
    /// single-report programs the teaching subset covers.
    fn display_columns(
        &self,
        fields: &[(String, usize)],
        line: usize,
    ) -> Result<Vec<DisplayColumn>, NaturalError> {
        let mut columns = Vec::with_capacity(fields.len());
        for (name, field_line) in fields {
            let field = self
                .fields
                .get(name)
                .ok_or_else(|| NaturalError::UndeclaredVariable {
                    name: name.clone(),
                    line: *field_line,
                })?;
            let header = name.clone();
            let width = print_width(&field.format).max(header.chars().count());
            columns.push(DisplayColumn {
                rendered: render_field(&field.value, &field.format),
                header,
                width,
            });
        }
        let _ = line;
        Ok(columns)
    }

    /// Evaluates an arithmetic expression.
    ///
    /// This recurses over the expression tree, which is allowed: the no-recursion rule
    /// governs statement execution, and a suspension can never occur mid-expression.
    fn evaluate_expr(&self, expr: &Expr, line: usize) -> Result<Decimal, NaturalError> {
        match expr {
            Expr::Value(operand) => match self.resolve(operand)? {
                Value::Number(n) => Ok(n),
                other => Err(NaturalError::NonNumericArithmetic {
                    name: match operand {
                        Operand::Variable { name, .. } => name.clone(),
                        Operand::Literal(_) => "a literal".to_string(),
                    },
                    kind: other.describe_kind().to_string(),
                    line,
                }),
            },
            Expr::Binary { left, op, right } => {
                let a = self.evaluate_expr(left, line)?;
                let b = self.evaluate_expr(right, line)?;
                match op {
                    ArithOp::Add => Ok(a + b),
                    ArithOp::Sub => Ok(a - b),
                    ArithOp::Mul => Ok(a * b),
                    ArithOp::Div => {
                        if b.is_zero() {
                            return Err(NaturalError::DivisionByZero { line });
                        }
                        Ok(a / b)
                    }
                }
            }
        }
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

/// One DISPLAY column: its generated header, its width, and this row's rendered value.
struct DisplayColumn {
    header: String,
    width: usize,
    rendered: String,
}

/// Joins columns with the single blank that separates them, trimming the line end.
fn join_columns(cells: impl Iterator<Item = String>) -> String {
    cells.collect::<Vec<_>>().join(" ").trim_end().to_string()
}

/// Centers a header over its column.
///
/// The documentation states the header is centered when the field is wider, and both
/// measured examples have symmetric padding, so they do not settle where the odd blank goes
/// when the padding is odd. This puts it on the right, and lesson fixtures should avoid
/// depending on that case until it is verified.
fn center(text: &str, width: usize) -> String {
    let len = text.chars().count();
    if len >= width {
        return text.to_string();
    }
    let pad = width - len;
    let left = pad / 2;
    format!("{}{}{}", " ".repeat(left), text, " ".repeat(pad - left))
}

/// Applies a target field's decimal scale to a computed result.
///
/// Truncation toward zero is the documented default; ROUNDED rounds away from zero when
/// the first discarded digit is 5 or more. Verified in
/// `research/07-output-formatting-semantics.md`, rows E1 through E4.
fn apply_scale(value: Decimal, format: &Format, rounded: bool) -> Decimal {
    let decimals = match format {
        Format::Numeric { decimals, .. } | Format::Packed { decimals, .. } => *decimals,
        Format::Integer { .. } => 0,
        _ => return value,
    };
    let strategy = if rounded {
        RoundingStrategy::MidpointAwayFromZero
    } else {
        RoundingStrategy::ToZero
    };
    value.round_dp_with_strategy(decimals, strategy)
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
