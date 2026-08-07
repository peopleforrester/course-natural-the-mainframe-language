// ABOUTME: Executes a parsed program as a resumable state machine driven by an explicit
// ABOUTME: program counter, never Rust recursion, so it can pause for INPUT and resume.

use crate::data::Database;
use crate::error::NaturalError;
use crate::parser::{ArithOp, CompareOp, Condition, Expr, Operand, Program, Statement, WriteItem};
use crate::screen::{Attribute, Screen, ScreenField};
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
    /// The program is showing a map. Supply the filled-in fields and the key the operator
    /// pressed with [`Interpreter::provide_screen`].
    ///
    /// A map read is a suspension of exactly the same kind as a line-mode INPUT; what
    /// differs is that the thing suspended is a screen rather than a prompt.
    NeedsScreen(Screen),
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

/// How deep PERFORM and CALLNAT may nest before the program is assumed to be recursing
/// without end. Real Natural programs nest a handful of levels; this is far above that
/// while still catching a routine that performs itself.
pub const MAX_CALL_DEPTH: usize = 256;

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
    /// The last committed state. Changes live in `database` until END TRANSACTION copies
    /// them here, which is what makes forgetting the commit visibly lose work.
    committed: Database,
    /// The record the innermost active loop is holding, which UPDATE and DELETE act on.
    current_record: Option<usize>,
    /// Distinct descriptor values a HISTOGRAM is walking, keyed by loop.
    histograms: BTreeMap<usize, HistogramCursor>,
    /// Return addresses for PERFORM. This lives here rather than on the Rust call stack
    /// precisely so a suspension can happen several frames deep and still resume.
    call_stack: Vec<usize>,
    /// A map presented and not yet filled in.
    pending_screen: Option<PendingScreen>,
    /// The statement index of the INPUT or map read currently being validated, so REINPUT
    /// knows where to send the operator back to.
    last_input_at: Option<usize>,
    /// Callers waiting for the object currently executing.
    frames: Vec<SavedFrame>,
    /// The subprogram objects CALLNAT can reach.
    library: Library,
    /// AID keys made active with SET KEY. A key that is not sensitized delivers ENTR, which
    /// is why a PF3 branch does nothing until the program asks for PF3.
    sensitized_keys: std::collections::BTreeSet<String>,
    all_keys_sensitized: bool,
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

/// The subprogram objects a program may call.
///
/// Natural programs live in a library alongside the subprograms they use. A lesson
/// supplies that library, so a learner can call a routine without also having to write it.
#[derive(Debug, Clone, Default)]
pub struct Library {
    objects: BTreeMap<String, String>,
    maps: BTreeMap<String, String>,
}

impl Library {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: &str, source: &str) {
        self.objects
            .insert(name.to_ascii_uppercase(), source.to_string());
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.objects
            .get(&name.to_ascii_uppercase())
            .map(String::as_str)
    }

    /// Every subprogram name in the library, so a caller can check them all.
    pub fn names(&self) -> Vec<String> {
        self.objects.keys().cloned().collect()
    }

    /// Adds a map object. Its source is a layout rather than statements, which is why it
    /// lives beside the subprograms rather than among them.
    pub fn add_map(&mut self, name: &str, layout: &str) {
        self.maps
            .insert(name.to_ascii_uppercase(), layout.to_string());
    }

    pub fn map(&self, name: &str) -> Option<&str> {
        self.maps
            .get(&name.to_ascii_uppercase())
            .map(String::as_str)
    }

    /// Every map name in the library.
    pub fn map_names(&self) -> Vec<String> {
        self.maps.keys().cloned().collect()
    }
}

/// A caller's execution state, kept while a subprogram runs.
///
/// Subprogram calls push onto this stack rather than recursing into another `step`, so a
/// suspension inside a called object resumes correctly. It is the same reason the
/// interpreter never recurses for statements.
struct SavedFrame {
    program: Program,
    pc: usize,
    fields: BTreeMap<String, Field>,
    views: BTreeMap<String, ViewBinding>,
    cursors: BTreeMap<usize, ReadCursor>,
    histograms: BTreeMap<usize, HistogramCursor>,
    call_stack: Vec<usize>,
    current_record: Option<usize>,
    /// Caller variable paired with the callee parameter whose value returns to it.
    /// Literal arguments have no destination and are absent.
    returns: Vec<(String, String)>,
}

/// A map presented to the operator and awaiting a response.
#[derive(Debug, Clone)]
struct PendingScreen {
    screen: Screen,
    /// Where to resume once the screen comes back.
    resume_at: usize,
}

/// A HISTOGRAM's distinct descriptor values, each with how many records carry it.
#[derive(Debug, Clone)]
struct HistogramCursor {
    values: Vec<(Value, usize)>,
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
        // The AID key the operator last pressed. Every "PF3 to exit" convention in
        // mainframe software reads this.
        fields.insert(
            "*PF-KEY".to_string(),
            Field {
                format: Format::Alpha { length: 4 },
                value: Value::Alpha("ENTR".to_string()),
            },
        );
        let counter = Field {
            format: Format::Numeric {
                int_digits: 9,
                decimals: 0,
            },
            value: Value::Number(Decimal::ZERO),
        };
        for name in ["*NUMBER", "*COUNTER"] {
            // System variables are ordinary numeric fields as far as everything else is
            // concerned, so WRITE, DISPLAY, and IF need no special case for them.
            fields.insert(name.to_string(), counter.clone());
            // A labelled loop gets its own copy per label, which is what keeps the value
            // readable after the loop has ended and the bare form has moved on.
            for label in program.labels.keys() {
                fields.insert(format!("{name}({label}.)"), counter.clone());
            }
        }
        Self {
            program,
            fields,
            database: Database::sample(),
            committed: Database::sample(),
            current_record: None,
            histograms: BTreeMap::new(),
            call_stack: Vec::new(),
            pending_screen: None,
            last_input_at: None,
            frames: Vec::new(),
            library: Library::new(),
            sensitized_keys: std::collections::BTreeSet::new(),
            all_keys_sensitized: false,
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

    /// Supplies the subprogram objects this program may CALLNAT.
    pub fn with_library(mut self, library: Library) -> Self {
        self.library = library;
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

    /// The database as of the last committed transaction.
    pub fn committed(&self) -> &Database {
        &self.committed
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

            // A map that has not come back yet is re-presented rather than skipped, for
            // the same reason a partially satisfied INPUT is.
            if let Some(pending) = &self.pending_screen {
                return Ok(Step::NeedsScreen(pending.screen.clone()));
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
                // The end of a subprogram is a return, not the end of the run.
                if self.frames.is_empty() {
                    return Ok(Step::Done);
                }
                self.return_from_subprogram();
                continue;
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
                                let field = self
                                    .fields
                                    .get(name)
                                    .ok_or_else(|| missing_field(name, *line))?;
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

                Statement::FindInit {
                    view,
                    with,
                    filter,
                    sorted_by,
                    limit,
                    key,
                    line,
                } => {
                    let binding = self.view_binding(&view, line)?;
                    let ddm_name = binding.ddm.clone();
                    if let Some(descriptor) = &sorted_by {
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

                    // WITH is the search the database performs, so it is what *NUMBER
                    // reports. WHERE is applied afterwards, record by record.
                    let mut matched = Vec::new();
                    for record in self.database.order(sorted_by.as_deref()) {
                        if self.record_matches(record, &with, line)? {
                            matched.push(record);
                        }
                    }
                    self.set_loop_variable("*NUMBER", key, matched.len());

                    if let Some(condition) = &filter {
                        let mut kept = Vec::with_capacity(matched.len());
                        for record in matched {
                            if self.record_matches(record, condition, line)? {
                                kept.push(record);
                            }
                        }
                        matched = kept;
                    }
                    if let Some(limit) = limit {
                        matched.truncate(limit);
                    }

                    self.cursors.insert(
                        key,
                        ReadCursor {
                            records: matched,
                            next: 0,
                        },
                    );
                    self.advance_cursor(key, &view, line)?;
                }

                Statement::JumpIfNoRecords { key, target } => {
                    // The cursor has already consumed its first record if one existed, so
                    // an empty search is one whose set was empty to begin with.
                    let empty = self
                        .cursors
                        .get(&key)
                        .map(|c| c.records.is_empty())
                        .unwrap_or(true);
                    if empty {
                        self.pc = target;
                    }
                }

                Statement::Ignore => {}

                Statement::Skip { lines } => {
                    for _ in 0..lines {
                        self.pending_output.push_back(String::new());
                    }
                }

                Statement::SetKey { keys, line: _ } => {
                    if keys.is_empty() {
                        self.all_keys_sensitized = true;
                    } else {
                        self.sensitized_keys.extend(keys);
                    }
                }

                Statement::ResetViewFields { view, line } => {
                    let binding = self.view_binding(&view, line)?;
                    let names: Vec<String> = binding.fields.clone();
                    for name in names {
                        if let Some(field) = self.fields.get_mut(&name) {
                            field.value = field.format.default_value();
                        }
                    }
                    // No record is current during the empty pass, so UPDATE and DELETE fail
                    // with the same message they would give outside a loop.
                    self.current_record = None;
                }

                Statement::Perform { name, line } => {
                    let Some(target) = self.program.subroutines.get(&name).copied() else {
                        return Err(NaturalError::UnknownSubroutine { name, line });
                    };
                    if self.call_stack.len() >= MAX_CALL_DEPTH {
                        return Err(NaturalError::CallStackTooDeep {
                            limit: MAX_CALL_DEPTH,
                        });
                    }
                    // self.pc already points past the PERFORM, so it is the return address.
                    self.call_stack.push(self.pc);
                    self.pc = target;
                }

                Statement::ReturnFromSubroutine => {
                    // Falling off the end of a definition that was never performed simply
                    // continues, which cannot happen in practice because the definition is
                    // jumped over, but costs nothing to handle.
                    if let Some(back) = self.call_stack.pop() {
                        self.pc = back;
                    }
                }

                Statement::Callnat { name, args, line } => {
                    self.enter_subprogram(&name, &args, line)?;
                }

                Statement::Store { view, line } => {
                    let binding = self.view_binding(&view, line)?;
                    let buffer = self.view_buffer(&binding);
                    self.database.store(&buffer);
                }

                Statement::UpdateRecord { line } => {
                    let Some(record) = self.current_record else {
                        return Err(NaturalError::NoCurrentRecord {
                            statement: "UPDATE".to_string(),
                            line,
                        });
                    };
                    // Every view the program declared writes back, which for the single
                    // view a Tier 1 program declares is simply that view's fields.
                    let buffer: Vec<(String, Value)> = self
                        .views
                        .values()
                        .flat_map(|binding| self.view_buffer(binding))
                        .collect();
                    self.database.update(record, &buffer);
                }

                Statement::DeleteRecord { line } => {
                    let Some(record) = self.current_record else {
                        return Err(NaturalError::NoCurrentRecord {
                            statement: "DELETE".to_string(),
                            line,
                        });
                    };
                    self.database.delete(record);
                }

                // The transaction boundary. Until this runs, changes exist only in the
                // working copy, so a program that forgets it loses its work. That is a
                // deliberate teaching surface, not an oversight.
                Statement::EndTransaction => self.committed = self.database.clone(),

                Statement::BackoutTransaction => self.database = self.committed.clone(),

                Statement::HistogramInit {
                    view,
                    descriptor,
                    limit,
                    key,
                    exit,
                    line,
                } => {
                    let binding = self.view_binding(&view, line)?;
                    let ddm_name = binding.ddm.clone();
                    let ddm = self
                        .database
                        .ddm(&ddm_name)
                        .expect("the binding resolved this DDM");
                    if ddm.field(&descriptor).is_none() {
                        return Err(NaturalError::UnknownDdmField {
                            name: descriptor.clone(),
                            ddm: ddm_name,
                            line,
                        });
                    }

                    // Distinct values in ascending order, each with how many records carry
                    // it. A histogram reads the index rather than the records themselves.
                    let mut values: Vec<(Value, usize)> = Vec::new();
                    for record in self.database.order(Some(&descriptor)) {
                        let Some(value) = self.database.value(record, &descriptor) else {
                            continue;
                        };
                        match values.last_mut() {
                            Some((seen, count)) if *seen == value => *count += 1,
                            _ => values.push((value, 1)),
                        }
                    }
                    if let Some(limit) = limit {
                        values.truncate(limit);
                    }
                    self.histograms
                        .insert(key, HistogramCursor { values, next: 0 });
                    if !self.advance_histogram(key, &descriptor) {
                        self.pc = exit;
                    }
                }

                Statement::HistogramNext {
                    key,
                    descriptor,
                    top,
                    ..
                } => {
                    if self.advance_histogram(key, &descriptor) {
                        self.pc = top;
                    }
                }

                Statement::InputUsingMap { map, line } => {
                    let screen = self.build_screen(&map, line)?;
                    // Resuming at the map read itself would re-present the screen; the
                    // resume point is the statement after it.
                    self.last_input_at = Some(self.pc - 1);
                    self.pending_screen = Some(PendingScreen {
                        screen,
                        resume_at: self.pc,
                    });
                }

                Statement::Reinput { message, line } => {
                    let Some(back) = self.last_input_at else {
                        return Err(NaturalError::ReinputWithoutInput { line });
                    };
                    if let Some(text) = message {
                        self.pending_output.push_back(text);
                    }
                    // Go back to the read itself, which presents the screen or prompt again.
                    self.pc = back;
                }

                Statement::Input { prompt, targets } => {
                    self.last_input_at = Some(self.pc - 1);
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

        compare(&left, &right, condition.op, line)
    }

    /// Builds the screen a map defines, filling entry fields with what they already hold.
    fn build_screen(&self, name: &str, line: usize) -> Result<Screen, NaturalError> {
        // A map is a separate object, so it comes from the library exactly as a subprogram
        // does. A program can name one but can never contain one.
        let Some(source) = self.library.map(name) else {
            return Err(NaturalError::UnknownMap {
                name: name.to_string(),
                line,
            });
        };
        let elements = crate::parser::parse_map(source)?;

        let mut screen = Screen::blank(name);
        for element in &elements {
            let mut column = element.column;

            // A label is protected text; the operator cannot type into it.
            if let Some(label) = &element.label {
                screen.fields.push(ScreenField {
                    row: element.row,
                    column,
                    width: label.chars().count(),
                    text: label.clone(),
                    attribute: Attribute::Protected,
                    bound_to: None,
                    modified: false,
                });
                // One blank separates a label from the field it introduces.
                column += label.chars().count() + 1;
            }

            let Some(bound) = &element.bound_to else {
                continue;
            };
            let Some(field) = self.fields.get(bound) else {
                return Err(missing_field(bound, element.line));
            };

            // Without an explicit AD= clause the attribute follows the field's format, so
            // a numeric field becomes a numeric-only entry field.
            let attribute = element.attribute.unwrap_or(match field.format {
                Format::Numeric { .. } | Format::Packed { .. } | Format::Integer { .. } => {
                    Attribute::Numeric
                }
                _ => Attribute::Unprotected,
            });

            screen.fields.push(ScreenField {
                row: element.row,
                column,
                width: print_width(&field.format),
                text: render_field(&field.value, &field.format)
                    .trim_end()
                    .to_string(),
                attribute,
                bound_to: Some(bound.clone()),
                modified: false,
            });
        }
        Ok(screen)
    }

    /// Supplies a completed screen and the AID key that ended it.
    ///
    /// Only fields the operator changed need be given, which mirrors Read Modified: a real
    /// 3270 returns the fields whose modified data tag is set, not the whole screen.
    pub fn provide_screen(
        &mut self,
        values: &[(String, String)],
        aid: &str,
    ) -> Result<(), NaturalError> {
        let Some(pending) = self.pending_screen.take() else {
            return Err(NaturalError::NotWaitingForInput);
        };

        for (name, text) in values {
            let name = crate::parser::normalize(name);
            let Some(format) = self.fields.get(&name).map(|f| f.format.clone()) else {
                continue;
            };
            let value = parse_input_value(text, &format, &name, 0)?;
            let coerced = coerce(value, &format, &name, 0)?;
            self.assign(&name, coerced, 0)?;
        }

        // A key the program never sensitized arrives as ENTR. Without this the PF3 branch
        // in a lesson would appear to work while doing nothing of the kind on a real system.
        let pressed = aid.to_ascii_uppercase();
        let delivered = if pressed == "ENTR"
            || self.all_keys_sensitized
            || self.sensitized_keys.contains(&pressed)
        {
            pressed
        } else {
            "ENTR".to_string()
        };
        if let Some(field) = self.fields.get_mut("*PF-KEY") {
            field.value = Value::Alpha(delivered);
        }
        self.pc = pending.resume_at;
        Ok(())
    }

    /// The screen currently being shown, if the program is waiting on one.
    pub fn current_screen(&self) -> Option<&Screen> {
        self.pending_screen.as_ref().map(|p| &p.screen)
    }

    /// Compiles a subprogram, binds its parameters, and makes it the executing object.
    fn enter_subprogram(
        &mut self,
        name: &str,
        args: &[Operand],
        line: usize,
    ) -> Result<(), NaturalError> {
        let Some(source) = self.library.get(name).map(str::to_string) else {
            return Err(NaturalError::UnknownSubprogram {
                name: name.to_string(),
                line,
            });
        };
        if self.frames.len() >= MAX_CALL_DEPTH {
            return Err(NaturalError::CallStackTooDeep {
                limit: MAX_CALL_DEPTH,
            });
        }

        let name_for_error = name.to_string();
        // A failure inside a called object says which object failed, because the line
        // number alone would point at source the learner may not have written.
        let named = |error: NaturalError| NaturalError::InSubprogram {
            name: name.to_string(),
            source_message: error.to_string(),
        };
        let program = crate::parser::parse(&source).map_err(named)?;

        if program.parameters.len() != args.len() {
            return Err(NaturalError::ParameterCountMismatch {
                name: name.to_string(),
                expected: program.parameters.len(),
                given: args.len(),
                line,
            });
        }

        // Values in, by position. A subprogram cannot see the caller's other fields, which
        // is the whole difference between it and an inline subroutine.
        //
        // Parameters pass by reference, so the callee writes through to the caller's own
        // storage. That makes format and length a hard match rather than a conversion: an
        // A2 argument against an A3 parameter is a different piece of memory, not a
        // shorter one. A literal is the exception, since it has no storage to share.
        let mut incoming = Vec::with_capacity(args.len());
        for (position, (parameter, argument)) in program.parameters.iter().zip(args).enumerate() {
            if let Operand::Variable { name, .. } = argument {
                let actual = self.format_of(name, line)?;
                if actual != parameter.format {
                    return Err(NaturalError::ParameterFormatMismatch {
                        subprogram: name_for_error.to_string(),
                        position: position + 1,
                        expected: parameter.format.describe(),
                        actual: actual.describe(),
                        line,
                    });
                }
            }
            let value = self.resolve(argument)?;
            let coerced = coerce(value, &parameter.format, &parameter.name, line)?;
            incoming.push((parameter.name.clone(), coerced));
        }

        // Results come back to whichever arguments were variables.
        let returns = program
            .parameters
            .iter()
            .zip(args)
            .filter_map(|(parameter, argument)| match argument {
                Operand::Variable { name, .. } => Some((name.clone(), parameter.name.clone())),
                Operand::Literal(_) => None,
            })
            .collect();

        let mut callee = Interpreter::new(program);
        for (field_name, value) in incoming {
            if let Some(field) = callee.fields.get_mut(&field_name) {
                field.value = value;
            }
        }
        if let Some(error) = callee.init_error.take() {
            return Err(named(error));
        }

        // Swap the callee in and keep the caller for when it returns. The database is
        // shared, because a subprogram reads and writes the same file the caller does.
        self.frames.push(SavedFrame {
            program: std::mem::replace(&mut self.program, callee.program),
            pc: std::mem::replace(&mut self.pc, 0),
            fields: std::mem::replace(&mut self.fields, callee.fields),
            views: std::mem::replace(&mut self.views, callee.views),
            cursors: std::mem::take(&mut self.cursors),
            histograms: std::mem::take(&mut self.histograms),
            call_stack: std::mem::take(&mut self.call_stack),
            current_record: self.current_record.take(),
            returns,
        });
        Ok(())
    }

    /// Restores the caller and hands back the parameter values.
    fn return_from_subprogram(&mut self) {
        let Some(frame) = self.frames.pop() else {
            return;
        };
        let results: Vec<(String, Value)> = frame
            .returns
            .iter()
            .filter_map(|(caller_var, parameter)| {
                self.fields
                    .get(parameter)
                    .map(|field| (caller_var.clone(), field.value.clone()))
            })
            .collect();

        self.program = frame.program;
        self.pc = frame.pc;
        self.fields = frame.fields;
        self.views = frame.views;
        self.cursors = frame.cursors;
        self.histograms = frame.histograms;
        self.call_stack = frame.call_stack;
        self.current_record = frame.current_record;

        for (caller_var, value) in results {
            // Coercing into the caller's own format keeps its declared length and scale
            // authoritative, exactly as an assignment would.
            let Some(format) = self.fields.get(&caller_var).map(|f| f.format.clone()) else {
                continue;
            };
            if let Ok(coerced) = coerce(value, &format, &caller_var, 0)
                && let Some(slot) = self.fields.get_mut(&caller_var)
            {
                slot.value = coerced;
            }
        }
    }

    /// The values a view's fields currently hold, ready to be written to the database.
    fn view_buffer(&self, binding: &ViewBinding) -> Vec<(String, Value)> {
        binding
            .fields
            .iter()
            .filter_map(|name| {
                self.fields
                    .get(name)
                    .map(|field| (name.clone(), field.value.clone()))
            })
            .collect()
    }

    /// Moves a histogram to its next distinct value, publishing the count in *NUMBER.
    fn advance_histogram(&mut self, key: usize, descriptor: &str) -> bool {
        let Some(cursor) = self.histograms.get_mut(&key) else {
            return false;
        };
        let Some((value, count)) = cursor.values.get(cursor.next).cloned() else {
            return false;
        };
        cursor.next += 1;
        let processed = cursor.next;

        if let Some(field) = self.fields.get_mut(descriptor) {
            field.value = value;
        }
        self.set_loop_variable("*NUMBER", key, count);
        self.set_loop_variable("*COUNTER", key, processed);
        true
    }

    /// Evaluates a search condition against one stored record.
    ///
    /// The record's fields are read straight from the database rather than from the view
    /// buffer, so selecting records never disturbs the record the program is looking at.
    fn record_matches(
        &self,
        record: usize,
        condition: &Condition,
        line: usize,
    ) -> Result<bool, NaturalError> {
        let left = self.operand_for_record(record, &condition.left)?;
        let right = self.operand_for_record(record, &condition.right)?;
        compare(&left, &right, condition.op, line)
    }

    fn operand_for_record(&self, record: usize, operand: &Operand) -> Result<Value, NaturalError> {
        match operand {
            Operand::Literal(value) => Ok(value.clone()),
            Operand::Variable { name, .. } => match self.database.value(record, name) {
                Some(value) => Ok(value),
                // Not a database field, so it is an ordinary program field.
                None => self.resolve(operand),
            },
        }
    }

    fn set_system_variable(&mut self, name: &str, count: usize) {
        if let Some(field) = self.fields.get_mut(name) {
            field.value = Value::Number(Decimal::from(count as u64));
        }
    }

    /// Sets a system variable both in its bare form and under any label naming this loop.
    /// The labelled copy is the one that survives the loop, which is the whole point of
    /// reference notation.
    fn set_loop_variable(&mut self, name: &str, key: usize, count: usize) {
        self.set_system_variable(name, count);
        let labelled: Vec<String> = self
            .program
            .labels
            .iter()
            .filter(|(_, k)| **k == key)
            .map(|(label, _)| format!("{name}({label}.)"))
            .collect();
        for field in labelled {
            self.set_system_variable(&field, count);
        }
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

        let processed = self.cursors.get(&key).map(|c| c.next).unwrap_or(0);
        self.set_loop_variable("*COUNTER", key, processed);
        // UPDATE and DELETE act on whatever was bound most recently, which is the
        // innermost active loop's record.
        self.current_record = Some(record);

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
                .ok_or_else(|| missing_field(name, *field_line))?;
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
            None => Err(missing_field(name, line)),
        }
    }

    fn resolve(&self, operand: &Operand) -> Result<Value, NaturalError> {
        match operand {
            Operand::Literal(value) => Ok(value.clone()),
            Operand::Variable { name, line } => self
                .fields
                .get(name)
                .map(|f| f.value.clone())
                .ok_or_else(|| missing_field(name, *line)),
        }
    }

    fn format_of(&self, name: &str, line: usize) -> Result<Format, NaturalError> {
        self.fields
            .get(name)
            .map(|f| f.format.clone())
            .ok_or_else(|| missing_field(name, line))
    }

    fn assign(&mut self, name: &str, value: Value, line: usize) -> Result<(), NaturalError> {
        match self.fields.get_mut(name) {
            Some(field) => {
                field.value = value;
                Ok(())
            }
            None => Err(missing_field(name, line)),
        }
    }
}

/// Names the right concept for a name that resolved to nothing. Reference notation that
/// finds no matching loop is a label problem, not an undeclared field, and saying so is the
/// difference between a learner fixing the label and hunting for a DEFINE DATA entry.
fn missing_field(name: &str, line: usize) -> NaturalError {
    if let Some(label) = reference_label(name) {
        return NaturalError::UnknownLabel { name: label, line };
    }
    // A name carrying an arithmetic operator is almost always one written without spaces,
    // as in #A*2. Reporting it as an undeclared field sends the learner to DEFINE DATA to
    // look for something that was never meant to be a field.
    if name.len() > 1 && name[1..].contains(['*', '/', '+']) {
        return NaturalError::MissingOperatorSpaces {
            written: name.to_string(),
            spaced: spaced_out(name),
            line,
        };
    }
    NaturalError::UndeclaredVariable {
        name: name.to_string(),
        line,
    }
}

/// Rewrites `#A*2` as `#A * 2` for a diagnostic. The leading character is kept as-is
/// because `*` also begins a system variable name.
fn spaced_out(name: &str) -> String {
    let mut out = String::with_capacity(name.len() * 3);
    for (index, c) in name.chars().enumerate() {
        if index > 0 && matches!(c, '*' | '/' | '+') {
            out.push(' ');
            out.push(c);
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Extracts `EMP` from `*NUMBER(EMP.)`.
fn reference_label(name: &str) -> Option<String> {
    let inner = name.strip_prefix('*')?.split_once('(')?.1;
    let label = inner.strip_suffix(")")?.strip_suffix('.')?;
    (!label.is_empty()).then(|| label.to_string())
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

/// Compares two values, requiring them to be of the same kind.
///
/// Silently coercing text and numbers would let a learner write a comparison that quietly
/// never matches, which is worse than an error. Alphanumeric comparison ignores trailing
/// blanks, matching Natural's padding of the shorter operand.
fn compare(left: &Value, right: &Value, op: CompareOp, line: usize) -> Result<bool, NaturalError> {
    let ordering = match (left, right) {
        (Value::Number(a), Value::Number(b)) => a.cmp(b),
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
    Ok(op.holds(ordering))
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
