# Adversarial fact-check: lesson 9 (changing the database) and lesson 10 (REINPUT)

Audited 2026-08-04 against official Software AG Natural and Adabas documentation at
documentation.softwareag.com. Every claim was treated as wrong until a documented
sentence proved it. Doc set used is Natural 9.1.1 for Mainframes (`nat911mf`) with
cross-checks in `nat911unx`, `nat912mf`, `nat913unx`, `nat841unx`, and the Adabas
command and concepts references.

## Findings

| Lesson.Step | Claim (quoted) | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| 9 lede | "None of them count until you commit with END TRANSACTION." | CONFIRMED, with a caveat | True for durability. Doc: "Successful execution of an END TRANSACTION statement ensures that all updates performed during the transaction have been or will be physically applied to the database regardless of subsequent user, Natural, database or operating system interruption. Updates performed within a transaction for which the END TRANSACTION statement has not been successfully completed will be backed out automatically." The caveat the lesson never states is that until the commit the updating program does see its own changes and the records sit in exclusive hold, locked against other users. | https://documentation.softwareag.com/natural/nat911mf/sm/endtrans.htm | 2026-08-04 |
| 9.1 | "Move the values you want into the view fields, then STORE." and the code line `STORE EMPLOYEES-VIEW` | CONFIRMED | The documented syntax is `STORE [RECORD] [IN] [FILE] view-name` with `PASSWORD=`, `CIPHER=`, `USING NUMBER` / `GIVING NUMBER` all optional, so the bare form compiles. The official example writes it in full as `STORE RECORD IN EMPL-VIEW`, which is the same statement with the optional noise words present. A `SET` / `WITH operand=operand` clause does exist, but the doc restricts it to reporting mode, so a structured-mode program that starts with `DEFINE DATA` cannot use it. The course teaching MOVE-then-STORE is the correct structured-mode form, not an omission. | https://documentation.softwareag.com/natural/nat911mf/sm/store.htm | 2026-08-04 |
| 9.1 | "Note the END TRANSACTION at the bottom: that is the commit." | MISLEADING | The `END TRANSACTION` in the sample is not at the bottom. It is on line 8 of a 12-line program, before the verification READ loop. A learner scanning for it at the bottom finds `END`, which is a different statement. Also worth knowing: the statement is documented as `END [OF] TRANSACTION [operand1]`, so `END OF TRANSACTION` is equally valid and appears in the official STORE example. | https://documentation.softwareag.com/natural/nat911mf/sm/endtrans.htm | 2026-08-04 |
| 9.2 | "Inside a READ or FIND loop, UPDATE writes the view fields back to the record the loop is holding." plus the bare `UPDATE` with no operand | CONFIRMED | Syntax is `UPDATE [RECORD] [IN] [STATEMENT] [(r)]`, so every element is optional and a bare `UPDATE` is legal. Prerequisite per doc: the record "must have been previously selected with a FIND, GET or READ statement (or, for Adabas only, with a STORE statement)". Hold is not something the programmer requests: "The use of the UPDATE statement causes each record read for processing in the corresponding FIND or READ statement to be placed in exclusive hold." There is no HOLD keyword on FIND or READ to add, and no HOLD profile parameter the learner must set. The restriction the samples do satisfy is that UPDATE "must not be entered on the same line as the statement used to select the record". | https://documentation.softwareag.com/natural/nat911mf/sm/update.htm | 2026-08-04 |
| 9.3 | "On a real system an uncommitted transaction is backed out when the program ends. The work is simply gone." | REFUTED | Program end is not a transaction boundary at all. Adabas defines the boundary as: "A logical transaction begins with the first command that places a record in hold status and ends when an ET (end transaction), BT (back out transaction), CL (close), or OP (open) command is issued for the same user." End of a Natural program issues none of those by default. The governing profile parameter is ETEOP, "determines whether or not an implicit END TRANSACTION statement is to be issued at the end of a Natural program (that is, before NEXT mode is reached)", default OFF, meaning "Natural will not issue any implicit END TRANSACTION statement at the end of a Natural program". So on defaults the transaction stays open past the end of the program, the records stay in exclusive hold, and the next program in the same session can still commit the work with its own END TRANSACTION. If a site runs ETEOP=ON, the work is committed at end of program, which is the exact opposite of what the lesson teaches. The automatic backout the lesson describes belongs to session end, not program end: ENDBT "determines whether or not an implicit BACKOUT TRANSACTION statement is to be issued at the end of the Natural session", default ON. This is the load-bearing claim of the whole lesson and it is wrong as written. | https://documentation.softwareag.com/adabas/ada744mfr/adamf/concepts/cfusing.htm , https://documentation.softwareag.com/natural/nat911mf/parms/eteop.htm , https://documentation.softwareag.com/natural/nat911mf/parms/endbt.htm | 2026-08-04 |
| 9.3 | "The second FIND runs after a fresh start and shows the original value." | REFUTED | There is no second FIND in the 9.3 sample. The program contains one FIND, one COMPUTE, one UPDATE, one WRITE inside the loop and one WRITE after it. The prose describes a program the learner is not looking at, most likely a leftover from the 9.2 sample which does have two FIND blocks. | (internal, code and prose mismatch in lesson-09.txt) | 2026-08-04 |
| 9.3 | "The update below happens, and the program can even see it" | CONFIRMED | A Natural program reads back its own uncommitted updates inside the same session, because the update is applied and the record is held until ET or BT. Nothing in the docs contradicts this. | https://documentation.softwareag.com/natural/nat911mf/sm/update.htm | 2026-08-04 |
| 9.4 | "DELETE removes the record the loop is holding." | CONFIRMED | Syntax `DELETE [RECORD] [IN] [STATEMENT] [(r)]`; the record must have been selected by a preceding FIND, READ or GET; with no `(r)` reference it applies to the innermost active processing loop that read a record; "The use of the DELETE statement causes each record selected in the corresponding FIND or READ statement to be placed in exclusive hold." The sample obeys the restriction that "A DELETE statement cannot be specified in the same statement line as a FIND, READ, or GET statement." | https://documentation.softwareag.com/natural/nat911mf/sm/delete.htm | 2026-08-04 |
| 9.4 | "BACKOUT TRANSACTION throws away everything since the last commit, which is how you undo deliberately." | CONFIRMED | Doc: the statement "is used to back out all database updates performed during the current logical transaction. This statement also releases all records held during the transaction." The course omits the released-holds half, which is the more visible effect on a shared system. Scope depends on the ET profile parameter: only the affected database when ET=OFF, every database referenced since the last END TRANSACTION or BACKOUT TRANSACTION when ET=ON. | https://documentation.softwareag.com/natural/nat911mf/sm/backout.htm | 2026-08-04 |
| 9.6 (checklist item) | Whether END TRANSACTION and BACKOUT TRANSACTION have documented operands the course omits | CONFIRMED omission on one, none on the other | END TRANSACTION does take an operand the course never mentions: `END [OF] TRANSACTION [operand1]`, where operand1 is transaction data of up to 2000 bytes written to the database named by ETDB, read back later by GET TRANSACTION DATA. That is the standard mainframe restart-point idiom and a learner will meet it in real code. BACKOUT TRANSACTION has no operands at all; the syntax is `BACKOUT [TRANSACTION]`, with the word TRANSACTION itself optional. | https://documentation.softwareag.com/natural/nat911mf/sm/endtrans.htm , https://documentation.softwareag.com/natural/nat911mf/sm/backout.htm | 2026-08-04 |
| 9.5 | "a histogram summary, a read loop with grading, a report, a rounded calculation, a filtered update, and a commit you can verify" | REFUTED | The capstone code contains no UPDATE and no END TRANSACTION. Two of the six promised elements, and specifically the two that lesson 9 exists to teach, are missing from the program. Everything else in the list is present. | (internal, code and prose mismatch in lesson-09.txt) | 2026-08-04 |
| 9.5 | `HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY` with `*NUMBER` | CONFIRMED | The syntax prints the keyword group as `[VALUE] [FOR] [FIELD] operand4`, so all three words are optional and `HISTOGRAM view FOR field` is valid. "As operand4, a descriptor, subdescriptor, superdescriptor or hyperdescriptor may be specified", which COUNTRY is. `*NUMBER` "contains the number of database records that contain the last value read", and the official example uses `*NUMBER` inside a HISTOGRAM loop exactly as the capstone does. | https://documentation.softwareag.com/natural/nat911mf/sm/histogra.htm | 2026-08-04 |
| 9.1 to 9.5 | All code samples are valid Natural | CONFIRMED against documented syntax, with one portability defect | Every statement used (DEFINE DATA / VIEW OF, MOVE, STORE, FIND, READ BY, COMPUTE ROUNDED, ADD, DECIDE FOR FIRST CONDITION with the mandatory WHEN NONE, DISPLAY, WRITE, HISTOGRAM, UPDATE, DELETE, END TRANSACTION, BACKOUT TRANSACTION) matches the documented syntax, and no sample puts UPDATE or DELETE on the selecting statement line. The portability defect: every view declares `2 SALARY` as a flat elementary field. In the Software AG demo EMPLOYEES file that Natural documentation uses, SALARY lives inside the INCOME periodic group, and the official view is written `2 INCOME (1:3)` with `3 SALARY` beneath it. The course ships its own flat DDM (crates/natural-core/src/data.rs defines EMPLOYEES with SALARY as packed, 9 digits, no periodic group), so the samples are internally consistent, but a learner who retypes them against a real demo file gets a compile error. | https://documentation.softwareag.com/natural/nat911unx/pg/pg_output_index.htm | 2026-08-04 |
| 10 lede and 10.1 | "REINPUT sends the operator back to the screen with a message saying why." and the code line `REINPUT 'You must be at least 18. Try again.'` | CONFIRMED | Doc: "The REINPUT statement is used to return to and re-execute an INPUT statement. It is generally used to display a message indicating that the data input as a result of the previous INPUT statement were invalid." The message operand sits in the WITH TEXT option, and both keywords print inside square brackets, so `REINPUT 'literal'` is legal shorthand for `REINPUT WITH TEXT 'literal'`. | https://documentation.softwareag.com/natural/nat911mf/sm/reinput.htm , https://documentation.softwareag.com/natural/nat913unx/sm/reinput.htm | 2026-08-04 |
| 10.2 | "there is no REPEAT here. REINPUT itself sends control back to the INPUT, so the validation loop is built into the statement." | CONFIRMED | The statement genuinely branches back to the INPUT and re-executes it, so no enclosing loop is required for the interactive case. One behavior the lesson should state, because a learner will notice it on screen: without the FULL option, "the contents of variables that were changed between the INPUT and REINPUT statement will not be displayed; that is, all variables on the screen will show the contents they had when the INPUT statement was originally executed." | https://documentation.softwareag.com/natural/nat913unx/sm/reinput.htm | 2026-08-04 |
| 10.2 | "A REINPUT with no INPUT above it is an error, because there is nothing to go back to." | CONFIRMED, reasoning imprecise | The error exists and is NAT1108, "REINPUT statement not preceded by INPUT statement". But the documented rule is dynamic, not lexical: "When a REINPUT statement is to be executed, the last communication with the screen must have been via an INPUT statement. REINPUT is not permitted for a screen that was produced by a WRITE or DISPLAY statement." So a program can have an INPUT above the REINPUT and still fail, if a WRITE or DISPLAY ran in between. A second failure mode has its own message, NAT1113, when the INPUT sits in a subroutine that was not entered via PERFORM. | https://documentation.softwareag.com/natural/nat841unx/mc/mcERRN_1101.htm | 2026-08-04 |
| 10 (checklist item) | Whether REINPUT has documented options the course omits | MISLEADING by omission | The lesson teaches only the bare message form. Real code uses: FULL (re-execute the INPUT fully); MARK, documented as "you can mark a specific field, that is, specify a field in which the cursor is to be placed when the REINPUT statement is executed", plus MARK POSITION for a position inside a field; ALARM, "causes the sound alarm feature of the terminal to be activated"; USING HELP; display attributes on the message; and the message-number form `WITH TEXT *operand1`, where operand1 is the number of a message text retrieved from a Natural message file, with negative numbers addressing Natural system messages. Missing message numbers give NAT1149, "Requested message is not available". Cursor placement in particular is the first thing a learner meets in production validation code. | https://documentation.softwareag.com/natural/nat913unx/sm/reinput.htm , https://documentation.softwareag.com/natural/nat841unx/mc/mcERRN_1101.htm | 2026-08-04 |
| 10 (whole lesson) | Lesson presents REINPUT as the general validation idiom | MISLEADING by omission | Two documented restrictions are never stated. First, "The REINPUT statement is not valid in batch mode", enforced at runtime by NAT1109, "REINPUT cannot be executed in batch mode. A program containing a REINPUT statement cannot be executed in batch mode." Batch is where a large share of real Natural runs, so a learner taught that validation loops need no REPEAT will write a batch program that dies. Second, "No WRITE or DISPLAY statements may be executed between an INPUT statement and its corresponding REINPUT statement", which is precisely the trap a learner falls into after eight lessons of WRITE. | https://documentation.softwareag.com/natural/nat911mf/sm/reinput.htm , https://documentation.softwareag.com/natural/nat841unx/mc/mcERRN_1101.htm | 2026-08-04 |
| 10.1 | Code sample validity | CONFIRMED | `INPUT 'Age?' #AGE`, the IF block, the REINPUT with a literal message, and the trailing WRITE all match documented syntax. The WRITE sits after the IF block, so it executes only on the accepted path and never between the INPUT and the REINPUT. The sample does not violate the WRITE restriction; the lesson simply never tells the learner the restriction exists. | https://documentation.softwareag.com/natural/nat911unx/sm/input1.htm , https://documentation.softwareag.com/natural/nat911mf/sm/reinput.htm | 2026-08-04 |

## Corrections required

1. Rewrite the 9.3 teaching point. The current sentence, "On a real system an uncommitted
   transaction is backed out when the program ends. The work is simply gone", is false on
   default settings and backwards on a common non-default setting. An accurate replacement:
   ending a program does not end a transaction. On defaults (ETEOP=OFF) the updates stay
   pending and the records stay locked in exclusive hold after the program returns to NEXT,
   and a later program in the same session can still commit or back them out. The automatic
   backout happens when the Natural session ends (ENDBT=ON is the default) or when the
   session or system fails. Some sites set ETEOP=ON, in which case Natural issues the
   END TRANSACTION for you at end of program and the work is committed rather than lost. The
   honest lesson is that leaving a transaction open is unpredictable and locks records, not
   that the work is always discarded.

2. Fix the 9.3 prose that refers to "The second FIND". The sample has one FIND. Either add
   the second FIND the prose describes, or rewrite the sentence to describe what the program
   actually does.

3. Fix the 9.5 capstone. The prose promises "a filtered update, and a commit you can verify"
   and the code delivers neither. Add an UPDATE under a condition plus an END TRANSACTION, or
   remove both items from the prose list. Leaving lesson 9's two central statements out of
   lesson 9's capstone is the most visible defect in the module after item 1.

4. Add the batch-mode restriction to lesson 10. State that REINPUT is interactive only, that
   a batch program hits NAT1109, and that batch validation is written with a loop and error
   reporting instead. Without this the lesson's "you did not have to write a loop" framing
   teaches a habit that fails in batch.

5. Add the WRITE and DISPLAY restriction to lesson 10, with the error number. A learner who
   adds a diagnostic WRITE between INPUT and REINPUT gets NAT1108 and, given the lesson's
   current explanation, will look for a missing INPUT rather than the WRITE.

6. Tighten the 10.2 sentence about a REINPUT with no INPUT "above it". The rule is that the
   last screen communication must have been an INPUT, which is a run-time condition, not a
   question of what appears earlier in the source.

7. Fix the 9.1 sentence "the END TRANSACTION at the bottom". It is in the middle of the
   program. Say "after the STORE" or "before the verification READ".

8. Introduce exclusive hold in lesson 9. UPDATE and DELETE place the record in exclusive hold
   automatically, and only END TRANSACTION or BACKOUT TRANSACTION releases it. That is the
   real-world consequence of a forgotten commit on a shared system, and it is the missing
   half of the 9.4 explanation of BACKOUT TRANSACTION.

9. Add a one-line note that END TRANSACTION accepts transaction data, `END [OF] TRANSACTION
   [operand1]`, read back with GET TRANSACTION DATA, so a learner is not surprised by the form
   in production code. Note also that END OF TRANSACTION is the same statement.

10. Decide what to say about the flat SALARY field. The course DDM is a deliberate teaching
    simplification, but the file is named EMPLOYEES, which is the Software AG demo file, and in
    that file SALARY sits inside the INCOME periodic group and needs index notation. One
    sentence in the module telling learners that the course dataset is flattened prevents a
    confusing first day on a real system.

11. Optionally add the REINPUT options a learner meets immediately: MARK for cursor
    placement, ALARM, FULL, and the `WITH TEXT *nnnn` message-number form. MARK is the one that
    matters most, since production validation routines nearly always position the cursor on the
    offending field.

## Sources

All accessed 2026-08-04.

- STORE statement: https://documentation.softwareag.com/natural/nat911mf/sm/store.htm
- UPDATE statement: https://documentation.softwareag.com/natural/nat911mf/sm/update.htm
- DELETE statement: https://documentation.softwareag.com/natural/nat911mf/sm/delete.htm
- END TRANSACTION statement: https://documentation.softwareag.com/natural/nat911mf/sm/endtrans.htm
- BACKOUT TRANSACTION statement: https://documentation.softwareag.com/natural/nat911mf/sm/backout.htm
- HISTOGRAM statement: https://documentation.softwareag.com/natural/nat911mf/sm/histogra.htm
- REINPUT statement (mainframe): https://documentation.softwareag.com/natural/nat911mf/sm/reinput.htm
- REINPUT statement (cross-check, syntax detail): https://documentation.softwareag.com/natural/nat913unx/sm/reinput.htm
- INPUT statement syntax 1: https://documentation.softwareag.com/natural/nat911unx/sm/input1.htm
- ETEOP profile parameter (mainframe): https://documentation.softwareag.com/natural/nat911mf/parms/eteop.htm
- ETEOP profile parameter (cross-check, UNIX): https://documentation.softwareag.com/natural/nat911unx/parms/eteop.htm
- ETIO profile parameter: https://documentation.softwareag.com/natural/nat911mf/parms/etio.htm
- ENDBT profile parameter (mainframe): https://documentation.softwareag.com/natural/nat911mf/parms/endbt.htm
- ENDBT profile parameter (cross-check): https://documentation.softwareag.com/natural/nat912mf/parms/endbt.htm
- Natural system error messages 1101 to 1150 (NAT1108, NAT1109, NAT1113, NAT1149): https://documentation.softwareag.com/natural/nat841unx/mc/mcERRN_1101.htm
- Adabas concepts, transaction logic: https://documentation.softwareag.com/adabas/ada744mfr/adamf/concepts/cfusing.htm
- Adabas ET command: https://documentation.softwareag.com/adabas/ada854mfr/comref/et.htm
- Adabas CL command: https://documentation.softwareag.com/adabas/ada854mfr/comref/cl.htm
- Index notation for multiple-value fields and periodic groups (demo EMPLOYEES view): https://documentation.softwareag.com/natural/nat911unx/pg/pg_output_index.htm
- Course DDM under audit: /home/michael/repos/portfolio/course-natural-the-mainframe-language/crates/natural-core/src/data.rs
