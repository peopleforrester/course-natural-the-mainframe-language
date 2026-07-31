// ABOUTME: Milestone M-C acceptance tests for IF, ELSE, and END-IF, including nesting and
// ABOUTME: the interaction between a suspended INPUT and a surrounding conditional block.

use natural_core::{NaturalError, run, run_with_input};

/// Wraps a body in a data block declaring one numeric and one alphanumeric field.
fn program(body: &str) -> String {
    format!("DEFINE DATA LOCAL\n1 #N (N5)\n1 #S (A10)\nEND-DEFINE\n{body}\nEND")
}

#[test]
fn a_true_condition_runs_the_then_branch() {
    let out = run(&program("MOVE 10 TO #N\nIF #N > 5\nWRITE 'bigger'\nEND-IF"))
        .expect("program should run");
    assert_eq!(out.lines, vec!["bigger"]);
}

#[test]
fn a_false_condition_skips_the_then_branch() {
    let out = run(&program("MOVE 1 TO #N\nIF #N > 5\nWRITE 'bigger'\nEND-IF"))
        .expect("program should run");
    assert!(
        out.lines.is_empty(),
        "expected no output, got {:?}",
        out.lines
    );
}

#[test]
fn a_false_condition_runs_the_else_branch() {
    let out = run(&program(
        "MOVE 1 TO #N\nIF #N > 5\nWRITE 'bigger'\nELSE\nWRITE 'smaller'\nEND-IF",
    ))
    .expect("program should run");
    assert_eq!(out.lines, vec!["smaller"]);
}

#[test]
fn a_true_condition_skips_the_else_branch() {
    let out = run(&program(
        "MOVE 10 TO #N\nIF #N > 5\nWRITE 'bigger'\nELSE\nWRITE 'smaller'\nEND-IF",
    ))
    .expect("program should run");
    assert_eq!(out.lines, vec!["bigger"]);
}

#[test]
fn execution_continues_after_the_block() {
    let out = run(&program(
        "MOVE 1 TO #N\nIF #N > 5\nWRITE 'skipped'\nEND-IF\nWRITE 'after'",
    ))
    .expect("program should run");
    assert_eq!(out.lines, vec!["after"]);
}

#[test]
fn the_optional_then_keyword_is_accepted() {
    let out = run(&program(
        "MOVE 10 TO #N\nIF #N > 5 THEN\nWRITE 'yes'\nEND-IF",
    ))
    .expect("program should run");
    assert_eq!(out.lines, vec!["yes"]);
}

#[test]
fn blocks_nest() {
    let body = "\
MOVE 10 TO #N
IF #N > 5
WRITE 'outer'
IF #N > 8
WRITE 'inner'
ELSE
WRITE 'not inner'
END-IF
WRITE 'still outer'
END-IF";
    let out = run(&program(body)).expect("program should run");
    assert_eq!(out.lines, vec!["outer", "inner", "still outer"]);
}

#[test]
fn a_nested_else_binds_to_its_own_if() {
    let body = "\
MOVE 10 TO #N
IF #N > 5
IF #N > 100
WRITE 'inner then'
ELSE
WRITE 'inner else'
END-IF
ELSE
WRITE 'outer else'
END-IF";
    let out = run(&program(body)).expect("program should run");
    assert_eq!(out.lines, vec!["inner else"]);
}

// ---- comparison operators ----

#[test]
fn every_comparison_operator_works() {
    let cases = [
        ("#N = 10", true),
        ("#N = 11", false),
        ("#N <> 11", true),
        ("#N <> 10", false),
        ("#N > 9", true),
        ("#N > 10", false),
        ("#N < 11", true),
        ("#N < 10", false),
        ("#N >= 10", true),
        ("#N >= 11", false),
        ("#N <= 10", true),
        ("#N <= 9", false),
    ];
    for (condition, expected) in cases {
        let out = run(&program(&format!(
            "MOVE 10 TO #N\nIF {condition}\nWRITE 'hit'\nEND-IF"
        )))
        .unwrap_or_else(|e| panic!("'{condition}' should run, got {e}"));
        assert_eq!(
            !out.lines.is_empty(),
            expected,
            "condition '{condition}' evaluated wrongly"
        );
    }
}

#[test]
fn the_mnemonic_operator_forms_work() {
    // Natural accepts EQ, NE, GT, LT, GE, and LE alongside the symbols.
    let cases = [
        ("#N EQ 10", true),
        ("#N NE 10", false),
        ("#N GT 9", true),
        ("#N LT 9", false),
        ("#N GE 10", true),
        ("#N LE 9", false),
    ];
    for (condition, expected) in cases {
        let out = run(&program(&format!(
            "MOVE 10 TO #N\nIF {condition}\nWRITE 'hit'\nEND-IF"
        )))
        .unwrap_or_else(|e| panic!("'{condition}' should run, got {e}"));
        assert_eq!(
            !out.lines.is_empty(),
            expected,
            "condition '{condition}' evaluated wrongly"
        );
    }
}

#[test]
fn two_fields_can_be_compared() {
    let source = "\
DEFINE DATA LOCAL
1 #A (N5)
1 #B (N5)
END-DEFINE
MOVE 7 TO #A
MOVE 7 TO #B
IF #A = #B
WRITE 'equal'
END-IF
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.lines, vec!["equal"]);
}

#[test]
fn alphanumeric_fields_compare_by_text() {
    let out = run(&program(
        "MOVE 'ADABAS' TO #S\nIF #S = 'ADABAS'\nWRITE 'match'\nEND-IF",
    ))
    .expect("program should run");
    assert_eq!(out.lines, vec!["match"]);
}

// ---- interaction with the suspension machinery ----

#[test]
fn an_input_inside_a_taken_branch_still_suspends_and_resumes() {
    // The architectural case: a jump target and a suspension point in the same program.
    let body = "\
MOVE 10 TO #N
IF #N > 5
INPUT 'Name?' #S
WRITE 'got' #S
END-IF";
    let out = run_with_input(&program(body), &["HOPPER"]).expect("program should run");
    assert_eq!(out.prompts, vec!["Name?"]);
    assert_eq!(out.lines, vec!["got HOPPER"]);
}

#[test]
fn an_input_inside_a_skipped_branch_never_asks() {
    let body = "\
MOVE 1 TO #N
IF #N > 5
INPUT 'Name?' #S
END-IF
WRITE 'done'";
    let out = run_with_input(&program(body), &[]).expect("program should run");
    assert!(out.prompts.is_empty(), "should not have prompted");
    assert_eq!(out.lines, vec!["done"]);
}

// ---- teaching errors ----

#[test]
fn an_unclosed_if_is_a_teaching_error() {
    let err = run(&program("IF #N > 5\nWRITE 'x'")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::MissingEndIf { .. }),
        "expected MissingEndIf, got {err:?}"
    );
}

#[test]
fn an_else_without_an_if_is_a_teaching_error() {
    let err = run(&program("ELSE\nWRITE 'x'")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnexpectedBlockKeyword { .. }),
        "expected UnexpectedBlockKeyword, got {err:?}"
    );
}

#[test]
fn an_end_if_without_an_if_is_a_teaching_error() {
    let err = run(&program("END-IF")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnexpectedBlockKeyword { .. }),
        "expected UnexpectedBlockKeyword, got {err:?}"
    );
}

#[test]
fn two_else_branches_for_one_if_is_a_teaching_error() {
    let err = run(&program("IF #N > 5\nELSE\nELSE\nEND-IF")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnexpectedBlockKeyword { .. }),
        "expected UnexpectedBlockKeyword, got {err:?}"
    );
}

#[test]
fn comparing_text_with_a_number_is_a_teaching_error() {
    let err = run(&program("IF #S = 5\nWRITE 'x'\nEND-IF")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::IncomparableValues { .. }),
        "expected IncomparableValues, got {err:?}"
    );
}

#[test]
fn a_malformed_condition_is_a_teaching_error() {
    let err = run(&program("IF #N\nWRITE 'x'\nEND-IF")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}
