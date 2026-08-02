// ABOUTME: Milestone M-D acceptance tests for HISTOGRAM, which walks the distinct values
// ABOUTME: of a descriptor and reports how many records carry each one.

use natural_core::{NaturalError, run};

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

#[test]
fn histogram_visits_each_distinct_value_once() {
    let out = run(&program(
        "HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY\nWRITE COUNTRY\nEND-HISTOGRAM",
    ))
    .expect("should run");
    let values: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    // CZ, ESP, F, UK, USA: five distinct countries across eight employees.
    assert_eq!(values, vec!["CZ", "ESP", "F", "UK", "USA"]);
}

#[test]
fn star_number_holds_the_count_for_each_value() {
    let out = run(&program(
        "HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY\nWRITE COUNTRY *NUMBER\nEND-HISTOGRAM",
    ))
    .expect("should run");
    let rows: Vec<String> = out
        .lines
        .iter()
        .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect();
    assert_eq!(
        rows,
        vec!["CZ 1", "ESP 1", "F 1", "UK 2", "USA 3"],
        "each country should report how many employees it has"
    );
}

#[test]
fn a_histogram_limit_caps_the_values_visited() {
    let out = run(&program(
        "HISTOGRAM (2) EMPLOYEES-VIEW FOR COUNTRY\nWRITE COUNTRY\nEND-HISTOGRAM",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 2);
}

#[test]
fn star_counter_counts_the_values_processed() {
    let out = run(&program(
        "HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY\nWRITE *COUNTER\nEND-HISTOGRAM",
    ))
    .expect("should run");
    let counts: Vec<String> = out.lines.iter().map(|l| l.trim().to_string()).collect();
    assert_eq!(counts, vec!["1", "2", "3", "4", "5"]);
}

#[test]
fn escape_bottom_leaves_a_histogram() {
    let out = run(&program(
        "HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY\nWRITE COUNTRY\nESCAPE BOTTOM\nEND-HISTOGRAM\n\
         WRITE 'done'",
    ))
    .expect("should run");
    assert_eq!(out.lines.len(), 2);
    assert_eq!(out.lines[1], "done");
}

#[test]
fn a_histogram_can_drive_a_report() {
    let out = run(&program(
        "HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY\nDISPLAY COUNTRY *NUMBER\nEND-HISTOGRAM",
    ))
    .expect("should run");
    // header, underline, blank, then five value rows.
    assert_eq!(out.lines.len(), 8);
}

// ---- teaching errors ----

#[test]
fn a_histogram_on_an_unknown_descriptor_is_a_teaching_error() {
    let err = run(&program(
        "HISTOGRAM EMPLOYEES-VIEW FOR NOPE\nWRITE COUNTRY\nEND-HISTOGRAM",
    ))
    .expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownDdmField { .. }),
        "expected UnknownDdmField, got {err:?}"
    );
}

#[test]
fn an_unclosed_histogram_is_a_teaching_error() {
    let err = run(&program(
        "HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY\nWRITE COUNTRY",
    ))
    .expect_err("should reject");
    assert!(
        matches!(err, NaturalError::MissingLoopEnd { .. }),
        "expected MissingLoopEnd, got {err:?}"
    );
}

#[test]
fn a_malformed_histogram_header_is_a_teaching_error() {
    let err = run(&program("HISTOGRAM EMPLOYEES-VIEW\nEND-HISTOGRAM")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}
