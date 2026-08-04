// ABOUTME: Milestone M-E acceptance tests for inline subroutines: DEFINE SUBROUTINE,
// ABOUTME: PERFORM, and the explicit call stack that keeps a suspension possible inside one.

use natural_core::{Decimal, NaturalError, Value, run, run_with_input};

fn program(declarations: &str, body: &str) -> String {
    format!("DEFINE DATA LOCAL\n{declarations}END-DEFINE\n{body}\nEND")
}

#[test]
fn perform_runs_a_subroutine_and_comes_back() {
    let out = run(&program(
        "1 #N (N5)\n",
        "\
WRITE 'before'
PERFORM SHOW-IT
WRITE 'after'
DEFINE SUBROUTINE SHOW-IT
WRITE 'inside'
END-SUBROUTINE",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["before", "inside", "after"]);
}

#[test]
fn a_subroutine_body_does_not_run_on_the_way_past() {
    // The definition is skipped during normal flow; only PERFORM enters it.
    let out = run(&program(
        "1 #N (N5)\n",
        "\
DEFINE SUBROUTINE NEVER
WRITE 'should not appear'
END-SUBROUTINE
WRITE 'only this'",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["only this"]);
}

#[test]
fn a_subroutine_may_be_defined_before_it_is_performed() {
    let out = run(&program(
        "1 #N (N5)\n",
        "\
DEFINE SUBROUTINE GREET
WRITE 'hello'
END-SUBROUTINE
PERFORM GREET",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["hello"]);
}

#[test]
fn a_subroutine_sees_and_changes_the_programs_fields() {
    // Inline subroutines share the program's data, which is exactly the distinction
    // module 13 draws against subprograms, whose data is passed as parameters.
    let out = run(&program(
        "1 #A (N5)\n1 #B (N5)\n1 #TOTAL (N7)\n",
        "\
MOVE 20 TO #A
MOVE 22 TO #B
PERFORM ADD-THEM
WRITE #TOTAL
DEFINE SUBROUTINE ADD-THEM
COMPUTE #TOTAL = #A + #B
END-SUBROUTINE",
    ))
    .expect("should run");
    assert_eq!(out.get("#TOTAL"), Some(&Value::Number(Decimal::from(42))));
}

#[test]
fn a_subroutine_can_perform_another_subroutine() {
    let out = run(&program(
        "1 #N (N5)\n",
        "\
PERFORM OUTER-ONE
DEFINE SUBROUTINE OUTER-ONE
WRITE 'outer in'
PERFORM INNER-ONE
WRITE 'outer out'
END-SUBROUTINE
DEFINE SUBROUTINE INNER-ONE
WRITE 'inner'
END-SUBROUTINE",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["outer in", "inner", "outer out"]);
}

#[test]
fn the_same_subroutine_can_be_performed_repeatedly() {
    let out = run(&program(
        "1 #I (I4)\n1 #N (N5)\n",
        "\
FOR #I = 1 TO 3
PERFORM BUMP
END-FOR
WRITE #N
DEFINE SUBROUTINE BUMP
ADD 1 TO #N
END-SUBROUTINE",
    ))
    .expect("should run");
    assert_eq!(out.get("#N"), Some(&Value::Number(Decimal::from(3))));
}

#[test]
fn a_subroutine_may_contain_a_loop_and_a_conditional() {
    let out = run(&program(
        "1 #I (I4)\n",
        "\
PERFORM COUNT-DOWN
DEFINE SUBROUTINE COUNT-DOWN
FOR #I = 1 TO 3
IF #I = 2
WRITE 'two'
ELSE
WRITE 'other'
END-IF
END-FOR
END-SUBROUTINE",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["other", "two", "other"]);
}

#[test]
fn a_subroutine_can_read_the_database() {
    let source = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 NAME
2 COUNTRY
END-DEFINE
PERFORM LIST-UK
DEFINE SUBROUTINE LIST-UK
FIND EMPLOYEES-VIEW WITH COUNTRY = 'UK' SORTED BY NAME
WRITE NAME
END-FIND
END-SUBROUTINE
END";
    let out = run(source).expect("should run");
    let names: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    assert_eq!(names, vec!["GARRET", "JONES"]);
}

// ---- the architectural case ----

#[test]
fn an_input_inside_a_subroutine_suspends_and_resumes() {
    // The reason the call stack lives in the interpreter rather than on the Rust call
    // stack: a suspension can occur several frames deep and must still resume correctly.
    let out = run_with_input(
        &program(
            "1 #NAME (A10)\n",
            "\
PERFORM ASK
WRITE 'back in the main program'
DEFINE SUBROUTINE ASK
WRITE 'in the subroutine'
INPUT 'Name?' #NAME
WRITE 'got' #NAME
END-SUBROUTINE",
        ),
        &["ADA"],
    )
    .expect("should run");
    assert_eq!(out.prompts, vec!["Name?"]);
    assert_eq!(
        out.lines,
        vec!["in the subroutine", "got ADA", "back in the main program"]
    );
}

#[test]
fn an_input_two_frames_deep_still_resumes() {
    let out = run_with_input(
        &program(
            "1 #NAME (A10)\n",
            "\
PERFORM OUTER-ONE
WRITE 'done'
DEFINE SUBROUTINE OUTER-ONE
PERFORM INNER-ONE
WRITE 'outer resumed'
END-SUBROUTINE
DEFINE SUBROUTINE INNER-ONE
INPUT 'Deep?' #NAME
WRITE 'inner got' #NAME
END-SUBROUTINE",
        ),
        &["YES"],
    )
    .expect("should run");
    assert_eq!(out.lines, vec!["inner got YES", "outer resumed", "done"]);
}

// ---- teaching errors ----

#[test]
fn performing_an_unknown_subroutine_is_a_teaching_error() {
    let err = run(&program("1 #N (N5)\n", "PERFORM NOWHERE")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownSubroutine { .. }),
        "expected UnknownSubroutine, got {err:?}"
    );
    assert!(
        err.to_string().contains("NOWHERE"),
        "message should name it, got: {err}"
    );
}

#[test]
fn an_unclosed_subroutine_is_a_teaching_error() {
    let err = run(&program("1 #N (N5)\n", "DEFINE SUBROUTINE OOPS\nWRITE 'x'"))
        .expect_err("should reject");
    assert!(
        matches!(err, NaturalError::MissingLoopEnd { .. }),
        "expected MissingLoopEnd, got {err:?}"
    );
}

#[test]
fn end_subroutine_without_a_definition_is_a_teaching_error() {
    let err = run(&program("1 #N (N5)\n", "END-SUBROUTINE")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnexpectedBlockKeyword { .. }),
        "expected UnexpectedBlockKeyword, got {err:?}"
    );
}

#[test]
fn two_subroutines_with_the_same_name_is_a_teaching_error() {
    let err = run(&program(
        "1 #N (N5)\n",
        "\
DEFINE SUBROUTINE SAME-NAME
WRITE 'a'
END-SUBROUTINE
DEFINE SUBROUTINE SAME-NAME
WRITE 'b'
END-SUBROUTINE",
    ))
    .expect_err("should reject");
    assert!(
        matches!(err, NaturalError::DuplicateSubroutine { .. }),
        "expected DuplicateSubroutine, got {err:?}"
    );
}

#[test]
fn endless_recursion_produces_a_friendly_error() {
    // A subroutine that performs itself would exhaust a real stack. The explicit stack has
    // a documented depth, so the learner gets a teaching error instead of a crash.
    let err = run(&program(
        "1 #N (N5)\n",
        "\
PERFORM LOOPY
DEFINE SUBROUTINE LOOPY
PERFORM LOOPY
END-SUBROUTINE",
    ))
    .expect_err("should reject");
    assert!(
        matches!(
            err,
            NaturalError::CallStackTooDeep { .. } | NaturalError::RunawayLoop { .. }
        ),
        "expected a depth or runaway error, got {err:?}"
    );
}
