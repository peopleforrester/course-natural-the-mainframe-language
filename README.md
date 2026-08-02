# Natural: The Mainframe Language (Interactive Course)

An interactive, browser-based course teaching the **Natural** programming language, the
Software AG 4GL built as the native language of the ADABAS database and still running
payroll, benefits, and licensing systems in government and insurance.

Lesson instructions sit on the left, and a **3270-styled terminal on the right runs a
Natural interpreter compiled to WebAssembly**. There is no backend, no mainframe, and no
per-student cost. The learner writes real Natural and it executes in their own browser tab.

## Status

**Tier 1 is complete and shipped**: modules 1 through 9, an interpreter that executes them,
and the browser front end that delivers them. 223 tests pass.

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

Tier 2 (modularization, data areas, `CALLNAT`, real 3270 maps) is a later release. See
`spec/course-spec.md` for the approved scope.

## Running it

```bash
cargo test --workspace                 # 223 tests
cargo run -p natural-cli -- examples/capstone.nat

wasm-pack build crates/natural-wasm --target web --out-dir ../../web/pkg --release
cd web && python3 -m http.server 8777  # then open http://localhost:8777/
```

`scripts/verify.sh` is the full gate: format, lint, tests, the wasm build, and a prose
check. `scripts/install-hooks.sh` wires it to a pre-push hook. This repo runs no hosted CI
by decision.

## Repository layout

| Path | What lives here |
|------|-----------------|
| `crates/natural-core` | The interpreter: lexer, parser, execution, sample database |
| `crates/natural-cli` | Native runner, used for development and testing |
| `crates/natural-wasm` | The browser boundary, a resumable session |
| `web/` | The VTT front end, lessons, vendored xterm.js and 3270 font |
| `research/` | Nine dated research spikes, plus their adversarial verification |
| `spec/` | The approved course specification and Tier 1 lesson outline |
| `docs/gotchas-rust-wasm.md` | Required reading before any Rust or wasm work |

## The two constraints that shaped everything

**Statement execution is an explicit loop with an explicit frame stack, never recursion.**
`INPUT` has to suspend the program and hand control back to the browser, and a recursive
evaluator cannot be paused. This is why the interpreter compiles blocks to a flat
instruction list with jumps, and it is not retrofittable.

**The interpreter is ours.** A free vendor Community Edition exists, but its licence bars
using it as a paid course's backend, and there is no academic path. Owning the runtime
removes the licensing wall and drives marginal delivery cost to zero.

## Accuracy

Every language behaviour the course teaches is verified against official Software AG
documentation, and the research behind it was adversarially re-checked. The verification
pass in `research/verification/` corrected real errors, including three retired platforms
that would have been taught as current and a misquoted workforce statistic whose source
argues the opposite of how it was being used.

Where documentation could not settle a detail, the code says so at the point of the
decision rather than implying certainty.
