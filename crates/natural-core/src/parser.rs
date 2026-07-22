// ABOUTME: Builds a flat statement list from tokens. One source line is one statement
// ABOUTME: for milestone M-A, and the program must terminate with END.

use crate::error::NaturalError;
use crate::lexer::{self, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Write { operands: Vec<String> },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

pub fn parse(source: &str) -> Result<Program, NaturalError> {
    let tokens = lexer::tokenize(source)?;
    let mut program = Program::default();
    let mut current: Vec<Token> = Vec::new();

    for token in tokens {
        if matches!(token, Token::Newline) {
            if parse_line(&current, &mut program)? {
                return Ok(program);
            }
            current.clear();
        } else {
            current.push(token);
        }
    }

    // A final line with no trailing newline is still a statement.
    if parse_line(&current, &mut program)? {
        return Ok(program);
    }

    Err(NaturalError::MissingEnd)
}

/// Parses one source line. Returns Ok(true) when the line was END, which terminates
/// the program. A blank line is ignored.
fn parse_line(tokens: &[Token], program: &mut Program) -> Result<bool, NaturalError> {
    let Some(first) = tokens.first() else {
        return Ok(false);
    };

    let (keyword, line) = match first {
        Token::Word { text, line } => (text.to_ascii_uppercase(), *line),
        Token::Text { line, .. } => {
            return Err(NaturalError::UnknownStatement {
                name: "a text literal".to_string(),
                line: *line,
            });
        }
        Token::Newline => return Ok(false),
    };

    match keyword.as_str() {
        "END" => Ok(true),
        "WRITE" => {
            let mut operands = Vec::new();
            for token in &tokens[1..] {
                match token {
                    Token::Text { value, .. } => operands.push(value.clone()),
                    // Variables become valid operands in milestone M-B, together with
                    // the DEFINE DATA block that declares them.
                    Token::Word { text, line } => {
                        return Err(NaturalError::UnknownStatement {
                            name: text.clone(),
                            line: *line,
                        });
                    }
                    Token::Newline => {}
                }
            }
            program.statements.push(Statement::Write { operands });
            Ok(false)
        }
        _ => Err(NaturalError::UnknownStatement {
            name: keyword,
            line,
        }),
    }
}
