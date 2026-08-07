# Adversarial fact-check: lessons 5 and 6

Audited 2026-08-04 against official Software AG documentation. Primary reference set is
Natural for Windows 9.3.3 (Statements and Programming Guide, copyright 1992-2026), which
is the current published statement reference. Older version trees were used only where a
page is absent from 9.3.3, and are labelled as such.

Verdicts are assigned per claim, not per sentence. A claim that is true of the course's
own teaching interpreter but false of Natural is marked REFUTED or MISLEADING, because
the lessons present it as how Natural behaves.

## Findings

| Lesson.Step | Claim (quoted) | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| 5.1 | `INPUT 'What is your name?' #NAME` (bare literal-then-field form) | CONFIRMED | Syntax 1 of INPUT permits a `'text'` element immediately before an operand. The reference states: "Any text string before a field will replace the field name as prompting text." The documentation carries the identical construction as its own example: `INPUT 'Text' VARI`. A second official example uses `INPUT 'Enter any PF key' /`. A real compiler accepts the course's line. Unstated but harmless: by default (session parameter IP not set to OFF) the field name itself is the prompting text, and the literal replaces it. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/input1.htm | 2026-08-04 |
| 5.1 | "The program is genuinely suspended while it waits, and resumes where it left off." | CONFIRMED | INPUT default mode is screen mode: "The INPUT statement may be used in screen, forms, or keyword/delimiter mode. ... The default mode is screen mode." Execution halts for terminal input. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/input.htm | 2026-08-04 |
| 5.2 | "Comparisons accept symbols or their mnemonic forms: = or EQ, > or GT, and so on." | CONFIRMED | The relational expression syntax lists, per operator: `EQ / = / EQUAL / EQUAL TO`; `NE / ^= / <> / NOT = / NOT EQ / NOTEQUAL / NOT EQUAL / NOT EQUAL TO`; `LT / LESS THAN / <`; `GE / GREATER EQUAL / >= / NOT < / NOT LT`; `GT / GREATER THAN / >`; `LE / LESS EQUAL / <= / NOT > / NOT GT`. The `>=` used in the code sample is documented. Note the documented long form is "LESS EQUAL" and "GREATER EQUAL", not "less than or equal to". | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_furth_lcc.htm | 2026-08-04 |
| 5.2 | IF / ELSE / END-IF structure | CONFIRMED | Structured mode syntax is `IF logical-condition [THEN] statement [ELSE statement] END-IF`. ELSE is bracketed (optional); END-IF is not bracketed (required). The lessons are in structured mode throughout, since every program opens with DEFINE DATA LOCAL. Reporting mode uses DO/DOEND instead and is out of scope for the course. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/if.htm | 2026-08-04 |
| 5.3 | "DECIDE FOR FIRST CONDITION takes the first branch whose condition is true." | CONFIRMED | "FIRST CONDITION: Processing of First Condition Only: Only the first true condition is to be processed." | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/decidefo.htm | 2026-08-04 |
| 5.3 | WHEN NONE presented as an ordinary trailing branch, with no statement that it is mandatory | MISLEADING | WHEN NONE is a REQUIRED clause. In the syntax diagram `[WHEN ANY statement]` and `[WHEN ALL statement]` are bracketed but `WHEN NONE statement` is not, and the notation reference states "Elements contained within square brackets are optional." A DECIDE FOR without WHEN NONE does not compile. The lesson's own sample includes it, so the sample is valid, but nothing tells the learner the clause is not optional. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/decidefo.htm and .../sm/synsym.htm | 2026-08-04 |
| 5.4 | `DECIDE ON FIRST VALUE OF #DAY` | CONFIRMED | Documented syntax is `DECIDE ON {FIRST|EVERY} [VALUE] [OF] op1`. Both VALUE and OF are optional keywords, so the course's fully spelled form is valid. Official example: `DECIDE ON FIRST VALUE OF *PF-KEY`. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/decideon.htm | 2026-08-04 |
| 5.4 | "A clause can list several values." (`VALUE 1, 2, 3, 4, 5`) | CONFIRMED | "You can specify one value, multiple values, or a range of values optionally preceded by one or more values. Multiple values must be separated from one another either by the input delimiter character (as specified with the session parameter ID) or by a comma. A comma must not be used for this purpose, however, if the comma is defined as decimal character (with the session parameter DC)." The default decimal character is the period, so the comma form is correct. Ranges use a colon (`VALUE 1 : 4`), which the lesson does not mention. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/decideon.htm | 2026-08-04 |
| 5.4 | NONE VALUE treated as an optional catch-all | REFUTED | The NONE clause is REQUIRED in DECIDE ON, exactly as in DECIDE FOR. The syntax diagram brackets `[ANY [VALUE] statement]` and `[ALL [VALUE] statement]` but does not bracket `NONE [VALUE] statement`. Both official examples (DECEX3, DECEX4) carry a NONE VALUE clause. The lesson's sample is valid, but the course never states the requirement, and the repo interpreter accepts DECIDE without it (see Corrections). | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/decideon.htm and .../sm/synsym.htm | 2026-08-04 |
| 5.4 | "When you are testing one field against a list of values, DECIDE ON is the clearer form." | CONFIRMED | Matches the documented purpose: "The DECIDE ON statement is used to specify multiple actions to be performed depending on the value (or values) contained in a variable," against DECIDE FOR, which decides "depending on multiple conditions (cases)." This is a pedagogical judgement resting on a correct factual base. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/decideon.htm | 2026-08-04 |
| 6 lede | "Natural has two output statements" | REFUTED | Natural has at least three output statements for report data. The DISPLAY reference states: "The statements WRITE and PRINT can be used to produce output in free (non-column) format." PRINT is a peer statement, and the WRITE reference confirms it by scope: "If the NOTITLE option is used, it applies to all DISPLAY, PRINT and WRITE statements within the same object which write data to the same report." WRITE TITLE and WRITE TRAILER are further output statements. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/display.htm and .../sm/write.htm | 2026-08-04 |
| 6.1 | "WRITE puts elements on a line separated by one blank" | MISLEADING | The one-blank separator is right, but the sentence conceals the dominant effect: each field prints at its full defined width, not its trimmed value length. "The length of the data determines the number of positions printed for each field." Measured against the documentation's own WRITEX01 output, `WRITE NAME FIRST-NAME SALARY (1:3)` places NAME (A20) in columns 1 to 20, one blank, FIRST-NAME (A20) in columns 22 to 41, one blank, then each salary in a 10-position numeric field. The course's `WRITE 'City:' #CITY 'Population:' #POP` with `#CITY (A20)` therefore renders as `City:` plus one blank plus `DERBY` padded out to 20 columns, not `City: DERBY Population: 261000`. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_output_display.htm and .../sm/write.htm | 2026-08-04 |
| 6.1 | "and never produces headers" | MISLEADING | True only of column headers, and only of the WRITE statement in isolation. Verbatim: "The WRITE statement itself does not produce any column headers." But two things do appear. First, a default page title: "Natural generates a single title line for each page resulting from a WRITE statement. This title contains the page number, the time of day, and the date," suppressible only with NOTITLE. Every official WRITE example output opens with a `Page 1 ... date time` line. Second: "Without the NOHDR option, the column headers (if any) of the DISPLAY statement would be output on this new page; with NOHDR they will not." A learner reading "never produces headers" will not predict either. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/write.htm | 2026-08-04 |
| 6.2 | "DISPLAY is column-oriented." | CONFIRMED | "The DISPLAY statement is used to specify the fields to be output on a report in column format. A column is created for each field and a field header is placed over the column." | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/display.htm | 2026-08-04 |
| 6.2 | "It generates a header from each field name, underlines it, leaves one blank line, and then prints rows." | CONFIRMED (incomplete) | Underlining and the blank line are exact: "Natural always underlines column headings and generates one blank line between the underlining and the data being displayed." The header source is a documented four-step precedence: explicit `'text'` in the DISPLAY statement, then the DEFINE DATA header, then the DDM default header for a database field, then the field name. "From each field name" is only the last resort, and holds for the lessons only because they use user-defined variables exclusively. Two omissions: the default page title line sits above the header block unless NOTITLE, and when the field is wider than the header "the heading will be centered over the column" rather than left-aligned. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/display.htm | 2026-08-04 |
| 6.2 | "Alphanumeric values sit left in their column, numerics sit right." | CONFIRMED | "The values contained in the field are left-justified for alphanumeric fields and right-justified for numeric fields." Overridable with AD=L and AD=R. The Programming Guide states it independently: "By default, values are displayed left-justified in alphanumeric fields and right-justified in numeric fields." | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/display.htm | 2026-08-04 |
| 6.3 | "the DISPLAY above sat inside a loop and still produced a single header block ... one header, then a row per record" | REFUTED | Column headers are re-output at the top of every new page of the report, not once per report. Decisive verbatim, from the NOHDR description in the WRITE reference: "The NOHDR option only takes effect if the execution of the WRITE statement causes a new page to be output. Without the NOHDR option, the column headers (if any) of the DISPLAY statement would be output on this new page; with NOHDR they will not." The page break is automatic: PS defaults to 0, and "If PS=0 is specified for the first report to be output (Report 0), the physical-device page-size minus 1 will be used." On the course's own fixed 24x80 Model 2 grid that is 23 lines, so any report past roughly 19 data rows shows a second page title and a second header block. The claim holds only because the sample loops three times. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/write.htm and .../parms/ps.htm | 2026-08-04 |
| 6.2/6.3 | Default column header for a user-defined variable is the variable name including the leading `#` | CONFIRMED | "If for a user-defined variable no header is defined in the DEFINE DATA statement, the variable name will be used as header," and the reference illustrates that rule with `DISPLAY NAME SALARY #NEW-SALARY`, a variable whose name carries the `#`. Corroborated by the ADDEX1 example output, where the name-and-value notation prints `#A:`, `#B:`, `#C:` with the `#` intact. The `#` is part of the variable name and is not stripped. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/display.htm and .../sm/add.htm | 2026-08-04 |
| 5.1 to 6.2 | Every code sample is valid Natural | CONFIRMED | All six samples compile against the documented syntax. DEFINE DATA LOCAL / END-DEFINE with level-1 entries, formats A20, N3, N1, A1, I4, A12, N4 are all valid. `FOR #I = 1 TO 3` is valid: the FOR syntax is `FOR operand1 [ [:]= | EQ | FROM ] operand2 [TO|THRU] operand3`, so the `=` is one of the permitted alternatives. MOVE, COMPUTE, WRITE, DISPLAY, IF/ELSE/END-IF, END-DECIDE and END are all correctly formed. Both DECIDE samples carry their mandatory NONE clause. Nothing here would be rejected by a real compiler. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/for.htm | 2026-08-04 |
| 5.2 | `1 #AGE (N3)` prompted with "How many years of service?" | MISLEADING (naming) | Not a syntax defect. The variable is named for age and holds length of service, and the lesson then compares it against 25. A learner copying the pattern learns to misname. Rename to `#YEARS-SERVICE`. | n/a (editorial) | 2026-08-04 |
| 6.2 | Sample relies on `#PRODUCT` repeating on every row | CONFIRMED | Identical-value suppression is off by default, so `WIDGET` prints on all three rows as the lesson implies. "IS - Identical Suppress ... Default setting: OFF." Page absent from the 9.3.3 tree, verified in the NaturalONE 9.1.2 parameter reference. | https://documentation.softwareag.com/naturalONE/natONE912/natux/parms/sp_is.htm | 2026-08-04 |

## Corrections required

Ordered by how much damage the error does to a paying learner.

**1. Lesson 6.3 is wrong as written and must be rewritten.** The section exists to teach
"one header, then a row per record," and that is not what Natural does. Headers repeat at
the top of every page. The correct teaching point is that DISPLAY emits one header block
per page rather than one per row, which is still the distinction from WRITE the module
wants, but is true. The current wording will mislead anyone who later runs a real report
longer than a screen. The repo's interpreter hard-codes the wrong behavior: `interp.rs`
carries a single `header_emitted: bool` flag set once per run (declared at line 87,
initialized at line 247, tested at line 412), so headers can never repeat. That flag needs
a page-line counter behind it, or the lesson must say plainly that the teaching
interpreter renders a single page.

**2. The mandatory NONE clause is untaught, and the interpreter contradicts Natural.**
`WHEN NONE` in DECIDE FOR and `NONE [VALUE]` in DECIDE ON are required by the syntax
diagrams. The repo's interpreter accepts DECIDE statements without them, and this is
pinned by passing tests in `crates/natural-core/tests/decide.rs`:
`first_value_stops_after_the_first_match` (lines 58 to 70),
`every_value_runs_all_matching_branches` (lines 71 to 84), `decide_on_works_with_text`
(lines 86 to 97), `decide_for_runs_the_first_true_condition` (lines 115 to 126) and the
DECIDE FOR EVERY test (lines 149 to 156) all omit the clause and expect success. There is
no `MissingWhenNone` error variant anywhere in the crate. A learner who forms the habit in
the browser will write DECIDE statements that a real compiler rejects. Either make the
clause mandatory in the parser with a teaching diagnostic that names the Natural concept,
or state the divergence explicitly in the lesson. Making it mandatory is the better
choice, because the course's own stated principle is that errors are teaching surfaces.

**3. Lesson 6.1 "never produces headers" must be narrowed to column headers.** Add that
Natural puts a default page title (page number, time of day, date) at the top of every
page of report output, WRITE included, and that NOTITLE suppresses it. This is not a
footnote: every official WRITE example output in the documentation opens with that line,
so any learner who touches real Natural sees it immediately and will believe the course
lied to them.

**4. Lesson 6.1 "separated by one blank" must state field-width padding.** The separator is
one blank, but each field occupies its full defined print width. `#CITY (A20)` consumes 20
columns whatever it holds, and a numeric field gets one extra high-order position reserved
for the sign ("One extra high-order print position is reserved for a sign when printing a
numeric field"). The interpreter trims instead: the test `write_still_emits_no_headers` in
`crates/natural-core/tests/display.rs` (lines 104 to 113) asserts that `WRITE #S` with
`#S (A5)` holding `ROW` yields exactly `"ROW"`, where Natural yields `ROW` plus two
trailing blanks. The interpreter already models the sign position correctly for DISPLAY,
so WRITE is the inconsistent one. Fix the interpreter or state the simplification.

**5. Lesson 6 lede: "Natural has two output statements" is false.** PRINT is a third, and
WRITE TITLE and WRITE TRAILER exist alongside. Rewrite as "two output statements you will
use constantly" or name PRINT and defer it.

**6. Lesson 6.2 should mention the page title and header centering.** The prose walks the
learner through the header block line by line and omits the title line above it, and says
nothing about the header being centered over a column wider than itself. The interpreter
already centers correctly, so only the prose is behind.

**7. Lesson 5.3 and 5.4 prose should state that the NONE clause is required.** One sentence
each. This is the cheapest fix on the list and it removes the single most likely way for a
graduate to write code that will not compile.

**8. Lesson 5.2: rename `#AGE`.** It holds years of service, is prompted as years of
service, and is compared against 25 as years of service.

**9. Optional, lesson 5.4: mention the colon range form.** `VALUE 1 : 5` is documented and
appears in the official DECIDE ON example. A learner reading maintenance code will meet it,
and the module's stated purpose is to prepare them for maintenance code.

Nothing in either lesson is a syntax error. All six code samples compile against the
documented grammar. Every defect above is either a false statement about runtime behavior
or a divergence between the teaching interpreter and the real compiler.

## Sources

All Software AG, all accessed 2026-08-04. Version 9.3.3 pages are Natural for Windows
9.3.3, the current published statement and programming reference (copyright notice
1992-2026).

- INPUT Syntax 1, Dynamic Screen Layout Specification: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/input1.htm
- INPUT (operating modes): https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/input.htm
- IF: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/if.htm
- DECIDE FOR: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/decidefo.htm
- DECIDE ON: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/decideon.htm
- Syntax Symbols (bracket convention): https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/synsym.htm
- DISPLAY: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/display.htm
- WRITE: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/write.htm
- FOR: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/for.htm
- ADD (example output showing the `#` retained in variable names): https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/add.htm
- Programming Guide, Logical Condition Criteria (relational operator table): https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_furth_lcc.htm
- Programming Guide, Statements DISPLAY and WRITE (DISPLX01 to DISPLX06, WRITEX01, WRITEX02 outputs; one-space column separation): https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_output_display.htm
- Programming Guide, Column Headers: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_output_headers.htm
- Programming Guide, Page Titles, Page Breaks, Blank Lines: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_output_titles.htm
- PS, Page Size for Natural Reports: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/parms/ps.htm
- SF, Spacing Factor: https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/parms/sf.htm
- IS, Identical Suppress (not present in the 9.3.3 tree; NaturalONE 9.1.2 used): https://documentation.softwareag.com/naturalONE/natONE912/natux/parms/sp_is.htm

Repo files cited as evidence of interpreter divergence:

- /home/michael/repos/portfolio/course-natural-the-mainframe-language/crates/natural-core/src/interp.rs
- /home/michael/repos/portfolio/course-natural-the-mainframe-language/crates/natural-core/tests/decide.rs
- /home/michael/repos/portfolio/course-natural-the-mainframe-language/crates/natural-core/tests/display.rs
