// ABOUTME: Runs every program in examples/, which the README tells a visitor to run. A
// ABOUTME: sample that no longer compiles is the first thing a stranger would hit.

use natural_core::{NaturalError, run_in_library};
use std::path::PathBuf;

fn examples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples")
}

#[test]
fn every_example_program_runs() {
    let dir = examples_dir();
    let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|error| panic!("could not read {}: {error}", dir.display()))
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "nat"))
        .collect();
    entries.sort();

    assert!(
        entries.len() >= 8,
        "expected the example set to still be there, found {}",
        entries.len()
    );

    let library = natural_core::Library::new();
    let mut failures = Vec::new();
    for path in &entries {
        let source = std::fs::read_to_string(path).expect("readable");
        // Some examples read input; supplying a few values proves the source compiles and
        // gets far enough to matter, and running out of them is not a defect.
        match run_in_library(&source, &library, &["1", "1", "1", "1"]) {
            Ok(_) | Err(NaturalError::InputRequired { .. }) => {}
            Err(error) => failures.push(format!(
                "  {}\n    {error}",
                path.file_name().expect("a name").to_string_lossy()
            )),
        }
    }

    assert!(
        failures.is_empty(),
        "{} example program(s) do not run:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn skip_is_the_documented_way_to_leave_a_blank_line() {
    // The diagnostic for a bare WRITE recommends SKIP, so SKIP has to exist. It did not,
    // which made that message point at a statement this interpreter did not implement.
    let out = run_in_library(
        "WRITE 'above'\nSKIP 1\nWRITE 'below'\nEND",
        &natural_core::Library::new(),
        &[],
    )
    .expect("SKIP should run");
    assert_eq!(out.lines, vec!["above", "", "below"]);
}

#[test]
fn skip_defaults_to_one_line_and_accepts_a_count() {
    let library = natural_core::Library::new();
    let one = run_in_library("WRITE 'a'\nSKIP\nWRITE 'b'\nEND", &library, &[]).expect("should run");
    assert_eq!(one.lines, vec!["a", "", "b"]);

    let three =
        run_in_library("WRITE 'a'\nSKIP 3\nWRITE 'b'\nEND", &library, &[]).expect("should run");
    assert_eq!(three.lines, vec!["a", "", "", "", "b"]);
}

#[test]
fn a_bare_write_still_points_the_learner_at_skip() {
    let err = run_in_library("WRITE 'a'\nWRITE\nEND", &natural_core::Library::new(), &[])
        .expect_err("a bare WRITE is rejected");
    assert!(
        err.to_string().contains("SKIP"),
        "the message should name SKIP, got: {err}"
    );
}
