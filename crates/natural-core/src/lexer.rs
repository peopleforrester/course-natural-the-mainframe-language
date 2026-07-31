// ABOUTME: Turns Natural source text into words, quoted text literals, and line breaks,
// ABOUTME: tracking 1-based line numbers so diagnostics can point at the learner's line.

use crate::error::NaturalError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A bare word: a statement keyword, or later an identifier or number.
    Word { text: String, line: usize },
    /// The contents of a quoted literal, with doubled quotes already unescaped.
    Text { value: String, line: usize },
    /// A statement boundary.
    Newline,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, NaturalError> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut line = 1;

    while i < chars.len() {
        let c = chars[i];

        if c == '\n' {
            tokens.push(Token::Newline);
            line += 1;
            i += 1;
        } else if c.is_whitespace() {
            // Covers spaces, tabs, and the carriage return of a CRLF file.
            i += 1;
        } else if c == '\'' {
            let (value, next) = read_text_literal(&chars, i, line)?;
            tokens.push(Token::Text { value, line });
            i = next;
        } else if c == '(' || c == ')' {
            // Parentheses delimit a format specification, so they are their own tokens
            // whether or not the source puts a space around them.
            tokens.push(Token::Word {
                text: c.to_string(),
                line,
            });
            i += 1;
        } else {
            let start = i;
            while i < chars.len()
                && !chars[i].is_whitespace()
                && chars[i] != '\''
                && chars[i] != '\n'
                && chars[i] != '('
                && chars[i] != ')'
            {
                i += 1;
            }
            tokens.push(Token::Word {
                text: chars[start..i].iter().collect(),
                line,
            });
        }
    }

    Ok(tokens)
}

/// Reads a `'...'` literal starting at the opening quote. A quote inside the text is
/// written by doubling it, which is the Natural convention. Returns the unescaped value
/// and the index just past the closing quote.
fn read_text_literal(
    chars: &[char],
    open: usize,
    line: usize,
) -> Result<(String, usize), NaturalError> {
    let mut value = String::new();
    let mut i = open + 1;

    loop {
        if i >= chars.len() || chars[i] == '\n' {
            return Err(NaturalError::UnterminatedString { line });
        }
        if chars[i] == '\'' {
            if chars.get(i + 1) == Some(&'\'') {
                value.push('\'');
                i += 2;
                continue;
            }
            return Ok((value, i + 1));
        }
        value.push(chars[i]);
        i += 1;
    }
}
