# CLAUDE.md

Project instructions for `course-natural-the-mainframe-language`.

## What this is

An interactive, browser-based beginner course teaching the **Natural** language (the
Software AG 4GL used with ADABAS). Instructions render on the left, and a live
terminal on the right runs a **custom Natural-subset interpreter written in Rust and
compiled to WebAssembly**, entirely client-side with no backend.

## Required reading before writing code

**`docs/gotchas-rust-wasm.md` is mandatory reading before any Rust or WASM work.**
It records verified-current toolchain facts that model training data and older
tutorials get wrong. Do not write build config, npm imports, or wasm glue from
memory. The full reasoning is in `research/06-rust-wasm-toolchain.md`.

The highest-value traps, summarized (see the doc for the rest):

- xterm.js is `@xterm/xterm` 6.0.0, scoped. The unscoped `xterm` package is
  formally deprecated.
- `wasm-pack` is alive (0.15.0) after a community takeover. Install with
  `cargo install wasm-pack --locked`, not npm. Build with `--target web`.
- Target `wasm32-unknown-unknown`. Not WASI, not the component model.
- Use `rust_decimal` for arithmetic, and do **not** enable its `wasm` feature.
- Docs live at `wasm-bindgen.github.io`, not `rustwasm.github.io` (org sunset
  2025-07-21).

## Non-negotiable architecture constraint

**The interpreter's statement execution must be an explicit loop with an explicit
frame stack, never a recursive tree-walking `eval` over the Rust call stack.**

A recursive evaluator cannot be paused, and the `INPUT` statement requires the
interpreter to yield to JavaScript and resume later (the resumable state machine
chosen in spike 06 to avoid `SharedArrayBuffer` and COOP/COEP headers). Recursive
*expression* evaluation is fine, because `INPUT` only occurs at statement level.
This is not retrofittable.

## Approved contract (Phase 1.3, sha256:135613afc4c9)

Binding until amended via `/prd-amend` and re-approval:

- **v1 scope is Tier 1 only, modules 1 to 9** (`spec/tier1-lesson-outline.md`).
  Tier 2 is a later release.
- Interpreter is **Rust**, compiled to WASM, client-side against xterm.js.
- Product intent is a **revenue course** sold B2B.
- v1 is positioned as **release 1** and must **not** promise job-readiness until
  Tier 2 ships.
- Repo stays **private** through the build.
- **Do not** host the free Adabas & Natural Community Edition as the course backend.
  Its license is personal-use-only and prohibits commercial production use.

## Product requirements that are easy to mistake for implementation details

- **Runaway-loop cap.** Module 7 teaches `REPEAT`. An unbounded loop must produce a
  friendly teaching error, because this runs in the learner's browser tab.
- **Per-lesson state reset.** Module 9 teaches `STORE`/`UPDATE`. Re-running a lesson
  must yield consistent results, so the sample dataset resets per run.
- **Errors are teaching surfaces.** Diagnostics name the Natural concept ("DEFINE
  DATA must be the first statement"), not parser internals.

## Content accuracy

- Natural syntax in lessons must be verified against official documentation. Items
  marked `[verify]` in the outline are not yet confirmed verbatim and must be checked
  against the statement reference before publishing.
- Calibrate exercises against `SoftwareAG/adabas-natural-code-samples` (Apache-2.0)
  rather than inventing programs.
- The course must state plainly that the browser runs a **teaching interpreter over
  sample data**, not a live ADABAS instance.

## Working agreements

- Branch flow is `staging` first, then `main` after tests pass. This is a code repo.
- State lives in `PROJECT_STATE.md`; decisions append to `decisions.md`.
- Prose has no em-dashes, per the global prose-style rule.
