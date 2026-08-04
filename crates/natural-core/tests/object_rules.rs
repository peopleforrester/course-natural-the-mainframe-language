// ABOUTME: Regression tests from the content audit for the rules that govern an object as a
// ABOUTME: whole: its name, its single DEFINE DATA, and how its parameters are bound.

use natural_core::{Library, NaturalError, run, run_in_library};

// ---- object names ----

#[test]
fn an_object_name_may_not_exceed_eight_characters() {
    // "The name of a Natural object can be 1 to 8 characters." A mainframe library member
    // name is eight characters wide, and the limit is that, not a style preference.
    let mut library = Library::new();
    library.add(
        "DOUBLEIT",
        "DEFINE DATA PARAMETER\n1 #V (N5)\nEND-DEFINE\nEND",
    );
    let err = run_in_library(
        "DEFINE DATA LOCAL\n1 #A (N5)\nEND-DEFINE\nCALLNAT 'DOUBLE-IT' #A\nEND",
        &library,
        &[],
    )
    .expect_err("a nine-character object name should be rejected");
    assert!(
        matches!(err, NaturalError::ObjectNameTooLong { .. }),
        "expected ObjectNameTooLong, got {err:?}"
    );
    assert!(
        err.to_string().contains("1 to 8"),
        "the message should state the rule, got: {err}"
    );
}

#[test]
fn an_eight_character_object_name_is_accepted() {
    let mut library = Library::new();
    library.add(
        "DOUBLEIT",
        "DEFINE DATA PARAMETER\n1 #V (N5)\nEND-DEFINE\nMULTIPLY #V BY 2\nEND",
    );
    let out = run_in_library(
        "DEFINE DATA LOCAL\n1 #A (N5)\nEND-DEFINE\nMOVE 21 TO #A\n\
         CALLNAT 'DOUBLEIT' #A\nWRITE #A\nEND",
        &library,
        &[],
    )
    .expect("eight characters is within the limit");
    assert!(out.lines[0].contains("42"), "got: {}", out.lines[0]);
}

#[test]
fn an_object_name_must_not_start_with_a_digit() {
    // The documented first character is an upper-case letter, a number sign, or a plus.
    let mut library = Library::new();
    library.add("2FAST", "DEFINE DATA PARAMETER\n1 #V (N5)\nEND-DEFINE\nEND");
    let err = run_in_library(
        "DEFINE DATA LOCAL\n1 #A (N5)\nEND-DEFINE\nCALLNAT '2FAST' #A\nEND",
        &library,
        &[],
    )
    .expect_err("a leading digit should be rejected");
    assert!(
        err.to_string().contains("first character"),
        "the message should name the rule that was broken, got: {err}"
    );
}

#[test]
fn a_hash_prefixed_object_name_is_allowed() {
    let mut library = Library::new();
    library.add(
        "#CALC",
        "DEFINE DATA PARAMETER\n1 #V (N5)\nEND-DEFINE\nMULTIPLY #V BY 3\nEND",
    );
    let out = run_in_library(
        "DEFINE DATA LOCAL\n1 #A (N5)\nEND-DEFINE\nMOVE 5 TO #A\n\
         CALLNAT '#CALC' #A\nWRITE #A\nEND",
        &library,
        &[],
    )
    .expect("a number sign is a documented first character");
    assert!(out.lines[0].contains("15"), "got: {}", out.lines[0]);
}

#[test]
fn a_view_name_is_not_an_object_name_and_may_be_longer() {
    // EMPLOYEES-VIEW is a variable, not a library member, so the eight-character rule has
    // nothing to say about it. Getting this wrong would break every database lesson.
    let out = run(
        "DEFINE DATA LOCAL\n1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n2 NAME\nEND-DEFINE\n\
         READ (1) EMPLOYEES-VIEW BY NAME\nWRITE NAME\nEND-READ\nEND",
    )
    .expect("a long view name is fine");
    assert_eq!(out.lines.len(), 1);
}

// ---- one DEFINE DATA per object ----

#[test]
fn an_object_may_not_open_two_define_data_blocks() {
    // PARAMETER and LOCAL are clauses of one statement, not two statements. Writing them
    // as separate blocks is the mistake this course itself shipped.
    let err = run("DEFINE DATA PARAMETER\n1 #P (N5)\nEND-DEFINE\n\
         DEFINE DATA LOCAL\n1 #L (N5)\nEND-DEFINE\nEND")
    .expect_err("a second DEFINE DATA should be rejected");
    assert!(
        matches!(err, NaturalError::RepeatedDefineData { .. }),
        "expected RepeatedDefineData, got {err:?}"
    );
    assert!(
        err.to_string().contains("END-DEFINE"),
        "the message should point at the single block, got: {err}"
    );
}

#[test]
fn parameter_and_local_are_clauses_of_one_block() {
    let mut library = Library::new();
    library.add(
        "COUNTEM",
        "DEFINE DATA\nPARAMETER\n1 #WANTED (A3)\n1 #HOWMANY (N3)\n\
         LOCAL\n1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n2 COUNTRY\nEND-DEFINE\n\
         FIND EMPLOYEES-VIEW WITH COUNTRY = #WANTED\n\
         IF NO RECORDS FOUND\nESCAPE BOTTOM\nEND-NOREC\n\
         ADD 1 TO #HOWMANY\nEND-FIND\nEND",
    );
    let out = run_in_library(
        "DEFINE DATA LOCAL\n1 #WHERE (A3)\n1 #COUNT (N3)\nEND-DEFINE\n\
         MOVE 'UK' TO #WHERE\nCALLNAT 'COUNTEM' #WHERE #COUNT\n\
         WRITE 'staff:' #COUNT\nEND",
        &library,
        &[],
    )
    .expect("one block with both clauses should compile");
    assert!(out.lines[0].contains('2'), "got: {}", out.lines[0]);
}

// ---- reserved words ----

#[test]
fn a_reserved_word_may_not_name_a_subroutine() {
    let err = run("DEFINE SUBROUTINE REPORT\nWRITE 'hello'\nEND-SUBROUTINE\n\
         PERFORM REPORT\nEND")
    .expect_err("a reserved word should be rejected as a name");
    assert!(
        matches!(err, NaturalError::ReservedWordAsName { .. }),
        "expected ReservedWordAsName, got {err:?}"
    );
}

#[test]
fn a_name_that_merely_contains_a_keyword_is_fine() {
    // REPORT-TOTALS is not REPORT. Over-eager matching here would be worse than the bug.
    let out = run(
        "DEFINE SUBROUTINE REPORT-TOTALS\nWRITE 'totals'\nEND-SUBROUTINE\n\
         PERFORM REPORT-TOTALS\nEND",
    )
    .expect("a compound name is not a reserved word");
    assert_eq!(out.lines, vec!["totals"]);
}

// ---- parameter binding ----

#[test]
fn a_parameter_must_match_the_argument_format_and_length() {
    // Parameters pass by reference, so the callee writes through to the caller's storage.
    // A2 against A3 is not a widening conversion; it is a different piece of memory.
    let mut library = Library::new();
    library.add(
        "COUNTEM",
        "DEFINE DATA PARAMETER\n1 #WANTED (A3)\nEND-DEFINE\nEND",
    );
    let err = run_in_library(
        "DEFINE DATA LOCAL\n1 #WHERE (A2)\nEND-DEFINE\nCALLNAT 'COUNTEM' #WHERE\nEND",
        &library,
        &[],
    )
    .expect_err("a length mismatch should be rejected");
    assert!(
        matches!(err, NaturalError::ParameterFormatMismatch { .. }),
        "expected ParameterFormatMismatch, got {err:?}"
    );
    assert!(
        err.to_string().contains("by reference"),
        "the message should say why the match must be exact, got: {err}"
    );
}

#[test]
fn a_matching_parameter_binds_cleanly() {
    let mut library = Library::new();
    library.add(
        "SHOWIT",
        "DEFINE DATA PARAMETER\n1 #WANTED (A3)\nEND-DEFINE\nWRITE #WANTED\nEND",
    );
    let out = run_in_library(
        "DEFINE DATA LOCAL\n1 #WHERE (A3)\nEND-DEFINE\nMOVE 'ESP' TO #WHERE\n\
         CALLNAT 'SHOWIT' #WHERE\nEND",
        &library,
        &[],
    )
    .expect("matching formats should bind");
    assert_eq!(out.lines[0].trim(), "ESP");
}
