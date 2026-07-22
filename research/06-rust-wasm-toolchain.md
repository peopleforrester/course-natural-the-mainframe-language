# Rust and WebAssembly Toolchain for the Natural Interpreter

Spike date: 2026-07-22

Adoption spike for a technology stack that is new to this project. Every version
number and every "recommended" claim below is verified against a live registry,
official documentation, or a release feed, and carries an access date. Anything
that could not be verified from a primary source is flagged explicitly as
unverified rather than guessed.

## Executive summary

The recommended stack is boring on purpose, because the interesting parts of this
project are the Natural semantics, not the build pipeline.

**Recommended toolchain (all verified as of 2026-07-22):**

| Concern | Choice | Version | Why |
|---|---|---|---|
| Rust | stable | 1.97.1, released 2026-07-16 | Current stable channel |
| Edition | 2024 | stable since Rust 1.85.0 (2025-02-20) | Current edition; greenfield crate, no migration cost |
| Wasm target | `wasm32-unknown-unknown` | Tier 2 | Browser target. WASI targets are for non-browser hosts |
| JS bindings | `wasm-bindgen` | 0.2.126, published 2026-06-24 | The only mature browser binding generator |
| Build wrapper | `wasm-pack` | 0.15.0, published 2026-05-15 | Alive again under new maintainers; wraps cargo, wasm-bindgen-cli, and wasm-opt |
| wasm-bindgen output mode | `--target web` | n/a | Plain ES module, no bundler required |
| Terminal | `@xterm/xterm` | 6.0.0, published 2025-12-22 | Scoped package. The unscoped `xterm` package is deprecated |
| Decimal arithmetic | `rust_decimal` | 1.42.1, published 2026-06-12 | Pure Rust, no C dependencies, fixed-point base-10 with an explicit scale |
| INPUT model | Resumable state machine that yields | n/a | Avoids COOP/COEP and SharedArrayBuffer entirely |
| Static site build | Vite, or no bundler at all | n/a | `--target web` output is directly importable as an ES module |

**Recommended project layout:** a three-crate Cargo workspace. `natural-core` is a
pure Rust library with zero wasm and zero JS dependencies, and it holds the lexer,
parser, the decimal value model, and the interpreter driven as a resumable state
machine. `natural-cli` is a thin binary that wraps the core with real stdin and
stdout. `natural-wasm` is a `cdylib` that wraps the core with `wasm_bindgen`
exports. All the tests that matter live in `natural-core` and run natively in
milliseconds; the wasm crate gets a small smoke suite only.

**The four things a developer on this project is most likely to get wrong**, all
of which are things that tutorials and older training data still teach
incorrectly:

1. Installing the npm package `xterm` and the addons `xterm-addon-*`. Those
   packages are formally deprecated. The correct names are `@xterm/xterm` and
   `@xterm/addon-*`.
2. Reaching for `SharedArrayBuffer` plus `Atomics.wait` to make `INPUT` block.
   That requires cross-origin isolation, which GitHub Pages cannot provide with
   real headers.
3. Building for a WASI target (`wasm32-wasip1` or `wasm32-wasip2`) because a
   blog post said WASI is the modern path. WASI is not the browser path.
4. Reading the wasm-bindgen guide at `rustwasm.github.io`, which is a stale
   mirror. The maintained guide is at `wasm-bindgen.github.io`.

---

## 1. Rust to wasm toolchain, current state July 2026

### The organizational change training data will miss

The single biggest thing that older material gets wrong here is not a version
number, it is who owns the tools. On 2025-07-21 the Rust project announced it was
sunsetting the `rustwasm` GitHub organization, which had housed wasm-bindgen,
web-sys, wasm-pack, and the Rust and WebAssembly book. The Rust and WebAssembly
Working Group had already been archived in 2024 after roughly five years of
inactivity (per the Rust Inside Rust blog post of 2025-07-21, accessed
2026-07-22).

This is easy to misread as "Rust wasm is dead." It is not what happened. The
announcement is explicit that neither wasm-bindgen, web-sys, wasm-pack, nor gloo
are being deprecated. wasm-bindgen moved to a new project-specific
`wasm-bindgen` GitHub organization with additional maintainers. As of 2026-07-22
the repository `github.com/wasm-bindgen/wasm-bindgen` shows a last push of
2026-07-22, so it is under active daily development.

The practical consequences for us are documentation URLs and issue trackers, not
code. See the gotchas file.

### wasm-bindgen

`wasm-bindgen` 0.2.126 is the current release, published 2026-06-24, per the
crates.io API (accessed 2026-07-22). The release cadence is brisk: 0.2.121 on
2026-05-07, 0.2.122 on 2026-05-22, 0.2.123 on 2026-06-08, 0.2.125 on 2026-06-12,
0.2.126 on 2026-06-24. `wasm-bindgen-cli` tracks the same version number, and
`wasm-bindgen-test` is at 0.3.76 as of 2026-06-24.

**Critical operational rule:** the `wasm-bindgen` crate version and the
`wasm-bindgen-cli` version must match exactly. A mismatch produces a confusing
schema-version error at bind time rather than a clean message. Using `wasm-pack`
removes this class of failure, because wasm-pack downloads and caches a matching
CLI for you.

Notable behavior changes visible in the upstream CHANGELOG (accessed 2026-07-22
via the raw file on the `main` branch) that older tutorials predate:

- The library MSRV rose to 1.71 in 0.2.106, with a stated policy that library
  MSRV changes come with a minor version bump. CLI MSRV was raised to 1.82 in
  0.2.118 and adjusted again around 0.2.122. Since we are on Rust 1.97.1 this is
  not a constraint for us, but it does mean pinned old toolchains will fail.
- `JsStatic` was deprecated in 0.2.93 in favor of `#[wasm_bindgen(thread_local)]`.
  Older examples that declare statics the old way now warn.
- The `--weak-refs` CLI flag was deprecated in 0.2.91; weak reference support is
  detected at runtime now. Passing the flag in a build script is dead weight.
- In 0.2.123, `JsOption<T>` changed semantics so that only `undefined` counts as
  empty and `null` is a distinct present value. This matters if we ever model
  optional Natural fields across the boundary; we should avoid `JsOption` in our
  design and pass explicit tagged structures instead.
- In 0.2.100 the test runner began capturing console output by default, so
  `--nocapture` is required to see `println!` from a wasm test.

### wasm-pack: alive, and it moved

This is the question the brief specifically flagged, and the answer changed
recently enough that training data is very likely to be wrong in either
direction.

`wasm-pack` 0.15.0 was published 2026-05-15 per crates.io (accessed 2026-07-22).
Before the recent revival, the last release was 0.13.1 on 2024-10-29, a gap of
roughly fifteen months that produced a lot of "wasm-pack is abandoned" commentary
that is now out of date. 0.14.0 landed 2026-01-20 and 0.15.0 on 2026-05-15.

Per the wasm-pack CHANGELOG (read via docs.rs, accessed 2026-07-22):

- 0.14.0 is described as a community takeover with multiple new maintainers. It
  added arbitrary wasm target support (including WASI targets), native Apple
  Silicon builds, a `--split-linked-modules` passthrough, and custom build
  profiles via `--profile`.
- 0.15.0 added `wasm64-unknown-unknown` support and a `--panic-unwind` flag, and
  vendored the project template into the repository. It also fixed a genuinely
  nasty regression where `npm install -g wasm-pack` returned a 404 because the
  installer referenced a previous maintainer's URL.
- The repository moved again in 0.15.0, from `drager/wasm-pack` to
  `wasm-bindgen/wasm-pack`, consolidating it under the same org as wasm-bindgen.

That last point is the strongest signal available: wasm-pack now lives alongside
wasm-bindgen under the same maintainers. Treating it as abandoned in 2026 would
be wrong.

**Caveat to record honestly:** the npm distribution of wasm-pack broke badly
enough to need a fix in 0.15.0. We should install wasm-pack via
`cargo install wasm-pack --locked` rather than via npm, so our build does not
depend on that distribution channel.

### The alternatives, and why we are not using them

**Trunk** (0.21.14 stable, published 2025-05-08; a 0.22.0-beta.1 exists from
2026-03-10, per crates.io accessed 2026-07-22) is a full application bundler for
Rust wasm apps. It is aimed at projects where Rust owns the whole page, typically
Yew or Leptos. Our page is mostly JavaScript (xterm.js owns the DOM) and Rust is
a library called from JS. Trunk is the wrong shape for that, and its stable
release is over a year old while a beta sits unreleased.

**cargo-component** (0.21.1, published 2025-03-18, per crates.io accessed
2026-07-22) builds WebAssembly Components against WIT interfaces. This is the
component model and WASI 0.2 path. It is genuinely the future for server-side and
plugin-host wasm. It is not the browser path today: browsers do not natively load
components, so a component still has to be transpiled down to core wasm plus JS
glue via `jco` before a browser can run it. That adds a build stage and a layer
of indirection for zero benefit to us. Its last release being over sixteen months
old as of this spike is a further reason not to make it a dependency of a course
deliverable.

**wasm-bindgen CLI directly**, with no wasm-pack, is a legitimate option and is
what wasm-pack does internally. `cargo build --target wasm32-unknown-unknown
--release` followed by `wasm-bindgen --target web --out-dir web/pkg
target/wasm32-unknown-unknown/release/natural_wasm.wasm` is the whole pipeline.
We should keep this documented as the fallback, because it removes one dependency
and makes the build legible in a teaching context. The reason to prefer wasm-pack
is that it guarantees the CLI version matches the crate version and it runs
`wasm-opt` for us.

### Target selection: `wasm32-unknown-unknown`, not WASI

Per the rustc platform support table (accessed 2026-07-22),
`wasm32-unknown-unknown`, `wasm32-wasip1`, `wasm32-wasip2`,
`wasm32-wasip1-threads`, and `wasm32-unknown-emscripten` are all Tier 2 with
std support. `wasm32v1-none` is Tier 3 and `no_std` only.

The distinction that matters:

- `wasm32-unknown-unknown` assumes no host environment at all. There is no
  filesystem, no stdin, no clock unless you import one. Everything the module can
  do, it does through imports that wasm-bindgen generates. **This is the browser
  target.** wasm-bindgen supports this target and this is what xterm.js
  integration needs.
- `wasm32-wasip1` and `wasm32-wasip2` assume a WASI host that provides
  POSIX-like capabilities including a real blocking stdin. That is exactly what
  we want semantically for a `READ`-style `INPUT`, and exactly what a browser
  does not provide. Running a WASI module in a browser requires a JS WASI shim
  such as `@bjorn3/browser_wasi_shim` or Wasmer's JS SDK, which reintroduces the
  blocking-stdin problem at the shim layer anyway.
- `wasm32v1-none` restricts to the WebAssembly 1.0 feature set with no imports.
  Not useful here.

Recommendation: build the browser artifact for `wasm32-unknown-unknown`. Do not
add a WASI target. The native CLI covers the "real blocking stdin" case by being
a native binary, which is simpler and faster than a WASI runtime.

---

## 2. xterm.js: the scoped package rename

This section is the highest-value part of the spike, because the failure mode is
silent and slow. The old package still installs, still works, and is three years
behind.

### Verified current state

Per the npm registry (accessed 2026-07-22):

| Package | dist-tag `latest` | Published | Status |
|---|---|---|---|
| `@xterm/xterm` | **6.0.0** | 2025-12-22 | Current |
| `@xterm/addon-fit` | **0.11.0** | 2025-12-22 | Current |
| `@xterm/addon-web-links` | **0.12.0** | 2025-12-22 | Current |
| `@xterm/addon-webgl` | **0.19.0** | 2025-12-22 | Current |
| `@xterm/addon-clipboard` | 0.2.0 | 2025-12-22 | Current |
| `@xterm/addon-canvas` | 0.7.0 | 2024-04-05 | Current but stale; see below |
| `xterm` | 5.3.0 | 2023-09-07 | **DEPRECATED** |
| `xterm-addon-fit` | 0.8.0 | 2023-09-07 | **DEPRECATED** |

The deprecation messages carried in the npm metadata are unambiguous. `xterm`
says: "This package is now deprecated. Move to @xterm/xterm instead."
`xterm-addon-fit` says: "This package is now deprecated. Move to @xterm/addon-fit
instead."

The scoped packages appeared with `@xterm/xterm` 5.4.0 on 2024-03-01, then 5.5.0
on 2024-04-05, then the 6.0.0 major on 2025-12-22. So the brief's recollection
that the rename happened around v5.4 is confirmed correct.

Two further observations from the registry data. First, the whole current family
was published on the same day, 2025-12-22, which is the 6.0.0 release train; the
addons are versioned independently of the core but released together. Second,
beta tags are being cut actively (for example `@xterm/xterm` 6.1.0-beta.291 on
2026-07-19), so the project is under active development, not in maintenance.

`@xterm/addon-canvas` is the exception: its `latest` is still 0.7.0 from
2024-04-05 and it did not ride the 6.0.0 train. The canvas renderer is the legacy
fallback renderer. We should use the WebGL addon and let xterm.js fall back to its
built-in DOM renderer if WebGL is unavailable, rather than adding the canvas
addon. Flagged as partially unverified: I did not find an official statement that
`@xterm/addon-canvas` is deprecated, only that it has not been updated in over
two years.

### Correct current install

```bash
npm install @xterm/xterm@6.0.0 \
            @xterm/addon-fit@0.11.0 \
            @xterm/addon-web-links@0.12.0 \
            @xterm/addon-webgl@0.19.0
```

### Correct current init

```js
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebLinksAddon } from '@xterm/addon-web-links';
import { WebglAddon } from '@xterm/addon-webgl';
import '@xterm/xterm/css/xterm.css';

const term = new Terminal({
  convertEol: true,
  cursorBlink: true,
  fontFamily: 'ui-monospace, SFMono-Regular, Menlo, monospace',
  fontSize: 14,
  // Natural screens are 24x80. Pin it for fidelity, or use FitAddon instead.
  cols: 80,
  rows: 24,
});

const fit = new FitAddon();
term.loadAddon(fit);
term.loadAddon(new WebLinksAddon());

term.open(document.getElementById('terminal'));

// WebGL must be loaded AFTER open(), because it needs the rendered element.
try {
  const webgl = new WebglAddon();
  webgl.onContextLoss(() => webgl.dispose());
  term.loadAddon(webgl);
} catch {
  // No WebGL2. xterm.js falls back to the DOM renderer on its own.
}

fit.fit();
window.addEventListener('resize', () => fit.fit());
```

Three details that older tutorials get wrong. The CSS import path is
`@xterm/xterm/css/xterm.css`, not `xterm/css/xterm.css`. `term.open()` must be
called before `loadAddon(new WebglAddon())`, because the WebGL renderer attaches
to the already-created DOM element. And the package ships both a CommonJS entry
(`lib/xterm.js`) and an ESM entry (`lib/xterm.mjs`) per the registry metadata
accessed 2026-07-22, so modern bundlers resolve the ESM build without special
configuration.

Note for our project specifically: `convertEol: true` matters. Our interpreter
will emit `\n` from Rust, and xterm.js is a real terminal emulator that treats a
bare `\n` as line feed without carriage return, producing a staircase. Either set
`convertEol` or emit `\r\n` from Rust. Choose one and enforce it in the wasm
boundary layer, not scattered through the interpreter.

---

## 3. Blocking INPUT in wasm

Natural's `INPUT` statement is synchronous in the source language. The learner
writes a linear program and expects execution to stop at `INPUT` until a line
arrives. WebAssembly in a browser has no blocking stdin and, on the main thread,
no way to block at all. There are three viable strategies, not two.

### Option A: resumable state machine that yields

The interpreter never blocks. It runs until it hits an `INPUT`, then returns
control to JavaScript with a status meaning "I need a line, here is the prompt
and the field metadata." All interpreter state (program counter, call stack,
variable storage) lives in a struct owned by the wasm module and persists across
the return. When the terminal collects a line, JS calls back into the module with
the string, and the module resumes from where it stopped.

The public wasm surface is small:

```rust
#[wasm_bindgen]
pub struct Session { /* owns the whole interpreter state */ }

#[wasm_bindgen]
impl Session {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> Result<Session, JsValue>;

    /// Run until output is produced, input is needed, or the program ends.
    /// Returns a tagged Step describing which.
    pub fn step(&mut self) -> Step;

    /// Deliver the line the terminal collected and unblock the pending INPUT.
    pub fn provide_input(&mut self, line: &str);
}
```

Implementation note that matters for a teaching interpreter: this is materially
easier if the interpreter is written as an explicit loop over an instruction or
statement cursor with an explicit call stack, rather than as a recursive
tree-walking `eval` that uses the Rust call stack. A recursive evaluator cannot
be paused mid-expression without either coroutines or unwinding. Since Natural's
`INPUT` occurs at statement level, not inside expression evaluation, a hybrid
works: recursive expression evaluation is fine, but statement execution must be
an explicit loop with an explicit frame stack. This is a design constraint on
`natural-core` that must be decided before the first line of the interpreter is
written, because retrofitting it is a rewrite.

**Costs:** zero special headers, zero threads, works on every browser and every
static host, testable natively with no browser at all (the native CLI drives the
identical state machine with a real `stdin.read_line()`). The interpreter design
constraint above is the only real cost.

### Option B: Web Worker plus `Atomics.wait` on a `SharedArrayBuffer`

The interpreter runs in a Web Worker and genuinely blocks. `INPUT` calls into a
JS import that does `Atomics.wait` on an `Int32Array` backed by a
`SharedArrayBuffer`. The main thread writes the typed line into the shared buffer
and calls `Atomics.notify`, and the worker wakes up and returns the string. The
Rust interpreter can then be a naive recursive evaluator with genuinely blocking
I/O, which is the simplest possible interpreter code.

The blockers are environmental, not architectural.

Per MDN's `Atomics.wait()` reference (accessed 2026-07-22), `Atomics.wait()`
throws a `TypeError` if the current thread cannot be blocked, explicitly
including the main thread, and requires an `Int32Array` or `BigInt64Array` over a
`SharedArrayBuffer`. So the worker is mandatory, not optional.

`SharedArrayBuffer` in turn requires the document to be cross-origin isolated,
which requires two response headers on the HTML document:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Per web.dev's cross-origin isolation guide (accessed 2026-07-22), only a
cross-origin isolated document reaches `crossOriginIsolated === true` and can
construct a `SharedArrayBuffer`.

Static host support for those headers, verified 2026-07-22:

| Host | Custom response headers | Verdict for option B |
|---|---|---|
| GitHub Pages | **No.** Long-standing feature request (community discussion 13309) still open | Blocked without a service worker hack |
| Netlify | Yes, via `_headers` or a `[[headers]]` block in `netlify.toml` | Works |
| Cloudflare Pages | Yes, via a `_headers` file | Works |
| Railway static | Not verified in this spike. Railway serves static sites behind a configurable process, so headers are presumably settable, but I did not confirm this against Railway's own documentation. **Flagged as unverified.** | Unknown |

The GitHub Pages workaround is `coi-serviceworker`, a service worker that
intercepts responses and injects the COOP and COEP headers client-side. It is
widely used and it does work, but it forces a full page reload on a visitor's
very first load in order to install the service worker, and it fails entirely if
the service worker registration is blocked. For a course deliverable where the
first thirty seconds of a learner's experience is the product, a mandatory
first-load reload is a real cost.

There is a second cost that is easy to overlook. `require-corp` means every
cross-origin subresource must itself opt in with `Cross-Origin-Resource-Policy`
or be fetched with CORS. Any embedded font, analytics script, or image from a CDN
that does not send that header simply stops loading. For a course page that may
later embed video or a font, that is a recurring maintenance tax.

### Option C: JSPI (JavaScript Promise Integration)

Worth documenting because it is the option that will eventually make this problem
disappear, and because it would be a mistake to design around it today.

JSPI lets synchronous wasm code call asynchronous JavaScript APIs. The wasm stack
is suspended and resumed by the engine, so a Rust function can call a JS function
that returns a Promise and simply appear to block. This is exactly our `INPUT`.

Status as of 2026-07-22: the specification reached Phase 4 at the W3C
WebAssembly CG in April 2025 and shipped in Chrome 137 and Firefox 139. It
remains behind a flag in Firefox. Safari removed its objection to the proposal in
late 2025 and has an assigned implementer, but has not shipped it. Per the WebKit
blog post announcing Interop 2026 (accessed 2026-07-22), JSPI is one of four
focus areas comprising twenty percent of the Interop 2026 score, which is a
strong signal that cross-browser convergence is a 2026 goal rather than a 2026
fact.

Verdict: not shippable in July 2026 for a course that must work in Safari. Revisit
after Interop 2026 concludes.

### Recommendation: option A, and the spec's instinct is correct

The spec's lean toward the resumable state machine is validated. The reasoning:

The COOP/COEP requirement is not merely an inconvenience, it is a coupling
between our interpreter's internal architecture and our hosting provider's
feature set. Choosing option B means that "where can this course be hosted" is
permanently constrained by "does this host let me set two headers," and it means
GitHub Pages, the most obvious free host for a static course artifact, requires a
service worker workaround. That is a bad trade for a deliverable whose entire
selling point is that it is a zero-backend static page.

The cost of option A is one design constraint on the interpreter: statement
execution is an explicit loop with an explicit frame stack. For a teaching-subset
interpreter this is arguably the better design anyway, because it makes single-
stepping, breakpoints, and an execution trace view trivial to add later. Those
are exactly the features a course about a mainframe language would want for
demonstrating program flow.

There is a second, decisive argument. Option A gives us one interpreter that both
the native CLI and the browser drive identically, because both call `step()` in a
loop and differ only in where the input line comes from. Option B gives us a
browser build that blocks and a native build that blocks, which sounds more
uniform, but the browser one only blocks through a JS shim we have to write,
test, and debug across browsers. Option A's core is testable with plain
`cargo test` and no browser at all, which matters a lot given this project
follows strict TDD.

Recommended shape, restated concretely: `step()` returns a tagged enum, one
variant of which is `NeedsInput { prompt, field_name, field_format }`. The
browser driver loops calling `step()`, writes `Output` variants to xterm.js, and
on `NeedsInput` stops the loop, enables the line editor, and resumes on Enter. If
a program ever runs long enough to jank the frame, the driver yields to the event
loop with a zero-delay timeout every N steps. That is a driver-side concern, not
an interpreter concern, which is the point.

---

## 4. Decimal arithmetic

Natural is a business 4GL. Its numeric model is base-10 with a declared precision
and scale, written `N7.2` for seven integer digits and two decimal places, with
packed variants (`P`) that differ in storage, not in arithmetic semantics. Edit
masks (`EM=ZZZ,ZZ9.99`) format those values for display. Binary floating point is
wrong for this by construction: a course that teaches business arithmetic and
then prints `0.30000000000000004` has taught the wrong thing.

### rust_decimal: recommended

Per crates.io (accessed 2026-07-22), `rust_decimal` 1.42.1 was published
2026-06-12, with 1.42.0 on 2026-05-06, 1.41.0 on 2026-03-27, and 1.40.0 on
2026-01-14. That is a steady release cadence, and the crate reports over 121
million total downloads. Its own description is "a decimal number implementation
written in pure Rust suitable for financial and fixed-precision calculations."

Per the project README on GitHub (accessed 2026-07-22):

- It is pure Rust with no C dependencies. That is the load-bearing property for
  wasm: anything binding a C library needs an emscripten sysroot or a vendored
  build, and this crate needs neither.
- It supports `std` (default), `no_std` with `alloc`, and `no_std` without
  `alloc`, and is documented as usable on bare-metal targets such as
  `x86_64-unknown-none`. A target with no OS is strictly harder than
  `wasm32-unknown-unknown`, so this is strong evidence of wasm compatibility.
- Its representation is a 96-bit mantissa plus a scale, giving roughly 28 base-10
  significant digits (29 in some cases).
- The stated MSRV is 1.67.1, well below our 1.97.1.
- There is an optional `wasm` feature, but read carefully: it enables
  `wasm-bindgen` support to expose `fromNumber`, `toNumber`, `fromString`, and
  `toString` across the JS boundary. It is not required in order to compile to
  wasm. We do not want it. Our decimal values should never cross the wasm
  boundary as decimals; they should cross as already-formatted strings, because
  the edit mask is our formatting authority, not JavaScript's.

**Why the representation fits Natural:** `rust_decimal` stores a scale
explicitly, so `Decimal::new(12345, 2)` is exactly 123.45 with scale 2, and the
scale survives arithmetic in a defined way. That maps directly onto Natural's
declared `N7.2`. A binary fixed-point crate would not, because Natural's scale is
decimal.

**One caveat to flag rather than paper over.** Natural's numeric fields support
up to 29 total digits in some documented variants, and `rust_decimal`'s useful
range is 28 significant digits, 29 in some cases. For a teaching subset with
`N7.2` and similar realistic business field sizes, this is a non-issue by an
enormous margin. It would only bite if we chose to implement the full maximum
field width. **I did not verify Natural's exact maximum digit count against
vendor documentation in this spike**; that belongs in the language-subset spec,
not here. The recommendation is to cap the teaching subset's declared precision
at something well inside `rust_decimal`'s range (say 20 integer digits and 7
decimal places) and to make exceeding it a clean interpreter error rather than a
silent wrong answer.

**One thing I could not fully verify.** I did not find an explicit
`wasm32-unknown-unknown` entry in `rust_decimal`'s published CI matrix. The
inference that it compiles cleanly rests on the crate being pure Rust with no
build script C dependency and on documented bare-metal `no_std` support. This
should be confirmed empirically as the very first task of the build spike:
`cargo build --target wasm32-unknown-unknown -p natural-core` with `rust_decimal`
as a dependency. That is a five-minute check and it removes the last doubt.

### Alternatives considered

**`fastnum` 0.7.5**, published 2026-06-11 (crates.io, accessed 2026-07-22). Per
its docs.rs documentation (accessed 2026-07-22), it provides fixed-size signed
and unsigned decimals at 64, 128, 256, and 512 bits (`D64` through `D512` and
unsigned `UD*` variants), is pure Rust depending only on `bnum` by default, is
`no_std` compatible, has nearly all methods `const`, and states it is "fully
compatible with WebAssembly." Everything about it fits, and if we ever needed
more than 28 digits, `D256` is the answer.

The reason it is the second choice and not the first is ecosystem maturity and
teaching risk. `rust_decimal` has an order of magnitude more usage, a longer
track record, and far more third-party material a learner or a maintainer can
consult when something surprises them. For a course deliverable that other people
will read and modify, that matters more than a feature advantage we do not need.
Keep `fastnum` documented as the escape hatch if a precision requirement emerges.

**`bigdecimal` 0.4.10**, published 2025-12-27 (crates.io, accessed 2026-07-22).
Arbitrary precision, backed by `num-bigint`. Arbitrary precision is the wrong
model for Natural: Natural fields have a fixed declared precision and scale, and
a type that grows without bound makes overflow and truncation semantics harder to
model correctly, not easier. It also heap-allocates, which is unnecessary cost in
a hot interpreter loop. Not recommended.

**`fixed` 1.31.0**, published 2026-03-20 (crates.io, accessed 2026-07-22). This
is binary fixed-point (`I32F32` and friends), not decimal fixed-point. It cannot
represent 0.1 exactly, which defeats the entire purpose. Frequently and wrongly
recommended when someone searches for "Rust fixed point"; it solves a different
problem. Not recommended.

**`rusty-money` 0.5.0**, published 2026-01-14 (crates.io, accessed 2026-07-22).
Wraps `rust_decimal` with currency semantics. Natural's numeric fields are not
currency-typed, so the extra layer buys us nothing and constrains us. Not
recommended.

### Edit masks

None of these crates implement Natural edit masks (`EM=ZZZ,ZZ9.99` with zero
suppression, sign placement, and insertion characters). That formatting layer is
ours to write, in `natural-core`, on top of whatever decimal type we choose. This
is good news for TDD: edit-mask formatting is a pure function from
(decimal value, mask string) to string, which is the easiest possible thing to
build test-first, and it is a place where a table-driven test suite pays for
itself immediately.

---

## 5. Rust language and tooling baseline

Per the official stable channel manifest at
`static.rust-lang.org/dist/channel-rust-stable.toml` (accessed 2026-07-22), the
current stable release is dated 2026-07-16 and ships Cargo 0.98.0. Per the
rust-lang GitHub release feed (accessed 2026-07-22), that is **Rust 1.97.1,
published 2026-07-16**, following 1.97.0 on 2026-07-09, 1.96.1 on 2026-07-05,
1.96.0 on 2026-05-28, and 1.95.0 on 2026-04-16.

Per the Rust Edition Guide (accessed 2026-07-22), the **2024 edition was
stabilized in Rust 1.85.0, released 2025-02-20**, and 2024 is the current
edition. There is no newer stable edition as of this spike.

For a greenfield crate the edition story is simple: put `edition = "2024"` in
`Cargo.toml` and never think about it again. `cargo fix --edition` exists for
migrating an existing crate forward and is irrelevant to us. The 2024 edition
changes most likely to surprise someone whose Rust knowledge predates it are
stricter `unsafe` attribute syntax, changes to `impl Trait` lifetime capture
rules, and the `gen` keyword being reserved. None of these are traps for new
code; they are traps for old code being migrated.

Recommended toolchain pinning: commit a `rust-toolchain.toml` so every developer
and CI runner gets the same compiler and the wasm target is installed
automatically.

```toml
[toolchain]
channel = "1.97.1"
components = ["rustfmt", "clippy"]
targets = ["wasm32-unknown-unknown"]
```

Pinning an exact patch version rather than `"stable"` is the right call for a
course deliverable, because a learner following the material a year from now
should get a reproducible build rather than whatever stable happens to be that
day. Plan to bump it deliberately.

---

## 6. Project layout and build

### Workspace layout

```
natural/
  Cargo.toml                  # [workspace], resolver = "3"
  rust-toolchain.toml
  crates/
    natural-core/             # lib. The whole language. No wasm, no JS, no I/O.
      Cargo.toml
      src/
        lib.rs
        lexer.rs
        parser.rs
        ast.rs
        value.rs              # Decimal-backed value model, N/P formats
        editmask.rs           # EM= formatting, pure functions
        interp.rs             # explicit statement loop + frame stack
        step.rs               # the Step enum: Output | NeedsInput | Halted | Error
      tests/                  # the bulk of the test suite lives here
    natural-cli/              # bin. Native REPL / file runner.
      Cargo.toml
      src/main.rs
    natural-wasm/             # cdylib. wasm_bindgen wrappers only.
      Cargo.toml
      src/lib.rs
  web/                        # the static site
    index.html
    src/main.js               # xterm.js driver
    package.json
    vite.config.js
```

The rule that makes this layout work: **`natural-core` has no `wasm-bindgen`
dependency and no conditional compilation for wasm.** It is an ordinary Rust
library that happens to compile everywhere. All wasm-specific code lives in
`natural-wasm`. This keeps the test suite fast and keeps the interpreter's design
honest, because anything that would tempt you to reach for a browser API cannot
be written in the core.

`natural-wasm/Cargo.toml` needs:

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
natural-core = { path = "../natural-core" }
wasm-bindgen = "0.2.126"

[dev-dependencies]
wasm-bindgen-test = "0.3.76"
```

The `rlib` alongside `cdylib` is not optional if you want `cargo test` to work on
that crate natively; a pure `cdylib` cannot be linked by a test harness.

Also worth setting in the workspace `Cargo.toml`, because wasm binary size is a
real user-facing cost on a course page:

```toml
[profile.release]
opt-level = "s"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

`panic = "abort"` is a deliberate choice: it removes unwinding machinery and
shrinks the binary meaningfully, at the cost of `catch_unwind`. Our interpreter
should return `Result` for Natural-level errors anyway; a Rust panic in the
interpreter is a bug, not a control flow mechanism. Note the wasm-bindgen
CHANGELOG entry for 0.2.122 indicating that `panic=unwind` now emits modern
exnref exception handling by default and requires Node 22.22.3 or newer, which is
one more reason to stay on abort.

### Build

```bash
# Browser artifact
wasm-pack build crates/natural-wasm --target web --out-dir ../../web/pkg --release

# Equivalent without wasm-pack, if we want the pipeline visible
cargo build -p natural-wasm --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir web/pkg \
  target/wasm32-unknown-unknown/release/natural_wasm.wasm
```

`--target web` produces a plain ES module with an exported `init()` function and
no bundler assumption. This is the right choice for us. `--target bundler` is the
wasm-pack default and produces output that requires webpack or Vite to resolve;
`--target no-modules` produces a global-script build for environments without
module support. We want `web`.

### Bundling for the static site

Two viable paths.

**No bundler.** Because `--target web` emits a real ES module, `index.html` can
do `<script type="module">` and import both the glue module and xterm.js from a
local `node_modules` copy or a vendored directory. This is the simplest thing
that works and it is the most legible in a teaching context, because there is no
build step between what the developer writes and what the browser runs. The cost
is that xterm.js and its addons ship as npm packages with bare specifiers, so you
either need an import map or you vendor the built files.

**Vite.** Recommended for the real deliverable. Vite resolves the xterm.js bare
specifiers, handles the CSS import, and produces a hashed static bundle suitable
for any static host. Two configuration details, both verified against community
issue reports rather than official docs (flagged accordingly):

- The `pkg` directory produced by wasm-pack must be added to
  `optimizeDeps.exclude`. Vite's dependency pre-bundling runs esbuild over
  `node_modules`, and esbuild does not understand `.wasm` imports, so the
  pre-bundle step breaks the glue module.
- If you import a `.wasm` file directly rather than through the wasm-bindgen glue
  module, the resulting module is async and needs top-level await, which means
  `build.target: 'esnext'` or the `vite-plugin-top-level-await` plugin. Using the
  glue module's explicit `init()` call sidesteps this entirely, which is another
  reason to prefer `--target web` and call `init()` ourselves.

Per the Vite features documentation (accessed 2026-07-22), Vite natively supports
importing `.wasm` as an ES module and with an `?init` suffix for explicit
instantiation control. `vite-plugin-wasm` exists for the ESM-integration proposal
path but we do not need it if we go through wasm-bindgen's glue.

### The `.wasm` MIME type gotcha

Per MDN's `WebAssembly.instantiateStreaming()` reference (accessed 2026-07-22):
"For this to work, `.wasm` files should be returned with an `application/wasm`
MIME type by the server."

If the server returns `application/octet-stream` or `text/plain`, streaming
instantiation fails. The failure message differs by browser and is not always
obviously about MIME types, which is what makes this a time sink. wasm-bindgen's
generated glue for `--target web` uses `instantiateStreaming` with a fallback to
`arrayBuffer()` plus `instantiate()`, so a wrong MIME type may produce a console
warning and still work, just slower and with more memory. That silent degradation
is arguably worse than a clean failure, because it hides the misconfiguration.

Status by host as of 2026-07-22: GitHub Pages, Netlify, and Cloudflare Pages all
serve `.wasm` as `application/wasm` by default. The classic failure is a hand-
rolled dev server, an nginx or Apache with an old `mime.types` file, or an S3
bucket where the object's `Content-Type` was set at upload time. For nginx, add
`application/wasm wasm;` to `mime.types`. For S3, set the object metadata
explicitly on upload.

### CSP constraints

Per MDN's `Content-Security-Policy: script-src` reference (accessed 2026-07-22):
"If a page has a CSP header and `'wasm-unsafe-eval'` isn't specified in the
`script-src` directive, WebAssembly is blocked from loading and executing on the
page."

So a page with `script-src 'self'` and nothing else will not run our module. The
minimum workable policy:

```
Content-Security-Policy: default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data:
```

Two notes. `'wasm-unsafe-eval'` is the narrow permission, allowing WebAssembly
compilation and instantiation but not JavaScript `eval()`; prefer it over
`'unsafe-eval'`, which allows both. If `'unsafe-eval'` is present it supersedes
`'wasm-unsafe-eval'`, so do not list both. And `style-src 'unsafe-inline'` is
listed because xterm.js writes inline styles for its renderer; **I did not verify
against xterm.js documentation whether v6.0.0 still requires inline styles**, so
treat that as a starting point to test and tighten rather than a verified
requirement.

Because we chose option A for `INPUT`, we do **not** need COOP or COEP headers,
which means every static host works with default configuration. That is the
concrete payoff of the architecture decision in section 3.

---

## 7. Testing strategy

Strict TDD plus wasm is a combination where the naive approach (test everything
in a browser) is slow enough to destroy the red-green-refactor loop. The strategy
below keeps the fast loop fast.

### Tier 1: `natural-core`, native, plain `cargo test`

This is where ninety-five percent of the tests live, and it is the only tier that
runs on every save. Because `natural-core` has no wasm dependency and no I/O, its
tests are ordinary Rust unit and integration tests running at native speed.

What belongs here:

- Lexer and parser tests, table-driven, one case per Natural construct.
- Decimal semantics: that `N7.2` arithmetic truncates and rounds where Natural
  says it does, and that overflow is a defined error and not a wrap.
- Edit-mask formatting: a table of (value, mask, expected string) triples. This
  is the single highest-value test table in the project and it should be written
  before `editmask.rs` exists.
- Interpreter behavior driven through the state machine. This is where option A
  from section 3 pays off a second time: an input-driven program is testable
  natively by scripting the responses.

```rust
#[test]
fn input_statement_pauses_and_resumes() {
    let mut s = Session::new("INPUT #NAME (A20) WRITE 'HELLO' #NAME END").unwrap();
    assert!(matches!(s.step(), Step::NeedsInput { .. }));
    s.provide_input("MICHAEL");
    assert_eq!(s.step(), Step::Output("HELLO MICHAEL\n".into()));
    assert_eq!(s.step(), Step::Halted);
}
```

That test has no browser, no worker, no async, and no JS in it, and it is
testing the exact mechanism the browser will use. That is the whole argument for
option A restated as a test.

### Tier 2: golden-file program tests

A `tests/programs/` directory with paired `.nat` source files, `.stdin` response
files, and `.expected` output files, driven by one test harness that walks the
directory. This is how you get coverage of realistic Natural programs without
writing a hundred hand-rolled assertions, and it doubles as the course's example
corpus. Every lesson's example program should end up here as a regression test,
which means a broken lesson fails CI.

### Tier 3: `natural-cli`, native integration

A handful of tests that spawn the actual binary, feed it stdin, and check stdout.
`assert_cmd` and `predicates` are the conventional crates. Keep this small: it is
testing argument parsing and the stdin wiring, not the language.

### Tier 4: wasm smoke tests, `wasm-bindgen-test`

Deliberately minimal. This tier exists to prove that the wasm boundary works, not
to test the language again. Five to ten tests: the module initializes, a session
constructs, `step()` returns something JS can read, `provide_input()` round-trips
a string across the boundary including non-ASCII, and an error surfaces as a
`JsValue` rather than an unhandled panic.

Per the wasm-bindgen guide's browser testing page (accessed 2026-07-22), the
recommended way to run these is through wasm-pack, which manages the WebDriver
clients for you:

```bash
wasm-pack test --headless --chrome crates/natural-wasm
```

In the test file:

```rust
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);
```

The guide notes that `wasm_bindgen_test_configure!` overrides the
`WASM_BINDGEN_USE_*` environment variables, that per-browser WebDriver
capabilities go in a `webdriver.json` at the crate root (or wherever
`WASM_BINDGEN_TEST_WEBDRIVER_JSON` points), and that dropping `--headless` gives
you devtools for debugging. Remember that since 0.2.100 console output is
captured by default, so add `--nocapture` when a test's `println!` output is what
you need to see.

### Tier 5: browser end-to-end, deferred

A Playwright test that loads the built page, types a Natural program, presses
Enter at an `INPUT` prompt, and asserts on the rendered terminal buffer. This
catches xterm.js wiring problems that no other tier can see. It is slow and it is
the tier most likely to be flaky, so it should run in CI on push, not on every
save, and it should stay small (three to five scenarios).

Note for the future: asserting on xterm.js content is done through the terminal's
buffer API rather than by reading the DOM, because the WebGL renderer draws to a
canvas and there is no text in the DOM to assert against. That is a trap worth
recording now.

### The pragmatic rule

Tiers 1 and 2 run in the TDD loop, on every change, and must stay under a couple
of seconds. Tiers 3, 4, and 5 run in CI. If a bug is found in tier 4 or 5, the
fix begins by reproducing it as a tier 1 test wherever that is possible, because
a bug that can only be reproduced in a browser is a bug that will regress.

---

## Sources

All URLs accessed 2026-07-22.

**Registries and manifests (version and date claims)**

- `https://crates.io/api/v1/crates/wasm-bindgen` substantiates wasm-bindgen
  0.2.126 published 2026-06-24 and the preceding release dates.
- `https://crates.io/api/v1/crates/wasm-bindgen-cli` substantiates that the CLI
  tracks the same 0.2.126 version.
- `https://crates.io/api/v1/crates/wasm-bindgen-test` substantiates
  wasm-bindgen-test 0.3.76, 2026-06-24.
- `https://crates.io/api/v1/crates/wasm-pack` substantiates wasm-pack 0.15.0
  published 2026-05-15, 0.14.0 on 2026-01-20, and the 0.13.1 gap back to
  2024-10-29.
- `https://crates.io/api/v1/crates/trunk` substantiates trunk stable 0.21.14
  (2025-05-08) and the unreleased 0.22.0-beta.1 (2026-03-10).
- `https://crates.io/api/v1/crates/cargo-component` substantiates cargo-component
  0.21.1, 2025-03-18.
- `https://crates.io/api/v1/crates/rust_decimal` substantiates rust_decimal
  1.42.1 published 2026-06-12, prior releases, description, and keywords.
- `https://crates.io/api/v1/crates/fastnum` substantiates fastnum 0.7.5,
  2026-06-11.
- `https://crates.io/api/v1/crates/bigdecimal` substantiates bigdecimal 0.4.10,
  2025-12-27.
- `https://crates.io/api/v1/crates/fixed` substantiates fixed 1.31.0, 2026-03-20,
  and that it is binary fixed-point.
- `https://crates.io/api/v1/crates/rusty-money` substantiates rusty-money 0.5.0,
  2026-01-14.
- `https://registry.npmjs.org/@xterm/xterm` substantiates `@xterm/xterm` 6.0.0
  published 2025-12-22, the 5.4.0 (2024-03-01) scoped-package debut, the ESM and
  CJS entry points, and the active 6.1.0-beta line.
- `https://registry.npmjs.org/@xterm/addon-fit`,
  `https://registry.npmjs.org/@xterm/addon-web-links`,
  `https://registry.npmjs.org/@xterm/addon-webgl`,
  `https://registry.npmjs.org/@xterm/addon-canvas`,
  `https://registry.npmjs.org/@xterm/addon-clipboard` substantiate the current
  addon versions and publish dates.
- `https://registry.npmjs.org/xterm` and
  `https://registry.npmjs.org/xterm-addon-fit` substantiate the deprecation
  notices, including the exact deprecation text directing users to the scoped
  packages.
- `https://static.rust-lang.org/dist/channel-rust-stable.toml` substantiates the
  stable channel date of 2026-07-16 and Cargo 0.98.0.
- `https://api.github.com/repos/rust-lang/rust/releases` substantiates Rust
  1.97.1 published 2026-07-16 and the preceding release sequence.
- `https://api.github.com/repos/wasm-bindgen/wasm-bindgen` substantiates the
  repository's new organization and its active push date.

**Official documentation**

- `https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org`
  substantiates the rustwasm organization sunset announced 2025-07-21, the
  transfer of wasm-bindgen to a new org, and the explicit statement that
  wasm-bindgen, web-sys, wasm-pack, and gloo are not deprecated.
- `https://doc.rust-lang.org/nightly/rustc/platform-support.html` substantiates
  the Tier 2 status of `wasm32-unknown-unknown`, `wasm32-wasip1`,
  `wasm32-wasip2`, `wasm32-wasip1-threads`, and `wasm32-unknown-emscripten`, and
  the Tier 3 `no_std` status of `wasm32v1-none`.
- `https://doc.rust-lang.org/edition-guide/rust-2024/index.html` substantiates
  that the 2024 edition was stabilized in Rust 1.85.0 on 2025-02-20 and is the
  current edition.
- `https://wasm-bindgen.github.io/wasm-bindgen/` is the current maintained
  location of the wasm-bindgen guide.
- `https://rustwasm.github.io/docs/wasm-bindgen/` substantiates that the old
  location is no longer maintained and explicitly redirects readers to
  `wasm-bindgen.github.io`.
- `https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/CHANGELOG.md`
  substantiates the MSRV changes, the `JsStatic` and `--weak-refs` deprecations,
  the `JsOption` null semantics change in 0.2.123, the `panic=unwind` exnref
  default in 0.2.122, and the 0.2.100 test-runner output capture change.
- `https://docs.rs/crate/wasm-pack/latest/source/CHANGELOG.md` substantiates the
  0.14.0 community takeover, the 0.15.0 feature set, the npm installer 404
  regression and its fix, and the repository move to `wasm-bindgen/wasm-pack`.
- `https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/browsers.html`
  substantiates the recommended wasm-bindgen-test browser setup, the
  `wasm_bindgen_test_configure!(run_in_browser)` macro, the `WASM_BINDGEN_USE_*`
  environment variables, `webdriver.json` and
  `WASM_BINDGEN_TEST_WEBDRIVER_JSON`, and the `wasm-pack test --headless`
  invocation.
- `https://github.com/paupino/rust-decimal` substantiates rust_decimal's pure
  Rust implementation, no_std and bare-metal support, 96-bit mantissa and 28 to
  29 significant digits, MSRV 1.67.1, and the purpose of the optional `wasm`
  feature.
- `https://docs.rs/fastnum/latest/fastnum/` substantiates fastnum's D64 through
  D512 types, const evaluation, no_std support, the `bnum`-only default
  dependency, and its stated WebAssembly compatibility.
- `https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait`
  substantiates that `Atomics.wait()` throws `TypeError` when the current thread
  cannot be blocked (including the main thread) and requires an `Int32Array` or
  `BigInt64Array` over a `SharedArrayBuffer`, and the existence of
  `Atomics.waitAsync()`.
- `https://web.dev/articles/coop-coep` substantiates the COOP `same-origin` plus
  COEP `require-corp` requirement for cross-origin isolation and
  `crossOriginIsolated`.
- `https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static`
  substantiates the `application/wasm` MIME type requirement for streaming
  instantiation.
- `https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/script-src`
  substantiates that WebAssembly is blocked without `'wasm-unsafe-eval'` in
  `script-src`, and that `'unsafe-eval'` supersedes it.
- `https://vite.dev/guide/features` substantiates Vite's native `.wasm` ES module
  import and the `?init` suffix.
- `https://webkit.org/blog/17818/announcing-interop-2026/` substantiates that
  JSPI is an Interop 2026 focus area worth twenty percent of the score, and that
  cross-browser consistency is still a goal rather than an accomplished fact.
- `https://v8.dev/blog/jspi` and
  `https://groups.google.com/a/chromium.org/g/blink-dev/c/w_jCD4gf7Bc`
  substantiate JSPI's Phase 4 standardization and its shipping in Chrome 137.

**Community sources, used only where no official source exists (treat as weaker)**

- `https://github.com/orgs/community/discussions/13309` is the open GitHub Pages
  feature request for COOP and COEP headers, substantiating that they are not
  configurable there.
- `https://github.com/gzuidhof/coi-serviceworker` is the service worker
  workaround for injecting COOP and COEP on hosts that cannot set them, including
  the first-load reload behavior.
- `https://github.com/vitejs/vite/discussions/2584` and
  `https://github.com/Menci/vite-plugin-wasm` substantiate the Vite
  `optimizeDeps.exclude` requirement for wasm-pack output and the top-level-await
  requirement for direct `.wasm` imports.

**Explicitly unverified in this spike**

- Railway static hosting's ability to set arbitrary response headers including
  COOP and COEP. Not confirmed against Railway's own documentation. Since the
  recommended architecture does not need those headers, this is not on the
  critical path, but the claim should not be repeated as fact.
- Whether `@xterm/addon-canvas` is formally deprecated, as opposed to merely
  unmaintained since 2024-04-05. No official statement found.
- Whether xterm.js 6.0.0 still requires `style-src 'unsafe-inline'`. The CSP
  recommendation above includes it defensively; test and tighten.
- `rust_decimal`'s explicit CI coverage of `wasm32-unknown-unknown`. Inferred
  from pure-Rust and bare-metal `no_std` support; confirm with an actual build as
  the first task of implementation.
- Natural's exact documented maximum digit count for `N` and `P` fields, relevant
  only to the 28-versus-29-digit edge described in section 4. Belongs in the
  language-subset spec.
