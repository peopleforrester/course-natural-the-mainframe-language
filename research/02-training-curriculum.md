<!-- ABOUTME: Research spike on the Natural language technical content and the realistic
     ABOUTME: beginner-to-competent training path, grounding the course curriculum design. -->

# Spike 02: Natural Technical Content and Training Path

Spike date: 2026-07-19

## Executive summary

Natural is Software AG's fourth-generation language (4GL) for the IBM mainframe,
designed from the start to read and write ADABAS records with high-level verbs
instead of hand-coded file access. This spike documents the language model, the
core syntax, the map-based screen model, the edit/compile/run workflow, and, most
importantly for the course, the realistic order in which a newcomer should learn
these pieces.

The headline findings for curriculum design:

1. Natural is unusually approachable for a mainframe language. A runnable program
   is three lines (`DISPLAY "Hello world!"` + `END`), and database access is a
   `READ` or `FIND` loop over a named view. A learner writes useful,
   database-touching code far earlier than they would in COBOL plus JCL plus a
   separate data layer.
2. The single most important structural concept is the `DEFINE DATA` block and the
   idea of data areas (LDA, GDA, PDA). Everything else hangs off understanding how
   data is declared, typed, and passed between objects.
3. A learner does NOT need deep TSO/ISPF or JCL knowledge to start, especially if
   the course uses NaturalONE (Eclipse) or the Community Edition container rather
   than a raw 3270 session. This is decisive for a browser-based course: the hard
   mainframe-operations prerequisites can be deferred or omitted.
4. Software AG runs an official learning portal with a free foundational course
   ("Natural Programming Basic") that grants a "Software AG Certified Natural
   Associate" badge on course completion (no proctored exam). That gives the course
   a credible external certification to point students toward.

Everything below that shows syntax is quoted from or verified against the official
Software AG Natural documentation. Where I present a construct that I did not fetch
a verbatim example for, I flag it explicitly.

---

## 1. The language model

### 1.1 Program structure

A Natural source object is a sequence of statements. In "structured mode" (the
modern default and what NaturalONE produces), a program that declares any data
must begin with a `DEFINE DATA` block, and blocks are closed with explicit
`END-*` keywords (`END-DEFINE`, `END-READ`, `END-IF`, `END-DECIDE`). The object
ends with `END`. Lines beginning with `*` are comments, and `/*` starts an
end-of-line comment.

The minimal complete program, quoted verbatim from the official First Steps
tutorial:

```natural
* The "Hello world!" example in Natural.
*
DISPLAY "Hello world!"
END /* End of program
```

Natural historically had two modes: "reporting mode" (older, looser, no mandatory
`DEFINE DATA`, implicit loops) and "structured mode" (block-oriented, explicit
`END-*` closers). Course material should teach structured mode only; reporting
mode is legacy and should be mentioned but not taught.

### 1.2 Object types

Natural is organized into discrete cataloged objects, each with a type. The main
object types, per the NaturalONE Programming Guide:

| Object type | What it is | How invoked | Runs standalone? |
|---|---|---|---|
| Program | Top-level executable unit | `RUN`, `FETCH`, `FETCH RETURN`, or executed directly | Yes |
| Subprogram | Reusable routine with strict parameter interface | `CALLNAT` | No |
| Subroutine (external) | Reusable routine, shares caller's global data | `PERFORM` | No |
| Subroutine (inline) | `DEFINE SUBROUTINE ... END-SUBROUTINE` inside another object | `PERFORM` | No |
| Function | Returns a value, callable in an expression | function-call syntax `name(<...>)` | No |
| Copycode | Source text spliced in at compile time | `INCLUDE` | No (not a routine) |
| Map | 3270 screen layout | `INPUT USING MAP` / `WRITE USING MAP` | No |
| DDM (Data Definition Module) | Logical description of a database file | referenced by `VIEW OF` | No (metadata) |
| Helproutine | Field-level help screen/logic | attached to a field, triggered by help key | No |
| Class | Object-oriented class (NaturalX) | instantiated | No |
| LDA / GDA / PDA | Local / Global / Parameter data areas | referenced via `USING` | No (data only) |
| Text | Free text object | n/a | No |
| Resource | Non-Natural file resource | n/a | No |

Verified distinctions between the four routine kinds (NaturalONE Programming
Guide, "Programs, Functions, Subprograms and Subroutines"):

- A **program** can be executed and tested by itself. It can also be called by
  another object with `FETCH RETURN` (suspends the caller, returns afterward) or
  `FETCH` (terminates the caller, no return).
- A **subprogram** is invoked with `CALLNAT` and cannot be executed by itself.
  When `CALLNAT` runs, the caller is suspended and the subprogram executes; on
  completion, control returns to the statement after the `CALLNAT`. Data is passed
  only through the parameter list (or a PDA); the subprogram has its own local
  data area and does not see the caller's local data. This strict isolation makes
  subprograms the standard unit for reusable business logic.
- A **subroutine** is invoked with `PERFORM`. An external subroutine has access to
  the caller's global data area and receives parameters via `PERFORM` or a PDA; an
  inline subroutine additionally sees the local data of the object that contains
  it.
- A **function** is defined with `DEFINE FUNCTION ... RETURNS ... END-FUNCTION`
  and is called inside an expression by its name. A global data area cannot be
  referenced inside a function definition.

The practical teaching order is: program first, then inline subroutine, then
external subroutine, then subprogram, then function. That is also the order the
official tutorial uses.

### 1.3 The DEFINE DATA block

`DEFINE DATA` is the declaration section. When present it must be the first
statement of the object, and it is closed by `END-DEFINE` (verified,
NaturalONE "DEFINE DATA - General"). Its clauses:

- `LOCAL` for variables local to this object (inline, or `USING` an LDA).
- `GLOBAL` for a shared global data area (`USING` a GDA).
- `PARAMETER` for the parameters this object receives (inline, or `USING` a PDA).
- `INDEPENDENT` for application-independent variables (names begin with `+`).
- `CONTEXT` for conversation-context variables (Natural RPC).
- `OBJECT` for object (class) data.

Fields are declared with a level number (1 for top level, 2/3/... for nested
group members), a name, and a format-length in parentheses. Verified inline
example (from the Data Areas page):

```natural
DEFINE DATA LOCAL
1 VIEWEMP VIEW OF EMPLOYEES
  2 NAME
  2 FIRST-NAME
1 #VARI-A (A20)
END-DEFINE
```

User-defined variable names conventionally begin with `#` (a convention, not a
hard rule) so they are visually distinct from database fields. `INIT <...>` sets
an initial value. Verified example from the tutorial database program:

```natural
DEFINE DATA
LOCAL
  1 #NAME-START        (A20) INIT <"ADKINSON">
  1 #NAME-END          (A20) INIT <"BENNETT">
  1 EMPLOYEES-VIEW VIEW OF EMPLOYEES
    2 FULL-NAME
      3 NAME (A20)
    2 DEPT (A6)
    2 LEAVE-DATA
      3 LEAVE-DUE (N2)
END-DEFINE
```

### 1.4 Data types and formats

Natural formats, verified against the Programming Guide "User-Defined Variables"
table:

| Format | Meaning | Definable length | Notes |
|---|---|---|---|
| A | Alphanumeric | 1 to 1073741824 (1 GB) | character/byte string |
| U | Unicode (UTF-16) | 1 to 536870912 (0.5 GB) | double-byte internally |
| B | Binary | 1 to 1073741824 (1 GB) | raw bytes |
| N | Numeric, unpacked | 1 to 29 total digits | zoned decimal |
| P | Packed numeric | 1 to 29 total digits | packed decimal, stored compactly |
| I | Integer | 1, 2, or 4 bytes | `I1`, `I2`, `I4` |
| F | Floating point | 4 or 8 bytes | `F4`, `F8` |
| L | Logical | fixed (1 byte) | TRUE / FALSE |
| D | Date | fixed (4 bytes) | |
| T | Time | fixed (7 bytes) | |
| C | Attribute control | fixed (2 bytes) | dynamic screen attributes |

Format-length notation, verified:

- `(A10)` alphanumeric, 10 positions.
- `(N7.2)` numeric, 7 digits before and 2 after the decimal.
- `(P7.2)` packed, 7 before and 2 after the decimal.
- `(I4)` 4-byte integer.
- `(F8)` 8-byte floating point.
- `(D)`, `(L)` no length given (fixed).

Rule for N and P (verified): the sum of the digits before and after the decimal
must not exceed 29, and the fractional part must not exceed 7. For the course, the
formats worth drilling are A, N, P, I, L, D, and T. B, U, F, and C can be
mentioned and deferred.

### 1.5 Data areas: LDA, GDA, PDA

Data areas are separately cataloged objects that hold field definitions, so the
same layout can be reused across many programs. Verified behavior (Data Areas
page):

- **LDA (Local Data Area)**: variables scoped to one object. Referenced with
  `DEFINE DATA LOCAL USING LDA-name`. Initialized when the object that uses it
  starts to execute.
- **GDA (Global Data Area)**: shared across a run unit. A new GDA instance is
  created when a subprogram that references it is invoked via `CALLNAT`; objects
  linked by `FETCH`/`FETCH RETURN` can share the same GDA instance.
- **PDA (Parameter Data Area)**: the interface contract for a subprogram or
  external subroutine. Parameters must match in sequence, format, and length
  between caller and callee; the variable names need not match.

Referencing an LDA:

```natural
DEFINE DATA LOCAL
  USING LDA39
END-DEFINE
```

Teaching note: the LDA/GDA/PDA distinction is the conceptual spine of modular
Natural. It maps cleanly onto "private state / shared state / call interface" and
should be taught right after the learner has felt the pain of duplicating a
`DEFINE DATA` block across two programs.

---

## 2. Core syntax and constructs

### 2.1 Assignment and computation

- Assignment operator `:=` (verified, seen in the STORE example below):
  `EMPL-VIEW.NAME := #NAME`.
- `MOVE source TO target` (verified in the FIND example:
  `MOVE '*** NO CAR ***' TO MAKE`).
- `COMPUTE target = arithmetic-expression` for arithmetic assignment. This is a
  standard core Natural verb documented in the Programming Guide ("Rules for
  Arithmetic Assignment"); I did not fetch a verbatim `COMPUTE` example in this
  spike, so treat the exact example as to-be-confirmed while the verb itself is
  confirmed to exist.

### 2.2 Conditional logic

`IF condition ... [ELSE ...] END-IF` is the basic conditional (verified to exist,
IF statement page). Multi-way branching uses two distinct constructs, both
verified with verbatim examples:

`DECIDE ON` branches on the value of a single field. Verified example:

```natural
** Example 'DECEX4': DECIDE ON (with EVERY option)
DEFINE DATA LOCAL
1 #FIELD (N1)
END-DEFINE

INPUT 'Enter any value between 1 and 9:' #FIELD (SG=OFF)

DECIDE ON EVERY VALUE OF #FIELD
  VALUE 1 : 4
    WRITE 'Content of #FIELD is 1-4'
  VALUE 2 : 5
    WRITE 'Content of #FIELD is 2-5'
  ANY VALUE
    WRITE 'Content of #FIELD is 1-5'
  ALL VALUE
    WRITE 'Content of #FIELD is 2-4'
  NONE VALUE
    WRITE 'Content of #FIELD is not 1-5'
END-DECIDE

END
```

`FIRST` evaluates only the first matching branch; `EVERY` evaluates all matching
branches. `VALUE a : b` is an inclusive range.

`DECIDE FOR` branches on independent boolean conditions rather than one field's
value. Verified example:

```natural
DEFINE DATA LOCAL
1 #FIELD1 (N5.4)
END-DEFINE
*
INPUT #FIELD1
*
DECIDE FOR EVERY CONDITION
  WHEN #FIELD1 >= 0
    WRITE '#FIELD1 is positive or zero.'
  WHEN #FIELD1 <= 0
    WRITE '#FIELD1 is negative or zero.'
  WHEN FRAC(#FIELD1) = 0
    WRITE '#FIELD1 has no decimal digits.'
  WHEN ANY
    WRITE 'Any of the above conditions is true.'
  WHEN ALL
    WRITE '#FIELD1 is zero.'
  WHEN NONE
    IGNORE
END-DECIDE
*
END
```

### 2.3 Loops and flow control

- `FOR counter = start TO end ... END-FOR` counts through a range.
- `REPEAT ... UNTIL condition` / `REPEAT ... WHILE condition ... END-REPEAT` is the
  general loop; `ESCAPE BOTTOM` / `ESCAPE TOP` leave or restart it.
- Database access verbs (`READ`, `FIND`, `HISTOGRAM`) are themselves loops, each
  closed by its own `END-READ` / `END-FIND` / `END-HISTOGRAM`.
- `ESCAPE` exits a processing loop or routine. `ESCAPE ROUTINE` terminates the
  current subroutine/subprogram early; the ESCAPE statement is confirmed in the
  documentation index. I did not fetch a verbatim `FOR`/`REPEAT` example in this
  spike, so treat those two skeletons as standard-but-confirm-verbatim, while the
  verbs themselves are documented Natural statements.

### 2.4 Calling other objects

- `PERFORM subroutine-name` invokes an internal or external subroutine (verified).
- `CALLNAT 'subprogram-name' parm1 parm2 ...` invokes a subprogram with a strict
  parameter list (verified behavior).
- `FETCH` / `FETCH RETURN 'program-name'` transfers to another program (verified).
- `INCLUDE copycode-name` splices copycode at compile time (verified concept).

### 2.5 Output and interaction

- `DISPLAY` produces columnar output with automatic column headers derived from
  field names (verified in Hello World and the database example). `3X` inserts
  three spaces between columns; `NOTITLE` suppresses the default page title.
- `WRITE` produces free-format output line by line (verified in DECIDE examples).
- `INPUT` reads user input, either as a simple prompt or through a map (see
  section 3).

### 2.6 Database access verbs

Natural's data manipulation language is built into the language. Verified against
the Programming Guide "Natural and Database Access": the same verbs (`FIND`,
`READ`, `HISTOGRAM`, `STORE`, `UPDATE`, `DELETE`, and the transaction verbs) work
across ADABAS, Db2, VSAM, and DL/I; a DDM hides the physical database structure
and a `VIEW OF` selects fields from it.

| Verb | Purpose |
|---|---|
| `READ` | Read records in logical (descriptor) or physical sequence, optionally within a value range |
| `FIND` | Retrieve the set of records matching a search criterion (`WITH`), optionally filtered (`WHERE`) and ordered (`SORTED BY`) |
| `HISTOGRAM` | Read the distinct values of a single descriptor and their record counts, without reading the records themselves |
| `GET` | Read a specific record directly by its internal sequence number (ISN) |
| `STORE` | Insert a new record |
| `UPDATE` | Modify the record currently held by a read/find loop |
| `DELETE` | Remove the record currently held |
| `END TRANSACTION` | Commit the current logical transaction |
| `BACKOUT TRANSACTION` | Roll back the current logical transaction |

Verified `READ` loop (First Steps tutorial):

```natural
READ EMPLOYEES-VIEW BY NAME
  STARTING FROM #NAME-START
  ENDING AT #NAME-END
*
  DISPLAY NAME 3X DEPT 3X LEAVE-DUE
*
END-READ
```

Verified `FIND` loops (FIND statement reference):

```natural
FIND EMPLOY-VIEW WITH CITY = 'FRANKFURT'
                SORTED BY NAME PERSONNEL-ID
  DISPLAY NOTITLE NAME (IS=ON) FIRST-NAME PERSONNEL-ID
END-FIND
```

```natural
FIND EMPLOY-VIEW WITH CITY = 'PARIS'
                WHERE JOB-TITLE = 'INGENIEUR COMMERCIAL'
  DISPLAY NOTITLE CITY JOB-TITLE PERSONNEL-ID NAME
END-FIND
```

```natural
FIND VEHIC-VIEW WITH PERSONNEL-ID = PERSONNEL-ID (EMP.)
  IF NO RECORDS FOUND
    MOVE '*** NO CAR ***' TO MAKE
  END-NOREC
  DISPLAY NOTITLE NAME FIRST-NAME MAKE
END-FIND
```

Verified field-assignment pattern for a write, followed by a commit (STORE
statement reference; note the reference page renders the commit as
`END OF TRANSACTION`, which is the verb `END TRANSACTION`):

```natural
EMPL-VIEW.PERSONNEL-ID := #PERSONNEL-ID
EMPL-VIEW.NAME         := #NAME
EMPL-VIEW.FIRST-NAME   := #FIRST-NAME
EMPL-VIEW.MAR-STAT     := #MAR-STAT
EMPL-VIEW.BIRTH        := #BIRTH-D
EMPL-VIEW.CITY         := #CITY
EMPL-VIEW.COUNTRY      := #COUNTRY

END TRANSACTION
```

The `(EMP.)` and `(IS=ON)` notations in the FIND examples are, respectively, a
reference label for a specific processing loop and a display attribute
(inter-row suppression of repeated values). Both are real Natural notation and
worth a short explanation when the course reaches nested loops.

---

## 3. Maps and screens (3270 screen I/O)

Natural does interactive screen I/O through **maps**: cataloged objects of type
Map that define a screen layout of text fields (literal labels) and data fields
(bound to variables). Verified behavior (Map object page and INPUT reference):

- A map is created in the map editor and referenced from a program with
  `INPUT USING MAP 'map-name'` for input screens or `WRITE USING MAP 'map-name'`
  for output screens.
- `INPUT` can also be used inline with a field list and prompt literals, without a
  separate map object. The simplest form is `INPUT 'prompt' #FIELD`, seen in the
  DECIDE example above.
- On a 3270-type buffered terminal, all data for one `INPUT` is typed on the
  screen and sent together with ENTER; this is the block-mode behavior that
  distinguishes 3270 I/O from character-at-a-time terminals.

### REINPUT for validation

`REINPUT` re-displays the current map with an error message and, optionally,
positions or highlights the offending field, without clearing what the user
already typed. Verified pattern (from the map/INPUT documentation):

```natural
IF #NAME-START = ' '
  REINPUT 'Please enter a starting name.' MARK *#NAME-START
END-IF
```

`MARK` positions the cursor on the named field; the message appears on the
message line. `REINPUT` is the idiomatic validation loop in Natural: display map,
read input, check, and `REINPUT` on error so the user corrects in place. For the
course this is the natural pairing lesson right after `INPUT USING MAP`.

Teaching note: for a browser-based VTT course, maps are the one area that most
depends on a real terminal, because they are inherently a 3270 full-screen
construct. Early modules can stay on `DISPLAY`/`WRITE`/inline `INPUT`, which work
line-by-line and are easier to render, and introduce full maps once the terminal
environment is confirmed to handle full-screen mode.

---

## 4. How programs are edited and run

### 4.1 Editors

In a native mainframe (or Community Edition) session, Natural has its own editors,
invoked from the NEXT/MORE command line or the Development Functions menu:

- The **program editor** for programs, subprograms, subroutines, copycode, and
  helproutines (source is line-oriented text).
- The **data-area editor** for LDA/GDA/PDA objects (a structured grid of level,
  name, format, length).
- The **map editor** for maps (a full-screen WYSIWYG layout tool).

Creating the Hello World program in a native session, verified from the tutorial:
log in to the target library, then at the Development Functions menu enter
`Code C  Type P  Name HELLO` (Create, type Program), type the source, and press
ENTER.

### 4.2 Compiling and cataloging: STOW, CATALL

Verified command semantics (First Steps tutorial and NaturalONE FAQ):

- `RUN` (abbreviated `R`) runs a syntax check, compiles in memory, and executes
  the current source without permanently saving the compiled object.
- `STOW` runs a syntax check and, only if error-free, saves BOTH the source and
  the cataloged (compiled) object to the Natural system file. This is the "save
  and compile" step that makes an object callable by others.
- `SAVE` stores only the source, no compiled object.
- `CATALOG` compiles and stores the object module only (no source).
- `CATALL` is the system command that (re)catalogs many objects in a library in
  one pass; NaturalONE's builder performs an equivalent automatic recatalog of
  dependent objects. Cataloging is required before an object can be called at run
  time.

### 4.3 Native environment vs SPoD vs NaturalONE

Three ways a developer reaches Natural, verified against the SPoD and NaturalONE
documentation:

- **Native 3270 session**: log directly into Natural on the mainframe through a
  terminal emulator, using the built-in editors and system commands. This is the
  traditional path and the one that most depends on TSO/ISPF familiarity.
- **SPoD (Single Point of Development) with Natural Studio**: Natural Studio is a
  Windows GUI client that connects to Natural servers on the mainframe (or Linux)
  and edits objects directly on the server. Changes are stored straight onto the
  Natural server.
- **NaturalONE (Eclipse)**: an Eclipse-based IDE. The key architectural
  difference from Natural Studio is that NaturalONE is Eclipse-based and works
  against a local copy of objects in an Eclipse project, then deploys/synchronizes
  to the Natural server. Because both NaturalONE and Natural Studio use the SPoD
  concept, developers using either can work against the same Natural server in
  parallel; Software AG recommends standardizing on one environment per FUSER.

For a modern learner and for a browser-based course, NaturalONE or a
container-based Natural (the Adabas & Natural Community Edition, see spike 05) is
the friendlier target than a raw 3270 session, because it removes most of the
mainframe-operations overhead.

### 4.4 Libraries and system files: FUSER and FNAT

Verified (NaturalONE FAQ and SPoD documentation): Natural objects live in
**libraries**, and libraries live in **system files**:

- **FNAT** holds Natural system programs and utilities (the product's own code).
- **FUSER** holds user-written application libraries and objects.
- The `SYSTEM` library exists in both FNAT and FUSER and acts as the default
  "steplib" (a search path for shared objects). Objects in FNAT and in the FUSER
  `SYSTEM` library are protected from casual deletion.

A learner needs a mental model of "my code goes in a library in FUSER; I log in to
a library, create objects, STOW them, and can call other objects in my library or
in a steplib." That is enough to be productive without deep systems-programmer
knowledge of the system files.

---

## 5. The realistic beginner-to-competent learning path

### 5.1 Prerequisites: what is and is not required

The important finding for course scoping is that the hard mainframe prerequisites
are largely avoidable:

- **TSO/ISPF**: helpful for a native 3270 session but NOT required if the learner
  uses NaturalONE or a container. Natural has its own editors and command line, so
  a student never has to touch ISPF to write Natural. Recommend teaching a small
  amount of "how to log in and navigate the Natural command line" instead of a
  full TSO/ISPF unit.
- **JCL**: NOT required for interactive development and learning. JCL matters only
  when running Natural in batch on z/OS, which is an advanced/operations topic. It
  can be a late optional module, not a prerequisite.
- **The mainframe environment generally**: a learner needs to understand what a
  library, a system file (FUSER), and a cataloged object are, but does not need to
  administer any of it.
- **ADABAS**: the learner needs a conceptual model of files, fields, descriptors
  (indexed fields you can search on), and DDMs/views. They do not need to be a DBA.
  A one-module conceptual treatment is enough to make `READ`/`FIND` make sense.
- **General programming**: variables, types, conditionals, loops, and subroutines.
  A learner who has written code in any language will move quickly; a true novice
  needs the usual programming-fundamentals grounding, which the course can supply.

### 5.2 What "job-ready" means

For maintenance-and-enhancement roles on existing Natural/ADABAS systems (the bulk
of the market, see spike 03), "job-ready competent" means the developer can:

- Read and modify an existing structured-mode program with a `DEFINE DATA` block.
- Write and debug `READ`, `FIND`, and `HISTOGRAM` loops against DDM views, and
  perform `STORE`/`UPDATE`/`DELETE` with correct `END TRANSACTION` boundaries.
- Build and use LDAs, GDAs, and PDAs, and factor logic into subprograms and
  subroutines with correct parameter interfaces.
- Build map-based screens and validate input with `INPUT`/`REINPUT`.
- Use the editor, `STOW`/`CATALOG`/`CATALL`, and understand libraries and steplibs.

That is a realistic several-weeks-to-a-few-months target for someone who already
programs, and longer for a true beginner. It is deliberately scoped below
"architect a new Natural application" and below "administer ADABAS."

### 5.3 Proposed learning sequence (modules)

Ordered so each module unlocks the next, mirroring the official tutorial's arc
(Hello World, database access, user input, loops, inline subroutines, processing
rules and helproutines, LDAs, GDAs, external subroutines, subprograms):

1. **Orientation**: what Natural and ADABAS are, where Natural fits on the
   mainframe, structured vs reporting mode, the object-type map. (Cross-reference
   spike 01 for history/vendor.)
2. **First program and the editor**: Hello World, the program editor, `RUN` vs
   `STOW`, syntax checking, comments, `END`.
3. **Data and DEFINE DATA**: `DEFINE DATA LOCAL`, level numbers, the format table
   (A, N, P, I, L, D, T), `INIT`, the `#` naming convention.
4. **Assignment and computation**: `:=`, `MOVE`, `COMPUTE`, arithmetic, `RESET`.
5. **Conditional logic**: `IF/END-IF`, `DECIDE ON`, `DECIDE FOR`, `FIRST` vs
   `EVERY`.
6. **Output**: `DISPLAY` (columnar, headers, `NOTITLE`, `nX`) vs `WRITE`.
7. **Database reading**: DDMs, `VIEW OF`, `READ ... BY ... STARTING/ENDING`,
   `FIND ... WITH ... WHERE ... SORTED BY`, `IF NO RECORDS FOUND`, `HISTOGRAM`.
8. **Database writing and transactions**: `STORE`, `UPDATE`, `DELETE`, `GET`,
   `END TRANSACTION`, `BACKOUT TRANSACTION`, ISN and hold logic.
9. **Loops and control**: `FOR`, `REPEAT ... WHILE/UNTIL`, `ESCAPE`, loop labels.
10. **Screens**: maps, the map editor, `INPUT USING MAP` / `WRITE USING MAP`,
    inline `INPUT`, `REINPUT` validation, `MARK`.
11. **Modularization I**: inline subroutines (`DEFINE SUBROUTINE`/`PERFORM`),
    copycode (`INCLUDE`).
12. **Data areas**: LDA, GDA, PDA as separate objects; the data-area editor; when
    to use each.
13. **Modularization II**: external subroutines, subprograms (`CALLNAT`),
    functions (`DEFINE FUNCTION`), parameter matching, `ESCAPE ROUTINE`.
14. **Environment and workflow**: libraries, FUSER/FNAT, steplibs,
    `STOW`/`CATALOG`/`CATALL`, NaturalONE vs native vs SPoD.
15. **Capstone**: a small multi-object application (a program calling a subprogram
    that reads/updates ADABAS through a PDA and drives a validated map).
16. **Optional/advanced**: helproutines, error handling, batch and JCL, Natural
    RPC, and OO Natural (classes).

Modules 1 through 8 are the core that gets a learner writing useful,
database-touching code. Modules 9 through 14 are what turn "can write a program"
into "job-ready for maintenance work." Module 15 proves it; module 16 is beyond
the baseline.

---

## 6. Official training and certification

Verified against the Software AG learning portal search results and the Credly
badge page:

- Software AG runs an official learning portal (learn.softwareag.com /
  knowledge.softwareag.com). It offers a foundational course, **"Natural
  Programming Basic"** (course code seen as E307A-75E; a related
  "Natural Programming Fundamentals" 307-73E also appears in the catalog).
- The Natural Programming Basic course gives an overview of the Natural language
  and the NaturalONE development environment, covers basic program structure
  (building a first "Hello World" on NaturalONE), basic data types and value
  assignment, and the most important statements shown through examples.
- On completion, the learner can claim the **"Software AG Certified Natural
  Associate"** digital badge (Credly). Verified badge facts: level Foundational,
  cost Free, requirement is completion of the Natural Programming Basic course, no
  proctored exam is mentioned, and it certifies knowledge in ADABAS and Natural.
- A parallel "Adabas Basic" track exists for the database side.

Curriculum implication: the course being designed here can position itself as
preparing a learner for, and going beyond, the free official Associate badge. The
official course is a validating anchor, not a competitor, because it is a short
overview rather than a full path to job-ready competence.

Flag: the exact current chapter list, duration, and enrollment mechanics of the
official course could not be captured, because the learning-portal course pages
are JavaScript-rendered (Moodle) and returned a "Loading" shell or a DNS failure
to the fetch tool. The course's existence, scope summary, code, and the badge
requirements ARE verified from the search snippets and the Credly badge page.
Confirm the live syllabus by logging into the portal directly.

---

## Proposed ordered module list for the course

1. Orientation to Natural and ADABAS on the mainframe
2. First program and the editor (RUN vs STOW)
3. Data and the DEFINE DATA block (formats A/N/P/I/L/D/T)
4. Assignment and computation (:=, MOVE, COMPUTE)
5. Conditional logic (IF, DECIDE ON, DECIDE FOR)
6. Output (DISPLAY vs WRITE)
7. Database reading (DDM/VIEW, READ, FIND, HISTOGRAM)
8. Database writing and transactions (STORE/UPDATE/DELETE/GET/END TRANSACTION)
9. Loops and control (FOR, REPEAT, ESCAPE)
10. Screens (maps, INPUT USING MAP, INPUT, REINPUT)
11. Modularization I (inline subroutines, copycode)
12. Data areas (LDA, GDA, PDA)
13. Modularization II (external subroutines, subprograms/CALLNAT, functions)
14. Environment and workflow (libraries, FUSER/FNAT, STOW/CATALL, NaturalONE/SPoD)
15. Capstone multi-object application
16. Optional/advanced (helproutines, batch/JCL, Natural RPC, OO Natural)

---

## Items I could not fully verify (flags)

- **COMPUTE, FOR, and REPEAT verbatim examples**: the verbs are confirmed as
  documented Natural statements, but I did not fetch a verbatim official example
  for each in this spike. The skeletons shown are standard Natural and should be
  confirmed against the statement reference before publishing as authoritative
  code.
- **Official course syllabus and duration**: the learning-portal course pages are
  JS-rendered and were not machine-readable; only the scope summary, course codes,
  and badge requirements are verified.
- **Ownership/vendor naming**: I refer to "Software AG" per the documentation and
  portal domains as they read on 2026-07-19. Current corporate ownership of the
  Adabas & Natural product line is the subject of spike 01; defer to it.
- **Current version**: mainframe documentation is published through 9.x
  (a 9.2.2 mainframe webhelp URL appears in results) and Windows through 9.1.3.
  Exact current GA version is the subject of spike 01.

---

## Sources

All accessed 2026-07-19.

- Natural (NaturalONE 9.1.3) Programming Guide, "Programs, Functions, Subprograms
  and Subroutines" - the four routine types, invocation, standalone execution,
  data passing.
  https://documentation.softwareag.com/naturalONE/natONE913/natov/pg/pg_obj_prog.htm
- Natural (NaturalONE 9.1.2) "DEFINE DATA - General" - DEFINE DATA must be first,
  clauses, END-DEFINE.
  https://documentation.softwareag.com/naturalONE/natONE912/natov/sm/defineda_fu.htm
- Natural (Unix 8.4.1) Programming Guide, "User-Defined Variables" - the format
  table (A/U/B/N/P/I/F/L/D/T/C) with lengths and the (A20)/(N7.2)/(I4) notation.
  https://documentation.softwareag.com/natural/nat841unx/pg/pg_defi_dv.htm
- Natural (Unix 9.1.4) Programming Guide, "Natural Data Types" - type-to-buffer
  mapping confirming I1/I2/I4, F4/F8, and the format letters.
  https://documentation.softwareag.com/natural/nat914unx/pg/pg_nni_nat_dat_typ.htm
- Natural (Mainframe 8.2.8) Programming Guide, "Data Areas" - LDA, GDA, PDA
  semantics, USING syntax, parameter matching rules.
  https://documentation.softwareag.com/natural/nat828mf/pg/pg_obj_darea.htm
- Natural (Mainframe 8.2.8) First Steps tutorial, overview and "Hello World!" -
  session list, the verbatim Hello World program, RUN/STOW workflow, creating an
  object from the Development Functions menu.
  https://documentation.softwareag.com/natural/nat828mf/firststeps/fs-over.htm
  https://documentation.softwareag.com/natural/nat828mf/firststeps/fs-hello.htm
- Natural (Mainframe 8.2.8) First Steps tutorial, "Database Access" - the verbatim
  DEFINE DATA + READ EMPLOYEES-VIEW program and the EMPLOYEES DDM.
  https://documentation.softwareag.com/natural/nat828mf/firststeps/fs-dbaccess.htm
- Natural (Mainframe 9.2.2) First Steps tutorial, "User Input" - adding data
  fields to a map, binding to #NAME-START/#NAME-END, system-variable fields.
  https://documentation.softwareag.com/natmf/9.2.2/en/webhelp/natmf-webhelp/firststeps/fs-input.htm
- Natural (Mainframe 9.1.1) Programming Guide, "Natural and Database Access" - DML
  verbs FIND/READ/STORE/DELETE, the role of DDMs, cross-database abstraction.
  https://documentation.softwareag.com/natural/nat911mf/pg/pg_dbms_dbgen.htm
- Natural (Mainframe 9.1.1) Statements, "FIND" - verbatim FIND examples with
  WITH/WHERE/SORTED BY and IF NO RECORDS FOUND.
  https://documentation.softwareag.com/natural/nat911mf/sm/find.htm
- Natural (Mainframe 9.1.1) Statements, "STORE" - verbatim field-assignment write
  and END TRANSACTION.
  https://documentation.softwareag.com/natural/nat911mf/sm/store.htm
- Natural (Unix 9.1.4) Statements, "DECIDE ON" - verbatim DECEX4 example with
  VALUE ranges, ANY/ALL/NONE VALUE, EVERY option.
  https://documentation.softwareag.com/natural/nat914unx/sm/decideon.htm
- Natural (Mainframe 8.2.7) Statements, "DECIDE FOR" - verbatim EVERY CONDITION
  example with WHEN/WHEN ANY/WHEN ALL/WHEN NONE.
  https://documentation.softwareag.com/natural/nat827mf/sm/decidefo.htm
- Natural (Mainframe 8.2.7) Statements, "INPUT" and "INPUT Syntax 2 - Using
  Predefined Map Layout" - INPUT modes, block-mode 3270 behavior, INPUT USING MAP.
  https://documentation.softwareag.com/natural/nat827mf/sm/input.htm
  https://documentation.softwareag.com/natural/nat827mf/sm/input2.htm
- Natural (Mainframe 9.1.2) Programming Guide, "Map" object - maps referenced via
  INPUT USING MAP / WRITE USING MAP; text vs data fields.
  https://documentation.softwareag.com/natural/nat912mf/pg/pg_obj_map.htm
- NaturalONE 9.1.1 FAQ - NaturalONE vs Natural Studio (Eclipse difference), SPoD
  parallel work, FUSER/FNAT, SYSTEM steplib, CATALL-equivalent recatalog.
  https://documentation.softwareag.com/naturalONE/natONE911/core/faq/faq.htm
- Natural SPoD 4.1.9 Architecture - Single Point of Development concept, Natural
  Studio as GUI to Natural servers.
  https://documentation.softwareag.com/natural/spod0419/core/doc/spod-intro-arc.htm
- Software AG Certified Natural Associate badge (Credly) - requirement is
  completion of Natural Programming Basic, Foundational level, free, no exam
  mentioned, certifies ADABAS + Natural.
  https://www.credly.com/org/software-ag/badge/software-ag-certified-natural-associate
- Software AG learning portal search results - Natural Programming Basic
  (E307A-75E) and Natural Programming Fundamentals (307-73E) course listings and
  scope summary.
  https://learn.softwareag.com/course/info.php?id=1467
- Adabas & Natural Community Edition Guide (Oct 2024, v1.3) - container-based
  Natural for hands-on learning without a physical mainframe (cross-reference
  spike 05).
  https://softwareag-usa.s3.amazonaws.com/Adanat_Docker/AN+Community+Edition+Guide.pdf
