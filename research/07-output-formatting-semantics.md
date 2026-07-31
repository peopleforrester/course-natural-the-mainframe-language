# Natural Output Formatting Semantics: WRITE, DISPLAY, and Edit Masks

Spike date: 2026-07-31

## Documentation baseline

All quotes and measurements in this document come from the Software AG product
documentation for **Natural for Windows 9.3.3** (the webhelp tree dated April 2026,
copyright line "Copyright © 1992-2026 Software GmbH"), at:

```
https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/
```

Two extraction methods were used. The PDF renditions (`pdf/sm.pdf`, `pdf/pg.pdf`,
`pdf/parms.pdf`, 2,714 pages total) were used to locate material. The HTML pages were
then used for every claim about **exact spacing**, because the PDF text layer collapses
runs of blanks and would have silently corrupted every column measurement. Wherever this
document states a character position, it was measured programmatically from the
whitespace-preserving `<pre>` blocks in the HTML.

A caution that applies throughout: Software AG renders some example output inside
`<pre>` blocks (whitespace exact, trustworthy) and some inside HTML **tables** (leading
blanks stripped by the browser, not trustworthy). The numeric edit-mask table in the EM
parameter reference is one of the table-rendered ones. Rows sourced from it are flagged.

## Executive summary

The model that falls out of the documentation is simple and highly regular, and it is
confirmed by direct measurement of eight separate official example programs.

Every output element occupies a **fixed print width** determined by its declared format,
and elements are joined with **exactly one blank**. There is no trimming anywhere.

* An alphanumeric `A(n)` field always occupies exactly `n` print positions. The value is
  left-justified and the remainder is **emitted as trailing blanks**. Trailing blanks are
  real output characters, not stripped.
* A numeric field occupies `(integer digits) + (1 if it has decimals) + (decimal digits)
  + 1` print positions. That final `+1` is a **leading** sign position, reserved
  unconditionally when `SG=ON`, which is the default. The value is right-justified within
  that width, leading zeros are suppressed, and the units digit is always printed.
* `P` prints identically to `N`. This is stated outright in the docs, not inferred.
* Elements are separated by one blank in `WRITE`. In `DISPLAY` the same one-blank default
  applies and is configurable via `SF`; in `WRITE` it is not configurable at all, because
  `SF` is absent from the `WRITE` parameter list.
* `DISPLAY` adds a column-header block that `WRITE` never produces: header text, an
  underline of hyphens spanning each column width, and one blank line beneath.

Two things could **not** be verified and are called out explicitly in the body: the exact
emission of a bare `WRITE` with no operands, and the exact printed form of a logical
(`L`) field with no edit mask. Everything else in the test table at the end is either
quoted from the docs or measured from official example output.

The single most consequential detail for fixture writing is the **leading sign position**.
It is easy to miss and it shifts every numeric column by one character.

---

## 1. WRITE default field output

### 1.1 The governing statements

The `WRITE` reference page states the free-format rule:

> The `WRITE` statement is used to produce output in free format.
>
> The `WRITE` statement differs from the `DISPLAY` statement in the following respects:
>
> * Line overflow is supported. If the line width is exceeded for a line, the next field
>   (or text) is written on the next line. Fields or text elements are not split between
>   lines.
> * No default column headers are created. **The length of the data determines the number
>   of positions printed for each field.**
> * A range of values/occurrences for an array is output horizontally rather than
>   vertically.

(Source: `sm/write.htm`, section "WRITE Usage". Emphasis added.)

"The length of the data determines the number of positions printed for each field" is the
whole rule for `WRITE`. There is no header, no centering, and no column negotiation. Each
field simply occupies its own print width.

The sign rule is stated on the `DISPLAY` page under "Defaults Applicable for a DISPLAY
Statement", and measurement confirms it applies identically to `WRITE`:

> **Sign**
> One extra high-order print position is reserved for a sign when printing a numeric
> field. The session parameter `SG` may be used to suppress the sign position.

(Source: `sm/display.htm`.)

"High-order" means leftmost. The sign position is a **leading** blank for a positive
value and a leading `-` for a negative value. This is confirmed by measurement in 1.3
below.

The justification rule, also from "Defaults Applicable for a DISPLAY Statement":

> The values contained in the field are left-justified for alphanumeric fields and
> right-justified for numeric fields.

And restated in the Programming Guide:

> By default, values are displayed left-justified in alphanumeric fields and
> right-justified in numeric fields. (These defaults can be changed with the `AD`
> parameter; see the Parameter Reference).

(Source: `pg/pg_output_parms.htm`, section "Leading Characters - LC Parameter".)

### 1.2 Alphanumeric `(A20)` holding 'Hello'

**Trailing blanks are emitted. The field is padded to its full declared length.**

The default output length is the declared field length, because the `AL` session
parameter has no default:

> **AL - Alphanumeric Length for Output**
> With this session parameter, you specify the default output length for an alphanumeric
> field; that is, when it is specified shorter than the field length, the field will be
> right-truncated.
>
> **Default setting**: none

(Source: `parms/sp_al.htm`.)

So `AL` is not applied unless you ask for it. Absent `AL` and absent an edit mask, an
`A20` field prints in 20 positions.

Measured proof, from example `WRITEX01` in the Programming Guide. The program is
`WRITE NAME FIRST-NAME SALARY (1:3)` where `NAME` and `FIRST-NAME` are both `A20` in the
`EMPLOYEES` DDM and `SALARY` is `P9`. The documented output, with columns measured:

```
JONES                VIRGINIA                  46000      42300      39300
```

| Element | Columns | Width |
|---|---|---|
| `NAME` (A20) | 1 to 20 | 20 |
| separator | 21 | 1 |
| `FIRST-NAME` (A20) | 22 to 41 | 20 |
| separator | 42 | 1 |
| `SALARY(1)` (P9) | 43 to 52 | 10 |
| separator | 53 | 1 |
| `SALARY(2)` (P9) | 54 to 63 | 10 |
| separator | 64 | 1 |
| `SALARY(3)` (P9) | 65 to 74 | 10 |

`JONES` is 5 characters and `FIRST-NAME` begins at column 22, which is only possible if
`NAME` occupied all 20 positions and one separator blank followed. The trailing blanks
are emitted.

(Source: `pg/pg_output_display.htm`, "Example of WRITE Statement". Measured from the
`<pre>` block.)

Therefore, for `#VAR (A20)` holding `'Hello'`:

```
Hello███████████████        (where █ denotes a blank; 5 chars + 15 blanks = 20)
```

A second independent confirmation comes from example `ASGEX1S`, where `#H (A3/1:3)` holds
`UVW`, blanks, `XYZ` and is written as `WRITE '=' #H (1:3)`:

```
#H: UVW     XYZ
```

Columns 5 to 7 hold `UVW`, column 8 is the separator, columns 9 to 11 hold the empty
second occurrence as three blanks, column 12 is the separator, columns 13 to 15 hold
`XYZ`. The empty middle occurrence is emitted as three blank characters, not skipped.

(Source: `sm/compute.htm`, example `ASGEX1S`.)

### 1.3 Numeric `(N7.2)` holding 19.99

First, the length notation. `N7.2` means 7 digits **before** the decimal point and 2
after, for 9 significant digits total:

> For fields defined with format N or P, you can use decimal position notation in the form
> `nn.mm`, where `nn` represents the number of positions before the decimal point, and
> `mm` represents the number of positions after the decimal point.

(Source: `pg` Programming Guide, "User-Defined Variables", format/length table.)

The Programming Guide's own annotated example is unambiguous:

```
1 #A4 (N7.2)      /* Unpacked numeric,
                  /* 7 positions before and 2 after decimal point.
1 #A6 (P7.2)      /* Packed numeric, 7 positions before and 2 after decimal point
                  /* and 1 sign position.
```

(Source: `pg`, "Examples of User-Defined Variables".)

So the print width of `N7.2` is:

```
1 (leading sign) + 7 (integer digits) + 1 (decimal separator) + 2 (decimals) = 11
```

**Exact output for 19.99:**

```
██████19.99                 (6 blanks, then 19.99; total 11 characters)
```

Breaking that down: column 1 is the sign position (blank, value is positive), columns 2
to 8 are the seven integer positions right-justified with leading zeros suppressed
(`█████19`), column 9 is the decimal separator, columns 10 and 11 are the decimals.

**There IS leading zero suppression, it IS right-justified, the total field width IS 11,
and there IS a leading position reserved for the sign.**

The zero suppression comes from the default edit mask, which the docs state explicitly:

> **Default Edit Masks**
> If no edit mask is specified for a field, a default edit mask is assigned to the field
> depending on the field format:
>
> | Field Format | Default Edit Mask |
> |---|---|
> | A | X |
> | B | H |
> | **N, P, I** | **Z9** |
> | F | scientific representation |
> | D | depends on default date format (as set with the profile parameter DTFORM) |
> | T | HH:II:SS |
> | L | blank / X |

(Source: `parms/sp_em.htm`, section "Default Edit Masks".)

`Z9` means: zero-suppress the leading digits, force the final integer digit. The `Z`
definition confirms the restriction that decimals are never suppressed:

> **Z**: Zero suppression for leading zeros. This is the default for numeric fields. The
> letter `Z` may be repeatedly specified to represent floating zero suppression. `Z` must
> not be specified to the right of the decimal separator character.

(Source: `parms/sp_em.htm`, "Characters for the Definition of Numeric Edit Masks".)

Measured proof of all of this, from example `ASGEX1S` in the `COMPUTE` reference. The
program declares `#A (N3)`, `#C (N0.3)`, `#D (N0.5)`, `#E (N1.3)`, `#F (N5)` and writes
each with the `'='` notation. The exact documented output:

```
#A:    5
#B: ABC
#C:  .450
#D: -.12345
#E: -0.123
#F:    200
#G: HELLO
#H: UVW     XYZ
```

Measured widths, where the `'='` notation emits `HEADING:` followed by one separator blank
and then the field:

| Variable | Format | Value | Print width | Rendered field | Check |
|---|---|---|---|---|---|
| `#A` | N3 | 5 | 3 + 1 = 4 | `███5` | `#A:` (3) + sep (1) + 4 = 8 chars ✓ |
| `#C` | N0.3 | .45 | 0 + 1 + 3 + 1 = 5 | `█.450` | 3 + 1 + 5 = 9 chars ✓ |
| `#D` | N0.5 | -0.12345 | 0 + 1 + 5 + 1 = 7 | `-.12345` | 3 + 1 + 7 = 11 chars ✓ |
| `#E` | N1.3 | -0.123 | 1 + 1 + 3 + 1 = 6 | `-0.123` | 3 + 1 + 6 = 10 chars ✓ |
| `#F` | N5 | 200 | 5 + 1 = 6 | `███200` | 3 + 1 + 6 = 10 chars ✓ |

(Source: `sm/compute.htm`, example `ASGEX1S`. Measured from the `<pre>` block.)

`#D: -.12345` is the decisive one. The minus sign sits in the **leftmost** position, in
front of the decimal point, exactly where "one extra high-order print position" predicts.
`#C:  .450` is the same field shape with a positive value and shows a blank in that same
leading position.

`#C: .450` also confirms two further points. Trailing decimal zeros are **printed**
(`.45` stored in `N0.3` renders as `.450`, not `.45`), and an `N0.n` field prints no
integer digit at all, because the default `Z9` mask's forced `9` is the units digit and
`N0.3` has none.

`#E: -0.123` is the complementary case: `N1.3` has exactly one integer position, and the
forced `9` prints the `0`.

Confirmed again by example `CPTEX1` in the same page, where `#B (N3.4)` holds `1.22`:

```
COMPUTE ROUNDED #B = 3 -4 / 2 * .89     #B:    1.2200
```

The literal is 35 characters (columns 1 to 35), `5X` occupies 36 to 40, `#B:` occupies 41
to 43, column 44 is the separator, and `#B` occupies columns 45 to 53. That is 9
positions, matching `3 + 1 + 4 + 1 = 9`, and the total line length is 53. Trailing
decimals are again forced: `1.22` renders as `1.2200`.

#### Value of 0

**An `N7.2` holding zero prints as `███████0.00`** (7 blanks, then `0.00`, total 11).

The relevant parameter default:

> **ZP - Zero Printing**
> This Natural profile and session parameter specifies how a field which contains a
> setting of all zeros is to be output.
>
> **ON**: Each field value which consists of all zeros is output as one zero, right
> justified (for numeric fields) or all zeros (for time fields).
> **OFF**: Each field value which consists of all zeros is suppressed.
>
> **Default setting**: ON

(Source: `parms/zp.htm`.)

Read in isolation, "output as one zero, right justified" could be taken to mean the whole
field collapses to a single `0` with no decimal point. It does not. Measured evidence
from example `REIEX3`, which declares `#B (N7.2)` and `#D (N3)` and renders them with the
value zero:

```
#A                      #B        0.00 #C       #D    0
```

`#B` occupies columns 25 to 26, column 27 is the separator, and the `#B` field occupies
columns 28 to 38: seven blanks then `0.00`, for 11 positions. `#D` occupies columns 49 to
50, column 51 is the separator, and the `#D` field occupies columns 52 to 55: three blanks
then `0`, for 4 positions matching `N3` at `3 + 1`.

(Source: `sm/reinput.htm`, example `REIEX3`. Measured from the `<pre>` block.)

Caveat: `REIEX3` is an `INPUT`/`REINPUT` screen rendering rather than a `WRITE`. It uses
the same field-width and edit-mask machinery and the widths agree exactly with every
`WRITE` measurement above, so I treat it as strong corroboration, but it is not a `WRITE`
example. See the open-questions section.

For an **integer** numeric field holding zero, a genuine `DISPLAY` example confirms the
single `0`. Example `FORMAX03` displays `BONUS` (a `P9` field, so a 10-position column)
for employees whose bonus is zero:

```
JONES                VIRGINIA                  46000       9000
JONES                MARSHA                    50000          0
```

The zero renders as a single `0` right-justified in the 10-position column, not as
`000000000`.

(Source: `pg/pg_output_parms.htm`, example `FORMAX03`.)

### 1.4 Integer `(I4)` holding 42

The documented decimal integer lengths:

> The following decimal integer lengths and possible values are applicable for format I:
>
> | Format/Length | Decimal Integer Length | Possible Values |
> |---|---|---|
> | I1 | 3 | -128 to 127 |
> | I2 | 5 | -32768 to 32767 |
> | I4 | 10 | -2147483648 to 2147483647 |

(Source: `pg`, "Rules for Arithmetic Assignment".)

The default edit mask for `I` is `Z9`, the same as `N` and `P`, and `SG=ON` reserves the
same leading sign position. So:

| Format | Digits | Print width |
|---|---|---|
| I1 | 3 | 4 |
| I2 | 5 | 6 |
| I4 | 10 | 11 |

**`I4` holding 42 prints as `█████████42`** (9 blanks then `42`, total 11).

The `I1` width is measured, not merely derived. Example `EMLOGV` declares `#INDEX (I1)`
and writes `WRITE NOTITLE #SWITCH (EM=FALSE/TRUE) 5X 'INDEX =' #INDEX`:

```
TRUE      INDEX =    1
ON        INDEX =    1
```

`#SWITCH` with `EM=FALSE/TRUE` occupies 5 positions (`TRUE` plus one pad, sized by the
longer of the two strings), `5X` supplies 5 blanks, `INDEX =` occupies columns 11 to 17,
column 18 is the separator, and `#INDEX` occupies columns 19 to 22. Four positions, which
is `3 + 1`. The second line is the same program with `EM=OFF/ON` (a 3-position mask) and
`7X`, and `INDEX` still lands on column 11, which cross-checks the arithmetic.

(Source: `parms/sp_em.htm`, example `EMLOGV`. Measured from the `<pre>` block.)

The `I2` and `I4` widths follow from the same rule applied to the documented digit counts,
but I found **no worked example** in the docs printing an `I2` or `I4` field, so those two
rows are marked DERIVED rather than VERIFIED in the test table.

### 1.5 Logical `(L)` holding TRUE/FALSE

**This is the one field format I could not fully verify. Treat with care.**

What the documentation does say, verbatim, is that the default edit mask for format `L` is
`blank / X` (from the Default Edit Masks table quoted in 1.3 above), and that logical edit
masks have this syntax:

> **Edit Masks for Logical Fields - Format L**
> For fields of format L (logical fields), edit masks can be defined as follows:
>
> `(EM=[false-string/]true-string)`
>
> The `false-string` must not be longer than 31 characters.

(Source: `parms/sp_em.htm`.)

Applying that syntax to the documented default `blank / X` gives false-string = a single
blank and true-string = `X`, hence a **1-position field printing `X` for TRUE and a blank
for FALSE**. The variable is 1 byte:

> For a variable of format L, no length can be specified. A variable of format L is always
> assigned a length of 1 byte by Natural.

(Source: `pg`, "Format L - Logical".)

Corroborating but not decisive: the `INPUT` statement documentation notes that data for
format `L` fields may be entered as blank (false) or non-blank (true), which matches the
same blank/`X` convention on the input side.

What the documentation does **not** contain, anywhere I could find across the Statements
reference, the Programming Guide, and the Parameter Reference, is a worked example that
writes a logical field **without** an edit mask and shows the output. Every logical example
in the docs (`EMLOGV`) supplies an explicit `EM=`. A search of the Software AG tech
community thread on format `L` variables found only discussion of the internal binary
representation, not the printed form.

So: the default mask string `blank / X` is VERIFIED verbatim. The resulting exact output
(`X` and a blank, in one print position) is a DERIVATION from that table plus the logical
edit-mask syntax. It is a well-supported derivation, but it is not a quoted example.

**It does not print `TRUE`/`FALSE`.** That form only appears when you write
`(EM=FALSE/TRUE)` explicitly, as `EMLOGV` does.

Recommendation for the course: teach logical output **with an explicit edit mask** in
every lesson that prints a logical. It is better pedagogy anyway (the learner sees where
`TRUE`/`FALSE` comes from) and it sidesteps the one fixture in this whole document I
cannot ground in a quoted example.

### 1.6 Packed `(P7.2)` versus `(N7.2)`

**They are identical in printed form.** This is stated outright:

> When a user-defined variable of format P is output with a `DISPLAY`, `WRITE`, or `INPUT`
> statement, Natural internally converts the format to N for the output.

(Source: `pg`, "User-Defined Variables", note following the format/length table.)

Confirmed by measurement. In `ASGEX1S`, `#A (N3)` occupies 4 positions. In `CPTEX1`,
`#A (P4)` occupies 5 positions and `#CUM-SALARY (P10)` occupies 11:

```
COMPUTE #A = 3 * 2 + 4 / 2 - 1          #A:     7
CUMULATIVE SALARY:       66300
```

For the first line: literal (30 chars, columns 1 to 30), `10X` (columns 31 to 40), `#A:`
(columns 41 to 43), separator (column 44), `#A` (columns 45 to 49, five positions,
`4 + 1`).

For the second line: the literal `'CUMULATIVE SALARY:'` is 18 characters (columns 1 to
18), column 19 is the separator, and `#CUM-SALARY` occupies columns 20 to 30, which is 11
positions matching `10 + 1`, with `66300` right-justified at columns 26 to 30. The total
line length is 30, which matches exactly.

(Source: `sm/compute.htm`, example `CPTEX1`. Measured from the `<pre>` block.)

So for the interpreter, `P` and `N` can share a single formatting routine. The difference
is purely internal storage.

---

## 2. Separation between operands

### 2.1 The gap is exactly one blank

For `DISPLAY` this is stated in prose:

> **Spacing Factor**
> The default spacing factor between elements is one position. There is a minimum of one
> space between columns (reserved for terminal attributes). This default may be overridden
> with the session parameter `SF`.

(Source: `sm/display.htm`, "Defaults Applicable for a DISPLAY Statement".)

And again in the Programming Guide:

> By default, the columns output with a `DISPLAY` statement are separated from one another
> by one space.

(Source: `pg/pg_output_display.htm`, "Column Spacing - SF Parameter and nX Notation".)

For `WRITE`, the documentation **never states the gap in prose**. I want to be explicit
about that, because it is the one high-traffic rule in this document that rests on
measurement rather than a quote. However, the measurement is unambiguous and consistent
across five independent official examples:

| Example | Evidence | Gap |
|---|---|---|
| `WRITEX01` | `NAME` A20 ends col 20, `FIRST-NAME` begins col 22 | 1 |
| `WRITEX01` | `SALARY(1)` ends col 52, `SALARY(2)` begins col 54 | 1 |
| `WRTEX1` | literal `'CITY:   '` ends col 8, `CITY` begins col 10 | 1 |
| `ASGEX1S` | `#H(1)` ends col 7, `#H(2)` begins col 9 | 1 |
| `CPTEX1` | literal ends col 18, `#CUM-SALARY` begins col 20 | 1 |

The `WRTEX1` row is the cleanest demonstration, because the literal itself ends in three
blanks. The source is `'CITY:   ' CITY` and the output is:

```
CITY:    MADRID
```

That is `CITY:` (5 characters), then the literal's own three trailing blanks, then **one
more blank** contributed by the inter-element gap, then the field. Four blanks total
between the colon and the `M`. The gap is real and additive, not a trim-and-rejoin.

**Conclusion: `WRITE` joins adjacent output elements with exactly one blank.**

### 2.2 SF applies to DISPLAY only

> **SF - Spacing Factor**
> This Natural profile and session parameter specifies the default number of spaces to be
> inserted between field settings of columns on Natural reports created using a `DISPLAY`
> statement.
>
> **Possible settings**: 1 - 30
> **Note**: The `SF` parameter cannot be set to 0; that is, at least one blank character
> must be placed between report columns.
>
> **Default setting**: 1

(Source: `parms/sf.htm`.)

Crucially, **`SF` does not appear in the `WRITE` statement's parameter list**. I verified
this against the full list on `sm/write.htm`, which enumerates `AD`, `AL`, `CD`, `CV`,
`DF`, `DL`, `DY`, `EM`, `EMU`, `FL`, `IS`, `LS`, `MC`, `MP`, `NL`, `PC`, `PM`, `PS`, `SG`,
`UC`, `ZP`, and nothing else. `SF` is absent.

So the `WRITE` inter-element gap is **fixed at one blank and not configurable**. To widen
it in a `WRITE` you use the `nX` notation:

> The `nX` notation is also available with the `WRITE` statement to insert spaces between
> individual output elements:
>
> `WRITE PERSONNEL-ID 5X NAME 3X JOB-TITLE`
>
> With the above statement, 5 spaces will be inserted between the fields `PERSONNEL-ID`
> and `NAME`, and 3 spaces between `NAME` and `JOB-TITLE`.

(Source: `pg/pg_output_display.htm`.)

Note the semantics: `nX` **replaces** the default gap, it does not add to it. Confirmed by
measurement in example `DISPLX04`, which uses `FORMAT SF=3` and `DISPLAY PERSONNEL-ID NAME
5X JOB-TITLE`:

```
30020013    GARRET                   TYPIST
```

`PERSONNEL-ID` occupies a 9-wide column (columns 1 to 9), the `SF=3` gap occupies columns
10 to 12, `NAME` begins at column 13 and runs to 32, the `5X` occupies columns 33 to 37,
and `JOB-TITLE` begins at column 38. Five, not six.

(Source: `pg/pg_output_display.htm`, example `DISPLX04`. Measured.)

One documentation inconsistency worth flagging: the parameter table on `sm/display.htm`
marks `SF` as specifiable at both statement and element level (`SE`), while
`pg/pg_output_display.htm` says "with a `DISPLAY` statement at statement level, but not at
element level." The Programming Guide wording is the more specific of the two. This does
not affect v1, since `SF` is out of scope for a Tier 1 course, but do not encode `SF` at
element level without testing.

---

## 3. WRITE versus DISPLAY

### 3.1 The core difference

> The `DISPLAY` statement is used to specify the fields to be output on a report in column
> format. A column is created for each field and a field header is placed over the column.
>
> **Note**: The statements `WRITE` and `PRINT` can be used to produce output in free
> (non-column) format.

(Source: `sm/display.htm`, "DISPLAY Usage".)

> The `WRITE` statement is used to produce output in free format (that is, not in
> columns). In contrast to the `DISPLAY` statement, the following applies to the `WRITE`
> statement:
>
> * If necessary, it automatically creates a line advance; that is, a field or text element
>   that does not fit onto the current output line, is automatically output in the next
>   line.
> * **It does not produce any headers.**
> * The values of a multiple-value field are output next to one another horizontally, and
>   not underneath one another.

(Source: `pg/pg_output_display.htm`, "WRITE Statement". Emphasis added.)

**`WRITE` emits no headers at all.** The `NOHDR` option on `WRITE` exists only to suppress
headers that a *`DISPLAY`* statement elsewhere in the same program would otherwise
regenerate on a new page:

> The `WRITE` statement itself does not produce any column headers. However, if you use the
> `WRITE` statement in conjunction with a `DISPLAY` statement, you can use the `NOHDR`
> option of the `WRITE` statement to suppress the column headers generated by the `DISPLAY`
> statement.

(Source: `sm/write.htm`.)

### 3.2 What the default DISPLAY headers actually are

The priority order is documented twice, with slightly different wording. The
authoritative enumeration:

> Column headers are produced for each field specified in the `DISPLAY` statement using the
> following rules:
>
> * The header text may be explicitly specified in the `DISPLAY` statement before the field
>   name. For example: `DISPLAY 'EMPLOYEE' NAME 'SALARY' SALARY`
> * If you do not specify an explicit header for a field, the header as defined in the
>   `DEFINE DATA` statement will be used.
> * If for a database field no header is defined in the `DEFINE DATA` statement, the default
>   header as defined in the DDM will be used.
> * If no default header is defined in the DDM, the field name will be used as header.
> * **If for a user-defined variable no header is defined in the `DEFINE DATA` statement,
>   the variable name will be used as header.**
> * Natural always underlines column headings and generates one blank line between the
>   underlining and the data being displayed.
> * If there are multiple `DISPLAY` statements in a program, the first `DISPLAY` statement
>   determines the column header(s) to be used; this is evaluated at compilation time.

(Source: `sm/display.htm`, under the `NOHDR` entry. Emphasis added.)

For a teaching interpreter with no DDM, the operative rule is the emphasized one: **the
header is the variable name**, including its leading `#`.

The condensed restatement elsewhere on the same page:

> Column headings are obtained and used by Natural according to the following priority:
>
> 1. heading `'text'` supplied in the `DISPLAY` statement;
> 2. the default heading defined in the DDM (database fields), or the name of a user-defined
>    variable;
> 3. the field name as defined in the DDM (if no heading text was defined for the database
>    field).
>
> The maximum number of column header lines is 15.

### 3.3 How columns are sized

> **Field Output**
> The length of the field or the field heading, whichever is greater, determines the column
> width for the report (unless the `HW` parameter is used).
>
> * If the field is longer than the heading, the heading will be centered over the column
>   unless the `HC=L` or `HC=R` parameter is used to produce a left-justified or
>   right-justified heading.
> * If the heading is longer than the field, the field will be left-justified under the
>   heading.
> * The values contained in the field are left-justified for alphanumeric fields and
>   right-justified for numeric fields.
> * Numeric fields may be displayed left-justified by specifying `AD=L`.
> * Alphanumeric fields may be displayed right-justified by specifying `AD=R`.
> * In a vertical display, the longest data value or heading among all fields determines the
>   column width (unless the `HW` parameter is used).

(Source: `sm/display.htm`, "Defaults Applicable for a DISPLAY Statement".)

Column width is therefore `max(field print width, header width)`, where field print width
already includes the numeric sign position.

Header centering is confirmed by measurement. Example `WRTEX3` displays `CITY` (A20),
`NAME` (A20), and `SALARY` (P9, so a 10-position field):

```
        CITY                 NAME           ANNUAL
                                            SALARY
-------------------- -------------------- ----------

ALBUQUERQUE          HAMMOND                   22000
```

`CITY` is 4 characters centered in a 20-wide column, landing at columns 9 to 12 (8 blanks
of left pad). `NAME` is 4 characters centered in columns 22 to 41, landing at 30 to 33.
`ANNUAL` is 6 characters centered in the 10-wide column at columns 43 to 52, landing at 45
to 50 (2 pad each side). The data row shows `22000` right-justified at columns 48 to 52.

(Source: `sm/write.htm`, example `WRTEX3`. Measured from the `<pre>` block.)

A cleaner sizing example is `DISPLX01`, which displays `PERSONNEL-ID` (an `A8` field in
the `EMPLOYEES` DDM) with the DDM header `PERSONNEL ID`:

```
PERSONNEL         NAME                  CURRENT
   ID                                  POSITION
--------- -------------------- -------------------------

30020013  GARRET               TYPIST
```

The header word `PERSONNEL` is 9 characters, longer than the 8-character field, so the
column is 9 wide (9 hyphens) and the field is **left-justified under the header**:
`30020013` occupies columns 1 to 8, column 9 is header-driven padding, column 10 is the
separator, and `NAME` begins at column 11.

(Source: `pg/pg_output_display.htm`, example `DISPLX01`. Measured.)

### 3.4 How the header is underlined

> By default, titles and headers are underlined with a hyphen (`-`).

(Source: `pg/pg_output_headers.htm`, "Underlining Character for Titles and Headers - UC
Parameter".)

> **UC - Underlining Character**
> This session parameter determines the character that is used as underlining character for
> the following: column headings generated by `DISPLAY` statements; page titles/trailers
> produced by `WRITE TITLE` / `WRITE TRAILER` statements with `UNDERLINED` option.
>
> **Default setting**: `-` Hyphen (-).

(Source: `parms/sp_uc.htm`.)

The underline spans **each column's full width**, and the inter-column gaps are **not**
underlined. Measured from `WRTEX3` above: `--------------------` (20) + one blank +
`--------------------` (20) + one blank + `----------` (10), for a total of 52 characters,
which matches the data rows exactly.

And one blank line follows the underline, per the rule quoted in 3.2: "Natural always
underlines column headings and generates one blank line between the underlining and the
data being displayed."

Header centering default:

> By default, column headers are centered above the columns. With the `HC` parameter, you
> can influence the placement of column headers. If you specify `HC=L` headers will be
> left-justified. `HC=R` headers will be right-justified.

(Source: `pg/pg_output_headers.htm`.)

### 3.5 The default page title, and a screen-versus-paper offset

Both statements generate a default title line unless `NOTITLE` is given:

> Natural generates a single title line for each page resulting from a `WRITE` statement.
> This title contains the page number, the time of day, and the date. Time of day is set at
> the beginning of program execution.

(Source: `sm/write.htm`, `NOTITLE` entry.)

Measured from eight separate documented outputs, all identical in layout:

```
Page      1                                                  04-11-11  14:15:54
```

| Element | Columns |
|---|---|
| `Page` | 1 to 4 |
| page number, right-justified | ends at 11 |
| date | 62 to 69 |
| time | 72 to 79 |

Total line width 79 in these examples. The date and time block is flush right against the
line size, so this layout is a function of `LS` and will move if `LS` differs.

Almost every lesson will use `NOTITLE`, so this matters mainly for a lesson that
deliberately teaches page titles.

One more layout rule that affects fixtures:

> **Terminal Screen Output**
> When the `DISPLAY` output is displayed on a terminal (emulation) screen, the output begins
> in physical Column 2 (because Column 1 must be reserved for possible use as an attribute
> position on a 3270-type terminal).
>
> **Printout on Paper**
> When the `DISPLAY` output is printed on paper, the printout begins in the leftmost column
> (Column 1).

(Source: `sm/display.htm`.)

This explains an inconsistency you will notice reading the docs: some example outputs are
indented by two spaces (`FORMAX03`, `FORMAX04`, `FORMAX05` are screen captures) and some
start at column 1 (`DISPLX01`, `WRITEX01`, `WRTEX3` are paper renderings). Both are
correct for their respective medium.

**Recommendation for this course:** use the **column 1** convention. xterm.js has no 3270
attribute byte, the reserved column would be a confusing artifact for a beginner, and the
majority of the doc examples that a learner might compare against use column 1. Document
the choice in the course so it is deliberate rather than accidental.

---

## 4. Blank or empty WRITE

**UNVERIFIED.** I could not find documentation of what a bare `WRITE` with no operands
emits, and I am not going to guess at it.

What I established:

The syntax diagram on `sm/write.htm` is composed from bracket **images** rather than text,
so the optionality of the output-element group cannot be read from the page text. Decoding
the image sequence gives this structure:

```
WRITE [(rep)] [NOTITLE] [NOHDR] [(statement-parameters)]
  { [ nX | nT | x/y | T*field | P*field | / ]... { 'text' [(attributes)] | 'c'(n) [(attributes)] | ['='] operand1 [(parameters)] } }...
```

The positioning notations sit inside square brackets (optional, repeatable) and the
text/operand alternatives sit inside curly braces (a mandatory choice) with a trailing
ellipsis. On that reading, at least one text or operand element is **required**, and a
bare `WRITE` would be a compile error. I could not confirm the bracket convention because
the "Syntax Symbols" page that `sm/write.htm` links to returns 404 at every URL I tried in
the 9.3.3 tree.

What the documentation **does** establish is that a bare `WRITE` is not the idiomatic way
to produce a blank line. There are two documented mechanisms:

> The `SKIP` statement is used to generate one or more blank lines in an output report.
>
> `operand1` represents the number (1 - 250) of blank lines to be generated.

(Source: `sm`, `SKIP` statement.)

And the slash notation inside a `WRITE`:

> **Line Advance - Slash Notation**: When placed between fields or text elements, a slash
> (`/`) causes positioning to the beginning of the next print line. Multiple slash (`/`)
> notations may be used to cause multiple line advances.

(Source: `sm/write.htm`.)

Example `WRTEX1` uses trailing `//` to produce blank lines, and the documented output
confirms the blank line appears.

**Recommendation:** teach `SKIP 1` and `WRITE /` for blank lines, which are both
documented and unambiguous, and either reject a bare `WRITE` with a teaching diagnostic or
defer the decision until it can be checked against a real Natural runtime. Do not silently
invent a behavior for it.

---

## 5. Text literals

### 5.1 Literals are emitted verbatim with no padding

> **Text Assignment: `'text'`**
> The character string enclosed by single quotes is displayed.

(Source: `sm/write.htm`, "Text/Attribute Assignments".)

Measured proof, again from `WRTEX1`. The source line is:

```
'CITY:   ' CITY    /
```

and the output line is:

```
CITY:    MADRID
```

The literal is exactly 8 characters (`CITY:` plus three trailing blanks) and occupies
columns 1 to 8 unchanged. No padding is added, no trailing blanks are stripped. Column 9
is the inter-element separator and `CITY` (an A20 field) begins at column 10.

The `COUNTRY` line in the same example is the same shape:

```
COUNTRY: E
```

The literal `'COUNTRY:'` is 8 characters at columns 1 to 8, column 9 is the separator, and
the field begins at column 10.

(Source: `sm/write.htm`, example `WRTEX1`. Measured.)

So: literals contribute exactly their own characters, and the one-blank inter-element gap
is applied on top.

### 5.2 Quote escaping: yes, doubling

> **Apostrophes Within Alphanumeric Constants**
> If you want an apostrophe to be part of an alphanumeric constant that is enclosed in
> apostrophes, you must write this as **two apostrophes** or as a single quotation mark.
>
> If you want an apostrophe to be part of an alphanumeric constant that is enclosed in
> quotation marks, you write this as a single apostrophe.
>
> Example: If you want the following to be output:
>
> ```
> HE SAID, 'HELLO'
> ```
>
> you can use any of the following notations:
>
> ```
> WRITE 'HE SAID, ''HELLO'''
> WRITE 'HE SAID, "HELLO"'
> WRITE "HE SAID, ""HELLO"""
> WRITE "HE SAID, 'HELLO'"
> ```

(Source: `pg`, "User-Defined Constants". Emphasis added.)

Note the wrinkle in the second and third forms: a double-quoted literal's quotation marks
are **translated to apostrophes** on output. The docs flag this as configurable:

> **Note**: If quotation marks are not converted to apostrophes as shown above, this is due
> to the setting of profile parameter `TQMARK` (Translate Quotation Marks); ask your Natural
> administrator for details.

Both apostrophes and quotation marks are valid literal delimiters:

> An alphanumeric constant must be enclosed in either apostrophes (`'`) or quotation marks
> (`"`).

Alphanumeric constants also concatenate with a hyphen:

```
MOVE 'XXXXXX' - 'YYYYYY' TO #FIELD
```

**Recommendation for v1:** support the single-quote form with doubling (`''`), since that
is the form used in essentially every documented example and in the
`SoftwareAG/adabas-natural-code-samples` repository. The `TQMARK` translation behavior is
a genuine trap for a beginner course; either support double-quoted literals with the
documented translation, or reject them with a teaching diagnostic that names the rule. Do
not accept double quotes and silently pass the quotation marks through, which would be a
third behavior matching neither `TQMARK` setting.

---

## 6. Line wrapping

### 6.1 Default line size

> **LS - Line Size**
> This Natural profile and session parameter specifies the maximum number of characters
> permitted per line for `DISPLAY`, `INPUT` and `WRITE` statements.
>
> **Profile Parameter LS**: When used as a profile parameter, `LS` is honored in batch mode
> only and defines the physical line size. In online mode, the line size is always set to
> the physical screen width.
>
> Possible settings: 35 - 250 (maximum number of characters permitted per line), or 0 (use
> physical line size, mostly 132).
>
> **Default setting**: 0
>
> **Session Parameter LS**: Possible settings 2 - 250. **Default setting**: Physical line
> size.

(Source: `parms/ls.htm`.)

The `DISPLAY` page restates it:

> **Report Width**
> The width of the report defaults to the value set when Natural is installed. This default
> value is normally 132 in batch mode or the line length of the terminal in TP mode. It may
> be overridden with the session parameter `LS`. In TP mode, line size (`LS`) and page size
> (`PS`) parameters are set by Natural based on the physical characteristics of the terminal
> type in use.

(Source: `sm/display.htm`.)

So there is **no single fixed default**. It is 132 in batch, or the terminal width online.
The documented examples in the Programming Guide render at a line size of 79 or 80.

For companion reference, page size:

> **PS - Page Size for Natural Reports**
> This Natural profile and session parameter specifies the maximum number of lines per page
> to be used for Natural reports created with the `DISPLAY` or `WRITE` statement.
> Possible settings: 1 - 250, or 0 (the physical page size is to be used).
> **Default setting**: 0

(Source: `parms/ps.htm`.)

**Recommendation:** pick an explicit `LS` for the course terminal and state it in the
lesson material rather than inheriting an implicit value. Since the doc examples render at
79 to 80 and xterm.js defaults to 80 columns, `LS=80` is the least surprising choice and
makes doc examples directly comparable. Whatever you choose, it must be a fixed constant
in the interpreter, because every expected-output fixture that wraps depends on it.

### 6.2 Behavior on overflow: WRITE wraps, DISPLAY errors

This is a hard behavioral split between the two statements.

`WRITE`:

> Line overflow is supported. If the line width is exceeded for a line, the next field (or
> text) is written on the next line. **Fields or text elements are not split between
> lines.**

(Source: `sm/write.htm`. Emphasis added.)

So `WRITE` wraps at **element granularity**. An element that does not fit is moved whole to
the next line. It is never split mid-field.

`DISPLAY`:

> Line size overflow is not permitted for output resulting from a `DISPLAY` statement. If a
> line overflow occurs, an error message is issued.

(Source: `sm/display.htm`.)

The Programming Guide notes the practical consequence for arrays, which is a real trap
worth teaching:

> As a `WRITE` statement displays multiple values horizontally instead of vertically, this
> may cause a line overflow and a - possibly undesired - line advance.

(Source: `pg`, "Index Notation in WRITE Statement".)

I did **not** verify the specific error number Natural issues for a `DISPLAY` line
overflow. The Messages and Codes manual was not consulted. If a lesson needs to show that
error, the number must be looked up before publishing.

---

## 7. Edit masks (EM=)

### 7.1 Scope and precedence

> With the session parameter `EM` you can specify an edit mask for an alphanumeric or
> numeric field, that is, determine character by character the format in which the field
> values are to be output.

(Source: `pg/pg_output_masks.htm`.)

Precedence, which matters for the interpreter's parameter resolution order:

> An edit mask specified at statement level will override an edit mask specified at report
> level. An edit mask specified at element level will override an edit mask specified at
> statement level.

> **An edit mask overrides any settings for the session parameters `AL`, `NL` and `SG`.**

(Source: `parms/sp_em.htm`, "EM Parameter Syntax". Emphasis added.)

That last rule is important and is stated three separate times across the docs (on the
`AL`, `NL`, and `SG` parameter pages as well). **When an edit mask is present, the
automatic sign position is gone.** The mask defines the field width completely. Getting
this wrong would put a phantom leading blank in front of every masked value.

`EM=OFF` disables masking entirely, including any DDM-defined default mask.

### 7.2 The character vocabulary

Numeric masks (formats N, P, I, F):

> An edit mask specified for a field of format N, P, I, or F must contain at least one `9`
> or `Z`.
>
> If more `9`s or `Z`s exist than the number of positions contained in the field value, the
> number of print positions in the edit mask will be adjusted to the number of digits
> defined for the field value.
>
> If fewer `9`s or `Z`s exist, the high-order digits before the decimal separator and/or
> low-order digits after the decimal separator will be truncated.

| Character | Documented function (verbatim) |
|---|---|
| `9` | "Position to be displayed (one digit of the field value)." |
| `Z` | "Zero suppression for leading zeros. This is the default for numeric fields. The letter `Z` may be repeatedly specified to represent floating zero suppression. `Z` must not be specified to the right of the decimal separator character. A zero value may be displayed as blanks using all `Z`s in the edit mask (see also session parameter `ZP`)." |
| `.` (period) | "The first period inserted is used as a decimal separator. Subsequent periods are treated as literal characters." (The actual character is whatever `DC` is set to.) |

**So `9` versus `Z` is exactly: forced digit versus zero-suppressed digit.** A `9`
position always prints a digit, including a leading zero. A `Z` position prints a blank
(or the filler character) instead of a leading zero. This is why the default mask `Z9`
prints `0` for a zero value: every position is suppressed except the final forced `9`.

Alphanumeric masks:

> Edit masks for alphanumeric fields must include an `X` for each alphanumeric character
> that is to be output. With a few exceptions, you may add leading, trailing and insertion
> characters (with or without enclosing them in apostrophes).

Hexadecimal masks:

> If the character `H` is specified as the first character in an edit mask, the content of
> an alphanumeric or numeric field will be displayed in hexadecimal format. **Each `H`
> represents two print positions** that will occur for each byte in the source field.
> Characters other than `H` serve as insertion or trailing characters in the mask.

(All from `parms/sp_em.htm`. Emphasis added.)

Confirmed: `A2` holding `AB` with `EM=HH` renders as `4142` in ASCII (4 print positions
from 2 mask characters).

Blanks:

> Blanks within an edit mask are represented by the character on your keyboard that in
> hexadecimal code corresponds to `H'20'` (ASCII) or `H'5F'` (EBCDIC), that is, the
> character `^` (or `¬`).
>
> Blanks behind the equal sign (`=`) of the `EM` parameter are not allowed (for example:
> `EM=<blank>XXX`).

Abbreviated notation:

> You may replace a sequence of the same significant characters with a numeric notation,
> such as `x(8)` for `xxxxxxxx`.
>
> ```
> EM=9(4)-9(5)         is equivalent to: EM=9999-99999
> EM=H(10)             is equivalent to: EM=HHHHHHHHHH
> EM=X(6)..X(3)        is equivalent to: EM=XXXXXX..XXX
> EM=YYYY-L(8)-DD-N(8) is equivalent to: EM=YYYY-LLLLLLLL-DD-NNNNNNNN
> ```

### 7.3 A correction on `I`

**`I` is not a numeric or alphanumeric edit-mask character.** In the edit-mask vocabulary,
`I` means **minutes**, and only in masks for fields of format `T` (time):

| Character | Usage (verbatim, Time format T only) |
|---|---|
| `T` | "Tenths of a second." |
| `SS` | "Seconds." |
| `ZS` | "Seconds, with zero suppression." |
| `II` | "Minutes." |
| `ZI` | "Minutes, with zero suppression." |
| `HH` | "Hours." |
| `ZH` | "Hours, with zero suppression." |
| `AP` | "AM/PM element." |

(Source: `parms/sp_em.htm`, "Time - Format T - only".)

The default edit mask for a `T` field is `HH:II:SS`, per the Default Edit Masks table.

`I` elsewhere in Natural means the **integer data format** (`I1`, `I2`, `I4`), which is a
different concept entirely. Worth being precise about in lesson copy, since a learner who
sees `I` in both places will conflate them.

### 7.4 Sign handling in masks

Leading sign characters, which must be the first character before the `9`s or `Z`s:

| Character | Documented function (verbatim) |
|---|---|
| `+` | "A floating sign is to be displayed preceding (leading sign character) or following (trailing sign character) the number. The sign may be generated as a plus or minus depending on the value of the field." |
| `-` | "A floating minus is to be displayed preceding (leading sign character) or following (trailing sign character) the number if the value of the field is negative." |
| `S` | "A sign is to be displayed to the left of the column. A plus sign is displayed for a positive value and a minus sign is displayed for a negative value." |
| `N` | "A minus sign is to be displayed to the left of the column if the value of the field is negative." |

Trailing signs:

> A trailing sign character can be specified for numeric edit masks by using the `+` or `-`
> character as the last character in the edit mask. A `+` will produce a trailing `+` or `-`
> sign depending on the value of the field. A `-` will produce a trailing space or `-` sign
> depending on the value of the field. If a leading and trailing sign are specified in the
> edit mask, both will be produced.

(Source: `parms/sp_em.htm`.)

### 7.5 Literal leading, insertion, and trailing characters

This is the subtlest rule in the whole edit-mask system, and it is easy to implement
wrongly:

> Any number of literal leading characters can appear before the first displayable position
> (as indicated by `Z` or `9`). These must follow any sign character. If there is no sign
> character and the first literal leading character is `+`, `-`, `S` or `N`, it must be
> enclosed in apostrophes. If a literal leading character is `H`, `X`, `Z` or `9`, it must
> be enclosed in apostrophes.
>
> **The first literal leading character specified will appear in the output only if the
> value contains leading zeros and the edit mask is defined with `Z` (leading zero
> suppression). This character will then be used as a filler character displayed instead of
> a blank for leading zeros. Subsequent literal leading characters will be displayed as they
> are input.**

(Source: `parms/sp_em.htm`. Emphasis added.)

In other words, the **first** leading literal is a **fill character**, not a printed
character. It is consumed, and it only shows up in positions that zero suppression
blanked out.

The documented `EM=*EURZZ9.9` row demonstrates all of it at once. `*` is the fill
character (never printed as itself), `EUR` are ordinary leading literals (always printed),
`ZZ9` is three digit positions with the first two suppressible, and `.9` is a forced
decimal:

| Value | Output |
|---|---|
| `0000.03` (N4.2) | `EUR**0.0` |
| `-0054` (N4) | `EUR*54.` |
| `0962` (N4) | `EUR962.` |

The same rule applies to alphanumeric masks:

> If leading characters are used before the first displayable position `X` of an
> alphanumeric edit mask, the first of these leading characters will not be displayed, but
> is used as filler character and replaces all leading blanks in the alphanumeric output
> field.

Confirmed by the documented example `WRITE #TEXT (EM=1234XXXX)` where `#TEXT` is `A4`
holding `BLUE`: the output is `234BLUE`. The `1` is consumed as the fill character and
never appears because `BLUE` has no leading blanks; `234` print normally.

And by the `#X (A4) INIT <'  34'>` example, where the value **does** have leading blanks:

```
WRITE #X (EM=*A:X:) 6X #X (EM=*A:XX:) 6X #X (EM=*A:XXX:) 6X #X (EM=*A:XXXX:)
```

produces

```
A:*:      A:**:      A:**3:      A:**34:
```

The `*` fill character replaces each leading blank.

### 7.6 Mask length versus field length

> **Length of Fields**
> It is important to be aware of the length of the field to which you assign an edit mask.
>
> * If the edit mask is longer than the field, this will yield unexpected results.
> * If the edit mask is shorter than the field, the field output will be truncated to just
>   those positions specified in the edit mask.

More precisely, for alphanumeric:

> If the number of positions specified with the mask is smaller than the field length, the
> overhanging field content is not displayed. If the number of positions specified with the
> mask is higher than the field length, the mask is truncated on the first overhanging
> position.

Confirmed by the documented pair:

```
WRITE #TEXT (EM=X-X-X)        /* 'B-L-U', 3 bytes of field only.
WRITE #TEXT (EM=X-X-X-X-X)    /* 'B-L-U-E-', with truncated mask.
```

Note `B-L-U-E-` retains the trailing separator: the mask is truncated at the first
overhanging **displayable** position, so the `-` before it survives.

Documented examples for `A12` holding `JOHNSON`:

| Edit Mask | Output |
|---|---|
| `EM=X.X.X.X.X` | `J.O.H.N.S` |
| `EM=****XXXXXX****` | `****JOHNSO**` |

For numeric, truncation is high-order for the integer part and low-order for decimals, per
the rule quoted in 7.2.

### 7.7 `EM=ZZZ,ZZ9.99` and thousands separators

The user asked specifically about `EM=ZZZ,ZZ9.99`. The documentation does **not** contain
that exact mask. It contains the closely related `EM=ZZ,ZZZ,ZZ9.99`, with a fully
documented output:

> A Natural program that is cataloged with parameter settings `DC='.'` and `THSEP=ON` uses
> the edit mask `(EM=ZZ,ZZZ,ZZ9.99)`.
>
> | Parameter Settings at Runtime | Displays as |
> |---|---|
> | `DC='.'` and `THSEPCH=','` | `1,234,567.89` |
> | `DC=','` and `THSEPCH='.'` | `1.234.567,89` |
> | `DC=','` and `THSEPCH='/'` | `1/234/567,89` |
> | `DC=','` and `THSEPCH=' '` | `1 234 567,89` |
> | `DC=','` and `THSEPCH=''''` | `1'234'567,89` |

(Source: `pg/pg_output_masks.htm`, "Customizing Separator Character Displays".)

This example has **no suppressed leading region** (the value fills every digit position),
so it does not tell you what happens to a comma that falls inside the zero-suppressed
zone.

There is a second trap here. Whether the comma is a literal or a dynamic separator depends
on a **compile-time** profile parameter:

> If `THSEP` is set to `OFF` (**default**), any character used as thousands separator in the
> edit mask is treated as literal and displayed unchanged at runtime. This setting retains
> downwards compatibility.
>
> If `THSEP` is set to `ON`, any comma (or period) in the edit mask is interpreted as dynamic
> thousands separators.

So **by default the comma is a plain literal**, and only becomes a locale-aware separator
when `THSEP=ON`.

**What I could not verify:** whether a literal insertion character (the comma) that falls
within a zero-suppressed region is itself suppressed, printed, or replaced by the fill
character. The `*EURZZ9.9` example shows the fill character applied to suppressed *digit*
positions, but no documented example places an insertion character inside a suppressed
region. Every table-rendered example that might have shown it has its leading whitespace
stripped by the HTML.

**Recommendation:** for v1, do not build lesson fixtures on `EM=ZZZ,ZZ9.99` with values
small enough to leave the comma inside the suppressed region. Either use values in the
millions (where the documented `1,234,567.89` behavior applies directly), or verify the
suppression behavior against a real Natural runtime first. This is a genuine gap, not a
formatting nicety, because the difference is a visible character in a beginner's first
"format a number" lesson.

### 7.8 Date and time masks

Date characters (format D):

| Character | Usage (verbatim) |
|---|---|
| `DD` | "Day." |
| `ZD` | "Day, with zero suppression." |
| `MM` | "Month." |
| `ZM` | "Month, with zero suppression." |
| `YYYY` | "Year, 4 digits." |
| `YY` | "Year, 2 digits." |
| `Y` | "Year, 1 digit. Must not be used for input fields." |
| `WW` / `ZW` | "Number of week" (with zero suppression for `ZW`). |
| `JJJ` / `ZZJ` | "Julian day" (with zero suppression for `ZZJ`). |
| `NN...` or `N(n)` | "Name of day (language-dependent)." |
| `O` | "Number of week day." |
| `LL...` or `L(n)` | "Name of month (language-dependent)." |
| `R` | "Year in Roman numerals (maximum 13 digits). Must not be used for input fields." |

(Source: `parms/sp_em.htm`, "Date - Format D, and Time - Format T".)

**`EM=YYYY-MM-DD` is verified with exact output.** Example `WRTEX4` displays `BIRTH (EM=YYYY-MM-DD)`:

```
WILCOX               NASHVILLE            1970-01-01
MORRISON             NASHVILLE            1949-07-10
BOYER                NEMOURS              1955-11-23
```

Ten print positions, zero-padded components, hyphen literals.

(Source: `sm/write.htm`, example `WRTEX4`. Measured from the `<pre>` block.)

`EM=MM/DD/YYYY` is verified from example `EMDATI`, which renders `*DATX` on 12 January
2005 as `01/12/2005`. `EM=MM.DD.YY` is verified from the Programming Guide's
examples-of-edit-masks table, showing `01.05.87` and `12.22.86`.

**`EM=DD/MM/YY` is not shown verbatim anywhere I found.** Given that `DD`, `MM`, and `YY`
are each individually documented as zero-padded fixed-width components and the `/` is an
ordinary literal, the output for 5 January 1987 is `05/01/87`. I mark this DERIVED, at high
confidence, rather than VERIFIED.

Note the syntactic restrictions, which a validating interpreter should enforce:

> For Input edit masks, you may not use the following: `DD` or `ZD` without `MM` or `ZM` or
> `LL` or `L(n)`; `MM` or `ZM` without `YYYY` or `YY`; `WW` or `ZW` without `YYYY` or `YY`;
> `JJJ` or `ZZJ` with `MM` or `ZM`; ... `MM` or `ZM` with `WW` or `ZW`.

And for both input and output masks, month-number and month-name are mutually exclusive
(`MM`/`ZM` with `LL`/`L(n)`), as are day-name and week-day-number (`NN`/`N(n)` with `O`).

Time examples, verified from `EMDATI`:

| Mask | Output |
|---|---|
| (none, format T default `HH:II:SS`) | `16:04:14` |
| `EM=HH.II.SS.T` | `16.04.14.8` |
| `EM=HH.II.SS' 'AP` | `04.04.14 PM` |
| `EM=HH` | `16` |

Date fields **without** an edit mask fall back to the `DF` parameter:

> **DF - Date Format**
> With the `DF` session parameter, you determine the length of a date when converted into
> alphanumeric representation without an edit mask being specified.
>
> **S**: 8-byte representation with 2-digit year component and delimiters (`yy-mm-dd`).
> **I**: 8-byte representation with 4-digit year component and no delimiters (`yyyymmdd`).
> **L**: 10-byte representation with 4-digit year component and delimiters (`yyyy-mm-dd`).
>
> **Default setting**: S
>
> **Notes**: The `DF` parameter is evaluated at compilation time. The sequence of the day,
> month and year components and the delimiter characters used are determined by the profile
> parameter `DTFORM`.

(Source: `parms/sp_df.htm`.)

So an unmasked `D` field defaults to an 8-character `yy-mm-dd` form, and the component
order and delimiter are themselves governed by `DTFORM`. For a course, always supply an
explicit edit mask on date output. The default is locale-dependent in two directions at
once and is not a stable fixture.

### 7.9 The documented numeric edit-mask results table

Reproduced from `parms/sp_em.htm`, "Examples of Numeric Edit Masks". The column headings
are the stored values and their formats.

**Whitespace warning:** this table is rendered as an HTML table, not a `<pre>` block, so
leading blanks are stripped by the browser. Where zero suppression would produce a leading
blank, the doc shows the value flush left. I have annotated the affected cells.

| Edit Mask | `0000.03` (N4.2) | `-0054` (N4) | `+0087` (N4) | `0962` (N4) | `1830` (N4) |
|---|---|---|---|---|---|
| `EM=9.9` | `0.0` | `4.` | `7.` | `2.` | `0.` |
| `EM=99` | `00` | `54` | `87` | `62` | `30` |
| `EM=S99` | `+00` | `-54` | `+87` | `+62` | `+30` |
| `EM=+Z9` | `+0` | `-54` | `+87` | `+62` | `+30` |
| `EM=-9.99` | `0.03` (leading blank stripped) | `-4.` | `7.` (leading blank stripped) | `2.` (stripped) | `0.` (stripped) |
| `EM=N9` | `0` | `-4` | `7` (stripped) | `2` (stripped) | `0` (stripped) |
| `EM=*9.99` | `0.03` | `4.` | `7.` | `2.` | `0.` |
| `EM=Z99` | `00` (leading blank stripped) | `54` (stripped) | `87` (stripped) | `962` | `830` |
| `EM=*EURZZ9.9` | `EUR**0.0` | `EUR*54.` | `EUR*87.` | `EUR962.` | `EUR830.` |
| `EM=999+` | `000+` | `054-` | `087+` | `962+` | `830+` |
| `EM=999-` | `000` | `054-` | `087` | `962` | `830` |
| `IC=$ EM=ZZZ.99` | `$.03` | `$54.` | `$87.` | `$962.` | `$830.` |

Reading notes that a correct implementation must reproduce:

* `EM=99` against `0962` gives `62`, not `962`. Fewer mask digits than field digits means
  **high-order truncation**, silently.
* `EM=9.9` against `-0054` (an `N4` with no decimals) gives `4.`. The mask's decimal
  position has nothing to draw from, so the separator prints with nothing after it.
* `EM=*9.99` shows the `*` **not appearing**. The fill character only materializes when the
  mask uses `Z`, per the rule in 7.5. `EM=*EURZZ9.9` in the same table does show `*`,
  because that mask uses `Z`.
* `EM=999-` prints a trailing **space** for positive values (compare `EM=999+`, which prints
  a trailing `+`).
* `EM=S99` prints `+` for positive; `EM=N9` prints nothing for positive.

The Programming Guide carries a smaller companion table:

| Edit Mask | Abbreviation | Output A | Output B |
|---|---|---|---|
| `EM=999.99` | `EM=9(3).9(2)` | `367.32` | `005.40` |
| `EM=ZZZZZ9` | `EM=Z(5)9(1)` | `0` | `579` |
| `EM=X^XXXXX` | `EM=X(1)^X(5)` | `B LUE` | `A 19379` |
| `EM=XXX...XX` | `EM=X(3)...X(2)` | `BLU...E` | `AAB...01` |
| `EM=MM.DD.YY` | | `01.05.87` | `12.22.86` |
| `EM=HH.II.SS.T` | | `08.54.12.7` | `14.32.54.3` |

(Source: `pg/pg_output_masks.htm`, "Examples of Edit Masks". Also an HTML table, so the
five leading blanks on the `EM=ZZZZZ9` outputs are stripped.)

`EM=999.99` giving `005.40` is a clean demonstration of forced digits: the leading zeros
print because the mask uses `9`, not `Z`.

Alphanumeric mask results, these ones from a whitespace-exact `<pre>` block (example
`EMMASK1`, `#TEXT (A4)` holding `BLUE`):

```
MASK 1:     B.L.U.E
MASK 2:     B L U E
MASK 3:     B--L--U
MASK 4:     B-L-U-E-
MASK 5:     B L U E
MASK 6:     BL....UE
MASK 7:     234BLUE
```

corresponding to `EM=X.X.X.X`, `EM=X^X^X^X`, `EM=X--X--X`, `EM=X-X-X-X-X-X`,
`EM=X' 'X' 'X' 'X`, `EM=XX....XXX`, `EM=1234XXXX`.

Note `MASK 3` (`EM=X--X--X`) shows only three characters of the four, because the mask has
three `X` positions. And `MASK 6` (`EM=XX....XXX`) shows `BL....UE`, because the field runs
out before the mask's third trailing `X`.

---

## 8. Numeric edge cases

### 8.1 Where the minus sign appears

**By default: leading, in the reserved high-order sign position.**

> One extra high-order print position is reserved for a sign when printing a numeric field.
> The session parameter `SG` may be used to suppress the sign position.

(Source: `sm/display.htm`.)

> **SG - Sign Position**
> This session parameter determines whether or not a sign position is to be allocated for a
> numeric field.
>
> **ON**: A sign position will be allocated.
> **OFF**: No sign position will be allocated.
>
> **Notes**: `SG=OFF` causes numeric fields with negative values to be output without a minus
> (`-`) sign. `SG=OFF` does not prevent you from entering negative values in input fields.
>
> **Default setting**: ON
>
> **Notes**: If the `EM` (edit mode) parameter is specified, it overrides the `SG` parameter.

(Source: `parms/sp_sg.htm`.)

Restated in the Programming Guide:

> By default, `SG=ON` applies, which means that a sign position is allocated for numeric
> fields. If you specify `SG=OFF`, negative values in numeric fields will be output without a
> minus sign (`-`).

The measured proof is `#D: -.12345` from `ASGEX1S` (see 1.3): the `-` occupies the leftmost
of the field's seven positions, ahead of the decimal point.

Note the consequence of `SG=OFF`: it does not merely drop the sign **position**, it drops
the **sign itself**. A negative value becomes indistinguishable from a positive one. That
is a genuine footgun and worth a warning in any lesson that mentions `SG`.

Direct doc confirmation that the sign position is a real, countable column comes from
example `FORMAX04`, which sets `FORMAT AL=10 NL=6` and applies `LC`, `IC`, `TC`:

> As you can see in the above example, any output length you specify with the `AL` or `NL`
> parameter does not include any characters specified with the `LC`, `IC` and `TC`
> parameters: the width of the `NAME` column, for example, is 11 characters - 10 for the
> field value (`AL=10`) plus 1 leading character.
>
> **The width of the `SALARY` and `BONUS` columns is 8 characters - 6 for the field value
> (`NL=6`), plus 1 leading/inserted character, plus 1 sign position (because `SG=ON`
> applies).**

(Source: `pg/pg_output_parms.htm`. Emphasis added.)

That sentence is the clearest statement in the entire documentation set that the sign
position is an additive `+1` on the numeric print width.

### 8.2 Truncation versus rounding on assignment

**Truncation is the default. `ROUNDED` exists and is opt-in.**

> **Field Truncation and Field Rounding**
> The following rules apply to field truncation and rounding:
>
> * High-order numeric field truncation is allowed **only when the digits to be truncated are
>   leading zeros**. Digits following an expressed or implied decimal point may be truncated.
> * Trailing positions of an alphanumeric field may be truncated.
> * If the option `ROUNDED` is specified, the last position of the result will be rounded up
>   if the first truncated decimal position of the value being assigned contains a value
>   greater than or equal to 5.

(Source: `pg`, "Rules for Arithmetic Assignment". Emphasis added.)

So there are two distinct behaviors, and conflating them would be a defect:

* **Low-order (decimal) truncation is silent and always allowed.** Assigning `-0.12345` to
  an `N1.3` gives `-0.123`, no error.
* **High-order (integer) truncation is only allowed if the discarded digits are zeros.**
  Discarding a significant integer digit is an error condition, not a silent wrap.

Both are demonstrated in `ASGEX1S`:

```
ASSIGN #D = #E = -0.12345 WRITE '=' #D / '=' #E
ASSIGN ROUNDED #F = 199.999 WRITE '=' #F
```

produces

```
#D: -.12345
#E: -0.123
#F:    200
```

`#E (N1.3)` receives `-0.12345` and truncates to `-0.123`, confirming truncation is the
default. `#F (N5)` with `ROUNDED` receives `199.999` and yields `200`, confirming rounding
is opt-in.

`ROUNDED` is available on `ASSIGN`, `COMPUTE`, `ADD`, `SUBTRACT`, `MULTIPLY`, and `DIVIDE`:

> **`ROUNDED` Option**: If the keyword `ROUNDED` is used, the result will be rounded.

Also confirmed in `CPTEX1`, where `COMPUTE #C = SQRT (#B)` on `#B = 1.2200` yields
`#C: 1.1045`. The square root of 1.22 is 1.104536..., truncated rather than rounded to four
decimals (rounding would give `1.1045` as well here, so this row is consistent but not
decisive on its own; `#E` above is the decisive case).

### 8.3 Overflow of declared precision

Two distinct overflow conditions are documented.

**Arithmetic result overflow:**

> **Error Conditions in Arithmetic Operations**
> In an addition, subtraction, multiplication or division, an error can occur if the total
> number of digits (before and after the decimal point) of the result is greater than 31.
>
> In an exponentiation, an error occurs in any of the following situations: if the base is of
> packed format with precision digits (for example, `P3.2`) and an exponent greater than 16;
> if the base is of floating-point format and the result is greater than approximately
> 7 * 10^75.

(Source: `pg`, "Rules for Arithmetic Assignment".)

**Assignment overflow** is governed by the high-order truncation rule in 8.2: truncating a
non-zero leading digit is not permitted, so assigning a value too large for the target
field is an error rather than a silent wrap or a field of asterisks.

**UNVERIFIED:** I did **not** confirm the specific runtime error number or message text
that Natural issues for either condition. The Messages and Codes manual was not consulted
in this spike. If a lesson intends to show an overflow error, the exact message must be
looked up before publishing, per the course's content-accuracy rule.

Given the project's "errors are teaching surfaces" requirement, my recommendation is that
the interpreter raise its own clearly-worded diagnostic that names the Natural concept
("the value 1000 does not fit in #TOTAL, which is defined as N3") rather than attempting to
reproduce a Natural error number that has not been verified.

---

## TEST TABLE

Rows are transcribable directly into interpreter unit tests. `█` denotes a single blank
character, used so that trailing and leading blanks are unambiguous. Assume default session
parameters throughout: `SG=ON`, `ZP=ON`, `SF=1`, no `AL`, no `NL`, no `EM`.

### A. Field print widths

| # | Format | Stored value | Exact expected output | Width | Status |
|---|---|---|---|---|---|
| A1 | `A20` | `'Hello'` | `Hello███████████████` | 20 | **VERIFIED** (WRITEX01, measured) |
| A2 | `A3` | `'UVW'` | `UVW` | 3 | **VERIFIED** (ASGEX1S `#H`, measured) |
| A3 | `A3` | `' '` (empty) | `███` | 3 | **VERIFIED** (ASGEX1S `#H(2)`, measured) |
| A4 | `A6` | `'ABC'` | `ABC███` | 6 | **VERIFIED** (ASGEX1S `#B`, measured) |
| A5 | `N3` | `5` | `███5` | 4 | **VERIFIED** (ASGEX1S `#A`, measured) |
| A6 | `N5` | `200` | `███200` | 6 | **VERIFIED** (ASGEX1S `#F`, measured) |
| A7 | `N0.3` | `.45` | `█.450` | 5 | **VERIFIED** (ASGEX1S `#C`, measured) |
| A8 | `N0.5` | `-0.12345` | `-.12345` | 7 | **VERIFIED** (ASGEX1S `#D`, measured) |
| A9 | `N1.3` | `-0.123` | `-0.123` | 6 | **VERIFIED** (ASGEX1S `#E`, measured) |
| A10 | `N3.4` | `1.22` | `███1.2200` | 9 | **VERIFIED** (CPTEX1 `#B`, measured) |
| A11 | `N7.2` | `19.99` | `██████19.99` | 11 | **DERIVED** from A7 to A10 width rule; width itself VERIFIED by A14 |
| A12 | `N3` | `0` | `███0` | 4 | **VERIFIED** (REIEX3 `#D`, measured; INPUT rendering) |
| A13 | `N7.2` | `0` | `███████0.00` | 11 | **VERIFIED** (REIEX3 `#B`, measured; INPUT rendering) |
| A14 | `N7.2` | any | width 11 | 11 | **VERIFIED** (REIEX3, measured) |
| A15 | `P4` | `7` | `████7` | 5 | **VERIFIED** (CPTEX1 `#A`, measured) |
| A16 | `P9` | `46000` | `█████46000` | 10 | **VERIFIED** (WRITEX01/WRTEX3, measured) |
| A17 | `P10` | `66300` | `██████66300` | 11 | **VERIFIED** (CPTEX1 `#CUM-SALARY`, measured) |
| A18 | `P7.2` | `19.99` | `██████19.99` | 11 | **DERIVED**; the P-prints-as-N rule is VERIFIED verbatim |
| A19 | `I1` | `1` | `███1` | 4 | **VERIFIED** (EMLOGV `#INDEX`, measured) |
| A20 | `I2` | `42` | `████42` | 6 | **DERIVED** (digit table + `Z9` + `SG=ON`); no doc example found |
| A21 | `I4` | `42` | `█████████42` | 11 | **DERIVED** (digit table + `Z9` + `SG=ON`); no doc example found |
| A22 | `L` | TRUE | `X` | 1 | **DERIVED** from Default Edit Masks `L -> blank / X`; no worked example |
| A23 | `L` | FALSE | `█` | 1 | **DERIVED**, same basis as A22 |
| A24 | `P9` | `0` | `█████████0` | 10 | **VERIFIED** (FORMAX03 `BONUS`, measured) |

### B. Composition and separation

| # | Statement | Exact expected output | Status |
|---|---|---|---|
| B1 | `WRITE 'Hello'` | `Hello` | **VERIFIED** (WRTEX1, measured) |
| B2 | `WRITE 'CITY:   ' #CITY` where `#CITY (A20) = 'MADRID'` | `CITY:████MADRID██████████████` | **VERIFIED** (WRTEX1, measured; 3 literal blanks + 1 separator) |
| B3 | `WRITE #A #B` where `#A (A20)='JONES'`, `#B (A20)='VIRGINIA'` | `JONES████████████████VIRGINIA████████████` (20 + sep + 20 = 41) | **VERIFIED** (WRITEX01, measured) |
| B4 | `WRITE 'CUMULATIVE SALARY:' #S` where `#S (P10) = 66300` | `CUMULATIVE SALARY:███████66300` | **VERIFIED** (CPTEX1, measured; 1 separator + 6 in-field blanks) |
| B5 | inter-element gap, `WRITE` | exactly 1 blank, not configurable | **VERIFIED BY MEASUREMENT** across 5 examples; not stated in prose anywhere |
| B6 | inter-column gap, `DISPLAY` | exactly 1 blank by default, `SF` configurable 1 to 30 | **VERIFIED** (quoted prose + `parms/sf.htm`) |
| B7 | `WRITE #A 5X #B` | `nX` replaces the default gap; 5 blanks total, not 6 | **VERIFIED** (DISPLX04, measured) |
| B8 | `WRITE '=' #A` where `#A (N3) = 5` | `#A:████5` (heading 3 + sep 1 + field 4 = 8) | **VERIFIED** (ASGEX1S, measured) |
| B9 | `WRITE #H (1:3)` where `#H (A3/1:3)` = `UVW`, empty, `XYZ` | `UVW█████XYZ` (3 + sep + 3 blanks + sep + 3 = 11) | **VERIFIED** (ASGEX1S, measured) |

### C. DISPLAY structure

| # | Aspect | Expected | Status |
|---|---|---|---|
| C1 | Column width | `max(field print width, header width)` | **VERIFIED** (quoted; DISPLX01 measured) |
| C2 | Header when field is wider | centered over the column | **VERIFIED** (quoted; WRTEX3 measured) |
| C3 | Field when header is wider | left-justified under the header | **VERIFIED** (quoted; DISPLX01 measured) |
| C4 | Underline character | `-` (hyphen) | **VERIFIED** (`parms/sp_uc.htm`) |
| C5 | Underline extent | full column width; gaps not underlined | **VERIFIED** (WRTEX3, measured: 20 + gap + 20 + gap + 10) |
| C6 | After underline | exactly one blank line before data | **VERIFIED** (quoted; measured in WRTEX3, DISPLX01) |
| C7 | Header for a user-defined variable with no explicit header | the variable name | **VERIFIED** (quoted) |
| C8 | Alphanumeric value justification | left | **VERIFIED** (quoted; measured) |
| C9 | Numeric value justification | right | **VERIFIED** (quoted; measured) |
| C10 | `WRITE` headers | none, ever | **VERIFIED** (quoted twice) |

### D. Edit masks

| # | Mask | Input value and format | Exact expected output | Status |
|---|---|---|---|---|
| D1 | `EM=X.X.X.X` | `A4` `BLUE` | `B.L.U.E` | **VERIFIED** (EMMASK1 `<pre>`) |
| D2 | `EM=X^X^X^X` | `A4` `BLUE` | `B L U E` | **VERIFIED** (EMMASK1 `<pre>`) |
| D3 | `EM=X--X--X` | `A4` `BLUE` | `B--L--U` | **VERIFIED** (EMMASK1 `<pre>`) |
| D4 | `EM=X-X-X-X-X-X` | `A4` `BLUE` | `B-L-U-E-` | **VERIFIED** (EMMASK1 `<pre>`) |
| D5 | `EM=XX....XXX` | `A4` `BLUE` | `BL....UE` | **VERIFIED** (EMMASK1 `<pre>`) |
| D6 | `EM=1234XXXX` | `A4` `BLUE` | `234BLUE` | **VERIFIED** (EMMASK1 `<pre>`) |
| D7 | `EM=*A:XXXX:` | `A4` `'  34'` | `A:**34:` | **VERIFIED** (`<pre>`) |
| D8 | `EM=X.X.X.X.X` | `A12` `JOHNSON` | `J.O.H.N.S` | **VERIFIED** (doc table) |
| D9 | `EM=****XXXXXX****` | `A12` `JOHNSON` | `****JOHNSO**` | **VERIFIED** (doc table) |
| D10 | `EM=999.99` | `367.32` | `367.32` | **VERIFIED** (doc table) |
| D11 | `EM=999.99` | `5.40` | `005.40` | **VERIFIED** (doc table; forced digits print leading zeros) |
| D12 | `EM=ZZZZZ9` | `0` | `█████0` | **VERIFIED** as `0`; the 5 leading blanks are DERIVED (doc table strips them) |
| D13 | `EM=ZZZZZ9` | `579` | `███579` | **VERIFIED** as `579`; leading blanks DERIVED |
| D14 | `EM=99` | `0962` (N4) | `62` | **VERIFIED** (doc table; high-order truncation) |
| D15 | `EM=S99` | `-0054` (N4) | `-54` | **VERIFIED** (doc table) |
| D16 | `EM=S99` | `+0087` (N4) | `+87` | **VERIFIED** (doc table) |
| D17 | `EM=N9` | `-0054` (N4) | `-4` | **VERIFIED** (doc table) |
| D18 | `EM=999+` | `-0054` (N4) | `054-` | **VERIFIED** (doc table) |
| D19 | `EM=999+` | `0962` (N4) | `962+` | **VERIFIED** (doc table) |
| D20 | `EM=999-` | `0962` (N4) | `962█` | **VERIFIED** as `962`; the trailing blank is quoted in prose ("will produce a trailing space") |
| D21 | `EM=*EURZZ9.9` | `0000.03` (N4.2) | `EUR**0.0` | **VERIFIED** (doc table) |
| D22 | `EM=*EURZZ9.9` | `0962` (N4) | `EUR962.` | **VERIFIED** (doc table) |
| D23 | `EM=*9.99` | `0000.03` (N4.2) | `0.03` | **VERIFIED** (doc table; `*` absent because mask has no `Z`) |
| D24 | `EM=ZZ,ZZZ,ZZ9.99` | `1234567.89`, `DC='.'`, `THSEPCH=','`, `THSEP=ON` | `1,234,567.89` | **VERIFIED** (doc table) |
| D25 | `EM=ZZZ,ZZ9.99` | small value leaving comma in suppressed region | **UNVERIFIED** | see 7.7; do not build fixtures on this |
| D26 | `EM=YYYY-MM-DD` | date 1970-01-01 | `1970-01-01` | **VERIFIED** (WRTEX4 `<pre>`) |
| D27 | `EM=MM/DD/YYYY` | date 2005-01-12 | `01/12/2005` | **VERIFIED** (EMDATI `<pre>`) |
| D28 | `EM=MM.DD.YY` | date 1987-01-05 | `01.05.87` | **VERIFIED** (doc table) |
| D29 | `EM=DD/MM/YY` | date 1987-01-05 | `05/01/87` | **DERIVED** (high confidence; component semantics quoted, exact string not shown) |
| D30 | `EM=HH.II.SS.T` | time 16:04:14.8 | `16.04.14.8` | **VERIFIED** (EMDATI `<pre>`) |
| D31 | `EM=HH.II.SS' 'AP` | time 16:04:14 | `04.04.14 PM` | **VERIFIED** (EMDATI `<pre>`) |
| D32 | default mask, format T | time 16:04:14 | `16:04:14` | **VERIFIED** (EMDATI `<pre>` + Default Edit Masks table) |
| D33 | `EM=FALSE/TRUE` | `L` TRUE | `TRUE█` (5 positions, sized by `FALSE`) | **VERIFIED** (EMLOGV, measured) |
| D34 | `EM=OFF/ON` | `L` TRUE | `ON█` (3 positions, sized by `OFF`) | **VERIFIED** (EMLOGV, measured) |
| D35 | `EM=HH` | `A2` `AB` (ASCII) | `4142` | **VERIFIED** (EMMASK2 `<pre>`) |
| D36 | `EM=H-H` | `A2` `AB` (ASCII) | `41-42` | **VERIFIED** (EMMASK2 `<pre>`) |
| D37 | `EM=X-X-X` | `A4` `BLUE` | `B-L-U` | **VERIFIED** (quoted inline comment) |
| D38 | `EM=X-X-X-X-X` | `A4` `BLUE` | `B-L-U-E-` | **VERIFIED** (quoted inline comment) |
| D39 | mask present | any numeric | sign position **not** added | **VERIFIED** (quoted: "An edit mask overrides any settings for the session parameters AL, NL and SG") |

### E. Arithmetic and assignment

| # | Statement | Result | Status |
|---|---|---|---|
| E1 | `#E (N1.3) := -0.12345` | `-0.123` (truncated) | **VERIFIED** (ASGEX1S, measured) |
| E2 | `ASSIGN ROUNDED #F (N5) = 199.999` | `200` | **VERIFIED** (ASGEX1S, measured) |
| E3 | default assignment behavior | truncation, not rounding | **VERIFIED** (quoted + E1) |
| E4 | `ROUNDED` rule | rounds up when the first truncated decimal is >= 5 | **VERIFIED** (quoted) |
| E5 | high-order truncation with non-zero digits | not permitted; error | **VERIFIED** (quoted); **exact error number UNVERIFIED** |
| E6 | arithmetic result > 31 total digits | error | **VERIFIED** (quoted); exact error number UNVERIFIED |

### F. Statement-level behavior

| # | Aspect | Expected | Status |
|---|---|---|---|
| F1 | `WRITE` line overflow | element moved whole to the next line; never split | **VERIFIED** (quoted) |
| F2 | `DISPLAY` line overflow | error message issued | **VERIFIED** (quoted); exact message UNVERIFIED |
| F3 | default `LS` | 132 in batch, terminal width online; no single fixed value | **VERIFIED** (quoted) |
| F4 | quote escaping | double the apostrophe: `'HE SAID, ''HELLO'''` -> `HE SAID, 'HELLO'` | **VERIFIED** (quoted) |
| F5 | double-quoted literal | quotation marks translated to apostrophes, subject to `TQMARK` | **VERIFIED** (quoted) |
| F6 | bare `WRITE`, no operands | **UNVERIFIED** | syntax diagram suggests at least one element is required; not confirmed |
| F7 | blank line, documented idiom | `SKIP n` or `WRITE /` | **VERIFIED** (quoted) |
| F8 | default page title | `Page` col 1-4, page number right-justified to col 11, date and time flush right | **VERIFIED** (measured across 8 examples at LS 79) |
| F9 | `DISPLAY` on a terminal screen | begins in physical column 2 | **VERIFIED** (quoted) |
| F10 | `DISPLAY` printed on paper | begins in column 1 | **VERIFIED** (quoted) |

---

## Open questions and explicit non-verifications

These are the items I could not ground in the official documentation. Each one is a place
where inventing a rule would put a defect into every fixture that touches it.

1. **Bare `WRITE` with no operands (F6).** Not documented. The syntax diagram's bracket
   images suggest at least one text or operand element is mandatory, but the "Syntax
   Symbols" reference page returns 404 throughout the 9.3.3 tree, so the bracket convention
   itself is unconfirmed. Recommend rejecting it with a teaching diagnostic, and teaching
   `SKIP 1` and `WRITE /` instead.

2. **Logical field output with no edit mask (A22, A23).** The default mask string
   `blank / X` is quoted verbatim from the Default Edit Masks table, and the logical
   edit-mask syntax `(EM=[false-string/]true-string)` is quoted verbatim. The resulting
   1-position `X`/blank output follows from combining them, but no worked example in the
   Statements reference, Programming Guide, or Parameter Reference prints an unmasked
   logical. Recommend teaching logical output with an explicit `EM=` in every lesson.

3. **Insertion characters inside a zero-suppressed region (D25).** Specifically, whether
   the comma in `EM=ZZZ,ZZ9.99` is suppressed, printed, or replaced by the fill character
   when the digits to its left are all suppressed. The only documented thousands-separator
   example uses a value that fills every position. Recommend avoiding this case in v1
   fixtures.

4. **Exact runtime error identifiers** for numeric overflow (E5, E6) and `DISPLAY` line
   overflow (F2). The Messages and Codes manual was not consulted. Given the project's
   "errors are teaching surfaces" requirement, the interpreter should emit its own
   concept-naming diagnostic rather than a guessed Natural error number.

5. **`I2` and `I4` print widths (A20, A21).** The digit counts (5 and 10) are quoted
   verbatim, and the `+1` sign rule is quoted verbatim and measured for `I1`, `N`, and `P`.
   But no documented example prints an `I2` or `I4` field, so the composed widths of 6 and
   11 are derived rather than observed. Confidence is high; the derivation is the same one
   that measurement confirms for five other formats.

6. **`N7.2` and `P7.2` exact strings (A11, A18).** The **width** of 11 is verified by
   measurement (REIEX3). The specific rendering of `19.99` within that width follows from
   the verified `Z9` default mask plus the verified justification rule, but `19.99` itself
   is not a documented value. Confidence is high.

7. **`REIEX3` is an `INPUT` rendering, not a `WRITE`** (A12, A13, A14). It is the only place
   in the docs where a decimal field holding zero is shown with exact spacing. Its widths
   agree exactly with every `WRITE` measurement, so I treat it as strong corroboration, but
   a purist would want a `WRITE` example.

8. **`SF` specification level inconsistency.** `sm/display.htm` marks `SF` as `SE`
   (statement and element level); `pg/pg_output_display.htm` says statement level only. Out
   of scope for Tier 1, but do not implement element-level `SF` without testing.

---

## Sources

All URLs are under the Natural for Windows 9.3.3 webhelp root:
`https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/`

All accessed **2026-07-31**.

| URL (relative to the root above) | Substantiates |
|---|---|
| `sm/write.htm` | WRITE usage and the three differences from DISPLAY; "The length of the data determines the number of positions printed for each field"; NOTITLE and NOHDR semantics; the full WRITE parameter list (confirming `SF` is absent); `nX`, `nT`, `x/y`, `T*`, `P*`, `/` and `'='` notations; text and character-repetition assignment; examples WRTEX1 through WRTEX5 with exact spacing |
| `sm/display.htm` | DISPLAY column format and the pointer to WRITE/PRINT for free format; column-header priority rules; "Natural always underlines column headings and generates one blank line"; "Defaults Applicable for a DISPLAY Statement" including report width, terminal column 2 versus paper column 1, spacing factor, field output and justification, and the high-order sign position; "Line size overflow is not permitted for output resulting from a DISPLAY statement" |
| `sm/compute.htm` | Examples ASGEX1S and CPTEX1 with exact spacing: N and P print widths, leading sign position, decimal rendering, trailing-decimal forcing, default truncation, `ASSIGN ROUNDED` |
| `sm/reinput.htm` | Example REIEX3 with exact spacing: `N7.2` holding zero renders as `0.00` in an 11-position field; `N3` holding zero renders in 4 positions |
| `pg/pg_output_display.htm` | "Statements DISPLAY and WRITE" chapter; WRITE "does not produce any headers"; "By default, the columns output with a DISPLAY statement are separated from one another by one space"; `nX` overrides `SF`; examples DISPLX01 to DISPLX05 and WRITEX01 with exact spacing |
| `pg/pg_output_parms.htm` | "Parameters to Influence the Output of Fields"; default left/right justification by format; `AL`/`NL` semantics; `SG` default ON and its effect; the "plus 1 sign position (because SG=ON applies)" width breakdown; `IS`, `ZP`, `ES` defaults; examples FORMAX03 to FORMAX08 |
| `pg/pg_output_masks.htm` | "Code Page Edit Masks - EM Parameter"; EM specification levels; numeric masks must contain at least one `9` or `Z`; alphanumeric masks need an `X` per character; `^` for blanks; field-length versus mask-length rules; the `DC`/`THSEP`/`THSEPCH` separator system and the `EM=ZZ,ZZZ,ZZ9.99` output table; the examples-of-edit-masks table |
| `pg/pg_output_headers.htm` | "By default, column headers are centered above the columns"; `HC=L`/`HC=R`; "By default, titles and headers are underlined with a hyphen (-)"; `FC`/`GC`/`HW` |
| `pg/pg_output_layout.htm` | Layout of an output page; `WRITE TITLE`/`WRITE TRAILER`/`AT TOP OF PAGE`/`AT END OF PAGE` ordering; example OUTPUX01 |
| `parms/sp_em.htm` | The definitive EM reference: **Default Edit Masks table** (`A -> X`, `B -> H`, `N,P,I -> Z9`, `T -> HH:II:SS`, `L -> blank / X`); "An edit mask overrides any settings for the session parameters AL, NL and SG"; numeric mask character definitions for `9`, `.`, `Z`; sign characters `+`, `-`, `S`, `N`; literal leading/insertion/trailing character rules including the fill-character rule; trailing sign characters; the numeric edit-mask results table; alphanumeric mask rules; hexadecimal `H` masks; date and time character tables; logical edit-mask syntax; examples EMMASK1, EMMASK2, EMDATI, EMLOGV |
| `parms/sp_al.htm` | `AL` semantics; **Default setting: none**; edit mask overrides AL |
| `parms/sp_nl.htm` | `NL` semantics and the `nn.m` notation; **Default setting: none**; edit mask overrides NL |
| `parms/sp_sg.htm` | `SG` ON/OFF semantics; **Default setting: ON**; `SG=OFF` drops the minus sign itself; EM overrides SG |
| `parms/zp.htm` | `ZP` ON/OFF semantics; **Default setting: ON**; "output as one zero, right justified" |
| `parms/sf.htm` | `SF` applies to `DISPLAY` statements; range 1 to 30; cannot be 0; **Default setting: 1** |
| `parms/ls.htm` | `LS` profile versus session parameter; batch versus online; ranges; default is the physical line size |
| `parms/ps.htm` | `PS` page size; **Default setting: 0** (physical page size) |
| `parms/sp_uc.htm` | `UC` underlining character; **Default setting: `-` hyphen** |
| `parms/sp_df.htm` | `DF` date format S/I/L; **Default setting: S** (`yy-mm-dd`); evaluated at compilation time; sequence and delimiters governed by `DTFORM` |
| `pdf/pg.pdf` (Programming Guide, 1,062 pp.) | Format/length table and the `nn.mm` decimal notation; the annotated `#A4 (N7.2)` example; "When a user-defined variable of format P is output with a DISPLAY, WRITE, or INPUT statement, Natural internally converts the format to N for the output"; Format L is 1 byte; the I1/I2/I4 decimal integer length table; "Apostrophes Within Alphanumeric Constants" and the `TQMARK` note; "Field Truncation and Field Rounding"; "Error Conditions in Arithmetic Operations"; data conversion rules 1 to 16 |
| `pdf/sm.pdf` (Statements, 1,134 pp.) | `SKIP` statement blank-line generation; `ROUNDED` option on ADD/SUBTRACT/MULTIPLY/DIVIDE/COMPUTE/ASSIGN; cross-statement location of all the above |
| `pdf/parms.pdf` (Parameter Reference, 518 pp.) | Cross-checking of the parameter pages listed above |
| `https://techcommunity.softwareag.com/t/user-defined-variable-format-l-logical/61551` | Consulted for the logical-field default output question; contains only internal binary representation, no output-form information. Recorded here so the negative result is not re-searched. |
