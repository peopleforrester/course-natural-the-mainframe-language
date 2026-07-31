// ABOUTME: Builds declarations and a flat statement list from tokens. One source line is
// ABOUTME: one statement, the optional DEFINE DATA block comes first, and END terminates.

use crate::error::NaturalError;
use crate::lexer::{self, Token};
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
    /// Unconditional jump, used to skip an ELSE branch.
    Jump {
        target: usize,
    },
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

/// A block whose jump target is not known until its closing keyword is reached.
enum OpenBlock {
    /// An IF with no ELSE yet. Holds the index of its IfFalseJump.
    If { false_jump: usize, line: usize },
    /// An IF whose ELSE has been seen. Holds the index of the Jump that skips the ELSE.
    Else { end_jump: usize, line: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    pub name: String,
    pub format: Format,
    pub line: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
    pub statements: Vec<Statement>,
}

/// Natural identifiers are not case sensitive, so every name is stored folded.
pub fn normalize(name: &str) -> String {
    name.to_ascii_uppercase()
}

#[derive(PartialEq)]
enum Mode {
    /// Before any executable statement, where a DEFINE DATA block may still open.
    Preamble,
    /// Inside the DEFINE DATA block, reading field declarations.
    DataBlock,
    /// After the data block, reading executable statements.
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
        let line = match open {
            OpenBlock::If { line, .. } | OpenBlock::Else { line, .. } => *line,
        };
        return Err(NaturalError::MissingEndIf { line });
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

    if *mode == Mode::DataBlock {
        if head == "END-DEFINE" {
            *mode = Mode::Body;
            return Ok(false);
        }
        // Reaching the end of the program while still inside the block means END-DEFINE
        // was forgotten. Say that, rather than complaining about a missing level number.
        if head == "END" {
            return Err(NaturalError::MissingEndDefine);
        }
        parse_declaration(tokens, line, program)?;
        return Ok(false);
    }

    if head == "DEFINE" {
        if *mode != Mode::Preamble {
            return Err(NaturalError::DefineDataNotFirst { line });
        }
        parse_define_data_header(tokens, line)?;
        *mode = Mode::DataBlock;
        return Ok(false);
    }

    // Any other statement closes the window in which DEFINE DATA may appear.
    *mode = Mode::Body;

    match head.as_str() {
        "END" => Ok(true),
        "END-DEFINE" => Err(NaturalError::UnknownStatement {
            name: "END-DEFINE without a DEFINE DATA block".to_string(),
            line,
        }),
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
        "INPUT" => {
            program.statements.push(parse_input(tokens, line)?);
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
            }) = blocks.pop()
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
        "END-IF" => {
            let Some(open) = blocks.pop() else {
                return Err(NaturalError::UnexpectedBlockKeyword {
                    keyword: "END-IF".to_string(),
                    hint: "An END-IF needs a matching IF above it.".to_string(),
                    line,
                });
            };
            let here = program_len(program);
            match open {
                OpenBlock::If { false_jump, .. } => patch_target(program, false_jump, here),
                OpenBlock::Else { end_jump, .. } => patch_target(program, end_jump, here),
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

fn parse_define_data_header(tokens: &[Token], line: usize) -> Result<(), NaturalError> {
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
        None | Some("LOCAL") => Ok(()),
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
) -> Result<(), NaturalError> {
    let words: Vec<String> = tokens.iter().filter_map(word_text).collect();

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

    program
        .declarations
        .push(Declaration { name, format, line });
    Ok(())
}

fn parse_write(tokens: &[Token], line: usize) -> Result<Statement, NaturalError> {
    let mut items = Vec::new();
    for token in &tokens[1..] {
        match token {
            Token::Text { value, .. } => items.push(WriteItem::Literal(value.clone())),
            Token::Word { text, line } if text.starts_with('#') => items.push(WriteItem::Field {
                name: normalize(text),
                line: *line,
            }),
            Token::Word { text, line } => {
                return Err(NaturalError::NotYetSupported {
                    feature: format!("writing '{text}'"),
                    line: *line,
                });
            }
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
    // <target> := <source>
    if tokens.len() != 3 {
        return Err(NaturalError::UnknownStatement {
            name: "an assignment takes one value, as in #FIELD := 3".to_string(),
            line,
        });
    }
    let target = require_name(&tokens[0], line)?;
    let source = operand_from(&tokens[2], line)?;
    Ok(Statement::Move {
        source,
        target,
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

fn program_len(program: &Program) -> usize {
    program.statements.len()
}

/// Fills in a jump target that was left unresolved when the instruction was emitted.
fn patch_target(program: &mut Program, index: usize, target: usize) {
    match &mut program.statements[index] {
        Statement::IfFalseJump { target: t, .. } | Statement::Jump { target: t } => *t = target,
        _ => unreachable!("only jump instructions are ever patched"),
    }
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
            if text.starts_with('#') {
                return Ok(Operand::Variable {
                    name: normalize(text),
                    line: *l,
                });
            }
            Value::from_token(text)
                .map(Operand::Literal)
                .ok_or_else(|| NaturalError::UnknownStatement {
                    name: format!("'{text}' is not a value this course understands"),
                    line: *l,
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

fn word_text(token: &Token) -> Option<String> {
    match token {
        Token::Word { text, .. } => Some(text.to_ascii_uppercase()),
        _ => None,
    }
}
