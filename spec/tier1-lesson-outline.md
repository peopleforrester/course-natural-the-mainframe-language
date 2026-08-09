# Tier 1 Lesson Outline (v1 Scope, Modules 1 to 9)

Derived from the approved spec (`spec/course-spec.md`, sha256:135613afc4c9). This is
the authoring plan for v1 and, read down the right-hand column, the **interpreter
feature backlog and test plan**.

Every step in Tier 1 must be runnable in the browser WASM terminal. If a lesson step
cannot execute, it does not belong in Tier 1.

Syntax below follows the forms verified against official documentation in
`mrf-knowledge/natural-course-research/02-training-curriculum.md` (held privately). Anything not yet verified verbatim is marked
`[verify]` and must be confirmed against the statement reference before publishing.

## Interpreter capability tiers

The nine modules stack into four interpreter milestones. Each milestone is a
shippable, testable increment of the Rust core.

| Milestone | Unlocks modules | Interpreter capability |
|---|---|---|
| **M-A** Execute and print | 1, 2 | Lexer, parser, `WRITE`, string literals, `END` |
| **M-B** Data and arithmetic | 3, 4 | `DEFINE DATA LOCAL`, formats, `MOVE`, `COMPUTE`, decimals |
| **M-C** Control, input, and output | 5, 6, 7 | `INPUT` (yield and resume), `IF`, `DECIDE`, `FOR`, `REPEAT`, `ESCAPE`, `DISPLAY`, edit masks |
| **M-D** Data access | 8, 9 | `VIEW OF`, `READ`, `FIND`, `HISTOGRAM`, `STORE`/`UPDATE`/`DELETE`, `END TRANSACTION` |

---

## Module 1: Orientation

**Objective:** understand what Natural is, where it runs, and why it is still worth
learning. Establishes credibility and motivation before any syntax.

**Content (no code execution):** Natural as a 4GL built as the native language of the
ADABAS database, developed from 1975 by Peter Pagé with Margit Neumann. Avoid asserting a
precise first-release year: the commonly cited 1979 has no primary source.

**Where it runs today, corrected 2026-08-01: IBM z/OS and Linux, plus Windows and
containers.** Do not teach z/VSE, BS2000, or legacy Unix as current platforms. All three
are retired with elapsed end-of-maintenance dates (z/VSE 2023-06-30, BS2000/OSD
2023-12-31, legacy Unix 2024-12-31), and the vendor has said it will focus on Linux and
z/OS. Mentioning them as history is fine and is good context for why maintenance work
exists; presenting them as live targets is a factual error.

Who maintains it now: Software GmbH, Silver Lake owned, standalone since January 2025.

The skills gap, stated carefully: Forrester and Compuware found 23 percent mainframe staff
loss with 63 percent of those roles unfilled. Demand is real, scarcity priced, and
concentrated in state government and insurance. Do not use the acute-retirement framing
built on the Futurum figure, which was misquoted and whose report argues the opposite. The
"2050 and beyond" commitment is real but dates from 2016 and predates the standalone
split, so present it as a stated long-term agenda rather than a guarantee.

**Terminal use:** a single "run your first program" teaser at the end, to prove the
terminal is real before the learner invests attention.

**Interpreter requirements:** none beyond M-A.

---

## Module 2: Your first program

**Objective:** write, run, and understand a complete Natural program.

**Constructs:** `WRITE`, string literals, `END`, and the source/edit model.

```natural
WRITE 'Hello World!'
END
```

**Teaching points:** every program ends with `END`. `WRITE` emits a line. The
RUN versus STOW distinction (RUN executes the source; STOW compiles and catalogs
it for reuse) is explained conceptually here, because the browser interpreter
executes source directly. Be explicit that a real environment catalogs objects.

**Interpreter requirements (M-A):** tokenize quoted literals and keywords; parse a
statement sequence terminated by `END`; emit lines to the terminal. Reject a program
with no `END` with a clear teaching error.

---

## Module 3: Data and the DEFINE DATA block

**Objective:** declare variables correctly. This is the conceptual spine of Natural.

**Constructs:** `DEFINE DATA LOCAL` / `END-DEFINE`, level numbers, user variables
prefixed `#`, and formats `A`, `N`, `P`, `I`, `L`, `D`, `T` with `(A20)`, `(N7.2)`,
`(I4)` notation. Arrays `(1:10)`.

```natural
DEFINE DATA LOCAL
1 #NAME (A20)
1 #SALARY (N7.2)
1 #COUNT (I4)
END-DEFINE
WRITE 'Declared.'
END
```

**Teaching points:** `DEFINE DATA` must come first. Format letters map to real
storage semantics: `A` alphanumeric, `N` unpacked numeric, `P` packed decimal,
`I` integer, `L` logical, `D` date, `T` time.

The `(N7.2)` notation means **seven positions before the decimal point and two
after**, so nine digit positions in total. Verified against the Natural Programming
Guide, "User-Defined Variables", which defines the `nn.m` form as "`nn` represents
the number of positions before the decimal point, and `m` represents the number of
positions after the decimal point". Learners routinely misread this as seven total,
so the lesson should state it explicitly and show a value that fills the field.

Documented limits worth teaching: `N` and `P` allow at most 29 positions, and `I`
accepts only lengths 1, 2, or 4 (bytes).

**Interpreter requirements (M-B):** parse the data block and level numbers; build a
symbol table with typed slots; enforce that `DEFINE DATA` precedes executable
statements; implement format parsing including precision. Declared-but-unset
variables initialize to type-appropriate defaults (blanks for `A`, zero for numeric).

---

## Module 4: Assignment and computation

**Objective:** put values into variables and calculate with them correctly.

**Constructs:** `MOVE`, `:=`, `COMPUTE`, `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE`,
`RESET`.

```natural
DEFINE DATA LOCAL
1 #PRICE (N7.2)
1 #QTY (I4)
1 #TOTAL (N9.2)
END-DEFINE
MOVE 19.99 TO #PRICE
#QTY := 3
COMPUTE #TOTAL = #PRICE * #QTY
WRITE 'Total:' #TOTAL
END
```
`[verify]` the verbatim `COMPUTE` example against the statement reference.

**Teaching points:** `MOVE x TO y` and `y := x` are both idiomatic. Rounding and
precision follow the declared format, so `(N7.2)` truncates or rounds to two places.
This is where a business 4GL differs from float-based languages, and it is worth
dwelling on.

**Interpreter requirements (M-B):** exact fixed-point decimal arithmetic, not
floating point. Assignment with format-driven coercion and precision enforcement.
`RESET` returns a variable to its type default. Arithmetic overflow against a
declared format must raise a clear teaching error rather than silently wrapping.

---

## Module 5: Input and conditional logic

**Objective:** read a value from the learner, then branch on it.

**Amended 2026-08-01** after curriculum validation (`mrf-knowledge/natural-course-research/09-curriculum-validation.md` (held privately)).
`INPUT` was missing from Tier 1 entirely, which was a genuine defect for three reasons.
Five of seven real published Natural syllabi teach user input in the first half. The
vendor's own verified `DECIDE ON` and `DECIDE FOR` examples both open with `INPUT`. And
this project makes the resumable state machine a non-negotiable architecture constraint
*because of* `INPUT`, yet no Tier 1 lesson exercised it, which would have left the single
riskiest part of the design unproven until Tier 2.

**Constructs:** prompt-style `INPUT`, then `IF` / `THEN` / `ELSE` / `END-IF`, `DECIDE ON`,
`DECIDE FOR`.

```natural
IF #SALARY > 50000
  WRITE 'Above threshold'
ELSE
  WRITE 'At or below threshold'
END-IF
```

`DECIDE ON` selects on the value of one operand (with `VALUE`, ranges, and
`ANY`/`ALL`/`NONE VALUE`); `DECIDE FOR` evaluates independent conditions
(`WHEN`, `WHEN ANY`, `WHEN ALL`, `WHEN NONE`). Both were verified with official
examples in spike 02.

**Teaching points:** `DECIDE ON` is the readable multi-way branch that new Natural
developers under-use, and it is common in the maintenance code they will meet.

**Interpreter requirements (M-C):** comparison and logical operators with
type-correct comparison across formats; `IF`/`ELSE` nesting; `DECIDE ON` value and
range matching; `DECIDE FOR` condition evaluation with the `NONE` fallback.

Plus the first real use of the yield machinery: `INPUT` returns a `NeedsInput` step to the
caller and resumes when a line is supplied, with all execution state held in the
interpreter struct rather than on the Rust call stack. This is the lesson that proves the
architecture, so it should be built early rather than last.

---

## Module 6: Output

**Objective:** produce readable, formatted output.

**Constructs:** `DISPLAY` versus `WRITE`, `PRINT`, edit masks `(EM=...)`.

**Teaching points:** `DISPLAY` is column-oriented and generates headers
automatically; `WRITE` is free-format. This distinction confuses beginners and
matters in real report code. Edit masks control presentation of numbers and dates,
for example currency and zero-suppression.

**Interpreter requirements (M-C):** `DISPLAY` column layout with derived headers and
consistent column widths; `WRITE` free-format spacing; edit-mask formatting for
numeric and date values. This is fiddly and deserves heavy fixture-based tests, since
output formatting is exactly what a learner compares against expected results.

---

## Module 7: Loops and control

**Objective:** repeat work and exit cleanly.

**Constructs:** `FOR` / `END-FOR`, `REPEAT` with `UNTIL` / `WHILE` / `END-REPEAT`,
`ESCAPE`.

```natural
FOR #I = 1 TO 10
  WRITE 'Iteration' #I
END-FOR
```
`[verify]` verbatim `FOR` and `REPEAT` examples against the statement reference.

**Interpreter requirements (M-C):** loop constructs with an iteration guard, plus
`ESCAPE` semantics. A runaway-loop cap is required, because this runs in the
learner's browser tab and an infinite loop must produce a friendly error rather than
freeze the page. Treat that cap as a product requirement, not an implementation
detail.

---

## Module 8: Reading data

**Objective:** read from the database, the thing Natural exists to do.

**Constructs:** DDMs and `VIEW OF`, `READ`, `FIND` (with `WITH`, `WHERE`,
`SORTED BY`, `IF NO RECORDS FOUND`), `HISTOGRAM`, `AT START OF DATA`,
`AT END OF DATA`, `END-READ` / `END-FIND`, and system variables `*COUNTER`,
`*NUMBER`.

```natural
DEFINE DATA LOCAL
1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
  2 NAME
  2 FIRST-NAME
END-DEFINE
READ EMPLOYEES-VIEW BY NAME
  DISPLAY NAME FIRST-NAME
END-READ
END
```

**Data:** the classic **EMPLOYEES** and **VEHICLES** sample data, loaded from real
fixture rows (JSON or CSV) shipped with the course. This is a genuine fixture-backed
dataset, not a stubbed integration: the rows are real and the verb semantics are
honest. The course must state plainly that the browser runs a teaching interpreter
over sample data rather than a live ADABAS instance.

**Teaching points:** the DDM is the bridge between the program and the database.
`READ` is physical/logical sequence; `FIND` is a search on a descriptor. Knowing
which to reach for is a genuine skill.

**Required opening for this module, added 2026-08-01.** The lesson must open by naming
the vendor's own distinction between **database loops** and **non-database loops**
(Programming Guide, "Loop Processing"). `READ` and `FIND` are loops, and the learner has
already met `FOR` and `REPEAT` in module 7. Without stating that these are the same
concept applied to records, teaching loops first actively confuses learners, and the
curriculum validation found that this framing is the condition under which the
loops-before-database ordering is defensible at all.

**Interpreter requirements (M-D):** load fixture datasets; resolve `VIEW OF` against
a DDM definition; implement `READ` ordering (`BY` descriptor), `FIND` with predicate
and `SORTED BY`, `HISTOGRAM` over descriptor values; loop event blocks
(`AT START OF DATA`, `AT END OF DATA`); populate `*COUNTER` and `*NUMBER`; handle the
`IF NO RECORDS FOUND` path.

---

## Module 9: Writing data and transactions

**Objective:** modify data and understand transaction boundaries.

**Constructs:** `STORE`, `UPDATE`, `DELETE`, `GET`, `END TRANSACTION`,
`BACKOUT TRANSACTION`.

**Teaching points:** `END TRANSACTION` is the commit, and forgetting it is a classic
beginner bug worth teaching as a deliberate failure exercise. Hold-and-update
semantics should be explained even though the teaching interpreter does not implement
real record locking; say so explicitly rather than implying full fidelity.

**Interpreter requirements (M-D):** mutate the in-memory dataset; implement
transaction staging so `END TRANSACTION` commits and `BACKOUT TRANSACTION` discards;
reset dataset state per lesson run so exercises are repeatable. Per-lesson state
reset is a hard requirement, since a learner re-running a lesson must get consistent
results.

---

## Capstone for v1

A short end-of-Tier-1 exercise combining modules 3 to 9: read the EMPLOYEES sample
data, filter and compute with it, format a report with `DISPLAY`, and write a record
back with a committed transaction. This proves Tier 1 competence without requiring
the modularization content that lives in Tier 2.

## Curriculum amendments from validation (2026-08-01)

`mrf-knowledge/natural-course-research/09-curriculum-validation.md` (held privately) compared this outline against seven real published
sequences, including Software AG's own five-day "Natural Programming Fundamentals"
instructor-led course with its eleven topics printed in order. The curriculum holds up and
does not need rebuilding. These specific changes are required:

1. **`INPUT` joins module 5.** Applied above. The largest single defect found.
2. **The loops-versus-database ordering question is settled in this file's favor.** The
   research doc (`mrf-knowledge/natural-course-research/02-training-curriculum.md` (held privately)) puts loops at 9 and database at 7;
   this outline puts loops at 7 and database at 8. Keep this outline's order and amend the
   research doc. Raw counts across syllabi favor database-first five to two, but the split
   sorts cleanly by source type: the only two sources that are *designed beginner
   programming courses* both teach loops first, while the database-first five are
   reference manuals, an Adabas-centric course, and a scripted walkthrough. The mitigation
   in module 8 above is mandatory for this call to hold.
3. **Module 3's example must actually use its variables.** As written it declares three
   fields and uses none, which teaches declaration as ceremony. Rebuild the example around
   a program that needs the fields it declares.
4. **Arrays are under-weighted.** Software AG places arrays at position 2 of 11 in its
   fundamentals course. This outline mentions them in a parenthetical. Give arrays real
   teaching space in module 3.
5. **Error handling is under-weighted.** Software AG puts it inside the *fundamentals*
   course and on the certification blueprint. This outline files it under "advanced" in
   module 16. Promote it into Tier 1 or state deliberately why not.
6. **Maps move from module 10 to module 14** in the full sequence, after modularization,
   matching the vendor's placement of user interface work at 10 of 11. This costs nothing
   now because 3270 maps are already outside v1 scope.

**Scope check, and a validation of the contract term.** Tier 1 covers roughly 55 to 65
percent of one five-day vendor course, and Software AG requires about ten classroom days
plus project experience before certifying. The contract term forbidding any job-readiness
claim until Tier 2 ships is therefore arithmetic rather than caution.

## Cross-cutting authoring requirements

- Every lesson ships **starter code** pre-loaded in the terminal and an
  **expected-output fixture** used both to check the learner and to test the
  interpreter.
- Errors are teaching surfaces. Interpreter diagnostics must name the Natural concept
  ("DEFINE DATA must be the first statement"), not the parser internals.
- Exercises and expected outputs should be calibrated against
  `SoftwareAG/adabas-natural-code-samples` (Apache-2.0) rather than invented.
- v1 marketing must not promise job-readiness. That is a sealed contract term,
  because Tier 2 carries the modularization and data-area content that maintenance
  work actually requires.
