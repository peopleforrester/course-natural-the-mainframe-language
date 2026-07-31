<!-- ABOUTME: Validation of the proposed Natural course module ordering against real published
     ABOUTME: instructor-led syllabi, vendor tutorials, and the official Software AG training catalog. -->

# Spike 09: Curriculum Validation Against Real Published Natural Syllabi

Spike date: 2026-07-31

## Executive summary

**The curriculum is sound. It does not need rebuilding, and it needs only one true
reordering. It does need one gap closed that I would classify as a defect rather than a
preference.**

I found seven real, published Natural teaching sequences, including the one that matters
most: the Software AG Global Education Services catalog entry for **Natural Programming
Fundamentals**, a five-day instructor-led beginner course whose eleven-topic content list
is printed in order. That is the closest published analogue to what we are building, and
our proposed ordering matches it more closely than it matches anything else in the corpus.

Findings in order of consequence:

1. **`INPUT` is missing from Tier 1 entirely, and that is a defect.** Every real syllabus
   in the corpus gets the learner taking user input early: position 5 of 12 in the vendor's
   own First Steps tutorial, position 4 of 11 in NATURAL Essentials, position 6 of 15 at
   Verhoef. Our Tier 1 outline never teaches it. Worse, `INPUT` is the single statement the
   entire interpreter architecture was designed around. `CLAUDE.md` makes the resumable
   state machine a non-negotiable constraint specifically because `INPUT` must yield to
   JavaScript, yet no Tier 1 lesson exercises it. We are paying the full architectural cost
   of `INPUT` and shipping none of its teaching value. The vendor's own verified `DECIDE ON`
   and `DECIDE FOR` example programs both open with an `INPUT` statement, so module 5 cannot
   even use the official examples verbatim as written today.

2. **The Tier 1 outline's ordering beats the research document's ordering.** Loops at 7 and
   database at 8 and 9 is better than database at 7 and 8 with loops at 9. The two documents
   disagree and the outline is right. Reasoning in section 9.

3. **Data areas at 12, between Modularization I at 11 and CALLNAT at 13, is the single
   best-validated decision in the whole curriculum.** The vendor's First Steps tutorial runs
   Inline Subroutines, then Local Data Areas, then Global Data Areas, then External
   Subroutines, then Subprograms. That is our 11, 12, 13 exactly. Do not touch it.

4. **`DEFINE DATA` at module 3 is correct and is not merely defensible.** Software AG's own
   five-day fundamentals course lists "Data definition" as topic 1 of 11, ahead of
   assignments and arithmetic. NATURAL Essentials puts it second. The compiler forces the
   issue anyway, since structured mode requires `DEFINE DATA` to be the first statement.

5. **Maps at module 10 is too early relative to the evidence, and should move to 14.** The
   vendor's fundamentals course puts character-oriented user interface development at
   position 10 of 11, after modularisation, not before it. Real maps bind to fields declared
   in an LDA, so maps depend on data areas. This costs us nothing because full 3270 map I/O
   is already out of scope for v1.

6. **Arrays are under-weighted.** Software AG puts "static tables and spreadsheet processing"
   at position 2 of 11, immediately after data definition. The official education package
   devotes an entire quarter of its four-program sequence to arrays. Our outline gives them
   a parenthetical `(1:10)` inside module 3. Fixing this also fixes the weakest part of the
   loops argument, because iterating a declared array is the honest first job for `FOR`.

7. **Error handling is misfiled as advanced.** Software AG lists it as topic 11 of 11 in the
   *fundamentals* course and it appears in the Natural Certified Application Developer test
   blueprint under core Natural syntax. We have it at module 16 alongside OO Natural.

8. **The prerequisites claim is confirmed.** No Natural *programming* course in the official
   catalog lists TSO/ISPF or JCL as an entry requirement. Operating-system and TP-monitor
   knowledge appears only as a prerequisite for the *administration* courses. Verhoef teaches
   JCL as topic 12 inside the course, which confirms it is content rather than a gate.

---

## 1. Source inventory and how much weight each carries

I graded sources by whether they represent a designed teaching sequence or merely a topical
index, because that distinction turned out to explain most of the apparent disagreement.

| Source | Type | Weight | Why |
|---|---|---|---|
| Software AG Global Education Services Catalog, Natural Programming Fundamentals, 5 days | Vendor instructor-led course, ordered content list | **Highest** | A designed beginner course for developers. Closest analogue to our product. |
| Software AG Global Education Services Catalog, Building Applications with Natural (I), 5 days | Vendor instructor-led course, ordered content list | **High** | Designed course, but assumes the 2-day Adabas Fundamentals prerequisite. |
| Natural First Steps tutorial (mainframe 8.2.7, 8.2.8, 9.2.2; Unix 9.1.2) | Vendor hands-on tutorial, explicitly sequenced | **High** | The vendor states the chapters must be worked in the given order. But it is a scripted walkthrough, not a course. |
| NATURAL Essentials (Stephen Paul Simpson, v2.10, 2000, 291 pages) | Third-party self-study course | **High** | A genuine designed course by a former Software AG employee. Old, but its sequencing logic is intact. |
| Verhoef Training, Adabas Natural Programming, 3 days | Third-party instructor-led, ordered topic list | **Medium** | Real published outline, but it is an Adabas-first course with Natural as the access layer. |
| SoftwareAG/adabas-natural-education-package (GitHub, archived 2023) | Official vendor education package, 4 programs | **Medium** | Vendor-authored beginner sequence, but only four programs deep. |
| Natural Programming Guide (9.1.3) table of contents | Vendor reference manual | **Low for ordering** | Organized topically, not pedagogically. Useful for how the vendor *frames* concepts, not for sequence. |
| Nisa Trainings (25 hours), MaxMunus (20 to 25 hours), Vistasparks (20+ hours) | Third-party online course marketing pages | **Low** | Coarse, marketing-shaped, and inconsistent. Useful only for duration data. |

Two providers named in the task could not be verified. `idestrainings.com` no longer
resolves in DNS. The Software AG learning portal course pages at `learn.softwareag.com`
and `knowledge.softwareag.com` are JavaScript-rendered Moodle shells that return only a
"Loading" placeholder to a fetch tool, so the current syllabi for Natural Programming Basic
(E307A-75E), Natural Programming Fundamentals (E307-73E and 307-73E), and NaturalONE Basic
(E310A-75E) could not be captured. I am reporting that rather than guessing at them.

---

## 2. The real syllabi, transcribed in order

### 2.1 Software AG, Natural Programming Fundamentals, 5 days

The highest-value find. Prerequisites are stated as "Basic knowledge in programming and
databases". Audience is "Software developers, Software architects", and the description
says the course is "both for inexperienced and experienced Software developers who have
acquired experience in other programming languages". Content, in the order printed:

1. Data definition
2. Static tables and spreadsheet processing
3. Assignments
4. Arithmetic
5. Loops
6. Conditional processing
7. File processing
8. Database access, transactional logic
9. Modularisation
10. Development of character-oriented user interfaces
11. Error handling

A 2023 Software AG Tech Community article confirms this course survives under the code
**307-66E, "Natural Programming - Basics"**, with a matching description, and that
**308-66E, "Natural Programming - Advanced"**, builds on it.

### 2.2 Software AG, Building Applications with Natural (I), 5 days

Prerequisites are "General programming skills" plus the 2-day Adabas Fundamentals course.
Content in order:

1. The Natural system, an overview
2. Working in the Natural development environment
3. Prototyping
4. Program editor, data area editor, map editor
5. Variables and data structures
6. Statements and commands
7. Application modularization techniques
8. Database accesses and modifications
9. Transaction logic
10. Creating reports
11. Debugging

### 2.3 Software AG, First Steps tutorial (mainframe and Unix)

The vendor's own hands-on sequence, with the instruction that topics must be read and
worked "in the sequence indicated". Identical ordering across the 8.2.7, 8.2.8, 9.2.2
mainframe editions and the 9.1.2 Unix edition:

1. About this Tutorial
2. Getting Started with Natural
3. Hello World!
4. Database Access
5. User Input
6. Loops and Labels
7. Inline Subroutines
8. Processing Rules and Helproutines
9. Local Data Areas
10. Global Data Areas
11. External Subroutines
12. Subprograms

### 2.4 NATURAL Essentials, Part 2 (Essential Syntax)

Part 1 front-loads *concepts* (module types, LDA, PDA, GDA, maps, libraries, structured
versus reporting mode, ISN, descriptors) with no syntax. Part 2 is the teaching sequence:

1. Structure of executable modules (`END`, comments, `INCLUDE`, `DEFINE SUBROUTINE`)
2. Data definition (`DEFINE DATA`, formats, record structures, database views, redefines,
   `INIT`, `CONST`, arrays, edit masks)
3. Data manipulation (`RESET`, `:=`, `COMPUTE`, `ADD`, `SUBTRACT`, `MOVE`, `COMPRESS`,
   `EXAMINE`)
4. Input and output (`INPUT USING MAP`, `REINPUT`, windows, `SET KEY`, `DISPLAY`, `WRITE`,
   `FORMAT`, `NEWPAGE`, `AT TOP OF PAGE`)
5. Flow control (`IF`, `DECIDE ON`, `DECIDE FOR`, `FOR`, `REPEAT`, `PERFORM`, `CALLNAT`,
   `FETCH`, `STOP`)
6. Database access (`READ`, `READ PHYSICAL`, labels, `*ISN`, `FIND`, `*COUNTER`, `GET`,
   `STORE`, `UPDATE`, `DELETE`)
7. Transaction control
8. Database query (`FIND NUMBER`, `HISTOGRAM`, `*NUMBER`)
9. Sequential files (`READ WORK FILE`, `WRITE WORK FILE`)
10. Exiting a routine or loop (`ESCAPE`, `ESCAPE TOP`, `ESCAPE BOTTOM`, `ESCAPE ROUTINE`)
11. System variables and functions

Part 3 is a complete four-module worked application with source and narrative. Appendix A
covers the environment, command line, program editor, data area editor, and map editor.

Two structural details are worth stealing. First, `PERFORM`, `CALLNAT`, and `FETCH` are
taught *inside* flow control rather than as a separate late modularization block. Second,
`ESCAPE` is deliberately deferred to a chapter *after* database access, because escaping a
database loop is its real job.

### 2.5 Verhoef Training, Adabas Natural Programming, 3 days

On-site only. Audience is "Developers requiring the ability to design and code in Adabas
Natural". Prerequisite: "Delegates should ideally have some programming skills in another
programming language."

1. Adabas Introduction
2. Adabas, Natural Access Statements
3. Natural 1, Objects
4. Natural 2, Objects Structure
5. Natural 3, Statements
6. Natural 4, IO Statements
7. Natural 5, Flow Control
8. Natural 6, Batch Access
9. Natural 7, Escape Statements
10. Natural 8, System Variables and Functions
11. Natural 9, Various AT Statements
12. JCL
13. Tools and Debugging (high level)
14. Case Study / Exercises
15. Reverse KT / Monk Interview (if needed)

### 2.6 Official Software AG education package (GitHub, archived)

Four programs, audience stated as beginners:

1. Hello World (`WRITE`)
2. Conditional constructs (`DECIDE ON` with `FIRST`, plus `DISPLAY`)
3. Arrays (two-dimensional, definition, initialization, transfer, printing via nested loops)
4. Adabas (compile a DDM, run a `HISTOGRAM` program, `REPEAT` and `ESCAPE` for loop control)

### 2.7 Third-party online providers

Nisa Trainings (25 hours) runs Introduction, Getting Started, Working with ADABAS
(`READ`/`FIND`), Integrating Natural with ADABAS, Advanced Natural Programming (loops,
conditions, subprograms), Advanced ADABAS. MaxMunus (20 to 25 hours) publishes an unordered
topic dump that opens with the four editors. Vistasparks (20+ hours) publishes nine generic
headings. None of these is a designed sequence and I weight them accordingly.

---

## 3. Side-by-side comparison

Numbers are the topic's position within that syllabus. A dash means the topic is absent.
"Ours" is the 16-module spec list; "T1" is the Tier 1 outline.

| Topic | Ours | T1 | SAG Fundamentals (5d) | SAG First Steps | NAT Essentials P2 | Verhoef (3d) | SAG BAN I (5d) |
|---|---|---|---|---|---|---|---|
| Orientation, what Natural is | 1 | 1 | - | - | Part 1 | 3 | 1 |
| Environment and editors | 2, 14 | 2 | - | 2 | App. A | 13 | 2, 4 |
| First runnable program | 2 | 2 | - | 3 | 1 | - | - |
| `DEFINE DATA`, formats | 3 | 3 | **1** | 4 (in situ) | 2 | 5 | 5 |
| Arrays and tables | in 3 | in 3 | **2** | - | in 2 | 5 | 5 |
| Assignment and arithmetic | 4 | 4 | 3, 4 | - | 3 | 5 | 6 |
| Conditional logic | 5 | 5 | 6 | - | 5 | 7 | 6 |
| Loops (`FOR`, `REPEAT`) | 9 | **7** | **5** | 6 | 5 | 7 | 6 |
| `ESCAPE` | 9 | 7 | 5 | 6 | **10** | **9** | 6 |
| Output (`DISPLAY`, `WRITE`) | 6 | 6 | - | 4 (in situ) | 4 | 6 | **10** |
| Interactive `INPUT` | in 10 | **absent** | 10 | **5** | **4** | **6** | - |
| Maps and screen design | 10 | out | **10** | 5 | 4, App. A | 6 | 4 |
| Database read | 7 | **8** | 8 | **4** | 6, 8 | **2** | 8 |
| Database write, transactions | 8 | **9** | 8 | - | 6, 7 | 2 | 8, 9 |
| Work files, sequential I/O | - | - | **7** | - | **9** | **8** | - |
| Modularization | 11, 13 | out | 9 | 7, 11, 12 | Part 1, 5 | 3, 4 | 7 |
| Data areas (LDA/GDA/PDA) | **12** | out | 9 | **9, 10** | Part 1, 2 | 4 | 5 |
| Error handling | 16 | - | **11** | - | - | - | - |
| Debugging | - | - | - | - | - | 13 | **11** |
| Batch and JCL | 16 | - | - | - | - | 8, 12 | - |
| System variables and functions | in 8 | 8 | - | 6 | 11 | 10 | - |
| `AT` statements (break, page) | - | part of 8 | - | - | 4 | 11 | 10 |

---

## 4. Where we match and where we diverge

### Matches, with no change needed

**Orientation first.** Verhoef, BAN(I), and NATURAL Essentials Part 1 all open with what
Natural is and what the object types are. Our module 1 is standard practice.

**`DEFINE DATA` early.** Software AG's fundamentals course opens on data definition. NATURAL
Essentials makes it section 2. BAN(I) has "Variables and data structures" at 5, ahead of
"Statements and commands" at 6. Our module 3 is squarely inside the published range.

**Assignment and arithmetic immediately after data definition.** Software AG runs data
definition, arrays, assignments, arithmetic as topics 1 through 4. NATURAL Essentials runs
data definition then data manipulation. Our modules 3 and 4 replicate this.

**Modularization I, then data areas, then Modularization II.** First Steps runs inline
subroutines (7), local data areas (9), global data areas (10), external subroutines (11),
subprograms (12). Our 11, 12, 13 is the same shape. Software AG's fundamentals course
collapses all of it into one "Modularisation" topic, which does not contradict us.

**Database write and transactions kept together.** Software AG lists "Database access,
transactional logic" as one topic. NATURAL Essentials runs database access then transaction
control back to back. BAN(I) runs "Database accesses and modifications" then "Transaction
logic". Our modules 8 and 9 in the Tier 1 numbering are adjacent, which is correct.

**Capstone as a multi-object application.** NATURAL Essentials Part 3 is exactly this: a
complete four-module worked application with narrative. Verhoef ends on "Case Study /
Exercises". Our module 15 is standard practice.

### Divergences that need a justification

**D1. `INPUT` is absent from Tier 1.** Four of the four syllabi that teach interactive input
at all place it in the first half. Verdict: this is not a defensible divergence. Fix in
section 10.

**D2. Loops before database (Tier 1) versus after (spec).** The corpus splits. Five sources
put database access before the general-purpose loop statements: First Steps, the Programming
Guide, Verhoef, Nisa, and the education package. Two put loops first: Software AG's
fundamentals course and NATURAL Essentials. The split is not random. The two that teach
loops first are precisely the two that are *designed beginner programming courses for
developers*. The five that put the database first are either topical reference manuals, or
Adabas-centric courses where Natural is the access layer, or scripted walkthroughs where the
learner types a supplied program rather than composing one. Our product is a designed
beginner programming course. Verdict: the divergence is justified, and the Tier 1 outline is
the correct one. Full reasoning in section 9.

**D3. Maps at module 10.** Software AG's fundamentals course puts character-oriented user
interface development at 10 of 11, after modularisation at 9. Ours puts maps at 10 of 16,
before modularization at 11. Verdict: move maps to 14, after Modularization II. Real maps
bind to fields declared in an LDA and are driven by programs that `PERFORM` validation
subroutines, so the dependency runs the way Software AG teaches it, not the way we have it.

**D4. Arrays get one parenthetical.** Software AG gives arrays their own topic at position 2
of 11. The official education package spends program 3 of 4 on them. The Programming Guide
gives them chapters 24 and 25. Verdict: not defensible as-is. Expand.

**D5. Error handling is filed under "optional/advanced".** Software AG puts it in the
*fundamentals* course, and the Natural Certified Application Developer blueprint lists error
handling under "Natural syntax", which is core test material. Verdict: promote it.

**D6. Conditionals before loops (ours) versus loops before conditionals (Software AG).**
Software AG lists Loops at 5 and Conditional processing at 6. Verdict: keep our order and do
not follow theirs. `REPEAT ... UNTIL`, `REPEAT ... WHILE`, and every useful `ESCAPE` all
require a condition, so conditionals must precede the loop statements that consume them. I
read the catalog ordering here as blurb sequencing rather than strict teaching sequence,
since a five-day course would not genuinely teach `REPEAT UNTIL` before `IF`.

**D7. No work-file content anywhere.** Software AG puts "File processing" at 7, *before*
database access. NATURAL Essentials has a "Sequential files" chapter. Verhoef has "Batch
Access" at 8. We have nothing until "batch and JCL" gets a mention in module 16. Verdict:
acceptable for v1 given the browser constraint, but it should be an explicit Tier 2 item
rather than an omission, because batch Natural maintenance work uses work files constantly.

**D8. No debugging module.** BAN(I) ends on Debugging. Verhoef has "Tools and Debugging".
The NaturalONE course includes the debugger. Verdict: a genuine gap, though a low-priority
one for a browser course that has no debugger to teach.

---

## 5. The five judgment calls, assessed

### 5.1 Is teaching `FOR`/`REPEAT` after database `READ`/`FIND` defensible?

**Evidence.** The vendor's own conceptual framing works against database-first. The
Programming Guide's Loop Processing chapter divides the world into "database loops", which
are "those created automatically by Natural to process data selected from a database as a
result of a READ, FIND or HISTOGRAM statement", and "non-database loops", which are
"initiated by the statements REPEAT, FOR, CALL FILE, CALL LOOP, SORT and READ WORK FILE".
In Natural's own vocabulary, `FOR` and `REPEAT` are defined by *not* being database loops.
That is a real risk: a learner who meets `READ ... END-READ` first, with no prior loop
vocabulary, has to absorb iteration and database access in the same lesson.

**The First Steps evidence is more subtle than its ordering suggests.** Its "Loops and
Labels" chapter does not introduce `FOR` at all. It introduces `REPEAT` as an outer wrapper
around an interactive program so the user can run it repeatedly, exits it with
`ESCAPE BOTTOM (RP1.)` on a sentinel value typed into a map, and then goes back and adds a
label to the earlier `READ` loop so `*COUNTER (RD1.)` can detect the empty result set. In
other words, the vendor teaches `REPEAT` *because* the learner already has an `INPUT` and a
`READ` to wrap. Without `INPUT`, that motivation does not exist, which is part of why our
module 7 currently has to fall back on `FOR #I = 1 TO 10 / WRITE 'Iteration' #I`.

**Judgment.** Loops first, but the placement only works if `FOR` has an honest job to do
before the database arrives. Counting to ten and printing is not that job. Iterating a
declared array is, and Software AG's own fundamentals course puts arrays at position 2 and
loops at position 5, while the education package's array program prints its two-dimensional
array with nested loops. So: teach arrays properly in module 3, teach `FOR` over an array in
module 7, and then open module 8 by naming the vendor's distinction out loud. The lesson
should say that `READ`, `FIND`, and `HISTOGRAM` are loops in exactly the sense the learner
already knows, closed by `END-READ` and `END-FIND` the way `FOR` is closed by `END-FOR`, and
that Software AG calls them database loops to distinguish them from the ones the learner has
already written. Naming the confusion is cheaper than sequencing around it.

**Which should come first: loops.** Marked as judgment informed by evidence, not as a
straight evidence read, because the raw syllabus count favors database-first five to two.

### 5.2 Should `DISPLAY` versus `WRITE` come before or after database access?

**Evidence, split.** NATURAL Essentials puts the whole input-and-output chapter at position
4, before flow control and before database access. The education package uses `DISPLAY` in
program 2, before Adabas in program 4. Against that, BAN(I) puts "Creating reports" at 10,
after database and transactions, and the Programming Guide places "Report Format and
Control" (Part VI) after "Database Access" (Part V). Software AG's fundamentals course does
not list output as a separate topic at all, which suggests it is folded in wherever needed.

**Judgment.** Keep the basic distinction at module 6, before the database. `DISPLAY` versus
`WRITE` is a small, sharp concept and beginners genuinely confuse them, so it earns an early
slot. But move the *heavy* report machinery to module 8. Column headers, `NOTITLE`, `NOHDR`,
page titles, `SKIP`, and `AT TOP OF PAGE` all want real rows to format, and `DISPLAY`'s
headline feature is that it derives column headers from *field* names. Demonstrated against
hand-declared `#SALARY` and `#NAME` variables, that feature is a degenerate version of
itself. Demonstrated against `EMPLOYEES-VIEW`, it is the reason `DISPLAY` exists.

I would also add `AT BREAK` and control-break processing to module 8 or Tier 2. It is
Programming Guide chapter 47, it is pervasive in real Natural report code, and it appears
nowhere in our curriculum. Marked as judgment.

### 5.3 Are maps at module 10 right, or do learners need `INPUT` earlier?

**Evidence, and it is one-sided.** First Steps puts User Input at 5 of 12, directly after
Database Access. NATURAL Essentials puts the input-and-output chapter at 4 of 11. Verhoef
puts "IO Statements" covering input and maps at 6 of 15. Only Software AG's fundamentals
course puts user interface development late, at 10 of 11.

The apparent conflict resolves once you separate two things the syllabi treat separately.
The Unix First Steps "User Input" chapter teaches the `INPUT` statement used *directly in a
program* first, and only then introduces the Map Editor as a way to separate the interface
from the logic. The early placements are all about *prompt-style `INPUT`*. The late placement
in the fundamentals course is about *map design*, which is a distinct skill involving the map
editor, field attributes, `REINPUT` validation, `MARK`, PF keys, and windows.

**Judgment, high confidence.** Split them. Prompt-style `INPUT` belongs at module 5, bundled
with conditional logic, and this is close to forced: the verified `DECIDE ON` and
`DECIDE FOR` example programs from spike 02 both begin with an `INPUT` statement, so module 5
cannot use the vendor's own examples verbatim without it. Map design belongs at 14, after
data areas and modularization, for the dependency reason in D3. Since full 3270 map I/O is
already out of v1 scope, this is a Tier 2 decision with no build cost.

The secondary benefit is architectural. `CLAUDE.md` makes the resumable state machine a
non-negotiable constraint on the interpreter, justified entirely by `INPUT`. Under the
current outline, milestone M-C ships that machinery with nothing to exercise it and no lesson
proving it works. Putting `INPUT` in module 5 puts it in M-C, where the resumability has to
be proven anyway.

### 5.4 Are data areas correctly placed at 12, after Modularization I and before CALLNAT?

**Evidence, and this is the cleanest validation in the report.** First Steps runs Inline
Subroutines (7), Processing Rules and Helproutines (8), Local Data Areas (9), Global Data
Areas (10), External Subroutines (11), Subprograms (12). Our 11, 12, 13 reproduces the
inline-subroutines then data-areas then external-routines arc precisely. The Programming
Guide reinforces the dependency: a PDA is the parameter interface for a subprogram, so
`CALLNAT` cannot be taught honestly before PDAs exist.

NATURAL Essentials appears to disagree by front-loading LDA, PDA, and GDA into its Part 1
concepts, but Part 1 is explicitly conceptual with no syntax, and the actual `DEFINE DATA
LOCAL USING` syntax lands in Part 2 section 2. That is a reference-organization choice, not
a teaching-order disagreement.

**Verdict: keep 11, 12, 13 exactly as they are.** Evidence-backed, not judgment.

### 5.5 Is `DEFINE DATA` genuinely the right module-3 foundation?

**Evidence.** Software AG's fundamentals course opens on data definition as topic 1 of 11,
ahead of assignments. NATURAL Essentials makes it section 2 of Part 2, immediately after
module structure. BAN(I) has "Variables and data structures" at 5, before "Statements and
commands" at 6. Three of the four designed courses put formal data declaration ahead of
almost all executable content.

**The one dissent is instructive.** First Steps does not teach `DEFINE DATA` as a unit at
all. It appears for the first time inside the Database Access chapter, as part of a program
the learner is told to type, because that is the first program that needs a view and some
variables. The vendor's hands-on tutorial teaches it *in service of a task*.

**Judgment.** Module 3 is right, and the language forces it: in structured mode `DEFINE DATA`
must be the first statement of any object that declares data, so the second a learner needs a
variable they have met it. What I would borrow from First Steps is the framing, not the
position. Module 3 should not be a tour of the format table. It should be built around a
program the learner wants to write that requires declared variables, with the format table
introduced as the thing that makes `(N7.2)` behave differently from a float. The current
module 3 example is `WRITE 'Declared.'`, which declares three variables and uses none of
them. That is a lesson about syntax rather than about data, and it is the one place where I
think our outline is weaker than the vendor's tutorial.

---

## 6. Prerequisites: the TSO/ISPF and JCL claim is confirmed

The research document claims TSO/ISPF and JCL are not prerequisites for learning Natural on
a modern editor. Confirmed, from the official catalog's own prerequisite fields.

| Course | Stated prerequisites |
|---|---|
| Natural Programming Fundamentals (5d) | "Basic knowledge in programming and databases" |
| Building Applications with Natural (I) (5d) | "General programming skills"; requires Adabas Fundamentals (2d) |
| Building Applications with Natural (II) (5d) | "Programming experience"; requires BAN (I) |
| Building Applications with NaturalONE (3d) | "Programming experience"; requires Natural Programming Fundamentals |
| Adabas Fundamentals (2d) | "Sound knowledge of information processing" |
| Verhoef Adabas Natural Programming (3d) | "Delegates should ideally have some programming skills in another programming language" |
| Natural Organization and Administration (4d) | Knowledge of an operating system (z/OS, BS2000) **and** a TP system (Com-plete, CICS, UTM) |

Operating-system, TP-monitor, and mainframe-operations knowledge appears exactly once, and
it is on an *administration* course, not a programming one. Verhoef, which is the most
mainframe-native course in the corpus, teaches JCL as topic 12 *inside* the three days, which
settles the question: JCL is late course content, not an entry gate. Our placement of
batch/JCL in module 16 matches that.

**One refinement the research document understates.** Adabas conceptual knowledge *is* a real
prerequisite in the vendor's own learning path. Building Applications with Natural (I)
requires the 2-day Adabas Fundamentals course covering "Adabas architecture and structures",
"Data definition, data description and design considerations", "Access logic for read and
update functions", and "Transaction logic". Notably, Natural Programming Fundamentals does
*not* require it, needing only "basic knowledge in programming and databases". So the vendor
runs both patterns. Our approach of folding conceptual Adabas into module 1 and DDM,
descriptor, and view concepts into module 8 is closer to the Fundamentals path, which is the
right one for us.

**Cross-training COBOL and mainframe developers, which the spec names as the paying
audience, satisfy every published prerequisite in the corpus.** Every single course assumes
prior programming experience in another language. Not one assumes a true novice. That is
worth internalizing: the word "beginner" in our spec means beginner *at Natural*, and the
published field agrees that this is the only kind of Natural beginner course anyone builds.

---

## 7. Time to competence, and what our scope is worth

Published durations:

| Offering | Duration | What it claims |
|---|---|---|
| Adabas Fundamentals | 2 days | Architecture, data definition, access logic, transaction logic |
| Natural Programming Fundamentals / Natural Programming Basics | 5 days | "develop new business applications and how to modify existing ones" |
| Building Applications with Natural (I) | 5 days | "implement applications with database accesses, processing logic and maps" |
| Building Applications with Natural (II) | 5 days | Advanced: access module design, RPC, error handling, complex Adabas, batch, work files, XML |
| Building Applications with NaturalONE | 3 days | IDE, data browser, debugger, versioning, web services |
| Natural Certified Application Developer | 3-hour exam, ~45 questions, 66% to pass | Requires BAN (I) and BAN (II), plus hands-on experience |
| Verhoef Adabas Natural Programming | 3 days | Design and code structured-mode programs against Adabas |
| Nisa Trainings / MaxMunus / Vistasparks | 20 to 25 hours | Natural plus Adabas administration, shallower |

**The vendor's own bar for certifiable competence is 10 classroom days plus real project
experience.** The certification page states that success "requires a solid knowledge in
Natural programming, as can be acquired through Software AG training courses and through
practical experience gained in medium-sized Natural development projects", and that
"it is imperative that you have hands-on experience developing and/or maintaining Natural
applications since this is a practice-oriented test".

**Sanity check on our scope.** Tier 1, modules 1 through 9, covers 6 of the 11 topics in the
vendor's 5-day fundamentals course: data definition, assignments, arithmetic, loops,
conditional processing, and database access with transactional logic. It omits arrays as a
topic, file processing, modularisation, user interfaces, and error handling. So **Tier 1 is
roughly 55 to 65 percent of a single 5-day vendor fundamentals course**, and roughly 30
percent of the 10-day path the vendor requires before its own certification exam.

Two conclusions follow. First, **the sealed contract term forbidding a job-readiness claim in
v1 is not conservatism, it is arithmetic.** The vendor requires twice our full 16-module
scope plus project experience before it will certify anyone. Second, at typical self-paced
expansion of two to three times instructor contact hours, Tier 1 is plausibly **8 to 15 hours
of learner time**, and the full 16 modules land in the neighborhood of a 5-day vendor
fundamentals course. That is a coherent product: Tier 1 as a paid introduction, Tier 1 plus
Tier 2 as an honest equivalent to Natural Programming Fundamentals, and the free official
Certified Natural Associate badge as a credible next step for the learner.

---

## 8. Gaps our curriculum has that the real syllabi do not

Ordered by how often they appear in the corpus.

1. **Interactive `INPUT`** appears in 5 of 7 syllabi, always in the first half. Absent from
   Tier 1 and only implicit in module 10 of the spec.
2. **Arrays as a taught topic** appear in 5 of 7, including position 2 of 11 in the vendor's
   fundamentals course. We have a parenthetical.
3. **Work files and sequential I/O** appear in 3 of 7, including *before* database access in
   the vendor's fundamentals course. Absent from our list.
4. **Error handling** appears in the vendor's fundamentals course, in BAN (II), in the
   Programming Guide (chapter 52, `ON ERROR` and the error transaction program), and in the
   certification blueprint. We file it under optional/advanced.
5. **Debugging** appears in BAN (I), Verhoef, and the NaturalONE course. Absent from our list.
6. **Control breaks (`AT BREAK`)** appear in Verhoef's "Various AT Statements" and Programming
   Guide chapter 47. We have `AT START OF DATA` and `AT END OF DATA` but not `AT BREAK`.
7. **`EXAMINE` and `COMPRESS`** get substantial treatment in NATURAL Essentials (seven
   subsections between them) and Programming Guide chapter 42. Our teaching subset omits both.
   String handling is routine maintenance work.

Items 1, 2, and 6 are cheap in a WASM interpreter and I would take all three. Items 3 and 5
are genuinely blocked by the browser architecture and should be named as such rather than
quietly dropped. Item 4 is a lesson-planning decision, not a build decision.

---

## 9. Verdict on the 7-versus-9 discrepancy

The research document orders database reading at 7, database writing at 8, loops at 9. The
Tier 1 outline orders loops at 7, database reading at 8, database writing at 9. They cannot
both stand.

**The Tier 1 outline is right. Amend the research document to match.**

Reasons, weighted:

The evidence is genuinely split five to two in favor of database-first, but the split sorts
cleanly by source type. Both sources that put general loops first are designed beginner
programming courses for developers: Software AG's own five-day Natural Programming
Fundamentals, and NATURAL Essentials. The five that put the database first are two reference
manuals organized topically, one Adabas-first course where Natural is the access layer, one
marketing-page syllabus that labels loops "advanced", and one scripted walkthrough where the
learner types supplied programs rather than composing them. When the corpus is filtered to
sources that share our product shape, the count reverses two to nil.

Under the outline's ordering, `END-READ` is the fourth `END-*` block the learner has met
rather than the first. That lets module 8 spend its whole budget on the things that are
actually specific to Natural database access, which are DDMs, views, descriptors, and the
choice between `READ` and `FIND`, instead of spending a third of it explaining that a
statement can enclose other statements.

The interpreter milestones already assume it. M-C delivers `FOR`, `REPEAT`, and `ESCAPE`
before M-D delivers `READ` and `FIND`. Reordering the lessons to match the research document
would mean shipping M-D before the loop constructs, or teaching lessons out of build order.
This is a supporting reason, not a pedagogical one, and I would not let it decide the
question on its own.

**The cost of the outline's ordering, and the required mitigation.** A learner who meets
`FOR` and `REPEAT` first may reasonably assume those are how you iterate a result set, and
then be surprised that `READ` loops by itself. That is a real risk and it is not hypothetical:
the vendor's own reference manual defines `FOR` and `REPEAT` negatively, as "non-database
loops". Module 8 must therefore open by naming the distinction in the vendor's own words,
and by showing side by side that `FOR ... END-FOR` and `READ ... END-READ` are the same
block shape with different iteration sources. If module 8 does not do that, the outline's
ordering is worse than the research document's.

---

## 10. Recommended ordering

### 10.1 Tier 1, v1 scope, nine modules

This keeps the sealed nine-module count and therefore needs no scope amendment. Three
changes from the current outline, marked **CHANGED**.

| # | Module | Change |
|---|---|---|
| 1 | Orientation | unchanged |
| 2 | Your first program (`WRITE`, `END`, RUN versus STOW) | unchanged |
| 3 | Data and `DEFINE DATA`: formats, level numbers, `INIT`, **and arrays as a first-class topic** | **CHANGED**: arrays promoted from a parenthetical; example program must *use* the variables it declares |
| 4 | Assignment and computation | unchanged |
| 5 | Conditional logic **and prompt-style `INPUT`** (`IF`, `DECIDE ON`, `DECIDE FOR`, `INPUT 'prompt' #FIELD`) | **CHANGED**: `INPUT` added, which lets module 5 use the vendor's verified `DECIDE` examples verbatim |
| 6 | Output basics (`DISPLAY` versus `WRITE`, `PRINT`, edit masks) | unchanged, but heavy report formatting deferred to 8 |
| 7 | Loops and block structure (`FOR` over an array, `REPEAT` with `UNTIL`/`WHILE` around an `INPUT` prompt, `ESCAPE`) | **CHANGED**: motivating examples replaced. `FOR` iterates the module-3 array; `REPEAT` wraps the module-5 `INPUT` with a sentinel exit, which is exactly the First Steps pattern and gives the runaway-loop cap a teaching role |
| 8 | Reading data (`VIEW OF`, `READ`, `FIND`, `HISTOGRAM`, loop labels, `ESCAPE BOTTOM (label)`, `*COUNTER`, `*NUMBER`, `AT START OF DATA`, `AT END OF DATA`, report formatting against real rows) | **CHANGED**: opens by naming database loops versus non-database loops; absorbs loop labels and the heavy `DISPLAY` formatting |
| 9 | Writing data and transactions | unchanged |

### 10.2 Full course, Tier 1 plus Tier 2

Four changes from the spec's 16-module list, marked **CHANGED**. The list grows to 18 because
two topics that every real syllabus treats as core were previously buried in the
optional/advanced module.

| # | Module | Change from spec |
|---|---|---|
| 1 | Orientation to Natural and Adabas | |
| 2 | First program and the editor model | |
| 3 | Data, `DEFINE DATA`, formats, and arrays | **CHANGED**: arrays promoted |
| 4 | Assignment and computation | |
| 5 | Conditional logic and prompt-style `INPUT` | **CHANGED**: `INPUT` added |
| 6 | Output basics (`DISPLAY` versus `WRITE`, edit masks) | |
| 7 | Loops and block structure (`FOR`, `REPEAT`, `ESCAPE`) | **CHANGED**: moved up from 9, per section 9 |
| 8 | Reading data (`READ`, `FIND`, `HISTOGRAM`, labels, `AT` statements, report formatting) | **CHANGED**: moved down from 7 |
| 9 | Writing data and transactions | **CHANGED**: moved down from 8 |
| 10 | String and table handling (`EXAMINE`, `COMPRESS`, `MOVE` variants, array processing, `AT BREAK` control breaks) | **NEW**: covers gaps 6 and 7 from section 8 |
| 11 | Modularization I (inline subroutines, `PERFORM`, copycode) | unchanged |
| 12 | Data areas (LDA, GDA, PDA; the data area editor) | unchanged |
| 13 | Modularization II (external subroutines, `CALLNAT`, functions, parameter matching, `ESCAPE ROUTINE`) | unchanged |
| 14 | Maps and screens (map editor, `INPUT USING MAP`, `WRITE USING MAP`, `REINPUT`, `MARK`, processing rules) | **CHANGED**: moved from 10, per D3 |
| 15 | Error handling (`ON ERROR`, error transaction programs, diagnostics) | **CHANGED**: promoted from module 16 |
| 16 | Environment and workflow (libraries, FUSER/FNAT, steplibs, `STOW`/`CATALOG`/`CATALL`, NaturalONE versus native versus SPoD, the debugger) | **CHANGED**: debugging folded in |
| 17 | Capstone: a multi-object application | |
| 18 | Optional/advanced (helproutines, work files, batch and JCL, Natural RPC, OO Natural) | |

### 10.3 What is evidence and what is judgment

| Recommendation | Basis |
|---|---|
| Keep `DEFINE DATA` at 3 | **Evidence.** Software AG fundamentals topic 1; NATURAL Essentials section 2; BAN (I) position 5 |
| Keep data areas at 12 between 11 and 13 | **Evidence.** First Steps chapters 7, 9, 10, 11, 12 map directly onto it |
| Add `INPUT` at module 5 | **Evidence** for early placement (5 of 7 syllabi); **judgment** for pairing it with conditionals, though the vendor's own `DECIDE` examples require `INPUT` |
| Promote arrays to a first-class module-3 topic | **Evidence.** Software AG fundamentals topic 2; education package program 3; Programming Guide chapters 24 and 25 |
| Loops at 7, database at 8 and 9 | **Judgment informed by evidence.** Raw syllabus count favors the opposite; the filtered count of designed beginner courses favors this |
| Open module 8 with the database-loop versus non-database-loop distinction | **Evidence** for the framing (Programming Guide chapter 46 wording); **judgment** that it belongs in the lesson |
| Keep `DISPLAY` versus `WRITE` at 6, defer heavy formatting to 8 | **Judgment.** The corpus splits evenly; the tiebreaker is that `DISPLAY`'s derived column headers need real fields |
| Move maps from 10 to 14 | **Evidence** from Software AG fundamentals topic 10 of 11 after modularisation; **judgment** on the exact slot |
| Promote error handling out of optional/advanced | **Evidence.** Software AG fundamentals topic 11; certification blueprint lists it under core Natural syntax |
| Add module 10 (strings, tables, control breaks) | **Judgment.** The individual statements are well evidenced; grouping them into one module is mine |
| Name work files and the debugger as architecture-blocked rather than omitted | **Judgment** |

---

## Sources

Accessed 2026-07-31 and 2026-08-01.

- Software AG **Global Education Services Catalog** (PDF, document creation date 2016-10-24).
  The Adabas and Natural section, pages 129 to 147, carries the ordered content lists,
  durations, prerequisites, and learning-path diagrams for Natural Programming Fundamentals
  (5 days), Building Applications with Natural (I) and (II) (5 days each), Building
  Applications with NaturalONE (3 days), Adabas Fundamentals (2 days), the administration
  courses, and the Natural Certified Application Developer test blueprint.
  https://ariscommunity.com/system/files/SAG_Global_Education_Services_Catalog.pdf
- Software AG Tech Community, "Adabas and Natural training modernized courses". Confirms the
  fundamentals course persists as Natural Programming - Basics (307-66E) with Natural
  Programming - Advanced (308-66E) above it.
  https://techcommunity.medium.com/adabas-natural-training-modernized-courses-cded4e5628df
- Software AG Tech Community, "Introducing Free Training Essentials for Adabas and Natural".
  Confirms free self-paced Basic Essentials tracks exist for Adabas, Natural Programming,
  and NaturalONE, with a Software AG Certified digital badge on completion.
  https://techcommunity.softwareag.com/t/introducing-free-training-essentials-for-adabas-natural/259320
- Natural for Mainframes **First Steps** tutorial overview, 8.2.8 and 9.2.2. The vendor's
  ordered twelve-chapter teaching sequence and the instruction to work it in order.
  https://documentation.softwareag.com/natural/nat828mf/firststeps/fs-over.htm
  https://documentation.softwareag.com/natmf/9.2.2/en/webhelp/natmf-webhelp/firststeps/fs-over.htm
- Natural for Mainframes First Steps, "Loops and Labels" (8.2.7). Shows `REPEAT` introduced as
  a wrapper around `INPUT USING MAP` with `ESCAPE BOTTOM (RP1.)`, and the labelled `READ` loop
  with `*COUNTER (RD1.)` and `REINPUT`.
  https://documentation.softwareag.com/natural/nat827mf/firststeps/fs-loop.htm
- Natural for UNIX First Steps, "User Input" (9.1.2). Confirms the same chapter order on the
  Unix edition and that the `INPUT` statement is taught in-program before the Map Editor.
  https://documentation.softwareag.com/natural/nat912unx/firststeps/fs-input.htm
- Natural **Programming Guide** 9.1.3 (PDF, October 2021). Full table of contents used for the
  vendor's topical framing: Part IV Field Definitions, Part V Database Access, Part VI Report
  Format and Control, Part VII Further Programming Aspects with chapter 44 Conditional
  Processing, chapter 45 Logical Condition Criteria, chapter 46 Loop Processing, chapter 47
  Control Breaks, chapter 52 Processing of Application Errors.
  https://documentation.softwareag.com/natural/nat913win/print/pg.pdf
- Natural Programming Guide, "Loop Processing" (9.1.1 mainframe). Source of the verbatim
  database-loop versus non-database-loop definitions.
  https://documentation.softwareag.com/natural/nat911mf/pg/pg_furth_loop.htm
- Natural Programming Guide overview table of contents (8.2.8 mainframe).
  https://documentation.softwareag.com/natural/nat828mf/pg/pg-over.htm
- **SoftwareAG/adabas-natural-education-package** (GitHub, archived 2023-01-26). Official
  vendor beginner package: Hello World, then conditional constructs with `DECIDE ON` and
  `DISPLAY`, then two-dimensional arrays with nested loops, then Adabas retrieval with a DDM,
  `HISTOGRAM`, `REPEAT`, and `ESCAPE`. Stated audience: beginners.
  https://github.com/SoftwareAG/adabas-natural-education-package
  https://raw.githubusercontent.com/SoftwareAG/adabas-natural-education-package/master/README.md
- **NATURAL Essentials**, Stephen Paul Simpson, version 2.10, January 2000, 291 pages. Full
  table of contents extracted from the PDF. Three parts plus appendices; Part 2 is the
  syntax teaching sequence used in section 2.4 above.
  http://spsimpson.com/nat-u/NATURAL%20Essentials.pdf
- **Verhoef Training**, Adabas Natural Programming, 3 days, on-site only. Fifteen-topic
  ordered outline, audience, and prerequisites.
  https://verhoef-training.co.uk/system-z-programming/adabas-natural-programming
- **Nisa Trainings**, NATURAL ADABAS Training, 25 hours, six modules.
  https://nisa-trainings.com/courses/natural-adabas-training/
- **MaxMunus**, NATURAL ADABAS Training, 20 to 25 hours, live instructor-led.
  https://www.maxmunus.com/page/NATURAL-ADABAS-Training
- **Vistasparks**, NATURAL ADABAS Training, 20+ hours, nine headings.
  https://vistasparks.com/product/natural-adabas-training
- Software AG Learning Portal course listings for Natural Programming Basic (E307A-75E),
  Natural Programming Fundamentals (307-73E and E307-73E), and NaturalONE Basic (E310A-75E).
  Course existence, codes, and scope summaries verified from search result snippets. **The
  course pages themselves could not be read**: they are JavaScript-rendered Moodle shells
  that return a "Loading" placeholder, and `knowledge.softwareag.com` did not resolve.
  https://learn.softwareag.com/course/view.php?id=1426
  https://learn.softwareag.com/course/info.php?id=1467
- Software AG Certified Natural Associate badge (Credly). Foundational level, free,
  requirement is completion of Natural Programming Basic.
  https://www.credly.com/org/software-ag/badge/software-ag-certified-natural-associate

### Sources sought and not obtained

- **idestrainings.com** no longer resolves in DNS. Its Natural Adabas syllabus could not be
  retrieved.
- **igmguru.com** returns HTTP 403 to automated fetches.
- **Koenig Solutions** Adabas page appeared in search results but its published outline
  covers Adabas administration rather than Natural programming, so it was not pursued.
- No **university or college for-credit Natural syllabus** surfaced in any search, which is
  consistent with spike 03's finding that Natural is not taught academically.
- No **government or employer reskilling program** with a published Natural curriculum
  surfaced. Searches returned COBOL reskilling programs (Open Mainframe Project) and Natural
  job postings, but no training curriculum.
