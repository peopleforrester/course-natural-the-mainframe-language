// ABOUTME: Milestone M-C acceptance tests for DISPLAY, the column-oriented report
// ABOUTME: statement, against the verified header, sizing, and underline rules.

use natural_core::{NaturalError, run};

fn program(declarations: &str, body: &str) -> String {
    format!("DEFINE DATA LOCAL\n{declarations}END-DEFINE\n{body}\nEND")
}

#[test]
fn display_emits_a_header_an_underline_a_blank_line_then_data() {
    // #NAME is A20 (field width 20) under the 5 character header "#NAME", so the column is
    // 20 wide and the header is centered. #AGE is N3 (field width 4, three digits plus the
    // sign position) under the 4 character header "#AGE", so that column is exactly 4.
    // Columns are separated by one blank.
    let out = run(&program(
        "1 #NAME (A20)\n1 #AGE (N3)\n",
        "MOVE 'GRACE' TO #NAME\nMOVE 42 TO #AGE\nDISPLAY #NAME #AGE",
    ))
    .expect("should run");

    let expected = vec![
        format!("{}#NAME{}#AGE", " ".repeat(7), " ".repeat(9)),
        format!("{} {}", "-".repeat(20), "-".repeat(4)),
        String::new(),
        format!("GRACE{}  42", " ".repeat(16)),
    ];
    assert_eq!(out.lines, expected);
}

#[test]
fn the_header_for_a_user_defined_variable_is_its_name() {
    let out = run(&program("1 #TOTAL (N3)\n", "DISPLAY #TOTAL")).expect("should run");
    // "#TOTAL" is 6 characters and the N3 field prints in 4, so the header decides the
    // column width.
    assert_eq!(out.lines[0], "#TOTAL");
    assert_eq!(out.lines[1], "-".repeat(6));
}

#[test]
fn the_column_is_as_wide_as_the_field_when_the_field_is_wider() {
    let out = run(&program("1 #S (A12)\n", "DISPLAY #S")).expect("should run");
    assert_eq!(out.lines[1], "-".repeat(12));
}

#[test]
fn a_field_is_left_justified_under_a_wider_header() {
    // Verified by DISPLX01: an 8 character value under a 9 character header occupies the
    // first 8 columns, with the extra column as header-driven padding.
    let out = run(&program(
        "1 #IDENTIFIER (A3)\n1 #TAIL (A4)\n",
        "MOVE 'ABC' TO #IDENTIFIER\nMOVE 'ZZZZ' TO #TAIL\nDISPLAY #IDENTIFIER #TAIL",
    ))
    .expect("should run");
    // "#IDENTIFIER" is 11 characters, the field is 3, so the column is 11 and ABC sits at
    // the far left of it.
    assert_eq!(out.lines[3], format!("ABC{} ZZZZ", " ".repeat(8)));
}

#[test]
fn numeric_values_are_right_justified_and_alphanumeric_left_justified() {
    let out = run(&program(
        "1 #A (A6)\n1 #N (N4)\n",
        "MOVE 'AB' TO #A\nMOVE 7 TO #N\nDISPLAY #A #N",
    ))
    .expect("should run");
    // #A column is 6 (field wins), value left-justified, so AB then 4 blanks. One blank
    // separates the columns. #N column is 5 (field width 5 beats the 2 character header)
    // with the value right-justified inside, so 4 more blanks then 7.
    assert_eq!(out.lines[3], format!("AB{}7", " ".repeat(9)));
}

#[test]
fn the_underline_covers_each_column_but_not_the_gap_between_them() {
    let out = run(&program("1 #A (A4)\n1 #B (A6)\n", "DISPLAY #A #B")).expect("should run");
    assert_eq!(out.lines[1], format!("{} {}", "-".repeat(4), "-".repeat(6)));
}

#[test]
fn exactly_one_blank_line_separates_the_underline_from_the_data() {
    let out = run(&program("1 #A (A4)\n", "MOVE 'XY' TO #A\nDISPLAY #A")).expect("should run");
    assert_eq!(out.lines.len(), 4);
    assert_eq!(out.lines[2], "");
}

#[test]
fn headers_are_emitted_once_no_matter_how_many_rows_follow() {
    // The reporting pattern: one DISPLAY inside a loop produces one header and many rows.
    let out = run(&program(
        "1 #I (I4)\n1 #S (A5)\n",
        "MOVE 'ROW' TO #S\nFOR #I = 1 TO 3\nDISPLAY #S\nEND-FOR",
    ))
    .expect("should run");
    // header, underline, blank, then three data rows. The A5 column is 5 wide and the
    // 2 character header is centered in it, so one blank precedes it.
    assert_eq!(out.lines.len(), 6);
    assert_eq!(out.lines[0], " #S");
    assert_eq!(out.lines[1], "-----");
    assert_eq!(out.lines[3], "ROW");
    assert_eq!(out.lines[4], "ROW");
    assert_eq!(out.lines[5], "ROW");
}

#[test]
fn write_still_emits_no_headers() {
    // The distinction module 6 teaches. WRITE is free-format and never generates headers.
    let out = run(&program(
        "1 #S (A5)\n",
        "MOVE 'ROW' TO #S\nWRITE #S\nWRITE #S",
    ))
    .expect("should run");
    assert_eq!(out.lines, vec!["ROW", "ROW"]);
}

#[test]
fn display_and_write_can_appear_in_the_same_program() {
    let out = run(&program(
        "1 #S (A4)\n",
        "WRITE 'Report follows'\nMOVE 'DATA' TO #S\nDISPLAY #S",
    ))
    .expect("should run");
    assert_eq!(out.lines[0], "Report follows");
    // The A4 column is 4 wide, so the 2 character header is centered with one blank
    // either side; the trailing blank falls at end of line and is trimmed.
    assert_eq!(out.lines[1], " #S");
    assert_eq!(out.lines[2], "----");
    assert_eq!(out.lines[3], "");
    assert_eq!(out.lines[4], "DATA");
}

// ---- teaching errors ----

#[test]
fn displaying_an_undeclared_field_is_a_teaching_error() {
    let err = run(&program("1 #A (A4)\n", "DISPLAY #NOPE")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UndeclaredVariable { .. }),
        "expected UndeclaredVariable, got {err:?}"
    );
}

#[test]
fn display_with_nothing_to_show_is_a_teaching_error() {
    let err = run(&program("1 #A (A4)\n", "DISPLAY")).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}
