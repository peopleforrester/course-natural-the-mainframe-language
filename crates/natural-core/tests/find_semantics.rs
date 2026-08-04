// ABOUTME: Regression tests from the content audit: documented FIND clause order, the
// ABOUTME: real IF NO RECORDS FOUND semantics, and *NUMBER reference notation.

use natural_core::{Decimal, NaturalError, Value, run};

const VIEW: &str = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 NAME
2 COUNTRY
2 SALARY
END-DEFINE
";

fn program(body: &str) -> String {
    format!("{VIEW}{body}\nEND")
}

// ---- clause order ----

#[test]
fn sorted_by_precedes_where_as_documented() {
    // The FIND statement documents the order WITH, then SORTED BY, then WHERE. Accepting
    // only the documented order matters more than convenience: the course promises real
    // Natural, and a learner who writes it the other way should find out here.
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' SORTED BY NAME WHERE SALARY > 45000\n\
         WRITE NAME\nEND-FIND",
    ))
    .expect("the documented order should be accepted");
    let names: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    assert_eq!(names, vec!["HAMMOND", "KOLENCE", "LOMAX"]);
}

#[test]
fn where_before_sorted_by_is_a_teaching_error() {
    let err = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' WHERE SALARY > 45000 SORTED BY NAME\n\
         WRITE NAME\nEND-FIND",
    ))
    .expect_err("the undocumented order should be rejected");
    assert!(
        matches!(err, NaturalError::ClauseOutOfOrder { .. }),
        "expected ClauseOutOfOrder, got {err:?}"
    );
    assert!(
        err.to_string().contains("SORTED BY") && err.to_string().contains("WHERE"),
        "the message should name both clauses, got: {err}"
    );
}

// ---- IF NO RECORDS FOUND ----

#[test]
fn the_norec_clause_runs_before_the_loop_and_the_loop_still_runs_once() {
    // Documented behavior, and genuinely surprising: the clause runs immediately BEFORE
    // the loop is entered, and the loop then executes exactly once with every database
    // field reset. A learner who assumes the loop is skipped gets a stray blank line.
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH CITY = 'ATLANTIS'\n\
         IF NO RECORDS FOUND\nWRITE 'none found'\nEND-NOREC\n\
         WRITE 'loop body ran'\nEND-FIND\nWRITE 'after'",
    ))
    .expect("should run");
    assert_eq!(
        out.lines,
        vec!["none found", "loop body ran", "after"],
        "the loop body must execute once after the NOREC clause"
    );
}

#[test]
fn database_fields_are_reset_for_that_single_pass() {
    let out = run(&program(
        "MOVE 'LEFTOVER' TO NAME\n\
         FIND EMPLOYEES-VIEW WITH CITY = 'ATLANTIS'\n\
         IF NO RECORDS FOUND\nWRITE 'none'\nEND-NOREC\n\
         WRITE 'name is [' NAME ']'\nEND-FIND",
    ))
    .expect("should run");
    let shown = out.lines.last().expect("a line");
    assert!(
        !shown.contains("LEFTOVER"),
        "database fields must be reset for the empty pass, got: {shown}"
    );
}

#[test]
fn escape_bottom_inside_the_clause_suppresses_that_pass() {
    // The documented remedy, and the honest reason to reach for ESCAPE BOTTOM.
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH CITY = 'ATLANTIS'\n\
         IF NO RECORDS FOUND\nWRITE 'none found'\nESCAPE BOTTOM\nEND-NOREC\n\
         WRITE 'should not appear'\nEND-FIND\nWRITE 'after'",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["none found", "after"]);
}

#[test]
fn a_find_that_matches_does_not_run_the_norec_clause() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'UK' SORTED BY NAME\n\
         IF NO RECORDS FOUND\nWRITE 'none'\nEND-NOREC\n\
         WRITE NAME\nEND-FIND",
    ))
    .expect("should run");
    let names: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    assert_eq!(names, vec!["GARRET", "JONES"]);
}

#[test]
fn a_find_with_no_norec_clause_still_skips_its_body_when_empty() {
    // Without the clause there is nothing to run before the loop, so an empty search
    // simply does nothing. Only the NOREC form gets the single reset pass.
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH CITY = 'ATLANTIS'\nWRITE 'body'\nEND-FIND\nWRITE 'after'",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["after"]);
}

// ---- reference notation ----

#[test]
fn a_labelled_find_exposes_star_number_after_the_loop() {
    // Outside the loop a system variable needs reference notation naming which loop it
    // came from, which is also why a learner needs labels before nested loops.
    let out = run(&program(
        "EMP. FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' SORTED BY NAME WHERE SALARY > 45000\n\
         WRITE NAME\nEND-FIND\n\
         WRITE 'matched before WHERE:' *NUMBER(EMP.)",
    ))
    .expect("should run");
    // Three USA employees matched the WITH search; the WHERE filter kept all three.
    assert!(
        out.lines.last().expect("a line").contains('3'),
        "*NUMBER(EMP.) should report the WITH count, got: {:?}",
        out.lines.last()
    );
}

#[test]
fn star_number_reports_the_with_count_not_the_where_count() {
    let out = run(&program(
        "EMP. FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' SORTED BY NAME WHERE SALARY > 50000\n\
         WRITE NAME\nEND-FIND\n\
         WRITE 'searched:' *NUMBER(EMP.)",
    ))
    .expect("should run");
    // Two survive the WHERE clause, but the search itself matched three.
    assert_eq!(out.lines.len(), 3);
    assert!(out.lines[2].contains('3'), "got: {}", out.lines[2]);
}

#[test]
fn a_label_may_also_be_used_on_read() {
    let out = run(&program(
        "REC. READ (2) EMPLOYEES-VIEW BY NAME\nWRITE NAME\nEND-READ\n\
         WRITE 'counted:' *COUNTER(REC.)",
    ))
    .expect("should run");
    assert!(out.lines.last().expect("a line").contains('2'));
}

#[test]
fn referencing_an_unknown_label_is_a_teaching_error() {
    let err = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'UK'\nWRITE NAME\nEND-FIND\n\
         WRITE *NUMBER(NOPE.)",
    ))
    .expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownLabel { .. }),
        "expected UnknownLabel, got {err:?}"
    );
}

// ---- the fixture counts the lessons assert ----

#[test]
fn the_sample_file_holds_the_counts_the_lessons_claim() {
    // Lesson 8 states "eight employees" and its exercise grades on "two of them" in UK.
    // A fixture change must not silently make published prose wrong.
    let out = run(&program("READ EMPLOYEES-VIEW\nWRITE NAME\nEND-READ")).expect("should run");
    assert_eq!(
        out.lines.len(),
        8,
        "lesson 8.2 says the file holds eight employees"
    );

    let uk = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'UK'\nWRITE NAME\nEND-FIND",
    ))
    .expect("should run");
    assert_eq!(
        uk.lines.len(),
        2,
        "the lesson 8 exercise expects two UK employees"
    );

    let usa = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA'\nWRITE NAME\nEND-FIND",
    ))
    .expect("should run");
    assert_eq!(
        usa.lines.len(),
        3,
        "lesson 8.3 and 13.4 assume three USA employees"
    );

    let esp = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'ESP'\nWRITE NAME\nEND-FIND",
    ))
    .expect("should run");
    assert_eq!(
        esp.lines.len(),
        1,
        "the lesson 13 exercise expects one ESP employee"
    );

    let total = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'UK'\nEND-FIND",
    ));
    assert!(total.is_ok());
}

#[test]
fn total_payroll_matches_what_the_lessons_publish() {
    // Lesson 11's exercise grades on 322100 and its prose shows an average of 40262.50.
    let out = run("DEFINE DATA LOCAL\n\
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n2 SALARY\n1 #TOTAL (P11)\nEND-DEFINE\n\
READ EMPLOYEES-VIEW\nADD SALARY TO #TOTAL\nEND-READ\nEND")
    .expect("should run");
    assert_eq!(
        out.get("#TOTAL"),
        Some(&Value::Number(Decimal::from(322100)))
    );
}
