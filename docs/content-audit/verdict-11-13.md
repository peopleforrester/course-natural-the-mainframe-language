# Adversarial fact-check: lessons 11, 12, 13 and the course subprogram library

Audit date: 2026-08-04. Method: every claim checked against official Software AG
Natural documentation at documentation.softwareag.com. Verbatim quotes are given
where they settle a point. Anything I could not settle from an official page is
marked UNVERIFIED rather than guessed.

Scope note: the course runs a teaching interpreter, not real Natural. Where a
sample would be rejected by a real Natural compiler, that is recorded as a defect
regardless of what the interpreter accepts, because the course sells the syntax as
Natural and learners will carry it to a real system.

## Findings

| Lesson.Step | Claim (quoted) | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| 11.1 | "DEFINE SUBROUTINE <name>" ... "END-SUBROUTINE", invoked with "PERFORM <name>" | CONFIRMED | Syntax is `DEFINE [SUBROUTINE] subroutine-name statement END-SUBROUTINE` in structured mode, `RETURN` in reporting mode. "The `DEFINE SUBROUTINE` statement is used to define a Natural subroutine. A subroutine is invoked with a `PERFORM` statement." The `SUBROUTINE` keyword is optional. Subroutine name may be up to 32 characters. | https://documentation.softwareag.com/natural/nat914unx/sm/definesu.htm | 2026-08-04 |
| 11.1 | "The definition is skipped during normal flow; only PERFORM enters it." | CONFIRMED (indirect) | Correct behavior, but I found no single verbatim sentence in the statement reference stating it. Support is structural and strong: the official First Steps tutorial places `DEFINE SUBROUTINE MARK-SPECIAL-EMPLOYEES ... END-SUBROUTINE` between `END-REPEAT` and `END`, and the documented output shows one pass with no trailing execution. The DEFINE SUBROUTINE page also states "An inline subroutine may be defined before or after the first `PERFORM` statement which references it", which is only coherent if the block is bypassed in sequential flow. Treat the claim as true; do not cite a doc sentence for it. | https://documentation.softwareag.com/natural/nat827mf/firststeps/fs-inlinesub.htm and https://documentation.softwareag.com/naturalONE/natONE912/natov/sm/definesu.htm | 2026-08-04 |
| 11.2 | "An inline subroutine shares the program data" and reads and writes the same fields | CONFIRMED | Verbatim: "An inline subroutine has access to the currently established global data area as well as to the local data area used by the invoking program." And: "An inline subroutine has access to all data fields within the object in which it is contained." Also verbatim: "No explicit parameters can be passed from the invoking program via the `PERFORM` statement to an internal subroutine." The course is right, and is right for the right reason. | https://documentation.softwareag.com/naturalONE/natONE912/natov/sm/definesu.htm and https://documentation.softwareag.com/natural/nat921mf/webhelp/natmf-webhelp/pg/pg_obj_pgm_routine.htm | 2026-08-04 |
| 11.3 | "Nesting works, and each PERFORM returns to its own caller." | CONFIRMED | Verbatim: "The invoked subroutine may contain a `PERFORM` statement to invoke another subroutine (the number of nested levels is limited by the size of the required memory)." So there is no fixed documented nesting level, only a memory bound. The separate restriction learners do hit is textual, not runtime: "An inline subroutine must not contain another `DEFINE SUBROUTINE` statement." Nesting `PERFORM` is fine; nesting `DEFINE SUBROUTINE` blocks is not. The lesson never says this and should. | https://documentation.softwareag.com/natural/nat914unx/sm/perform.htm and https://documentation.softwareag.com/naturalONE/natONE912/natov/sm/definesu.htm | 2026-08-04 |
| 11.3 | "Try to write a subroutine that performs itself and the interpreter will stop you rather than letting the program run out of stack." | REFUTED | Real Natural permits recursive PERFORM. Verbatim from the PERFORM statement reference: "A subroutine may invoke itself (recursive subroutine). If database operations are contained within an external subroutine that is invoked recursively, Natural will ensure that the database operations are logically separated." The course is presenting an interpreter limitation as a property of the Natural language. This is the defect class the brief named. | https://documentation.softwareag.com/natural/nat914unx/sm/perform.htm | 2026-08-04 |
| 11.2, 11.3 | Subroutines named `REPORT`, `OUTER`, `INNER` | MISLEADING | `REPORT`, `OUTER` and `INNER` all appear in the Natural reserved keywords list. DEFINE SUBROUTINE states "the same naming conventions apply as for user-defined variables", and the user-defined variable rules state that such a name must not be a Natural reserved keyword, with the softer guidance "To avoid any naming conflicts, you are strongly recommended not to use Natural reserved keywords as names for variables. In case of doubt, use the keyword check function of the compiler." Best case the course teaches a habit the vendor warns against; worst case the keyword check rejects it. `GATHER`, `SAY-HELLO` and `TOTAL-PAY` are clear. The `#`-prefixed variables (`#TOTAL`, `#COUNT`, `#VALUE`, `#RESULT`, `#IN`, `#OUT`) are also clear, because the `#` makes the whole name non-reserved, which is exactly why the convention exists. | https://documentation.softwareag.com/natural/nat827mf/pg/pg_keyw.htm and https://documentation.softwareag.com/naturalONE/natONE912/natov/sm/definesu.htm | 2026-08-04 |
| 11.2, 11.4 | `1 EMPLOYEES-VIEW VIEW OF EMPLOYEES` with `2 SALARY` at level 2 | MISLEADING | In the Software AG demo EMPLOYEES file, SALARY is a multiple-value field inside the periodic group INCOME, not a level-2 scalar. The official view example is `1 MYVIEW VIEW OF EMPLOYEES / 2 NAME / 2 JOB-TITLE / 2 INCOME (1:2) / 3 CURR-CODE / 3 SALARY / 3 BONUS (1:1)`, and the rule is "To reference one or more occurrences of a multiple-value field or a periodic group, you specify an 'index notation' after the field name." If the course's DDM mirrors the demo file (the field names NAME, SALARY and COUNTRY all suggest it does), `2 SALARY` plus `ADD SALARY TO #TOTAL` is not valid against it. If the course ships a deliberately flattened teaching DDM, that is defensible but must be stated, because the learner will hit this on day one against a real EMPLOYEES file. UNVERIFIED against the course's own DDM, which was not in scope. | https://documentation.softwareag.com/natural/nat913win/pg/pg_dbms_ada.htm | 2026-08-04 |
| 11.2 | `READ EMPLOYEES-VIEW` with no sequence clause; `COMPUTE ROUNDED` | CONFIRMED | Valid. "The `ALL` option is used by default if operand1 and `ALL` are omitted", and "`PHYSICAL` is the default sequence." `END-READ` is required in structured mode. The loop is opened and closed inside the subroutine, which satisfies "Any processing loop initiated within a subroutine must be closed before `END-SUBROUTINE` is issued." | https://documentation.softwareag.com/natural/nat914unx/sm/read.htm | 2026-08-04 |
| 12.1 | "Everything you have declared so far has been DEFINE DATA LOCAL: fields belonging to this program alone." | CONFIRMED | Verbatim: "Variables defined as local are used only within a single Natural object." The follow-on claim that an inline subroutine shares them is confirmed by the 11.2 row above. | https://documentation.softwareag.com/natural/nat913unx/pg/pg_obj_darea.htm | 2026-08-04 |
| 12.2 | "A subprogram declares a DEFINE DATA PARAMETER block. Those fields, in that order, are what a caller passes. Nothing else crosses between them." | CONFIRMED, with one omission | Verbatim: "These parameters are the only data available to the subprogram from the invoking object", and "A subprogram has no access to the global data area used by the invoking object." Order is confirmed: "The sequence, format and length of the parameters in the invoking object must match exactly the sequence, format and length of the `DEFINE DATA PARAMETER` structure." The one genuine exception the sentence overstates is application-independent variables: a variable defined with `DEFINE DATA INDEPENDENT` has its "content ... shared by all Natural objects executed within one application that refer to that name". A single clause is enough to be honest here. | https://documentation.softwareag.com/naturalONE/natONE912/natov/pg/pg_obj_prog.htm and https://documentation.softwareag.com/natural/nat911mf/firststeps/fs-subprog.htm and https://documentation.softwareag.com/natural/nat827mf/sm/defineda_aiv.htm | 2026-08-04 |
| 12.2, 12.3 | "A subprogram cannot see the caller's other fields, even ones with the same name." | CONFIRMED | Verbatim: "the names of the variables in the invoking object and the invoked subprogram need not be the same as the parameter data are transferred by address, not by name", and "The names of the variables in the invoking object and the invoked subprogram may be different." The 12.3 demonstration program, with `#IN` local to the caller and `#IN` as DOUBLE-IT's first parameter, is a correct illustration. The caller's `#IN` does stay at 999. | https://documentation.softwareag.com/natural/nat911unx/sm/callnat.htm | 2026-08-04 |
| 12 (whole) | Lesson titled "Data areas" | MISLEADING | The title overpromises and the omission is material, not cosmetic. In Natural a data area is a separate cataloged object: "A separate data area is a Natural object that can be used by multiple Natural programs, subprograms, subroutines, helproutines or classes." There are three kinds, LDA, GDA and PDA, referenced as `DEFINE DATA LOCAL USING <lda>`, `PARAMETER USING <pda>`, `GLOBAL USING <gda>`. The lesson covers only inline LOCAL and PARAMETER clauses and never names a data area object. Two concrete consequences. First, the docs call the separate-object form the recommended practice: "For a clear application structure and for easier maintainability, it is usually better to define fields in data areas outside the programs", so the course teaches the minority form as if it were the norm. Second, GDA is the direct counterexample to this lesson's own lede, "Where a field lives decides who can see it": a GDA is shared mutable state across a whole calling hierarchy, it is pervasive in the legacy Natural code these learners will meet, and its rule is quotable: "Each Natural object can reference only one GDA; that is, a `DEFINE DATA` statement must not contain more than one `GLOBAL` clause." A learner finishing this lesson believes Natural has exactly two data scopes. It has more. | https://documentation.softwareag.com/natural/nat913unx/pg/pg_obj_darea.htm | 2026-08-04 |
| 12.2, 13.1 | "CALLNAT passes values in and results back" | MISLEADING | The description implies copy-in and copy-out. Natural's default is neither. Verbatim: "By default, the parameters are passed _by reference_, that is, the data are transferred via address parameters, the parameter values themselves are not moved." Every by-reference parameter is therefore two-way: the subprogram writes directly into the caller's storage, and there is no in-only or out-only distinction unless the author creates one. This matters here more than in most courses, because lesson 12 exists specifically to teach isolation. The isolation is real for non-parameter fields and absent for parameters, and the course states only the first half. The options the course never mentions are `BY VALUE` ("Formats and lengths of the variables in the invoking object and the subprogram may be different; however, they have to be data transfer compatible"), `BY VALUE RESULT`, `OPTIONAL`, and the `AD=O`/`AD=M`/`AD=A` markers. | https://documentation.softwareag.com/natural/nat914unx/sm/callnat.htm and https://documentation.softwareag.com/natural/nat912unx/sm/defineda_pda.htm | 2026-08-04 |
| 13.1 | `CALLNAT 'DOUBLE-IT' #VALUE #RESULT` including the quoting of the subprogram name | CONFIRMED as to quoting, REFUTED as to the name | The quoting form is right. Syntax is `CALLNAT operand1 [USING] operand2 ...`, and the name is "specified either as a constant of 1 to 8 characters, or ... as an alphanumeric variable of length 1 to 8", with official examples in apostrophes such as `CALLNAT 'CNTEX1N'`. The name itself is invalid. See the next row. | https://documentation.softwareag.com/natural/nat914unx/sm/callnat.htm | 2026-08-04 |
| 12.2, 12.3, 13.1, 13.2, 13.4, library | Subprogram names `DOUBLE-IT` (9 characters) and `COUNT-STAFF` (11 characters) | REFUTED | Both exceed the hard limit on Natural object names. Verbatim: "The name of a Natural object can be 1 to 8 characters", first character an upper-case letter, `#` or `+`. CALLNAT repeats the same bound independently: operand1 is "a constant of 1 to 8 characters, or ... an alphanumeric variable of length 1 to 8". Note the trap the course has fallen into: subroutine names may be up to 32 characters, so `SAY-HELLO` and `TOTAL-PAY` in lesson 11 are fine, but a subprogram is a library object and gets the 8-character rule. The course applies the subroutine rule to subprograms. This is the highest-reach defect in the set: it invalidates every CALLNAT sample in lessons 12 and 13, both library objects, and the 13.4 exercise. | https://documentation.softwareag.com/natural/nat913win/using/use_rules.htm and https://documentation.softwareag.com/natural/nat914unx/sm/callnat.htm | 2026-08-04 |
| 13.2 | `CALLNAT 'COUNT-STAFF' 'UK' #HOWMANY` | REFUTED | Passing the alphanumeric constant `'UK'` (A2) to a parameter declared `1 #COUNTRY (A3)` breaks the by-reference matching rule. Verbatim: "If parameters are passed _by reference_ ... The sequence, format and length of the parameters in the invoking object must match exactly the sequence, format and length of the `DEFINE DATA PARAMETER` structure." A2 is not A3, so this raises NAT0936, "Format/length conflict in parameter :1: (:2: :3:/:4:/:5:/:6:)." Passing a constant is otherwise legal ("If operand2 is a constant, AD cannot be explicitly specified. For constants AD=O always applies"), and COUNT-STAFF only reads `#COUNTRY`, so AD=O is not the problem. The length is. The preceding line in the same sample, `MOVE 'USA' TO #WHERE`, is fine, because MOVE converts and CALLNAT does not. This is a sample that teaches correct usage and does not run. | https://documentation.softwareag.com/natural/nat911unx/sm/callnat.htm and https://documentation.softwareag.com/natural/nat841unx/mc/mcERRN_0901.htm | 2026-08-04 |
| 13.3 | "The call has to match the subprogram's parameter block" and a mismatch is an error | CONFIRMED | `CALLNAT 'DOUBLE-IT' #VALUE` against a two-field parameter block is a genuine Natural error: NAT0935, "Conflicting number of parameters (:1: :2:/:3:/:4:/:5:)." The compiler can catch it ahead of runtime when PCHECK is on: "the compiler will check the number, format, length and array index bounds of the parameters." The escape hatch the lesson could name is `OPTIONAL`: "A parameter that is to be skipped must be defined with the keyword `OPTIONAL` in the subprogram's `DEFINE DATA PARAMETER` statement." Since DOUBLE-IT declares no OPTIONAL, the lesson's deliberate error works as intended. | https://documentation.softwareag.com/natural/nat841unx/mc/mcERRN_0901.htm and https://documentation.softwareag.com/natural/nat911unx/sm/callnat.htm | 2026-08-04 |
| library, COUNT-STAFF | A subprogram declaring a `DEFINE DATA PARAMETER ... END-DEFINE` block followed by a separate `DEFINE DATA LOCAL ... END-DEFINE` block | REFUTED | Two DEFINE DATA statements in one object is a compile error. Verbatim: "When a `DEFINE DATA` statement is used, it must be the first statement of the program/routine", enforced as NAT0004, text "DEFINE DATA must be the first statement if present", explanation "If DEFINE DATA is used in a program, it must always be the first statement." PARAMETER and LOCAL are clauses of one statement, not statements: `DEFINE DATA [GLOBAL ...] [PARAMETER ...] [LOCAL ...] [INDEPENDENT ...] [CONTEXT ...] [OBJECT ...] END-DEFINE`. The official subprogram example makes the intended shape explicit: `DEFINE DATA / PARAMETER USING PDA01 / LOCAL USING LDA02 / END-DEFINE`. | https://documentation.softwareag.com/natural/nat911mf/sm/defineda_basic.htm and https://documentation.softwareag.com/natural/nat913unx/mc/mcERRN_0001.htm and https://documentation.softwareag.com/natural/nat913unx/sm/defineda.htm and https://documentation.softwareag.com/natural/nat911mf/firststeps/fs-subprog.htm | 2026-08-04 |
| library, COUNT-STAFF | PARAMETER declared before LOCAL | CONFIRMED | The relative order is right, and only the block structure above is wrong. "If the `GLOBAL` and the `PARAMETER` clauses are used, `GLOBAL` must be the first clause of the statement and `PARAMETER` must follow `GLOBAL`", with the remaining clauses in any order. With no GLOBAL clause, PARAMETER first then LOCAL is correct. | https://documentation.softwareag.com/natural/nat911mf/sm/defineda_basic.htm | 2026-08-04 |
| library, COUNT-STAFF | `RESET #HOWMANY` / `FIND EMPLOYEES-VIEW WITH COUNTRY = #COUNTRY` / `ADD 1 TO #HOWMANY` / `END-FIND` | CONFIRMED as syntax, MISLEADING as teaching | The syntax is valid: `FIND [ALL|(operand1)] [RECORDS] [IN] [FILE] view-name [[WITH] ... basic-search-criteria] statement END-FIND`, and "The WITH clause is required. It is used to specify the basic-search-criteria consisting of key fields (descriptors) defined in the database", with non-descriptors permitted if marked N in the DDM. Whether COUNTRY is a descriptor in the course's DDM is UNVERIFIED. The pedagogy is the issue: counting by incrementing inside a FIND loop is the expensive way, and Natural gives the answer directly. "The system variable *NUMBER contains the number of records found after the evaluation of the WITH criterion." A subprogram whose entire job is to return a count should use `FIND NUMBER` and `*NUMBER`, and a course that models it otherwise teaches a habit reviewers will flag. | https://documentation.softwareag.com/natural/nat914unx/sm/find.htm | 2026-08-04 |
| library, DOUBLE-IT | `DEFINE DATA PARAMETER / 1 #IN (N5) / 1 #OUT (N7) / END-DEFINE / COMPUTE #OUT = #IN * 2 / END` | CONFIRMED | Valid apart from the object name. Single DEFINE DATA, PARAMETER only, arithmetic in range (N5 doubled is at most 199998, which fits N7), terminated by END. Caller declarations in 12.3 and 13.1 (`#VALUE (N5)`, `#RESULT (N7)`) match the parameter block exactly in sequence, format and length, which is what by-reference passing requires. | https://documentation.softwareag.com/natural/nat912unx/sm/defineda_pda.htm | 2026-08-04 |
| 11.1, 11.3 | `DEFINE DATA LOCAL / 1 #N (N5) / END-DEFINE` in programs that never reference `#N` | CONFIRMED valid, minor | Not an error. Natural requires only that a DEFINE DATA statement not be empty: "an 'empty' DEFINE DATA statement is not allowed ... at least one clause ... must be specified and at least one field must be defined." A dead field in a beginner sample is noise that invites the question "what is #N for", which the lesson never answers. | https://documentation.softwareag.com/natural/nat911mf/sm/defineda_basic.htm | 2026-08-04 |

## Corrections required

Ordered by reach and severity.

1. Rename both library subprograms to 8 characters or fewer and update every call
   site. `DOUBLE-IT` and `COUNT-STAFF` are 9 and 11 characters against a hard 1 to
   8 limit for Natural object names, restated independently in the CALLNAT
   reference. Affected: library.txt (both objects), 12.2 prose, 12.3 code, 13.1
   prose and code, 13.2 code (twice), 13.3 code, 13.4 task text. Suggested names
   that keep the teaching intent: `DOUBLEIT` and `CNTSTAFF`. While making the
   change, add one sentence distinguishing the two limits, because the course
   currently conflates them: a subroutine name may be up to 32 characters, a
   subprogram is a library object and is capped at 8.

2. Merge COUNT-STAFF's two DEFINE DATA blocks into one. Replace the current
   `DEFINE DATA PARAMETER ... END-DEFINE` plus `DEFINE DATA LOCAL ... END-DEFINE`
   with a single statement:

   ```
   DEFINE DATA
   PARAMETER
   1 #COUNTRY (A3)
   1 #HOWMANY (N3)
   LOCAL
   1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
   2 COUNTRY
   END-DEFINE
   ```

   As written it violates NAT0004 and would not compile. This is also a teaching
   opportunity the course is currently spending on a wrong example, since a
   subprogram with both a parameter block and its own locals is the standard shape.

3. Rewrite the recursion sentence in 11.3. Delete "Try to write a subroutine that
   performs itself and the interpreter will stop you rather than letting the
   program run out of stack." Natural explicitly permits recursive PERFORM. Say
   instead that Natural allows a subroutine to invoke itself, that nesting depth is
   bounded by memory rather than by a fixed level, and that this course's browser
   interpreter caps recursion so a runaway program cannot hang the learner's tab.
   That keeps the cap honest and stops the course from teaching a language rule
   that does not exist. If the cap message is shared with the module 7 REPEAT
   runaway cap, word it so it plainly reads as an interpreter guard rather than a
   Natural diagnostic.

4. Fix `CALLNAT 'COUNT-STAFF' 'UK' #HOWMANY` in 13.2. An A2 constant cannot be
   passed by reference to an A3 parameter without a NAT0936 format and length
   conflict. Two acceptable fixes: pass `'UK '` with the trailing blank so the
   constant is A3, or drop the constant and reuse `#WHERE`. The first is better if
   the course wants to keep showing a literal argument, because it makes the
   exact-match rule visible. Either way, add a line naming the rule, since this
   sample is the only place a learner meets a constant argument.

5. Rename the subroutines that collide with reserved keywords. `REPORT` in 11.2,
   `OUTER` and `INNER` in 11.3. Suggested: `SHOW-TOTALS`, `OUTER-STEP`,
   `INNER-STEP`. `GATHER` is clear and can stay. The `#`-prefixed variables are all
   clear and need no change; if anything the course should say why the `#` prefix
   is the convention, since it is exactly what keeps `#TOTAL` and `#COUNT` legal
   when `TOTAL` and `COUNT` are not.

6. Correct the parameter-passing model in 12.2 and 13.1. Replace "passes values in
   and results back" with the fact: Natural passes by reference by default, so a
   parameter is the caller's storage under a different name and any parameter can
   be written back. Then state the isolation claim precisely, which is that
   non-parameter fields do not cross. Add a short mention of BY VALUE and OPTIONAL.
   This is a small edit that removes the course's one wrong mental model.

7. Resolve the lesson 12 title. Either retitle to something the content delivers,
   for example "Where fields live: LOCAL and PARAMETER", or add a closing step that
   names the three data area object types (LDA, GDA, PDA), shows one
   `DEFINE DATA LOCAL USING` line, and states that a GDA is shared across a calling
   hierarchy. The second is preferable: GDA is everywhere in the legacy Natural
   these learners are being trained for, and the current lesson leaves them
   believing Natural has exactly two data scopes. It also directly contradicts the
   lesson's own lede, which promises that where a field lives decides who can see
   it, and then omits the scope that most complicates that promise.

8. Decide and disclose the EMPLOYEES DDM shape. In the real Software AG demo file,
   SALARY is a multiple-value field inside the periodic group INCOME, and the
   official view form is `2 INCOME (1:2) / 3 CURR-CODE / 3 SALARY / 3 BONUS (1:1)`.
   The course uses a flat `2 SALARY`. If that flattening is deliberate, add one
   line to lessons 11.2 and 11.4 saying the course DDM is simplified and that a
   production EMPLOYEES view needs group and index notation. If it is not
   deliberate, the view declarations need correcting. This is the one finding I
   could not close, because the course's own DDM was not in scope.

9. Optional, lower value. Consider rewriting COUNT-STAFF to use `FIND NUMBER` and
   `*NUMBER` instead of incrementing inside a FIND loop, which is the idiomatic
   Natural way to return a count. Consider removing the unused `1 #N (N5)` from the
   11.1 and 11.3 samples. Consider adding the restriction that an inline subroutine
   must not contain another DEFINE SUBROUTINE statement, which belongs in 11.3
   where nesting is introduced and is the restriction learners actually trip over.

## Sources

All accessed 2026-08-04.

- DEFINE SUBROUTINE, Natural for UNIX 9.1.4: https://documentation.softwareag.com/natural/nat914unx/sm/definesu.htm
- DEFINE SUBROUTINE, NaturalONE 9.1.2: https://documentation.softwareag.com/naturalONE/natONE912/natov/sm/definesu.htm
- PERFORM, Natural for UNIX 9.1.4: https://documentation.softwareag.com/natural/nat914unx/sm/perform.htm
- CALLNAT, Natural for UNIX 9.1.4: https://documentation.softwareag.com/natural/nat914unx/sm/callnat.htm
- CALLNAT, Natural for UNIX 9.1.1: https://documentation.softwareag.com/natural/nat911unx/sm/callnat.htm
- DEFINE DATA, function and basic syntax rules, Natural for Mainframes 9.1.1: https://documentation.softwareag.com/natural/nat911mf/sm/defineda_basic.htm
- DEFINE DATA, syntax overview, Natural for UNIX 9.1.3: https://documentation.softwareag.com/natural/nat913unx/sm/defineda.htm
- Defining Parameter Data, Natural for UNIX 9.1.2: https://documentation.softwareag.com/natural/nat912unx/sm/defineda_pda.htm
- Defining Application-Independent Variables, Natural for Mainframes 8.2.7: https://documentation.softwareag.com/natural/nat827mf/sm/defineda_aiv.htm
- Data Areas, Natural for UNIX 9.1.3: https://documentation.softwareag.com/natural/nat913unx/pg/pg_obj_darea.htm
- Programs and Subordinate Routines, Natural for Mainframes 9.2.1: https://documentation.softwareag.com/natural/nat921mf/webhelp/natmf-webhelp/pg/pg_obj_pgm_routine.htm
- Programs, Functions, Subprograms and Subroutines, NaturalONE 9.1.2: https://documentation.softwareag.com/naturalONE/natONE912/natov/pg/pg_obj_prog.htm
- First Steps, Inline Subroutines, Natural for Mainframes 8.2.7: https://documentation.softwareag.com/natural/nat827mf/firststeps/fs-inlinesub.htm
- First Steps, Subprograms, Natural for Mainframes 9.1.1: https://documentation.softwareag.com/natural/nat911mf/firststeps/fs-subprog.htm
- Rules and Naming Conventions, Natural for Windows 9.1.3: https://documentation.softwareag.com/natural/nat913win/using/use_rules.htm
- Natural Reserved Keywords, Natural for Mainframes 8.2.7: https://documentation.softwareag.com/natural/nat827mf/pg/pg_keyw.htm
- READ, Natural for UNIX 9.1.4: https://documentation.softwareag.com/natural/nat914unx/sm/read.htm
- FIND, Natural for UNIX 9.1.4: https://documentation.softwareag.com/natural/nat914unx/sm/find.htm
- Accessing Data in an Adabas Database, Natural for Windows 9.1.3: https://documentation.softwareag.com/natural/nat913win/pg/pg_dbms_ada.htm
- System Error Messages 0001-0050, Natural for UNIX 9.1.3: https://documentation.softwareag.com/natural/nat913unx/mc/mcERRN_0001.htm
- System Error Messages 0901-0950, Natural for UNIX 8.4.1: https://documentation.softwareag.com/natural/nat841unx/mc/mcERRN_0901.htm
