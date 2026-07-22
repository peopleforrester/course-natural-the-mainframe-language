# Course Specification: Natural, The Mainframe Language

Status: DRAFT for approval (Phase 1.2). Not yet approved. Do not begin construction
until Michael seals the contract at Phase 1.3.

Spec date: 2026-07-19. Grounded in the five research spikes in `research/` and the
VTT extraction in `reference/vtt-model/`.

## 1. What this course is

An interactive, browser-based beginner course that teaches the **Natural**
programming language (the Software AG 4GL used with ADABAS). The learner reads
instructions on the left and writes and runs real Natural code in a live terminal
on the right, with nothing to install.

The distinguishing bet: the terminal runs a **custom Natural interpreter compiled
to WebAssembly, entirely in the browser**. No mainframe, no vendor runtime, no
backend, no per-student cost.

## 2. Why (the honest market case)

The research supports building this, but as a specific kind of asset, not a
mass-market course. Facts from the spikes:

- **The audience is a small B2B niche.** Live US openings naming Natural/ADABAS run
  in the low tens, contract-heavy, concentrated in state government and insurance,
  at scarcity rates of roughly $55 to $86/hour (spike 03). This is not COBOL-scale
  enrollment.
- **There is a real skills-gap tailwind.** Retiring mainframe staff and unfilled
  backfill roles are documented, and the vendor has committed support well beyond
  2050 (spike 03). Demand is thin but durable, and the paying audience is
  cross-training COBOL/mainframe developers and employers backfilling retirements.
- **The competition gap is near-total.** No modern interactive beginner Natural
  course exists on Udemy, Pluralsight, Coursera, LinkedIn Learning, or edX. The
  field is vendor reference docs, login-gated vendor LMS courses, multi-day
  instructor-led corporate training, and a 25-year-old free self-study PDF
  (spike 04). We would be first.
- **Correction to the founding premise.** The University of Texas is a major
  operational *user* of Natural/ADABAS (its degree-audit system has run on it since
  1985, and it is the most Natural-heavy organization Sumble tracks), but there is
  no evidence UT teaches Natural as a for-credit course (spike 03). We should market
  on the real story (a live enterprise/government skills gap with no modern
  training), not on a "taught at UT" claim we cannot support.

Implication for investment: build it as a cheap-to-run, evergreen, near-zero-
competition long-tail asset. The WASM architecture is what makes that possible,
because it drives marginal delivery cost to zero. It also doubles as a strong
portfolio and marketing piece regardless of direct course revenue.

## 3. What Natural is (grounding, verified)

- A 4GL first released in **1979** by Software AG (Darmstadt, Germany; company
  founded 1969), designed as the native programming language for the **ADABAS**
  database (spike 01).
- Runs on z/OS, z/VSE, BS2000, and on Linux/Unix/Windows and cloud. NaturalONE is
  the Eclipse-based IDE (spike 01).
- **Current ownership (verified live):** Adabas & Natural is a standalone business
  under **Software GmbH**, owned by Silver Lake, since January 7, 2025. IBM bought
  webMethods and StreamSets from the old Software AG but **not** Adabas & Natural
  (spike 01, spike 04).
- **Current versions (as of July 2026):** two lines, Natural for z/OS 9.2.x (9.2.4)
  and Natural for Linux/Cloud 9.3.x (9.3.3); NaturalONE 9.3.3 (spike 01). The course
  targets language semantics common across current versions, not a single build.
- **Licensing reality that shapes the architecture:** a free Adabas & Natural
  Community Edition (Docker) exists in 2026 but is **personal-use only, commercial
  production prohibited**. Hosting it per-student in a paid course is outside that
  license (spike 05). This is the wall that pushes us to own our runtime.

## 4. Technical architecture

Decision, per spike 05: **build one custom Natural-subset interpreter in Rust,
ship it two ways from a single codebase.**

- **Primary target (ship this): WASM in the browser.** The Rust interpreter
  compiles to `wasm32` and drives an **xterm.js** terminal 100% client-side.
  `INPUT` is handled with a yield/resume state machine (interpreter yields on input,
  resumes when xterm.js delivers a line) to avoid the cross-origin-isolation headers
  that `SharedArrayBuffer` blocking reads would require. The whole course is
  static-hostable (GitHub Pages, a CDN, Railway static, anything). Zero backend,
  zero per-student cost, zero vendor licensing exposure.
- **Fallback (same codebase): native CLI in a ttyd container.** The identical Rust
  interpreter built as a native binary, dropped into the ttyd/nginx VTT from the
  Unleash workshop (`reference/vtt-model/`), for any lesson that outgrows the browser
  sandbox (real filesystem, multi-file programs, larger datasets).
- **Documented but not shipped: paid high-fidelity path.** License Natural for Open
  Systems and run the real runtime plus Adabas in per-student containers. Kept as an
  upgrade path for a client who demands byte-exact fidelity; not the default.
- **Rejected:** hosting the free `natural-ce` image as the course backend (license),
  and a scripted fake terminal (teaches nothing durable).

The front end reuses the VTT split-pane almost verbatim: instructions and
click-to-copy steps on the left, terminal on the right. The only structural change
from the Unleash version is that the right pane is a WASM-backed xterm.js instance
instead of an `<iframe>` to server-side ttyd.

### Reference material we build on (not fork)

- `MarkusAmshove/natls` (MIT): a real Natural parser and language server in Java.
  Not an interpreter and not WASM-friendly, but the authoritative reference grammar
  and test corpus for our parser, and a possible LSP for in-browser syntax
  highlighting later (spike 05).
- `SoftwareAG/adabas-natural-code-samples` (Apache-2.0): official sample programs to
  calibrate the teaching subset and seed exercises and expected-output fixtures.
- No open-source Natural interpreter exists. The execution engine is greenfield
  (spike 05).

## 5. The teaching subset (what the interpreter must execute)

Scoped to the beginner curriculum, per spike 05, calibrated against real syntax
verified in spike 02:

- **Program structure:** `DEFINE DATA LOCAL` / `END-DEFINE`, level numbers, formats
  (`A`, `N`, `P`, `I`, `L`, `D`, `T`) with `(A20)` / `(N7.2)` / `(I4)` notation,
  arrays `(1:10)`, `END`.
- **Assignment and math:** `MOVE`, `COMPUTE`, `:=`, `ADD`, `SUBTRACT`, `MULTIPLY`,
  `DIVIDE`, `RESET`. Honest fixed-point/packed-decimal arithmetic (Rust
  `rust_decimal`), because business arithmetic and edit masks are the point.
- **Control flow:** `IF`/`THEN`/`ELSE`/`END-IF`, `DECIDE ON`, `DECIDE FOR`,
  `FOR`/`END-FOR`, `REPEAT`/`UNTIL`/`WHILE`/`END-REPEAT`, `ESCAPE`, `PERFORM` of
  internal `DEFINE SUBROUTINE`.
- **I/O:** `WRITE`, `DISPLAY`, `PRINT`, prompt-style `INPUT`, basic edit masks
  `(EM=...)`. This is what makes the terminal feel alive.
- **Data access over a sample dataset:** `READ` / `FIND` / `HISTOGRAM` with
  `AT START OF DATA` / `AT END OF DATA`, `END-READ` / `END-FIND`, plus `STORE` /
  `UPDATE` / `DELETE` / `GET` / `END TRANSACTION`, executed against the **classic
  EMPLOYEES / VEHICLES sample data loaded from real fixture rows** (JSON/CSV). This
  is a genuine fixture-backed sample dataset, not a stubbed integration: the teaching
  goal is the Natural verb semantics, and the rows are real. It is explicitly not a
  mock of a live system.
- **System variables:** `*DATX`, `*TIMX`, `*USER`, `*PROGRAM`, `*COUNTER`,
  `*NUMBER`, plus a few intrinsic string functions.

Out of subset for v1 (taught conceptually, or in the optional server lab): full
3270 map/screen I/O, real Adabas internals, JCL/batch, Natural RPC, SPoD, and OO
Natural classes.

## 6. Course layout

Sixteen modules from spike 02, reorganized into three delivery tiers by how the
browser interpreter can honestly support them. This is the key reconciliation
between the curriculum and the WASM architecture.

### Tier 1: fully interactive in the browser (the core course)

Every step is runnable in the WASM terminal.

1. Orientation to Natural and ADABAS, and why it still matters (the skills gap)
2. Your first program and the editor model (RUN vs STOW)
3. Data and the `DEFINE DATA` block (formats A/N/P/I/L/D/T)
4. Assignment and computation (`:=`, `MOVE`, `COMPUTE`)
5. Conditional logic (`IF`, `DECIDE ON`, `DECIDE FOR`)
6. Output (`DISPLAY` vs `WRITE`, edit masks)
7. Loops and control (`FOR`, `REPEAT`, `ESCAPE`)
8. Reading data (`READ`, `FIND`, `HISTOGRAM` over the EMPLOYEES/VEHICLES sample set)
9. Writing data and transactions (`STORE`/`UPDATE`/`DELETE`/`GET`/`END TRANSACTION`)

### Tier 2: interactive with adapted I/O

Runnable in the browser using prompt-style `INPUT` in place of full 3270 screens;
the map concept is taught, the interaction is line-oriented.

10. Interactive input (`INPUT`, `REINPUT` validation; maps explained, run line-mode)
11. Modularization I (inline subroutines, copycode)
12. Data areas (LDA, GDA, PDA) and passing data
13. Modularization II (external subroutines, `CALLNAT` subprograms, functions)

### Tier 3: conceptual, with an optional server-side lab

Read-and-understand content plus an optional ttyd-container lab (the fallback
architecture) for learners who want to touch the real environment.

14. The real environment (libraries, FUSER/FNAT, STOW/CATALL, NaturalONE, SPoD)
15. Capstone: a multi-object application built end to end in the browser
16. Beyond the basics (real 3270 maps, batch/JCL, Natural RPC, OO Natural) as a
    guided reading map toward the vendor docs and the paid tools

Positioning anchor: the course prepares a learner for and goes beyond Software AG's
free **"Natural Programming Basic"** course and its **Certified Natural Associate**
badge (Credly, foundational, free, no proctored exam), which is a validating anchor
rather than a competitor (spike 02, spike 04).

## 7. Build plan (proposed phases, post-approval)

Each phase is a lifecycle unit (test, implement, verify, stage, promote). Rough
sizing from spike 05 (4 to 8 weeks total for a solid interpreter).

- **P1. Interpreter core (Rust CLI).** Lexer, parser for the Tier-1 subset,
  tree-walking interpreter, honest decimal/format semantics, `WRITE`/`DISPLAY`.
  Test-backed against fixtures collected from `adabas-natural-code-samples` and the
  `natls` corpus. Ships as a native CLI first so it is testable in isolation.
- **P2. Sample-data layer.** EMPLOYEES/VEHICLES fixtures and the `READ`/`FIND`/
  `HISTOGRAM`/`STORE`/`UPDATE` verbs over them, plus system variables.
- **P3. WASM + terminal.** `wasm-bindgen` build, xterm.js page, yield/resume `INPUT`
  state machine. First runnable browser lesson end to end.
- **P4. Front end (VTT).** Adapt the split-pane from `reference/vtt-model/` to the
  WASM terminal; instruction rendering, click-to-copy, step progression, per-lesson
  starter code.
- **P5. Content.** Author Tier-1 modules (1 to 9) with runnable steps and checks.
- **P6. Tier 2 and 3, capstone, polish, publish.**

A walking-skeleton milestone (one Tier-1 lesson running end to end in the browser)
should come as early as the end of P3, before authoring all content, to de-risk the
architecture bet.

## 8. Decisions (resolved by Michael, 2026-07-22)

1. **Scope of v1: Tier 1 only (modules 1 to 9).** Ship the fastest path to a live,
   fully interactive course, then extend. Tier 2 follows in a later release.
2. **Interpreter language: Rust, with a scoped research spike first.** A focused
   Rust-to-WASM-interpreter spike runs before P1, per the new-technology
   research-first rule. Rust is confirmed for the WASM toolchain and for exact
   decimal semantics.
3. **Product intent: a revenue course.** Sold B2B, aimed at employers backfilling
   retiring Natural developers and at COBOL/mainframe developers cross-training.
4. **Repo visibility:** stays private through the build. Revisit before launch.

### Recorded tension to manage (raised at approval)

Tier 1 alone is not "job-ready" by spike 02's own definition: modules 1 to 8 get a
learner writing database-touching code, but modules 9 to 14 are what turn that into
job-ready maintenance capability. A B2B buyer is purchasing job-readiness. Therefore
v1 must be positioned and priced as **release 1 of a course that extends into
Tier 2**, not as the finished job-ready product. Marketing copy and pricing must not
promise job-readiness until Tier 2 ships. This constraint is part of the approved
contract.

## 9. Risks

- **Fidelity gap:** our subset is not 100% Natural. Mitigation: beginners do not
  reach the corners where the subset and real Natural diverge, and the subset grows
  over time. Be transparent in the course that it is a teaching interpreter.
- **New-tech risk (Rust/WASM):** adopting Rust triggers the research-first rule.
  Mitigation: a scoped Rust-to-WASM-interpreter spike before P1.
- **Vendor drift:** Silver Lake could change the free CE or docs. Mitigation: the
  shipped architecture depends on neither; docs are cited with access dates.
- **Market size:** thin demand. Mitigation: near-zero delivery cost and first-mover
  position make even a small paying audience worthwhile, and the asset has portfolio
  value independent of course sales.

## 10. Success criteria for Phase 1

- All five research spikes complete and committed (done).
- VTT model extracted and documented (done).
- This spec approved by Michael, with the open questions in section 8 resolved and
  the contract sealed in `decisions.md` and `PROJECT_STATE.md` (pending).
