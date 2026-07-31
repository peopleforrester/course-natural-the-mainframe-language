# Gotchas: Rust, WebAssembly, and xterm.js

Read this before writing code. Every item is a trap that older tutorials, blog
posts, and model training data still teach the wrong way. Full reasoning and
citations are in `research/06-rust-wasm-toolchain.md` (spike date 2026-07-22).

All versions below verified as of 2026-07-22 against the linked source.

---

## npm package names

* **xterm.js moved to scoped packages.** The unscoped packages carry a formal npm
  deprecation notice ("This package is now deprecated. Move to @xterm/xterm
  instead"), verified via `registry.npmjs.org` on 2026-07-22.

  | Wrong but common | Correct current | Version |
  |---|---|---|
  | `xterm` (5.3.0, 2023) | `@xterm/xterm` | 6.0.0 |
  | `xterm-addon-fit` | `@xterm/addon-fit` | 0.11.0 |
  | `xterm-addon-web-links` | `@xterm/addon-web-links` | 0.12.0 |
  | `xterm-addon-webgl` | `@xterm/addon-webgl` | 0.19.0 |

* **The CSS import path is scoped too.** Wrong: `import 'xterm/css/xterm.css'`.
  Correct: `import '@xterm/xterm/css/xterm.css'`.

* **Do not add `@xterm/addon-canvas`.** Its latest is 0.7.0 from 2024-04-05 and
  it did not ride the 6.0.0 release train. Use the WebGL addon and let xterm.js
  fall back to its built-in DOM renderer.

## xterm.js usage

* **Load the WebGL addon after `term.open()`, not before.** It attaches to the
  already-rendered element. Wrap it in try/catch so a machine without WebGL2
  degrades instead of throwing.

* **A bare `\n` produces a staircase.** xterm.js is a real terminal emulator: line
  feed without carriage return moves down but not left. Either set
  `convertEol: true` in the `Terminal` options or emit `\r\n` from Rust. Pick one
  and enforce it in the wasm boundary layer, not scattered through the
  interpreter.

* **Do not assert on the DOM in browser tests.** The WebGL renderer draws to a
  canvas, so there is no text in the DOM. Use the terminal's buffer API.

## Rust toolchain

* **Current stable is Rust 1.97.1, released 2026-07-16** (per
  `static.rust-lang.org/dist/channel-rust-stable.toml`, accessed 2026-07-22).
  Ships Cargo 0.98.0.

* **Use `edition = "2024"`.** Stabilized in Rust 1.85.0 on 2025-02-20 and still
  the current edition. Greenfield crate, so `cargo fix --edition` is irrelevant.
  Wrong but common: `edition = "2021"` copied from an old template.

* **Commit a `rust-toolchain.toml` pinning the exact patch version** and listing
  `wasm32-unknown-unknown` under `targets`, so the target installs automatically
  and a learner following this material next year gets a reproducible build.

## Wasm target selection

* **Build for `wasm32-unknown-unknown`.** That is the browser target.

  | Wrong for us | Why |
  |---|---|
  | `wasm32-wasip1` / `wasm32-wasip2` | WASI targets assume a POSIX-ish host. Browsers are not one. Running them in a browser needs a JS WASI shim, which reintroduces the blocking-stdin problem anyway. |
  | `wasm32-unknown-emscripten` | Drags in the Emscripten toolchain for no benefit. |
  | `wasm32v1-none` | Tier 2 without host tools, `no_std` only. |

* **The component model and `cargo-component` are not the browser path today.**
  Browsers do not load components natively; a component must be transpiled back
  to core wasm plus JS glue via `jco` first. `cargo-component`'s latest release
  is 0.21.1 from 2025-03-18. Do not adopt it for this project.

## wasm-bindgen and wasm-pack

* **wasm-pack is NOT abandoned, and this is the claim most likely to be stale in
  training data.** It went fifteen months without a release (0.13.1 on
  2024-10-29), then 0.14.0 landed 2026-01-20 under new community maintainers and
  0.15.0 on 2026-05-15. The repo moved to `wasm-bindgen/wasm-pack`, the same org
  as wasm-bindgen. Verified via crates.io and the wasm-pack CHANGELOG on
  2026-07-22.

* **Install wasm-pack with cargo, not npm.** Wrong: `npm install -g wasm-pack`,
  which was returning a 404 until 0.15.0 fixed a stale installer URL from a
  previous maintainer. Correct: `cargo install wasm-pack --locked`.

* **The `wasm-bindgen` crate version and the `wasm-bindgen-cli` version must
  match exactly.** A mismatch gives a confusing schema-version error. Both are at
  0.2.126 (published 2026-06-24). Using `wasm-pack` avoids this class of failure
  because it fetches a matching CLI.

* **Use `--target web`, not the wasm-pack default.** Wrong for a static site:
  `wasm-pack build` (defaults to `--target bundler`, which requires a bundler to
  resolve). Correct:

  ```bash
  wasm-pack build crates/natural-wasm --target web --out-dir ../../web/pkg --release
  ```

* **The wasm crate needs `crate-type = ["cdylib", "rlib"]`.** With `cdylib`
  alone, `cargo test` cannot link that crate natively.

* **The documentation moved.** Wrong URL: `rustwasm.github.io/docs/wasm-bindgen/`,
  which now says it is no longer maintained at that domain. Correct:
  `wasm-bindgen.github.io/wasm-bindgen/`.

* **What the rustwasm sunset actually said** (corrected on 2026-08-01; an earlier
  version of this doc got it backwards). The `rustwasm` GitHub org was sunset on
  2025-07-21. Only **wasm-bindgen** was transferred to a new maintainer org. The
  announcement did **not** say wasm-pack and gloo were transferred; it invited
  users to fork them. wasm-pack was subsequently picked up by new community
  maintainers and now lives under the `wasm-bindgen` org, which is why it is
  healthy again today. Do not repeat the claim that everything was simply
  "transferred, not deprecated".

* **Deprecated wasm-bindgen patterns still shown in tutorials:**
  * `JsStatic` (deprecated 0.2.93). Use `#[wasm_bindgen(thread_local_v2)]`.
    Careful here: plain `#[wasm_bindgen(thread_local)]` was ITSELF deprecated in
    0.2.96, so a tutorial (or an earlier version of this doc) recommending it is
    also stale. `thread_local_v2` is the current attribute.
  * The `--weak-refs` CLI flag (deprecated 0.2.91). Weak reference support is
    detected at runtime; passing the flag is dead weight.
  * `JsOption<T>` changed in 0.2.123 so only `undefined` is empty and `null` is a
    distinct present value. Avoid it; pass explicit tagged structures.

* **`println!` output in wasm tests is swallowed by default** since 0.2.100. Add
  `--nocapture` when you need to see it.

## Blocking INPUT

* **Do not reach for `SharedArrayBuffer` plus `Atomics.wait`.** It works, but it
  costs you your hosting options. Requirements, all verified 2026-07-22:
  * `Atomics.wait()` throws `TypeError` on the main thread (MDN). A Web Worker
    is mandatory.
  * `SharedArrayBuffer` requires cross-origin isolation, meaning
    `Cross-Origin-Opener-Policy: same-origin` and
    `Cross-Origin-Embedder-Policy: require-corp` on the document.
  * **GitHub Pages cannot set custom headers.** Only the `coi-serviceworker`
    hack works there, and it forces a full page reload on a visitor's first
    load. Netlify (`netlify.toml` or `_headers`) and Cloudflare Pages
    (`_headers`) can set them. Railway static: unverified.
  * `require-corp` also breaks every cross-origin subresource (fonts, CDN
    images, analytics) that does not send `Cross-Origin-Resource-Policy`.

* **Do not design around JSPI yet, but the picture is moving.** JavaScript
  Promise Integration is the right long-term answer for a synchronous `INPUT`. It
  reached Phase 4 in April 2025, shipped in Chrome 137, and (corrected on
  2026-08-01) shipped **unflagged in Firefox 153 on 2026-07-21**, not behind a
  flag as this doc previously said. Safari remains the sole blocker: `false` on
  desktop and iOS through 26.5, putting global support near 67 percent. It is an
  Interop 2026 focus area. Revisit when Safari ships; until then the state
  machine below is still the correct choice.

* **Correct approach for this project: a resumable state machine.** The
  interpreter runs until it needs input, returns a `NeedsInput` step to JS, and
  resumes when `provide_input(line)` is called. No headers, no workers, no
  threads, works on every host and every browser, and testable natively with
  `cargo test`.

* **Design constraint this imposes, and it is not retrofittable:** statement
  execution must be an explicit loop with an explicit frame stack, not a
  recursive tree-walking `eval` that uses the Rust call stack. A recursive
  evaluator cannot be paused. Recursive *expression* evaluation is fine, because
  `INPUT` only occurs at statement level. Decide this before writing `interp.rs`.

## Decimal arithmetic

* **Use `rust_decimal` 1.42.1** (published 2026-06-12). Pure Rust, no C
  dependencies, `no_std` capable including bare-metal targets, MSRV 1.67.1. It
  stores an explicit base-10 scale, which is what maps onto Natural's `N7.2`.

* **Do NOT enable rust_decimal's `wasm` feature.** It is not needed to compile to
  wasm. It only adds `wasm-bindgen` glue exposing `fromNumber`/`toNumber` to JS.
  Our decimals should cross the wasm boundary as already-formatted strings,
  because the Natural edit mask is our formatting authority, not JavaScript's.

* **Do not use `fixed`.** Despite the name, it is *binary* fixed-point
  (`I32F32`), which cannot represent 0.1 exactly. It is a frequent wrong answer
  to a search for "Rust fixed point" and it defeats the entire purpose.

* **Do not use `bigdecimal`.** Arbitrary precision is the wrong model for fields
  with a declared precision and scale, and it heap-allocates in the interpreter
  hot loop.

* **Precision ceiling:** rust_decimal gives about 28 significant digits (29 in
  some cases). Cap the teaching subset's declared precision well inside that and
  make exceeding it a clean interpreter error. If a genuine need for more
  precision appears, `fastnum` 0.7.5 (`D256`) is the documented escape hatch.

* **Verify the wasm build on day one.** rust_decimal has no published
  `wasm32-unknown-unknown` CI entry that I could find; the compatibility claim is
  inferred from it being pure Rust. Run
  `cargo build -p natural-core --target wasm32-unknown-unknown` as the first
  task and settle it.

* **Edit masks are ours to write.** No crate implements Natural `EM=ZZZ,ZZ9.99`.
  It is a pure function from (value, mask) to string, so write the test table
  first.

## Serving and hosting

* **`.wasm` must be served as `application/wasm`.** MDN: "For this to work,
  `.wasm` files should be returned with an `application/wasm` MIME type by the
  server." The nasty part is that wasm-bindgen's `--target web` glue falls back
  from `instantiateStreaming` to `arrayBuffer()` plus `instantiate()`, so a wrong
  MIME type often produces a console warning and a slower, more memory-hungry
  load rather than a clean failure. Silent degradation hides the bug.
  * GitHub Pages, Netlify, and Cloudflare Pages get this right by default.
  * Usual offenders: hand-rolled dev servers, nginx or Apache with an old
    `mime.types` (add `application/wasm wasm;`), and S3 objects whose
    `Content-Type` was set at upload.

* **A strict CSP blocks WebAssembly.** MDN: without `'wasm-unsafe-eval'` in
  `script-src`, "WebAssembly is blocked from loading and executing on the page."
  * Wrong: `script-src 'self'`
  * Correct: `script-src 'self' 'wasm-unsafe-eval'`
  * Prefer `'wasm-unsafe-eval'` over `'unsafe-eval'`; the latter also allows
    JavaScript `eval()` and supersedes the former, so never list both.
  * xterm.js may need `style-src 'unsafe-inline'`. Unverified for 6.0.0; start
    permissive and tighten with a test.

* **We do NOT need COOP or COEP**, because of the state-machine choice above.
  That is the whole point: every static host works with default configuration.

## Crates that look right and are not

* **Do NOT adopt `segeljakt/xterm-js-rs`** despite it looking like an exact match
  for a Rust plus wasm plus xterm.js stack. Its last crate release is **0.1.2 from
  November 2021**, so it binds **xterm.js 4.x** while this project targets
  `@xterm/xterm` **6.0.0**. Two major versions of drift in the very API it wraps.
  It is also MIT only, not the usual Rust dual license. Write the small amount of
  `wasm-bindgen` glue by hand against the current xterm.js API instead.
  (Added 2026-08-01; an earlier spike described this crate as "close to the exact
  stack recommended here", which would have been a costly detour.)

* `cryptool-org/wasm-webterm` is **Apache-2.0**, not MIT as previously recorded, and
  it is an xterm.js v4-era addon. Useful as prior art for the pattern, not as a
  dependency.

## Vite

* **Add the wasm-pack `pkg` directory to `optimizeDeps.exclude`.** Vite
  pre-bundles dependencies with esbuild, which does not understand `.wasm`
  imports, so pre-bundling breaks the generated glue module.

* **Avoid direct `.wasm` imports.** A directly imported `.wasm` module is async
  and needs top-level await, meaning `build.target: 'esnext'` or
  `vite-plugin-top-level-await`. Importing the wasm-bindgen glue module and
  calling its `init()` yourself sidesteps this entirely.

## Build profile

* Set `panic = "abort"`, `lto = true`, `codegen-units = 1`, `opt-level = "s"`,
  and `strip = true` in `[profile.release]`. Binary size is user-facing on a
  course page. Natural-level errors should be `Result`, never panics.
* Related: wasm-bindgen 0.2.122 made `panic=unwind` emit modern exnref exception
  handling by default, which requires Node 22.22.3 or newer. One more reason to
  stay on abort.
