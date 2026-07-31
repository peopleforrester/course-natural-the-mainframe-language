# Decisions Log

Append-only. Newest entries at the bottom. See [[state-persistence]] schema.

## 2026-07-19T00:00:00Z · 1.1 · Repo initialized for the Natural course

Created the local repo and a private GitHub repo under `peopleforrester`. The
course teaches the Natural mainframe language using the VTT model (instructions
left, terminal right) adapted from the "Unleash an Agent, Watch It Burn"
workshop. Private at first per Michael's instruction; opened only after the
spec and initial content are ready.

Decided: research lands in this repo's `research/` directory (course-specific
and voluminous) rather than only as mrf-knowledge spikes. Framework-level
findings (emulator/WASM feasibility) may also be promoted to a spike later.

## 2026-07-19T00:00:00Z · 1.2 · Course spec drafted from five research spikes

Synthesized spikes 01 to 05 into `spec/course-spec.md`. Key decisions proposed:

- Architecture: a custom Natural-subset interpreter in Rust, compiled to WASM,
  running fully client-side against xterm.js. Native CLI in a ttyd container is the
  fallback. Rejected hosting the free Community Edition (personal-use-only license)
  and a scripted fake terminal.
- Course: 16 modules in three delivery tiers (Tier 1 = modules 1 to 9, fully
  interactive in the browser).
- Corrected the founding premise: UT is a major operational USER of Natural/ADABAS,
  not a school that teaches it for credit. Positioning shifts to the enterprise and
  government skills gap.

Alternatives considered and set aside: hosting the vendor runtime per student (legal
wall), a purely scripted terminal (no durable learning). Awaiting Phase 1.3 approval
and answers to the four open questions in spec section 8 before any construction.

## 2026-07-22T18:45:25Z · 1.3 · Course spec approved, contract sealed

Approved-by: Michael@2026-07-22T18:45:25Z
Plan: `spec/course-spec.md` (sha256:135613afc4c9)

Resolved the four open questions from spec section 8:

- v1 scope: Tier 1 only (modules 1 to 9). Chose the fastest path to a live,
  fully interactive course over building all tiers up front.
- Interpreter language: Rust, with a scoped Rust-to-WASM research spike before P1
  implementation, per the new-technology research-first rule.
- Product intent: a revenue course sold B2B, not a portfolio showpiece.
- Repo visibility: private through the build.

Tension raised at approval and folded into the contract: Tier 1 alone is not
"job-ready" by spike 02's definition, since modules 9 to 14 are what produce
maintenance-ready competence. Because the buyer is purchasing job-readiness, v1 must
be sold as release 1 of a course extending into Tier 2, and must not promise
job-readiness until Tier 2 ships. Michael accepted Tier-1-first with this positioning
constraint attached rather than expanding v1 scope.

Next action: the Rust-to-WASM interpreter research spike (gates P1).

## 2026-07-22T19:30:00Z · 2.3 · Milestone M-A complete (execute and print)

Built the Rust workspace and the first interpreter milestone test-first: lexer, parser,
WRITE, quoted text literals, and END. Nine acceptance tests written before any
implementation, confirmed failing, then implemented to green.

Design decision honored from spike 06, and recorded because it is not retrofittable:
statement execution is an explicit program-counter loop in `Interpreter::step`, not a
recursive tree-walking evaluator. All execution state lives in the struct rather than on
the Rust call stack, so the interpreter can yield to JavaScript and resume when INPUT
arrives in Tier 2.

Toolchain pinned per-project via `rust-toolchain.toml` (1.97.1) rather than updating the
machine's global stable, so other repos are unaffected. Running cargo here installed and
selected 1.97.1, which independently confirmed spike 06's current-stable claim.

Verified: 9/9 tests pass, clippy clean under -D warnings, fmt clean, CLI runs a real .nat
file end to end, and the missing-END error path prints the teaching message with a
non-zero exit. `natural-core` compiles to wasm32-unknown-unknown, settling the gotchas
doc's day-one wasm check for the core crate. The `rust_decimal` wasm question remains
open until M-B introduces that dependency.

## 2026-07-31T00:00:00Z · 3.2 · Hosted CI declined; local gate is the enforcement point

Michael decided against GitHub Actions for this repo. The suite is fast, the repo is
private, and paid runner minutes buy nothing that a local run does not already give.

Lifecycle step 3.2 "Confirm CI" is therefore satisfied by `scripts/verify.sh` rather
than a hosted pipeline. This is a CONDITIONAL skip recorded with its reason, not a
silent bypass. The gate runs format, clippy under `-D warnings`, the test suite, the
wasm32-unknown-unknown build, and an em-dash scan over tracked markdown. It is wired
to a repo-local pre-push hook by `scripts/install-hooks.sh`, so nothing reaches the
remote without passing, and `--no-verify` remains the deliberate override.

The gate found real em-dashes in `reference/vtt-model/vtt-architecture.md` on its first
run, which the earlier manual scan had missed because it only covered `research/`.
Revisit this decision if the repo goes public or gains outside contributors, where
hosted CI checks pull requests from people who have not installed the hook.
