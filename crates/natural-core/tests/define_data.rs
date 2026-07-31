// ABOUTME: Milestone M-B acceptance tests: the DEFINE DATA block, format and length
// ABOUTME: parsing, default values, and assignment through MOVE, :=, and RESET.

use natural_core::{Decimal, Format, NaturalError, Value, run};
use std::str::FromStr;

fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).expect("test literal should parse")
}

#[test]
fn declares_variables_with_format_appropriate_defaults() {
    let source = "\
DEFINE DATA LOCAL
1 #NAME (A20)
1 #SALARY (N7.2)
1 #COUNT (I4)
1 #ACTIVE (L)
END-DEFINE
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.get("#NAME"), Some(&Value::Alpha(String::new())));
    assert_eq!(out.get("#SALARY"), Some(&Value::Number(Decimal::ZERO)));
    assert_eq!(out.get("#COUNT"), Some(&Value::Number(Decimal::ZERO)));
    assert_eq!(out.get("#ACTIVE"), Some(&Value::Logical(false)));
}

#[test]
fn n7_2_means_seven_integer_positions_and_two_decimals() {
    // Verified against the Natural Programming Guide, "User-Defined Variables":
    // in the nn.m form, nn is the count of positions BEFORE the decimal point.
    // Nine digit positions in total, not seven.
    let source = "\
DEFINE DATA LOCAL
1 #BIG (N7.2)
END-DEFINE
MOVE 1234567.89 TO #BIG
END";
    let out = run(source).expect("a value filling all nine positions should fit");
    assert_eq!(out.get("#BIG"), Some(&Value::Number(dec("1234567.89"))));
}

#[test]
fn parses_each_supported_format() {
    let source = "\
DEFINE DATA LOCAL
1 #A (A10)
1 #N (N5.2)
1 #P (P3.1)
1 #I (I2)
1 #L (L)
END-DEFINE
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.format("#A"), Some(&Format::Alpha { length: 10 }));
    assert_eq!(
        out.format("#N"),
        Some(&Format::Numeric {
            int_digits: 5,
            decimals: 2
        })
    );
    assert_eq!(
        out.format("#P"),
        Some(&Format::Packed {
            int_digits: 3,
            decimals: 1
        })
    );
    assert_eq!(out.format("#I"), Some(&Format::Integer { bytes: 2 }));
    assert_eq!(out.format("#L"), Some(&Format::Logical));
}

#[test]
fn a_format_with_no_decimals_has_zero_scale() {
    let source = "\
DEFINE DATA LOCAL
1 #WHOLE (N5)
END-DEFINE
END";
    let out = run(source).expect("program should run");
    assert_eq!(
        out.format("#WHOLE"),
        Some(&Format::Numeric {
            int_digits: 5,
            decimals: 0
        })
    );
}

#[test]
fn move_puts_text_into_an_alphanumeric_field() {
    let source = "\
DEFINE DATA LOCAL
1 #NAME (A20)
END-DEFINE
MOVE 'ADABAS' TO #NAME
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.get("#NAME"), Some(&Value::Alpha("ADABAS".to_string())));
}

#[test]
fn move_puts_a_number_into_a_numeric_field() {
    let source = "\
DEFINE DATA LOCAL
1 #SALARY (N7.2)
END-DEFINE
MOVE 19.99 TO #SALARY
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.get("#SALARY"), Some(&Value::Number(dec("19.99"))));
}

#[test]
fn the_assignment_operator_is_equivalent_to_move() {
    let source = "\
DEFINE DATA LOCAL
1 #QTY (I4)
END-DEFINE
#QTY := 3
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.get("#QTY"), Some(&Value::Number(dec("3"))));
}

#[test]
fn alphanumeric_assignment_truncates_to_the_declared_length() {
    let source = "\
DEFINE DATA LOCAL
1 #SHORT (A5)
END-DEFINE
MOVE 'TRUNCATED' TO #SHORT
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.get("#SHORT"), Some(&Value::Alpha("TRUNC".to_string())));
}

#[test]
fn move_copies_between_variables() {
    let source = "\
DEFINE DATA LOCAL
1 #SRC (A10)
1 #DST (A10)
END-DEFINE
MOVE 'COPY ME' TO #SRC
MOVE #SRC TO #DST
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.get("#DST"), Some(&Value::Alpha("COPY ME".to_string())));
}

#[test]
fn reset_restores_the_format_default() {
    let source = "\
DEFINE DATA LOCAL
1 #NAME (A10)
1 #TOTAL (N5.2)
END-DEFINE
MOVE 'SOMETHING' TO #NAME
MOVE 42.5 TO #TOTAL
RESET #NAME #TOTAL
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.get("#NAME"), Some(&Value::Alpha(String::new())));
    assert_eq!(out.get("#TOTAL"), Some(&Value::Number(Decimal::ZERO)));
}

#[test]
fn variable_names_are_case_insensitive() {
    let source = "\
DEFINE DATA LOCAL
1 #Name (A10)
END-DEFINE
MOVE 'x' TO #NAME
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.get("#name"), Some(&Value::Alpha("x".to_string())));
}

#[test]
fn logical_fields_accept_true_and_false() {
    let source = "\
DEFINE DATA LOCAL
1 #FLAG (L)
END-DEFINE
MOVE TRUE TO #FLAG
END";
    let out = run(source).expect("program should run");
    assert_eq!(out.get("#FLAG"), Some(&Value::Logical(true)));
}

// ---- teaching errors ----

#[test]
fn define_data_must_be_the_first_statement() {
    let err = run("WRITE 'too early'\nDEFINE DATA LOCAL\n1 #A (A5)\nEND-DEFINE\nEND")
        .expect_err("should reject a late DEFINE DATA");
    assert!(
        matches!(err, NaturalError::DefineDataNotFirst { .. }),
        "expected DefineDataNotFirst, got {err:?}"
    );
    assert!(
        err.to_string().contains("DEFINE DATA"),
        "message should name the concept, got: {err}"
    );
}

#[test]
fn an_unclosed_define_data_block_is_a_teaching_error() {
    let err = run("DEFINE DATA LOCAL\n1 #A (A5)\nEND")
        .expect_err("should reject a block with no END-DEFINE");
    assert!(
        matches!(err, NaturalError::MissingEndDefine),
        "expected MissingEndDefine, got {err:?}"
    );
}

#[test]
fn assigning_to_an_undeclared_variable_is_a_teaching_error() {
    let err = run("DEFINE DATA LOCAL\n1 #A (A5)\nEND-DEFINE\nMOVE 'x' TO #NOPE\nEND")
        .expect_err("should reject an undeclared target");
    assert!(
        matches!(err, NaturalError::UndeclaredVariable { .. }),
        "expected UndeclaredVariable, got {err:?}"
    );
    assert!(
        err.to_string().contains("#NOPE"),
        "message should name the variable, got: {err}"
    );
}

#[test]
fn declaring_the_same_variable_twice_is_a_teaching_error() {
    let err = run("DEFINE DATA LOCAL\n1 #A (A5)\n1 #A (N3)\nEND-DEFINE\nEND")
        .expect_err("should reject a duplicate declaration");
    assert!(
        matches!(err, NaturalError::DuplicateVariable { .. }),
        "expected DuplicateVariable, got {err:?}"
    );
}

#[test]
fn a_value_too_wide_for_the_declared_field_is_a_teaching_error() {
    let err = run("DEFINE DATA LOCAL\n1 #SMALL (N3.2)\nEND-DEFINE\nMOVE 12345.67 TO #SMALL\nEND")
        .expect_err("should reject a value exceeding the integer positions");
    assert!(
        matches!(err, NaturalError::NumericOverflow { .. }),
        "expected NumericOverflow, got {err:?}"
    );
}

#[test]
fn integer_format_accepts_only_one_two_or_four_bytes() {
    // Verified against the Natural Programming Guide: format I permits lengths 1, 2, and 4.
    let err = run("DEFINE DATA LOCAL\n1 #BAD (I3)\nEND-DEFINE\nEND").expect_err("should reject I3");
    assert!(
        matches!(err, NaturalError::InvalidFormat { .. }),
        "expected InvalidFormat, got {err:?}"
    );
}

#[test]
fn numeric_precision_beyond_the_documented_maximum_is_a_teaching_error() {
    // The Programming Guide caps N and P at 29 positions.
    let err = run("DEFINE DATA LOCAL\n1 #HUGE (N28.5)\nEND-DEFINE\nEND")
        .expect_err("should reject more than 29 total positions");
    assert!(
        matches!(err, NaturalError::InvalidFormat { .. }),
        "expected InvalidFormat, got {err:?}"
    );
}

#[test]
fn an_unreadable_format_is_a_teaching_error() {
    let err = run("DEFINE DATA LOCAL\n1 #BAD (Q9)\nEND-DEFINE\nEND")
        .expect_err("should reject an unknown format letter");
    assert!(
        matches!(err, NaturalError::InvalidFormat { .. }),
        "expected InvalidFormat, got {err:?}"
    );
}

#[test]
fn text_cannot_be_moved_into_a_numeric_field() {
    let err = run("DEFINE DATA LOCAL\n1 #N (N5.2)\nEND-DEFINE\nMOVE 'abc' TO #N\nEND")
        .expect_err("should reject a type mismatch");
    assert!(
        matches!(err, NaturalError::IncompatibleAssignment { .. }),
        "expected IncompatibleAssignment, got {err:?}"
    );
}

#[test]
fn programs_without_a_data_block_still_run() {
    // DEFINE DATA is optional. A program that declares nothing is still valid.
    let out = run("WRITE 'no data block'\nEND").expect("program should run");
    assert_eq!(out.lines, vec!["no data block"]);
}
