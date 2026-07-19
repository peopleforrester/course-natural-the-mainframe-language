# Running Natural Without a Mainframe: Emulator, Container, and WASM Feasibility

Spike date: 2026-07-19

## Executive summary

Natural does not need a physical mainframe. The vendor ships a real, off-mainframe
Natural that runs on Linux and Windows, and a free Docker "Community Edition" of that
runtime still exists in 2026. So a genuine Natural runtime in a container is technically
achievable today. The problem is not technical, it is legal: the Community Edition is
licensed for personal use only, with commercial production use explicitly prohibited,
and the paid "Natural for Open Systems" carries a per-seat commercial license from a
single vendor (now a Silver Lake owned standalone business called Software GmbH). A paid
interactive course that hosts a Natural runtime per student is a commercial production
use, which the free edition's license does not cover.

Recommended architecture: **build a small custom interpreter for a teaching subset of
Natural, in Rust, exposed two ways from one codebase: a native CLI binary for a
server-side ttyd/xterm.js terminal, and a WebAssembly build for a 100 percent
client-side, zero-backend browser terminal.** This is option B and option C from the
brief fused into one deliverable, because a single Rust interpreter compiles to both a
native binary and to wasm32 with almost no extra work. The browser build is the primary
target (no backend, no per-student cost, no licensing exposure), and the native CLI is
the fallback for any lesson that outgrows the browser sandbox.

The real vendor runtime (option A) stays as a documented "high-fidelity but not free for
this use" reference, not the shipped architecture, because of the licensing wall.

There is meaningful prior art to build on. `natls` (MarkusAmshove) is an MIT licensed,
actively maintained Natural parser and language server in Java. It is a parser and
linter, not an interpreter, and it is Java so it does not compile to wasm cleanly, but
its grammar coverage and test corpus are a strong reference for what a teaching subset
must handle. The official `SoftwareAG/adabas-natural-code-samples` repo (Apache 2.0)
gives realistic sample programs and patterns to calibrate the subset and to seed lesson
content.

## Decision table

| Option | Fidelity to real Natural | Backend cost | Build effort | Licensing risk |
|---|---|---|---|---|
| A. Official Natural CE in a per-student container | Highest (it is real Natural) | High (container per student, Adabas too) | Low to build, high to operate | High. Free CE is personal-use-only; commercial course hosting is prohibited. Paid license needed. |
| A'. Official Natural for Open Systems, paid, per-student | Highest | High | Medium | Medium to high cost, but legally clean if licensed. Single-vendor dependency. |
| B. Custom CLI interpreter of a teaching subset (server-side) | Medium (subset, our semantics) | Low to medium (thin container, no DB engine) | Medium (4 to 8 weeks for a solid subset) | Low. Our code, our license. No vendor IP. |
| C. Custom interpreter compiled to WASM, browser-only | Medium (same subset) | Zero (fully client-side) | Medium+ (B plus a browser I/O layer) | Low. Same as B. |
| D. Static/scripted "fake" terminal (canned outputs) | Low (illusion only) | Zero | Low | Low, but pedagogically weak and brittle. |

Recommended: **B and C from one Rust codebase**, browser (C) as primary, CLI (B) as
fallback. Keep A' documented as the paid high-fidelity upgrade path.

---

## 1. Official local / non-mainframe Natural

**Yes, off-mainframe Natural exists and is a first-class product.** Software AG has for
years shipped "Natural" for Windows and for UNIX/Linux (often marketed under the "Natural
for Open Systems" umbrella alongside the mainframe edition). The installation
documentation for the Windows and UNIX runtimes is current and versioned (Natural 9.x
lines, with a Natural for Windows 9.3.2 install guide dated July 2025). The UNIX install
needs roughly 600 MB for Natural plus about 200 MB for Natural Security, and a paid
install requires vendor-provided license files. This is a normal commercial product with
a normal commercial license.

**A free Community Edition still exists in 2026, and it is Docker-based.** The vendor page
at softwareag.com/en/developer/adabas-natural-community-edition/ is live and still offers
the "Adabas & Natural Community Edition." It bundles:

- NaturalONE Community Edition (an Eclipse-based IDE)
- Natural Community Edition (the runtime and development environment)
- Adabas Community Edition (the database, with a demo database)
- Adabas Manager Community Edition (web admin UI)

Delivery is via Docker images on Docker Hub and the vendor's public registry. The
`softwareag/natural-ce` image is real and recently maintained: the latest tag observed
was **9.3.3, about 114 MB, last pushed roughly 19 days before this spike (early July
2026)**. Companion images `softwareag/adabas-ce` and `softwareag/adabasmanager-ce` exist,
and AWS Marketplace listings mirror them. So the free editions have not been discontinued
as of 2026-07-19; they are actively updated.

**License terms (the critical constraint).** The Community Edition page states plainly:
"This Community Edition is for personal use only. Use for commercial production purposes
is prohibited." Access requires registration. Separately, the Docker Hub image gates the
pull behind a "Limited Use License Agreement" that grants a non-exclusive,
non-transferable license, prohibits sublicensing and redistribution, and prohibits
reverse engineering. Read together, an individual learner may legally run the CE on their
own machine for self-education. A company running the CE runtime as the backend of a paid
course, one instance per paying student, is a commercial production use and a
redistribution-adjacent hosting scenario that the free license does not authorize. That
is the wall.

**Ownership change is real and matters.** As of January 7, 2025, Adabas & Natural was
spun out as a standalone business under the holding entity "Software GmbH," owned by the
investment firm Silver Lake (the same restructuring that saw webMethods, StreamSets, and
TrendMiner sold to IBM in July 2024, and Alfabet and Cumulocity divested). Adabas &
Natural and ARIS now each run as independent, separately managed companies. Practical
consequences for this course:

- The product is not discontinued; there was an October 2025 release, and the CE Docker
  images are current (9.3.3 in mid 2026). Continuity is good in the near term.
- But roadmap, pricing, and the continued existence of the free CE now sit with a
  PE-owned standalone whose incentives can change. Do not architect the course so that it
  breaks the day the free CE is pulled. This is another argument for owning our runtime.

**Bottom line for scope item 1:** A free, Dockerized, non-mainframe Natural runtime
exists in 2026 and an individual can legally use it to learn. It is not licensed for the
course to host per student commercially without a paid agreement.

## 2. Containerized runtime feasibility (ttyd model)

Technically this works and is the highest-fidelity option. `natural-ce` is a small image
(about 114 MB), and Natural itself is lightweight to run. Wrapping it in a ttyd container
(ttyd is a C tool built on libwebsockets that serves an xterm.js terminal over
WebSocket, and it explicitly supports one-container-per-client Docker patterns) gives the
exact "instructions left, live terminal right" model the course wants.

Caveats:

- **You usually need two containers, not one.** Natural's data access verbs (READ, FIND,
  HISTOGRAM, STORE, UPDATE) target Adabas. To exercise them for real you also run
  `adabas-ce` with its demo database, so the per-student footprint is Natural plus Adabas
  plus the ttyd wrapper. Memory and startup time are dominated by Adabas, not Natural.
  Budget a few hundred MB of RAM per active student and non-trivial cold-start time. That
  is the operational cost line in the decision table.
- **Licensing is the blocker, not resources.** Per section 1, hosting the free CE as the
  backend of a paid course is outside the CE license. Doing it legally means buying
  "Natural for Open Systems" seats (option A'), which is defensible for fidelity but adds
  recurring cost and a hard single-vendor dependency to a Silver Lake owned standalone.
- **Isolation and security.** A real shell-adjacent runtime per student needs the usual
  container hardening (no host mounts, CPU/memory caps, network egress off, ephemeral
  filesystems, per-session teardown). ttyd's "new container per client" mode plus an
  orchestrator handles this, but it is real ops surface.

Verdict: keep option A' as the "if a client pays for maximum fidelity" upgrade. Do not
make it the default shipped architecture for a self-serve course.

## 3. Build-your-own CLI emulator (a teaching subset)

Feasible and, for a beginner curriculum, the sweet spot. Natural is a verbose,
statement-oriented 4GL. A beginner course does not need the whole language; it needs a
coherent subset with honest semantics.

**Recommended teaching subset (covers a beginner curriculum):**

- Program structure: `DEFINE DATA LOCAL` ... `END-DEFINE`, level numbers (`01`, `02`),
  types (`A` alpha, `N`/`P` numeric with `(An)`, `(N7.2)` style formats), arrays
  `(1:10)`, and `END`.
- Assignment and math: `MOVE`, `COMPUTE`, `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE`,
  `RESET`.
- Control flow: `IF`/`THEN`/`ELSE`/`END-IF`, `DECIDE FOR`/`DECIDE ON`, `FOR`/`END-FOR`,
  `REPEAT`/`UNTIL`/`WHILE`/`END-REPEAT`, `ESCAPE`, `PERFORM` of internal subroutines
  (`DEFINE SUBROUTINE`).
- I/O: `WRITE`, `DISPLAY`, `PRINT`, `INPUT` (map-free, prompt style), and basic edit
  masks `(EM=...)`. This is what makes the terminal feel alive.
- Data access, mocked: `READ`/`FIND`/`HISTOGRAM` over a small in-memory sample "database"
  loaded from JSON or CSV fixtures, with `AT START OF DATA`/`AT END OF DATA`,
  `END-READ`/`END-FIND`, and system variables like `*COUNTER`, `*NUMBER`. Mocking the DB
  is legitimate here because the teaching goal is the Natural verb semantics, not real
  Adabas internals. (Note: the global rules discourage "mock modes," but a fixture-backed
  sample dataset for a language teaching tool is genuine functionality, not a stubbed-out
  real integration. Call it a sample dataset, not a mock, and load real fixture rows.)
- System variables and functions students meet early: `*DATX`, `*TIMX`, `*USER`,
  `*PROGRAM`, plus string handling and a few intrinsic functions.

That subset covers the first several modules of any "Natural basics" path (compare the
vendor's own "Natural Programming Basic" course outline and the
`adabas-natural-code-samples` patterns: DEFINE DATA, arrays, date/time math, string
manipulation, sort, and simple DB reads).

**Rough effort.** For an experienced compiler-literate engineer:

- Lexer plus Pratt/recursive-descent parser for the subset: about 1 to 2 weeks.
- Tree-walking interpreter with the Natural data model (levels, formats, edit masks,
  fixed-point/packed decimal semantics done honestly): about 2 to 3 weeks. The
  fixed-point decimal and edit-mask formatting are the fiddly parts, not the control flow.
- Sample-data READ/FIND layer plus system variables: about 1 week.
- Terminal I/O behavior (WRITE/DISPLAY column formatting, INPUT prompting): about 1 week.

Call it **4 to 8 weeks** for a solid, test-backed subset interpreter, faster if the subset
is trimmed. This is the dominant cost of options B and C, and it is a one-time cost we own
forever.

**Existing open source to build on (searched GitHub specifically):**

- **`MarkusAmshove/natls`** (a.k.a. "NatLS"). The most important find. An MIT licensed
  Natural language server and parser in Java. Latest release v0.18 on 2026-01-12, about
  2,043 commits, clearly active. Modules: `natparse` (the parser), `natlint` (linter),
  `natgen` (code generation), `natls` (LSP server), `natqube` (SonarQube plugin). It is a
  parser/linter/LSP, **not an interpreter**, and it is Java (so not a direct wasm path).
  How to use it: as a **reference grammar and test corpus** for our own parser, and as a
  ready-made LSP if we ever want in-browser syntax highlighting and diagnostics for the
  editor pane. MIT license means we can read and reuse liberally. Its own docs note the
  parser is still incomplete for context-sensitive constructs and does not yet handle
  Reporting Mode, which tells us exactly where Natural's hard parsing corners are.
- **`SoftwareAG/adabas-natural-code-samples`**. Official, Apache 2.0, about 138 commits,
  60+ pattern folders (DEFINE DATA basics, array sort, date/time math, string handling,
  DB operations). Not an interpreter, but an excellent, license-clean source of realistic
  sample programs to calibrate the subset and to seed lesson exercises and expected
  outputs.
- **No open-source Natural *interpreter* or *emulator* exists.** Repeated GitHub and web
  searches surfaced parsers, language servers, code samples, and unrelated "natural
  language" projects, but nothing that executes Natural. There is also no published ANTLR
  grammar for Software AG Natural in `antlr/grammars-v4`. So the execution engine is
  genuinely greenfield; we are not duplicating an existing runtime, and `natparse` is the
  closest prior art for the front end.

Verdict: option B is realistic and de-risked by strong reference material. We write the
interpreter; we borrow grammar knowledge and sample programs.

## 4. WASM / browser-only path

**Compiling the official runtime to wasm: not viable.** Natural CE is closed source,
distributed only as Docker images under a limited-use license that prohibits reverse
engineering. There is no legal or practical path to a browser wasm build of the vendor
runtime.

**Compiling our own interpreter to wasm: straightforward and recommended.** This is the
whole reason to pick a systems language for option B. If the interpreter from section 3 is
written in **Rust**, the same codebase compiles to `wasm32-unknown-unknown` via
`wasm-pack`/`wasm-bindgen` with essentially no algorithmic changes. The only browser-
specific work is the I/O boundary:

- Output (WRITE/DISPLAY) becomes calls that push strings into an **xterm.js** terminal.
- Input (INPUT) is the one genuinely fiddly part, because wasm has no blocking stdin.
  Standard solutions: drive the interpreter as a resumable state machine that yields on
  INPUT and resumes when xterm.js delivers a line, or run the interpreter in a Web Worker
  and use `Atomics.wait` on a `SharedArrayBuffer` for synchronous-looking blocking reads.
  The state-machine approach avoids the cross-origin-isolation headers that
  `SharedArrayBuffer` requires, so prefer it for a static-hosted course.

**Prior art that this is a solved shape:**

- **xterm.js** is the de facto web terminal (used by VS Code's terminal, and by
  interactive course platforms like hack.courses, LabEx, and Next Tech). Pairing an
  interpreter with xterm.js is well-trodden.
- **`cryptool-org/wasm-webterm`** is an xterm.js addon that runs WASI/Emscripten wasm
  binaries in a browser terminal, proof that "wasm program plus xterm.js in the browser"
  is a supported pattern.
- **`segeljakt/xterm-js-rs`** gives Rust/wasm bindings to xterm.js directly, which is
  close to the exact stack recommended here.
- Numerous toy-language interpreters already run this way (Rust-to-wasm and Go-to-wasm
  interpreters wired to browser terminals). The pattern is boring, which is what you want.

**Language choice for the interpreter.** Recommend **Rust**:

- Best-in-class wasm toolchain (`wasm-pack`, `wasm-bindgen`), small binaries, no GC
  runtime to ship.
- Exact fixed-point/decimal arithmetic (crates like `rust_decimal`) matters for Natural's
  packed/numeric semantics and edit masks.
- One codebase yields both the native CLI (option B) and the wasm module (option C).

Alternatives and why not: **Go** compiles to wasm but ships a larger runtime and a heavier
binary; fine but less lean. **AssemblyScript** keeps everything in TypeScript-land and is
attractive if the team is JS-only, but its decimal/number story is weaker for a language
whose whole point is business arithmetic. **C/Emscripten** works but is the least pleasant
to maintain. Reusing `natls` (Java) in the browser would mean TeaVM/CheerpJ gymnastics for
a codebase that is a parser not an interpreter; not worth it. Rust wins.

Verdict: option C is the primary shipped target. Same Rust interpreter, a thin
wasm-bindgen I/O shim, xterm.js on the page, no backend at all.

## 5. Recommendation

**Primary architecture (ship this):** a custom Natural-subset interpreter written in
**Rust**, compiled to **WebAssembly**, driving an **xterm.js** terminal entirely in the
browser. Instructions render on the left, the wasm-backed terminal runs on the right, and
the whole course is static-hostable (a CDN, GitHub Pages, anything). Zero backend, zero
per-student cost, zero vendor licensing exposure, and it cannot be broken by a future
change to the free CE. INPUT is handled with the yield/resume state-machine pattern to
avoid `SharedArrayBuffer` header requirements.

**Fallback (same codebase, for lessons that outgrow the browser):** the identical Rust
interpreter built as a **native CLI binary**, dropped into a **ttyd/xterm.js container**
server-side. This covers anything that wants a real filesystem, larger sample datasets, or
multi-file programs, at a modest hosting cost, and it reuses 100 percent of the
interpreter. This is option B, kept warm as an escape hatch.

**Documented upgrade path (do not ship by default):** for a client who demands byte-exact
fidelity to real Natural, license **Natural for Open Systems** (paid) and run the official
runtime plus **Adabas** in per-student ttyd containers (option A'). This is legally clean
only with a paid license; the free Community Edition is not licensed for commercial
per-student hosting.

**Explicitly rejected:** hosting the free `natural-ce` Docker image as the course backend.
It is the fastest path to a real runtime and it is legally off-limits for a paid course.
Also rejected: a purely scripted fake terminal (option D), which teaches nothing durable.

**The core tradeoff, stated plainly:** we trade some fidelity (a curated subset with our
own honest semantics, not 100 percent of real Natural) in exchange for zero backend cost,
zero licensing risk, and full ownership of the runtime. For a beginner course that is the
right trade, because beginners never reach the corners of the language where our subset
and real Natural diverge, and we can grow the subset over time. Fidelity where it counts
(DEFINE DATA, the control-flow verbs, WRITE/DISPLAY/INPUT, and mocked READ/FIND over
sample data) is entirely achievable in the interpreter.

**Suggested first steps:**

1. Freeze the teaching subset (section 3) against the first N modules of the curriculum.
2. Read `natparse` in `natls` and the `adabas-natural-code-samples` programs to lock the
   grammar and to collect expected-output fixtures.
3. Build the Rust lexer/parser/interpreter CLI first (testable in isolation), then add the
   wasm-bindgen I/O shim and xterm.js page. One codebase, two targets.

---

## Sources

- [Adabas & Natural Community Edition | Software AG](https://www.softwareag.com/en/developer/adabas-natural-community-edition/) - confirms the free Community Edition still exists in 2026, is Docker-based, bundles NaturalONE/Natural/Adabas/Adabas Manager, and is licensed "for personal use only. Use for commercial production purposes is prohibited." Requires registration. Accessed 2026-07-19.
- [softwareag/natural-ce - Docker Hub](https://hub.docker.com/r/softwareag/natural-ce) - the Natural CE runtime image; latest tag 9.3.3, ~114 MB, last pushed ~19 days before the spike (early July 2026); pull gated behind a Limited Use License Agreement (non-exclusive, non-transferable, no redistribution/sublicensing, no reverse engineering). Accessed 2026-07-19.
- [softwareag/adabas-ce - Docker Hub](https://hub.docker.com/r/softwareag/adabas-ce) - companion Adabas CE database image needed to exercise READ/FIND for real. Accessed 2026-07-19.
- [Adabas & Natural Community Edition Guide (Oct 2024, v1.3, PDF)](https://softwareag-usa.s3.amazonaws.com/Adanat_Docker/AN+Community+Edition+Guide.pdf) - official guide; personal-use-only terms, Windows 10/11 and Linux x86-64 support, Docker prerequisite. Accessed 2026-07-19.
- [Adabas & Natural and ARIS launch as standalone businesses (Software AG blog)](https://www.softwareag.com/en/blog/insights/adabas-natural-and-aris-launch-as-standalone/) - Adabas & Natural is now a standalone business under holding entity "Software GmbH," owned by Silver Lake; announcement dated January 7, 2025. Establishes the ownership change and single-vendor dependency. Accessed 2026-07-19.
- [Software GmbH press release (PR Newswire)](https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html) - corroborates the standalone split and the Alfabet/Cumulocity divestitures. Accessed 2026-07-19.
- [Adabas & Natural - Release information Oct. 2025 (Software AG Tech Community)](https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504) - evidence the product line is still actively released in late 2025. Accessed 2026-07-19.
- [Natural Installation Version 9.3.2 July 2025 (PDF)](https://documentation.softwareag.com/natwin/9.3.2/en/webhelp/natwin-webhelp/pdf/install.pdf) - current Windows install guide; confirms a supported off-mainframe Windows runtime and paid license-file requirement. Accessed 2026-07-19.
- [Installing and Setting Up Natural on UNIX](https://documentation.softwareag.com/natural/nat6314unx/install/inst-prod.htm) - UNIX/Linux install; ~600 MB Natural + ~200 MB Natural Security footprint. Accessed 2026-07-19.
- [MarkusAmshove/natls - GitHub](https://github.com/MarkusAmshove/natls) - MIT licensed Natural parser/linter/language server in Java; latest release v0.18 (2026-01-12), ~2,043 commits, active; modules natparse/natlint/natgen/natls/natqube; parser only, no interpreter, no wasm. Primary reference for grammar and test corpus. Accessed 2026-07-19.
- [SoftwareAG/adabas-natural-code-samples - GitHub](https://github.com/SoftwareAG/adabas-natural-code-samples) - official, Apache-2.0, ~138 commits, 60+ pattern folders of real Natural sample programs; source of realistic exercises and expected outputs. Accessed 2026-07-19.
- [xterm.js - GitHub](https://github.com/xtermjs/xterm.js/) and [xtermjs.org](https://xtermjs.org/) - the standard web terminal component for the right-hand pane. Accessed 2026-07-19.
- [ttyd - Share your terminal over the web](https://tsl0922.github.io/ttyd/) - C/libwebsockets tool serving xterm.js over WebSocket, supports per-client Docker containers; the server-side fallback model. Accessed 2026-07-19.
- [cryptool-org/wasm-webterm - GitHub](https://github.com/cryptool-org/wasm-webterm) - xterm.js addon that runs WASI/Emscripten wasm binaries in a browser terminal; prior art for the wasm-in-browser terminal pattern. Accessed 2026-07-19.
- [segeljakt/xterm-js-rs - GitHub](https://github.com/segeljakt/xterm-js-rs) - Rust/WebAssembly bindings to xterm.js; close match to the recommended Rust+wasm+xterm.js stack. Accessed 2026-07-19.
- [appcypher/awesome-wasm-langs - GitHub](https://github.com/appcypher/awesome-wasm-langs) - curated list confirming the maturity of Rust/Go/AssemblyScript to-wasm toolchains for language work. Accessed 2026-07-19.
- [Natural Programming Basic (Software AG Learn)](https://learn.softwareag.com/course/info.php?id=1467) - vendor beginner course outline; used to calibrate the teaching subset scope. Accessed 2026-07-19.

### GitHub projects found (license and last activity)

| Project | What it is | License | Last activity |
|---|---|---|---|
| [MarkusAmshove/natls](https://github.com/MarkusAmshove/natls) | Natural parser, linter, and LSP (Java). No interpreter, no wasm. | MIT | v0.18 released 2026-01-12; ~2,043 commits; active |
| [SoftwareAG/adabas-natural-code-samples](https://github.com/SoftwareAG/adabas-natural-code-samples) | Official Natural sample programs / patterns | Apache-2.0 | ~138 commits; no releases; moderate activity |
| [xtermjs/xterm.js](https://github.com/xtermjs/xterm.js/) | Web terminal component | MIT | Actively maintained (used by VS Code) |
| [tsl0922/ttyd](https://tsl0922.github.io/ttyd/) | Web terminal server over WebSocket | MIT | Actively maintained |
| [cryptool-org/wasm-webterm](https://github.com/cryptool-org/wasm-webterm) | xterm.js addon to run wasm (WASI/Emscripten) binaries in-browser | MIT | Maintained; xterm.js v4-era addon |
| [segeljakt/xterm-js-rs](https://github.com/segeljakt/xterm-js-rs) | Rust/WASM bindings to xterm.js | MIT/Apache-2.0 (dual, typical Rust) | Community-maintained crate |
| [appcypher/awesome-wasm-langs](https://github.com/appcypher/awesome-wasm-langs) | Curated list of languages/VMs targeting wasm | CC0 / list | Community-maintained |

Note: no open-source Natural *interpreter* or *emulator* was found on GitHub, and no
Software AG Natural grammar exists in `antlr/grammars-v4`. The execution engine is
greenfield; `natparse` (inside natls) is the closest reusable front-end reference.
