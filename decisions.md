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

## 2026-07-31T00:00:00Z · 2.3 · Milestone M-B complete (data and assignment)

Added the DEFINE DATA block, format and length parsing, the field environment, and
assignment through MOVE, the := operator, and RESET. Twenty-two acceptance tests written
before implementation. Adopted `thiserror` for the error enum per the Rust house rule,
and `rust_decimal` for exact base-10 arithmetic per spike 06.

Corrected a factual error in the approved lesson outline. It stated that `(N7.2)` means
seven digits in total. The Natural Programming Guide, "User-Defined Variables", defines
the `nn.m` form as "`nn` represents the number of positions before the decimal point, and
`m` represents the number of positions after", so `N7.2` is nine positions. The outline
and the interpreter now both encode the documented meaning, and a regression test pins
it. Also encoded from the same source: N and P allow at most 29 positions, and I accepts
only lengths 1, 2, and 4.

Caught and fixed a violation of the non-negotiable architecture constraint during
implementation. Statements that produce no output initially called `step` recursively to
reach the next observable effect, which is Rust call-stack recursion in statement
execution. It is now an explicit loop, so the interpreter stays pausable for INPUT.

`WRITE` of a variable is deliberately not implemented yet and returns a clear
NotYetSupported diagnostic. Printing a field depends on Natural's default output
formatting rules, which are being verified in spike 07 rather than invented, because
every lesson's expected-output fixture will depend on them.

Verified: 31 tests pass, clippy clean under -D warnings, fmt clean, and the wasm build
now succeeds with `rust_decimal` in the tree, settling that open question.

## 2026-08-01T00:00:00Z · 2.3 · Adversarial verification pass; factual corrections applied

Michael asked for the research to be re-run and verified as factually as possible. Eight
agents ran: five adversarial fact-checks instructed to REFUTE rather than confirm, plus
three new spikes (07 output formatting, 08 mainframe emulators and 3270, 09 curriculum
validation). Verdict tables are in `research/verification/`.

Material errors found and corrected, worst first:

1. **Supported platforms were wrong.** z/VSE, BS2000/OSD, and legacy Unix are retired with
   elapsed end-of-maintenance dates (2023-06-30, 2023-12-31, 2024-12-31). Current
   platforms are z/OS and Linux, plus Windows and containers. The course would have taught
   three dead platforms as live.
2. **"Near-total competition gap" was false.** At least six live independent providers sell
   Natural training, Software AG's own modules are free with a badge, and a German
   textbook exists (ISBN 978-3-86541-994-1) that English-only searching missed. The
   differentiator is now format and quality, not novelty. Udemy and Pluralsight were never
   actually checked and are marked UNKNOWN, because a failed fetch had been written up as
   evidence of absence.
3. **The Futurum 79 percent statistic was misquoted and its report argues the opposite.**
   Withdrawn. Forrester and Compuware 23/63 verified to the primary release and stand.
4. **The Community Edition licensing ARGUMENT was wrong; the conclusion survives.** The
   binding EULA grants use "for your internal production use" and never says "personal use
   only". The real bars are the integrated-solution, no-rent/no-sublicense, and
   no-distribution clauses, plus a confidentiality clause that also constrains publishing
   CE screenshots in lesson content. No academic license path exists. The contract term is
   unchanged and the case for owning our interpreter is stronger.
5. **"First released 1979" has no primary source.** Development is documented from 1975
   under Peter Page with Margit Neumann.
6. **Toolchain corrections:** `thread_local` was itself deprecated, so the gotchas doc was
   pointing at a dead API; `thread_local_v2` is correct. The rustwasm sunset post told
   users to fork wasm-pack and gloo rather than transferring them. JSPI shipped unflagged
   in Firefox 153, leaving Safari as the sole blocker, which does not change the
   architecture. `segeljakt/xterm-js-rs` is a trap: it binds xterm.js 4.x while we target
   6.0.0.

Curriculum: validated against seven real published sequences including Software AG's own
five-day course. It holds up. One genuine defect, INPUT was absent from Tier 1 despite the
architecture being shaped around it, so INPUT joins module 5 and the yield machinery gets
exercised in v1. The loops-before-database ordering was upheld conditional on module 8
naming the vendor's database-loops distinction.

New architecture constraint from spike 08: the screen buffer must be first-class in the
interpreter's execution state from the start, because a map read is a yield point like
INPUT. Recorded in CLAUDE.md beside the state-machine constraint.

Contract terms are unchanged. What changed is the accuracy of the justifications behind
them, and in every case the corrected facts still support the sealed decisions.

## 2026-08-01T00:00:00Z · 2.3 · Output formatting second pass; core model survived

The numeric width rule and the reserved leading sign position were re-verified
adversarially and survived. They were confirmed verbatim in the statements reference and
the session-parameter page, corroborated by 14 character-counted measurements across 11
programs, and then stress-tested by mechanically extracting every DISPLAY underline row
from 535 documentation pages looking for a narrower numeric column. None exists. The
interpreter's formatting is therefore built on a rule that has been actively attacked and
held.

Three code changes came out of it:

- Trailing blanks at end of line are now trimmed. Padding between elements is documented
  and verified and is unchanged; end-of-line trailing blanks could not be established
  either way and most examples show them absent. Since they are invisible in a terminal,
  trimming avoids encoding an unverified behavior and keeps fixtures robust. Recorded as a
  course convention rather than a claim about Natural.
- Rejecting a bare WRITE is now confirmed correct: it is a syntax error, and the page the
  first spike reported as a 404 is in fact live.
- Logical output as blank or X moved from DERIVED to VERIFIED, stated verbatim in the
  statements reference.

One trap worth remembering: the default numeric mask must be encoded as a RULE, not as the
literal mask string `Z9`. An explicit `EM=Z9` on an N7.2 field truncates it to two digits.
This interpreter already encodes the rule, so no change was needed, but a future edit-mask
implementation could easily get it wrong.

## 2026-08-01T00:00:00Z · 2.2 · Milestone M-C1: INPUT, and the architecture bet is proven

Implemented INPUT as a resumable suspension, test-first, 16 acceptance tests. This is the
milestone that validates the whole client-side design: the interpreter now genuinely stops
mid-program, returns control to its caller, and resumes with execution state intact. Until
now the state machine was a constraint we were honoring on faith.

Design points worth recording:

- `Step` gained `NeedsInput(InputRequest)`. The payload is a struct, not a bare prompt
  string, specifically so a Tier 2 map read can grow it into a screen (fields, attributes,
  cursor) without changing the enum shape or the resume protocol. That is the screen-buffer
  constraint from spike 08 honored at the earliest point it could be.
- One INPUT may read several fields, so the interpreter suspends once per field and keeps
  its position in a `PendingInput` struct rather than on the call stack. Calling `step`
  again without supplying a value re-asks rather than skipping the field, which is the
  behavior a browser event loop will actually produce.
- Undeclared fields are rejected when INPUT is reached, not after the learner has typed
  something, so the error arrives before the wasted keystrokes.
- Input text is converted through the same `coerce` path as MOVE, so length, precision, and
  overflow rules cannot drift between the two.
- `run` now refuses a program containing INPUT with a clear InputRequired error instead of
  silently skipping the read. `run_with_input` drives the suspension for tests and lessons,
  and records every prompt so a lesson can assert that an exercise asked what it should.

Line-mode prompt rendering (literal if given, else the field name) is recorded as a course
convention rather than a Natural fact, because real Natural presents a map instead of
prompting field by field.

## 2026-08-01T00:00:00Z · 2.2 · Milestone M-C2: IF, ELSE, and block structure

Added conditional blocks, test-first, 20 acceptance tests, all green on the first run.

The design decision that matters: **blocks compile to a flat instruction list with jumps,
not to a nested statement tree.** An IF emits an `IfFalseJump` whose target is patched when
its ELSE or END-IF is reached, and an ELSE emits a `Jump` over the else branch. Open blocks
live on a parser-side stack that also produces the diagnostics for an unclosed IF, a
stray ELSE, a stray END-IF, and a second ELSE on one IF.

This keeps `step` a plain program-counter loop. Control flow becomes an assignment to the
program counter, so nothing recurses and the interpreter can still suspend anywhere,
including in the middle of a taken branch. Two tests pin that property directly: an INPUT
inside a taken branch suspends and resumes correctly, and an INPUT inside a skipped branch
never prompts at all. The same jump machinery will carry FOR and REPEAT in the next
increment, so loops need no new execution model.

Comparison requires both operands to be the same kind. Silently coercing text and numbers
would let a learner write a comparison that quietly never matches, which is worse than an
error. Alphanumeric comparison ignores trailing blanks, matching Natural's blank padding of
the shorter operand.
