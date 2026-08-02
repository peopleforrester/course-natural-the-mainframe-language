# The course front end

The VTT: lesson instructions on the left, a 3270-styled terminal on the right running the
Natural interpreter compiled to WebAssembly. Everything is client-side; there is no
backend and no per-student cost.

## Running it

```bash
wasm-pack build crates/natural-wasm --target web --out-dir ../../web/pkg --release
cd web && python3 -m http.server 8777
```

Then open http://localhost:8777/.

Any static host works. The page needs no COOP or COEP headers, because the interpreter
suspends and resumes rather than blocking, which is the reason it was built as a state
machine. See `docs/gotchas-rust-wasm.md`.

## What is vendored, and under what license

| Path | What | License |
|------|------|---------|
| `vendor/xterm.js`, `vendor/xterm.css` | `@xterm/xterm` 6.0.0, the scoped package | MIT |
| `fonts/3270-Regular.woff` | `rbanffy/3270font` v3.0.1 | BSD-3-Clause and OFL-1.1-RFN |
| `pkg/` | Build output from `crates/natural-wasm` | this project |

The 3270 font descends from the x3270 font, which was hand-copied from a physical IBM
3270. Its license text is in `fonts/LICENSE-3270font.txt`.

## Layout notes

- The terminal is a fixed **24x80 Model 2** grid with `scrollback: 0` and no fit addon,
  because a real 3270 neither scrolls nor reflows.
- The strip below the grid is the **Operator Information Area**. On a real terminal it
  reports what the machine is doing; here it surfaces interpreter state, so `X SYSTEM`
  means running and `X Program check` means the program failed.
- Green and amber phosphor are both available from the terminal bar.
