// ABOUTME: Milestone M-D acceptance tests for STORE, UPDATE, DELETE, and the transaction
// ABOUTME: boundary, including the classic beginner bug of forgetting END TRANSACTION.

use natural_core::{NaturalError, Value, run};

const VIEW: &str = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 PERSONNEL-ID
2 NAME
2 FIRST-NAME
2 CITY
2 COUNTRY
2 DEPT
2 JOB-TITLE
2 SALARY
END-DEFINE
";

fn program(body: &str) -> String {
    format!("{VIEW}{body}\nEND")
}

/// Adds one employee to the view buffer, ready to be stored.
const NEW_EMPLOYEE: &str = "\
MOVE '99999999' TO PERSONNEL-ID
MOVE 'TURING' TO NAME
MOVE 'ALAN' TO FIRST-NAME
MOVE 'LONDON' TO CITY
MOVE 'UK' TO COUNTRY
MOVE 'TECH01' TO DEPT
MOVE 'CRYPTANALYST' TO JOB-TITLE
MOVE 71000 TO SALARY
";

// ---- STORE ----

#[test]
fn a_committed_store_adds_a_record() {
    let out = run(&program(&format!(
        "{NEW_EMPLOYEE}STORE EMPLOYEES-VIEW\nEND TRANSACTION"
    )))
    .expect("should run");
    assert_eq!(out.committed().len(), 9);
}

#[test]
fn a_store_without_end_transaction_is_lost() {
    // The classic beginner bug, and a deliberate teaching surface. The record is written
    // during the run but never committed, so nothing persists.
    let out = run(&program(&format!("{NEW_EMPLOYEE}STORE EMPLOYEES-VIEW"))).expect("should run");
    assert_eq!(
        out.committed().len(),
        8,
        "an uncommitted STORE must not persist"
    );
}

#[test]
fn a_stored_record_is_visible_to_the_same_program_before_it_commits() {
    // Within the run the change is real; it is only persistence that waits for the commit.
    let out = run(&program(&format!(
        "{NEW_EMPLOYEE}STORE EMPLOYEES-VIEW\n\
         FIND EMPLOYEES-VIEW WITH NAME = 'TURING'\nWRITE FIRST-NAME\nEND-FIND"
    )))
    .expect("should run");
    assert_eq!(out.lines.len(), 1);
    assert_eq!(out.lines[0].trim(), "ALAN");
}

#[test]
fn the_stored_values_are_the_ones_that_were_moved_in() {
    let out = run(&program(&format!(
        "{NEW_EMPLOYEE}STORE EMPLOYEES-VIEW\nEND TRANSACTION"
    )))
    .expect("should run");
    let db = out.committed();
    let index = db
        .find("NAME", "TURING")
        .expect("the record should be there");
    assert_eq!(
        db.value(index, "JOB-TITLE"),
        Some(Value::Alpha("CRYPTANALYST".to_string()))
    );
}

// ---- BACKOUT ----

#[test]
fn backout_transaction_discards_pending_changes() {
    let out = run(&program(&format!(
        "{NEW_EMPLOYEE}STORE EMPLOYEES-VIEW\nBACKOUT TRANSACTION\nEND TRANSACTION"
    )))
    .expect("should run");
    assert_eq!(out.committed().len(), 8);
}

#[test]
fn backout_leaves_already_committed_work_alone() {
    let out = run(&program(&format!(
        "{NEW_EMPLOYEE}STORE EMPLOYEES-VIEW\nEND TRANSACTION\nBACKOUT TRANSACTION"
    )))
    .expect("should run");
    assert_eq!(out.committed().len(), 9);
}

// ---- UPDATE ----

#[test]
fn update_writes_the_current_record_back() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH NAME = 'JONES'\n\
         MOVE 60000 TO SALARY\nUPDATE\nEND-FIND\nEND TRANSACTION",
    ))
    .expect("should run");
    let db = out.committed();
    let index = db.find("NAME", "JONES").expect("JONES should still exist");
    assert_eq!(
        db.value(index, "SALARY"),
        Some(Value::Number(rust_decimal::Decimal::from(60000)))
    );
}

#[test]
fn an_uncommitted_update_is_lost() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH NAME = 'JONES'\nMOVE 60000 TO SALARY\nUPDATE\nEND-FIND",
    ))
    .expect("should run");
    let db = out.committed();
    let index = db.find("NAME", "JONES").expect("JONES should still exist");
    assert_eq!(
        db.value(index, "SALARY"),
        Some(Value::Number(rust_decimal::Decimal::from(39300))),
        "the original salary should survive an uncommitted update"
    );
}

#[test]
fn update_can_run_for_every_record_of_a_loop() {
    let out = run(&program(
        "READ EMPLOYEES-VIEW\nCOMPUTE SALARY = SALARY + 1000\nUPDATE\nEND-READ\n\
         END TRANSACTION",
    ))
    .expect("should run");
    let db = out.committed();
    let index = db.find("NAME", "GARRET").expect("GARRET should exist");
    assert_eq!(
        db.value(index, "SALARY"),
        Some(Value::Number(rust_decimal::Decimal::from(23000)))
    );
    assert_eq!(db.len(), 8, "an update must not change the record count");
}

// ---- DELETE ----

#[test]
fn delete_removes_the_current_record() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH NAME = 'JONES'\nDELETE\nEND-FIND\nEND TRANSACTION",
    ))
    .expect("should run");
    let db = out.committed();
    assert_eq!(db.len(), 7);
    assert!(db.find("NAME", "JONES").is_none(), "JONES should be gone");
}

#[test]
fn a_deleted_record_disappears_from_later_reads_in_the_same_program() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH NAME = 'JONES'\nDELETE\nEND-FIND\n\
         READ EMPLOYEES-VIEW\nWRITE NAME\nEND-READ",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 7);
    assert!(!out.lines.iter().any(|l| l.trim() == "JONES"));
}

#[test]
fn an_uncommitted_delete_is_lost() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH NAME = 'JONES'\nDELETE\nEND-FIND",
    ))
    .expect("should run");
    assert_eq!(out.committed().len(), 8);
}

// ---- the reset guarantee, now that writes exist ----

#[test]
fn every_run_starts_from_the_same_sample_data() {
    // The per-lesson reset requirement, tested where it actually matters: a program that
    // commits a change must not affect the next run.
    let source = program(&format!(
        "{NEW_EMPLOYEE}STORE EMPLOYEES-VIEW\nEND TRANSACTION\nWRITE 'stored'"
    ));
    let first = run(&source).expect("should run");
    let second = run(&source).expect("should run");
    assert_eq!(first.committed().len(), 9);
    assert_eq!(
        second.committed().len(),
        9,
        "the second run must start from eight records again, not nine"
    );
}

// ---- teaching errors ----

#[test]
fn update_outside_a_loop_is_a_teaching_error() {
    let err = run(&program("UPDATE")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::NoCurrentRecord { .. }),
        "expected NoCurrentRecord, got {err:?}"
    );
}

#[test]
fn delete_outside_a_loop_is_a_teaching_error() {
    let err = run(&program("DELETE")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::NoCurrentRecord { .. }),
        "expected NoCurrentRecord, got {err:?}"
    );
}

#[test]
fn storing_into_an_undeclared_view_is_a_teaching_error() {
    let err = run(&program("STORE NOPE-VIEW")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownView { .. }),
        "expected UnknownView, got {err:?}"
    );
}
