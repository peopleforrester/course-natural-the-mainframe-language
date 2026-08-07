# Adversarial fact-check: lesson 03 and lesson 04

Audited 2026-08-04 against official Software AG Natural documentation at
documentation.softwareag.com. Every code sample was additionally executed
against the course interpreter (`cargo run -p natural-cli`) to compare
promised behavior with delivered behavior.

Result: 6 defects. None of them is a false arithmetic result. The two that
matter are in lesson 4.4 (the lesson warns about the wrong verb and omits
the only genuinely reversed one) and lesson 3.2 (a documented limit is
missing, and the interpreter does not enforce it either).

## Findings

| Lesson.Step | Claim (quoted) | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| 3.1 | "Declarations come first, before any executable statement." | CONFIRMED | The documented rule is stronger than the lesson states: "When a DEFINE DATA statement is used, it must be the first statement of the program/routine." Comment lines may precede it; the official example programs open with `** Example ...` comment lines above DEFINE DATA. | https://documentation.softwareag.com/natural/nat911mf/sm/defineda_basic.htm | 2026-08-04 |
| 3.1 | "A is alphanumeric, N is numeric, P is packed numeric, I is a binary integer, and L is logical." | MISLEADING | A, P and L are exact. The format table gives N as "Numeric (unpacked)" and I as "Integer", plain. "Binary" is a separate Natural format code, B, with definable length 1 to 1073741824. Describing I as "a binary integer" collides with B, and dropping "(unpacked)" from N removes the contrast that makes P meaningful. | https://documentation.softwareag.com/natural/nat841unx/pg/pg_defi_dv.htm | 2026-08-04 |
| 3.1 | Example uses level number `1` and names prefixed with `#` | CONFIRMED | "Level number is a 1- or 2-digit number in the range from 01 to 99 (the leading zero is optional)". Permitted first characters are "A - Z (Upper-case alphabetical character), & (Ampersand), # (Number sign), + (Plus sign)", and "If the first character is a number sign (#) ... the name must consist of at least one additional character." `1 #NAME (A20)` satisfies both. The `#` prefix is a permitted convention, not a requirement, and the lesson does not claim otherwise. | https://documentation.softwareag.com/natural/nat913win/sm/defineda_lda.htm , https://documentation.softwareag.com/natural/nat913win/using/use_rules.htm | 2026-08-04 |
| 3.2 | "(N7.2) means seven digits before the decimal point and two after, so nine digit positions in total. It does not mean seven digits altogether." | CONFIRMED | Verbatim from the Programming Guide: "For fields defined with format N or P, you can use decimal position notation in the form nn.m, where nn represents the number of positions before the decimal point, and m represents the number of positions after the decimal point." | https://documentation.softwareag.com/natural/nat841unx/pg/pg_defi_dv.htm | 2026-08-04 |
| 3.2 | "N and P allow at most 29 positions, and I accepts only lengths 1, 2, and 4." | MISLEADING (incomplete) | The I lengths are exact ("1, 2 or 4"). The 29 cap is exact but is only half the rule. Verbatim: "The sum of the values of nn and m must not exceed 29, and the value of m must not exceed 7." The lesson omits the decimal cap, so a learner reading this step concludes that (N20.9) or (N5.8) is legal. It is not. The course interpreter accepts `(N5.8)`, `(N5.15)` and `(P5.9)` without complaint (verified locally 2026-08-04), so the learner gets no correction from the runtime either. | https://documentation.softwareag.com/natural/nat841unx/pg/pg_defi_dv.htm | 2026-08-04 |
| 3.3 | "A field always occupies its full print width." | CONFIRMED | The official WRITE example WRITEX01 outputs `NAME` (A20) in columns 1 to 20 with `FIRST-NAME` starting at column 22, so the A20 field is blank-padded to its full 20 positions and elements are separated by one blank. AL and NL are documented as setting "the default output length"; absent them, the field length governs. The word "always" is correct at default settings only, since AL, NL and an edit mask (EM) each override it. | https://documentation.softwareag.com/natural/nat913win/pg/pg_output_display.htm , https://documentation.softwareag.com/natural/nat841unx/parms/sp_al.htm , https://documentation.softwareag.com/natural/nat828mf/parms/sp_nl.htm | 2026-08-04 |
| 3.3 | "A numeric field also reserves one leading position for a sign, even when the value is positive" | CONFIRMED | SG is the governing session parameter and its documented default is ON: "By default, SG=ON applies, which means that a sign position is allocated for numeric fields." SG applies to DISPLAY, FORMAT, INPUT, PRINT and WRITE. The width arithmetic is spelled out: "The width of the SALARY and BONUS columns is 8 characters - 6 for the field value (NL=6), plus 1 leading/inserted character, plus 1 sign position (because SG=ON applies)." That the position is leading rather than trailing is demonstrated in WRITEX01, where SALARY (P9) occupies 10 columns (43 to 52, 54 to 63, 65 to 74) with the digits right justified, leaving the spare position on the left. | https://documentation.softwareag.com/natural/nat828mf/parms/sp_sg.htm , https://documentation.softwareag.com/natural/nat828mf/pg/pg_output_parms.htm , https://documentation.softwareag.com/natural/nat913win/pg/pg_output_display.htm | 2026-08-04 |
| 4.1 | "MOVE x TO y and y := x are the same thing." | MISLEADING | True for the scalar copy the lesson shows, and the docs back that: "Data transfer is performed with a MOVE or COMPUTE statement", both governed by one data-transfer compatibility table, and "The difference between the two statements is that in the MOVE statement the value to be moved is specified on the left; in the COMPUTE statement the value to be assigned is specified on the right." But they are not the same statement. Only COMPUTE / `:=` accepts an arithmetic-expression operand, so `MOVE #A * #B TO #C` is not valid while `#C := #A * #B` is. MOVE additionally carries SUBSTRING, BY NAME / POSITION, EDITED, NORMALIZED and other forms with their own rules, one of which is explicitly exempted from the shared rules: "MOVE with the SUBSTRING option is a byte-by-byte move (that is, the rules described under Rules for Arithmetic Assignment in the Programming Guide do not apply)." | https://documentation.softwareag.com/natural/nat913win/pg/pg_furth_arithm.htm , https://documentation.softwareag.com/natural/nat912win/pg/pg_furth_compu.htm , https://documentation.softwareag.com/natural/nat913win/sm/move.htm | 2026-08-04 |
| 4.1 | "COMPUTE evaluates an expression." | CONFIRMED | "The COMPUTE (or ASSIGN) statement is used to perform an arithmetic or assignment operation." Connecting operators are `**`, `*`, `/`, `+`, `-` and parentheses. `#QTY := 3` is valid structured mode: "In structured mode, when the statement keyword COMPUTE (or ASSIGN) is omitted, the equal sign (=) must be preceded by a colon (:)." | https://documentation.softwareag.com/natural/nat913win/sm/compute.htm | 2026-08-04 |
| 4.1 | "Put spaces around operators." | CONFIRMED, but the strength is overstated | This is real Natural, not an interpreter invention. Verbatim: "Each operator should be preceded and followed by at least one blank so as to avoid any conflict with a variable name that contains any of the above characters." Note "should", not "must", and note the stated reason: Natural variable names may legally contain characters that also serve as operators, so `A-B` is ambiguous. The course interpreter is stricter than the wording: `COMPUTE #B = #A*2` is rejected with "Line 6: '#A*2' has not been declared. Add it to the DEFINE DATA block before you use it." (verified locally 2026-08-04). Real Natural does not treat `*` as a name character, so this input would not fail the same way on a real compiler, and the message names a parser artifact rather than the concept. | https://documentation.softwareag.com/natural/nat913win/sm/compute.htm | 2026-08-04 |
| 4.2 | "In most languages 0.1 + 0.2 is 0.30000000000000004. Here it is exactly 0.30, because Natural stores decimal digits rather than binary fractions." | CONFIRMED for the formats used, over-general as written | The Programming Guide draws exactly this contrast: business arithmetic uses "fields of format P (packed numeric)" which represent values as "a sum of powers of ten", while scientific calculation uses "fields of format F (floating point)" which represent numbers as "a sum of powers of two". So the claim is right for N and P, which is what `(N5.2)` in the sample uses. It is over-general about the language: Natural does have a binary floating point format F, and "In expressions where formats are mixed between numeric (N, P) and floating point (F), a conversion to floating point format is performed." Interpreter output for the sample is `0.30`, as promised. | https://documentation.softwareag.com/natural/nat912mf/pg/pg_furth_arithm.htm | 2026-08-04 |
| 4.3 | "Assigning a value with more decimals than the field holds truncates toward zero. Add ROUNDED when you want rounding instead." | CONFIRMED | Truncation is the default: "Digits following an expressed or implied decimal point may be truncated." Rounding is opt-in and conditional: "If the option ROUNDED is specified, the last position of the result will be rounded up if the first truncated decimal position of the value being assigned contains a value greater than or equal to 5." The sample's `COMPUTE ROUNDED #ROUND = 1.29` is syntactically valid, because "when the ROUNDED option is used, the statement keyword COMPUTE (or ASSIGN) must be specified" and the sample specifies it. Interpreter output is 1.2 and 1.3, as promised. | https://documentation.softwareag.com/natural/nat912mf/pg/pg_furth_arithm.htm , https://documentation.softwareag.com/natural/nat913win/sm/compute.htm | 2026-08-04 |
| 4.4 | `ADD 50 TO #N` adds to the field | CONFIRMED | Syntax without GIVING is `ADD operand1 TO operand2`, and "The result is stored in operand2", equivalent to "operand2 := operand2 + operand1". The operand table permits a constant (C) for operand1 and not for operand2. | https://documentation.softwareag.com/natural/nat913win/sm/add.htm | 2026-08-04 |
| 4.4 | `SUBTRACT 25 FROM #N` subtracts from the field | CONFIRMED | `SUBTRACT operand1 FROM operand2`, equivalence "operand2 := operand2 - operand1". Constant permitted for operand1 only. | https://documentation.softwareag.com/natural/nat913win/sm/subtract.htm | 2026-08-04 |
| 4.4 | "DIVIDE 4 INTO #N divides the field by four" | CONFIRMED | "operand1 is the divisor, operand2 is the dividend. The result is stored in operand2", stated as equivalent to "operand2 := operand2 / operand1". Constant permitted for operand1 only. | https://documentation.softwareag.com/natural/nat913win/sm/divide.htm | 2026-08-04 |
| 4.4 | "ADD, SUBTRACT, MULTIPLY, and DIVIDE each read and write one field. Watch the direction of DIVIDE" | MISLEADING (worst defect in these two lessons) | The lesson flags the one verb that is regular and stays silent about the one that is not. DIVIDE puts its target last and stores into operand2, exactly like ADD and SUBTRACT. MULTIPLY is the outlier: `MULTIPLY [ROUNDED] operand1 BY operand2`, and "operand1 is the multiplicand, operand2 is the multiplier. The result is stored in operand1." The operand tables make the reversal machine-checkable: for ADD, SUBTRACT and DIVIDE the Possible Structure column lists C (Constant) for operand1 and not for operand2; for MULTIPLY it lists C for operand2 and not for operand1. So the target of MULTIPLY is named first and cannot be a constant. MULTIPLY is named in the prose but is the only one of the four never demonstrated in the code sample, and a learner generalizing from the three shown verbs writes `MULTIPLY 3 BY #N`, which is invalid Natural. The course interpreter does reject it, but with "Line 5: '3' has not been declared. Add it to the DEFINE DATA block before you use it." (verified locally 2026-08-04), which teaches the wrong lesson. `MULTIPLY #N BY 3` runs and yields 300.00. | https://documentation.softwareag.com/natural/nat913win/sm/multiply.htm , https://documentation.softwareag.com/natural/nat913win/sm/divide.htm , https://documentation.softwareag.com/natural/nat913win/sm/add.htm , https://documentation.softwareag.com/natural/nat914unx/sm/synsym.htm | 2026-08-04 |
| 4.4 | Prose says "DIVIDE 4 INTO #N", the code sample on the same step runs `DIVIDE 5 INTO #N` | MISLEADING (minor) | Both are arithmetically correct. The step explains a divisor of four and then demonstrates a divisor of five, which costs the learner a re-read at the exact moment the lesson is asking them to concentrate on operand direction. | Internal inconsistency, no external source | 2026-08-04 |
| 4.5 | "A part costs 12.50 and you are buying 7 ... The answer should be 87.50" into "(N7.2)" | CONFIRMED | 12.50 multiplied by 7 is 87.50 exactly. An (N7.2) field holds 7 integer positions and 2 decimal positions, so its range reaches 9999999.99 and 87.50 fits with wide margin. Both operands and the product carry 2 decimal positions, so no truncation or rounding is in play. Solved against the course interpreter: output `Total:       87.50`. | https://documentation.softwareag.com/natural/nat841unx/pg/pg_defi_dv.htm | 2026-08-04 |
| 3.x and 4.x | Every code sample is valid Natural | CONFIRMED | All eight samples parse and run. No sample violates a documented rule: DEFINE DATA is first, every element carries a level number and a format, END-DEFINE closes the block, END closes the program, `#QTY := 3` is legal structured mode, and `COMPUTE ROUNDED #ROUND = 1.29` supplies the keyword that ROUNDED requires. Output widths produced by the interpreter match what the documented AL / NL / SG defaults predict: `(A10)` prints in 10 columns, `(N5)` in 6, `(N7.2)` in 11, `(N9.2)` in 13, with one blank between elements. | https://documentation.softwareag.com/natural/nat913win/sm/compute.htm | 2026-08-04 |

## Corrections required

Six edits. The first two are content defects that reach the learner as
false or absent facts. The last three are interpreter defects that the
lessons expose.

**1. Lesson 4.4 prose, the whole sentence pair. Highest priority.**

Replace:

> ADD, SUBTRACT, MULTIPLY, and DIVIDE each read and write one field. Watch the direction of DIVIDE: DIVIDE 4 INTO #N divides the field by four.

With:

> ADD, SUBTRACT, MULTIPLY, and DIVIDE each read and write one field. Three of them name that field last: ADD 50 TO #N, SUBTRACT 25 FROM #N, and DIVIDE 5 INTO #N all update #N, and DIVIDE 5 INTO #N divides the field by five. MULTIPLY is the exception. It names the field first: MULTIPLY #N BY 3 multiplies #N by three. Writing MULTIPLY 3 BY #N is an error, because the first operand is where the result goes and a constant cannot receive a result.

Then add MULTIPLY to the code sample so all four verbs are demonstrated.
Insert after the SUBTRACT lines and before DIVIDE:

```
MULTIPLY #N BY 2
WRITE 'After MULTIPLY:' #N
```

Recheck the printed values after inserting it, because the running total
changes: 100, then 150, then 125, then 250, then 50 after `DIVIDE 5 INTO #N`.
Either accept the new numbers or move MULTIPLY after DIVIDE to keep the
existing 150 / 125 / 25 sequence intact.

**2. Lesson 3.2 prose, the limits sentence.**

Replace:

> Limits worth knowing: N and P allow at most 29 positions, and I accepts only lengths 1, 2, and 4.

With:

> Limits worth knowing: for N and P the digits before and after the decimal point must add up to 29 or fewer, and no more than 7 of them may sit after the decimal point. So (N22.7) is legal and (N5.8) is not. I accepts only lengths 1, 2, and 4.

**3. Lesson 3.1 prose, the format letters.**

Replace:

> A is alphanumeric, N is numeric, P is packed numeric, I is a binary integer, and L is logical.

With:

> A is alphanumeric, N is unpacked numeric, P is packed numeric, I is an integer, and L is logical.

Reason: Natural has a separate format code B for binary, so "binary
integer" reads as though I and B are the same thing, and "unpacked" is
what makes the N versus P distinction land.

**4. Lesson 4.1 prose, the MOVE equivalence.**

Replace:

> MOVE x TO y and y := x are the same thing.

With:

> MOVE x TO y and y := x do the same thing when you are copying one value. They are not interchangeable everywhere: only := and COMPUTE can evaluate an expression on the right, so you can write #TOTAL := #PRICE * #QTY but not MOVE #PRICE * #QTY TO #TOTAL.

**5. Lesson 4.4 code and prose, the divisor mismatch.**

Change the prose example from `DIVIDE 4 INTO #N` to `DIVIDE 5 INTO #N` so
it matches the code sample on the same step. Correction 1 above already
does this; if correction 1 is deferred, make this change on its own.

**6. Interpreter defects the lessons expose. File separately, fix before
these lessons are re-run.**

- The decimal-position cap is not enforced. `(N5.8)`, `(N5.15)` and
  `(P5.9)` are accepted and run. Real Natural rejects all three because m
  must not exceed 7. Add the check next to the existing 29-position check,
  which already produces a good teaching message ("'N22.8' asks for 30
  digit positions. Natural allows at most 29.").
- `MULTIPLY 3 BY #N` reports "Line 5: '3' has not been declared. Add it to
  the DEFINE DATA block before you use it." The rejection is correct; the
  message is not. It should name the concept, along the lines of "MULTIPLY
  stores its result in the first operand, so that operand must be a
  variable, not the constant 3. Write MULTIPLY #N BY 3."
- `COMPUTE #B = #A*2` reports "'#A*2' has not been declared". Natural's own
  rule is that an operator "should be preceded and followed by at least one
  blank so as to avoid any conflict with a variable name". If the
  interpreter is going to require the blanks, the message should say so
  rather than reporting a phantom undeclared variable. Note that `*` is not
  a legal Natural name character, so a real compiler would not fail this
  input the same way; the ambiguity the documentation is actually guarding
  against is the hyphen, as in `#A-2`.

## Sources

All accessed 2026-08-04.

- User-Defined Variables (format table, nn.m notation, 29 and 7 caps, I lengths): https://documentation.softwareag.com/natural/nat841unx/pg/pg_defi_dv.htm and https://documentation.softwareag.com/natural/nat914unx/pg/pg_defi_dv.htm
- DEFINE DATA, Function and Basic Syntax Rules (must be first statement): https://documentation.softwareag.com/natural/nat911mf/sm/defineda_basic.htm
- DEFINE DATA LOCAL (level numbers 01 to 99): https://documentation.softwareag.com/natural/nat913win/sm/defineda_lda.htm
- Rules and Naming Conventions (permitted first characters, # prefix): https://documentation.softwareag.com/natural/nat913win/using/use_rules.htm
- SG, Sign Position (default ON, applies to WRITE): https://documentation.softwareag.com/natural/nat828mf/parms/sp_sg.htm
- Parameters to Influence the Output of Fields (column width arithmetic including the sign position): https://documentation.softwareag.com/natural/nat828mf/pg/pg_output_parms.htm and https://documentation.softwareag.com/natural/nat914win/pg/pg_output_parms.htm
- AL, Alphanumeric Length for Output: https://documentation.softwareag.com/natural/nat841unx/parms/sp_al.htm
- NL, Numeric Length for Output: https://documentation.softwareag.com/natural/nat828mf/parms/sp_nl.htm
- WRITE statement: https://documentation.softwareag.com/natural/nat913win/sm/write.htm
- Statements DISPLAY and WRITE (example WRITEX01, the column evidence for field widths): https://documentation.softwareag.com/natural/nat913win/pg/pg_output_display.htm
- COMPUTE statement (ASSIGN equivalence, `:=` in structured mode, ROUNDED, blanks around operators): https://documentation.softwareag.com/natural/nat913win/sm/compute.htm
- Data Computation (MOVE versus COMPUTE, operator list): https://documentation.softwareag.com/natural/nat912win/pg/pg_furth_compu.htm
- Rules for Arithmetic Assignment (truncation, ROUNDED, powers of ten versus powers of two, data transfer): https://documentation.softwareag.com/natural/nat912mf/pg/pg_furth_arithm.htm and https://documentation.softwareag.com/natural/nat913win/pg/pg_furth_arithm.htm
- ADD statement: https://documentation.softwareag.com/natural/nat913win/sm/add.htm
- SUBTRACT statement: https://documentation.softwareag.com/natural/nat913win/sm/subtract.htm
- MULTIPLY statement: https://documentation.softwareag.com/natural/nat913win/sm/multiply.htm
- DIVIDE statement: https://documentation.softwareag.com/natural/nat913win/sm/divide.htm
- MOVE statement: https://documentation.softwareag.com/natural/nat913win/sm/move.htm
- Syntax Symbols and Operand Definition Tables (legend for C, S, A, G, N, M, E): https://documentation.softwareag.com/natural/nat914unx/sm/synsym.htm

Local verification: all eight lesson code samples plus the probe cases
(`MULTIPLY 3 BY #N`, `MULTIPLY #N BY 3`, `COMPUTE #B = #A*2`, `(N5.8)`,
`(N5.15)`, `(P5.9)`, `(N22.8)`, `(N30)`, `(I3)`) were executed with
`cargo run -q -p natural-cli` in this repository on 2026-08-04.
