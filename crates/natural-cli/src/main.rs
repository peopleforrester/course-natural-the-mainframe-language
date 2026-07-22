// ABOUTME: Native CLI runner for the Natural teaching interpreter. Reads a .nat source
// ABOUTME: file and prints its terminal output, so the core is exercisable without a browser.

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: natural-cli <program.nat>");
        return ExitCode::from(2);
    };

    let source = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read {}: {e}", path.to_string_lossy());
            return ExitCode::from(2);
        }
    };

    match natural_core::run_to_lines(&source) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
