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

## 2026-08-01T00:00:00Z · 2.2 · Milestone M-C3: loops, ESCAPE, and the runaway cap

Added FOR, REPEAT with optional UNTIL and WHILE guards, and ESCAPE BOTTOM and TOP.
Twenty-three acceptance tests. Loops reuse the jump machinery built for IF, so no new
execution model was needed, which is what the flat-instruction-list decision was for.

ESCAPE resolves to the nearest enclosing LOOP, deliberately skipping any IF blocks it sits
inside, and its jump target is patched when that loop closes. Pending escapes are collected
on the block stack alongside the loop, so ESCAPE TOP lands on the FOR increment rather than
skipping it, which would have produced an infinite loop from correct-looking source.

The runaway-loop cap is implemented as a limit on total statements executed, not on
iterations of any particular construct. That catches a runaway of any shape, including one
assembled from jumps rather than from a REPEAT. The default is one million statements,
which is several orders of magnitude above any Tier 1 lesson while still failing in well
under a second in the browser. `with_step_limit` exists so tests stay fast and so a lesson
that genuinely needs more can ask for it.

The error message is the product requirement working as intended. It names the cause and
gives the two documented fixes: add an ESCAPE BOTTOM, or give the REPEAT an UNTIL or WHILE
condition that eventually becomes true. Verified end to end through the CLI.

## 2026-08-01T00:00:00Z · 2.2 · COMPUTE, the arithmetic verbs, and exact decimal math

Closed a real gap: milestone M-B claimed module 4 but had only implemented MOVE, the
assignment operator, and RESET. COMPUTE and the arithmetic verbs were missing. Twenty-one
acceptance tests added.

The expression evaluator recurses over an expression tree, and that is deliberate and
allowed. The no-Rust-recursion constraint governs STATEMENT execution, because a suspension
can occur between statements. It can never occur in the middle of evaluating an expression,
so recursive descent for parsing and recursive evaluation for arithmetic are both fine. The
constraint is now stated that precisely in the code, so a future reader does not
over-apply it.

Rounding follows the verified rows from the output-formatting spike. Truncation toward zero
is the default (row E1), ROUNDED rounds away from zero when the first discarded digit is
five or more (rows E2 and E4). A test pins each.

One test earns its place beyond coverage: `arithmetic_is_exact_base_ten_not_floating_point`
asserts that 0.1 + 0.2 stores exactly 0.30. In binary floating point that is
0.30000000000000004. This is the property that makes the language usable for money, and it
is the reason `rust_decimal` was chosen over an f64 shortcut.

ADD, SUBTRACT, MULTIPLY, and DIVIDE desugar to COMPUTE over the same target. Worth noting
the direction trap: `DIVIDE 4 INTO #N` divides the TARGET by 4, and `MULTIPLY #N BY 3`
names its target first while the others name it last.

Verified end to end with an invoice program: three line items, a running total, an
8.25 percent tax computed with ROUNDED, and a final sum. Every figure is exact.

## 2026-08-01T00:00:00Z · 2.2 · DECIDE ON and DECIDE FOR complete module 5

Added both multi-way branches with their FIRST and EVERY variants, seventeen tests.

Each clause compiles to a run of IfTrueJump tests that land on the clause body, followed by
a Jump to the next clause. The body start is known before the tests are emitted, because
the number of tests equals the number of values in the clause, which is what lets the whole
construct compile in a single forward pass with no backpatching of body positions.

The FIRST and EVERY difference falls out of one line: under FIRST a matched branch emits a
jump to END-DECIDE when the next clause opens, and under EVERY it does not, so control
falls through into the next clause's tests. No second code path.

Mixing VALUE and WHEN is a frequent beginner slip, so it is detected rather than silently
misparsed, and the diagnostic names the keyword the block actually expects.

Verified with a grading program that chains both forms: a DECIDE FOR ladder assigns a
letter grade from a score, then a DECIDE ON with a multi-value clause branches on that
letter.

## 2026-08-01T00:00:00Z · 2.2 · DISPLAY completes module 6 and Tier 1 modules 1 to 7

Added the column-oriented report statement, twelve tests, built entirely from the rules
measured in the output-formatting spike: column width is the greater of the field's print
width and its header width, the header for a user-defined variable is its name including
the leading hash, headers are centered over a wider field, a field narrower than its header
sits at the left of the column, alphanumeric values are left-justified and numeric values
right-justified inside the field, columns are underlined with hyphens that do not span the
gaps, and exactly one blank line separates the underline from the data.

Two implementation notes:

- The interpreter gained a `pending_output` queue. One DISPLAY produces four lines on its
  first execution, and `step` returns one line at a time, so the queue is drained before
  any further statement runs. That also guarantees output produced before a suspension is
  delivered before the suspension is reported.
- Headers are emitted once per report rather than once per row, which is what makes a
  DISPLAY inside a loop produce the expected shape. Natural fixes headers from the first
  DISPLAY at compile time; this fixes them at the first DISPLAY executed, which is
  equivalent for single-report programs and is recorded as such in the code.

Three test expectations were wrong on first run and the implementation was right: I had
written two headers as left-justified when the documented rule is centered, and dropped a
column separator in the third. Fixing the tests rather than the code was the correct call
because each expectation is traceable to a measured documentation example.

Header centering when the padding is ODD remains unverified. Both measured examples are
symmetric. The extra blank currently goes on the right, and that is marked in the code so
lesson fixtures avoid depending on it.

Tier 1 modules 1 through 7 are now fully executable. Verified with a quarterly sales
report: a loop computing quantities and amounts, rendered as an aligned DISPLAY table.
