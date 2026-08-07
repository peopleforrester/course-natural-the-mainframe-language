# Adversarial fact-check: lessons 7 and 8

Audited 2026-08-04 against official Software AG Natural documentation at
documentation.softwareag.com. Every claim was treated as wrong until an official page
proved otherwise. Where the vendor wording settles a point it is quoted verbatim.

Documentation versions consulted: Natural for Mainframes 8.2.7, 9.1.1, 9.1.2, and
Natural for UNIX 8.4.1 and 9.1.1. Statement syntax is stable across these versions and
the clause orders quoted below agree in every version checked.

## Findings

| Lesson.Step | Claim (quoted) | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| 7.1 | `FOR #I = 1 TO 5` with `END-FOR` | CONFIRMED | The syntax box is `FOR operand1 [:]= / EQ / FROM operand2 TO / THRU operand3 [STEP operand4] statement END-FOR`. The equals sign is one of the accepted assignment tokens, so `=` is correct. The docs add that "The keywords `[:]=`, `EQ` or `FROM` can be omitted", which is why the vendor's own example reads `FOR #INDEX 1 TO 5`. The course form is legal, and arguably clearer for beginners than the vendor example. | https://documentation.softwareag.com/natural/nat827mf/sm/for.htm | 2026-08-04 |
| 7.2 | "REPEAT loops forever until something ends it" with `END-REPEAT` | CONFIRMED | The docs state: "If no logical condition is specified, the loop must be exited by an `ESCAPE`, `STOP` or `TERMINATE` statement specified within the loop." And: "In structured mode, the Natural reserved word `END-REPEAT` must be used to end the `REPEAT` statement." | https://documentation.softwareag.com/natural/nat827mf/sm/repeat.htm | 2026-08-04 |
| 7.2 | "ESCAPE BOTTOM leaves the loop; ESCAPE TOP starts the next pass." | CONFIRMED | Verbatim: BOTTOM "indicates that processing is to continue with the first statement following the processing loop. The loop is terminated and loop-end processing (final BREAK and END DATA) is executed." TOP "indicates that processing is to continue at the top of the processing loop. This starts the next repetition of the processing loop." Both course descriptions are accurate. | https://documentation.softwareag.com/natural/nat827mf/sm/escape.htm | 2026-08-04 |
| 7.2 | ESCAPE requires an option (course teaches that it does) | CONFIRMED | The syntax offers exactly four alternatives, one of which must be selected: `ESCAPE TOP [REPOSITION]`, `ESCAPE BOTTOM [(r)] [IMMEDIATE]`, `ESCAPE ROUTINE [IMMEDIATE]`, `ESCAPE MODULE [IMMEDIATE]`. Bare `ESCAPE` is not a legal form. | https://documentation.softwareag.com/natural/nat827mf/sm/escape.htm | 2026-08-04 |
| 7.3 | "A REPEAT with nothing to stop it would hang a real session" | MISLEADING | Literally true for the exact code shown, which does no database I/O and calls no other program, but the sentence leaves a learner believing real Natural has no runaway protection at all. It has three mechanisms. MADIO caps "the maximum number of DBMS calls permitted between two screen I/O operations (also in batch mode)", default 512, and on breach "the Natural program is interrupted and the user is notified with an appropriate Natural error message". MAXCL caps "the maximum number of program calls permitted between two screen I/O operations", default 50, raising NAT1029. The LT session parameter "limits the number of records which may be read in a database processing loop". A runaway REPEAT that touches the database or calls a subprogram is therefore stopped by the runtime. Only a pure in-memory spin like the course example escapes all three. | https://documentation.softwareag.com/natural/nat912unx/parms/madio.htm and https://documentation.softwareag.com/natural/nat911mf/parms/maxcl.htm and https://documentation.softwareag.com/natural/nat912mf/pg/pg_furth_loop.htm | 2026-08-04 |
| 7.3 | Whether a real compiler rejects a REPEAT with no ESCAPE and no condition | UNVERIFIED | The documentation states the loop "must be exited by an ESCAPE, STOP or TERMINATE statement", but nowhere describes this as a compile-time diagnostic, and no error number is associated with it. Do not assert either way in course prose. | https://documentation.softwareag.com/natural/nat827mf/sm/repeat.htm | 2026-08-04 |
| 7.4 | `REPEAT UNTIL #N >= 3` as a top-tested form | CONFIRMED | Two syntax forms are documented. Syntax 1 puts UNTIL or WHILE at the end of the loop; syntax 2 puts UNTIL or WHILE immediately after the REPEAT keyword, before the statements. "The placement of the logical condition (either at the beginning or at the end of the loop) determines when it is to be evaluated." For UNTIL: "The processing loop will be continued until the logical condition becomes true." The course's top-placed UNTIL is legal and the semantics as taught are right. | https://documentation.softwareag.com/natural/nat827mf/sm/repeat.htm | 2026-08-04 |
| 8.1 | "Natural calls READ and FIND database loops, as opposed to the non-database loops (FOR, REPEAT)" | CONFIRMED, but incomplete | This is genuinely the vendor's own terminology, not an invention. Verbatim: "Database processing loops are those created automatically by Natural to process data selected from a database as a result of a `READ`, `FIND` or `HISTOGRAM` statement." And: "Non-database processing loops are initiated by the statements `REPEAT`, `FOR`, `CALL FILE`, `CALL LOOP`, `SORT` and `READ WORK FILE`." The framing is correct. The defect is that the course omits HISTOGRAM from the database-loop list and then teaches HISTOGRAM in 8.5, leaving the learner with no category for it. | https://documentation.softwareag.com/natural/nat912mf/pg/pg_furth_loop.htm | 2026-08-04 |
| 8.2 | "A VIEW OF names the file and lists the fields you want. Those fields take their format from the file definition, so you do not declare formats for them." | CONFIRMED | Verbatim: "The `view-definition` option is used to define a data view as derived from a data definition module (DDM)." On format and length: "If omitted, these are taken from the DDM." Supplying a format is optional, not forbidden, and the vendor's own example shows both `2 NAME(A20)` and a bare `2 NAME` in the same program. The course's simplification is accurate for beginners. | https://documentation.softwareag.com/natural/nat912mf/sm/defineda_view.htm | 2026-08-04 |
| 8.2 | `READ EMPLOYEES-VIEW BY NAME` with `END-READ` | CONFIRMED | `IN`, `LOGICAL` and `SEQUENCE` are optional noise words around `BY descriptor`, so the short form is legal. The vendor's own example program contains the line `READ EMPLOY-VIEW BY NAME` verbatim. The docs also confirm the loop framing: "The READ statement causes a processing loop to be initiated." | https://documentation.softwareag.com/natural/nat911mf/sm/read.htm | 2026-08-04 |
| 8.3 | `FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' WHERE SALARY > 45000 SORTED BY NAME` and the teaching that the clause order is WITH, then WHERE, then SORTED BY | REFUTED | The clause order is wrong and a real compiler rejects it. The documented order of the FIND clauses is: MULTI-FETCH, view-name, PASSWORD, CIPHER, WITH, COUPLED, STARTING WITH ISN, SORTED BY, RETAIN, SHARED HOLD, SKIP RECORDS IN HOLD, WHERE, IF NO RECORDS FOUND, then the statements. SORTED BY comes BEFORE WHERE, not after. Verified independently in the 8.2.7 and 9.1.1 mainframe syntax diagrams, which agree. The correct line is `FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' SORTED BY NAME WHERE SALARY > 45000`. | https://documentation.softwareag.com/natural/nat827mf/sm/find.htm and https://documentation.softwareag.com/natural/nat911mf/sm/find.htm | 2026-08-04 |
| 8.3 | "WITH is the search the database performs. WHERE filters further, record by record, after they come back" | CONFIRMED | Verbatim: "The `WHERE` clause may be used to specify an additional selection criterion (`logical-condition`) which is evaluated _after_ a value has been read and _before_ any processing is performed on the value (including the `AT BREAK` evaluation)." The course's plain-English rendering is precise. A corroborating detail worth teaching: "If a processing limit is specified in a `FIND` statement containing a `WHERE` clause, records which are rejected as a result of the `WHERE` clause are _not_ counted against the limit." | https://documentation.softwareag.com/natural/nat827mf/sm/find.htm | 2026-08-04 |
| 8.3 | "*NUMBER reports how many records the WITH search found, before WHERE narrowed them" | CONFIRMED | This is one of the strongest points in the two lessons and the vendor wording matches it almost word for word: "The system variable `*NUMBER` contains the number of records found after the evaluation of the `WITH` criterion and before evaluation of any `WHERE` criteria." | https://documentation.softwareag.com/natural/nat827mf/sm/find.htm | 2026-08-04 |
| 8.3 | `WRITE 'The search matched' *NUMBER 'records before the WHERE filter.'` placed after `END-FIND` | REFUTED | Invalid as written. A bare `*NUMBER` resolves to the innermost ACTIVE loop, and after END-FIND there is none. Verbatim rule: "The Natural system variables `*ISN`, `*NUMBER`, and `*COUNTER` are automatically created for each `FIND` statement issued. A reference number must be supplied if the system variable was referenced outside the current processing loop or through a `FIND UNIQUE`, `FIND FIRST`, or `FIND NUMBER` statement." Also: "If (r) is not specified, `*NUMBER` automatically refers to the innermost active FIND, HISTOGRAM or READLOB processing loop by default." The statement needs a label on the FIND and `*NUMBER(EMP.)` on the WRITE. | https://documentation.softwareag.com/natural/nat911unx/sm/find.htm and https://documentation.softwareag.com/natural/nat912mf/vari/appl.htm | 2026-08-04 |
| 8.3 | "*COUNTER" as records processed so far | CONFIRMED | Verbatim: "`*COUNTER` contains the number of times a processing loop initiated by a FIND, READ, HISTOGRAM or PARSE statement has been entered." The key nuance, which reinforces the lesson's WITH versus WHERE point and is worth adding: "`*COUNTER` is not incremented if a record is rejected as a result of the criteria specified in a `WHERE` clause." So in the 8.3 program *COUNTER counts post-WHERE rows while *NUMBER counts pre-WHERE rows, which is exactly the contrast the lesson is reaching for. | https://documentation.softwareag.com/natural/nat912mf/vari/appl.htm | 2026-08-04 |
| 8.4 | "IF NO RECORDS FOUND is a clause of the FIND itself, and runs instead of the loop" | REFUTED | The first half is right and the second half is wrong. It is a clause of FIND, but it does not run instead of the loop. Verbatim: "If no records meet the specified `WITH` and `WHERE` criteria, the `IF NO RECORDS FOUND` clause causes the `FIND` processing loop to be executed once with an 'empty' record." And: "If one or more statements are specified with the `IF NO RECORDS FOUND` clause, the statements will be executed immediately before the processing loop is entered." And: "Unless other value assignments are made in the statements accompanying an `IF NO RECORDS FOUND` clause, Natural will reset to empty all database fields which reference the file specified in the current loop." The docs give the remedy explicitly: "If this is not desired, specify the statement `ESCAPE BOTTOM` within the `IF NO RECORDS FOUND` clause." On a real system the 8.4 program prints the message and then one blank NAME line, which the lesson's prose says will not happen. | https://documentation.softwareag.com/natural/nat911mf/sm/find.htm | 2026-08-04 |
| 8.4 | Closed by `END-NOREC` | CONFIRMED | "`END-NOREC`" ends the IF NO RECORDS FOUND clause in structured mode. The vendor example program uses the same shape the lesson does, an `IF NO RECORDS FOUND` block terminated by `END-NOREC` nested inside a FIND closed by `END-FIND`. | https://documentation.softwareag.com/natural/nat911mf/sm/find.htm | 2026-08-04 |
| 8.5 | "HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY" walks distinct values with counts in *NUMBER "without reading the records themselves", closed by END-HISTOGRAM | CONFIRMED | `VALUE`, `FOR` and `FIELD` are all optional noise words before the descriptor, so `HISTOGRAM view-name FOR COUNTRY` is legal. The no-record-access claim is exactly right: "The values are read directly from the Adabas inverted lists", and the statement "does not provide access to any database fields other than the field specified in the `HISTOGRAM` statement." On the counts: during HISTOGRAM, `*NUMBER` "contains the number of database records that contain the last value read." One caveat the lesson does not state: "As operand4, a descriptor, subdescriptor, superdescriptor or hyperdescriptor may be specified." HISTOGRAM cannot walk a non-descriptor field. | https://documentation.softwareag.com/natural/nat911mf/sm/histogra.htm and https://documentation.softwareag.com/natural/nat841unx/sm/histogra.htm | 2026-08-04 |
| n/a | The read limit forms `READ (2) EMPLOYEES-VIEW` and `FIND (2) ...` | CONFIRMED as syntax, NOT PRESENT in these lessons | Both forms are valid. For READ: "The number of records to be read may be limited by specifying operand1 (enclosed in parentheses, immediately after the keyword READ) - either as a numeric constant (in the range from 0 to 4294967295) or as the name of a numeric variable", with the vendor example `READ (9) EMPL BY NAME`. For FIND: "The number of records to be processed from the selected set may be limited by specifying operand1 (enclosed in parentheses, immediately after the keyword FIND)", with the vendor example `FIND (5) IN EMPLOYEES WITH ...`. Neither form appears anywhere in lesson 7 or lesson 8 as delivered. If the audit brief expected them to be taught here, they are missing. | https://documentation.softwareag.com/natural/nat911mf/sm/read.htm and https://documentation.softwareag.com/natural/nat911mf/sm/find.htm | 2026-08-04 |
| 8.2, 8.6 | "The sample file holds eight employees" and "There are two of them" | UNVERIFIED | These describe the course's own fixture, not anything in the vendor documentation, so no external source can settle them. Verify against the interpreter's sample dataset in the repo before publishing, and re-verify after any fixture change, because 8.6 grades on the count. | n/a | 2026-08-04 |

## Corrections required

Ordered by severity.

**1. Lesson 8.3, clause order. Blocking.** The FIND statement puts SORTED BY before
WHERE. The published sample is invalid Natural and a real compiler rejects it, so a
learner who carries the pattern to a real system gets a syntax error and has been
taught the wrong mental model of the statement. Fix the code and the surrounding prose
together, since the lesson explicitly teaches the order.

**2. Lesson 8.3, *NUMBER outside the loop. Blocking.** The trailing WRITE references a
bare `*NUMBER` after `END-FIND`, where no FIND loop is active. Reference notation is
mandatory outside the loop. Combined with correction 1, the sample becomes:

```
EMP. FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA'
                         SORTED BY NAME
                         WHERE SALARY > 45000
  DISPLAY *COUNTER NAME SALARY
END-FIND
WRITE 'The search matched' *NUMBER(EMP.) 'records before the WHERE filter.'
```

The label also gives the lesson a natural place to introduce reference notation, which
a learner needs anyway before nested loops.

**3. Lesson 8.4, IF NO RECORDS FOUND semantics. Blocking.** Replace "runs instead of
the loop" with the documented behavior: the clause statements run immediately before
the loop is entered, and the loop is then entered exactly once with every database
field for that file reset to empty. Teach the documented remedy in the same breath:

```
FIND EMPLOYEES-VIEW WITH CITY = 'ATLANTIS'
  IF NO RECORDS FOUND
    WRITE 'Nobody works in ATLANTIS.'
    ESCAPE BOTTOM
  END-NOREC
  WRITE NAME
END-FIND
```

Without the `ESCAPE BOTTOM` the program prints the message and then a blank NAME line.
This also pays off lesson 7, because it is a real reason to reach for ESCAPE BOTTOM
rather than a contrived one.

**4. Lesson 7.3, runaway protection. Correct the claim.** "Would hang a real session"
overstates it. Say instead that this particular loop touches neither the database nor
another program, so it escapes Natural's own guards, and name them: MADIO caps DBMS
calls between screen I/Os at 512 by default, MAXCL caps program calls at 50 by default
and raises NAT1029, and the LT session parameter caps records read in a database loop.
That turns the interpreter's statement cap into an honest analogue of a real mechanism
instead of an invented one, and it is a better lesson.

**5. Lesson 8.1, incomplete category list.** The vendor's database-loop list is READ,
FIND and HISTOGRAM. Add HISTOGRAM, since lesson 8.5 teaches it four steps later and
currently leaves it uncategorized. The non-database list can stay as FOR and REPEAT for
beginners, with a note that SORT and READ WORK FILE also qualify.

**6. Lesson 8.5, missing descriptor precondition.** State that HISTOGRAM only works on
a descriptor, subdescriptor, superdescriptor or hyperdescriptor. As written the lesson
implies it works on any field, which is the kind of gap that produces a confused
support question rather than a compile error the learner can read.

**7. Three missing spaces after a sentence-ending period.** Live typography defects at
7.3 ("fix it.This is"), 8.2 ("formats for them.The sample file"), and 8.3 ("after they
come back.*NUMBER reports"). The 8.3 one is the worst, because it visually welds the
system variable name onto the previous sentence.

**8. Sample-data counts.** Confirm "eight employees" and "two of them" against the
interpreter fixture before publishing, and add a check so a fixture change cannot
silently invalidate the 8.6 exercise text.

Everything not listed above passed. Lesson 7 is otherwise accurate: the FOR syntax, the
REPEAT contract, both ESCAPE options, the mandatory-option rule, and the top-tested
REPEAT UNTIL are all confirmed against the vendor statement reference. In lesson 8 the
view definition, the READ loop, the WITH versus WHERE distinction, the *NUMBER and
*COUNTER definitions, END-NOREC, and the HISTOGRAM inverted-list claim are all
confirmed, several of them nearly verbatim.

## Sources

All accessed 2026-08-04.

- FOR statement, Natural for Mainframes 8.2.7. https://documentation.softwareag.com/natural/nat827mf/sm/for.htm
- REPEAT statement, Natural for Mainframes 8.2.7. https://documentation.softwareag.com/natural/nat827mf/sm/repeat.htm
- ESCAPE statement, Natural for Mainframes 8.2.7. https://documentation.softwareag.com/natural/nat827mf/sm/escape.htm
- FIND statement, Natural for Mainframes 8.2.7. https://documentation.softwareag.com/natural/nat827mf/sm/find.htm
- FIND statement, Natural for Mainframes 9.1.1. https://documentation.softwareag.com/natural/nat911mf/sm/find.htm
- FIND statement, Natural for UNIX 9.1.1. https://documentation.softwareag.com/natural/nat911unx/sm/find.htm
- READ statement, Natural for Mainframes 9.1.1. https://documentation.softwareag.com/natural/nat911mf/sm/read.htm
- READ statement, Natural for Mainframes 8.2.7. https://documentation.softwareag.com/natural/nat827mf/sm/read.htm
- HISTOGRAM statement, Natural for Mainframes 9.1.1. https://documentation.softwareag.com/natural/nat911mf/sm/histogra.htm
- HISTOGRAM statement, Natural for UNIX 8.4.1. https://documentation.softwareag.com/natural/nat841unx/sm/histogra.htm
- ADD statement, Natural for Mainframes 9.1.1. https://documentation.softwareag.com/natural/nat911mf/sm/add.htm
- Loop Processing, Natural Programming Guide, Mainframes 9.1.2. https://documentation.softwareag.com/natural/nat912mf/pg/pg_furth_loop.htm
- System variables, Natural for Mainframes 9.1.2. https://documentation.softwareag.com/natural/nat912mf/vari/appl.htm
- View Definition, DEFINE DATA, Natural for Mainframes 9.1.2. https://documentation.softwareag.com/natural/nat912mf/sm/defineda_view.htm
- MADIO parameter, Natural for UNIX 9.1.2. https://documentation.softwareag.com/natural/nat912unx/parms/madio.htm
- MAXCL parameter, Natural for Mainframes 9.1.1. https://documentation.softwareag.com/natural/nat911mf/parms/maxcl.htm
