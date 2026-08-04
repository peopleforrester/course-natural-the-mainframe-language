// ABOUTME: Milestone M-D acceptance tests for FIND: descriptor search, the WHERE filter,
// ABOUTME: the IF NO RECORDS FOUND clause, and the *NUMBER and *COUNTER system variables.

use natural_core::{Decimal, NaturalError, Value, run};

const VIEW: &str = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 NAME
2 FIRST-NAME
2 CITY
2 COUNTRY
2 SALARY
END-DEFINE
";

fn program(body: &str) -> String {
    format!("{VIEW}{body}\nEND")
}

#[test]
fn find_selects_records_matching_the_descriptor() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH NAME = 'JONES'\nWRITE FIRST-NAME\nEND-FIND",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 1);
    assert_eq!(out.lines[0].trim(), "VIRGINIA");
}

#[test]
fn find_visits_every_matching_record() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA'\nWRITE NAME\nEND-FIND",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 3);
}

#[test]
fn a_find_with_no_matches_runs_its_body_no_times() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'ZZZ'\nWRITE NAME\nEND-FIND\nWRITE 'after'",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["after"]);
}

#[test]
fn find_can_be_sorted() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' SORTED BY NAME\nWRITE NAME\nEND-FIND",
    ))
    .expect("should run");
    let names: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    assert_eq!(names, vec!["HAMMOND", "KOLENCE", "LOMAX"]);
}

#[test]
fn a_find_limit_caps_the_records_processed() {
    let out = run(&program(
        "FIND (2) EMPLOYEES-VIEW WITH COUNTRY = 'USA' SORTED BY NAME\nWRITE NAME\nEND-FIND",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 2);
}

#[test]
fn the_where_clause_filters_further() {
    // SORTED BY comes before WHERE, which is the documented order.
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' SORTED BY NAME WHERE SALARY > 50000\n\
         WRITE NAME\nEND-FIND",
    ))
    .expect("should run");
    let names: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    assert_eq!(names, vec!["KOLENCE", "LOMAX"]);
}

// ---- IF NO RECORDS FOUND ----

#[test]
fn if_no_records_found_runs_when_nothing_matched() {
    // The clause does not replace the loop. Natural enters the loop once with the database
    // fields reset, so an empty NAME prints here. ESCAPE BOTTOM is how you suppress it, and
    // find_semantics.rs covers that path.
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH NAME = 'NOBODY'\n\
         IF NO RECORDS FOUND\nWRITE 'none at all'\nEND-NOREC\n\
         WRITE NAME\nEND-FIND\nWRITE 'after'",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 3);
    assert_eq!(out.lines[0], "none at all");
    assert!(
        out.lines[1].trim().is_empty(),
        "the reset pass prints an empty NAME, got: {:?}",
        out.lines[1]
    );
    assert_eq!(out.lines[2], "after");
}

#[test]
fn if_no_records_found_is_skipped_when_records_matched() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH NAME = 'JONES'\n\
         IF NO RECORDS FOUND\nWRITE 'none at all'\nEND-NOREC\n\
         WRITE FIRST-NAME\nEND-FIND",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 1);
    assert_eq!(out.lines[0].trim(), "VIRGINIA");
}

#[test]
fn the_loop_body_still_runs_for_every_match_alongside_a_norec_clause() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA'\n\
         IF NO RECORDS FOUND\nWRITE 'none'\nEND-NOREC\n\
         WRITE NAME\nEND-FIND",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 3);
}

// ---- system variables ----

#[test]
fn star_number_holds_how_many_records_the_search_found() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA'\nESCAPE BOTTOM\nEND-FIND\nWRITE *NUMBER",
    ))
    .expect("should run");
    assert_eq!(out.get("*NUMBER"), Some(&Value::Number(Decimal::from(3))));
}

#[test]
fn star_number_counts_the_search_not_the_where_filter() {
    // A real teaching point: WITH is what the database searched, WHERE is applied
    // afterwards, so *NUMBER reports the wider count.
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' WHERE SALARY > 50000\n\
         WRITE NAME\nEND-FIND\nWRITE *NUMBER",
    ))
    .expect("should run");
    assert_eq!(out.get("*NUMBER"), Some(&Value::Number(Decimal::from(3))));
    // Two records survived the WHERE clause, plus the line printing *NUMBER.
    assert_eq!(out.lines.len(), 3);
}

#[test]
fn star_counter_counts_records_processed_so_far() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' SORTED BY NAME\nWRITE *COUNTER\nEND-FIND",
    ))
    .expect("should run");
    let counts: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    assert_eq!(counts, vec!["1", "2", "3"]);
}

#[test]
fn star_counter_works_in_a_read_loop_too() {
    let out = run(&program(
        "READ (2) EMPLOYEES-VIEW\nWRITE *COUNTER\nEND-READ",
    ))
    .expect("should run");
    let counts: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    assert_eq!(counts, vec!["1", "2"]);
}

// ---- interaction ----

#[test]
fn escape_bottom_leaves_a_find_loop() {
    let out = run(&program(
        "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA'\nWRITE NAME\nESCAPE BOTTOM\nEND-FIND\n\
         WRITE 'done'",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 2);
    assert_eq!(out.lines[1], "done");
}

#[test]
fn a_find_can_aggregate_like_a_read() {
    let source = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 COUNTRY
2 SALARY
1 #TOTAL (P11)
END-DEFINE
FIND EMPLOYEES-VIEW WITH COUNTRY = 'UK'
ADD SALARY TO #TOTAL
END-FIND
END";
    let out = run(source).expect("should run");
    // GARRET 22000 plus JONES 39300.
    assert_eq!(
        out.get("#TOTAL"),
        Some(&Value::Number(Decimal::from(61300)))
    );
}

// ---- teaching errors ----

#[test]
fn finding_on_an_undeclared_view_is_a_teaching_error() {
    let err = run(&program(
        "FIND NOPE-VIEW WITH NAME = 'X'\nWRITE NAME\nEND-FIND",
    ))
    .expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownView { .. }),
        "expected UnknownView, got {err:?}"
    );
}

#[test]
fn an_unclosed_find_is_a_teaching_error() {
    let err = run(&program("FIND EMPLOYEES-VIEW WITH NAME = 'X'\nWRITE NAME"))
        .expect_err("should reject");
    assert!(
        matches!(err, NaturalError::MissingLoopEnd { .. }),
        "expected MissingLoopEnd, got {err:?}"
    );
}

#[test]
fn an_end_norec_without_its_clause_is_a_teaching_error() {
    let err = run(&program("END-NOREC")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnexpectedBlockKeyword { .. }),
        "expected UnexpectedBlockKeyword, got {err:?}"
    );
}

#[test]
fn a_find_without_a_with_clause_is_a_teaching_error() {
    let err = run(&program("FIND EMPLOYEES-VIEW\nWRITE NAME\nEND-FIND")).expect_err("reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}
