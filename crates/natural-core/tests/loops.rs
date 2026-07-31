// ABOUTME: Milestone M-C acceptance tests for FOR, REPEAT, and ESCAPE, including the
// ABOUTME: runaway-loop cap that keeps an unbounded loop from freezing a learner's browser.

use natural_core::{Interpreter, NaturalError, Step, parse_program, run, run_with_input};

fn program(body: &str) -> String {
    format!("DEFINE DATA LOCAL\n1 #I (I4)\n1 #N (N5)\n1 #S (A10)\nEND-DEFINE\n{body}\nEND")
}

// ---- FOR ----

#[test]
fn a_for_loop_runs_its_body_once_per_value() {
    let out = run(&program("FOR #I = 1 TO 3\nWRITE 'tick'\nEND-FOR")).expect("should run");
    assert_eq!(out.lines, vec!["tick", "tick", "tick"]);
}

#[test]
fn the_loop_variable_holds_the_current_value() {
    let out = run(&program("FOR #I = 1 TO 3\nWRITE #I\nEND-FOR")).expect("should run");
    // I4 prints in 11 positions: ten digits plus the sign position.
    assert_eq!(out.lines, vec!["          1", "          2", "          3"]);
}

#[test]
fn a_for_loop_whose_range_is_empty_never_runs() {
    let out = run(&program(
        "FOR #I = 5 TO 1\nWRITE 'never'\nEND-FOR\nWRITE 'after'",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["after"]);
}

#[test]
fn the_bound_may_come_from_a_field() {
    let out = run(&program(
        "MOVE 2 TO #N\nFOR #I = 1 TO #N\nWRITE 'x'\nEND-FOR",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["x", "x"]);
}

#[test]
fn the_assignment_form_of_the_bound_is_accepted() {
    let out = run(&program("FOR #I := 1 TO 2\nWRITE 'y'\nEND-FOR")).expect("should run");
    assert_eq!(out.lines, vec!["y", "y"]);
}

#[test]
fn for_loops_nest() {
    let source = "\
DEFINE DATA LOCAL
1 #I (I4)
1 #J (I4)
END-DEFINE
FOR #I = 1 TO 2
FOR #J = 1 TO 2
WRITE 'inner'
END-FOR
END-FOR
END";
    let out = run(source).expect("should run");
    assert_eq!(out.lines.len(), 4);
}

#[test]
fn execution_continues_after_a_for_loop() {
    let out = run(&program(
        "FOR #I = 1 TO 2\nWRITE 'in'\nEND-FOR\nWRITE 'out'",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["in", "in", "out"]);
}

// ---- REPEAT ----

#[test]
fn repeat_runs_until_an_escape() {
    let body = "\
MOVE 0 TO #N
REPEAT
MOVE 1 TO #N
ESCAPE BOTTOM
END-REPEAT
WRITE 'done'";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["done"]);
}

#[test]
fn repeat_until_stops_when_the_condition_becomes_true() {
    let body = "\
MOVE 0 TO #I
REPEAT UNTIL #I >= 3
MOVE 1 TO #N
WRITE 'loop'
FOR #I = 1 TO 3
WRITE 'inner'
END-FOR
END-REPEAT";
    let out = run(&program(body)).expect("should run");
    // The inner FOR leaves #I at 3, so the outer REPEAT stops after one pass.
    assert_eq!(out.lines, vec!["loop", "inner", "inner", "inner"]);
}

#[test]
fn repeat_while_runs_only_while_the_condition_holds() {
    let body = "\
MOVE 5 TO #N
REPEAT WHILE #N < 5
WRITE 'never'
END-REPEAT
WRITE 'after'";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["after"]);
}

// ---- ESCAPE ----

#[test]
fn escape_bottom_leaves_a_for_loop_early() {
    let body = "\
FOR #I = 1 TO 10
WRITE 'once'
ESCAPE BOTTOM
END-FOR
WRITE 'after'";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["once", "after"]);
}

#[test]
fn escape_top_starts_the_next_iteration() {
    let body = "\
FOR #I = 1 TO 3
IF #I = 2
ESCAPE TOP
END-IF
WRITE 'kept'
END-FOR";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["kept", "kept"]);
}

#[test]
fn escape_from_inside_a_conditional_still_finds_its_loop() {
    let body = "\
FOR #I = 1 TO 10
IF #I > 2
ESCAPE BOTTOM
END-IF
WRITE 'run'
END-FOR";
    let out = run(&program(body)).expect("should run");
    assert_eq!(out.lines, vec!["run", "run"]);
}

// ---- the runaway cap: a product requirement, not an implementation detail ----

#[test]
fn an_unbounded_loop_produces_a_friendly_teaching_error() {
    let source = program("REPEAT\nMOVE 1 TO #N\nEND-REPEAT");
    let program_ir = parse_program(&source).expect("should parse");
    let mut interp = Interpreter::new(program_ir).with_step_limit(5_000);

    let err = loop {
        match interp.step() {
            Ok(Step::Done) => panic!("an unbounded loop should never finish"),
            Ok(_) => continue,
            Err(e) => break e,
        }
    };

    assert!(
        matches!(err, NaturalError::RunawayLoop { .. }),
        "expected RunawayLoop, got {err:?}"
    );
    let message = err.to_string();
    assert!(
        message.contains("ESCAPE") || message.contains("loop"),
        "the error should teach how to fix it, got: {message}"
    );
}

#[test]
fn the_cap_is_on_by_default() {
    // A lesson author should not have to opt in to the protection.
    let source = program("REPEAT\nMOVE 1 TO #N\nEND-REPEAT");
    let interp = Interpreter::new(parse_program(&source).expect("should parse"));
    assert!(interp.step_limit() > 0, "a default step limit must be set");
}

#[test]
fn a_loop_that_finishes_normally_is_unaffected() {
    let out = run(&program(
        "FOR #I = 1 TO 50\nMOVE 1 TO #N\nEND-FOR\nWRITE 'fine'",
    ))
    .expect("a bounded loop should not trip the cap");
    assert_eq!(out.lines, vec!["fine"]);
}

// ---- interaction with the suspension machinery ----

#[test]
fn an_input_inside_a_loop_suspends_on_every_iteration() {
    let body = "\
FOR #I = 1 TO 2
INPUT 'Value?' #S
WRITE 'got' #S
END-FOR";
    let out = run_with_input(&program(body), &["one", "two"]).expect("should run");
    assert_eq!(out.prompts, vec!["Value?", "Value?"]);
    assert_eq!(out.lines, vec!["got one", "got two"]);
}

// ---- teaching errors ----

#[test]
fn an_unclosed_for_is_a_teaching_error() {
    let err = run(&program("FOR #I = 1 TO 3\nWRITE 'x'")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::MissingLoopEnd { .. }),
        "expected MissingLoopEnd, got {err:?}"
    );
}

#[test]
fn an_unclosed_repeat_is_a_teaching_error() {
    let err = run(&program("REPEAT\nWRITE 'x'")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::MissingLoopEnd { .. }),
        "expected MissingLoopEnd, got {err:?}"
    );
}

#[test]
fn an_escape_outside_any_loop_is_a_teaching_error() {
    let err = run(&program("ESCAPE BOTTOM")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::EscapeOutsideLoop { .. }),
        "expected EscapeOutsideLoop, got {err:?}"
    );
}

#[test]
fn an_escape_without_a_direction_is_a_teaching_error() {
    let err = run(&program("FOR #I = 1 TO 3\nESCAPE\nEND-FOR")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}

#[test]
fn closing_a_for_with_end_repeat_is_a_teaching_error() {
    let err = run(&program("FOR #I = 1 TO 3\nWRITE 'x'\nEND-REPEAT")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnexpectedBlockKeyword { .. }),
        "expected UnexpectedBlockKeyword, got {err:?}"
    );
}

#[test]
fn a_malformed_for_header_is_a_teaching_error() {
    let err = run(&program("FOR #I 1 TO 3\nEND-FOR")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}
