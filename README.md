# Natural: The Mainframe Language (Interactive Course)

![A 3270 terminal glowing green in a dark machine room](assets/hero.png)

An interactive, browser-based course teaching the **Natural** programming language, the
Software AG 4GL built as the native language of the ADABAS database and still running
payroll, benefits, and licensing systems in government and insurance.

Lesson instructions sit on the left, and a **3270-styled terminal on the right runs a
Natural interpreter compiled to WebAssembly**. There is no backend, no mainframe, and no
per-student cost. The learner writes real Natural and it executes in their own browser tab.

## Status

**Tiers 1 and 2 are complete and shipped**: modules 1 through 15, an interpreter that
executes them, and the browser front end that delivers them. 314 tests pass, and every
code sample the course publishes is one of them.

| Module | Covered |
|--------|---------|
| 1. What Natural is | orientation, history, where it actually runs |
| 2. Your first program | `WRITE`, `END`, text literals |
| 3. Data and `DEFINE DATA` | formats A, N, P, I, L; lengths; print widths |
| 4. Assignment and computation | `MOVE`, `:=`, `COMPUTE`, `ROUNDED`, the arithmetic verbs |
| 5. Input and decisions | `INPUT`, `IF`/`ELSE`, `DECIDE ON`, `DECIDE FOR` |
| 6. `WRITE` and `DISPLAY` | free-format output versus generated column reports |
| 7. Loops | `FOR`, `REPEAT`, `UNTIL`, `WHILE`, `ESCAPE`, runaway protection |
| 8. Reading the database | `VIEW OF`, `READ`, `FIND`, `WHERE`, `HISTOGRAM`, `*NUMBER` |
| 9. Changing the database | `STORE`, `UPDATE`, `DELETE`, `END TRANSACTION`, `BACKOUT` |
| 10. Validating input | `REINPUT` |
| 11. Subroutines | `DEFINE SUBROUTINE`, `PERFORM`, nesting |
| 12. Data areas | LOCAL versus PARAMETER, and what each one scopes |
| 13. Subprograms | `CALLNAT`, parameter passing, object isolation |
| 14. Maps | `INPUT USING MAP`, `SET KEY`, attribute bytes, AID keys |
| 15. Capstone | a maintenance program combining all of it |

## Running it

```bash
cargo test --workspace                 # 314 tests
cargo run -p natural-cli -- examples/capstone.nat

wasm-pack build crates/natural-wasm --target web --out-dir ../../web/pkg --release
cd web && python3 -m http.server 8777  # then open http://localhost:8777/
```

`scripts/verify.sh` is the full gate: format, lint, tests, the wasm build, and a prose
check. `scripts/install-hooks.sh` wires it to a pre-push hook. This repo runs no hosted CI
by decision.

`scripts/e2e-walkthrough.mjs` drives every lesson step in a real browser and asserts on the
terminal, including the map suspension and the AID keys. It needs Chrome, so it is opt-in
rather than part of the gate:

```bash
node scripts/e2e-walkthrough.mjs      # 15 lessons, 44 runnable steps
```

## Repository layout

| Path | What lives here |
|------|-----------------|
| `crates/natural-core` | The interpreter: lexer, parser, execution, sample database |
| `crates/natural-cli` | Native runner, used for development and testing |
| `crates/natural-wasm` | The browser boundary, a resumable session |
| `web/` | The VTT front end, lessons, vendored xterm.js and 3270 font |
| `examples/` | Standalone Natural programs the CLI runs, exercised by the test suite |
| `research/` | Nine dated research spikes, plus their adversarial verification |
| `spec/` | The approved course specification and Tier 1 lesson outline |
| `docs/gotchas-rust-wasm.md` | Required reading before any Rust or wasm work |
| `docs/content-audit-2026-08.md` | The content audit, its findings, and what was done |

## The two constraints that shaped everything

**Statement execution is an explicit loop with an explicit frame stack, never recursion.**
`INPUT` has to suspend the program and hand control back to the browser, and a recursive
evaluator cannot be paused. This is why the interpreter compiles blocks to a flat
instruction list with jumps, and it is not retrofittable.

Tier 2 is what that constraint bought. `PERFORM` pushes onto an explicit call stack,
`CALLNAT` swaps the whole executing object while keeping the caller in a frame stack, and a
map read suspends on a `Screen`. An `INPUT` two frames deep inside nested subroutines, or
inside a called subprogram, still resumes correctly. None of that would work if any of them
had used Rust recursion.

**The interpreter is ours.** A free vendor Community Edition exists, but its license bars
using it as a paid course's backend, and there is no academic path. Owning the runtime
removes the licensing wall and drives marginal delivery cost to zero.

## Accuracy

Every language behavior the course teaches is checked against official Software AG
documentation, and both the research and the finished lessons were then adversarially
re-checked.

The research pass corrected three retired platforms that would have been taught as current
and a misquoted workforce statistic whose source argues the opposite of how it was being
used. Those corrections are in `research/verification/`.

The content pass in August 2026 went further and checked roughly 160 individual claims
across all fifteen lessons. It refuted 26 of them. The worst was invented syntax: `DEFINE
MAP` does not exist in Natural, and two lessons had been built on it. Others would not have
compiled, including object names past the eight-character limit and a `FIND` whose clauses
were in the undocumented order. One was simply false, and it was a module's central point:
the end of a program is not a transaction boundary. `docs/content-audit-2026-08.md` records
the lot.

What made that possible to miss was that nothing tested the course's own source. It does
now: `crates/natural-core/tests/lesson_samples.rs` runs every published sample through the
interpreter, and the verification gate regenerates its fixture from the lesson content
first. Three lessons teach by failing on purpose, and they are asserted to keep failing.

Two divergences from production Natural remain, deliberately, and the course states each
one on the page where it happens. The sample `EMPLOYEES` file is flattened, where the real
demo file nests salary in a repeating group. And the source format for maps is this
project's own, because a real map is drawn in a screen editor and has no hand-written form
to copy; the statements that use a map are real.

Where documentation could not settle a detail, the code says so at the point of the
decision rather than implying certainty.
