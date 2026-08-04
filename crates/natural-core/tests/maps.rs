// ABOUTME: Milestone M-G acceptance tests for REINPUT validation and the 3270 map model:
// ABOUTME: fields with attributes, AID keys, and a screen as the thing being suspended.

use natural_core::{Attribute, NaturalError, Screen, Step, Value, parse_program};
use natural_core::{Interpreter, Library, run_in_library};

/// The map objects these tests show.
///
/// A map is a separate object, so the tests register them the way a lesson does rather than
/// embedding them in program source. No statement lets a program contain one.
fn maps() -> Library {
    let mut library = Library::new();
    library.add_map(
        "EMPENTRY",
        "TEXT 2 5 'EMPLOYEE MAINTENANCE'\nFIELD 5 5 'Name:' #NAME\nFIELD 6 5 'Dept:' #DEPT",
    );
    library.add_map("M", "FIELD 7 12 'Name:' #NAME");
    library.add_map("M2", "FIELD 5 5 'Name:' #NAME\nFIELD 6 5 'Dept:' #DEPT");
    library.add_map("M3", "FIELD 5 5 'Salary:' #SALARY");
    library.add_map(
        "M4",
        "FIELD 5 5 'Code:' #CODE (AD=I)\nFIELD 6 5 'PIN:' #PIN (AD=N)",
    );
    library.add_map("M5", "FIELD 3 3 'X:' #N");
    library.add_map("M6", "FIELD 5 5 'X:' #N");
    library.add_map("M7", "FIELD 5 5 'X:' #N");
    library.add_map("M8", "FIELD 5 5 'X:' #NOPE");
    library.add_map("M9", "TEXT 1 30 'TITLE'\nFIELD 3 5 'Name:' #NAME");
    library.add_map("M10", "FIELD 3 5 'Name:' #NAME");
    library.add_map("M11", "FIELD 3 5 'PIN:' #PIN (AD=N)");
    library.add_map("M12", "FIELD 5 5 'B:' #B");
    library
}

fn program(declarations: &str, body: &str) -> String {
    format!("DEFINE DATA LOCAL\n{declarations}END-DEFINE\n{body}\nEND")
}

// ---- REINPUT ----

#[test]
fn reinput_asks_again_with_a_message() {
    // A validation loop: reject the value, say why, and read it again.
    let source = program(
        "1 #AGE (N3)\n",
        "\
INPUT 'Age?' #AGE
IF #AGE < 18
REINPUT 'You must be at least 18.'
END-IF
WRITE 'Accepted:' #AGE",
    );
    let out = run_in_library(&source, &maps(), &["10", "21"]).expect("should run");
    assert_eq!(out.prompts, vec!["Age?", "Age?"]);
    assert!(
        out.lines.iter().any(|l| l.contains("at least 18")),
        "the REINPUT message should be shown, got {:?}",
        out.lines
    );
    assert!(out.lines.iter().any(|l| l.contains("Accepted")));
}

#[test]
fn reinput_is_skipped_when_the_value_is_acceptable() {
    let source = program(
        "1 #AGE (N3)\n",
        "\
INPUT 'Age?' #AGE
IF #AGE < 18
REINPUT 'Too young.'
END-IF
WRITE 'Accepted'",
    );
    let out = run_in_library(&source, &maps(), &["30"]).expect("should run");
    assert_eq!(out.prompts.len(), 1);
    assert!(!out.lines.iter().any(|l| l.contains("Too young")));
}

#[test]
fn reinput_can_loop_more_than_once() {
    let source = program(
        "1 #N (N3)\n",
        "\
INPUT 'Positive number?' #N
IF #N = 0
REINPUT 'Zero is not allowed.'
END-IF
WRITE 'Got' #N",
    );
    let out = run_in_library(&source, &maps(), &["0", "0", "5"]).expect("should run");
    assert_eq!(out.prompts.len(), 3);
}

#[test]
fn reinput_outside_an_input_is_a_teaching_error() {
    let err = run_in_library(&program("1 #N (N3)\n", "REINPUT 'nope'"), &maps(), &[])
        .expect_err("should reject");
    assert!(
        matches!(err, NaturalError::ReinputWithoutInput { .. }),
        "expected ReinputWithoutInput, got {err:?}"
    );
}

// ---- maps: the field model ----

#[test]
fn a_map_presents_a_screen_of_fields() {
    let source = "\
DEFINE DATA LOCAL
1 #NAME (A20)
1 #DEPT (A6)
END-DEFINE
INPUT USING MAP 'EMPENTRY'
WRITE 'Entered' #NAME 'in' #DEPT
END";
    let program = parse_program(source).expect("should parse");
    let mut interp = Interpreter::new(program).with_library(maps());

    let screen = match interp.step().expect("should suspend") {
        Step::NeedsScreen(screen) => screen,
        other => panic!("expected NeedsScreen, got {other:?}"),
    };

    // Protected text and unprotected input fields are distinct, which is the whole point
    // of the 3270 attribute byte.
    assert_eq!(screen.name, "EMPENTRY");
    let labels: Vec<&str> = screen
        .fields
        .iter()
        .filter(|f| f.attribute == Attribute::Protected)
        .map(|f| f.text.trim())
        .collect();
    assert!(labels.contains(&"EMPLOYEE MAINTENANCE"));
    assert!(labels.contains(&"Name:"));

    let inputs: Vec<&str> = screen
        .fields
        .iter()
        .filter(|f| f.attribute != Attribute::Protected)
        .filter_map(|f| f.bound_to.as_deref())
        .collect();
    assert_eq!(inputs, vec!["#NAME", "#DEPT"]);
}

#[test]
fn a_map_field_carries_its_row_and_column() {
    let source = "\
DEFINE DATA LOCAL
1 #NAME (A20)
END-DEFINE
INPUT USING MAP 'M'
END";
    let mut interp =
        Interpreter::new(parse_program(source).expect("should parse")).with_library(maps());
    let Step::NeedsScreen(screen) = interp.step().expect("should suspend") else {
        panic!("expected a screen");
    };
    let label = screen
        .fields
        .iter()
        .find(|f| f.text.contains("Name"))
        .unwrap();
    assert_eq!((label.row, label.column), (7, 12));
    // The entry field follows its label on the same row.
    let entry = screen.fields.iter().find(|f| f.bound_to.is_some()).unwrap();
    assert_eq!(entry.row, 7);
    assert!(entry.column > label.column);
}

#[test]
fn supplying_the_screen_fills_the_bound_fields() {
    let source = "\
DEFINE DATA LOCAL
1 #NAME (A20)
1 #DEPT (A6)
END-DEFINE
INPUT USING MAP 'M2'
WRITE #NAME #DEPT
END";
    let out = run_in_library(source, &maps(), &["HOPPER", "TECH01"]).expect("should run");
    assert_eq!(out.get("#NAME"), Some(&Value::Alpha("HOPPER".to_string())));
    assert_eq!(out.get("#DEPT"), Some(&Value::Alpha("TECH01".to_string())));
}

#[test]
fn a_numeric_map_field_is_marked_numeric() {
    let source = "\
DEFINE DATA LOCAL
1 #SALARY (N7)
END-DEFINE
INPUT USING MAP 'M3'
END";
    let mut interp =
        Interpreter::new(parse_program(source).expect("should parse")).with_library(maps());
    let Step::NeedsScreen(screen) = interp.step().expect("should suspend") else {
        panic!("expected a screen");
    };
    let entry = screen.fields.iter().find(|f| f.bound_to.is_some()).unwrap();
    assert_eq!(entry.attribute, Attribute::Numeric);
}

#[test]
fn a_map_field_can_be_intensified_or_hidden() {
    let source = "\
DEFINE DATA LOCAL
1 #CODE (A8)
1 #PIN (A4)
END-DEFINE
INPUT USING MAP 'M4'
END";
    let mut interp =
        Interpreter::new(parse_program(source).expect("should parse")).with_library(maps());
    let Step::NeedsScreen(screen) = interp.step().expect("should suspend") else {
        panic!("expected a screen");
    };
    let by_field = |name: &str| {
        screen
            .fields
            .iter()
            .find(|f| f.bound_to.as_deref() == Some(name))
            .unwrap()
            .attribute
    };
    assert_eq!(by_field("#CODE"), Attribute::Intensified);
    assert_eq!(by_field("#PIN"), Attribute::Hidden);
}

#[test]
fn the_screen_reports_its_model_two_dimensions() {
    let source = "\
DEFINE DATA LOCAL
1 #N (A5)
END-DEFINE
INPUT USING MAP 'M5'
END";
    let mut interp =
        Interpreter::new(parse_program(source).expect("should parse")).with_library(maps());
    let Step::NeedsScreen(screen) = interp.step().expect("should suspend") else {
        panic!("expected a screen");
    };
    assert_eq!((screen.rows, screen.columns), (24, 80));
}

// ---- AID keys ----

#[test]
fn the_aid_key_the_operator_pressed_is_readable() {
    // SET KEY sensitizes PF3 so it reaches the program at all. Without it the key would
    // arrive as ENTR; an_unsensitized_pf_key_arrives_as_enter covers that path.
    // *PF-KEY is how a program knows which key ended the screen, and it is the basis of
    // every "PF3 to exit" convention in mainframe software.
    let source = "\
DEFINE DATA LOCAL
1 #N (A5)
END-DEFINE
SET KEY PF3
INPUT USING MAP 'M6'
IF *PF-KEY = 'PF3'
WRITE 'Operator asked to exit.'
ELSE
WRITE 'Carrying on with' #N
END-IF
END";
    let mut interp =
        Interpreter::new(parse_program(source).expect("should parse")).with_library(maps());
    let Step::NeedsScreen(_) = interp.step().expect("should suspend") else {
        panic!("expected a screen");
    };
    interp
        .provide_screen(&[("#N".to_string(), "ABC".to_string())], "PF3")
        .expect("should accept the screen");
    let mut lines = Vec::new();
    while let Ok(step) = interp.step() {
        match step {
            Step::Output(line) => lines.push(line),
            Step::Done => break,
            _ => break,
        }
    }
    assert!(lines.iter().any(|l| l.contains("asked to exit")));
}

#[test]
fn enter_is_the_default_aid_key() {
    let source = "\
DEFINE DATA LOCAL
1 #N (A5)
END-DEFINE
INPUT USING MAP 'M7'
WRITE 'Key was' *PF-KEY
END";
    let out = run_in_library(source, &maps(), &["ZZ"]).expect("should run");
    assert!(
        out.lines.iter().any(|l| l.contains("ENTR")),
        "the default AID should be ENTER, got {:?}",
        out.lines
    );
}

#[test]
fn defining_a_map_inside_a_program_is_a_teaching_error() {
    // The DEFINE statements are CLASS, DATA, FUNCTION, PRINTER, PROTOTYPE, SUBROUTINE,
    // WINDOW and WORK FILE. A map is a separate object and never part of a program.
    let err = parse_program(
        "DEFINE DATA LOCAL\n1 #N (A5)\nEND-DEFINE\nDEFINE MAP M\nFIELD 1 1 'x' #N\nEND-MAP\nEND",
    )
    .expect_err("DEFINE MAP should be rejected");
    assert!(
        err.to_string().contains("no such statement"),
        "the message should say the statement does not exist, got: {err}"
    );
    assert!(
        err.to_string().contains("INPUT USING MAP"),
        "the message should point at how a map is actually used, got: {err}"
    );
}

#[test]
fn an_unsensitized_pf_key_arrives_as_enter() {
    // A PF key the program never asked for is delivered as ENTR, so a PF3 branch does
    // nothing until SET KEY names the key. This is the single most surprising thing about
    // AID keys and the reason a cancel button can silently do the wrong thing.
    let source = "\
DEFINE DATA LOCAL
1 #N (A5)
END-DEFINE
INPUT USING MAP 'M7'
IF *PF-KEY = 'PF3'
WRITE 'cancelled'
ELSE
WRITE 'confirmed'
END-IF
END";
    let mut interp =
        Interpreter::new(parse_program(source).expect("should parse")).with_library(maps());
    let Step::NeedsScreen(_) = interp.step().expect("should suspend") else {
        panic!("expected a screen");
    };
    interp
        .provide_screen(&[("#N".to_string(), "AB".to_string())], "PF3")
        .expect("should accept");
    let mut lines = Vec::new();
    while let Ok(step) = interp.step() {
        match step {
            Step::Output(line) => lines.push(line),
            _ => break,
        }
    }
    assert!(
        lines.iter().any(|l| l.contains("confirmed")),
        "an unsensitized PF3 should arrive as ENTR, got {lines:?}"
    );
}

// ---- teaching errors ----

#[test]
fn using_an_undefined_map_is_a_teaching_error() {
    let source = "\
DEFINE DATA LOCAL
1 #N (A5)
END-DEFINE
INPUT USING MAP 'NOSUCH'
END";
    let err = run_in_library(source, &maps(), &[]).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UnknownMap { .. }),
        "expected UnknownMap, got {err:?}"
    );
}

#[test]
fn a_map_field_bound_to_an_undeclared_variable_is_a_teaching_error() {
    let source = "\
DEFINE DATA LOCAL
1 #N (A5)
END-DEFINE
INPUT USING MAP 'M8'
END";
    let err = run_in_library(source, &maps(), &[]).expect_err("should reject");
    assert!(
        matches!(err, NaturalError::UndeclaredVariable { .. }),
        "expected UndeclaredVariable, got {err:?}"
    );
}

#[test]
fn a_map_renders_to_a_full_24_by_80_grid() {
    // The renderer the browser uses. Text lands at its declared row and column, and every
    // row is a full 80 columns because a 3270 screen does not reflow.
    let source = "\
DEFINE DATA LOCAL
1 #NAME (A10)
END-DEFINE
INPUT USING MAP 'M9'
END";
    let mut interp =
        Interpreter::new(parse_program(source).expect("should parse")).with_library(maps());
    let Step::NeedsScreen(screen) = interp.step().expect("should suspend") else {
        panic!("expected a screen");
    };
    let rendered: Vec<String> = screen.render();
    assert_eq!(rendered.len(), 24);
    assert_eq!(rendered[0].trim_end(), format!("{}TITLE", " ".repeat(29)));
    assert!(rendered[2].starts_with("    Name:"));
    assert!(
        rendered.iter().all(|l| l.chars().count() == 80),
        "every rendered row must be exactly 80 columns"
    );
}

#[test]
fn a_screen_shows_values_the_fields_already_hold() {
    let source = "\
DEFINE DATA LOCAL
1 #NAME (A10)
END-DEFINE
MOVE 'PRESET' TO #NAME
INPUT USING MAP 'M10'
END";
    let mut interp =
        Interpreter::new(parse_program(source).expect("should parse")).with_library(maps());
    let Step::NeedsScreen(screen) = interp.step().expect("should suspend") else {
        panic!("expected a screen");
    };
    assert!(
        screen.render()[2].contains("PRESET"),
        "an entry field should show its current value"
    );
}

#[test]
fn a_hidden_field_does_not_render_its_value() {
    let source = "\
DEFINE DATA LOCAL
1 #PIN (A4)
END-DEFINE
MOVE '1234' TO #PIN
INPUT USING MAP 'M11'
END";
    let mut interp =
        Interpreter::new(parse_program(source).expect("should parse")).with_library(maps());
    let Step::NeedsScreen(screen) = interp.step().expect("should suspend") else {
        panic!("expected a screen");
    };
    assert!(
        !screen.render()[2].contains("1234"),
        "a hidden field must not display its value"
    );
}

#[test]
fn a_screen_is_a_first_class_suspension_like_input() {
    // Both suspend the same machine. This is the property spike 08 required be designed
    // for from the start rather than retrofitted.
    let source = "\
DEFINE DATA LOCAL
1 #A (A5)
1 #B (A5)
END-DEFINE
INPUT 'A?' #A
INPUT USING MAP 'M12'
WRITE #A #B
END";
    let mut interp =
        Interpreter::new(parse_program(source).expect("should parse")).with_library(maps());
    assert!(matches!(interp.step().unwrap(), Step::NeedsInput(_)));
    interp.provide_input("ONE").unwrap();
    assert!(matches!(interp.step().unwrap(), Step::NeedsScreen(_)));
    interp
        .provide_screen(&[("#B".to_string(), "TWO".to_string())], "ENTR")
        .unwrap();
    let Step::Output(line) = interp.step().unwrap() else {
        panic!("expected output");
    };
    assert!(line.contains("ONE") && line.contains("TWO"));
}

#[test]
fn the_screen_type_is_reusable_outside_a_map() {
    // The renderer is plain data, so a lesson checker or the browser can build one.
    let screen = Screen::blank("TEST");
    assert_eq!(screen.rows, 24);
    assert_eq!(screen.columns, 80);
    assert_eq!(screen.render().len(), 24);
}
