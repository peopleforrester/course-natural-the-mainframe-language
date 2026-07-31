// ABOUTME: Output formatting acceptance tests transcribed from the verified table in
// ABOUTME: research/07-output-formatting-semantics.md. Row ids in comments are that table's.

use natural_core::run_to_lines;

/// Runs a program whose body is a single WRITE and returns the one output line.
fn line(declarations: &str, body: &str) -> String {
    let source = format!("DEFINE DATA LOCAL\n{declarations}END-DEFINE\n{body}\nEND");
    let mut lines = run_to_lines(&source).expect("program should run");
    assert_eq!(lines.len(), 1, "expected exactly one output line");
    lines.remove(0)
}

// ---- A. Field print widths ----
//
// The governing rule, verified in the spike: a numeric field's print width is
// integer digits, plus one for the decimal point when the field has decimals, plus the
// decimal digits, plus ONE MORE leading position reserved for the sign under the default
// SG=ON. Nothing is trimmed; every field occupies its full width.

#[test]
fn a11_numeric_reserves_a_leading_sign_position() {
    // N7.2 holding 19.99: width 11, so six leading blanks.
    assert_eq!(
        line("1 #S (N7.2)\n", "MOVE 19.99 TO #S\nWRITE #S"),
        "      19.99"
    );
}

#[test]
fn a12_small_numeric_zero() {
    // N3 holding 0: width 4. The units digit is forced by the default Z9 mask.
    assert_eq!(line("1 #D (N3)\n", "WRITE #D"), "   0");
}

#[test]
fn a13_numeric_zero_forces_units_and_decimals() {
    // N7.2 holding 0: width 11, printed as 0.00 rather than blank.
    assert_eq!(line("1 #B (N7.2)\n", "WRITE #B"), "       0.00");
}

#[test]
fn a15_packed_four_digits() {
    // P4 holding 7: width 5.
    assert_eq!(line("1 #A (P4)\n", "MOVE 7 TO #A\nWRITE #A"), "    7");
}

#[test]
fn a16_packed_nine_digits() {
    // P9 holding 46000: width 10.
    assert_eq!(
        line("1 #A (P9)\n", "MOVE 46000 TO #A\nWRITE #A"),
        "     46000"
    );
}

#[test]
fn a17_packed_ten_digits() {
    // P10 holding 66300: width 11.
    assert_eq!(
        line("1 #C (P10)\n", "MOVE 66300 TO #C\nWRITE #C"),
        "      66300"
    );
}

#[test]
fn a24_packed_zero() {
    // P9 holding 0: width 10.
    assert_eq!(line("1 #BONUS (P9)\n", "WRITE #BONUS"), "         0");
}

#[test]
fn a18_packed_prints_identically_to_numeric() {
    // The documentation states outright that Natural converts P to N for output.
    let packed = line("1 #P (P7.2)\n", "MOVE 19.99 TO #P\nWRITE #P");
    let numeric = line("1 #N (N7.2)\n", "MOVE 19.99 TO #N\nWRITE #N");
    assert_eq!(packed, numeric);
    assert_eq!(packed, "      19.99");
}

#[test]
fn a19_one_byte_integer_has_three_digits() {
    // I1 holding 1: three digits plus the sign position, so width 4.
    assert_eq!(
        line("1 #INDEX (I1)\n", "MOVE 1 TO #INDEX\nWRITE #INDEX"),
        "   1"
    );
}

#[test]
fn a21_four_byte_integer_has_ten_digits() {
    // I4 holding 42: ten digits plus the sign position, so width 11.
    assert_eq!(
        line("1 #N (I4)\n", "MOVE 42 TO #N\nWRITE #N"),
        "         42"
    );
}

#[test]
fn negative_values_use_the_sign_position() {
    // N1.3 holding -0.123 fills its width of 6 exactly.
    assert_eq!(
        line("1 #E (N1.3)\n", "MOVE -0.123 TO #E\nWRITE #E"),
        "-0.123"
    );
}

// ---- B. Composition and separation ----

#[test]
fn b1_a_literal_is_emitted_verbatim_with_no_padding() {
    assert_eq!(run_to_lines("WRITE 'Hello'\nEND").unwrap(), vec!["Hello"]);
}

#[test]
fn b2_literal_blanks_are_kept_and_the_separator_is_added() {
    // The literal carries three trailing blanks; WRITE adds one more as the separator,
    // giving four before MADRID. The A20 field then pads out to its full width.
    assert_eq!(
        line(
            "1 #CITY (A20)\n",
            "MOVE 'MADRID' TO #CITY\nWRITE 'CITY:   ' #CITY"
        ),
        "CITY:    MADRID              "
    );
}

#[test]
fn b3_two_alpha_fields_are_each_padded_and_separated_by_one_blank() {
    // 20 + one separator + 20 = 41 characters.
    let out = line(
        "1 #A (A20)\n1 #B (A20)\n",
        "MOVE 'JONES' TO #A\nMOVE 'VIRGINIA' TO #B\nWRITE #A #B",
    );
    assert_eq!(out, "JONES                VIRGINIA            ");
    assert_eq!(out.len(), 41);
}

#[test]
fn b4_literal_then_packed_field() {
    // An 18 character literal, one separator, then P10 at width 11.
    let out = line(
        "1 #S (P10)\n",
        "MOVE 66300 TO #S\nWRITE 'CUMULATIVE SALARY:' #S",
    );
    assert_eq!(out, "CUMULATIVE SALARY:       66300");
    assert_eq!(out.len(), 30);
}

#[test]
fn alpha_shorter_than_its_field_keeps_real_trailing_blanks() {
    let out = line("1 #N (A20)\n", "MOVE 'Hello' TO #N\nWRITE #N");
    assert_eq!(out, "Hello               ");
    assert_eq!(out.len(), 20);
}
