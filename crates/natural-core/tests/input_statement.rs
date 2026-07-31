// ABOUTME: Milestone M-C acceptance tests for INPUT: the interpreter suspends mid-program,
// ABOUTME: hands control to the caller, and resumes with the supplied value.

use natural_core::{Decimal, NaturalError, Step, Value, run, run_with_input};
use std::str::FromStr;

fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).expect("test literal should parse")
}

const NAME_PROGRAM: &str = "\
DEFINE DATA LOCAL
1 #NAME (A20)
END-DEFINE
INPUT 'What is your name?' #NAME
WRITE 'Hello' #NAME
END";

#[test]
fn a_program_that_needs_input_suspends_and_resumes() {
    let out = run_with_input(NAME_PROGRAM, &["ADABAS"]).expect("program should run");
    assert_eq!(out.lines, vec!["Hello ADABAS"]);
    assert_eq!(out.get("#NAME"), Some(&Value::Alpha("ADABAS".to_string())));
}

#[test]
fn the_literal_becomes_the_prompt() {
    let out = run_with_input(NAME_PROGRAM, &["x"]).expect("program should run");
    assert_eq!(out.prompts, vec!["What is your name?"]);
}

#[test]
fn without_a_literal_the_field_name_is_the_prompt() {
    let source = "\
DEFINE DATA LOCAL
1 #NAME (A20)
END-DEFINE
INPUT #NAME
END";
    let out = run_with_input(source, &["x"]).expect("program should run");
    assert_eq!(out.prompts, vec!["#NAME"]);
}

#[test]
fn one_input_statement_can_read_several_fields_in_turn() {
    let source = "\
DEFINE DATA LOCAL
1 #FIRST (A10)
1 #LAST (A10)
END-DEFINE
INPUT #FIRST #LAST
WRITE #FIRST #LAST
END";
    let out = run_with_input(source, &["GRACE", "HOPPER"]).expect("program should run");
    assert_eq!(out.prompts, vec!["#FIRST", "#LAST"]);
    assert_eq!(out.get("#LAST"), Some(&Value::Alpha("HOPPER".to_string())));
}

#[test]
fn input_converts_text_to_the_declared_numeric_format() {
    let source = "\
DEFINE DATA LOCAL
1 #QTY (N5.2)
END-DEFINE
INPUT 'How many?' #QTY
END";
    let out = run_with_input(source, &["12.50"]).expect("program should run");
    assert_eq!(out.get("#QTY"), Some(&Value::Number(dec("12.50"))));
}

#[test]
fn input_applies_the_declared_length_to_alphanumeric_fields() {
    let source = "\
DEFINE DATA LOCAL
1 #SHORT (A5)
END-DEFINE
INPUT #SHORT
END";
    let out = run_with_input(source, &["TRUNCATED"]).expect("program should run");
    assert_eq!(out.get("#SHORT"), Some(&Value::Alpha("TRUNC".to_string())));
}

#[test]
fn input_accepts_a_logical_value() {
    let source = "\
DEFINE DATA LOCAL
1 #FLAG (L)
END-DEFINE
INPUT #FLAG
END";
    let out = run_with_input(source, &["TRUE"]).expect("program should run");
    assert_eq!(out.get("#FLAG"), Some(&Value::Logical(true)));
}

#[test]
fn statements_after_input_run_with_the_supplied_value() {
    // The point of the test: execution genuinely continues past the suspension point, and
    // the value assigned during the pause is visible to later statements.
    let source = "\
DEFINE DATA LOCAL
1 #N (N3)
1 #DOUBLE (N4)
END-DEFINE
INPUT #N
MOVE #N TO #DOUBLE
WRITE #DOUBLE
END";
    let out = run_with_input(source, &["21"]).expect("program should run");
    assert_eq!(out.get("#DOUBLE"), Some(&Value::Number(dec("21"))));
}

#[test]
fn output_before_and_after_a_suspension_is_ordered_correctly() {
    let source = "\
DEFINE DATA LOCAL
1 #N (A5)
END-DEFINE
WRITE 'before'
INPUT #N
WRITE 'after'
END";
    let out = run_with_input(source, &["x"]).expect("program should run");
    assert_eq!(out.lines, vec!["before", "after"]);
}

// ---- driving the state machine directly, as the browser will ----

#[test]
fn the_caller_can_drive_the_suspension_by_hand() {
    use natural_core::{Interpreter, parse_program};

    let mut interp = Interpreter::new(parse_program(NAME_PROGRAM).expect("should parse"));

    match interp.step().expect("should suspend") {
        Step::NeedsInput(request) => {
            assert_eq!(request.prompt, "What is your name?");
            assert_eq!(request.field, "#NAME");
        }
        other => panic!("expected NeedsInput, got {other:?}"),
    }

    // Stepping again without supplying a value must ask again rather than skipping ahead.
    assert!(matches!(
        interp.step().expect("should still suspend"),
        Step::NeedsInput(_)
    ));

    interp
        .provide_input("NATURAL")
        .expect("should accept input");

    assert_eq!(
        interp.step().expect("should resume"),
        Step::Output("Hello NATURAL".to_string())
    );
    assert_eq!(interp.step().expect("should finish"), Step::Done);
}

// ---- teaching errors ----

#[test]
fn text_that_is_not_a_number_is_a_teaching_error() {
    let source = "\
DEFINE DATA LOCAL
1 #QTY (N5)
END-DEFINE
INPUT #QTY
END";
    let err = run_with_input(source, &["not a number"]).expect_err("should reject the value");
    assert!(
        matches!(err, NaturalError::InvalidInput { .. }),
        "expected InvalidInput, got {err:?}"
    );
    assert!(
        err.to_string().contains("#QTY"),
        "message should name the field, got: {err}"
    );
}

#[test]
fn a_value_too_large_for_the_field_is_still_checked_on_input() {
    let source = "\
DEFINE DATA LOCAL
1 #SMALL (N2)
END-DEFINE
INPUT #SMALL
END";
    let err = run_with_input(source, &["12345"]).expect_err("should reject the value");
    assert!(
        matches!(err, NaturalError::NumericOverflow { .. }),
        "expected NumericOverflow, got {err:?}"
    );
}

#[test]
fn input_into_an_undeclared_field_is_a_teaching_error() {
    let err = run_with_input("INPUT #NOPE\nEND", &["x"]).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UndeclaredVariable { .. }),
        "expected UndeclaredVariable, got {err:?}"
    );
}

#[test]
fn running_a_program_that_needs_input_without_supplying_any_is_a_clear_error() {
    let err = run(NAME_PROGRAM).expect_err("should report that input is required");
    assert!(
        matches!(err, NaturalError::InputRequired { .. }),
        "expected InputRequired, got {err:?}"
    );
}

#[test]
fn supplying_input_when_none_was_asked_for_is_an_error() {
    use natural_core::{Interpreter, parse_program};
    let mut interp = Interpreter::new(parse_program("WRITE 'x'\nEND").expect("should parse"));
    let err = interp
        .provide_input("unexpected")
        .expect_err("should reject input that was never requested");
    assert!(
        matches!(err, NaturalError::NotWaitingForInput),
        "expected NotWaitingForInput, got {err:?}"
    );
}

#[test]
fn input_with_no_field_is_a_teaching_error() {
    let err = run_with_input("INPUT\nEND", &[]).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}
