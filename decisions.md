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
