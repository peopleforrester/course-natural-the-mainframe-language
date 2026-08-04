// ABOUTME: Builds declarations and a flat statement list from tokens. One source line is
// ABOUTME: one statement, the optional DEFINE DATA block comes first, and END terminates.

use crate::error::NaturalError;
use crate::lexer::{self, Token};
use crate::screen::Attribute;
use crate::value::{Format, Value};

/// A value a statement operates on, resolved at run time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Literal(Value),
    Variable { name: String, line: usize },
}

/// One element of a WRITE statement's output list.
///
/// A literal is emitted verbatim with no padding. A field is padded to its declared print
/// width. WRITE separates consecutive elements with exactly one blank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteItem {
    Literal(String),
    Field { name: String, line: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Write {
        items: Vec<WriteItem>,
    },
    /// Column-oriented report output, with generated headers and an underline.
    Display {
        fields: Vec<(String, usize)>,
        line: usize,
    },
    Move {
        source: Operand,
        target: String,
        line: usize,
    },
    Reset {
        targets: Vec<(String, usize)>,
    },
    Input {
        /// An optional literal shown before reading. Without one, the field name is used.
        prompt: Option<String>,
        targets: Vec<(String, usize)>,
    },
    /// Evaluate an expression and store it, truncating to the target's scale unless
    /// `rounded` is set.
    Compute {
        target: String,
        expr: Expr,
        rounded: bool,
        line: usize,
    },
    /// Evaluate the condition and jump to `target` when it is FALSE.
    ///
    /// Blocks are compiled to a flat instruction list with jumps rather than a nested
    /// tree. That keeps execution an explicit program-counter loop, which is what lets the
    /// interpreter suspend anywhere, including inside a conditional branch.
    IfFalseJump {
        condition: Condition,
        target: usize,
        line: usize,
    },
    /// Evaluate the condition and jump to `target` when it is TRUE.
    IfTrueJump {
        condition: Condition,
        target: usize,
        line: usize,
    },
    /// Unconditional jump, used to skip an ELSE branch or close a loop.
    Jump {
        target: usize,
    },
    /// Start a counted loop: set the control field to `from`, then jump to `exit` if the
    /// range is already empty.
    ForInit {
        var: String,
        from: Operand,
        to: Operand,
        exit: usize,
        line: usize,
    },
    /// End a counted loop: step the control field, then jump to `top` if it still fits.
    ForNext {
        var: String,
        to: Operand,
        top: usize,
        line: usize,
    },
    /// Begin a database loop: resolve the record set, bind the first record, or jump to
    /// `exit` when the set is empty.
    ReadInit {
        view: String,
        by: Option<String>,
        limit: Option<usize>,
        /// Identifies this loop's cursor, so nested and repeated reads stay independent.
        key: usize,
        exit: usize,
        line: usize,
    },
    /// Advance a database loop: bind the next record and jump to `top`, or fall through.
    ReadNext {
        view: String,
        key: usize,
        top: usize,
        line: usize,
    },
    /// Begin a descriptor search. WITH selects the records, WHERE narrows them afterwards,
    /// which is why *NUMBER reports the WITH count rather than the surviving count.
    FindInit {
        view: String,
        with: Condition,
        filter: Option<Condition>,
        sorted_by: Option<String>,
        limit: Option<usize>,
        key: usize,
        line: usize,
    },
    /// Jump when the search that owns `key` found nothing.
    JumpIfNoRecords {
        key: usize,
        target: usize,
    },
    /// Empty every field of a view. Emitted at the end of an IF NO RECORDS FOUND clause,
    /// because Natural resets the loop's database fields before the single pass it runs.
    ResetViewFields {
        view: String,
        line: usize,
    },
    /// Do nothing, deliberately. Natural has no empty statement, so a clause that should
    /// take no action says IGNORE rather than being left blank.
    Ignore,
    /// Sensitize AID keys. An unsensitized PF key delivers ENTR to the program, so without
    /// this a test of *PF-KEY for PF3 can never be true.
    SetKey {
        keys: Vec<String>,
        line: usize,
    },
    /// Append the view buffer as a new record.
    Store {
        view: String,
        line: usize,
    },
    /// Write the view buffer back over the record the active loop is holding.
    UpdateRecord {
        line: usize,
    },
    /// Remove the record the active loop is holding.
    DeleteRecord {
        line: usize,
    },
    /// Commit every change made since the last transaction boundary.
    EndTransaction,
    /// Discard every change made since the last transaction boundary.
    BackoutTransaction,
    /// Read distinct values of a descriptor, with *NUMBER holding each value's count.
    HistogramInit {
        view: String,
        descriptor: String,
        limit: Option<usize>,
        key: usize,
        exit: usize,
        line: usize,
    },
    /// Advance a histogram to the next distinct value.
    HistogramNext {
        key: usize,
        descriptor: String,
        top: usize,
        line: usize,
    },
    /// Call an inline subroutine, remembering where to come back to.
    Perform {
        name: String,
        line: usize,
    },
    /// Call a separate subprogram object, passing arguments to its parameter block.
    Callnat {
        name: String,
        args: Vec<Operand>,
        line: usize,
    },
    /// Return to whatever performed this subroutine.
    ReturnFromSubroutine,
    /// Present a map and wait for the operator to fill it in.
    InputUsingMap {
        map: String,
        line: usize,
    },
    /// Send the operator back to the screen they just completed, with a message.
    Reinput {
        message: Option<String>,
        line: usize,
    },
}

/// One element of a map definition, before it becomes a positioned screen field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapElement {
    pub row: usize,
    pub column: usize,
    /// The literal label shown before the entry field, if any.
    pub label: Option<String>,
    /// The program field this element reads into. Absent for a TEXT element.
    pub bound_to: Option<String>,
    /// An explicit attribute from an (AD=..) clause, if given.
    pub attribute: Option<Attribute>,
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

impl CompareOp {
    fn parse(token: &str) -> Option<CompareOp> {
        match token.to_ascii_uppercase().as_str() {
            "=" | "EQ" | "==" => Some(CompareOp::Eq),
            "<>" | "NE" | "!=" | "^=" => Some(CompareOp::Ne),
            ">" | "GT" => Some(CompareOp::Gt),
            "<" | "LT" => Some(CompareOp::Lt),
            ">=" | "GE" => Some(CompareOp::Ge),
            "<=" | "LE" => Some(CompareOp::Le),
            _ => None,
        }
    }

    /// Applies the operator to the result of an ordering comparison.
    pub fn holds(self, ordering: std::cmp::Ordering) -> bool {
        use std::cmp::Ordering::{Equal, Greater, Less};
        match self {
            CompareOp::Eq => ordering == Equal,
            CompareOp::Ne => ordering != Equal,
            CompareOp::Gt => ordering == Greater,
            CompareOp::Lt => ordering == Less,
            CompareOp::Ge => matches!(ordering, Greater | Equal),
            CompareOp::Le => matches!(ordering, Less | Equal),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub left: Operand,
    pub op: CompareOp,
    pub right: Operand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// An arithmetic expression tree.
///
/// Expressions may be evaluated recursively, unlike statements. The constraint that
/// forbids Rust recursion applies to statement execution only, because a suspension can
/// occur between statements but never in the middle of evaluating an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Value(Operand),
    Binary {
        left: Box<Expr>,
        op: ArithOp,
        right: Box<Expr>,
    },
}

/// Where an ESCAPE should land, resolved when its loop is closed.
#[derive(Debug, Clone, Copy)]
enum EscapeKind {
    /// Leave the loop entirely.
    Bottom,
    /// Begin the next iteration.
    Top,
}

/// An ESCAPE waiting for its enclosing loop's targets to become known.
struct PendingEscape {
    index: usize,
    kind: EscapeKind,
}

/// A block whose jump target is not known until its closing keyword is reached.
enum OpenBlock {
    /// An IF with no ELSE yet. Holds the index of its IfFalseJump.
    If { false_jump: usize, line: usize },
    /// An IF whose ELSE has been seen. Holds the index of the Jump that skips the ELSE.
    Else { end_jump: usize, line: usize },
    For {
        init: usize,
        var: String,
        to: Operand,
        body_start: usize,
        escapes: Vec<PendingEscape>,
        line: usize,
    },
    Repeat {
        top: usize,
        /// A UNTIL or WHILE guard at the top of the loop, if present.
        guard: Option<usize>,
        escapes: Vec<PendingEscape>,
        line: usize,
    },
    Read {
        init: usize,
        view: String,
        body_start: usize,
        escapes: Vec<PendingEscape>,
        line: usize,
    },
    Subroutine {
        /// The jump that carries normal flow past the definition.
        skip: usize,
        line: usize,
    },
    Histogram {
        init: usize,
        descriptor: String,
        body_start: usize,
        escapes: Vec<PendingEscape>,
        line: usize,
    },
    Find {
        key: usize,
        view: String,
        /// The guard emitted after FindInit, patched to the NOREC body or to the exit.
        guard: usize,
        /// Where the main loop body begins. Moves past the NOREC clause once one is seen.
        body_start: usize,
        /// The jump that skips the NOREC body when records were found.
        skip_norec: Option<usize>,
        /// Whether a NOREC clause was seen, which decides how the guard is patched.
        has_norec: bool,
        in_norec: bool,
        escapes: Vec<PendingEscape>,
        line: usize,
    },
    Decide {
        /// The operand every VALUE clause is compared against, or None for DECIDE FOR.
        subject: Option<Operand>,
        /// FIRST stops after one match; EVERY lets execution fall through to later tests.
        first: bool,
        /// Jumps out of a matched branch, patched to the END-DECIDE position.
        end_jumps: Vec<usize>,
        /// The current clause's "did not match" jump, patched when the next clause opens.
        pending_next: Option<usize>,
        /// True once a clause body has been emitted, so the next clause knows to close it.
        in_clause: bool,
        /// Whether the required NONE clause has been seen. Both DECIDE forms show ANY and
        /// ALL bracketed and NONE unbracketed, and brackets are what mark an element
        /// optional, so a DECIDE without NONE does not compile.
        has_none: bool,
        line: usize,
    },
}

impl OpenBlock {
    fn line(&self) -> usize {
        match self {
            OpenBlock::If { line, .. }
            | OpenBlock::Else { line, .. }
            | OpenBlock::For { line, .. }
            | OpenBlock::Repeat { line, .. }
            | OpenBlock::Read { line, .. }
            | OpenBlock::Find { line, .. }
            | OpenBlock::Histogram { line, .. }
            | OpenBlock::Subroutine { line, .. }
            | OpenBlock::Decide { line, .. } => *line,
        }
    }

    /// True for the loop kinds, which are the blocks an ESCAPE can target. A READ is a
    /// loop in Natural exactly as FOR and REPEAT are, which is the point module 8 opens on.
    fn is_loop(&self) -> bool {
        matches!(
            self,
            OpenBlock::For { .. }
                | OpenBlock::Repeat { .. }
                | OpenBlock::Read { .. }
                | OpenBlock::Find { .. }
                | OpenBlock::Histogram { .. }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    pub name: String,
    pub format: Format,
    pub line: usize,
}

/// A `VIEW OF` declaration: a named window onto some of a database file's fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewDeclaration {
    pub name: String,
    pub ddm: String,
    /// The DDM field names the program made visible, in declaration order.
    pub fields: Vec<(String, usize)>,
    pub line: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
    /// Fields declared in a DEFINE DATA PARAMETER block, in order. These are what a
    /// CALLNAT binds to, and their order is the call's parameter order.
    pub parameters: Vec<Declaration>,
    pub views: Vec<ViewDeclaration>,
    pub statements: Vec<Statement>,
    /// Where each inline subroutine's body begins. Names resolve at run time, so a
    /// subroutine may be performed before the line that defines it.
    pub subroutines: std::collections::BTreeMap<String, usize>,
    /// Statement labels, mapping the label to the loop key it names. A label is what makes
    /// reference notation such as *NUMBER(EMP.) possible outside the loop itself.
    pub labels: std::collections::BTreeMap<String, usize>,
}

/// Natural identifiers are not case sensitive, so every name is stored folded.
pub fn normalize(name: &str) -> String {
    name.to_ascii_uppercase()
}

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    /// Before any executable statement, where a DEFINE DATA block may still open.
    Preamble,
    /// The object's single DEFINE DATA has closed. A second one is an error rather than a
    /// second block, because PARAMETER and LOCAL are clauses of the one statement.
    AfterDefine,
    /// Inside a DEFINE DATA LOCAL block.
    DataBlock,
    /// Inside a DEFINE DATA PARAMETER block, whose fields become the call interface.
    ParameterBlock,
    /// After the data blocks, reading executable statements.
    Body,
}

pub fn parse(source: &str) -> Result<Program, NaturalError> {
    let tokens = lexer::tokenize(source)?;
    let mut program = Program::default();
    let mut mode = Mode::Preamble;
    let mut blocks: Vec<OpenBlock> = Vec::new();
    let mut current: Vec<Token> = Vec::new();
    let mut finished = false;

    for token in tokens {
        if matches!(token, Token::Newline) {
            if parse_line(&current, &mut program, &mut mode, &mut blocks)? {
                finished = true;
                break;
            }
            current.clear();
        } else {
            current.push(token);
        }
    }

    // A final line with no trailing newline is still a statement.
    if !finished && parse_line(&current, &mut program, &mut mode, &mut blocks)? {
        finished = true;
    }

    // An unclosed block is reported before a missing END, because it is the more specific
    // and more useful diagnostic.
    if let Some(open) = blocks.first() {
        let line = open.line();
        return Err(match open {
            OpenBlock::If { .. } | OpenBlock::Else { .. } => NaturalError::MissingEndIf { line },
            OpenBlock::For { .. } => NaturalError::MissingLoopEnd {
                keyword: "FOR".to_string(),
                closer: "END-FOR".to_string(),
                line,
            },
            OpenBlock::Repeat { .. } => NaturalError::MissingLoopEnd {
                keyword: "REPEAT".to_string(),
                closer: "END-REPEAT".to_string(),
                line,
            },
            OpenBlock::Read { .. } => NaturalError::MissingLoopEnd {
                keyword: "READ".to_string(),
                closer: "END-READ".to_string(),
                line,
            },
            OpenBlock::Find { .. } => NaturalError::MissingLoopEnd {
                keyword: "FIND".to_string(),
                closer: "END-FIND".to_string(),
                line,
            },
            OpenBlock::Histogram { .. } => NaturalError::MissingLoopEnd {
                keyword: "HISTOGRAM".to_string(),
                closer: "END-HISTOGRAM".to_string(),
                line,
            },
            OpenBlock::Subroutine { .. } => NaturalError::MissingLoopEnd {
                keyword: "DEFINE SUBROUTINE".to_string(),
                closer: "END-SUBROUTINE".to_string(),
                line,
            },
            OpenBlock::Decide { .. } => NaturalError::MissingLoopEnd {
                keyword: "DECIDE".to_string(),
                closer: "END-DECIDE".to_string(),
                line,
            },
        });
    }
    if finished {
        return Ok(program);
    }
    if mode == Mode::DataBlock {
        return Err(NaturalError::MissingEndDefine);
    }
    Err(NaturalError::MissingEnd)
}

/// Parses one source line. Returns Ok(true) when the line was END, which terminates the
/// program. A blank line is ignored.
fn parse_line(
    tokens: &[Token],
    program: &mut Program,
    mode: &mut Mode,
    blocks: &mut Vec<OpenBlock>,
) -> Result<bool, NaturalError> {
    // Reference notation arrives as four tokens because the lexer splits parentheses.
    // Rejoining them here keeps *NUMBER(EMP.) an ordinary field name everywhere else.
    let tokens = &join_reference_notation(tokens);

    // A statement label prefixes the statement it names, as in EMP. FIND ... . Strip it
    // before dispatch so every statement arm sees the keyword it expects.
    let (label, tokens) = split_label(tokens);

    let Some(first) = tokens.first() else {
        return Ok(false);
    };

    let (head, line) = match first {
        Token::Word { text, line } => (text.to_ascii_uppercase(), *line),
        Token::Text { line, .. } => {
            return Err(NaturalError::UnknownStatement {
                name: "a text literal".to_string(),
                line: *line,
            });
        }
        Token::Newline => return Ok(false),
    };

    if let Some(name) = &label
        && !matches!(head.as_str(), "FIND" | "READ" | "HISTOGRAM")
    {
        return Err(NaturalError::UnknownStatement {
            name: format!(
                "'{name}.' as a label on {head}. Label a FIND, READ, or HISTOGRAM, which are \
                 the loops reference notation can name"
            ),
            line,
        });
    }

    if matches!(*mode, Mode::DataBlock | Mode::ParameterBlock) {
        if head == "END-DEFINE" {
            // An object has exactly one DEFINE DATA, so this closes the only one there is.
            *mode = Mode::AfterDefine;
            return Ok(false);
        }
        // PARAMETER and LOCAL are clauses within the block, so meeting one switches which
        // clause the following declarations belong to rather than opening a new statement.
        if head == "PARAMETER" {
            *mode = Mode::ParameterBlock;
            return Ok(false);
        }
        if head == "LOCAL" {
            *mode = Mode::DataBlock;
            return Ok(false);
        }
        // Reaching the end of the program while still inside the block means END-DEFINE
        // was forgotten. Say that, rather than complaining about a missing level number.
        if head == "END" {
            return Err(NaturalError::MissingEndDefine);
        }
        parse_declaration(tokens, line, program, *mode == Mode::ParameterBlock)?;
        return Ok(false);
    }

    // A map is a separate object, not part of a program. There is no DEFINE MAP statement
    // in Natural: the DEFINE statements are CLASS, DATA, FUNCTION, PRINTER, PROTOTYPE,
    // SUBROUTINE, WINDOW, and WORK FILE. A real map is drawn in the map editor and saved
    // under its own name, which is why a program can only ever refer to one.
    if head == "DEFINE" && is_map_definition(tokens) {
        return Err(NaturalError::UnknownStatement {
            name: "DEFINE MAP. There is no such statement: a map is a separate object, so \
                   build it in the Library tab and name it with INPUT USING MAP 'NAME'"
                .to_string(),
            line,
        });
    }

    // DEFINE SUBROUTINE is a statement, unlike DEFINE DATA which opens the data block.
    if head == "DEFINE" && is_subroutine_definition(tokens) {
        *mode = Mode::Body;
        let name = subroutine_name(tokens, line)?;
        if program.subroutines.contains_key(&name) {
            return Err(NaturalError::DuplicateSubroutine { name, line });
        }
        // Normal flow jumps over the definition; only PERFORM enters it.
        let skip = program.statements.len();
        program
            .statements
            .push(Statement::Jump { target: usize::MAX });
        program
            .subroutines
            .insert(name.clone(), program.statements.len());
        program.subroutines.insert(name, program.statements.len());
        blocks.push(OpenBlock::Subroutine { skip, line });
        return Ok(false);
    }

    if head == "DEFINE" {
        if *mode == Mode::AfterDefine {
            return Err(NaturalError::RepeatedDefineData { line });
        }
        if *mode != Mode::Preamble {
            return Err(NaturalError::DefineDataNotFirst { line });
        }
        *mode = parse_define_data_header(tokens, line)?;
        return Ok(false);
    }

    // Any other statement closes the window in which DEFINE DATA may appear.
    *mode = Mode::Body;

    match head.as_str() {
        // END TRANSACTION is a statement. It has to be tested before the bare END arm,
        // because match arms are tried in order and a bare END terminates the program.
        "END" if is_transaction_boundary(tokens) => {
            program.statements.push(Statement::EndTransaction);
            Ok(false)
        }
        "END" => Ok(true),
        "END-DEFINE" => Err(NaturalError::UnknownStatement {
            name: "END-DEFINE without a DEFINE DATA block".to_string(),
            line,
        }),
        "DISPLAY" => {
            let mut fields = Vec::new();
            for token in strip_report_options(&tokens[1..]) {
                match token {
                    Token::Word { text, line: l } => fields.push((normalize(text), *l)),
                    Token::Text { .. } => {
                        return Err(NaturalError::NotYetSupported {
                            feature: "a header literal in DISPLAY".to_string(),
                            line,
                        });
                    }
                    Token::Newline => {}
                }
            }
            if fields.is_empty() {
                return Err(NaturalError::UnknownStatement {
                    name: "DISPLAY with no fields, as in DISPLAY #NAME #AGE".to_string(),
                    line,
                });
            }
            program.statements.push(Statement::Display { fields, line });
            Ok(false)
        }
        "WRITE" => {
            program.statements.push(parse_write(tokens, line)?);
            Ok(false)
        }
        "MOVE" => {
            program.statements.push(parse_move(tokens, line)?);
            Ok(false)
        }
        "RESET" => {
            program.statements.push(parse_reset(tokens, line)?);
            Ok(false)
        }
        "INPUT" if is_map_input(tokens) => {
            // The map name is a constant, so it is quoted. A learner who writes it bare
            // gets told here rather than discovering it on a real system.
            let Some(Token::Text { value, .. }) = tokens.get(3) else {
                return Err(NaturalError::UnknownStatement {
                    name: "INPUT USING MAP without a quoted map name, as in \
                           INPUT USING MAP 'EMPMAP'"
                        .to_string(),
                    line,
                });
            };
            validate_object_name(value, line)?;
            let map = normalize(value);
            program
                .statements
                .push(Statement::InputUsingMap { map, line });
            Ok(false)
        }
        "INPUT" => {
            program.statements.push(parse_input(tokens, line)?);
            Ok(false)
        }
        "COMPUTE" | "ASSIGN" => {
            program.statements.push(parse_compute(&tokens[1..], line)?);
            Ok(false)
        }
        "ADD" | "SUBTRACT" | "MULTIPLY" | "DIVIDE" => {
            program
                .statements
                .push(parse_arithmetic_verb(&head, &tokens[1..], line)?);
            Ok(false)
        }
        // IF NO RECORDS FOUND is a clause of the enclosing FIND, not a conditional.
        "IF" if is_no_records_clause(tokens) => {
            let Some(OpenBlock::Find {
                guard,
                skip_norec,
                in_norec,
                ..
            }) = blocks.last_mut()
            else {
                return Err(NaturalError::UnexpectedBlockKeyword {
                    keyword: "IF NO RECORDS FOUND".to_string(),
                    hint: "It belongs directly inside a FIND block.".to_string(),
                    line,
                });
            };
            if skip_norec.is_some() {
                return Err(NaturalError::UnexpectedBlockKeyword {
                    keyword: "IF NO RECORDS FOUND".to_string(),
                    hint: "A FIND takes only one of these clauses.".to_string(),
                    line,
                });
            }
            *in_norec = true;
            let guard = *guard;
            // When records WERE found, jump over the clause to the main loop body.
            let skip = program.statements.len();
            program
                .statements
                .push(Statement::Jump { target: usize::MAX });
            // When none were found, the guard lands on the clause body, just past the skip.
            let clause_start = program.statements.len();
            patch_target(program, guard, clause_start);
            if let Some(OpenBlock::Find {
                skip_norec,
                has_norec,
                ..
            }) = blocks.last_mut()
            {
                *skip_norec = Some(skip);
                *has_norec = true;
            }
            Ok(false)
        }
        "END-NOREC" => {
            let Some(OpenBlock::Find {
                skip_norec,
                body_start,
                in_norec,
                view,
                ..
            }) = blocks.last_mut()
            else {
                return Err(block_mismatch("END-NOREC", "IF NO RECORDS FOUND", line));
            };
            if !*in_norec {
                return Err(block_mismatch("END-NOREC", "IF NO RECORDS FOUND", line));
            }
            *in_norec = false;
            let skip = skip_norec.expect("in_norec implies the skip jump exists");
            let view = view.clone();
            // The documented behavior, and the surprising part: the clause does not replace
            // the loop. Natural enters the loop once anyway, having "reset to empty all
            // database fields which reference the file specified in the current loop".
            // Falling through into the body is therefore correct, and ESCAPE BOTTOM inside
            // the clause is the documented way to skip that pass.
            program
                .statements
                .push(Statement::ResetViewFields { view, line });
            // The main loop body begins after the clause.
            let main = program.statements.len();
            *body_start = main;
            patch_target(program, skip, main);
            Ok(false)
        }
        "IF" => {
            let condition = parse_condition(&tokens[1..], line)?;
            blocks.push(OpenBlock::If {
                false_jump: program.statements.len(),
                line,
            });
            // The target is patched when ELSE or END-IF is reached.
            program.statements.push(Statement::IfFalseJump {
                condition,
                target: usize::MAX,
                line,
            });
            Ok(false)
        }
        "ELSE" => {
            let Some(OpenBlock::If {
                false_jump,
                line: open_line,
            }) = pop_matching(blocks, |b| matches!(b, OpenBlock::If { .. }))
            else {
                return Err(NaturalError::UnexpectedBlockKeyword {
                    keyword: "ELSE".to_string(),
                    hint: "An ELSE needs an IF above it, and each IF takes only one ELSE."
                        .to_string(),
                    line,
                });
            };
            // Close the THEN branch by jumping over the ELSE branch.
            let end_jump = program.statements.len();
            program
                .statements
                .push(Statement::Jump { target: usize::MAX });
            // A false condition now lands on the first statement of the ELSE branch.
            patch_target(program, false_jump, program_len(program));
            blocks.push(OpenBlock::Else {
                end_jump,
                line: open_line,
            });
            Ok(false)
        }
        "DECIDE" => {
            let (subject, first) = parse_decide_header(&tokens[1..], line)?;
            blocks.push(OpenBlock::Decide {
                subject,
                first,
                end_jumps: Vec::new(),
                pending_next: None,
                in_clause: false,
                has_none: false,
                line,
            });
            Ok(false)
        }
        "IGNORE" => {
            program.statements.push(Statement::Ignore);
            Ok(false)
        }
        "SET"
            if matches!(tokens.get(1), Some(Token::Word { text, .. })
            if text.eq_ignore_ascii_case("KEY")) =>
        {
            // Bare SET KEY sensitizes every key; naming keys sensitizes just those.
            let keys: Vec<String> = tokens[2..]
                .iter()
                .filter_map(word_text)
                .map(|w| normalize(&w))
                .collect();
            program.statements.push(Statement::SetKey { keys, line });
            Ok(false)
        }
        "VALUE" | "WHEN" | "NONE" => {
            open_decide_clause(&head, tokens, line, program, blocks)?;
            Ok(false)
        }
        "END-DECIDE" => {
            let Some(OpenBlock::Decide {
                end_jumps,
                pending_next,
                has_none,
                subject,
                ..
            }) = pop_matching(blocks, |b| matches!(b, OpenBlock::Decide { .. }))
            else {
                return Err(block_mismatch("END-DECIDE", "DECIDE", line));
            };
            if !has_none {
                // Use IGNORE when the answer really is "do nothing", which is what the
                // documentation's own note tells you to write.
                let on = subject.is_some();
                return Err(NaturalError::MissingNoneClause {
                    keyword: if on { "DECIDE ON" } else { "DECIDE FOR" }.to_string(),
                    clause: if on { "NONE VALUE" } else { "WHEN NONE" }.to_string(),
                    line,
                });
            }
            let here = program_len(program);
            // The last clause's body falls through to here, so it needs no closing jump.
            if let Some(pending) = pending_next {
                patch_target(program, pending, here);
            }
            for jump in end_jumps {
                patch_target(program, jump, here);
            }
            Ok(false)
        }
        "FIND" => {
            let header = parse_find_header(&tokens[1..], line)?;
            let key = program.statements.len();
            register_label(program, &label, key);
            program.statements.push(Statement::FindInit {
                view: header.view.clone(),
                with: header.with,
                filter: header.filter,
                sorted_by: header.sorted_by,
                limit: header.limit,
                key,
                line,
            });
            // The guard runs the NOREC clause when one exists, and otherwise skips the
            // whole loop. Its target is only known once the block's shape is seen.
            let guard = program.statements.len();
            program.statements.push(Statement::JumpIfNoRecords {
                key,
                target: usize::MAX,
            });
            blocks.push(OpenBlock::Find {
                key,
                view: header.view,
                guard,
                body_start: program.statements.len(),
                skip_norec: None,
                has_norec: false,
                in_norec: false,
                escapes: Vec::new(),
                line,
            });
            Ok(false)
        }
        "END-FIND" => {
            let Some(OpenBlock::Find {
                key,
                view,
                guard,
                body_start,
                has_norec,
                escapes,
                ..
            }) = pop_matching(blocks, |b| matches!(b, OpenBlock::Find { .. }))
            else {
                return Err(block_mismatch("END-FIND", "FIND", line));
            };
            let next = program.statements.len();
            program.statements.push(Statement::ReadNext {
                view,
                key,
                top: body_start,
                line,
            });
            let after = program.statements.len();
            // With a NOREC clause the guard already points at that clause, which falls
            // through into the single reset pass. Without one, an empty search has nothing
            // to run before the loop, so it skips the loop entirely.
            if !has_norec {
                patch_target(program, guard, after);
            }
            patch_escapes(program, escapes, after, next);
            Ok(false)
        }
        "REINPUT" => {
            let message = tokens.iter().find_map(|t| match t {
                Token::Text { value, .. } => Some(value.clone()),
                _ => None,
            });
            program
                .statements
                .push(Statement::Reinput { message, line });
            Ok(false)
        }
        "CALLNAT" => {
            let Some(Token::Text { value, .. }) = tokens.get(1) else {
                return Err(NaturalError::UnknownStatement {
                    name: "CALLNAT without a subprogram name, as in CALLNAT 'DOUBLEIT' #A"
                        .to_string(),
                    line,
                });
            };
            validate_object_name(value, line)?;
            let mut args = Vec::new();
            for token in &tokens[2..] {
                if matches!(token, Token::Newline) {
                    continue;
                }
                args.push(operand_from(token, line)?);
            }
            program.statements.push(Statement::Callnat {
                name: normalize(value),
                args,
                line,
            });
            Ok(false)
        }
        "PERFORM" => {
            let words: Vec<String> = tokens.iter().filter_map(word_text).collect();
            let name = words
                .get(1)
                .cloned()
                .ok_or_else(|| NaturalError::UnknownStatement {
                    name: "PERFORM without a subroutine name".to_string(),
                    line,
                })?;
            program.statements.push(Statement::Perform { name, line });
            Ok(false)
        }
        "END-SUBROUTINE" => {
            let Some(OpenBlock::Subroutine { skip, .. }) =
                pop_matching(blocks, |b| matches!(b, OpenBlock::Subroutine { .. }))
            else {
                return Err(block_mismatch("END-SUBROUTINE", "DEFINE SUBROUTINE", line));
            };
            program.statements.push(Statement::ReturnFromSubroutine);
            patch_target(program, skip, program_len(program));
            Ok(false)
        }
        "STORE" => {
            // STORE EMPLOYEES-VIEW, and the fuller STORE RECORD IN EMPLOYEES-VIEW.
            let words: Vec<String> = tokens.iter().filter_map(word_text).collect();
            let view = words
                .iter()
                .skip(1)
                .find(|w| !matches!(w.as_str(), "RECORD" | "IN" | "FILE"))
                .ok_or_else(|| NaturalError::UnknownStatement {
                    name: "STORE without a view, as in STORE EMPLOYEES-VIEW".to_string(),
                    line,
                })?
                .clone();
            program.statements.push(Statement::Store { view, line });
            Ok(false)
        }
        "UPDATE" => {
            program.statements.push(Statement::UpdateRecord { line });
            Ok(false)
        }
        "DELETE" => {
            program.statements.push(Statement::DeleteRecord { line });
            Ok(false)
        }
        "BACKOUT" => {
            program.statements.push(Statement::BackoutTransaction);
            Ok(false)
        }
        "HISTOGRAM" => {
            let (view, descriptor, limit) = parse_histogram_header(&tokens[1..], line)?;
            let init = program.statements.len();
            register_label(program, &label, init);
            program.statements.push(Statement::HistogramInit {
                view: view.clone(),
                descriptor: descriptor.clone(),
                limit,
                key: init,
                exit: usize::MAX,
                line,
            });
            blocks.push(OpenBlock::Histogram {
                init,
                descriptor,
                body_start: program.statements.len(),
                escapes: Vec::new(),
                line,
            });
            Ok(false)
        }
        "END-HISTOGRAM" => {
            let Some(OpenBlock::Histogram {
                init,
                descriptor,
                body_start,
                escapes,
                ..
            }) = pop_matching(blocks, |b| matches!(b, OpenBlock::Histogram { .. }))
            else {
                return Err(block_mismatch("END-HISTOGRAM", "HISTOGRAM", line));
            };
            let next = program.statements.len();
            program.statements.push(Statement::HistogramNext {
                key: init,
                descriptor,
                top: body_start,
                line,
            });
            let after = program.statements.len();
            patch_target(program, init, after);
            patch_escapes(program, escapes, after, next);
            Ok(false)
        }
        "READ" => {
            let (view, by, limit) = parse_read_header(&tokens[1..], line)?;
            let init = program.statements.len();
            register_label(program, &label, init);
            program.statements.push(Statement::ReadInit {
                view: view.clone(),
                by,
                limit,
                key: init,
                exit: usize::MAX,
                line,
            });
            blocks.push(OpenBlock::Read {
                init,
                view,
                body_start: program.statements.len(),
                escapes: Vec::new(),
                line,
            });
            Ok(false)
        }
        "END-READ" => {
            let Some(OpenBlock::Read {
                init,
                view,
                body_start,
                escapes,
                ..
            }) = pop_matching(blocks, |b| matches!(b, OpenBlock::Read { .. }))
            else {
                return Err(block_mismatch("END-READ", "READ", line));
            };
            let next = program.statements.len();
            program.statements.push(Statement::ReadNext {
                view,
                key: init,
                top: body_start,
                line,
            });
            let after = program.statements.len();
            patch_target(program, init, after);
            patch_escapes(program, escapes, after, next);
            Ok(false)
        }
        "FOR" => {
            let (var, from, to) = parse_for_header(&tokens[1..], line)?;
            let init = program.statements.len();
            program.statements.push(Statement::ForInit {
                var: var.clone(),
                from,
                to: to.clone(),
                exit: usize::MAX,
                line,
            });
            blocks.push(OpenBlock::For {
                init,
                var,
                to,
                body_start: program.statements.len(),
                escapes: Vec::new(),
                line,
            });
            Ok(false)
        }
        "END-FOR" => {
            let Some(OpenBlock::For {
                init,
                var,
                to,
                body_start,
                escapes,
                ..
            }) = pop_matching(blocks, |b| matches!(b, OpenBlock::For { .. }))
            else {
                return Err(block_mismatch("END-FOR", "FOR", line));
            };
            // Stepping the control field and re-testing happens here, so ESCAPE TOP lands
            // on this instruction rather than skipping the increment.
            let next = program.statements.len();
            program.statements.push(Statement::ForNext {
                var,
                to,
                top: body_start,
                line,
            });
            let after = program.statements.len();
            patch_target(program, init, after);
            patch_escapes(program, escapes, after, next);
            Ok(false)
        }
        "REPEAT" => {
            let top = program.statements.len();
            let guard = parse_repeat_guard(&tokens[1..], line)?.map(|condition| {
                let index = program.statements.len();
                program.statements.push(condition_jump(condition, line));
                index
            });
            blocks.push(OpenBlock::Repeat {
                top,
                guard,
                escapes: Vec::new(),
                line,
            });
            Ok(false)
        }
        "END-REPEAT" => {
            let Some(OpenBlock::Repeat {
                top,
                guard,
                escapes,
                ..
            }) = pop_matching(blocks, |b| matches!(b, OpenBlock::Repeat { .. }))
            else {
                return Err(block_mismatch("END-REPEAT", "REPEAT", line));
            };
            program.statements.push(Statement::Jump { target: top });
            let after = program.statements.len();
            if let Some(guard) = guard {
                patch_target(program, guard, after);
            }
            patch_escapes(program, escapes, after, top);
            Ok(false)
        }
        "ESCAPE" => {
            let kind = match tokens.get(1) {
                Some(Token::Word { text, .. }) if text.eq_ignore_ascii_case("BOTTOM") => {
                    EscapeKind::Bottom
                }
                Some(Token::Word { text, .. }) if text.eq_ignore_ascii_case("TOP") => {
                    EscapeKind::Top
                }
                _ => {
                    return Err(NaturalError::UnknownStatement {
                        name: "ESCAPE without a direction. Write ESCAPE BOTTOM to leave the \
                               loop, or ESCAPE TOP to start the next pass"
                            .to_string(),
                        line,
                    });
                }
            };
            // An ESCAPE belongs to the nearest enclosing loop, not to an IF it happens to
            // sit inside, so the search skips conditional blocks.
            let Some(loop_block) = blocks.iter_mut().rev().find(|b| b.is_loop()) else {
                return Err(NaturalError::EscapeOutsideLoop { line });
            };
            let index = program.statements.len();
            match loop_block {
                OpenBlock::For { escapes, .. }
                | OpenBlock::Repeat { escapes, .. }
                | OpenBlock::Read { escapes, .. }
                | OpenBlock::Find { escapes, .. }
                | OpenBlock::Histogram { escapes, .. } => {
                    escapes.push(PendingEscape { index, kind });
                }
                _ => unreachable!("is_loop guarantees a loop block"),
            }
            program
                .statements
                .push(Statement::Jump { target: usize::MAX });
            Ok(false)
        }
        "END-IF" => {
            let Some(open) = pop_matching(blocks, |b| {
                matches!(b, OpenBlock::If { .. } | OpenBlock::Else { .. })
            }) else {
                return Err(block_mismatch("END-IF", "IF", line));
            };
            let here = program_len(program);
            match open {
                OpenBlock::If { false_jump, .. } => patch_target(program, false_jump, here),
                OpenBlock::Else { end_jump, .. } => patch_target(program, end_jump, here),
                _ => unreachable!("pop_matching restricted the kinds"),
            }
            Ok(false)
        }
        _ => {
            // An assignment is written target := source, so the line opens with a name.
            if tokens.len() >= 3 && matches!(&tokens[1], Token::Word { text, .. } if text == ":=") {
                program.statements.push(parse_assignment(tokens, line)?);
                return Ok(false);
            }
            Err(NaturalError::UnknownStatement { name: head, line })
        }
    }
}

fn parse_define_data_header(tokens: &[Token], line: usize) -> Result<Mode, NaturalError> {
    let words: Vec<String> = tokens.iter().filter_map(word_text).collect();
    match words.get(1).map(|w| w.as_str()) {
        Some("DATA") => {}
        _ => {
            return Err(NaturalError::UnknownStatement {
                name: "DEFINE without DATA".to_string(),
                line,
            });
        }
    }
    match words.get(2).map(|w| w.as_str()) {
        // A bare DEFINE DATA is treated as LOCAL, which is what the teaching subset uses.
        None | Some("LOCAL") => Ok(Mode::DataBlock),
        Some("PARAMETER") => Ok(Mode::ParameterBlock),
        Some(other) => Err(NaturalError::NotYetSupported {
            feature: format!("DEFINE DATA {other}"),
            line,
        }),
    }
}

fn parse_declaration(
    tokens: &[Token],
    line: usize,
    program: &mut Program,
    is_parameter: bool,
) -> Result<(), NaturalError> {
    let words: Vec<String> = tokens.iter().filter_map(word_text).collect();

    // A view declaration reads: <level> <name> VIEW OF <ddm>
    if words.len() >= 4 && words[2] == "VIEW" && words[3] == "OF" {
        let Some(ddm) = words.get(4) else {
            return Err(NaturalError::UnknownStatement {
                name: "VIEW OF without a file name, as in VIEW OF EMPLOYEES".to_string(),
                line,
            });
        };
        program.views.push(ViewDeclaration {
            name: normalize(&words[1]),
            ddm: normalize(ddm),
            fields: Vec::new(),
            line,
        });
        return Ok(());
    }

    // A field inside the most recent view reads: <level> <ddm-field-name>
    // Those fields take their format from the DDM, so they carry no format specification.
    if words.len() == 2
        && let Some(view) = program.views.last_mut()
        && words[0] != "1"
    {
        view.fields.push((normalize(&words[1]), line));
        return Ok(());
    }

    // A declaration reads: <level> <name> ( <format> )
    let level_looks_numeric = words
        .first()
        .map(|w| w.chars().all(|c| c.is_ascii_digit()))
        .unwrap_or(false);
    if !level_looks_numeric {
        return Err(NaturalError::UnknownStatement {
            name: format!(
                "'{}' inside DEFINE DATA. A field starts with a level number, as in 1 #NAME (A20)",
                words.first().cloned().unwrap_or_default()
            ),
            line,
        });
    }

    let Some(raw_name) = words.get(1) else {
        return Err(NaturalError::InvalidFormat {
            detail: "this field has a level number but no name.".to_string(),
            line,
        });
    };

    let open = words.iter().position(|w| w == "(");
    let close = words.iter().position(|w| w == ")");
    let (open, close) = match (open, close) {
        (Some(o), Some(c)) if c > o + 1 => (o, c),
        _ => {
            return Err(NaturalError::InvalidFormat {
                detail: format!(
                    "'{raw_name}' needs a format in parentheses, as in 1 {raw_name} (A20)."
                ),
                line,
            });
        }
    };

    let spec = words[open + 1..close].join("");
    let format = Format::parse(&spec, line)?;
    let name = normalize(raw_name);

    if program.declarations.iter().any(|d| d.name == name) {
        return Err(NaturalError::DuplicateVariable {
            name: raw_name.clone(),
            line,
        });
    }

    let declaration = Declaration { name, format, line };
    if is_parameter {
        program.parameters.push(declaration.clone());
    }
    program.declarations.push(declaration);
    Ok(())
}

fn parse_write(tokens: &[Token], line: usize) -> Result<Statement, NaturalError> {
    let mut items = Vec::new();
    for token in strip_report_options(&tokens[1..]) {
        match token {
            Token::Text { value, .. } => items.push(WriteItem::Literal(value.clone())),
            Token::Word { text, line } => items.push(WriteItem::Field {
                name: normalize(text),
                line: *line,
            }),
            Token::Newline => {}
        }
    }
    if items.is_empty() {
        // The WRITE syntax diagram requires at least one output element, and the documented
        // way to emit a blank line is SKIP or WRITE with a slash. See
        // research/07-output-formatting-semantics.md rows F6 and F7.
        return Err(NaturalError::UnknownStatement {
            name: "WRITE with nothing to write. Give it something to output, or use SKIP to \
                   leave a blank line"
                .to_string(),
            line,
        });
    }
    Ok(Statement::Write { items })
}

/// Drops the report options that may precede a WRITE or DISPLAY output list.
///
/// NOTITLE suppresses the default page title, which Natural otherwise generates once per
/// page with the page number, the time of day, and the date. NOHDR suppresses a DISPLAY's
/// column headers on a page that a WRITE causes.
///
/// This environment behaves as though NOTITLE were always in effect, so both are accepted
/// and neither changes the output. That is a deliberate divergence, disclosed in lesson 2:
/// a title carrying the current time would make every lesson's output differ between runs,
/// which would break the exercises and teach nothing. What matters is that a learner who
/// writes either keyword is not told it does not exist.
fn strip_report_options(tokens: &[Token]) -> &[Token] {
    let mut at = 0;
    while let Some(Token::Word { text, .. }) = tokens.get(at) {
        if text.eq_ignore_ascii_case("NOTITLE") || text.eq_ignore_ascii_case("NOHDR") {
            at += 1;
        } else {
            break;
        }
    }
    &tokens[at..]
}

fn parse_move(tokens: &[Token], line: usize) -> Result<Statement, NaturalError> {
    // MOVE <source> TO <target>
    let to_at = tokens
        .iter()
        .position(|t| matches!(t, Token::Word { text, .. } if text.eq_ignore_ascii_case("TO")));
    let Some(to_at) = to_at else {
        return Err(NaturalError::UnknownStatement {
            name: "MOVE without TO. Write it as MOVE <value> TO <field>".to_string(),
            line,
        });
    };

    if to_at != 2 || tokens.len() != to_at + 2 {
        return Err(NaturalError::UnknownStatement {
            name: "MOVE takes one value and one target, as in MOVE 'x' TO #FIELD".to_string(),
            line,
        });
    }

    let source = operand_from(&tokens[1], line)?;
    let target = require_name(&tokens[to_at + 1], line)?;
    Ok(Statement::Move {
        source,
        target,
        line,
    })
}

fn parse_assignment(tokens: &[Token], line: usize) -> Result<Statement, NaturalError> {
    if tokens.len() < 3 {
        return Err(NaturalError::UnknownStatement {
            name: "an assignment needs a value, as in #FIELD := 3".to_string(),
            line,
        });
    }
    let target = require_name(&tokens[0], line)?;

    // A single-token right-hand side is a plain move, which is what keeps text and logical
    // assignment working. Anything longer is an arithmetic expression.
    if tokens.len() == 3 {
        return Ok(Statement::Move {
            source: operand_from(&tokens[2], line)?,
            target,
            line,
        });
    }
    Ok(Statement::Compute {
        target,
        expr: parse_expr(&tokens[2..], line)?,
        rounded: false,
        line,
    })
}

fn parse_reset(tokens: &[Token], line: usize) -> Result<Statement, NaturalError> {
    let mut targets = Vec::new();
    for token in &tokens[1..] {
        targets.push((require_name(token, line)?, line));
    }
    if targets.is_empty() {
        return Err(NaturalError::UnknownStatement {
            name: "RESET needs at least one field, as in RESET #TOTAL".to_string(),
            line,
        });
    }
    Ok(Statement::Reset { targets })
}

/// A statement label is a name ending in a period, standing before the statement it names.
/// Only a loop can carry one here, because a label's whole purpose in this course is to let
/// reference notation say which loop a system variable came from.
fn split_label(tokens: &[Token]) -> (Option<String>, &[Token]) {
    let Some(Token::Word { text, .. }) = tokens.first() else {
        return (None, tokens);
    };
    // A label is a bare name with a trailing period and nothing else, which distinguishes
    // it from decimals and from END-FIND style keywords.
    let Some(name) = text.strip_suffix('.') else {
        return (None, tokens);
    };
    if name.is_empty() || tokens.len() < 2 || !name.chars().all(is_label_char) {
        return (None, tokens);
    }
    (Some(normalize(name)), &tokens[1..])
}

fn is_label_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '#'
}

/// The reserved keywords, one per line, with comment lines describing where they came from.
const RESERVED_KEYWORDS: &str = include_str!("../data/reserved-keywords.txt");

/// True when `name` is a Natural reserved keyword.
pub fn is_reserved_keyword(name: &str) -> bool {
    let name = normalize(name);
    RESERVED_KEYWORDS
        .lines()
        .any(|line| !line.starts_with('#') && line == name)
}

/// Checks a name that will be saved as a library object.
///
/// "The name of a Natural object can be 1 to 8 characters ... the first character must be
/// an upper-case alphabetical character, a number sign (#), or a plus sign (+)." A DDM is
/// the documented exception at 1 to 32, which is why a view name never comes through here.
fn validate_object_name(name: &str, line: usize) -> Result<(), NaturalError> {
    let name = normalize(name);
    let length = name.chars().count();
    if length > 8 {
        return Err(NaturalError::ObjectNameTooLong { name, length, line });
    }
    let first = name.chars().next().unwrap_or(' ');
    if !(first.is_ascii_uppercase() || first == '#' || first == '+') {
        return Err(NaturalError::UnknownStatement {
            name: format!(
                "'{name}' as an object name. The first character must be a letter, a number \
                 sign, or a plus sign"
            ),
            line,
        });
    }
    if length < 2 && (first == '#' || first == '+') {
        return Err(NaturalError::UnknownStatement {
            name: format!(
                "'{name}' as an object name. A name starting with the first character \
                 '{first}' needs at least one more character"
            ),
            line,
        });
    }
    Ok(())
}

/// Records a loop's label so reference notation can resolve it later.
fn register_label(program: &mut Program, label: &Option<String>, key: usize) {
    if let Some(name) = label {
        program.labels.insert(name.clone(), key);
    }
}

/// Rejoins `*NUMBER ( EMP. )` into one word token. The lexer splits parentheses, so without
/// this every consumer of an operand would need to know about reference notation.
fn join_reference_notation(tokens: &[Token]) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut i = 0;
    while i < tokens.len() {
        if let Token::Word { text, line } = &tokens[i]
            && text.starts_with('*')
            && let Some([open, label, close]) = tokens.get(i + 1..i + 4)
            && matches!(open, Token::Word { text, .. } if text == "(")
            && matches!(close, Token::Word { text, .. } if text == ")")
            && let Token::Word { text: name, .. } = label
            && name.ends_with('.')
        {
            out.push(Token::Word {
                text: format!("{}({})", normalize(text), normalize(name)),
                line: *line,
            });
            i += 4;
            continue;
        }
        out.push(tokens[i].clone());
        i += 1;
    }
    out
}

fn program_len(program: &Program) -> usize {
    program.statements.len()
}

/// Fills in a jump target that was left unresolved when the instruction was emitted.
fn patch_target(program: &mut Program, index: usize, target: usize) {
    match &mut program.statements[index] {
        Statement::IfFalseJump { target: t, .. }
        | Statement::IfTrueJump { target: t, .. }
        | Statement::Jump { target: t }
        | Statement::ForInit { exit: t, .. }
        | Statement::ReadInit { exit: t, .. }
        | Statement::JumpIfNoRecords { target: t, .. }
        | Statement::HistogramInit { exit: t, .. } => *t = target,
        _ => unreachable!("only jump instructions are ever patched"),
    }
}

/// Resolves every ESCAPE collected for one loop once its boundaries are known.
fn patch_escapes(program: &mut Program, escapes: Vec<PendingEscape>, bottom: usize, top: usize) {
    for escape in escapes {
        let target = match escape.kind {
            EscapeKind::Bottom => bottom,
            EscapeKind::Top => top,
        };
        patch_target(program, escape.index, target);
    }
}

/// Pops the innermost block when it is of the expected kind, leaving the stack untouched
/// otherwise so the caller can report a mismatched closer.
fn pop_matching(
    blocks: &mut Vec<OpenBlock>,
    matches_kind: impl Fn(&OpenBlock) -> bool,
) -> Option<OpenBlock> {
    if blocks.last().is_some_and(matches_kind) {
        blocks.pop()
    } else {
        None
    }
}

fn block_mismatch(keyword: &str, opener: &str, line: usize) -> NaturalError {
    NaturalError::UnexpectedBlockKeyword {
        keyword: keyword.to_string(),
        hint: format!("It needs a matching {opener}, and inner blocks must be closed first."),
        line,
    }
}

/// Builds the top-of-loop guard for REPEAT UNTIL or REPEAT WHILE.
///
/// UNTIL leaves the loop when its condition becomes true; WHILE leaves when its condition
/// becomes false. They are the same jump with opposite senses.
fn condition_jump(guard: (Condition, bool), line: usize) -> Statement {
    let (condition, exit_when_true) = guard;
    if exit_when_true {
        Statement::IfTrueJump {
            condition,
            target: usize::MAX,
            line,
        }
    } else {
        Statement::IfFalseJump {
            condition,
            target: usize::MAX,
            line,
        }
    }
}

/// Parses the optional `UNTIL <condition>` or `WHILE <condition>` after REPEAT.
#[allow(clippy::type_complexity)]
fn parse_repeat_guard(
    tokens: &[Token],
    line: usize,
) -> Result<Option<(Condition, bool)>, NaturalError> {
    let Some(Token::Word { text, .. }) = tokens.first() else {
        if tokens.is_empty() {
            return Ok(None);
        }
        return Err(NaturalError::UnknownStatement {
            name: "REPEAT takes nothing, or UNTIL or WHILE followed by a condition".to_string(),
            line,
        });
    };

    let exit_when_true = match text.to_ascii_uppercase().as_str() {
        "UNTIL" => true,
        "WHILE" => false,
        _ => {
            return Err(NaturalError::UnknownStatement {
                name: "REPEAT takes nothing, or UNTIL or WHILE followed by a condition".to_string(),
                line,
            });
        }
    };
    Ok(Some((parse_condition(&tokens[1..], line)?, exit_when_true)))
}

/// True for `DEFINE MAP`.
/// Parses a map object's layout into its field model.
///
/// A map is a separate Natural object, and on a real system it is drawn in the map editor
/// rather than typed. It therefore has no authentic hand-written source to copy, so the
/// layout language below is this course's own and is disclosed as such in the lessons. What
/// it produces is the part that matters: a field model with positions, labels, bindings,
/// and attribute bytes, which is what a Natural programmer actually reasons about.
pub fn parse_map(source: &str) -> Result<Vec<MapElement>, NaturalError> {
    let tokens = lexer::tokenize(source)?;
    let mut elements = Vec::new();
    let mut current: Vec<Token> = Vec::new();

    let flush = |line: &mut Vec<Token>, elements: &mut Vec<MapElement>| {
        let taken = std::mem::take(line);
        let Some(Token::Word { text, line: at }) = taken.first() else {
            return Ok(());
        };
        parse_map_element(&normalize(text), &taken, *at, elements)
    };

    for token in tokens {
        if matches!(token, Token::Newline) {
            flush(&mut current, &mut elements)?;
        } else {
            current.push(token);
        }
    }
    flush(&mut current, &mut elements)?;
    Ok(elements)
}

fn is_map_definition(tokens: &[Token]) -> bool {
    matches!(tokens.get(1), Some(Token::Word { text, .. })
        if text.eq_ignore_ascii_case("MAP"))
}

/// True for `INPUT USING MAP <name>`.
fn is_map_input(tokens: &[Token]) -> bool {
    matches!(tokens.get(1), Some(Token::Word { text, .. })
        if text.eq_ignore_ascii_case("USING"))
}

/// Parses one line of a map's layout: `TEXT row col 'label'` or
/// `FIELD row col 'label' #VAR [(AD=x)]`.
fn parse_map_element(
    head: &str,
    tokens: &[Token],
    line: usize,
    elements: &mut Vec<MapElement>,
) -> Result<(), NaturalError> {
    let malformed = || NaturalError::UnknownStatement {
        name: "a map element. Write TEXT 2 5 'Title' or FIELD 5 5 'Name:' #NAME".to_string(),
        line,
    };
    if !matches!(head, "TEXT" | "FIELD") {
        return Err(malformed());
    }

    let words: Vec<Option<String>> = tokens.iter().map(keyword_at).collect();
    let number = |i: usize| {
        words
            .get(i)
            .and_then(|w| w.as_deref())
            .and_then(|w| w.parse::<usize>().ok())
    };
    let row = number(1).ok_or_else(malformed)?;
    let column = number(2).ok_or_else(malformed)?;

    let label = tokens.iter().find_map(|t| match t {
        Token::Text { value, .. } => Some(value.clone()),
        _ => None,
    });

    // The bound variable is the first word after the position that is not a number and
    // not part of an attribute clause.
    let mut bound_to = None;
    let mut attribute = None;
    let mut index = 3;
    while index < tokens.len() {
        if let Token::Word { text, .. } = &tokens[index] {
            if text == "(" {
                // (AD=I), (AD=N), and so on.
                if let Some(Token::Word { text: spec, .. }) = tokens.get(index + 1) {
                    attribute = attribute_from(spec);
                }
                index += 1;
                continue;
            }
            if text != ")" && bound_to.is_none() && text.parse::<usize>().is_err() {
                bound_to = Some(normalize(text));
            }
        }
        index += 1;
    }

    if head == "TEXT" {
        bound_to = None;
    }

    elements.push(MapElement {
        row,
        column,
        label,
        bound_to,
        attribute,
        line,
    });
    Ok(())
}

/// Reads an `AD=` attribute clause. `I` is intensified and `N` is non-display, following
/// the session parameter of the same name.
fn attribute_from(spec: &str) -> Option<Attribute> {
    let value = spec.to_ascii_uppercase();
    let value = value.strip_prefix("AD=").unwrap_or(&value);
    match value {
        "I" => Some(Attribute::Intensified),
        "N" => Some(Attribute::Hidden),
        "P" => Some(Attribute::Protected),
        _ => None,
    }
}

/// True for `DEFINE SUBROUTINE`, which is a statement rather than a data block opener.
fn is_subroutine_definition(tokens: &[Token]) -> bool {
    matches!(tokens.get(1), Some(Token::Word { text, .. })
        if text.eq_ignore_ascii_case("SUBROUTINE"))
}

fn subroutine_name(tokens: &[Token], line: usize) -> Result<String, NaturalError> {
    let Some(Token::Word { text, .. }) = tokens.get(2) else {
        return Err(NaturalError::UnknownStatement {
            name: "DEFINE SUBROUTINE without a name".to_string(),
            line,
        });
    };
    let name = normalize(text);
    // A subroutine name may run to 32 characters, unlike an object name, but it still
    // cannot be a word the compiler already means something by.
    if is_reserved_keyword(&name) {
        return Err(NaturalError::ReservedWordAsName {
            name,
            a_what: "a subroutine".to_string(),
            line,
        });
    }
    Ok(name)
}

/// True for `END TRANSACTION`, which is a statement rather than the program terminator.
fn is_transaction_boundary(tokens: &[Token]) -> bool {
    matches!(tokens.get(1), Some(Token::Word { text, .. })
        if text.eq_ignore_ascii_case("TRANSACTION"))
}

/// Parses `[(limit)] <view> FOR <descriptor>`.
#[allow(clippy::type_complexity)]
fn parse_histogram_header(
    tokens: &[Token],
    line: usize,
) -> Result<(String, String, Option<usize>), NaturalError> {
    let malformed = || NaturalError::UnknownStatement {
        name: "a HISTOGRAM. Write it as HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY".to_string(),
        line,
    };
    let words: Vec<Option<String>> = tokens.iter().map(keyword_at).collect();
    let word = |i: usize| words.get(i).and_then(|w| w.as_deref());
    let mut at = 0;

    let mut limit = None;
    if word(0) == Some("(") {
        limit = Some(
            word(1)
                .ok_or_else(malformed)?
                .parse::<usize>()
                .map_err(|_| malformed())?,
        );
        if word(2) != Some(")") {
            return Err(malformed());
        }
        at = 3;
    }

    let view = word(at).ok_or_else(malformed)?.to_string();
    at += 1;
    // FOR and IN both introduce the descriptor in the documented syntax.
    if !matches!(word(at), Some("FOR") | Some("IN")) {
        return Err(malformed());
    }
    let descriptor = word(at + 1).ok_or_else(malformed)?.to_string();
    Ok((view, descriptor, limit))
}

/// True for the `IF NO RECORDS FOUND` form, which is a FIND clause rather than an IF.
fn is_no_records_clause(tokens: &[Token]) -> bool {
    let words: Vec<String> = tokens.iter().filter_map(word_text).collect();
    words.len() >= 2 && words[1] == "NO"
}

struct FindHeader {
    view: String,
    with: Condition,
    filter: Option<Condition>,
    sorted_by: Option<String>,
    limit: Option<usize>,
}

/// Parses `[(limit)] <view> WITH <condition> [WHERE <condition>] [SORTED BY <field>]`.
fn parse_find_header(tokens: &[Token], line: usize) -> Result<FindHeader, NaturalError> {
    let malformed = || NaturalError::UnknownStatement {
        name: "a FIND. Write it as FIND EMPLOYEES-VIEW WITH NAME = 'JONES'".to_string(),
        line,
    };
    // Aligned with `tokens` position for position. A text literal has no keyword form, so
    // it is None rather than being dropped: filtering literals out would shift every index
    // and silently misread a condition such as WITH NAME = 'JONES'.
    let words: Vec<Option<String>> = tokens.iter().map(keyword_at).collect();
    let word = |i: usize| words.get(i).and_then(|w| w.as_deref());
    let mut at = 0;

    let mut limit = None;
    if word(0) == Some("(") {
        let count = word(1).ok_or_else(malformed)?;
        limit = Some(count.parse::<usize>().map_err(|_| malformed())?);
        if word(2) != Some(")") {
            return Err(malformed());
        }
        at = 3;
    }

    let view = word(at).ok_or_else(malformed)?.to_string();
    at += 1;

    if word(at) != Some("WITH") {
        return Err(malformed());
    }
    let with_start = at + 1;

    // The documented clause order is WITH, then SORTED BY, then WHERE. Enforcing it rather
    // than accepting either arrangement is deliberate: the course promises real Natural, so
    // a learner who writes it the other way should find out here instead of on a real system.
    let where_at = words.iter().position(|w| w.as_deref() == Some("WHERE"));
    let sorted_at = words.iter().position(|w| w.as_deref() == Some("SORTED"));

    if let (Some(w), Some(s)) = (where_at, sorted_at)
        && w < s
    {
        return Err(NaturalError::ClauseOutOfOrder {
            first: "WHERE".to_string(),
            second: "SORTED BY".to_string(),
            order: "WITH, then SORTED BY, then WHERE".to_string(),
            line,
        });
    }

    // The search condition runs up to whichever clause comes first, which after the order
    // check above is SORTED BY when both are present.
    let with_end = sorted_at.or(where_at).unwrap_or(tokens.len());
    let with = parse_condition(&tokens[with_start..with_end], line)?;

    let sorted_by = match sorted_at {
        Some(start) => {
            if word(start + 1) != Some("BY") {
                return Err(malformed());
            }
            Some(word(start + 2).ok_or_else(malformed)?.to_string())
        }
        None => None,
    };

    let filter = match where_at {
        Some(start) => Some(parse_condition(&tokens[start + 1..], line)?),
        None => None,
    };

    Ok(FindHeader {
        view,
        with,
        filter,
        sorted_by,
        limit,
    })
}

/// Parses `[(limit)] <view> [BY <descriptor>]`.
#[allow(clippy::type_complexity)]
fn parse_read_header(
    tokens: &[Token],
    line: usize,
) -> Result<(String, Option<String>, Option<usize>), NaturalError> {
    let malformed = || NaturalError::UnknownStatement {
        name: "a READ. Write it as READ EMPLOYEES-VIEW BY NAME".to_string(),
        line,
    };
    let words: Vec<String> = tokens.iter().filter_map(word_text).collect();
    let mut at = 0;

    // An optional record limit in parentheses, as in READ (3) EMPLOYEES-VIEW.
    let mut limit = None;
    if words.first().map(String::as_str) == Some("(") {
        let count = words.get(1).ok_or_else(malformed)?;
        limit = Some(count.parse::<usize>().map_err(|_| malformed())?);
        if words.get(2).map(String::as_str) != Some(")") {
            return Err(malformed());
        }
        at = 3;
    }

    let view = words.get(at).ok_or_else(malformed)?.clone();
    at += 1;

    // BY and IN LOGICAL SEQUENCE BY both introduce a descriptor; the teaching subset
    // accepts the short form.
    let by = match words.get(at).map(String::as_str) {
        Some("BY") => Some(words.get(at + 1).ok_or_else(malformed)?.clone()),
        None => None,
        Some(_) => return Err(malformed()),
    };

    Ok((view, by, limit))
}

/// Parses `#VAR = <from> TO <to>`, also accepting `:=` and `FROM` for the first part.
fn parse_for_header(
    tokens: &[Token],
    line: usize,
) -> Result<(String, Operand, Operand), NaturalError> {
    let malformed = || NaturalError::UnknownStatement {
        name: "a FOR header. Write it as FOR #I = 1 TO 10".to_string(),
        line,
    };

    if tokens.len() != 5 {
        return Err(malformed());
    }
    let var = require_name(&tokens[0], line)?;

    let Token::Word { text: assign, .. } = &tokens[1] else {
        return Err(malformed());
    };
    if !matches!(assign.to_ascii_uppercase().as_str(), "=" | ":=" | "FROM") {
        return Err(malformed());
    }

    let Token::Word { text: to_kw, .. } = &tokens[3] else {
        return Err(malformed());
    };
    if !to_kw.eq_ignore_ascii_case("TO") {
        return Err(malformed());
    }

    Ok((
        var,
        operand_from(&tokens[2], line)?,
        operand_from(&tokens[4], line)?,
    ))
}

/// Parses `<operand> <operator> <operand>` with an optional trailing THEN.
fn parse_condition(tokens: &[Token], line: usize) -> Result<Condition, NaturalError> {
    let mut tokens = tokens;
    if let Some(Token::Word { text, .. }) = tokens.last()
        && text.eq_ignore_ascii_case("THEN")
    {
        tokens = &tokens[..tokens.len() - 1];
    }

    let malformed = || NaturalError::UnknownStatement {
        name: "a condition. Write it as IF #FIELD > 10, with spaces around the operator"
            .to_string(),
        line,
    };

    if tokens.len() != 3 {
        return Err(malformed());
    }
    let Token::Word { text: op, .. } = &tokens[1] else {
        return Err(malformed());
    };
    let op = CompareOp::parse(op).ok_or_else(malformed)?;

    Ok(Condition {
        left: operand_from(&tokens[0], line)?,
        op,
        right: operand_from(&tokens[2], line)?,
    })
}

/// Parses the DECIDE header, returning the compared operand (for ON) and whether only the
/// first matching clause runs.
fn parse_decide_header(
    tokens: &[Token],
    line: usize,
) -> Result<(Option<Operand>, bool), NaturalError> {
    let malformed = || NaturalError::UnknownStatement {
        name: "a DECIDE header. Write DECIDE ON FIRST VALUE OF #FIELD, or \
               DECIDE FOR FIRST CONDITION"
            .to_string(),
        line,
    };
    let words: Vec<String> = tokens.iter().filter_map(word_text).collect();

    let first = match words.get(1).map(String::as_str) {
        Some("FIRST") => true,
        Some("EVERY") => false,
        _ => return Err(malformed()),
    };

    match words.first().map(String::as_str) {
        // DECIDE ON <FIRST|EVERY> VALUE [OF] <operand>
        Some("ON") => {
            if words.get(2).map(String::as_str) != Some("VALUE") {
                return Err(malformed());
            }
            let subject_at = if words.get(3).map(String::as_str) == Some("OF") {
                4
            } else {
                3
            };
            let token = tokens.get(subject_at).ok_or_else(malformed)?;
            Ok((Some(operand_from(token, line)?), first))
        }
        // DECIDE FOR <FIRST|EVERY> CONDITION
        Some("FOR") => {
            if words.get(2).map(String::as_str) != Some("CONDITION") {
                return Err(malformed());
            }
            Ok((None, first))
        }
        _ => Err(malformed()),
    }
}

/// Emits the tests for one VALUE, WHEN, or NONE clause and closes the previous clause.
///
/// Each clause compiles to a run of IfTrueJump tests that land on the clause body, followed
/// by a Jump to the next clause. The body start is known in advance because the number of
/// tests is known, which is what lets this stay a single forward pass.
fn open_decide_clause(
    head: &str,
    tokens: &[Token],
    line: usize,
    program: &mut Program,
    blocks: &mut [OpenBlock],
) -> Result<(), NaturalError> {
    let Some(OpenBlock::Decide {
        subject,
        first,
        end_jumps,
        pending_next,
        in_clause,
        has_none,
        ..
    }) = blocks.last_mut()
    else {
        return Err(NaturalError::UnexpectedBlockKeyword {
            keyword: head.to_string(),
            hint: "It belongs inside a DECIDE block.".to_string(),
            line,
        });
    };

    let is_value = head == "VALUE";
    let is_none = head == "NONE" || (head == "WHEN" && clause_is_none(tokens));
    if !is_none {
        // VALUE belongs to DECIDE ON and WHEN belongs to DECIDE FOR. Mixing them is a
        // frequent slip, so the diagnostic names the keyword that was expected.
        let expected_value = subject.is_some();
        if is_value != expected_value {
            return Err(NaturalError::UnexpectedBlockKeyword {
                keyword: head.to_string(),
                hint: if expected_value {
                    "A DECIDE ON block uses VALUE clauses.".to_string()
                } else {
                    "A DECIDE FOR block uses WHEN clauses.".to_string()
                },
                line,
            });
        }
    }

    // Close the previous clause. Under FIRST, a matched branch skips the rest.
    if *in_clause && *first {
        end_jumps.push(program.statements.len());
        program
            .statements
            .push(Statement::Jump { target: usize::MAX });
    }
    if let Some(previous) = pending_next.take() {
        patch_target(program, previous, program.statements.len());
    }
    *in_clause = true;

    if is_none {
        // A NONE clause runs unconditionally once control reaches it.
        *has_none = true;
        return Ok(());
    }

    let conditions: Vec<Condition> = if is_value {
        let subject = subject.clone().expect("VALUE implies DECIDE ON");
        split_value_list(&tokens[1..], line)?
            .into_iter()
            .map(|operand| Condition {
                left: subject.clone(),
                op: CompareOp::Eq,
                right: operand,
            })
            .collect()
    } else {
        vec![parse_condition(&tokens[1..], line)?]
    };

    let body_start = program.statements.len() + conditions.len() + 1;
    for condition in conditions {
        program.statements.push(Statement::IfTrueJump {
            condition,
            target: body_start,
            line,
        });
    }
    let pending = program.statements.len();
    program
        .statements
        .push(Statement::Jump { target: usize::MAX });

    if let Some(OpenBlock::Decide { pending_next, .. }) = blocks.last_mut() {
        *pending_next = Some(pending);
    }
    Ok(())
}

fn clause_is_none(tokens: &[Token]) -> bool {
    matches!(tokens.get(1), Some(Token::Word { text, .. }) if text.eq_ignore_ascii_case("NONE"))
}

/// Splits a `VALUE 1, 2, 3` list, tolerating commas attached to either side.
fn split_value_list(tokens: &[Token], line: usize) -> Result<Vec<Operand>, NaturalError> {
    let mut operands = Vec::new();
    for token in tokens {
        match token {
            Token::Word { text, line: l } => {
                let trimmed = text.trim_matches(',');
                if trimmed.is_empty() {
                    continue;
                }
                operands.push(operand_from(
                    &Token::Word {
                        text: trimmed.to_string(),
                        line: *l,
                    },
                    line,
                )?);
            }
            Token::Text { .. } => operands.push(operand_from(token, line)?),
            Token::Newline => {}
        }
    }
    if operands.is_empty() {
        return Err(NaturalError::UnknownStatement {
            name: "a VALUE clause with no value, as in VALUE 1".to_string(),
            line,
        });
    }
    Ok(operands)
}

/// Parses `[ROUNDED] #TARGET = <expression>`.
fn parse_compute(tokens: &[Token], line: usize) -> Result<Statement, NaturalError> {
    let malformed = || NaturalError::UnknownStatement {
        name: "a calculation. Write it as COMPUTE #TOTAL = #PRICE * #QTY".to_string(),
        line,
    };

    let mut tokens = tokens;
    let mut rounded = false;
    if let Some(Token::Word { text, .. }) = tokens.first()
        && text.eq_ignore_ascii_case("ROUNDED")
    {
        rounded = true;
        tokens = &tokens[1..];
    }

    if tokens.len() < 3 {
        return Err(malformed());
    }
    let target = require_name(&tokens[0], line)?;
    let Token::Word { text: eq, .. } = &tokens[1] else {
        return Err(malformed());
    };
    if !matches!(eq.as_str(), "=" | ":=") {
        return Err(malformed());
    }

    Ok(Statement::Compute {
        target,
        expr: parse_expr(&tokens[2..], line)?,
        rounded,
        line,
    })
}

/// Desugars ADD, SUBTRACT, MULTIPLY, and DIVIDE into a COMPUTE over the same target.
///
/// Each verb reads and writes one field, so `ADD 5 TO #N` is exactly `#N = #N + 5`. Note
/// the direction of DIVIDE: `DIVIDE 4 INTO #N` divides the TARGET by 4, not the other way.
fn parse_arithmetic_verb(
    verb: &str,
    tokens: &[Token],
    line: usize,
) -> Result<Statement, NaturalError> {
    let (keyword, op) = match verb {
        "ADD" => ("TO", ArithOp::Add),
        "SUBTRACT" => ("FROM", ArithOp::Sub),
        "MULTIPLY" => ("BY", ArithOp::Mul),
        _ => ("INTO", ArithOp::Div),
    };
    let shape = || NaturalError::UnknownStatement {
        name: match verb {
            "ADD" => "ADD. Write it as ADD 5 TO #TOTAL".to_string(),
            "SUBTRACT" => "SUBTRACT. Write it as SUBTRACT 5 FROM #TOTAL".to_string(),
            "MULTIPLY" => "MULTIPLY. Write it as MULTIPLY #TOTAL BY 2".to_string(),
            _ => "DIVIDE. Write it as DIVIDE 2 INTO #TOTAL".to_string(),
        },
        line,
    };

    let at = tokens
        .iter()
        .position(|t| matches!(t, Token::Word { text, .. } if text.eq_ignore_ascii_case(keyword)))
        .ok_or_else(shape)?;
    if at == 0 || at + 1 >= tokens.len() {
        return Err(shape());
    }

    // MULTIPLY and DIVIDE name the target on the side that is not the keyword's operand.
    let (target_token, amount) = match verb {
        "MULTIPLY" => (&tokens[0], &tokens[at + 1..]),
        _ if verb == "ADD" || verb == "SUBTRACT" || verb == "DIVIDE" => {
            (&tokens[at + 1], &tokens[..at])
        }
        _ => unreachable!("the verb set is closed"),
    };
    let target = require_name(target_token, line)?;
    if target.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        let hint = match verb {
            "MULTIPLY" => "MULTIPLY puts its answer in the FIRST operand, so write \
                           MULTIPLY #TOTAL BY 3."
                .to_string(),
            "DIVIDE" => "DIVIDE puts its answer in the operand after INTO, so write \
                         DIVIDE 3 INTO #TOTAL."
                .to_string(),
            other => format!(
                "{other} puts its answer in the operand after {keyword}, which has to be a \
                 field."
            ),
        };
        return Err(NaturalError::ConstantAsResultField {
            statement: verb.to_string(),
            value: target,
            hint,
            line,
        });
    }

    Ok(Statement::Compute {
        target: target.clone(),
        expr: Expr::Binary {
            left: Box::new(Expr::Value(Operand::Variable { name: target, line })),
            op,
            right: Box::new(parse_expr(amount, line)?),
        },
        rounded: false,
        line,
    })
}

/// Recursive-descent expression parser: sums of products, with parentheses.
fn parse_expr(tokens: &[Token], line: usize) -> Result<Expr, NaturalError> {
    let (expr, rest) = parse_sum(tokens, line)?;
    if !rest.is_empty() {
        return Err(NaturalError::UnknownStatement {
            name: "an expression with something left over. Put spaces around each operator"
                .to_string(),
            line,
        });
    }
    Ok(expr)
}

fn parse_sum(tokens: &[Token], line: usize) -> Result<(Expr, &[Token]), NaturalError> {
    let (mut left, mut rest) = parse_product(tokens, line)?;
    while let Some(Token::Word { text, .. }) = rest.first() {
        let op = match text.as_str() {
            "+" => ArithOp::Add,
            "-" => ArithOp::Sub,
            _ => break,
        };
        let (right, remainder) = parse_product(&rest[1..], line)?;
        left = Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
        rest = remainder;
    }
    Ok((left, rest))
}

fn parse_product(tokens: &[Token], line: usize) -> Result<(Expr, &[Token]), NaturalError> {
    let (mut left, mut rest) = parse_factor(tokens, line)?;
    while let Some(Token::Word { text, .. }) = rest.first() {
        let op = match text.as_str() {
            "*" => ArithOp::Mul,
            "/" => ArithOp::Div,
            _ => break,
        };
        let (right, remainder) = parse_factor(&rest[1..], line)?;
        left = Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
        rest = remainder;
    }
    Ok((left, rest))
}

fn parse_factor(tokens: &[Token], line: usize) -> Result<(Expr, &[Token]), NaturalError> {
    let incomplete = || NaturalError::UnknownStatement {
        name: "an incomplete expression".to_string(),
        line,
    };
    let Some(first) = tokens.first() else {
        return Err(incomplete());
    };

    if let Token::Word { text, .. } = first
        && text == "("
    {
        let (inner, rest) = parse_sum(&tokens[1..], line)?;
        match rest.first() {
            Some(Token::Word { text, .. }) if text == ")" => Ok((inner, &rest[1..])),
            _ => Err(NaturalError::UnknownStatement {
                name: "an expression with an unclosed parenthesis".to_string(),
                line,
            }),
        }
    } else {
        Ok((Expr::Value(operand_from(first, line)?), &tokens[1..]))
    }
}

fn parse_input(tokens: &[Token], line: usize) -> Result<Statement, NaturalError> {
    // INPUT ['prompt'] #FIELD [#FIELD ...]
    let mut prompt = None;
    let mut targets = Vec::new();

    for token in &tokens[1..] {
        match token {
            Token::Text { value, .. } => {
                if prompt.is_some() || !targets.is_empty() {
                    return Err(NaturalError::NotYetSupported {
                        feature: "more than one prompt in a single INPUT".to_string(),
                        line,
                    });
                }
                prompt = Some(value.clone());
            }
            Token::Word { text, line: l } => targets.push((normalize(text), *l)),
            Token::Newline => {}
        }
    }

    if targets.is_empty() {
        return Err(NaturalError::UnknownStatement {
            name: "INPUT without a field to read into, as in INPUT #NAME".to_string(),
            line,
        });
    }
    Ok(Statement::Input { prompt, targets })
}

fn operand_from(token: &Token, line: usize) -> Result<Operand, NaturalError> {
    match token {
        Token::Text { value, .. } => Ok(Operand::Literal(Value::Alpha(value.clone()))),
        Token::Word { text, line: l } => {
            // A number or TRUE/FALSE is a literal. Anything else naming something is a
            // field reference. Database fields come from a DDM and carry no `#`, so the
            // prefix cannot be what distinguishes them; an unknown name is reported at run
            // time as an undeclared field, which is the diagnostic a learner needs anyway.
            Ok(match Value::from_token(text) {
                Some(value) => Operand::Literal(value),
                None => Operand::Variable {
                    name: normalize(text),
                    line: *l,
                },
            })
        }
        Token::Newline => Err(NaturalError::UnknownStatement {
            name: "a missing value".to_string(),
            line,
        }),
    }
}

fn require_name(token: &Token, line: usize) -> Result<String, NaturalError> {
    match token {
        Token::Word { text, .. } => Ok(normalize(text)),
        _ => Err(NaturalError::UnknownStatement {
            name: "a field name was expected here".to_string(),
            line,
        }),
    }
}

/// The uppercase keyword form of a token, or None for anything that is not a bare word.
///
/// Unlike [`word_text`] used with `filter_map`, this preserves position, which matters
/// whenever a clause is located by index within a token slice that may contain literals.
fn keyword_at(token: &Token) -> Option<String> {
    match token {
        Token::Word { text, .. } => Some(text.to_ascii_uppercase()),
        _ => None,
    }
}

fn word_text(token: &Token) -> Option<String> {
    match token {
        Token::Word { text, .. } => Some(text.to_ascii_uppercase()),
        _ => None,
    }
}
