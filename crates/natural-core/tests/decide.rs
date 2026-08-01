// ABOUTME: Milestone M-C acceptance tests for DECIDE ON and DECIDE FOR, the multi-way
// ABOUTME: branches a learner meets constantly in real Natural maintenance code.

use natural_core::{NaturalError, run, run_with_input};

fn program(body: &str) -> String {
    format!("DEFINE DATA LOCAL\n1 #N (N5)\n1 #S (A10)\nEND-DEFINE\n{body}\nEND")
}

// ---- DECIDE ON ----

#[test]
fn decide_on_runs_the_matching_value_branch() {
    let body = "\
MOVE 2 TO #N
DECIDE ON FIRST VALUE OF #N
VALUE 1
WRITE 'one'
VALUE 2
WRITE 'two'
VALUE 3
WRITE 'three'
END-DECIDE";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["two"]);
}

#[test]
fn decide_on_falls_through_to_none_when_nothing_matches() {
    let body = "\
MOVE 99 TO #N
DECIDE ON FIRST VALUE OF #N
VALUE 1
WRITE 'one'
NONE VALUE
WRITE 'no match'
END-DECIDE";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["no match"]);
}

#[test]
fn a_value_clause_can_list_several_values() {
    let body = "\
MOVE 3 TO #N
DECIDE ON FIRST VALUE OF #N
VALUE 1, 2
WRITE 'low'
VALUE 3, 4
WRITE 'high'
NONE VALUE
WRITE 'other'
END-DECIDE";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["high"]);
}

#[test]
fn first_value_stops_after_the_first_match() {
    let body = "\
MOVE 1 TO #N
DECIDE ON FIRST VALUE OF #N
VALUE 1
WRITE 'first'
VALUE 1
WRITE 'second'
END-DECIDE";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["first"]);
}

#[test]
fn every_value_runs_all_matching_branches() {
    let body = "\
MOVE 1 TO #N
DECIDE ON EVERY VALUE OF #N
VALUE 1
WRITE 'first'
VALUE 1
WRITE 'second'
END-DECIDE";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["first", "second"]);
}

#[test]
fn decide_on_works_with_text() {
    let body = "\
MOVE 'RED' TO #S
DECIDE ON FIRST VALUE OF #S
VALUE 'BLUE'
WRITE 'cool'
VALUE 'RED'
WRITE 'warm'
END-DECIDE";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["warm"]);
}

#[test]
fn execution_continues_after_end_decide() {
    let body = "\
MOVE 1 TO #N
DECIDE ON FIRST VALUE OF #N
VALUE 1
WRITE 'inside'
END-DECIDE
WRITE 'after'";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["inside", "after"]);
}

// ---- DECIDE FOR ----

#[test]
fn decide_for_runs_the_first_true_condition() {
    let body = "\
MOVE 50 TO #N
DECIDE FOR FIRST CONDITION
WHEN #N > 100
WRITE 'large'
WHEN #N > 10
WRITE 'medium'
WHEN #N > 0
WRITE 'small'
END-DECIDE";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["medium"]);
}

#[test]
fn decide_for_falls_through_to_when_none() {
    let body = "\
MOVE 0 TO #N
DECIDE FOR FIRST CONDITION
WHEN #N > 10
WRITE 'big'
WHEN NONE
WRITE 'nothing matched'
END-DECIDE";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["nothing matched"]);
}

#[test]
fn decide_for_every_condition_runs_all_true_branches() {
    let body = "\
MOVE 50 TO #N
DECIDE FOR EVERY CONDITION
WHEN #N > 10
WRITE 'over ten'
WHEN #N > 20
WRITE 'over twenty'
WHEN #N > 100
WRITE 'over a hundred'
END-DECIDE";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["over ten", "over twenty"]);
}

// ---- interaction with the rest of the language ----

#[test]
fn a_decide_branch_may_contain_a_loop_and_a_conditional() {
    let source = "\
DEFINE DATA LOCAL
1 #N (N5)
1 #I (I4)
END-DEFINE
MOVE 2 TO #N
DECIDE ON FIRST VALUE OF #N
VALUE 2
FOR #I = 1 TO 2
IF #I = 1
WRITE 'one'
ELSE
WRITE 'two'
END-IF
END-FOR
END-DECIDE
END";
    let out = run(source).expect("should run");
    assert_eq!(out.lines, vec!["one", "two"]);
}

#[test]
fn an_input_inside_a_decide_branch_suspends_correctly() {
    let body = "\
MOVE 1 TO #N
DECIDE ON FIRST VALUE OF #N
VALUE 1
INPUT 'Name?' #S
WRITE 'hello' #S
NONE VALUE
WRITE 'skipped'
END-DECIDE";
    let out = run_with_input(&program(body), &["GRACE"]).expect("should run");
    assert_eq!(out.prompts, vec!["Name?"]);
    assert_eq!(out.lines, vec!["hello GRACE"]);
}

// ---- teaching errors ----

#[test]
fn an_unclosed_decide_is_a_teaching_error() {
    let body = "\
DECIDE ON FIRST VALUE OF #N
VALUE 1
WRITE 'x'";
    let err = run(&program(body)).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::MissingLoopEnd { .. }),
        "expected MissingLoopEnd, got {err:?}"
    );
}

#[test]
fn a_value_clause_outside_a_decide_is_a_teaching_error() {
    let err = run(&program("VALUE 1\nWRITE 'x'")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnexpectedBlockKeyword { .. }),
        "expected UnexpectedBlockKeyword, got {err:?}"
    );
}

#[test]
fn a_when_clause_inside_a_decide_on_is_a_teaching_error() {
    // WHEN belongs to DECIDE FOR; VALUE belongs to DECIDE ON. Mixing them is a common
    // beginner slip and deserves a diagnostic that names the right keyword.
    let body = "\
DECIDE ON FIRST VALUE OF #N
WHEN #N > 1
WRITE 'x'
END-DECIDE";
    let err = run(&program(body)).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnexpectedBlockKeyword { .. }),
        "expected UnexpectedBlockKeyword, got {err:?}"
    );
}

#[test]
fn an_end_decide_without_a_decide_is_a_teaching_error() {
    let err = run(&program("END-DECIDE")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnexpectedBlockKeyword { .. }),
        "expected UnexpectedBlockKeyword, got {err:?}"
    );
}

#[test]
fn a_malformed_decide_header_is_a_teaching_error() {
    let err = run(&program("DECIDE ON #N\nVALUE 1\nEND-DECIDE")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}
