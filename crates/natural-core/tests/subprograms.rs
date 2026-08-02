// ABOUTME: Milestone M-F acceptance tests for CALLNAT and parameter data areas: separate
// ABOUTME: objects with their own data, communicating only through their parameter list.

use natural_core::{Decimal, Library, NaturalError, Value, run_in_library};

/// A subprogram that doubles its first parameter into its second.
const DOUBLE_IT: &str = "\
DEFINE DATA PARAMETER
1 #IN (N5)
1 #OUT (N7)
END-DEFINE
COMPUTE #OUT = #IN * 2
END";

/// A subprogram that reports how many employees a country has.
const COUNT_STAFF: &str = "\
DEFINE DATA PARAMETER
1 #COUNTRY (A3)
1 #HOWMANY (N3)
END-DEFINE
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 COUNTRY
END-DEFINE
RESET #HOWMANY
FIND EMPLOYEES-VIEW WITH COUNTRY = #COUNTRY
ADD 1 TO #HOWMANY
END-FIND
END";

fn library() -> Library {
    let mut lib = Library::new();
    lib.add("DOUBLE-IT", DOUBLE_IT);
    lib.add("COUNT-STAFF", COUNT_STAFF);
    lib
}

#[test]
fn callnat_passes_values_in_and_results_back() {
    let source = "\
DEFINE DATA LOCAL
1 #VALUE (N5)
1 #RESULT (N7)
END-DEFINE
MOVE 21 TO #VALUE
CALLNAT 'DOUBLE-IT' #VALUE #RESULT
WRITE 'Doubled:' #RESULT
END";
    let out = run_in_library(source, &library(), &[]).expect("should run");
    assert_eq!(out.get("#RESULT"), Some(&Value::Number(Decimal::from(42))));
}

#[test]
fn a_subprogram_has_its_own_data_not_the_callers() {
    // The distinction module 13 exists to teach. #HIDDEN is invisible to the subprogram,
    // and the subprogram's own fields do not leak back.
    let source = "\
DEFINE DATA LOCAL
1 #VALUE (N5)
1 #RESULT (N7)
1 #IN (N5)
END-DEFINE
MOVE 5 TO #VALUE
MOVE 999 TO #IN
CALLNAT 'DOUBLE-IT' #VALUE #RESULT
WRITE 'Result:' #RESULT
WRITE 'Caller #IN untouched:' #IN
END";
    let out = run_in_library(source, &library(), &[]).expect("should run");
    assert_eq!(out.get("#RESULT"), Some(&Value::Number(Decimal::from(10))));
    // The caller's #IN happens to share a name with a subprogram parameter and must not
    // have been disturbed by it.
    assert_eq!(out.get("#IN"), Some(&Value::Number(Decimal::from(999))));
}

#[test]
fn a_subprogram_can_read_the_database() {
    let source = "\
DEFINE DATA LOCAL
1 #WHERE (A3)
1 #COUNT (N3)
END-DEFINE
MOVE 'USA' TO #WHERE
CALLNAT 'COUNT-STAFF' #WHERE #COUNT
WRITE 'USA staff:' #COUNT
END";
    let out = run_in_library(source, &library(), &[]).expect("should run");
    assert_eq!(out.get("#COUNT"), Some(&Value::Number(Decimal::from(3))));
}

#[test]
fn a_subprogram_may_be_called_repeatedly_with_different_values() {
    let source = "\
DEFINE DATA LOCAL
1 #I (I4)
1 #RESULT (N7)
1 #TOTAL (N9)
END-DEFINE
FOR #I = 1 TO 4
CALLNAT 'DOUBLE-IT' #I #RESULT
ADD #RESULT TO #TOTAL
END-FOR
WRITE 'Sum of doubles:' #TOTAL
END";
    let out = run_in_library(source, &library(), &[]).expect("should run");
    // (1+2+3+4) doubled is 20.
    assert_eq!(out.get("#TOTAL"), Some(&Value::Number(Decimal::from(20))));
}

#[test]
fn a_literal_may_be_passed_as_a_parameter() {
    let source = "\
DEFINE DATA LOCAL
1 #COUNT (N3)
END-DEFINE
CALLNAT 'COUNT-STAFF' 'UK' #COUNT
WRITE 'UK staff:' #COUNT
END";
    let out = run_in_library(source, &library(), &[]).expect("should run");
    assert_eq!(out.get("#COUNT"), Some(&Value::Number(Decimal::from(2))));
}

#[test]
fn a_subprogram_can_call_another_subprogram() {
    let mut lib = library();
    lib.add(
        "QUADRUPLE",
        "\
DEFINE DATA PARAMETER
1 #N (N5)
1 #R (N7)
END-DEFINE
DEFINE DATA LOCAL
1 #ONCE (N7)
END-DEFINE
CALLNAT 'DOUBLE-IT' #N #ONCE
CALLNAT 'DOUBLE-IT' #ONCE #R
END",
    );
    let source = "\
DEFINE DATA LOCAL
1 #V (N5)
1 #R (N7)
END-DEFINE
MOVE 3 TO #V
CALLNAT 'QUADRUPLE' #V #R
END";
    let out = run_in_library(source, &lib, &[]).expect("should run");
    assert_eq!(out.get("#R"), Some(&Value::Number(Decimal::from(12))));
}

#[test]
fn output_from_a_subprogram_appears_in_order() {
    let mut lib = Library::new();
    lib.add(
        "ANNOUNCE",
        "DEFINE DATA PARAMETER\n1 #WHO (A10)\nEND-DEFINE\nWRITE 'from the subprogram:' #WHO\nEND",
    );
    let source = "\
DEFINE DATA LOCAL
1 #NAME (A10)
END-DEFINE
WRITE 'before'
MOVE 'ADA' TO #NAME
CALLNAT 'ANNOUNCE' #NAME
WRITE 'after'
END";
    let out = run_in_library(source, &lib, &[]).expect("should run");
    assert_eq!(out.lines[0], "before");
    assert!(out.lines[1].contains("ADA"));
    assert_eq!(out.lines[2], "after");
}

// ---- the architectural case ----

#[test]
fn an_input_inside_a_subprogram_suspends_and_resumes() {
    let mut lib = Library::new();
    lib.add(
        "ASK-NAME",
        "\
DEFINE DATA PARAMETER
1 #NAME (A10)
END-DEFINE
INPUT 'Your name?' #NAME
END",
    );
    let source = "\
DEFINE DATA LOCAL
1 #WHO (A10)
END-DEFINE
CALLNAT 'ASK-NAME' #WHO
WRITE 'Hello' #WHO
END";
    let out = run_in_library(source, &lib, &["GRACE"]).expect("should run");
    assert_eq!(out.prompts, vec!["Your name?"]);
    assert_eq!(out.lines, vec!["Hello GRACE"]);
}

// ---- teaching errors ----

#[test]
fn calling_an_unknown_subprogram_is_a_teaching_error() {
    let source = "CALLNAT 'NO-SUCH-THING'\nEND";
    let err = run_in_library(source, &library(), &[]).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownSubprogram { .. }),
        "expected UnknownSubprogram, got {err:?}"
    );
}

#[test]
fn passing_the_wrong_number_of_parameters_is_a_teaching_error() {
    let source = "\
DEFINE DATA LOCAL
1 #V (N5)
END-DEFINE
CALLNAT 'DOUBLE-IT' #V
END";
    let err = run_in_library(source, &library(), &[]).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::ParameterCountMismatch { .. }),
        "expected ParameterCountMismatch, got {err:?}"
    );
}

#[test]
fn a_callnat_without_a_name_is_a_teaching_error() {
    let err = run_in_library("CALLNAT\nEND", &library(), &[]).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownStatement { .. }),
        "expected UnknownStatement, got {err:?}"
    );
}

#[test]
fn a_syntax_error_inside_a_subprogram_names_that_subprogram() {
    let mut lib = Library::new();
    lib.add(
        "BROKEN",
        "DEFINE DATA PARAMETER\n1 #X (N5)\nEND-DEFINE\nFLARP\nEND",
    );
    let source = "DEFINE DATA LOCAL\n1 #V (N5)\nEND-DEFINE\nCALLNAT 'BROKEN' #V\nEND";
    let err = run_in_library(source, &lib, &[]).expect_err("should reject");
    assert!(
        err.to_string().contains("BROKEN"),
        "the message should say which object failed, got: {err}"
    );
}
