# Adversarial fact-check: Lesson 1 and Lesson 2

Audited 2026-08-04 against official Software AG documentation at
documentation.softwareag.com and softwareag.com, with secondary sources used only
where Software AG publishes nothing on the point (company history).

Scope: `tmp/content-audit/lesson-01.txt` and `tmp/content-audit/lesson-02.txt`.

A note on method before the table. Software AG product documentation carries no
history section, so claims 1 and 5 cannot be settled from documentation.softwareag.com
at all. For those two I state the best available source and grade the claim against
it, and I say plainly where the evidence is secondary. Platform and syntax claims
were all settled against primary Software AG documentation.

## Findings

| Lesson.Step | Claim (quoted from the content) | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| 1.LEDE | "Natural is a fourth-generation language" | CONFIRMED | Software AG's own documentation uses the term. The Predict Application Audit user guide refers to "Software AG's 4GL Natural". Note that current Software AG marketing pages have dropped the term and describe Natural as a programming environment instead, so the label is historically correct rather than currently promoted. | https://documentation.softwareag.com/natural/pac261/ugpaa/paaover.htm | 2026-08-04 |
| 1.LEDE | "built as the native programming language of the ADABAS database" | MISLEADING | Historically defensible, but stated without qualification it will teach a learner that Natural requires Adabas. Official documentation: "Natural has a built-in data manipulation language (DML) that allows Natural applications to access all database systems supported by Natural using the same language statements." The documented set is Adabas (nested-relational), SQL-type DBMS (Db2, Oracle, Sybase, Informix, MS SQL Server), and VSAM file systems. The page does not privilege Adabas. | https://documentation.softwareag.com/one/9.3.2/en/webhelp/one-webhelp/natmf/pg/pg_dbms_dbgen.htm | 2026-08-04 |
| 1.LEDE | "It still runs payroll, benefits, and licensing systems in government and insurance" | UNVERIFIED | Government and insurance as sectors are supportable from Software AG's own customer material: the State of Delaware criminal justice system (DELJIS), a large US state Comptroller's office, and AAFMAA. I found no Software AG source naming payroll, benefits, or licensing as Natural workloads. The three named workloads are plausible industry lore, not documented fact. | https://www.softwareag.com/en/resources/adabas-natural/mission-critical-applications/ ; https://www.softwareag.com/en_corporate/resources/adabas-natural/cs/customers-customer-stories-office-of-the-comptroller.html | 2026-08-04 |
| 1.LEDE | "which is why people are still hired to maintain it" | UNVERIFIED | This is an inference about the labour market, not a documented claim. No Software AG source supports it. It is sales copy in a lesson that elsewhere promises documentation-verified accuracy. | n/a | 2026-08-04 |
| 1.1 | "Software AG developed Natural from 1975, under Peter Pag&eacute; with Margit Neumann." | CONFIRMED (secondary sources only) | The 1975 start date and both names check out across three independent secondary sources. Computerwoche's Hall of Fame entry for Dr. Peter Pag&eacute; records that he joined in 1971 and four years later developed Natural together with Margit Neumann. German Wikipedia: "Die ersten Versionen der ersten Variante fuer Grossrechner wurden ab 1975 von Peter Pag&eacute; unter Mitwirkung von Margit Neumann entwickelt." English Wikipedia: "From 1975, together with Margit Neumann, he developed the innovative software development environment Natural", cited to Computerwoche. The name "Margit Neumann" is spelled consistently in every source and her credit as co-developer is correct. No Software AG primary source exists. | https://www.computerwoche.de/hall-of-fame/dr-peter-page,27 ; https://de.wikipedia.org/wiki/Natural_(Programmiersprache) ; https://en.wikipedia.org/wiki/Peter_Pag%C3%A9 | 2026-08-04 |
| 1.1 | Deliberate avoidance of the commonly cited 1979 release date | CONFIRMED (the avoidance is still correct) | The 1979 figure traces to a single uncited sentence on the English Wikipedia article for Software AG: "In 1979, Natural, a 4GL application development English-like language, that was mainly developed by Peter Pag&eacute;, was launched." No footnote is attached to it. I found no Software AG press release, anniversary page, or documentation establishing any release year. Keep avoiding 1979. | https://en.wikipedia.org/wiki/Software_AG | 2026-08-04 |
| 1.1 | "It was designed around ADABAS, so reading and writing database records is part of the language rather than a library bolted on." | CONFIRMED | Database access is genuinely at statement level, not library level. Official documentation names FIND, READ, STORE and DELETE as built-in DML statements and states that Natural "translates the DML statements into database-specific commands". The second half of the sentence is exactly right. Only the "designed around ADABAS" framing needs the qualification noted in the LEDE row. | https://documentation.softwareag.com/one/9.3.2/en/webhelp/one-webhelp/natmf/pg/pg_dbms_dbgen.htm | 2026-08-04 |
| 1.1 | "Today it runs on IBM z/OS and Linux, plus Windows and containers." | CONFIRMED | Software AG's Natural for Linux and Cloud page states that Natural "operates on the following operating systems: Linux (on-premises or Cloud)" and "Windows", and adds "Natural is also available on IBM z/OS." The same page describes Natural applications as "cloud native, using microservices and running in containers", and Software AG publishes an official Natural container image. Natural for Windows 9.3.2 documentation is dated July 2025, so Windows is a live target and not a legacy one. | https://www.softwareag.com/en/resources/adabas-natural/natural-for-linux-and-cloud/ ; https://hub.docker.com/r/softwareag/natural-ce ; https://documentation.softwareag.com/natwin/9.3.2/en/webhelp/natwin-webhelp/relnotes/rn-932.htm | 2026-08-04 |
| 1.1 | "Older platforms ... (AIX, Solaris, HP-UX) have all reached end of maintenance" | CONFIRMED with dates | Natural release notes for 9.3.1 and 9.3.2 state: "The end-of-maintenance date (EOM) for Software AG support of the Legacy-Unix platforms is December 31, 2024." and "For the period from December 31, 2024 to December 31, 2025 Software AG will offer options for non-standard sustained support on the Legacy-Unix platforms." Both dates are past as of 2026-08-04. Same notes: "Software AG has decided Linux x86 will be its strategic open systems platform for Adabas and Natural 2050+ going forward." | https://documentation.softwareag.com/natux/9.3.1/en/webhelp/natux-webhelp/relnotes/rn-931.htm | 2026-08-04 |
| 1.1 | "Older platforms ... (z/VSE, BS2000) have all reached end of maintenance" | MISLEADING | The direction of travel is right but the wording overstates what Software AG has published. What official documentation actually says is a support-drop, not an end-of-maintenance date: "Starting with Natural for Mainframe version 9.2, BS2000 and VSE platforms are no longer supported." and "Natural add-on products are no longer delivered for the z/VSE and BS2000 operating systems." Current Software AG platform pages name only z/OS on the mainframe side. However, Natural 9.1.x shipped BS2000 and z/VSE installation guides as recently as October 2023, and Software AG publishes end-of-maintenance dates only behind the Empower login, which I cannot reach. So "reached end of maintenance" is a stronger and differently-shaped claim than the evidence supports. | https://documentation.softwareag.com/nop/5.5.3/en/webhelp/nop-webhelp/rnotes/rnotes.htm ; https://www.softwareag.com/en/resources/adabas-natural/ibm-platforms-products/ ; https://documentation.softwareag.com/natural/nat912mf/pdf/inst_bs2.pdf | 2026-08-04 |
| 1.1 | "Adabas & Natural has been a standalone business under Software GmbH since January 2025." | CONFIRMED | Press release dateline "DARMSTADT, Germany, Jan. 7, 2025". The release states that "Adabas & Natural (A&N) and ARIS are each making significant investments in product innovation and talent, in order to execute on their multi-year growth plans" as standalone businesses, and that Software GmbH "continues to be the holding company for ARIS, Adabas & Natural (A&N) and Software AG's central functions." Both the month and the holding-company relationship are correct as written. | https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html ; https://www.softwareag.com/en/blog/insights/adabas-natural-and-aris-launch-as-standalone/ | 2026-08-04 |
| 1.1 | Prose rendering: "bolted on.Today it runs" and "current targets.Adabas & Natural" | REFUTED (published defect) | Two sentence boundaries in the Step 1.1 prose have no space after the full stop. This is live on a paid course page. | n/a (source file line 9) | 2026-08-04 |
| 1.2 | Code: `WRITE 'Hello from the mainframe.'` / `END` | CONFIRMED valid Natural | Both statements are real, correctly spelled, correctly ordered, and the literal delimiters are legal. A real Natural compiler accepts this. See the 2.4 row for what it actually prints. | https://documentation.softwareag.com/natural/nat912win/sm/write.htm | 2026-08-04 |
| 1.3 | "Everything you write here is real Natural syntax, verified against the official documentation." | MISLEADING | True of the syntax, false of the semantics the course then teaches. Every code sample in these two lessons is valid Natural, but the output model taught in 2.4 is not what real Natural produces (see below). A promise pitched at this level of confidence should not be made while a documented output-behaviour mismatch is live. | n/a | 2026-08-04 |
| 2.LEDE / 2.1 | "Every Natural program ends with END." / "END terminates the program, and every program needs one." | CONFIRMED | Programming Guide, verbatim: "The END statement is used to mark the end of a Natural program, function, subprogram, external subroutine or helproutine." "Every one of these objects must contain an END statement as the last statement." "Every object may contain only one END statement." | https://documentation.softwareag.com/natural/nat912win/pg/pg_furth_end.htm | 2026-08-04 |
| 2.LEDE / 2.1 | "WRITE puts a line on the screen." / "WRITE outputs a line." | CONFIRMED as a simplification | Official: "The WRITE statement is used to produce output in free format (that is, not in columns)." and "If necessary, it automatically creates a line advance; that is, a field or text element that does not fit onto the current output line, is automatically output in the next line." Two caveats the course should eventually own: a single WRITE can emit more than one line (slash notation between fields, and line overflow), and WRITE writes to a report, of which the screen is only the default report 0. Neither is wrong at this teaching level. | https://documentation.softwareag.com/natural/nat912unx/pg/pg_output_display.htm ; https://documentation.softwareag.com/natural/nat912win/sm/write.htm | 2026-08-04 |
| 2.1 | Code: two `WRITE` statements then `END` | CONFIRMED valid Natural | Compiles. See 2.4 for the output-count problem. | https://documentation.softwareag.com/natural/nat912win/sm/write.htm | 2026-08-04 |
| 2.2 | "Write a quote inside a text literal by doubling it. This is the documented Natural convention." | CONFIRMED on the rule, MISLEADING on the word "quote" | The rule is exactly documented. User-Defined Constants: "If you want an apostrophe to be part of an alphanumeric constant that is enclosed in apostrophes, you must write this as two apostrophes or as a single quotation mark." The documented example is `WRITE 'HE SAID, ''HELLO'''`. The TQMARK parameter page removes any doubt: "Do not confuse quotation mark (\") with double apostrophes (''). Double apostrophes within a text constant are always output as a single apostrophe ('), regardless of the setting of the TQMARK parameter." The defect is vocabulary. Natural treats the apostrophe (') and the quotation mark (") as different characters with different rules, and the doubling rule taught here applies to the apostrophe. A learner who reads "quote" as "quotation mark" and writes `WRITE 'He said ""hi""'` will not get what the lesson led them to expect. | https://documentation.softwareag.com/natural/nat841unx/pg/pg_defi_udc.htm ; https://documentation.softwareag.com/natural/nat912unx/parms/tqmark.htm | 2026-08-04 |
| 2.2 | Code: `WRITE 'It''s a mainframe.'` / `END` | CONFIRMED valid Natural | Matches the documented example form character for character. | https://documentation.softwareag.com/natural/nat841unx/pg/pg_defi_udc.htm | 2026-08-04 |
| 2.3 | "Leave the END off and see what happens" / omitting END is an error | CONFIRMED | Follows directly from "Every one of these objects must contain an END statement as the last statement." The deliberately broken sample is correctly broken. Separately: I could not find a Natural system error message dedicated to a missing END. I checked the NAT0001-0049 and NAT0900-0949 ranges and the Messages and Codes catalogues. The general syntax error is NAT0001 "Missing/invalid syntax; undefined variable name/keyword." Treat any specific NAT number in the interpreter's diagnostic as UNVERIFIED and do not invent one. | https://documentation.softwareag.com/natural/nat912win/pg/pg_furth_end.htm ; https://documentation.softwareag.com/natural/nat828mf/mc_mf/mcERRN_0001.htm | 2026-08-04 |
| 2.4 | "Write a program that outputs exactly three lines: ONE, TWO, then THREE." | REFUTED | Achievable with the taught syntax in the sense the course means, but false as a statement about Natural. Real Natural adds a page title. Programming Guide, verbatim: "For each page output via a DISPLAY or WRITE statement, Natural automatically generates a single default title line that contains the page number, the date and the time of day." The documented example output for a single `WRITE 'HELLO'` is a title line, then a blank line, then HELLO. Three WRITE statements plus END therefore produce five lines of report output, not three. To get exactly three lines the first output statement must carry NOTITLE: `WRITE NOTITLE 'ONE'`. This is the most consequential finding in the audit, because it is not a typo but a divergence between the teaching interpreter's output model and documented Natural behaviour, and it propagates to Lesson 1.2 and Lesson 2.1 as well. | https://documentation.softwareag.com/natural/nat912win/pg/pg_output_titles.htm ; https://documentation.softwareag.com/natural/nat912mf/pg/pg_output_titles.htm | 2026-08-04 |
| 2.4 | Exercise starter: `WRITE 'ONE'` | CONFIRMED valid Natural | Valid as a statement. Wrong as the seed for a three-line-output exercise, per the row above. | https://documentation.softwareag.com/natural/nat912win/sm/write.htm | 2026-08-04 |

## Code sample validity sweep

Every code sample across both lessons was checked statement by statement against
the Natural statements reference. Result: no invented keywords, no invalid syntax
forms, no statement a real Natural compiler would reject, with the single intended
exception of Step 2.3, which is deliberately missing its END.

- `WRITE` is a real statement, spelled correctly, used with legal alphanumeric literals.
- `END` is a real statement, correctly placed last, appearing exactly once per object.
- Alphanumeric literals use apostrophes, which is one of the two documented delimiters.
- The doubled apostrophe in `'It''s a mainframe.'` matches the documented example exactly.
- No sample uses a variable, so the absence of DEFINE DATA is legal in both reporting
  mode and structured mode.
- All literals are well inside the documented 1 to 72 character limit for text notation.

The syntax is clean. The problem is output semantics, not syntax.

## Corrections required

Ordered worst first.

**1. Lesson 2.4, exercise task and the whole output model.** The course teaches an
output that documented Natural does not produce. Pick one of two fixes and apply it
consistently across Lesson 1.2, Lesson 2.1, Lesson 2.2 and Lesson 2.4.

The honest fix is to teach NOTITLE now. Change the exercise starter to
`WRITE NOTITLE 'ONE'` and add one sentence to Step 2.1: "Natural puts a page title
line with the page number, date and time above your output. NOTITLE on the first
output statement suppresses it." Then the sample code and the promise in Lesson 1.3
both hold.

The cheaper fix is to keep the samples as they are and correct the claim, changing
"outputs exactly three lines" to "outputs the three lines ONE, TWO and THREE", plus a
one-line note in Lesson 1.3 stating that the teaching terminal suppresses Natural's
default page title so lessons stay focused on the statement being taught. That
preserves accuracy but leaves a known divergence between the course and real Natural,
which the Lesson 1.3 honesty section must then disclose rather than hide.

Do not leave the current wording. "Exactly three lines" is a testable claim and it is
false against the documentation.

**2. Lesson 1.1, the z/VSE and BS2000 sentence.** Replace "have all reached end of
maintenance and are no longer current targets" with wording that separates the two
groups, since only one of them has a published end-of-maintenance date. Suggested:
"BS2000 and z/VSE were dropped from Natural for Mainframes 9.2 onward. AIX, Solaris
and HP-UX reached end of maintenance on 31 December 2024, with sustained-support
options that ended on 31 December 2025. None of the five is a current target."

**3. Lesson 2.2, the word "quote".** Replace "Write a quote inside a text literal by
doubling it" with "Write an apostrophe inside a text literal by doubling it." Natural
distinguishes the apostrophe from the quotation mark, and the doubling rule taught
here is the apostrophe rule. Optionally add the second documented route, that a single
quotation mark inside apostrophe-delimited text also produces an apostrophe, though
that behaviour depends on the TQMARK profile parameter and is probably out of scope
for Lesson 2.

**4. Lesson 1 LEDE, the payroll/benefits/licensing claim.** Either cite it or soften
it. No Software AG source names those three workloads. Software AG's own public
customer material supports government and insurance as sectors and names the Delaware
criminal justice system and a large US state Comptroller's office. Suggested:
"It still runs core record-keeping systems in government and insurance." If the
specific triad matters for the sales pitch, source it from a named customer story and
cite that story.

**5. Lesson 1 LEDE, "native programming language of the ADABAS database".** Add one
qualifying clause so a learner does not conclude Natural is Adabas-only. Suggested:
"built as the native programming language of the ADABAS database, though it now reads
SQL databases and VSAM through the same statements." This is a small addition that
prevents a wrong mental model, and it is directly supported by the documentation.

**6. Lesson 1.1, the two missing sentence spaces.** Insert a space after "bolted on."
and after "current targets." This is live on a paid page.

**7. Lesson 1 LEDE, "which is why people are still hired to maintain it".** Not a
documented claim. Keep it if the course is comfortable labelling it as the author's
read of the market rather than a verified fact, but it sits inside a lesson that
closes by promising documentation-verified accuracy, so the mismatch is worth a
deliberate decision rather than an accident.

**8. Lesson 2.3, the interpreter's diagnostic.** No NAT error number for a missing END
could be found in the published catalogues. If the interpreter's message quotes a NAT
number, verify it or drop it. Naming the concept without a number is both safer and
what the course's own error-design principle already calls for.

## What was checked and found clean

- The 1975 date and the crediting of Peter Pag&eacute; and Margit Neumann. Three
  independent secondary sources agree, the name is spelled correctly, and Neumann's
  credit as co-developer is accurate. Software AG publishes no history, so no primary
  source exists to check against.
- The decision to avoid 1979. Still correct. The 1979 figure is an uncited Wikipedia
  sentence and I could find no primary source for any release year.
- The current platform list z/OS, Linux, Windows, containers. Correct and current.
- The January 2025 Software GmbH standalone-business claim. Correct, including the
  holding-company relationship.
- The END statement requirement. Correct and documented in exactly the terms the
  lesson uses.
- The doubled-apostrophe rule. Correct, documented, and confirmed to be independent of
  the TQMARK parameter.
- The "fourth-generation language" label. Software AG's own documentation uses it.
- Every code sample's syntax.

## Sources

Official Software AG documentation and product pages:

- Natural Programming Guide, End of Statement, Program or Application:
  https://documentation.softwareag.com/natural/nat912win/pg/pg_furth_end.htm
- Natural Programming Guide, Page Titles, Page Breaks, Blank Lines:
  https://documentation.softwareag.com/natural/nat912win/pg/pg_output_titles.htm
- Natural Programming Guide, Page Titles, Page Breaks, Blank Lines (mainframe set):
  https://documentation.softwareag.com/natural/nat912mf/pg/pg_output_titles.htm
- Natural Programming Guide, Statements DISPLAY and WRITE:
  https://documentation.softwareag.com/natural/nat912unx/pg/pg_output_display.htm
- Natural Programming Guide, User-Defined Constants:
  https://documentation.softwareag.com/natural/nat841unx/pg/pg_defi_udc.htm
- Natural Programming Guide, Text Notation:
  https://documentation.softwareag.com/natural/nat841unx/pg/pg_furth_txtnot.htm
- Natural Programming Guide, Natural and Database Access:
  https://documentation.softwareag.com/one/9.3.2/en/webhelp/one-webhelp/natmf/pg/pg_dbms_dbgen.htm
- Natural Statements, WRITE:
  https://documentation.softwareag.com/natural/nat912win/sm/write.htm
- Natural Statements, WRITE (mainframe set, with examples):
  https://documentation.softwareag.com/natural/nat828mf/sm/write.htm
- Natural Parameter Reference, TQMARK:
  https://documentation.softwareag.com/natural/nat912unx/parms/tqmark.htm
- Natural System Error Messages 0001-0049:
  https://documentation.softwareag.com/natural/nat828mf/mc_mf/mcERRN_0001.htm
- Natural System Error Messages 0900-0949:
  https://documentation.softwareag.com/naturalONE/natONE912/natmf/mc_mf/mcERRN_0900.htm
- Release Information for Natural Version 9.3.1 (Legacy-Unix EOM):
  https://documentation.softwareag.com/natux/9.3.1/en/webhelp/natux-webhelp/relnotes/rn-931.htm
- Release Information for Natural Version 9.3.2:
  https://documentation.softwareag.com/natux/9.3.2/en/webhelp/natux-webhelp/relnotes/rn-932.htm
- Release Information for Natural for Windows Version 9.3.2:
  https://documentation.softwareag.com/natwin/9.3.2/en/webhelp/natwin-webhelp/relnotes/rn-932.htm
- Entire Operations 5.5.3 Release Notes (BS2000 and z/VSE support drop):
  https://documentation.softwareag.com/nop/5.5.3/en/webhelp/nop-webhelp/rnotes/rnotes.htm
- Natural Installation for BS2000 Version 9.1.2, October 2023:
  https://documentation.softwareag.com/natural/nat912mf/pdf/inst_bs2.pdf
- Predict Application Audit User Guide (4GL usage):
  https://documentation.softwareag.com/natural/pac261/ugpaa/paaover.htm
- Natural for Linux and Cloud product page:
  https://www.softwareag.com/en/resources/adabas-natural/natural-for-linux-and-cloud/
- IBM Supported Platforms for Adabas and Natural:
  https://www.softwareag.com/en/resources/adabas-natural/ibm-platforms-products/
- Adabas and Natural on IBM Z mainframe:
  https://www.softwareag.com/en/adabas-natural/ibm-z-mainframe/
- Future-Proof Your Mission-Critical Apps with Adabas and Natural 2050+:
  https://www.softwareag.com/en/resources/adabas-natural/mission-critical-applications/
- Large US State Comptroller Office customer story:
  https://www.softwareag.com/en_corporate/resources/adabas-natural/cs/customers-customer-stories-office-of-the-comptroller.html
- Natural Community Edition container image:
  https://hub.docker.com/r/softwareag/natural-ce
- Software GmbH standalone-business announcement (blog):
  https://www.softwareag.com/en/blog/insights/adabas-natural-and-aris-launch-as-standalone/
- Software GmbH standalone-business announcement (press release, 7 January 2025):
  https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html

Secondary sources, used only for company history, which Software AG does not publish:

- Computerwoche Hall of Fame, Dr. Peter Pag&eacute;:
  https://www.computerwoche.de/hall-of-fame/dr-peter-page,27
- German Wikipedia, Natural (Programmiersprache):
  https://de.wikipedia.org/wiki/Natural_(Programmiersprache)
- English Wikipedia, Peter Pag&eacute;:
  https://en.wikipedia.org/wiki/Peter_Pag%C3%A9
- English Wikipedia, Software AG (source of the uncited 1979 claim):
  https://en.wikipedia.org/wiki/Software_AG
