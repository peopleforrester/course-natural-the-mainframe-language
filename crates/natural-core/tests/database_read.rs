// ABOUTME: Milestone M-D acceptance tests for VIEW OF and the READ loop over the
// ABOUTME: EMPLOYEES sample data, the statements that make this recognisably Natural.

use natural_core::{NaturalError, Value, run, run_with_input};

const VIEW: &str = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 NAME
2 FIRST-NAME
2 CITY
2 SALARY
END-DEFINE
";

fn program(body: &str) -> String {
    format!("{VIEW}{body}\nEND")
}

#[test]
fn read_visits_every_record() {
    let out = run(&program("READ EMPLOYEES-VIEW\nWRITE NAME\nEND-READ")).expect("should run");
    // The fixture holds eight employees.
    assert_eq!(out.lines.len(), 8);
}

#[test]
fn a_view_field_holds_the_current_record_value() {
    let out = run(&program(
        "READ (1) EMPLOYEES-VIEW\nWRITE NAME FIRST-NAME\nEND-READ",
    ))
    .expect("should run");
    // A20 fields, so each is padded to 20 and the line end is trimmed.
    assert_eq!(out.lines, vec![format!("GARRET{}MARY", " ".repeat(15))]);
}

#[test]
fn read_by_a_descriptor_sorts_ascending() {
    let out = run(&program(
        "READ EMPLOYEES-VIEW BY NAME\nWRITE NAME\nEND-READ",
    ))
    .expect("should run");
    let names: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    assert_eq!(names[0], "ABELLAN");
    assert_eq!(names[1], "ADAM");
    assert_eq!(names[names.len() - 1], "NOVAK");
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted, "BY NAME should read in ascending name order");
}

#[test]
fn a_read_limit_stops_after_that_many_records() {
    let out = run(&program(
        "READ (3) EMPLOYEES-VIEW BY NAME\nWRITE NAME\nEND-READ",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 3);
    assert_eq!(out.lines[0].trim(), "ABELLAN");
}

#[test]
fn without_a_limit_or_order_records_come_back_in_stored_sequence() {
    let out = run(&program("READ EMPLOYEES-VIEW\nWRITE NAME\nEND-READ")).expect("should run");
    assert_eq!(out.lines[0].trim(), "GARRET");
    assert_eq!(out.lines[1].trim(), "ABELLAN");
}

#[test]
fn numeric_view_fields_carry_their_ddm_format() {
    // SALARY is P9 in the DDM, so it prints in ten positions: nine digits plus the sign.
    let out = run(&program("READ (1) EMPLOYEES-VIEW\nWRITE SALARY\nEND-READ")).expect("should run");
    assert_eq!(out.lines, vec![format!("{}22000", " ".repeat(5))]);
}

#[test]
fn a_read_loop_works_with_display() {
    let out = run(&program(
        "READ (2) EMPLOYEES-VIEW BY NAME\nDISPLAY NAME CITY\nEND-READ",
    ))
    .expect("should run");
    // header, underline, blank, then two rows.
    assert_eq!(out.lines.len(), 5);
    assert_eq!(out.lines[3].trim_end(), "ABELLAN              MADRID");
}

#[test]
fn the_loop_body_can_compute_from_view_fields() {
    let source = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 NAME
2 SALARY
1 #TOTAL (P11)
END-DEFINE
READ EMPLOYEES-VIEW
ADD SALARY TO #TOTAL
END-READ
WRITE 'Total payroll:' #TOTAL
END";
    let out = run(source).expect("should run");
    // 22000 + 32000 + 46000 + 39300 + 23000 + 52000 + 66300 + 41500
    assert_eq!(
        out.get("#TOTAL"),
        Some(&Value::Number(rust_decimal::Decimal::from(322100)))
    );
}

#[test]
fn a_conditional_inside_a_read_filters_records() {
    let source = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 NAME
2 COUNTRY
END-DEFINE
READ EMPLOYEES-VIEW BY NAME
IF COUNTRY = 'USA'
WRITE NAME
END-IF
END-READ
END";
    let out = run(source).expect("should run");
    let names: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    assert_eq!(names, vec!["HAMMOND", "KOLENCE", "LOMAX"]);
}

#[test]
fn escape_bottom_leaves_a_read_loop() {
    let out = run(&program(
        "READ EMPLOYEES-VIEW\nWRITE NAME\nESCAPE BOTTOM\nEND-READ\nWRITE 'done'",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 2);
    assert_eq!(out.lines[1], "done");
}

#[test]
fn execution_continues_after_end_read() {
    let out = run(&program(
        "READ (2) EMPLOYEES-VIEW\nWRITE NAME\nEND-READ\nWRITE 'after'",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 3);
    assert_eq!(out.lines[2], "after");
}

#[test]
fn an_input_inside_a_read_loop_suspends_on_each_record() {
    let source = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 NAME
1 #NOTE (A10)
END-DEFINE
READ (2) EMPLOYEES-VIEW
INPUT 'Note?' #NOTE
WRITE NAME #NOTE
END-READ
END";
    let out = run_with_input(source, &["one", "two"]).expect("should run");
    assert_eq!(out.prompts.len(), 2);
    assert_eq!(out.lines.len(), 2);
}

#[test]
fn the_sample_data_is_rebuilt_for_every_run() {
    // Two runs of the same program must produce identical output. This is the per-lesson
    // reset requirement, and it is what makes a STORE or UPDATE exercise repeatable.
    let source = program("READ EMPLOYEES-VIEW BY NAME\nWRITE NAME\nEND-READ");
    let first = run(&source).expect("should run");
    let second = run(&source).expect("should run");
    assert_eq!(first.lines, second.lines);
}

// ---- teaching errors ----

#[test]
fn a_view_of_an_unknown_file_is_a_teaching_error() {
    let source = "\
DEFINE DATA LOCAL
1 X-VIEW VIEW OF NOSUCHFILE
2 NAME
END-DEFINE
END";
    let err = run(source).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownDdm { .. }),
        "expected UnknownDdm, got {err:?}"
    );
}

#[test]
fn a_field_missing_from_the_ddm_is_a_teaching_error() {
    let source = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 NOT-A-FIELD
END-DEFINE
END";
    let err = run(source).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownDdmField { .. }),
        "expected UnknownDdmField, got {err:?}"
    );
    assert!(
        err.to_string().contains("NOT-A-FIELD"),
        "message should name the field, got: {err}"
    );
}

#[test]
fn reading_an_undeclared_view_is_a_teaching_error() {
    let err = run(&program("READ NOPE-VIEW\nWRITE NAME\nEND-READ")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownView { .. }),
        "expected UnknownView, got {err:?}"
    );
}

#[test]
fn an_unclosed_read_is_a_teaching_error() {
    let err = run(&program("READ EMPLOYEES-VIEW\nWRITE NAME")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::MissingLoopEnd { .. }),
        "expected MissingLoopEnd, got {err:?}"
    );
}

#[test]
fn sorting_by_a_field_that_is_not_in_the_ddm_is_a_teaching_error() {
    let err = run(&program(
        "READ EMPLOYEES-VIEW BY NOPE\nWRITE NAME\nEND-READ",
    ))
    .expect_err("reject");
    assert!(
        matches!(err, NaturalError::UnknownDdmField { .. }),
        "expected UnknownDdmField, got {err:?}"
    );
}
