// ABOUTME: Milestone M-A acceptance tests: a program of WRITE statements and a
// ABOUTME: terminating END produces the expected lines of terminal output.

use natural_core::{NaturalError, run_to_lines};

#[test]
fn writes_a_string_literal() {
    let lines = run_to_lines("WRITE 'Hello World!'\nEND").expect("program should run");
    assert_eq!(lines, vec!["Hello World!"]);
}

#[test]
fn writes_multiple_lines_in_order() {
    let source = "WRITE 'first'\nWRITE 'second'\nEND";
    let lines = run_to_lines(source).expect("program should run");
    assert_eq!(lines, vec!["first", "second"]);
}

#[test]
fn keywords_are_case_insensitive() {
    // Natural source is conventionally uppercase, but the language does not require it.
    let lines = run_to_lines("write 'lower'\nend").expect("program should run");
    assert_eq!(lines, vec!["lower"]);
}

#[test]
fn write_with_no_operand_emits_a_blank_line() {
    let lines = run_to_lines("WRITE 'a'\nWRITE\nWRITE 'b'\nEND").expect("program should run");
    assert_eq!(lines, vec!["a", "", "b"]);
}

#[test]
fn a_quote_is_escaped_by_doubling_it() {
    let lines = run_to_lines("WRITE 'it''s here'\nEND").expect("program should run");
    assert_eq!(lines, vec!["it's here"]);
}

#[test]
fn missing_end_is_a_teaching_error() {
    let err = run_to_lines("WRITE 'no end here'").expect_err("should reject a program with no END");
    assert!(
        matches!(err, NaturalError::MissingEnd),
        "expected MissingEnd, got {err:?}"
    );
    // Diagnostics name the Natural concept, not parser internals.
    assert!(
        err.to_string().contains("END"),
        "message should mention END, got: {err}"
    );
}

#[test]
fn an_unterminated_string_is_a_teaching_error() {
    let err = run_to_lines("WRITE 'oops\nEND").expect_err("should reject an unterminated string");
    assert!(
        matches!(err, NaturalError::UnterminatedString { line: 1 }),
        "expected UnterminatedString on line 1, got {err:?}"
    );
}

#[test]
fn an_unknown_statement_is_a_teaching_error() {
    let err = run_to_lines("FLARP 'x'\nEND").expect_err("should reject an unknown statement");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}

#[test]
fn blank_lines_are_ignored() {
    let lines = run_to_lines("\n\nWRITE 'x'\n\nEND\n").expect("program should run");
    assert_eq!(lines, vec!["x"]);
}
