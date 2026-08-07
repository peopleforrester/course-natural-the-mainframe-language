# Adversarial fact-check: Lesson 14 (Maps) and Lesson 15 (Capstone)

Audit date: 2026-08-04. Method: every claim checked against official Software AG
Natural documentation at documentation.softwareag.com, plus IBM and 3270
programmer references for the hardware claims. Verdicts are CONFIRMED, REFUTED,
MISLEADING, or UNVERIFIED. Nothing is accepted on the basis of plausibility.

Headline: the map syntax taught in Lesson 14 and reused throughout Lesson 15 is
not Natural. There is no DEFINE MAP statement, no TEXT statement, and no FIELD
statement in the Natural language. Every code sample in both lessons would be
rejected by a real Natural compiler at the first `DEFINE MAP` line.

## Findings table

| Lesson.Step | Claim (quoted) | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| 14.1, 14.2, 14.3, 15.1, 15.2 | `DEFINE MAP EMPLOYEE-ENTRY` ... `END-MAP` (inline map block inside the program source) | **REFUTED** | There is no `DEFINE MAP` statement and no `END-MAP` statement in Natural. The complete set of DEFINE statements is DEFINE CLASS, DEFINE DATA, DEFINE FUNCTION, DEFINE PRINTER, DEFINE PROTOTYPE, DEFINE SUBROUTINE, DEFINE WINDOW, DEFINE WORK FILE. No Natural statement contains the word MAP at all. A map is a separate object: "The Natural map editor is used to create a Natural object of type map. A map is a screen layout that can be referenced in a Natural object such as a program by using either an INPUT USING MAP statement (for input maps) or a WRITE USING MAP statement (for output maps)." Once created, "a map has been stored as a source object and a cataloged object in a library in a Natural system file." | https://documentation.softwareag.com/natural/nat911mf/sm/sm-over.htm ; https://documentation.softwareag.com/natural/nat914unx/edis/edis_ux_map.htm | 2026-08-04 |
| 14.1, 14.2, 14.3, 15.1, 15.2 | `TEXT 2 25 'EMPLOYEE MAINTENANCE'` and `FIELD 7 10 'Name:' #NAME` | **REFUTED** | Neither TEXT nor FIELD is a Natural statement. They do not appear in the Natural statement list. In a real map, text constants and data fields are placed with the map editor, not with source statements. The map "contains text fields (literal strings) and data fields (variables, which can be user-defined or Natural system variables)", but those are editor artifacts stored in the map object. | https://documentation.softwareag.com/natural/nat911mf/sm/sm-over.htm ; https://documentation.softwareag.com/natural/nat914unx/edis/edis_ux_map.htm | 2026-08-04 |
| 14.1, 14.2, 14.3, 15.1, 15.2 | (implicit) that a program can lay out a screen inline | **MISLEADING as taught** | Natural does support inline screen layout, but with the INPUT statement itself (Syntax 1, dynamic screen layout), not with a map block. The positional operands are documented: "x/y - Places the next element on line x, beginning in column y. y must not be zero. Backward positioning in the same line is not permitted", plus `nX` ("causes n spaces to be inserted between fields") and `nT` (tabulation). The course invented a third thing that exists in neither form. | https://documentation.softwareag.com/natural/nat912unx/sm/input1.htm | 2026-08-04 |
| 14.1 | `INPUT USING MAP EMPLOYEE-ENTRY` (statement name) | **CONFIRMED** | INPUT USING MAP is the correct statement for reading an input map. "This form of the INPUT statement is used to perform input processing using a map layout that has been created using the Natural map editor." | https://documentation.softwareag.com/naturalONE/natONE913/natmf/sm/input2.htm | 2026-08-04 |
| 14.1 | `INPUT USING MAP EMPLOYEE-ENTRY` (map name unquoted, 14 characters) | **REFUTED** | Two separate defects. First, quoting: "The map-name must be specified as an alphanumeric constant (up to 8 characters)." The official example is `INPUT USING MAP 'MAP001'`. Written bare, `EMPLOYEE-ENTRY` is parsed as a user-defined variable, which is a different syntax path and is undefined here. Second, length: "The name of a Natural object can be 1 to 8 characters". EMPLOYEE-ENTRY is 14. `RAISE-MAP` in 15.2 is 9. Both are illegal object names. Also: "The map used in this manner must have been created prior to the compilation of the program which references the map." | https://documentation.softwareag.com/naturalONE/natONE913/natmf/sm/input2.htm ; https://documentation.softwareag.com/natural/nat828mf/edis/mapt_mf_INPUT_USING.htm ; https://documentation.softwareag.com/natural/nat827mf/using/use_rules.htm | 2026-08-04 |
| 14.1 | "Type a value for each field and press Enter to move through them." | **MISLEADING** | On a 3270, Enter does not move between fields. Enter is an AID key: it raises an attention interrupt and transmits the whole screen, ending the INPUT. Field-to-field movement is Tab, Backtab, and the cursor keys. An AID is "which key on the keyboard was depressed by the terminal operator to cause the attention interrupt", and ENTER is listed among the AIDs (x'7D'). | https://www.prycroft6.com.au/misc/3270.html | 2026-08-04 |
| 14.2 | "Every field on a 3270 carries an attribute byte." | **CONFIRMED (imprecise wording)** | Correct in substance, loose in phrasing. The field does not carry the byte; the byte precedes the field and is itself part of the buffer. "A special 'Attribute Character' uses up one of the characters in the 3270 buffer and is always displayed as a blank on the terminal screen." Worth teaching, because it explains why a 3270 field always has a blank in front of it. | http://www.tommysprinkle.com/mvs/P3270/fields.htm ; https://www.prycroft6.com.au/misc/3270.html | 2026-08-04 |
| 14.2 | "A label is protected, so the operator cannot type into it." | **CONFIRMED** | True, and the mechanism is the attribute byte, not anything intrinsic to being a label. "If bit 2 is set to one, the field is protected which means it is an output field... no data can be entered from the keyboard into a protected field." In Natural, the map editor generates text constants as protected output fields; in a program the equivalent is the AD field mode group, where "AD=O - The value of the field is to be displayed during INPUT execution. The field is an output field and may not be modified." | http://www.tommysprinkle.com/mvs/P3270/fields.htm ; https://documentation.softwareag.com/natural/nat911win/parms/sp_ad.htm | 2026-08-04 |
| 14.2 | "A numeric field accepts digits only." | **MISLEADING** | Not true of the 3270 hardware attribute. The numeric bit only restricts the keyboard on terminals fitted with the Numeric Lock special feature; on keyboards without it, numeric-only permits any data into the field, and IBM's own guidance is that "the receiving program must still inspect the entry to ensure that it is a number of the form it expects." The lesson conflates the hardware attribute with Natural's own format validation of an N-format field, which is a separate mechanism applied by Natural at input time. | https://www.ibm.com/docs/SSGMCP_5.4.0/applications/designing/dfhp3at.html ; https://www.ibm.com/support/pages/3270-numeric-lock-setting-does-not-limit-keyboard-input-stated-ibm-personal-communications-line-help | 2026-08-04 |
| 14.2 | "(AD=I) intensifies a field" | **CONFIRMED** | Verbatim: "AD=I - The value of the field is displayed intensified." | https://documentation.softwareag.com/natural/nat911win/parms/sp_ad.htm | 2026-08-04 |
| 14.2 | "(AD=N) hides what is typed into it, which is how password fields have always worked" | **CONFIRMED** | Verbatim: "AD=N - A value entered in the field will not be displayed." Add one precision for the 3270 tie-in: AD=I and AD=N are both in the AD field-representation group, and on the 3270 basic attribute byte intensified and nondisplay are encoded in the same two-bit subfield, so a field cannot be both intensified and nondisplay. | https://documentation.softwareag.com/natural/nat911win/parms/sp_ad.htm ; https://www.prycroft6.com.au/misc/3270.html | 2026-08-04 |
| 14.2 | `#USER (AD=I)` parenthesized attribute form | **CONFIRMED (form only)** | The parenthesized field-level session parameter is correct Natural, for example `CALLNAT 'CNTEX1N' #FIELD1 (AD=M) #FIELD2 (AD=O)`. It is valid on an INPUT element or set in the map editor. It is not valid on the invented FIELD statement it appears on here. | https://documentation.softwareag.com/one/9.3.1/en/webhelp/one-webhelp/natux/sm/callnat.htm ; https://documentation.softwareag.com/natural/nat911win/parms/sp_ad.htm | 2026-08-04 |
| 14.3 | "The key that ends a screen is an AID key" | **CONFIRMED** | An AID identifies "which key on the keyboard was depressed by the terminal operator to cause the attention interrupt". Listed AIDs include ENTER, PF1 to PF24, PA1 to PA3, and CLEAR. | https://www.prycroft6.com.au/misc/3270.html | 2026-08-04 |
| 14.3 | "the program reads it from *PF-KEY" | **CONFIRMED** | *PF-KEY exists, format A4. "This system variable contains the identification of the key which was pressed last." Values are PA1 to PA3, PF1 to PF48, ENTR, CLR, PEN, PGDN, PGUP. Comparing against the literal 'PF3' is the correct format. | https://documentation.softwareag.com/natural/nat911mf/vari/inout.htm | 2026-08-04 |
| 14.3 | Question raised in the brief: is *PF-NAME the right variable instead? | **CONFIRMED that *PF-KEY is correct here** | Both exist and they are not interchangeable. *PF-KEY (A4) holds the key identification. *PF-NAME (A10) "contains the name of the function key that was pressed last, that is, the name as assigned to the key with the NAMED clause of the SET KEY statement." For a comparison against 'PF3', *PF-KEY is the right one. | https://documentation.softwareag.com/natural/nat911mf/vari/inout.htm | 2026-08-04 |
| 14.3 | `IF *PF-KEY = 'PF3'` ... "watch which branch runs" | **REFUTED as written** | The program as printed cannot take the PF3 branch. "*PF-KEY only contains the identification of a key if that key is currently sensitive; otherwise *PF-KEY will contain ENTR." Sensitivity is not automatic: the KEY profile parameter has "Default setting: none", and per SET KEY, when a key with no assigned function is pressed, "either a warning message will be issued prompting the user to press a valid key, or the value ENTR will be placed into the Natural system variable *PF-KEY; that is, Natural will react as if the ENTER key had been pressed." The program needs `SET KEY PF3` before the INPUT: "SET KEY PF2 ... causes PF2 to be made program-sensitive." | https://documentation.softwareag.com/natural/nat911mf/vari/inout.htm ; https://documentation.softwareag.com/natural/nat911unx/parms/key.htm ; https://documentation.softwareag.com/natural/nat911mf/sm/setkey.htm | 2026-08-04 |
| 14.3 | "Every 'PF3 to exit' convention in mainframe software is this one field." | **REFUTED** | *PF-KEY is a Natural system variable and exists only inside Natural. The PF3 equals End or Exit convention is an IBM SAA Common User Access convention that predates and surrounds Natural: ISPF panels conform to CUA and use PF3 for End independently of Natural, and CICS programs read the AID from EIBAID, not from *PF-KEY. The convention is shared; the field is not. | https://www.ibm.com/docs/en/zos/2.2.0?topic=panels-using-pf-keys ; https://www.ibm.com/docs/en/ibm-mq/9.0.x?topic=ocpmz-using-function-keys-command-line-ispf-control-panels-zos | 2026-08-04 |
| 15.1 | `CALLNAT 'COUNT-STAFF' #WHERE #HOWMANY` | **REFUTED** | The name is 11 characters. On Natural for Mainframes: "The name may be specified either as a constant of 1 to 8 characters, or - if different subprograms are to be called dependent on program logic - as an alphanumeric variable of length 1 to 8." Independently, "The name of a Natural object can be 1 to 8 characters", so no subprogram could ever be catalogued under that name. | https://documentation.softwareag.com/natural/nat911mf/sm/callnat.htm ; https://documentation.softwareag.com/natural/nat827mf/using/use_rules.htm | 2026-08-04 |
| 15.1, 15.2 | `1 EMPLOYEES-VIEW VIEW OF EMPLOYEES` with `2 SALARY` | **REFUTED against the standard demo file** | In the Software AG demo EMPLOYEES file, SALARY is a multiple-value field and every official example indexes it. The ACREX2 example prints exactly: `1 EMPLOY-VIEW VIEW OF EMPLOYEES` / `2 NAME` / `2 FIRST-NAME` / `2 SALARY    (1)`. Other official examples use `SALARY (1:1)`. Bare `2 SALARY` would be rejected. The same applies to `ADD #RISE TO SALARY` and `WRITE 'Before:' SALARY` in 15.2, which need the occurrence index. | https://documentation.softwareag.com/natural/nat911mf/sm/accept.htm | 2026-08-04 |
| 15.1 | `FIND EMPLOYEES-VIEW WITH COUNTRY = #WHERE SORTED BY NAME` | **CONFIRMED** | SORTED BY is a valid FIND clause: "SORTED [BY] descriptor ... [DESCENDING]", maximum three descriptors. It cannot be combined with the RETAIN clause, which is not used here. | https://documentation.softwareag.com/natural/nat911unx/sm/find.htm | 2026-08-04 |
| 15.1 | Statement ordering: `DEFINE SUBROUTINE LIST-THEM` appears after the `PERFORM LIST-THEM` and before `END` | **CONFIRMED** | Verbatim: "An inline subroutine may be defined before or after the first PERFORM statement which references it." The official first-steps example uses exactly this shape, with the DEFINE SUBROUTINE block sitting between the invoking logic and the final END. The only stated restriction is that "Any processing loop initiated within a subroutine must be closed before END-SUBROUTINE is issued", which the FIND/END-FIND pair satisfies. | https://documentation.softwareag.com/natural/nat911unx/sm/definesu.htm ; https://documentation.softwareag.com/natural/nat827mf/firststeps/fs-inlinesub.htm | 2026-08-04 |
| 15.1 | "Fill in a country code (try USA, UK, ESP, F, or CZ)" | **UNVERIFIED** | Cannot be checked against official documentation because it describes the course's own sample dataset. Flagging it anyway: #WHERE is declared A3 and the standard demo EMPLOYEES COUNTRY values are three-character codes. A one-character 'F' and a two-character 'CZ' in the same list look inconsistent with a three-byte descriptor and should be reconciled against the shipped dataset. | n/a | 2026-08-04 |
| 15.2 | `REINPUT 'Enter an amount greater than zero.'` after `INPUT USING MAP` | **CONFIRMED** | REINPUT does work after INPUT USING MAP. "The REINPUT statement is used to return to and re-execute an INPUT statement." The one relevant constraint is satisfied by luck rather than design: "No WRITE or DISPLAY statements may be executed between an INPUT statement and its corresponding REINPUT statement", and the sample happens to place the check before any WRITE. Also note "The REINPUT statement is not valid in batch mode." | https://documentation.softwareag.com/natural/nat841unx/sm/reinput.htm | 2026-08-04 |
| 15.2 | `IF NO RECORDS FOUND` / `WRITE 'No employee called' #WHO` / `END-NOREC` then falling through to `ADD #RISE TO SALARY` and `UPDATE` | **REFUTED** | This is a live functional bug, not a stylistic one. "If no records meet the specified WITH and WHERE criteria, the IF NO RECORDS FOUND clause causes the FIND processing loop to be executed once with an 'empty' record", and "Unless other value assignments are made in the statements accompanying an IF NO RECORDS FOUND clause, Natural will reset to empty all database fields which reference the file specified in the current loop." So after printing "No employee called", the program enters the loop body once and runs WRITE, ADD and UPDATE against a null record with no held ISN. The documented remedy is explicit: "If this is not desired, specify the statement ESCAPE BOTTOM within the IF NO RECORDS FOUND clause." UPDATE also has a hard precondition the empty record does not meet: "The record to be updated must have been previously selected with a FIND, GET or READ statement." | https://documentation.softwareag.com/natural/nat911unx/sm/find.htm ; https://documentation.softwareag.com/natural/nat912unx/sm/update.htm | 2026-08-04 |
| 15.2 | "read a screen, validate it, change the database, commit, and prove the change stuck" | **MISLEADING** | The stated behavior is not what the printed program does on the no-match path, per the row above. On the match path the description holds. The prose promises an outcome the code does not deliver for the case the code explicitly handles. | https://documentation.softwareag.com/natural/nat911unx/sm/find.htm | 2026-08-04 |
| 15.2 | `UPDATE` inside the FIND loop, `END TRANSACTION` placed after `END-FIND` | **CONFIRMED** | Both correct. "The use of the UPDATE statement causes each record read for processing in the corresponding FIND or READ statement to be placed in exclusive hold", and "END TRANSACTION has to be placed outside such a loop or after the outermost loop of nested loops." The END TRANSACTION also "results in the release of all records placed in hold status during the transaction", so the re-read after it is sound. | https://documentation.softwareag.com/natural/nat912unx/sm/update.htm ; https://documentation.softwareag.com/natural/nat827mf/sm/endtrans.htm | 2026-08-04 |
| 15.2 | Overall: "the whole program would be valid Natural" | **REFUTED** | It would not compile. The DEFINE MAP block, the TEXT and FIELD statements, the unquoted and over-length map name, the over-length CALLNAT name, and the unindexed SALARY are each independently fatal. Correcting only the runtime logic would still leave a program a real compiler rejects at line one of the map block. | (composite of rows above) | 2026-08-04 |

## Corrections required

**C1. Stop teaching DEFINE MAP as Natural syntax.** This is the defect that
matters most, because a learner will carry it to a real system and it will fail
immediately. Either state plainly, in the lesson body and not in a footnote,
that DEFINE MAP is a course-only shorthand invented so the browser interpreter
can show a map without a map editor, or replace it with something real. The two
real options are: build the map as a separate object in the map editor and
reference it, which the browser cannot do; or use the INPUT statement's own
dynamic screen layout, which the browser can do and which is genuine Natural.
The second is the recommendation. `INPUT 02/25 'EMPLOYEE MAINTENANCE' 07/10
'Name:' #NAME 09/10 'Dept:' #DEPT` teaches the same screen positioning, the same
AD attributes, and the same suspend-and-resume behavior, and it compiles on a
real system. If the course keeps DEFINE MAP, it must say in the same screen of
text that no Natural compiler accepts it and show the real INPUT USING MAP plus
map-object arrangement alongside.

**C2. Fix the map and subprogram names.** Natural object names are 1 to 8
characters. Rename EMPLOYEE-ENTRY, RAISE-MAP, and COUNT-STAFF. Quote map names
in INPUT USING MAP, as in `INPUT USING MAP 'EMPMAINT'`. State the 8-character
limit explicitly in the lesson, because it surprises every learner coming from a
modern language and it silently shapes every naming convention in a real Natural
shop.

**C3. Add SET KEY to Lesson 14.3.** As printed, the PF3 branch is unreachable:
an unsensitized PF key puts ENTR in *PF-KEY, so the ELSE branch always runs and
the lesson's own instruction to "watch which branch runs" produces the wrong
result. Add `SET KEY PF3` before the INPUT and explain program sensitivity. This
is a teaching opportunity rather than a burden, because key sensitivity is
exactly the kind of mainframe-specific behavior the course exists to convey.

**C4. Add ESCAPE BOTTOM to the capstone's IF NO RECORDS FOUND clause.** Without
it the program updates an empty record after announcing that no record was
found. This is the documented default, not an edge case. The fix is one line:
put `ESCAPE BOTTOM` after the WRITE and before END-NOREC. This behavior also
deserves a callout of its own, because the empty-loop-pass surprises people who
assume the clause works like an else branch.

**C5. Index SALARY.** Write `2 SALARY (1)` in the view and `SALARY (1)` at every
use, matching the official examples, or change the course dataset's field so
SALARY is a genuine scalar and say so. Leaving it bare teaches a form that fails
against the demo file the course says it calibrates against.

**C6. Correct the numeric-field claim.** "A numeric field accepts digits only"
should become something like: the numeric attribute bit asks the terminal to
restrict the keyboard, but it only enforces on terminals with the Numeric Lock
feature, so the program must still validate. Then show that Natural's N-format
validation is what actually protects you. This is more accurate and more useful.

**C7. Correct the Enter-moves-between-fields sentence.** Tab moves between
fields. Enter is an AID key and transmits the screen. If the course's browser
terminal genuinely advances fields on Enter, that divergence from a real 3270
must be stated, because it will confuse anyone who later touches a real emulator.

**C8. Soften the PF3 attribution.** "Every 'PF3 to exit' convention in mainframe
software is this one field" is false. Rewrite as: PF3 for End or Exit is an IBM
SAA Common User Access convention that ISPF, CICS applications, and Natural
applications all follow, and *PF-KEY is how a Natural program participates in it.

**C9. Reconcile the country codes** in the Lesson 15.1 prompt with the shipped
dataset and with the A3 declaration of #WHERE.

**C10. Minor, worth doing.** Wrap the map read in a REPEAT loop as the official
PROG001 example does, so the capstone shows the real shape of a maintenance
program rather than a single-shot read. Mention that the canonical Natural place
for the "amount greater than zero" check is a map processing rule containing a
REINPUT with MARK, since that is what a learner will encounter in existing code.

## Sources

All accessed 2026-08-04.

Software AG Natural, official documentation:

- Statements overview, full statement inventory: https://documentation.softwareag.com/natural/nat911mf/sm/sm-over.htm
- Map Editor, what a map object is and how it is stored: https://documentation.softwareag.com/natural/nat914unx/edis/edis_ux_map.htm
- Map (programming guide object type): https://documentation.softwareag.com/natural/nat914unx/pg/pg_obj_map.htm
- INPUT Syntax 2, using a predefined map layout: https://documentation.softwareag.com/naturalONE/natONE913/natmf/sm/input2.htm
- Invoking a Map with INPUT USING MAP, PROG001 example: https://documentation.softwareag.com/natural/nat828mf/edis/mapt_mf_INPUT_USING.htm
- INPUT Syntax 1, dynamic screen layout and x/y positioning: https://documentation.softwareag.com/natural/nat912unx/sm/input1.htm
- AD, Attribute Definition session parameter: https://documentation.softwareag.com/natural/nat911win/parms/sp_ad.htm
- Input/Output-Related System Variables, *PF-KEY and *PF-NAME: https://documentation.softwareag.com/natural/nat911mf/vari/inout.htm
- SET KEY statement, program sensitivity: https://documentation.softwareag.com/natural/nat911mf/sm/setkey.htm
- KEY profile parameter, default setting: https://documentation.softwareag.com/natural/nat911unx/parms/key.htm
- REINPUT statement: https://documentation.softwareag.com/natural/nat841unx/sm/reinput.htm
- Processing Rules, REINPUT in map validation: https://documentation.softwareag.com/natural/nat911mf/edis/map_mf_proc_rules.htm
- FIND statement, IF NO RECORDS FOUND and SORTED BY: https://documentation.softwareag.com/natural/nat911unx/sm/find.htm
- UPDATE statement, record selection precondition and hold: https://documentation.softwareag.com/natural/nat912unx/sm/update.htm
- END TRANSACTION statement, loop placement and hold release: https://documentation.softwareag.com/natural/nat827mf/sm/endtrans.htm
- DEFINE SUBROUTINE statement, inline placement rule: https://documentation.softwareag.com/natural/nat911unx/sm/definesu.htm
- Inline Subroutines, first steps example: https://documentation.softwareag.com/natural/nat827mf/firststeps/fs-inlinesub.htm
- CALLNAT statement, mainframe operand1 length: https://documentation.softwareag.com/natural/nat911mf/sm/callnat.htm
- Rules and Naming Conventions, 1 to 8 character object names: https://documentation.softwareag.com/natural/nat827mf/using/use_rules.htm
- ACCEPT/REJECT, ACREX2 example with SALARY (1): https://documentation.softwareag.com/natural/nat911mf/sm/accept.htm

IBM and 3270 references:

- CICS 3270 field attributes, numeric attribute and Numeric Lock: https://www.ibm.com/docs/SSGMCP_5.4.0/applications/designing/dfhp3at.html
- IBM support note, 3270 Numeric Lock does not limit keyboard input as documented: https://www.ibm.com/support/pages/3270-numeric-lock-setting-does-not-limit-keyboard-input-stated-ibm-personal-communications-line-help
- 3270 Data Stream Programming, attribute character and bit assignments: http://www.tommysprinkle.com/mvs/P3270/fields.htm
- 3270 Programming Overview, attribute bytes and AID keys: https://www.prycroft6.com.au/misc/3270.html
- z/OS, Using PF keys, PF3 for End: https://www.ibm.com/docs/en/zos/2.2.0?topic=panels-using-pf-keys
- IBM MQ on z/OS, ISPF function keys conform to CUA standards: https://www.ibm.com/docs/en/ibm-mq/9.0.x?topic=ocpmz-using-function-keys-command-line-ispf-control-panels-zos
