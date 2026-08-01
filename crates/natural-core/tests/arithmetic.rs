// ABOUTME: Milestone M-B acceptance tests for COMPUTE and the arithmetic verbs, including
// ABOUTME: the verified truncate-by-default and ROUNDED semantics and exact decimal math.

use natural_core::{Decimal, NaturalError, Value, run};
use std::str::FromStr;

fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).expect("test literal should parse")
}

fn program(declarations: &str, body: &str) -> String {
    format!("DEFINE DATA LOCAL\n{declarations}END-DEFINE\n{body}\nEND")
}

// ---- COMPUTE ----

#[test]
fn compute_evaluates_an_expression() {
    let out = run(&program(
        "1 #PRICE (N7.2)\n1 #QTY (I4)\n1 #TOTAL (N9.2)\n",
        "MOVE 19.99 TO #PRICE\nMOVE 3 TO #QTY\nCOMPUTE #TOTAL = #PRICE * #QTY",
    ))
    .expect("should run");
    assert_eq!(out.get("#TOTAL"), Some(&Value::Number(dec("59.97"))));
}

#[test]
fn multiplication_binds_tighter_than_addition() {
    let out = run(&program("1 #R (N7.2)\n", "COMPUTE #R = 2 + 3 * 4")).expect("should run");
    assert_eq!(out.get("#R"), Some(&Value::Number(dec("14.00"))));
}

#[test]
fn parentheses_override_precedence() {
    let out = run(&program("1 #R (N7.2)\n", "COMPUTE #R = ( 2 + 3 ) * 4")).expect("should run");
    assert_eq!(out.get("#R"), Some(&Value::Number(dec("20.00"))));
}

#[test]
fn subtraction_and_division_work() {
    let out = run(&program("1 #R (N7.2)\n", "COMPUTE #R = 100 - 40 / 4")).expect("should run");
    assert_eq!(out.get("#R"), Some(&Value::Number(dec("90.00"))));
}

#[test]
fn assign_is_a_synonym_for_compute() {
    let out = run(&program("1 #R (N5)\n", "ASSIGN #R = 6 * 7")).expect("should run");
    assert_eq!(out.get("#R"), Some(&Value::Number(dec("42"))));
}

#[test]
fn the_assignment_operator_accepts_an_expression() {
    let out = run(&program("1 #R (N5)\n", "#R := 6 * 7")).expect("should run");
    assert_eq!(out.get("#R"), Some(&Value::Number(dec("42"))));
}

// ---- verified rounding semantics ----

#[test]
fn e1_assignment_truncates_excess_decimals_by_default() {
    // Verified row E1: #E (N1.3) := -0.12345 stores -0.123, truncated toward zero.
    let out = run(&program("1 #E (N1.3)\n", "COMPUTE #E = -0.12345")).expect("should run");
    assert_eq!(out.get("#E"), Some(&Value::Number(dec("-0.123"))));
}

#[test]
fn e2_the_rounded_option_rounds_instead() {
    // Verified row E2: ASSIGN ROUNDED #F (N5) = 199.999 stores 200.
    let out = run(&program("1 #F (N5)\n", "ASSIGN ROUNDED #F = 199.999")).expect("should run");
    assert_eq!(out.get("#F"), Some(&Value::Number(dec("200"))));
}

#[test]
fn e4_rounded_rounds_up_from_a_half() {
    // Verified row E4: ROUNDED rounds up when the first truncated decimal is 5 or more.
    let out = run(&program(
        "1 #A (N3.1)\n1 #B (N3.1)\n",
        "COMPUTE ROUNDED #A = 1.25\nCOMPUTE ROUNDED #B = 1.24",
    ))
    .expect("should run");
    assert_eq!(out.get("#A"), Some(&Value::Number(dec("1.3"))));
    assert_eq!(out.get("#B"), Some(&Value::Number(dec("1.2"))));
}

#[test]
fn arithmetic_is_exact_base_ten_not_floating_point() {
    // The whole reason this is a business language. In binary floating point
    // 0.1 + 0.2 is 0.30000000000000004; here it must be exactly 0.30.
    let out = run(&program("1 #R (N5.2)\n", "COMPUTE #R = 0.1 + 0.2")).expect("should run");
    assert_eq!(out.get("#R"), Some(&Value::Number(dec("0.30"))));
}

// ---- the arithmetic verbs ----

#[test]
fn add_to_accumulates_into_the_target() {
    let out = run(&program("1 #N (N5)\n", "MOVE 10 TO #N\nADD 5 TO #N")).expect("should run");
    assert_eq!(out.get("#N"), Some(&Value::Number(dec("15"))));
}

#[test]
fn subtract_from_reduces_the_target() {
    let out =
        run(&program("1 #N (N5)\n", "MOVE 10 TO #N\nSUBTRACT 4 FROM #N")).expect("should run");
    assert_eq!(out.get("#N"), Some(&Value::Number(dec("6"))));
}

#[test]
fn multiply_by_scales_the_target() {
    let out = run(&program("1 #N (N5)\n", "MOVE 10 TO #N\nMULTIPLY #N BY 3")).expect("should run");
    assert_eq!(out.get("#N"), Some(&Value::Number(dec("30"))));
}

#[test]
fn divide_into_divides_the_target() {
    let out =
        run(&program("1 #N (N5.2)\n", "MOVE 10 TO #N\nDIVIDE 4 INTO #N")).expect("should run");
    assert_eq!(out.get("#N"), Some(&Value::Number(dec("2.50"))));
}

#[test]
fn an_accumulator_in_a_loop_is_the_classic_pattern() {
    let out = run(&program(
        "1 #I (I4)\n1 #TOTAL (N7.2)\n",
        "FOR #I = 1 TO 4\nADD 2.5 TO #TOTAL\nEND-FOR\nWRITE #TOTAL",
    ))
    .expect("should run");
    assert_eq!(out.get("#TOTAL"), Some(&Value::Number(dec("10.00"))));
    // N7.2 prints in 11 positions.
    assert_eq!(out.lines, vec!["      10.00"]);
}

// ---- teaching errors ----

#[test]
fn dividing_by_zero_is_a_teaching_error() {
    let err = run(&program("1 #R (N5)\n", "COMPUTE #R = 10 / 0")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::DivisionByZero { .. }),
        "expected DivisionByZero, got {err:?}"
    );
}

#[test]
fn a_result_too_large_for_the_target_is_a_teaching_error() {
    let err =
        run(&program("1 #SMALL (N2)\n", "COMPUTE #SMALL = 50 * 50")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::NumericOverflow { .. }),
        "expected NumericOverflow, got {err:?}"
    );
}

#[test]
fn arithmetic_on_text_is_a_teaching_error() {
    let err = run(&program(
        "1 #S (A5)\n1 #R (N5)\n",
        "MOVE 'abc' TO #S\nCOMPUTE #R = #S + 1",
    ))
    .expect_err("should reject");
    assert!(
        matches!(err, NaturalError::NonNumericArithmetic { .. }),
        "expected NonNumericArithmetic, got {err:?}"
    );
}

#[test]
fn computing_into_an_undeclared_field_is_a_teaching_error() {
    let err = run(&program("1 #R (N5)\n", "COMPUTE #NOPE = 1 + 1")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UndeclaredVariable { .. }),
        "expected UndeclaredVariable, got {err:?}"
    );
}

#[test]
fn a_malformed_compute_is_a_teaching_error() {
    let err = run(&program("1 #R (N5)\n", "COMPUTE #R 1 + 1")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}

#[test]
fn an_unbalanced_parenthesis_is_a_teaching_error() {
    let err = run(&program("1 #R (N5)\n", "COMPUTE #R = ( 1 + 2")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}
