# Agent instructions

Project instructions for `course-natural-the-mainframe-language`, loaded by every
coding agent. `CLAUDE.md` is a symlink to this file.

## What this is

An interactive, browser-based beginner course teaching the **Natural** language (the
Software AG 4GL used with ADABAS). Instructions render on the left, and a live
terminal on the right runs a **custom Natural-subset interpreter written in Rust and
compiled to WebAssembly**, entirely client-side with no backend.

## Required reading before writing code

**`docs/gotchas-rust-wasm.md` is mandatory reading before any Rust or WASM work.**
It records verified-current toolchain facts that model training data and older
tutorials get wrong. Do not write build config, npm imports, or wasm glue from
memory. The full reasoning is in `mrf-knowledge/natural-course-research/06-rust-wasm-toolchain.md` (held privately).

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

**Corollary added 2026-08-01, from `mrf-knowledge/natural-course-research/08-mainframe-emulators-3270.md` (held privately): the screen
buffer must be a first-class concept in the interpreter's execution state from the start,
not bolted on for Tier 2.** A Natural map read is a yield point exactly like `INPUT`, so
the thing being suspended is a screen, not just a line of text. Model execution state so a
pending screen can be part of it. Retrofitting this later costs as much as retrofitting
the state machine itself.

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
  The contract term is unchanged, but its justification was **reworded 2026-08-01**
  because the original reasoning was wrong. The binding Docker EULA (v2025.1) does
  **not** say "personal use only"; it actually grants use "for your internal
  production use". That phrase appears only as marketing copy on one web page, and
  appears in neither the CE Guide nor the EULA. The real bars are the
  integrated-solution clause, the no-rent and no-sublicense clause, and the
  no-distribution clause. There is no academic or education license path; every
  University Relations page is now a soft-404. The only legal route to a real
  runtime is a negotiated commercial license from a company that also sells
  competing training. This strengthens the decision to own our interpreter.

- **Confidentiality clause: do not publish Community Edition screenshots** in course
  material. The CE license carries a confidentiality obligation that constrains
  reproducing what the product shows, which matters for authoring lesson content.

## Product requirements that are easy to mistake for implementation details

- **Runaway-loop cap.** Module 7 teaches `REPEAT`. An unbounded loop must produce a
  friendly teaching error, because this runs in the learner's browser tab.
- **Per-lesson state reset.** Module 9 teaches `STORE`/`UPDATE`. Re-running a lesson
  must yield consistent results, so the sample dataset resets per run.
- **Errors are teaching surfaces.** Diagnostics name the Natural concept ("DEFINE
  DATA must be the first statement"), not parser internals.

## Terminal look and feel

Decided from `mrf-knowledge/natural-course-research/08-mainframe-emulators-3270.md` (held privately):

- Use the **`rbanffy/3270font`** webfont (BSD-3-Clause and OFL-1.1-RFN). It descends from
  the x3270 font, which was hand-copied from a physical 3270, and it is the single
  cheapest large jump in authenticity available.
- Fixed **24x80 Model 2 grid**, `scrollback: 0`, and no FitAddon, because a real 3270 does
  not scroll or reflow.
- Green and amber palettes, plus a subtle self-authored CSS CRT overlay. Do **not** vendor
  `cool-retro-term-renderer`; it is GPL-3.0.
- Add an **Operator Information Area** strip below the grid. After the font it is the most
  recognizable 3270 signal, and it turns otherwise invisible interpreter state (`X SYSTEM`
  while running, `X Protected` on a bad keystroke) into a teaching surface.
- Tier 2 maps are implemented at the **field-model** level: fields, attribute bytes,
  protected/numeric/intensified/hidden, the modified data tag, AID keys, and Read Modified
  semantics. Explicitly **skip the 3270 data stream** (SBA/SF/SFE orders, 12-bit
  addressing, EBCDIC, TN3270E), which is weeks of work invisible to a Natural programmer.
  Reference APIs worth reading: `racingmars/go3270` and `TN3270Sharp`, both MIT.

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
