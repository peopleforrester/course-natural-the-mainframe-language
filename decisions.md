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

## 2026-08-01T00:00:00Z · 2.2 · Milestone M-D1: the sample database, VIEW OF, and READ

Added the EMPLOYEES sample file, view declarations, and the READ loop. Eighteen tests, all
green on the first run.

The data is real fixture rows in `crates/natural-core/data/employees.csv`, embedded with
`include_str!` rather than read at run time, because the browser build has no filesystem.
Eight employees across four countries with the field shapes the documentation examples use,
so lesson output looks like the documentation a learner will go on to read.

Three design decisions worth recording:

- **A view binds into the same field map every other statement already uses.** When a
  cursor lands on a record, the record's values are copied into the interpreter's fields
  under their DDM names. WRITE, DISPLAY, IF, and COMPUTE then work on database fields with
  no special handling at all, which is also a fair model of what Natural does: the view is
  a buffer.
- **READ reuses the loop machinery built for FOR and REPEAT**, and reports itself as a loop
  to ESCAPE. That is not just economy of implementation. It is exactly the point module 8
  is required to open on: a database loop IS a loop, and the curriculum validation made
  that framing a condition of teaching loops before database access.
- **The database is rebuilt for every interpreter**, so per-lesson reset is structural
  rather than something a lesson author has to remember. A test asserts two runs of the
  same program produce identical output.

Field references no longer require a `#` prefix, because DDM fields do not have one. Any
word that is not a number or TRUE/FALSE is now a field reference, and an unknown name is
reported at run time as an undeclared field, which is the diagnostic a learner needs
regardless.

Verified with a US payroll report: read by name, filter on country, display four columns,
and accumulate a total.

## 2026-08-01T00:00:00Z · 2.2 · Milestone M-D2: FIND, its NOREC clause, and system variables

Added FIND with WITH, WHERE, SORTED BY, a record limit, the IF NO RECORDS FOUND clause,
and the *NUMBER and *COUNTER system variables. Nineteen tests, completing module 8.

The semantic distinction worth having built carefully: **WITH is the search the database
performs and WHERE is applied afterwards, record by record, so *NUMBER reports the WITH
count rather than the surviving count.** A test pins exactly that, because it is a real
teaching point rather than an implementation detail, and getting it backwards would teach
learners the wrong mental model of where work happens.

The NOREC clause compiles to a guard plus two jumps: a guard after FindInit that lands on
the clause when the search was empty, a jump emitted at the clause opening that skips it
when records were found, and a jump at END-NOREC that leaves the FIND once the clause has
run. Without a NOREC clause the guard simply targets the loop exit, so both shapes share
one code path.

System variables are registered as ordinary numeric fields, so WRITE, DISPLAY, and IF need
no special case for them.

One bug worth recording because the class of it will recur. The FIND header parser located
its clauses by index over a word list built with `filter_map`, which silently DROPS text
literals and therefore shifted every index. `WITH NAME = 'JONES'` parsed as a malformed
condition. The fix keeps an index-aligned vector where a literal is None rather than
absent, and the helper is named and documented so the next positional parser does not
repeat it. Sixteen tests failed from that single root cause.

## 2026-08-01T00:00:00Z · 2.2 · Milestone M-D3: writes, transactions, HISTOGRAM. Tier 1 complete.

Added STORE, UPDATE, DELETE, END TRANSACTION, BACKOUT TRANSACTION, and HISTOGRAM.
Twenty-five tests. The Tier 1 interpreter now executes all nine modules.

The transaction model is real, not cosmetic, because the outline requires forgetting
END TRANSACTION to be a teaching surface. Changes are made to a working copy;
END TRANSACTION copies it over the committed state and BACKOUT copies the committed state
back. `Outcome::committed()` reports the committed state only, so a lesson can check
whether the learner remembered to commit. Three tests pin the consequence: an uncommitted
STORE, UPDATE, and DELETE each leave the data untouched.

DELETE tombstones rather than removing. Removing from the middle of the record vector would
shift every later index out from under a cursor that had already resolved its record set,
which is exactly what a DELETE inside a READ loop does.

Two bugs found and fixed during the work, both worth recording:

- END TRANSACTION was being consumed by the bare END arm, terminating the program instead
  of committing, because match arms are tried in order. Seven tests failed from it. The
  guarded arm now precedes the terminator and the reason is written beside it.
- The FIND header parser's index misalignment from the previous milestone was the same
  class of bug, so both positional parsers now use the index-preserving helper.

Verified with a capstone program that exercises every module in one run: a HISTOGRAM
summary by country, a READ loop accumulating payroll and grading each employee with
DECIDE FOR, a DISPLAY report, a ROUNDED raise budget, then a FIND with WHERE that updates
the lowest-paid UK employee, commits, and re-reads to prove the change persisted.

## 2026-08-02T00:00:00Z · 2.3 · Tier 1 shipped end to end, verified in a real browser

The wasm bindings and the VTT front end are built, and the client-side architecture is
proven in the delivery vehicle rather than only in tests.

`natural-wasm` exposes the interpreter as a `NaturalSession` the page drives one step at a
time. Values cross the boundary as strings, because the Natural edit mask is this
project's formatting authority and letting JavaScript render a decimal would put the wrong
formatter in charge. A rejected INPUT value returns its diagnostic instead of ending the
run, so the page re-prompts and the lesson continues.

The front end is a split pane: lessons left, a fixed 24x80 Model 2 grid right, with
`scrollback: 0` and no fit addon because a real 3270 neither scrolls nor reflows. The
rbanffy 3270 webfont is vendored with its licence, green and amber phosphor are both
available, and an Operator Information Area strip below the grid turns interpreter state
into a visible signal (`X SYSTEM` while running, `X Program check` on failure). Every
lesson code block runs on click.

One new toolchain trap, now recorded in the gotchas doc: wasm-pack's bundled `wasm-opt` is
older than the wasm current rustc emits, so a release build fails validation with "Bulk
memory operations require bulk memory". The error suggests disabling wasm-opt, which works
but ships a larger module. The right fix is to enable the features in
`[package.metadata.wasm-pack.profile.release]`.

Verification performed, three passes:

1. Local gate green; 223 tests pass.
2. Clean rebuild from an empty target directory: 223 tests, zero failures, release build
   and wasm build both clean. The wasm module is 178 KB.
3. In a headless browser against the freshly built artifact: all 30 lesson code blocks
   execute, the two deliberate teaching failures fail correctly and none is silently
   empty; INPUT suspends, accepts a value, rejects a bad one with a teaching error and
   re-prompts, then resumes; the runaway cap stops a million-statement loop in 382 ms with
   the page still responsive; and three invariants survive the wasm boundary intact,
   namely uncommitted work being discarded, 0.1 + 0.2 storing exactly 0.30, and N7.2
   printing in eleven positions with its reserved sign position.
