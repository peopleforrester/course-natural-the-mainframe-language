// ABOUTME: The browser boundary. Wraps the interpreter as a resumable session JavaScript
// ABOUTME: can drive one step at a time, so a suspended INPUT hands control back to the page.

use natural_core::{Interpreter, Step, parse_program};
use wasm_bindgen::prelude::*;

/// What a single advance of the program produced.
///
/// Values cross the boundary as strings rather than as numbers. The Natural edit mask is
/// this project's formatting authority, so letting JavaScript render a decimal would put
/// the wrong formatter in charge. See docs/gotchas-rust-wasm.md.
#[wasm_bindgen]
pub struct StepResult {
    kind: String,
    text: String,
    field: String,
}

#[wasm_bindgen]
impl StepResult {
    /// One of "output", "input", "done", or "error".
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.kind.clone()
    }

    /// The output line, the input prompt, or the error message.
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }

    /// For an input request, the field the value will be assigned to.
    #[wasm_bindgen(getter)]
    pub fn field(&self) -> String {
        self.field.clone()
    }
}

impl StepResult {
    fn of(kind: &str, text: String, field: String) -> Self {
        Self {
            kind: kind.to_string(),
            text,
            field,
        }
    }
}

/// A running Natural program the page drives one step at a time.
///
/// The page calls `step` until it reports "done" or "error". When it reports "input" the
/// page shows the prompt, waits for the learner, and calls `provide_input`. Nothing
/// blocks, so the browser stays responsive and no cross-origin isolation headers are
/// needed, which is the whole reason the interpreter is a state machine.
#[wasm_bindgen]
pub struct NaturalSession {
    interpreter: Option<Interpreter>,
    /// A parse failure, reported on the first step so construction cannot throw.
    parse_error: Option<String>,
}

#[wasm_bindgen]
impl NaturalSession {
    /// Compiles a program. Syntax errors surface on the first `step`.
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> NaturalSession {
        match parse_program(source) {
            Ok(program) => NaturalSession {
                interpreter: Some(Interpreter::new(program)),
                parse_error: None,
            },
            Err(error) => NaturalSession {
                interpreter: None,
                parse_error: Some(error.to_string()),
            },
        }
    }

    /// Advances to the next observable effect.
    pub fn step(&mut self) -> StepResult {
        if let Some(message) = self.parse_error.take() {
            return StepResult::of("error", message, String::new());
        }
        let Some(interpreter) = self.interpreter.as_mut() else {
            return StepResult::of("done", String::new(), String::new());
        };
        match interpreter.step() {
            Ok(Step::Output(line)) => StepResult::of("output", line, String::new()),
            Ok(Step::NeedsInput(request)) => StepResult::of("input", request.prompt, request.field),
            Ok(Step::Done) => StepResult::of("done", String::new(), String::new()),
            Err(error) => {
                // A Natural-level error ends the run, exactly as it would on a mainframe.
                self.interpreter = None;
                StepResult::of("error", error.to_string(), String::new())
            }
        }
    }

    /// Supplies the value a suspended INPUT is waiting for.
    ///
    /// Returns an empty string on success, or the diagnostic when the value is rejected,
    /// so the page can re-prompt rather than ending the lesson.
    #[wasm_bindgen(js_name = provideInput)]
    pub fn provide_input(&mut self, text: &str) -> String {
        let Some(interpreter) = self.interpreter.as_mut() else {
            return "The program is not running.".to_string();
        };
        match interpreter.provide_input(text) {
            Ok(()) => String::new(),
            Err(error) => error.to_string(),
        }
    }

    /// How many records the sample file holds after the last committed transaction.
    ///
    /// Lessons use this to check whether a learner remembered END TRANSACTION.
    #[wasm_bindgen(js_name = committedRecordCount)]
    pub fn committed_record_count(&self) -> usize {
        self.interpreter
            .as_ref()
            .map(|i| i.committed().len())
            .unwrap_or(0)
    }

    /// The value of a field, rendered exactly as Natural would print it.
    #[wasm_bindgen(js_name = fieldValue)]
    pub fn field_value(&self, name: &str) -> Option<String> {
        let interpreter = self.interpreter.as_ref()?;
        let field = interpreter.fields().get(&name.to_ascii_uppercase())?;
        Some(natural_core::render_field(&field.value, &field.format))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_session_reports_output_then_done() {
        let mut session = NaturalSession::new("WRITE 'hello'\nEND");
        let first = session.step();
        assert_eq!(first.kind(), "output");
        assert_eq!(first.text(), "hello");
        assert_eq!(session.step().kind(), "done");
    }

    #[test]
    fn a_session_suspends_for_input_and_resumes() {
        let mut session = NaturalSession::new(
            "DEFINE DATA LOCAL\n1 #N (A10)\nEND-DEFINE\nINPUT 'Name?' #N\nWRITE #N\nEND",
        );
        let request = session.step();
        assert_eq!(request.kind(), "input");
        assert_eq!(request.text(), "Name?");
        assert_eq!(request.field(), "#N");

        assert_eq!(session.provide_input("ADA"), "");
        let output = session.step();
        assert_eq!(output.kind(), "output");
        assert_eq!(output.text(), "ADA");
    }

    #[test]
    fn a_syntax_error_is_reported_rather_than_thrown() {
        let mut session = NaturalSession::new("WRITE 'no end here'");
        let result = session.step();
        assert_eq!(result.kind(), "error");
        assert!(result.text().contains("END"));
    }

    #[test]
    fn a_rejected_input_value_is_reported_without_ending_the_run() {
        let mut session = NaturalSession::new(
            "DEFINE DATA LOCAL\n1 #N (N3)\nEND-DEFINE\nINPUT #N\nWRITE #N\nEND",
        );
        session.step();
        let complaint = session.provide_input("not a number");
        assert!(!complaint.is_empty(), "a bad value should be reported");
        // The session is still alive, so the page can prompt again.
        assert_eq!(session.provide_input("42"), "");
    }

    #[test]
    fn committed_record_count_reflects_the_transaction_boundary() {
        let source = "\
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
2 NAME
END-DEFINE
MOVE 'TURING' TO NAME
STORE EMPLOYEES-VIEW
END";
        let mut session = NaturalSession::new(source);
        while session.step().kind() == "output" {}
        assert_eq!(
            session.committed_record_count(),
            8,
            "an uncommitted store must not persist"
        );
    }
}
