# Content audit, August 2026: findings and what was done

Every finding below is closed. This file is kept as the record of what was wrong and how it
was established, because the individual corrections are only meaningful next to the claims
they replaced. The process fix that came out of it is `crates/natural-core/tests/lesson_samples.rs`,
which runs every published sample through the interpreter on each build.

Audit date: 2026-08-04. Seven adversarial spikes across all 15 lessons, 56 steps,
43 code blocks, 6 exercises. Roughly 140 claims checked.

| Verdict | Count |
|---|---|
| CONFIRMED | 84 |
| REFUTED | 23 |
| MISLEADING | 23 |
| UNVERIFIED | 10 |

## The headline

Lesson 1.3 tells the learner:

> "Everything you write here is real Natural syntax, verified against the official
> documentation."

**That claim is currently false**, and it is live on a page selling training. Several
published samples would not compile on a real system. Correcting or qualifying this claim
is the first priority, ahead of every individual defect, because it is the promise the
whole course rests on.

## Tier 1: invented syntax

Verified directly against the statements reference, not only by an agent.

- **`DEFINE MAP`, `END-MAP`, `TEXT`, `FIELD` do not exist in Natural.** The DEFINE
  statements are CLASS, DATA, FUNCTION, PRINTER, PROTOTYPE, SUBROUTINE, WINDOW, and WORK
  FILE. A real map is a separate object built in the map editor and referenced by a
  quoted name. Affects every sample in lessons 14 and 15.
- `INPUT USING MAP <name>` needs a quoted constant: `INPUT USING MAP 'MAPNAME'`.

## Tier 2: will not compile

- **Object names are 1 to 8 characters.** `DOUBLE-IT` (9), `COUNT-STAFF` (11),
  `EMPLOYEE-ENTRY` (14), `RAISE-MAP` (9) all exceed it. The 32-character rule the course
  applied belongs to subroutine names, not object names.
- **One DEFINE DATA statement per object.** PARAMETER and LOCAL are clauses of a single
  statement with a single END-DEFINE. `COUNT-STAFF` uses two separate blocks (NAT0004).
- **FIND clause order is WITH, then SORTED BY, then WHERE.** Lessons 8.3 and 15.1 teach
  and demonstrate the reverse.
- **Reserved keywords cannot name subroutines.** `REPORT`, `OUTER`, `INNER` are reserved.
- **By-reference parameters must match format and length exactly.** `CALLNAT 'COUNT-STAFF'
  'UK' #HOWMANY` passes an A2 constant to an A3 parameter (NAT0936).
- **`MULTIPLY 3 BY #N` is invalid**; a constant is permitted only as operand2.
- **A system variable referenced outside its loop needs reference notation**, for example
  `*NUMBER(EMP.)` with a labelled FIND.

## Tier 3: false statements of fact

- **Lesson 9.3, the module's centerpiece.** "An uncommitted transaction is backed out when
  the program ends" is false. Program end is not a transaction boundary. With ETEOP=OFF
  (the default) the work stays pending and records stay in exclusive hold; with ETEOP=ON
  Natural commits it, the opposite of what the lesson teaches. Backout belongs to session
  end or failure.
- **Lesson 8.4.** IF NO RECORDS FOUND does not run "instead of the loop". It runs
  immediately before the loop, which is then entered exactly once with every database
  field reset. The published sample prints a stray blank line. `ESCAPE BOTTOM` inside the
  clause is the documented remedy.
- **Lesson 11.3.** Recursive PERFORM is permitted: "A subroutine may invoke itself
  (recursive subroutine)." The course teaches an interpreter cap as a language rule.
- **Lesson 14.3.** The PF3 branch is unreachable without `SET KEY PF3`; an unsensitized
  key delivers `ENTR`, so the lesson's stated outcome never occurs.
- **Lesson 2.4.** "Outputs exactly three lines" is false: Natural emits a default page
  title plus a blank line unless NOTITLE is given. Spike 07 recorded this and the
  implementation never followed it.
- **Lesson 7.3.** "Would hang a real session" overstates it. MADIO caps DBMS calls at 512,
  MAXCL caps program calls at 50 (NAT1029), and LT caps records read.
- **Lesson 14.** "Every PF3 to exit convention in mainframe software is this one field" is
  false; PF3 is an SAA/CUA convention that ISPF and CICS implement independently.
- **Lesson 14.** "Press Enter to move through them" is wrong for a 3270: Tab moves between
  fields, Enter transmits the screen.
- **Lesson 1.1.** z/VSE and BS2000 have no published end-of-maintenance date. The
  documented statement is that they are not supported from version 9.2. The AIX, Solaris,
  and HP-UX date of 2024-12-31 is confirmed, so the sentence needs splitting.
- **Lesson 6 lede.** "Natural has two output statements" is false. PRINT is a third, named
  alongside DISPLAY and WRITE in the NOTITLE scope note.
- **Lesson 6.1.** "WRITE never produces headers" is misleading twice over: Natural emits a
  default page title on every page unless NOTITLE, and WRITE without NOHDR re-emits a
  DISPLAY's headers on a page it causes.
- **Lesson 5.2.** `#AGE` holds years of service and is compared against 25 as years of
  service. The variable name contradicts its own use.

## Tier 2b: interpreter behavior that is wrong, not merely undocumented

These three are worse than a prose defect. The interpreter itself implements Natural
incorrectly, so a learner who experiments and trusts what they see is being taught the
wrong thing by the tool, not just by the text.

- **DISPLAY re-emits its column headers at the top of every page**, not once per report.
  The WRITE NOHDR description says so directly: "Without the NOHDR option, the column
  headers (if any) of the DISPLAY statement would be output on this new page." PS defaults
  to physical page size minus 1, so 23 lines on our own 24x80 grid. `interp.rs` hard-codes
  a one-shot `header_emitted` flag.
- **NONE is a required clause in both DECIDE FOR and DECIDE ON.** It is unbracketed in the
  syntax diagrams, and brackets are what mark optional elements. The lessons never say so,
  the interpreter accepts DECIDE without it, and five tests in `tests/decide.rs` pin the
  wrong behavior. A graduate writes code the real compiler rejects.
- ~~WRITE prints each field at its full defined width and ours trims instead.~~
  **Withdrawn on measurement.** WRITE already pads correctly: `NAME (A20)` occupies columns
  1 to 20, the separator sits at 21, the next field starts at 22, and a numeric takes a
  leading sign position. The finding cited `display.rs` line 104, and this crate has no
  `display.rs`. Counted as a fabricated finding alongside the two spacing reports below.

## Tier 4: internal contradictions

- Lesson 9.5's capstone prose promises "a filtered update, and a commit you can verify"
  and the code contains neither an UPDATE nor an END TRANSACTION.
- Lesson 9.3 says "the second FIND" where the sample has one FIND.
- Lesson 4.4 prose says `DIVIDE 4 INTO #N`; the code on the same step runs `DIVIDE 5`.
- Lesson 15.2 has a live logic bug: the NOREC clause without ESCAPE BOTTOM runs WRITE, ADD
  and UPDATE against a null record after announcing no match.

## Tier 5: omissions worth closing

- HISTOGRAM is missing from the database-loop list in 8.1 and taught uncategorized in 8.5.
- HISTOGRAM requires a descriptor; the lesson implies any field works.
- REINPUT is invalid in batch (NAT1109), and no WRITE or DISPLAY may run between INPUT and
  REINPUT (NAT1108). After eight lessons of WRITE this is the first trap a learner hits.
- Exclusive hold is never mentioned, though UPDATE and DELETE lock records.
- Lesson 12 is titled "Data areas" but names no data area object; LDA, GDA and PDA are all
  absent, and the GDA omission contradicts the lesson's own lede.
- Parameters pass BY REFERENCE by default. "Values in and results back" is misleading in
  the one lesson whose subject is isolation.
- `(N7.2)` style formats also cap decimals at 7; only the 29-position cap is taught.

## The sample-data divergence

Several samples declare a flat `2 SALARY` in a view of EMPLOYEES. In the real Software AG
demo file, SALARY is a multiple-value field inside the INCOME periodic group and needs
index notation. The course ships its own flat DDM, so the samples are internally valid and
run correctly here, but they will not compile unchanged against the real demo file. This
is a disclosure problem rather than a bug.

## Two false positives, for the record

Two independent agents reported "missing spaces after sentence-ending periods" in lessons
1, 3, 7, 8, 9, 12 and 14. Checked directly: every instance is a `</p><p>` or `</p><div>`
boundary that renders as a correct paragraph break. Both agents stripped tags without
substituting whitespace and then measured their own artifact. No fix required, and worth
remembering that an adversarial agent can manufacture a defect as well as find one.

## Recommended remediation order

1. **Correct the accuracy claim in lesson 1.3 immediately.** It is live and false.
2. **Tier 4**, the internal contradictions. Cheap, unambiguous, and embarrassing.
3. **Tier 3**, the false facts. Prose edits, no interpreter work except the NOREC
   semantics and the page title.
4. **Tier 2**, the syntax defects. Requires interpreter changes so the course cannot keep
   accepting what a real compiler rejects: FIND clause order, the 8-character object-name
   limit, single DEFINE DATA, reserved-word checking, parameter format matching, and
   reference notation.
5. **Tier 1**, maps. Needs a design decision, recorded below.

## The maps decision

`DEFINE MAP` cannot stay. Three options:

- **A. Real map objects.** Maps become objects in the course library, referenced as
  `INPUT USING MAP 'EMPMAP'`. The program-side syntax is then exactly real Natural. The
  map's own source format stays ours, disclosed as such, because a real map is built in a
  GUI editor and has no text form a learner would hand-write.
- **B. Drop map lessons** and teach the INPUT statement's real dynamic screen layout,
  which uses genuine x/y positioning notation.
- **C. Keep the current syntax** and relabel it loudly as course notation rather than
  Natural.

Recommend **A**, with **B** folded in as an additional lesson. A keeps the 3270 teaching
value and makes every line a learner types on the program side real, and the object
library already exists from lesson 13.
