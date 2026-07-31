# Verification: v07 Output Formatting Semantics

Adversarial re-verification of `research/07-output-formatting-semantics.md`.

Verification date: **2026-08-01**. All sources accessed 2026-08-01.

Documentation baseline: Natural for Windows 9.3.3 webhelp, copyright line
"Copyright © 1992-2026 Software GmbH", root
`https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/`.

## Method actually used

Every page was fetched as raw HTML with `curl` and parsed locally. `<pre>` blocks were
extracted with tags stripped and entities unescaped, then every column position was
computed programmatically (start and end column of each non-blank run). No PDF text layer
was consulted. No markdown-converting fetcher was used, because both collapse runs of
blanks.

One method correction applies to the original spike and is recorded under "Corrections
required": **`<pre>` blocks in this documentation are not universally safe for absolute
column measurement.** At least one output block is rendered with its leading blanks
removed. Absolute columns are trustworthy only when the block's internal geometry
self-checks (total line length equals the sum of the computed field widths and gaps, or at
least one element is known to start in column 1).

---

## VERDICT TABLE

| Claim | Verdict | Evidence, counted from an official example | Source URL | Accessed |
|---|---|---|---|---|
| **1. `SG` exists** | **CONFIRMED** | Dedicated parameter page. "This session parameter determines whether or not a sign position is to be allocated for a numeric field." Applicable statements listed: DISPLAY, FORMAT, INPUT, PRINT, WRITE. | [parms/sp_sg.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/parms/sp_sg.htm) | 2026-08-01 |
| **1. `SG` default is ON** | **CONFIRMED** | Parameter page: "Default setting: ON". Programming Guide restates: "By default, `SG=ON` applies, which means that a sign position is allocated for numeric fields." | [parms/sp_sg.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/parms/sp_sg.htm), [pg/pg_output_parms.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_output_parms.htm) | 2026-08-01 |
| **1. One extra HIGH-ORDER position reserved** | **CONFIRMED verbatim** | `sm/display.htm`, "Defaults Applicable for a DISPLAY Statement": "**Sign** One extra high-order print position is reserved for a sign when printing a numeric field. The session parameter `SG` may be used to suppress the sign position." | [sm/display.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/display.htm) | 2026-08-01 |
| **1. Reserved UNCONDITIONALLY, including for positive values** | **CONFIRMED, by prose and by 14 independent measurements** | Prose, `FORMAX04`: "The width of the `SALARY` and `BONUS` columns is 8 characters - 6 for the field value (`NL=6`), plus 1 leading/inserted character, plus 1 sign position (because `SG=ON` applies)." Measured underline on that example: `-----------`(11) `-----------`(11) `--------`(8) `--------`(8), matching 10+1, 10+1, 6+1+1, 6+1+1. Positive-value measurements: `CPTEX1` `CUMULATIVE SALARY:` is 18 chars in cols 1-18, gap col 19, `#CUM-SALARY (P10)` occupies cols 20-30 (11 positions) with `66300` right-justified at 26-30 and total line length exactly 30. `ASGEX1S` `#C: (N0.3)` = `.450` at cols 6-9 inside a 5-wide field starting col 5, so col 5 is a blank sign position on a positive value. | [pg/pg_output_parms.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_output_parms.htm), [sm/compute.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/compute.htm) | 2026-08-01 |
| **1. The sign position is LEADING (leftmost), not trailing** | **CONFIRMED** | `ASGEX1S` `#D (N0.5) = -0.12345` renders as `-.12345` at cols 5-11. The minus occupies col 5, ahead of the decimal point. `FORMAX04` shows `LC=>` at col 36 and the sign position at col 37, so the sign sits inside the field, left of the digits. | [sm/compute.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/compute.htm) | 2026-08-01 |
| **1. Rule applies to `WRITE`, not just `DISPLAY`** | **CONFIRMED by measurement; NOT stated in prose for WRITE** | `SG` is in the `WRITE` parameter list (`SE`). `WRITEX01` (`WRITE NAME FIRST-NAME SALARY (1:3)`): `NAME` A20 cols 1-20, gap 21, `FIRST-NAME` A20 cols 22-41, gap 42, `SALARY(1)` P9 cols 43-52 with `46000` at 48-52, gap 53, `SALARY(2)` cols 54-63 with `42300` at 59-63, gap 64, `SALARY(3)` cols 65-74 with `39300` at 70-74. Total line length exactly 74. Every P9 occupies 10, not 9. | [sm/write.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/write.htm), [pg/pg_output_display.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_output_display.htm) | 2026-08-01 |
| **1. Hunt for a contradicting example** | **NONE FOUND** | Every hyphen-underline row in every `system-output` block across 535 downloaded pages was extracted and its column widths computed. Every numeric column equals `digits + 1` unless the header is wider, and every column narrower than that is alphanumeric (the recurring 9-wide column is `PERSONNEL-ID`, an `A8` under the 9-character DDM header `PERSONNEL`). New confirming case: `REPEAX01` declares `1 #PAY1 (N8)` under the 5-character header `#PAY1`, and the column measures 9. | [pg/pg_furth_loop.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_furth_loop.htm) | 2026-08-01 |
| **2. `N7.2` print width is 11** | **CONFIRMED (was DERIVED)** | `REIEX3` declares `1 #B (N7.2)`. Output line: `#B` label at cols 25-26, gap 27, field cols 28-38 (11 positions), `0.00` at 35-38. Total line length 55 with `#D (N3)` closing at col 55. | [sm/reinput.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/reinput.htm) | 2026-08-01 |
| **2. `N7.2` holding 19.99 emits 6 blanks then `19.99`** | **DERIVED, high confidence. Not shown verbatim.** | Follows arithmetically from the verified 11-position width, the verified right-justification rule, and the verified leading-zero suppression: 1 sign + 5 suppressed integer positions = 6 blanks, then `19.99`. The value 19.99 appears nowhere in the documentation. | derived | 2026-08-01 |
| **2. `N7.2` holding zero emits 7 blanks then `0.00`** | **CONFIRMED** | Same `REIEX3` line. Field cols 28-38, `0.00` at 35-38, so cols 28-34 are 7 blanks. Caveat unchanged from the spike: this is an `INPUT`/`REINPUT` screen rendering, not a `WRITE`. | [sm/reinput.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/reinput.htm) | 2026-08-01 |
| **2. `I4` print width is 11** | **CONFIRMED (was DERIVED). New evidence.** | `RSTEX1` declares `1 #INTEGER (I4) INIT <5>` and writes `'=' #INTEGER`. Measured: heading `#INTEGER:` at cols 46-54, gap 55, field cols 56-66 (11 positions), `5` at col 66. Same line, `#BINARY (B4)` occupies exactly 8 positions (cols 37-44) with no sign position, confirming the `+1` is numeric-only. | [sm/reset.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/reset.htm) | 2026-08-01 |
| **2. `I2` print width is 6** | **UNVERIFIED, high-confidence derivation** | Still no worked example printing an unmasked `I2`. Every `I2` in the corpus is either an index variable that is never printed or is printed with an explicit `EM=9`. The digit table is verbatim (`I1`=3, `I2`=5, `I4`=10) and both bracketing cases are now measured: `I1` = 4 (`EMLOGV`, field cols 19-22) and `I4` = 11 (`RSTEX1`). | [pg/pg_furth_arithm.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_furth_arithm.htm) | 2026-08-01 |
| **2. `P7.2` prints identically to `N7.2`** | **CONFIRMED verbatim, plus measurement** | "When a user-defined variable of format P is output with a `DISPLAY`, `WRITE`, or `INPUT` statement, Natural internally converts the format to N for the output." Measured widths agree exactly with the N rule: `P4` = 5 (`CPTEX1`, cols 45-49), `P9` = 10 (`WRITEX01`, `FORMAX03`), `P10` = 11 (`CPTEX1`, cols 20-30). | [pg/pg_defi_dv.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_defi_dv.htm) | 2026-08-01 |
| **3. `A(n)` occupies exactly n positions, left-justified, remainder emitted as real blanks (MID-LINE)** | **CONFIRMED** | `RSTEX1`: `NAME` is `A20` holding `ADAM` at cols 7-10, and the next element `#BINARY:` starts at col 28, which requires cols 11-26 to be emitted blanks plus a gap at 27. `ASGEX1S`: `#H (A3/1:3)` with an empty second occurrence renders `UVW` at 5-7 and `XYZ` at 13-15, so the empty occurrence is emitted as three real blanks at 9-11. `WRTEX1`: `NAME` A20 holding `ABELLAN` at 7-13, next element at col 28. | [sm/reset.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/reset.htm), [sm/compute.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/compute.htm), [sm/write.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/write.htm) | 2026-08-01 |
| **3. Trailing blanks "not stripped, even at end of line"** | **UNVERIFIED, and the documentation mostly shows the opposite** | The docs are inconsistent. Lines that END on an alphanumeric field are usually truncated at the last non-blank character: `DISPLX01` `30020013  GARRET               TYPIST` is 37 characters when the full geometry is 56; `FORMAX08` `JONES                MARSHA` is 27 when the geometry is 41; `WRTEX5` line 2 is 72 where line 1 is 73. Against that, `FORMAX05` rows 1 and 2 are exactly 43 characters, the full geometry including the trailing pad, while its row 3 is 42. This is presentation noise in the documentation, not evidence about the runtime. Nothing in the corpus settles it. | [pg/pg_output_display.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_output_display.htm), [pg/pg_output_parms.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_output_parms.htm) | 2026-08-01 |
| **4. Explicit doc statement of the `WRITE` inter-element gap** | **DOES NOT EXIST. Spike's admission confirmed.** | Every `sm/`, `pg/`, `parms/`, and `firststeps/` page (535 files) was converted to text and searched for any sentence combining a separation word with an element/field/operand word. The only hits are the `DISPLAY` spacing-factor prose, the `WRITE` statement-parameter syntax note ("If more than one parameter is specified, they must be separated by one or more blanks"), and a `DISPLAY` walkthrough in First Steps. There is no prose statement of the `WRITE` gap. | corpus search | 2026-08-01 |
| **4. The `WRITE` gap is exactly one blank** | **CONFIRMED by measurement, and the decisive case is cleaner than the spike states** | `WRTEX1` prints `'CITY:   ' CITY` and `'COUNTRY:' COUNTRY` in the same program. Measured: `CITY:` at cols 1-5 and `MADRID` at cols 10-15, versus `COUNTRY:` at cols 1-8 and `E` at col 10. Both literals are 8 characters, both fields begin at col 10, so the gap is exactly 1 in both. Corroborated by `WRITEX01` (gaps at cols 21, 42, 53, 64), `CPTEX1` (`'CUMULATIVE SALARY:'` ends col 18, field starts col 20), `ASGEX1S`, and `RSTEX1` (gaps at cols 27, 45, 55, 67). | [sm/write.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/write.htm) | 2026-08-01 |
| **4. `SF` does not apply to `WRITE`** | **CONFIRMED, three ways** | `parms/sf.htm` defines SF as "the default number of spaces to be inserted between field settings of columns on Natural reports created using a `DISPLAY` statement", and lists "Applicable statements: SET GLOBALS" only. `sm/write.htm`'s "List of Parameters" enumerates AD, AL, CD, CV, DF, DL, DY, EM, EMU, FL, IS, LS, MC, MP, NL, PC, PM, PS, SG, UC, ZP. SF is absent. `pg/pg_output_display.htm` describes SF only under DISPLAY. | [parms/sf.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/parms/sf.htm), [sm/write.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/write.htm) | 2026-08-01 |
| **4. `nX` REPLACES the default gap** | **CONFIRMED verbatim, plus measurement in both statements** | Verbatim: "An `nX` notation overrides the specification made with the `SF` parameter." Measured for DISPLAY, `DISPLX04` (`FORMAT SF=3`, `DISPLAY PERSONNEL-ID NAME 5X JOB-TITLE`): underline segments 9, gap 3, 20, gap 5, 25, total 62. The `5X` produced 5, not 8. Measured for WRITE, `CPTEX1`: a 35-character literal followed by `5X` puts the next element at col 41 (35+5+1), and a 30-character literal followed by `10X` also puts it at col 41. Additive semantics would give 42. Second WRITE case, `CPTEX1` again: `'CURRENT SALARY: '` is 16 characters, `4X`, then `SALARY(1)` P9 occupying cols 21-30. Additive would start the field at col 22. | [pg/pg_output_display.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_output_display.htm), [sm/compute.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/compute.htm) | 2026-08-01 |
| **5. Default numeric edit mask is `Z9`** | **CONFIRMED verbatim as the string "Z9". The expansion is an interpretation, and implementing the literal string is a defect.** | "Default Edit Masks" table: `A`->`X`, `B`->`H`, `N, P, I`->`Z9`, `F`->scientific representation, `D`->depends on DTFORM, `T`->`HH:II:SS`, `L`->`blank / X`. The spike's gloss ("leading zeros suppressed, the units digit forced, decimal positions always forced") is consistent with every measurement but is not doc wording. See Correction 1: a literal two-character `EM=Z9` applied to an `N7.2` field would truncate it to two integer digits and drop the decimals, which is provably not what the default does. | [parms/sp_em.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/parms/sp_em.htm) | 2026-08-01 |
| **5. `Z` cannot appear right of the decimal separator** | **CONFIRMED verbatim** | "`Z` must not be specified to the right of the decimal separator character. A zero value may be displayed as blanks using all `Z`s in the edit mask (see also session parameter `ZP`)." | [parms/sp_em.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/parms/sp_em.htm) | 2026-08-01 |
| **5. Trailing decimals are forced** | **CONFIRMED by measurement** | `ASGEX1S` `#C (N0.3)` assigned `.45` renders `.450`. `CPTEX1` `#B (N3.4)` holding `1.22` renders `1.2200` at cols 48-53 inside the 9-position field at cols 45-53. | [sm/compute.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/compute.htm) | 2026-08-01 |
| **6. Decimal truncation on assignment is silent and the default** | **CONFIRMED verbatim, plus measurement** | "High-order numeric field truncation is allowed only when the digits to be truncated are leading zeros. Digits following an expressed or implied decimal point may be truncated." Measured: `ASGEX1S` assigns `-0.12345` to `#E (N1.3)` and the output is `-0.123`, no error. | [pg/pg_furth_arithm.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_furth_arithm.htm), [sm/compute.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/compute.htm) | 2026-08-01 |
| **6. `ROUNDED` is opt-in** | **CONFIRMED verbatim, plus measurement** | "If the option `ROUNDED` is specified, the last position of the result will be rounded up if the first truncated decimal position of the value being assigned contains a value greater than or equal to 5." Measured: `ASSIGN ROUNDED #F = 199.999` into `#F (N5)` yields `200` at cols 8-10 of a field at cols 5-10. | [pg/pg_furth_arithm.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_furth_arithm.htm) | 2026-08-01 |
| **6. High-order truncation of a non-zero digit is an error, not silent** | **CONFIRMED, and the error number is now known** | The rule ("allowed only when the digits to be truncated are leading zeros") is verbatim. The runtime error is `NAT1305: Numeric value truncated in MOVE/ASSIGN operation.` Explanation: "The receiving field in a MOVE/ASSIGN operation is not long enough to hold the result of the value." | [pg/pg_furth_arithm.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_furth_arithm.htm), [mc/mcERRN_1300.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/mc/mcERRN_1300.htm) | 2026-08-01 |
| **7a. Bare `WRITE` with no operands** | **RESOLVED as a syntax error, with a caveat** | The "Syntax Symbols" page is live at `sm/synsym.htm` (HTTP 200; the spike's 404 claim is wrong). It defines `{ }` as "you must choose exactly one of the alternatives", `[ ]` as optional, and `...` as "a term preceding an ellipsis may optionally be repeated", which means the term itself must appear at least once. Decoding the image sequence in `sm/write.htm`'s Syntax 1 diagram (`cbo5b`, `sbo5 ... sbc5`, `dot3`, `cbo3 ... cbc3`, `cbc5b`, `dot3`) gives an outer braced group, repeatable, containing an optional repeatable positioning group and a mandatory 3-way alternation of `'text'` / `'c'(n)` / `['='] operand1`. So at least one text or operand element is required. Caveat: no worked example and no error number was found. The two `WRITE /` lines in the corpus (`ATBREX06`) are statement continuations, not bare `WRITE`s. | [sm/synsym.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/synsym.htm), [sm/write.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/write.htm) | 2026-08-01 |
| **7b. Logical field with no edit mask** | **RESOLVED. New verbatim source found.** | `sm/compress.htm`, operand1 note: "Using operand1 without an explicit Edit Mask, a ... - Logical variable (format L) with value \<false\> is represented by a blank and value \<true\> is represented by char "X"." This is prose, not a derivation, and it agrees exactly with the Default Edit Masks table entry `L`->`blank / X`. Format L is 1 byte, so the print width is 1. Residual caveat: the compress.htm sentence describes conversion to alphanumeric representation, and there is still no worked example of a `WRITE` on an unmasked logical. | [sm/compress.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/compress.htm), [parms/sp_em.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/parms/sp_em.htm) | 2026-08-01 |
| **7c. Insertion character inside a zero-suppressed region** | **RESOLVED. It is NOT printed.** | `sm/input.htm` documents `RESET N (N7,3)` with `INPUT N (AD=M EM=Z'.'ZZZ'.'ZZZ,999EUR)` under `DC=,`, and gives a value table. Value `0` displays as `,000EUR`. Value `1` displays as `1,000EUR`. Value `1,234` (that is, 1.234) displays as `1,234EUR`. Value `1234` displays as `1.234,000EUR`. Value `1234567` displays as `1.234.567,000EUR`. In the `1` and `1,234` cases both literal `.` insertion characters lie left of the single printed digit and neither appears. In the `1234` case the first `.` (between digit 1 and digit 2, all suppressed) is absent while the second `.` (immediately right of the first printed digit) is present. Rule: an insertion character is emitted only once a significant digit has been printed to its left. | [sm/input.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/sm/input.htm) | 2026-08-01 |
| **7c. Corollary: a suppressed `Z` position with no fill character prints a blank** | **CONFIRMED** | `EDITMX04`, `BONUS (1,1) (EM=SZ99,999+)` holding 4000 renders `+ 04,000+`. The `S` sign gives `+`, the `Z` position holding zero gives a blank, the two forced `9`s give `04`, then the comma, `000`, and the trailing `+`. With a fill character the same value under `EM=S*ZZZ,999` renders `+**4,000`. | [pg/pg_exas.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pg/pg_exas.htm) | 2026-08-01 |
| **7d. Runtime error identifiers for overflow** | **RESOLVED** | `NAT1305: Numeric value truncated in MOVE/ASSIGN operation.` `NAT1304: Value has been rounded and does not fit into field.` ("Rounding has caused the value to exceed the number of digits defined for the field.") `NAT1301: Intermediate result too large.` ("The construct of the arithmetic expression generates an intermediate result with too many digits.") `NAT1302: Division by zero not permitted by parameter ZD=ON.` `NAT1142: Input results in integer value overflow.` For the `DISPLAY` line-overflow case in F2: `NAT0271: Page width (line size) exceeded in DISPLAY statement.` Related: `NAT0302: Element in WRITE/INPUT statement does not fit on 1 line.` and `NAT0412: "nX" or "nT" notation positions beyond line size.` | [mc/mcERRN_1300.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/mc/mcERRN_1300.htm), [mc/mcERRN_0250.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/mc/mcERRN_0250.htm), [mc/mcERRN_0300.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/mc/mcERRN_0300.htm), [mc/mcERRN_0400.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/mc/mcERRN_0400.htm), [mc/mcERRN_1100.htm](https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/mc/mcERRN_1100.htm) | 2026-08-01 |

### Bonus findings that change or add test rows

| # | Finding | Verdict | Evidence |
|---|---|---|---|
| X1 | `B4` prints in 8 positions with **no** sign position | **CONFIRMED** | `RSTEX1`: `#BINARY (B4) INIT <1>` renders `00000001` at cols 37-44 exactly. Default mask `H`, two print positions per byte, and `ZP`'s note scopes zero-printing to formats N, I, P, F and T only. |
| X2 | `N2` prints in 3 positions | **CONFIRMED** | `RSTEX1`: `#NUMERIC (N2)` wraps to the next line as ` 25` (value at cols 2-3) and `  0` (value at col 3). |
| X3 | `N8` prints in 9 positions | **CONFIRMED** | `REPEAX01`: `1 #PAY1 (N8)` produces a 9-hyphen column under the 5-character header `#PAY1`. |
| X4 | After a `WRITE` line advance or wrap, the continued element starts at **column 1** with no leading separator | **CONFIRMED** | `RSTEX1`: the line reaches col 76 with `#NUMERIC:`, then the 3-position field is emitted on the next line starting at col 1 (`25` at cols 2-3). |
| X5 | An **explicit** edit mask removes the automatic sign position and the mask alone determines the width | **CONFIRMED by measurement** | `EDITMX04`: `SALARY (1)` is `P9` (normally a 10-wide column) but with `(EM=*USD^ZZZ,999)` its column measures 11, which is exactly the mask width (`USD` + blank + 3 + comma + 3 = 11). No twelfth column for a sign. Value 28000 renders `USD *28,000`. |
| X6 | If a mask has MORE digit positions than the field has digits, the mask is shortened to the field | **CONFIRMED by measurement** | `EDITMX04`: `LEAVE-DUE` is `N2` in the DDM and `(EM=+999)` renders `+13` in a 3-wide column, not `+013` in 4. Matches the verbatim rule "the number of print positions in the edit mask will be adjusted to the number of digits defined for the field value". |
| X7 | An unmasked `D` field prints in 8 positions | **CONFIRMED** | `DF` default is `S`, the 8-byte `yy-mm-dd` form. Consistent with the 8-wide columns observed, though every 8-wide column measured in the corpus turned out to be alphanumeric under a wider header, so this rests on the parameter page rather than measurement. Recommendation to always mask dates stands. |

---

## Corrections required

### 1. Do not implement the default numeric mask as the literal two-character string `Z9`

This is the most important implementation correction in this document. The Default Edit
Masks table says `Z9` and that is verbatim correct, but the same page also says:

> If fewer `9`s or `Z`s exist, the high-order digits before the decimal separator and/or
> low-order digits after the decimal separator will be truncated.

and the documented examples table confirms it: `EM=99` applied to `0962` (an `N4`) yields
`62`, silently discarding the high-order digit. A literal `EM=Z9` applied to an `N7.2`
field would therefore render `19.99` as `9`, not as `      19.99`. Every measurement in
this document shows the default does the opposite.

Encode the default as a **rule** sized to the field: suppress leading zeros across the
field's own integer positions, force the units digit, force every decimal position. That
is what `Z(i-1)9.9(d)` would produce. Never build the default by expanding a stored two
character mask string.

### 2. The default edit mask does not count as "an edit mask" for the SG override

`parms/sp_em.htm` states "An edit mask overrides any settings for the session parameters
`AL`, `NL` and `SG`." Read naively, and combined with "if no edit mask is specified, a
default edit mask is assigned", this would abolish the sign position everywhere. It does
not. The override applies only to an **explicitly specified** mask. Measured both ways:
without an explicit mask every numeric field measures `digits + 1` (14 cases); with an
explicit mask the width equals the mask (`EDITMX04`, `P9` under `EM=*USD^ZZZ,999` giving
an 11-wide column). The spike's row D39 is correct but its wording ("mask present") should
be tightened to "explicit mask present".

### 3. The `(P7.2) /* ... and 1 sign position` comment is about packed STORAGE, not print width

The spike quotes this annotated example immediately before deriving the print width, which
implies the comment supports the print-width claim. It does not. The same block reads:

```
1 #A3 (P4)        /* Packed numeric, 4 positions and 1 sign position.
1 #A4 (N7.2)      /* Unpacked numeric,
                  /* 7 positions before and 2 after decimal point.
1 #A6 (P7.2)      /* Packed numeric, 7 positions before and 2 after decimal point
                  /* and 1 sign position.
```

The "1 sign position" note appears only on the `P` declarations and never on the `N` one.
It describes the packed-decimal sign nibble in storage. If it were about printing it would
appear on `N7.2` too, and `N` fields would then lack a print sign position, which
measurement disproves. Remove this quote from section 1.3 or relabel it as a storage note.
The print-width claim does not need it: `pg/pg_output_parms.htm`'s `FORMAX04` sentence
("plus 1 sign position (because `SG=ON` applies)") is the correct prose citation, and the
measurements carry the rest.

### 4. Retract the claim that trailing blanks survive to end of line

Section 1.2 and test row A1 assert that an `A20` holding `Hello` emits
`Hello` plus 15 trailing blanks as a complete output line. Mid-line padding is proven many
times over and is safe. End-of-line padding is not documented, and the majority of doc
examples show the line truncated at the last non-blank character. Because expected-output
fixtures compare strings exactly, this must be a stated project convention rather than an
inherited claim. Recommendation: right-strip every expected-output line in fixtures, pad
only between elements, and write the convention into the interpreter's output
documentation.

### 5. Correct the `<pre>`-blocks-are-always-safe assumption

The spike's method note says HTML `<pre>` blocks are whitespace-exact and trustworthy.
That is true of most of them but not all. `WRTEX2` in `sm/write.htm` runs
`WRITE NOTITLE 5X NAME 50T JOB-TITLE` and its output block, verified in the raw HTML
source, begins `ABELLAN` at column 1 and places `MAQUINISTA` at columns 45-54. Both are
exactly 5 columns left of where the program's own `5X` and `50T` put them. The block was
saved with its leading blanks removed. The internal geometry is self-consistent, so the
error is a uniform shift, but any fixture built from that block's absolute columns would
be wrong.

Rule to adopt: accept an absolute column measurement from a `<pre>` block only when the
total line length equals the computed geometry, or when an element known to start in
column 1 anchors the block.

### 6. Two official pages publish contradictory output for the same program

`EMLOGV` in `parms/sp_em.htm` and `LOGICX05` in `pg/pg_furth_lcc.htm` are the identical
program. Their documented outputs differ by one column (22 versus 21 characters per line).

The `sp_em.htm` version is correct and self-consistent: `FALSE` occupies cols 1-5 (the
`EM=FALSE/TRUE` mask is sized by the longer string), `5X` gives cols 6-10, the literal
`INDEX =` occupies 11-17, the separator is col 18, and `#INDEX (I1)` occupies 19-22.

The `pg_furth_lcc.htm` version places `INDEX` at col 10 on both the `TRUE` and the `FALSE`
line, which is impossible under any consistent rule: it would require the mask field to be
4 wide on one line and 5 wide on the next while the `5X` stayed constant. That block has
lost one leading column.

Use `parms/sp_em.htm`. Test rows D33 and D34 are correct as written.

### 7. `sm/synsym.htm` is live; the spike's 404 claim is wrong

The spike states the "Syntax Symbols" page returns 404 at every URL tried in the 9.3.3
tree, and treats the bracket convention as unconfirmed. The page is at
`sm/synsym.htm#Syntax_Symbols`, linked from `sm/write.htm` itself, and returns HTTP 200.
Its definitions of `[ ]`, `{ }`, and `...` are what allow item 7a above to be resolved.
Update the spike's open-question 1 accordingly.

### 8. `SF` has a three-way documentation inconsistency, wider than the spike recorded

The spike noted `sm/display.htm` versus `pg/pg_output_display.htm`. There is a third
position: `parms/sf.htm` lists "Applicable statements: **SET GLOBALS**" only, naming
neither `DISPLAY` nor `WRITE`. So the three sources say, respectively, statement and
element level on DISPLAY, statement level only on DISPLAY, and SET GLOBALS only. None of
them says `WRITE`, which is all Tier 1 needs. Leave `SF` out of v1.

### 9. Test-table status upgrades

| Row | Old status | New status | Reason |
|---|---|---|---|
| A11 (`N7.2` width 11) | DERIVED | **VERIFIED** for the width | `REIEX3` measured, field cols 28-38 |
| A11 (`N7.2` = 19.99 exact string) | DERIVED | **DERIVED, unchanged** | the value 19.99 is not in the docs |
| A21 (`I4` width 11) | DERIVED | **VERIFIED** | `RSTEX1` measured, field cols 56-66 |
| A20 (`I2` width 6) | DERIVED | **DERIVED, unchanged** | no example found; now bracketed by measured `I1`=4 and `I4`=11 |
| A22, A23 (logical `X` / blank) | DERIVED | **VERIFIED by prose** | `sm/compress.htm` verbatim |
| D25 (`EM=ZZZ,ZZ9.99` suppressed comma) | UNVERIFIED | **RESOLVED**: the comma is not emitted while the region to its left is fully suppressed | `sm/input.htm` value table |
| E5, E6 (error numbers) | UNVERIFIED | **RESOLVED**: NAT1305, NAT1304, NAT1301, NAT1302 | Messages and Codes |
| F2 (`DISPLAY` overflow message) | UNVERIFIED | **RESOLVED**: NAT0271 | Messages and Codes |
| F6 (bare `WRITE`) | UNVERIFIED | **RESOLVED**: syntax error, at least one text or operand element is required | `sm/synsym.htm` plus the `sm/write.htm` diagram |
| New: X1 to X7 | n/a | add as test rows | see the bonus table above |

---

## What survived without a scratch

The sign-position rule and the width formula are the two things this exercise was built to
break, and neither moved. The formula

```
print width = 1 (sign) + integer digits + (1 if decimals) + decimal digits
```

was checked against `N0.3`, `N0.5`, `N1.3`, `N2`, `N3`, `N3.4`, `N5`, `N7.2`, `N8`, `P4`,
`P9`, `P10`, `I1`, and `I4`, in both `WRITE` and `DISPLAY`, in eleven separate example
programs across seven documentation pages, and it fitted every one to the character. The
underline row of every `DISPLAY` example in the entire 535-page corpus was extracted
mechanically and searched for a numeric column narrower than `digits + 1`. There is none.

The one-blank inter-element gap for `WRITE` remains undocumented in prose and remains
correct in measurement, now across seven examples rather than five.

---

## Sources

All accessed 2026-08-01. Root:
`https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/`

| Path | What it substantiated here |
|---|---|
| `sm/write.htm` | WRITE usage and the three differences from DISPLAY; the full session-parameter list (SF absent, SG present as SE); the Syntax 1 diagram image sequence; `nX`/`nT` descriptions; examples WRTEX1 through WRTEX5 with exact spacing |
| `sm/display.htm` | "One extra high-order print position is reserved for a sign"; spacing factor default; field output and justification; report width; terminal column 2 versus paper column 1; DISPLAY parameter list (SF marked SE) |
| `sm/compute.htm` | ASGEX1S and CPTEX1 measured: N and P print widths, leading sign position, forced decimals, silent decimal truncation, ASSIGN ROUNDED, nX replacing the default gap in WRITE |
| `sm/reset.htm` | **RSTEX1, the decisive new measurement**: I4 = 11 positions, B4 = 8 positions with no sign, N2 = 3 positions, A20 padded mid-line, WRITE wrap starting at column 1 |
| `sm/reinput.htm` | REIEX3 measured: N7.2 = 11 positions with 7 blanks then `0.00`; N3 = 4 positions |
| `sm/compress.htm` | **Verbatim resolution of the unmasked logical field**: without an explicit edit mask, format L false is a blank and true is `X`. Also the COMPRESS default one-blank separation and CMPEX4 |
| `sm/input.htm` | **Verbatim resolution of the suppressed insertion character**: the `EM=Z'.'ZZZ'.'ZZZ,999EUR` value table |
| `sm/synsym.htm` | Syntax Symbols: `[ ]`, `{ }`, `...` conventions. Live at HTTP 200, contradicting the spike |
| `pg/pg_output_display.htm` | "An `nX` notation overrides the specification made with the `SF` parameter"; DISPLX01 to DISPLX06, WRITEX01, WRITEX02 measured |
| `pg/pg_output_parms.htm` | "plus 1 sign position (because `SG=ON` applies)"; SG applicable statements including WRITE; FORMAX03 to FORMAX08 measured |
| `pg/pg_output_masks.htm` | EM specification levels; the `EM=ZZ,ZZZ,ZZ9.99` separator table; the examples-of-edit-masks table |
| `pg/pg_exas.htm` | **EDITMX04**, the proof that an explicit mask replaces the sign position and defines the column width, and that a mask longer than the field is shortened to the field |
| `pg/pg_furth_arithm.htm` | "Field Truncation and Field Rounding" verbatim; the I1/I2/I4 decimal integer length table (3, 5, 10); arithmetic error conditions |
| `pg/pg_furth_loop.htm` | REPEAX01: `#PAY1 (N8)` producing a 9-wide column |
| `pg/pg_furth_compu.htm` | COMPRX04 with `EM=ZZZ,ZZ9` |
| `pg/pg_furth_lcc.htm` | LOGICX05, the defective twin of EMLOGV. Recorded so the discrepancy is not rediscovered |
| `pg/pg_defi_dv.htm` | "Natural internally converts the format to N for the output"; the annotated format/length example whose "1 sign position" comment is about packed storage |
| `parms/sp_em.htm` | Default Edit Masks table verbatim; "An edit mask overrides any settings for AL, NL and SG"; numeric mask character definitions; the numeric edit-mask results table; EMLOGV |
| `parms/sp_sg.htm` | SG semantics, default ON, applicable statements, EM override |
| `parms/sf.htm` | SF defined for DISPLAY reports only; "Applicable statements: SET GLOBALS"; default 1 |
| `parms/zp.htm` | ZP default ON; "output as one zero, right justified"; applicable to WRITE |
| `mc/mcERRN_1300.htm` | NAT1301, NAT1302, NAT1304, NAT1305 |
| `mc/mcERRN_0250.htm` | NAT0271, DISPLAY page-width overflow |
| `mc/mcERRN_0300.htm` | NAT0302, element does not fit on one line |
| `mc/mcERRN_0400.htm` | NAT0412, nX/nT beyond line size |
| `mc/mcERRN_1100.htm` | NAT1141, NAT1142, input overflow |

Corpus note: 535 pages under `sm/`, `pg/`, `parms/`, and `firststeps/` were enumerated
from `navig/contents.js` and downloaded in full, plus 25 pages of the Messages and Codes
manual. All negative results above ("no example found", "no prose statement exists") are
mechanical searches over that corpus, not impressions.
