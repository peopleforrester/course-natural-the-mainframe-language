// ABOUTME: Natural-level diagnostics. Every message names the Natural concept the
// ABOUTME: learner got wrong, never a parser internal, because errors are teaching surfaces.

use thiserror::Error;

/// An error a learner can act on. Variants are deliberately concept-shaped rather than
/// mirroring the interpreter's internal structure.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NaturalError {
    #[error("This program has no END statement. Every Natural program must finish with END.")]
    MissingEnd,

    #[error(
        "Line {line}: a text literal was opened with a quote but never closed. \
         Close it with a matching quote, and write a quote inside text by doubling it."
    )]
    UnterminatedString { line: usize },

    #[error("Line {line}: '{name}' is not a Natural statement this course knows yet.")]
    UnknownStatement { name: String, line: usize },

    #[error(
        "Line {line}: DEFINE DATA must be the first statement in the program. \
         Move the data block above your other statements."
    )]
    DefineDataNotFirst { line: usize },

    #[error("The DEFINE DATA block was never closed. Add END-DEFINE after your last field.")]
    MissingEndDefine,

    #[error(
        "Line {line}: '{name}' has not been declared. \
         Add it to the DEFINE DATA block before you use it."
    )]
    UndeclaredVariable { name: String, line: usize },

    #[error("Line {line}: '{name}' is already declared. Each field needs its own name.")]
    DuplicateVariable { name: String, line: usize },

    #[error("Line {line}: {detail}")]
    InvalidFormat { detail: String, line: usize },

    #[error(
        "Line {line}: the value {value} does not fit in '{name}', which allows \
         {int_digits} position(s) before the decimal point."
    )]
    NumericOverflow {
        name: String,
        value: String,
        int_digits: u32,
        line: usize,
    },

    #[error("Line {line}: cannot assign {source_kind} to '{name}', which is {target_kind}.")]
    IncompatibleAssignment {
        name: String,
        source_kind: String,
        target_kind: String,
        line: usize,
    },

    #[error("Line {line}: {feature} is not available yet in this course build.")]
    NotYetSupported { feature: String, line: usize },

    #[error(
        "The value '{text}' typed into {name} is not valid: that field holds {expected}. \
         Correct it on the screen and transmit again."
    )]
    InvalidScreenInput {
        text: String,
        name: String,
        expected: String,
    },

    #[error("Line {line}: '{text}' is not a valid value for '{name}', which holds {expected}.")]
    InvalidInput {
        text: String,
        name: String,
        expected: String,
        line: usize,
    },

    #[error("This program asks for input ({prompt}), so it cannot be run without supplying any.")]
    InputRequired { prompt: String },

    #[error("The program is not waiting for input right now.")]
    NotWaitingForInput,

    #[error("Line {line}: this IF was never closed. Add END-IF after the statements it guards.")]
    MissingEndIf { line: usize },

    #[error("Line {line}: {keyword} does not belong here. {hint}")]
    UnexpectedBlockKeyword {
        keyword: String,
        hint: String,
        line: usize,
    },

    #[error("Line {line}: cannot compare {left} with {right}. Compare like with like.")]
    IncomparableValues {
        left: String,
        right: String,
        line: usize,
    },

    #[error("Line {line}: this {keyword} was never closed. Add {closer} after its statements.")]
    MissingLoopEnd {
        keyword: String,
        closer: String,
        line: usize,
    },

    #[error("Line {line}: ESCAPE only works inside a loop. Put it inside a FOR or REPEAT block.")]
    EscapeOutsideLoop { line: usize },

    #[error(
        "This program ran more than {limit} statements without finishing, so it was stopped. \
         A REPEAT loop keeps going until something ends it: add an ESCAPE BOTTOM, or give it \
         an UNTIL or WHILE condition that eventually becomes true."
    )]
    RunawayLoop { limit: usize },

    #[error(
        "Line {line}: there is no database file called '{name}' in this course. \
         The sample data provides EMPLOYEES."
    )]
    UnknownDdm { name: String, line: usize },

    #[error("Line {line}: '{name}' is not a field of the {ddm} file.")]
    UnknownDdmField {
        name: String,
        ddm: String,
        line: usize,
    },

    #[error(
        "Line {line}: '{name}' is not a view. Declare it in DEFINE DATA with \
         VIEW OF before reading it."
    )]
    UnknownView { name: String, line: usize },

    #[error(
        "Line {line}: {statement} works on the record a READ or FIND loop is holding, and \
         no loop is active here. Put it inside the loop."
    )]
    NoCurrentRecord { statement: String, line: usize },

    #[error(
        "Line {line}: there is no subroutine called '{name}'. Define it with \
         DEFINE SUBROUTINE {name} before performing it."
    )]
    UnknownSubroutine { name: String, line: usize },

    #[error("Line {line}: a subroutine called '{name}' is already defined.")]
    DuplicateSubroutine { name: String, line: usize },

    #[error(
        "This program nested more than {limit} calls deep, which usually means a routine \
         performs itself with nothing to stop it. Check that each PERFORM eventually \
         reaches an END-SUBROUTINE without calling back into the same routine."
    )]
    CallStackTooDeep { limit: usize },

    #[error("Line {line}: there is no subprogram called '{name}' in this library.")]
    UnknownSubprogram { name: String, line: usize },

    #[error(
        "Line {line}: '{name}' expects {expected} parameter(s) but {given} were passed. \
         The call has to match the subprogram's DEFINE DATA PARAMETER block."
    )]
    ParameterCountMismatch {
        name: String,
        expected: usize,
        given: usize,
        line: usize,
    },

    #[error("In subprogram '{name}': {source_message}")]
    InSubprogram {
        name: String,
        source_message: String,
    },

    #[error(
        "Line {line}: REINPUT only works while an INPUT is being processed. It sends the \
         operator back to the screen they just filled in."
    )]
    ReinputWithoutInput { line: usize },

    #[error(
        "Line {line}: there is no map called '{name}' in this library. A map is a separate \
         object, so add it in the Library tab before a program can use it."
    )]
    UnknownMap { name: String, line: usize },

    #[error("Line {line}: cannot divide by zero.")]
    DivisionByZero { line: usize },

    #[error(
        "Line {line}: '{name}' holds {kind}, so it cannot be used in a calculation. \
         Arithmetic needs numeric fields."
    )]
    NonNumericArithmetic {
        name: String,
        kind: String,
        line: usize,
    },

    #[error(
        "Line {line}: {second} must come before {first}. The documented clause order for \
         this statement is {order}."
    )]
    ClauseOutOfOrder {
        first: String,
        second: String,
        order: String,
        line: usize,
    },

    #[error(
        "Line {line}: no loop here is labelled '{name}'. Outside the loop itself, a system \
         variable needs a label saying which loop it came from, as in *NUMBER(MYLOOP.)."
    )]
    UnknownLabel { name: String, line: usize },

    #[error(
        "Line {line}: '{name}' is {length} characters. A Natural object name is 1 to 8 \
         characters, because it has to fit a mainframe library member name."
    )]
    ObjectNameTooLong {
        name: String,
        length: usize,
        line: usize,
    },

    #[error(
        "Line {line}: an object has one DEFINE DATA statement. PARAMETER and LOCAL are \
         clauses inside it, so write them in a single block ending with one END-DEFINE."
    )]
    RepeatedDefineData { line: usize },

    #[error(
        "Line {line}: '{name}' is a Natural reserved word, so it cannot name {a_what}. \
         Pick a name that is not also a statement or clause keyword."
    )]
    ReservedWordAsName {
        name: String,
        a_what: String,
        line: usize,
    },

    #[error(
        "Line {line}: parameter {position} of '{subprogram}' is {expected}, but {actual} was \
         passed. Parameters are passed by reference, so format and length must match exactly."
    )]
    ParameterFormatMismatch {
        subprogram: String,
        position: usize,
        expected: String,
        actual: String,
        line: usize,
    },

    #[error(
        "Line {line}: this {keyword} has no {clause} clause. Natural requires one, so that \
         every possible outcome is handled even when you expect it cannot happen."
    )]
    MissingNoneClause {
        keyword: String,
        clause: String,
        line: usize,
    },

    #[error(
        "Line {line}: Natural needs a space around each arithmetic operator, so write \
         '{written}' as '{spaced}'."
    )]
    MissingOperatorSpaces {
        written: String,
        spaced: String,
        line: usize,
    },

    #[error(
        "Line {line}: {statement} cannot put its result into the constant {value}. \
         {hint}"
    )]
    ConstantAsResultField {
        statement: String,
        value: String,
        hint: String,
        line: usize,
    },
}
