// ABOUTME: The Natural data model: field formats parsed from a DEFINE DATA specification,
// ABOUTME: and the runtime values those fields hold, with exact base-10 arithmetic.

use crate::error::NaturalError;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use std::str::FromStr;

/// The documented maximum number of digit positions for formats N and P, per the Natural
/// Programming Guide, "User-Defined Variables".
const MAX_NUMERIC_POSITIONS: u32 = 29;

/// A field's declared format and length.
///
/// The `nn.m` length form means `nn` positions BEFORE the decimal point and `m` positions
/// after, so `N7.2` occupies nine digit positions rather than seven. Verified against the
/// Natural Programming Guide, "User-Defined Variables".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
    /// Alphanumeric, `A` followed by a character count.
    Alpha { length: usize },
    /// Unpacked numeric, `N`.
    Numeric { int_digits: u32, decimals: u32 },
    /// Packed decimal, `P`. Storage differs from `N` on a real mainframe; the arithmetic
    /// a learner observes does not, so the teaching interpreter treats them alike.
    Packed { int_digits: u32, decimals: u32 },
    /// Binary integer, `I`, permitted in lengths 1, 2, and 4 only.
    Integer { bytes: u8 },
    /// Logical, `L`, holding true or false.
    Logical,
}

impl Format {
    /// The value a field holds before anything is assigned to it, and the value `RESET`
    /// returns it to.
    pub fn default_value(&self) -> Value {
        match self {
            Format::Alpha { .. } => Value::Alpha(String::new()),
            Format::Numeric { .. } | Format::Packed { .. } | Format::Integer { .. } => {
                Value::Number(Decimal::ZERO)
            }
            Format::Logical => Value::Logical(false),
        }
    }

    /// A learner-facing description used in diagnostics.
    pub fn describe(&self) -> String {
        match self {
            Format::Alpha { length } => format!("alphanumeric ({length} characters)"),
            Format::Numeric {
                int_digits,
                decimals,
            } => format!("numeric ({int_digits} digits, {decimals} decimal places)"),
            Format::Packed {
                int_digits,
                decimals,
            } => format!("packed numeric ({int_digits} digits, {decimals} decimal places)"),
            Format::Integer { bytes } => format!("an integer ({bytes} bytes)"),
            Format::Logical => "logical".to_string(),
        }
    }

    /// Parses a specification such as `A20`, `N7.2`, `P3.1`, `I4`, or `L`.
    pub fn parse(spec: &str, line: usize) -> Result<Format, NaturalError> {
        let spec = spec.trim();
        let invalid = |detail: String| NaturalError::InvalidFormat { detail, line };

        let mut chars = spec.chars();
        let letter = chars
            .next()
            .ok_or_else(|| invalid("a field needs a format, for example (A20).".to_string()))?
            .to_ascii_uppercase();
        let rest: String = chars.collect();

        match letter {
            'L' => {
                if rest.is_empty() {
                    Ok(Format::Logical)
                } else {
                    Err(invalid(
                        "format L holds true or false and takes no length.".to_string(),
                    ))
                }
            }
            'A' => {
                let length: usize = rest.parse().map_err(|_| {
                    invalid(format!(
                        "'{spec}' is not a valid alphanumeric format. Write a length, as in (A20)."
                    ))
                })?;
                if length == 0 {
                    return Err(invalid(
                        "an alphanumeric field needs at least one character.".to_string(),
                    ));
                }
                Ok(Format::Alpha { length })
            }
            'I' => {
                let bytes: u8 = rest.parse().map_err(|_| {
                    invalid(format!(
                        "'{spec}' is not a valid integer format. Use I1, I2, or I4."
                    ))
                })?;
                if !matches!(bytes, 1 | 2 | 4) {
                    return Err(invalid(format!(
                        "format I allows only lengths 1, 2, and 4, so '{spec}' is not valid."
                    )));
                }
                Ok(Format::Integer { bytes })
            }
            'N' | 'P' => {
                let (int_digits, decimals) = parse_numeric_length(&rest, spec, line)?;
                if int_digits == 0 && decimals == 0 {
                    return Err(invalid(format!(
                        "'{spec}' needs at least one digit position."
                    )));
                }
                if int_digits + decimals > MAX_NUMERIC_POSITIONS {
                    return Err(invalid(format!(
                        "'{spec}' asks for {} digit positions. Natural allows at most {MAX_NUMERIC_POSITIONS}.",
                        int_digits + decimals
                    )));
                }
                if letter == 'N' {
                    Ok(Format::Numeric {
                        int_digits,
                        decimals,
                    })
                } else {
                    Ok(Format::Packed {
                        int_digits,
                        decimals,
                    })
                }
            }
            other => Err(invalid(format!(
                "'{other}' is not a format this course knows. Use A, N, P, I, or L."
            ))),
        }
    }
}

fn parse_numeric_length(rest: &str, spec: &str, line: usize) -> Result<(u32, u32), NaturalError> {
    let invalid = |detail: String| NaturalError::InvalidFormat { detail, line };
    let bad = || {
        invalid(format!(
            "'{spec}' is not a valid numeric format. Write it as N5 or N7.2."
        ))
    };

    match rest.split_once('.') {
        None => {
            let int_digits: u32 = rest.parse().map_err(|_| bad())?;
            Ok((int_digits, 0))
        }
        Some((before, after)) => {
            let int_digits: u32 = before.parse().map_err(|_| bad())?;
            let decimals: u32 = after.parse().map_err(|_| bad())?;
            Ok((int_digits, decimals))
        }
    }
}

/// A value held by a field or produced by a literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Alpha(String),
    Number(Decimal),
    Logical(bool),
}

impl Value {
    /// A learner-facing description used in diagnostics.
    pub fn describe_kind(&self) -> &'static str {
        match self {
            Value::Alpha(_) => "text",
            Value::Number(_) => "a number",
            Value::Logical(_) => "true or false",
        }
    }

    /// Parses a source token that is not a quoted literal into a value, if it looks like one.
    pub fn from_token(token: &str) -> Option<Value> {
        match token.to_ascii_uppercase().as_str() {
            "TRUE" => Some(Value::Logical(true)),
            "FALSE" => Some(Value::Logical(false)),
            _ => Decimal::from_str(token).ok().map(Value::Number),
        }
    }
}

/// Coerces `value` into `format`, applying the field's declared length and scale.
///
/// Alphanumeric assignment truncates on the right. Numeric assignment truncates excess
/// decimal places toward zero and rejects a value too wide for the declared integer
/// positions, because silently losing the most significant digits would teach a lie.
pub fn coerce(
    value: Value,
    format: &Format,
    name: &str,
    line: usize,
) -> Result<Value, NaturalError> {
    let mismatch = |value: &Value| NaturalError::IncompatibleAssignment {
        name: name.to_string(),
        source_kind: value.describe_kind().to_string(),
        target_kind: format.describe(),
        line,
    };

    match format {
        Format::Alpha { length } => match value {
            Value::Alpha(text) => Ok(Value::Alpha(text.chars().take(*length).collect())),
            other => Err(mismatch(&other)),
        },
        Format::Logical => match value {
            Value::Logical(b) => Ok(Value::Logical(b)),
            other => Err(mismatch(&other)),
        },
        Format::Numeric {
            int_digits,
            decimals,
        }
        | Format::Packed {
            int_digits,
            decimals,
        } => match value {
            Value::Number(n) => {
                let scaled = n.round_dp_with_strategy(*decimals, RoundingStrategy::ToZero);
                check_int_digits(scaled, *int_digits, name, line)?;
                Ok(Value::Number(scaled))
            }
            other => Err(mismatch(&other)),
        },
        Format::Integer { bytes } => match value {
            Value::Number(n) => {
                let truncated = n.trunc();
                let limit = match bytes {
                    1 => Decimal::from(i8::MAX),
                    2 => Decimal::from(i16::MAX),
                    _ => Decimal::from(i32::MAX),
                };
                if truncated.abs() > limit {
                    return Err(NaturalError::NumericOverflow {
                        name: name.to_string(),
                        value: n.to_string(),
                        int_digits: limit.to_string().len() as u32,
                        line,
                    });
                }
                Ok(Value::Number(truncated))
            }
            other => Err(mismatch(&other)),
        },
    }
}

fn check_int_digits(
    value: Decimal,
    int_digits: u32,
    name: &str,
    line: usize,
) -> Result<(), NaturalError> {
    let whole = value.trunc().abs();
    // Ten raised to the digit count is the first value that no longer fits.
    let mut ceiling = Decimal::ONE;
    for _ in 0..int_digits {
        ceiling *= Decimal::TEN;
    }
    if whole >= ceiling {
        return Err(NaturalError::NumericOverflow {
            name: name.to_string(),
            value: value.to_string(),
            int_digits,
            line,
        });
    }
    Ok(())
}
