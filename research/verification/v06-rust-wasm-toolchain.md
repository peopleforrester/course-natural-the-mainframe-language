# Verification: 06-rust-wasm-toolchain

Adversarial fact-check of `research/06-rust-wasm-toolchain.md` (spike date 2026-07-22)
and `docs/gotchas-rust-wasm.md`.

Verification pass ran 2026-07-31 into 2026-08-01. Every version claim was re-queried
against the live registry or manifest, not against the spike's own citations. Where the
spike marked something unverified, I attempted to resolve it and recorded the result.

Method: direct HTTP against `crates.io/api/v1`, `registry.npmjs.org`,
`static.rust-lang.org`, `api.github.com`, and `raw.githubusercontent.com`, plus MDN
browser-compat-data as the authority on browser support. One claim was settled by
building it locally rather than by reading about it.

**Headline: the version numbers held up. The prose around them did not.** Nine claims
are wrong or stale. Two are outright refuted by the primary source the spike itself
cites. Four previously-unverified items are now resolved.

---

## Verdict table

### Rust toolchain and edition

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| "the current stable release is dated 2026-07-16 and ships Cargo 0.98.0 ... that is **Rust 1.97.1, published 2026-07-16**" | CONFIRMED | `date = "2026-07-16"`, `pkg.rust` = `1.97.1 (8bab26f4f 2026-07-14)`, `pkg.cargo` = `0.98.0 (c980f4866 2026-06-30)`. Still the stable channel today. | https://static.rust-lang.org/dist/channel-rust-stable.toml | 2026-08-01 |
| "1.97.0 on 2026-07-09, 1.96.1 on 2026-07-05, 1.96.0 on 2026-05-28, and 1.95.0 on 2026-04-16" | CONFIRMED | Exact match on all four publish timestamps. | https://api.github.com/repos/rust-lang/rust/releases | 2026-08-01 |
| "the **2024 edition was stabilized in Rust 1.85.0, released 2025-02-20**, and 2024 is the current edition. There is no newer stable edition" | CONFIRMED | Edition guide `rust-2024` states "Release version 1.85.0". Tag `1.85.0` published 2025-02-20T17:09:32Z. `rust-2027/` and `rust-2028/` both 404 in the edition guide. | https://doc.rust-lang.org/edition-guide/rust-2024/index.html | 2026-08-01 |
| "`channel = "1.97.1"`" in `rust-toolchain.toml` | CONFIRMED | Matches live stable. Local `rustc 1.97.1 (8bab26f4f 2026-07-14)`. | local toolchain | 2026-08-01 |

### Wasm target selection

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| "`wasm32-unknown-unknown`, `wasm32-wasip1`, `wasm32-wasip2`, `wasm32-wasip1-threads`, and `wasm32-unknown-emscripten` are all Tier 2 with std support" | CONFIRMED | All five listed under "Tier 2 without Host Tools" with std `✓`. | https://doc.rust-lang.org/nightly/rustc/platform-support.html | 2026-08-01 |
| "`wasm32v1-none` is Tier 3 and `no_std` only" / gotchas: "`wasm32v1-none` \| Tier 3, `no_std` only" | **REFUTED (tier is wrong)** | `wasm32v1-none` is **Tier 2 without Host Tools**, std column `*` (no_std). The `no_std` half is right; the tier is not. | https://doc.rust-lang.org/nightly/rustc/platform-support.html | 2026-08-01 |
| (not claimed) `wasm64-unknown-unknown` | CONTEXT | Tier 3, std `?`. Relevant only because wasm-pack 0.15.0 added support for it. | same | 2026-08-01 |

### wasm-bindgen

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| "`wasm-bindgen` 0.2.126 is the current release, published 2026-06-24" | CONFIRMED | `max_stable_version` = 0.2.126, created 2026-06-24T19:05:14Z. Unchanged. | https://crates.io/api/v1/crates/wasm-bindgen | 2026-08-01 |
| "0.2.121 on 2026-05-07, 0.2.122 on 2026-05-22, 0.2.123 on 2026-06-08, 0.2.125 on 2026-06-12, 0.2.126 on 2026-06-24" | CONFIRMED | Exact match on all five. | same | 2026-08-01 |
| "`wasm-bindgen-cli` tracks the same version number" | CONFIRMED | 0.2.126, published 2026-06-24T19:05:27Z. | https://crates.io/api/v1/crates/wasm-bindgen-cli | 2026-08-01 |
| "`wasm-bindgen-test` is at 0.3.76 as of 2026-06-24" | CONFIRMED | 0.3.76, 2026-06-24T19:05:31Z. | https://crates.io/api/v1/crates/wasm-bindgen-test | 2026-08-01 |
| "The library MSRV rose to 1.71 in 0.2.106" | STALE | True as history (0.2.106: "New MSRV policy, and bump of the MSRV fo 1.71"). Wrong as a present-tense statement: library `rust-version` is now **1.77**, raised via 0.2.118 (1.71 to 1.76) and again since. | CHANGELOG + https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/Cargo.toml | 2026-08-01 |
| "CLI MSRV was raised to 1.82 in 0.2.118 and adjusted again around 0.2.122" | **REFUTED** | CLI hit 1.82 in **0.2.101** ("MSRV of CLI tools bumped to v1.82"). 0.2.118 raised it 1.82 to 1.86. No 0.2.122 MSRV change exists. Current CLI `rust-version` = **1.86**. | CHANGELOG + https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/crates/cli/Cargo.toml | 2026-08-01 |
| "`JsStatic` was deprecated in 0.2.93 in favor of `#[wasm_bindgen(thread_local)]`" | **STALE (replacement is wrong)** | The 0.2.93 deprecation is real. But `#[wasm_bindgen(thread_local)]` was **itself deprecated in 0.2.96** in favor of `#[wasm_bindgen(thread_local_v2)]`. Both attributes still parse (`parser.rs` lines 122 to 123); V1 is the deprecated one. Telling a reader to use `thread_local` sends them to a deprecated API. | https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/CHANGELOG.md (0.2.96) | 2026-08-01 |
| "The `--weak-refs` CLI flag was deprecated in 0.2.91" | CONFIRMED | 0.2.91: "Deprecate `--weak-refs` and `WASM_BINDGEN_WEAKREF` in favor of automatic run-time detection." | CHANGELOG | 2026-08-01 |
| "In 0.2.123, `JsOption<T>` changed semantics so that only `undefined` counts as empty and `null` is a distinct present value" | CONFIRMED | Verbatim in the 0.2.123 section, PR #5170. Also removes `impl<T> UpcastFrom<Null> for JsOption<T>` and changes the Debug/Display placeholder from `"null"` to `"undefined"`. | CHANGELOG | 2026-08-01 |
| "In 0.2.100 the test runner began capturing console output by default, so `--nocapture` is required to see `println!` from a wasm test" | CONFIRMED (imprecise wording) | 0.2.100: "`console.*()` calls in tests are now always intercepted by default. To show them use `--nocapture`." Same release added `--nocapture` to the runner. The changelog says `console.*()`, not `println!`; on `wasm32-unknown-unknown` these are the same output path, so the practical advice holds. | CHANGELOG | 2026-08-01 |
| "wasm-bindgen CHANGELOG entry for 0.2.122 indicating that `panic=unwind` now emits modern exnref exception handling by default and requires Node 22.22.3 or newer" | CONFIRMED | Verbatim: "`-Cpanic=unwind` on wasm targets now emits modern (exnref) exception handling by default ... and requires Node.js 22.22.3+ (for `WebAssembly.JSTag`)." | CHANGELOG (0.2.122) | 2026-08-01 |
| "the repository `github.com/wasm-bindgen/wasm-bindgen` shows a last push of 2026-07-22, so it is under active daily development" | CONFIRMED | `pushed_at` 2026-07-31T19:36:10Z, not archived, 9105 stars. Still daily. | https://api.github.com/repos/wasm-bindgen/wasm-bindgen | 2026-08-01 |

### wasm-pack

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| "`wasm-pack` 0.15.0 was published 2026-05-15" | CONFIRMED | 0.15.0, 2026-05-15T08:25:44Z, still `max_stable_version`. CHANGELOG has an "Unreleased" section, no newer release. | https://crates.io/api/v1/crates/wasm-pack | 2026-08-01 |
| "the last release was 0.13.1 on 2024-10-29, a gap of roughly fifteen months ... 0.14.0 landed 2026-01-20" | CONFIRMED | 0.13.1 2024-10-29, 0.14.0 2026-01-20. Gap is 14.7 months. "Roughly fifteen" is fair. | same | 2026-08-01 |
| "0.14.0 is described as a community takeover with multiple new maintainers ... arbitrary wasm target support ... native Apple Silicon builds, a `--split-linked-modules` passthrough, and custom build profiles via `--profile`" | CONFIRMED | All four features present in the 0.14.0 CHANGELOG section (PRs #1524, #1529, #1443, #1428). | https://raw.githubusercontent.com/wasm-bindgen/wasm-pack/master/CHANGELOG.md | 2026-08-01 |
| "0.15.0 added `wasm64-unknown-unknown` support and a `--panic-unwind` flag, and vendored the project template" | CONFIRMED | PRs #1553, #1572, #1573. | same | 2026-08-01 |
| "it fixed a genuinely nasty regression where `npm install -g wasm-pack` returned a 404 because the installer referenced a previous maintainer's URL" | CONFIRMED | PR #1579 verbatim: "The 0.14.0 npm package shipped with the old `drager/wasm-pack` release URL and was never republished after the repository moved, so `npm install -g wasm-pack` failed with a 404." | same | 2026-08-01 |
| "The repository moved **again in 0.15.0**, from `drager/wasm-pack` to `wasm-bindgen/wasm-pack`" | CONFIRMED (timing imprecise) | The destination is right: `wasm-bindgen/wasm-pack`, non-fork, 7264 stars, `pushed_at` 2026-07-30. But the move happened **between** 0.14.0 and 0.15.0, not in it. PR #1579's own wording ("was never republished after the repository moved") puts the move before the 0.15.0 fix. 0.15.0 shipped the URL cleanup (#1571, #1567), not the move. | https://api.github.com/repos/wasm-bindgen/wasm-pack | 2026-08-01 |
| gotchas: "Install wasm-pack with cargo, not npm. Wrong: `npm install -g wasm-pack`, which was returning a 404" | CONFIRMED with caveat | The 404 was real and is **fixed**. PR #1579 also "adds release workflow automation so the npm package no longer requires a manual `npm publish`". So npm is no longer broken. `cargo install wasm-pack --locked` is still the right call for a reproducible pinned build, but the stated reason is now historical, not current. | CHANGELOG (0.15.0) | 2026-08-01 |
| "wasm-pack now lives alongside wasm-bindgen under the same maintainers. Treating it as abandoned in 2026 would be wrong." | CONFIRMED | Active: `pushed_at` 2026-07-30, unreleased changes queued. | https://api.github.com/repos/wasm-bindgen/wasm-pack | 2026-08-01 |

### rustwasm org sunset

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| "On 2025-07-21 the Rust project announced it was sunsetting the `rustwasm` GitHub organization" | CONFIRMED | Post dated July 21, 2025, by Alex Crichton. All 21 repos remaining in the org are `archived: true`. | https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/ | 2026-08-01 |
| "The Rust and WebAssembly Working Group had already been archived in 2024 after roughly five years of inactivity" | CONFIRMED | "In 2024 the Rust and WebAssembly Working Group was officially archived in the Rust project after ~5 years of inactivity". | same | 2026-08-01 |
| "The announcement is explicit that neither wasm-bindgen, web-sys, wasm-pack, nor gloo are being deprecated." | **REFUTED** | The post says the opposite for three of the four. Only wasm-bindgen was transferred. Verbatim: "The `wasm-bindgen` repository is going to be transferred to a new wasm-bindgen organization ... **All other repositories in the rustwasm organization are going to be archived in place or transferred to their existing maintainers if they elect to do so.**" It lists `rustwasm/wasm-pack` and `rustwasm/gloo` under "Archiving other repositories" and advises: "**If your use case critically relies on these repositories it is recommended to fork the repository.**" The post never says these projects are not deprecated. | same | 2026-08-01 |
| gotchas: "wasm-bindgen, web-sys, wasm-pack, and gloo were all transferred, not deprecated" | **REFUTED** | Same evidence. Accurate version: wasm-bindgen was transferred to a new org (web-sys rides along, it is in the same monorepo). wasm-pack was archived in `rustwasm`, handed to its existing maintainer `drager`, revived by a community takeover in 0.14.0, and only later landed in the `wasm-bindgen` org. gloo was archived. The happy outcome for wasm-pack was community rescue, not the Rust project's plan. | same | 2026-08-01 |
| "The maintained guide is at `wasm-bindgen.github.io`" / "`rustwasm.github.io` ... is a stale mirror" | CONFIRMED | `rustwasm.github.io/docs/wasm-bindgen/` returns 200 with the banner "This documentation is no longer maintained at this domain, and is now maintained at wasm-bindgen.github.io instead." `wasm-bindgen.github.io/wasm-bindgen/` returns 200 and is current. | both URLs | 2026-08-01 |

### xterm.js

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| "`@xterm/xterm` \| **6.0.0** \| 2025-12-22" | CONFIRMED | `dist-tags.latest` = 6.0.0, published 2025-12-22T13:50:12Z. | https://registry.npmjs.org/@xterm%2fxterm | 2026-08-01 |
| "`@xterm/addon-fit` \| **0.11.0** \| 2025-12-22" | CONFIRMED | 0.11.0, 2025-12-22T13:50:25Z. | https://registry.npmjs.org/@xterm%2faddon-fit | 2026-08-01 |
| "`@xterm/addon-web-links` \| **0.12.0** \| 2025-12-22" | CONFIRMED | 0.12.0, 2025-12-22T13:50:56Z. | https://registry.npmjs.org/@xterm%2faddon-web-links | 2026-08-01 |
| "`@xterm/addon-webgl` \| **0.19.0** \| 2025-12-22" | CONFIRMED | 0.19.0, 2025-12-22T13:51:03Z. | https://registry.npmjs.org/@xterm%2faddon-webgl | 2026-08-01 |
| "`@xterm/addon-clipboard` \| 0.2.0 \| 2025-12-22" | CONFIRMED | 0.2.0, 2025-12-22T13:50:21Z. | https://registry.npmjs.org/@xterm%2faddon-clipboard | 2026-08-01 |
| "`@xterm/addon-canvas` \| 0.7.0 \| 2024-04-05" | CONFIRMED | 0.7.0, 2024-04-05T14:01:55Z. Registry `modified` 2024-07-14. Frozen. | https://registry.npmjs.org/@xterm%2faddon-canvas | 2026-08-01 |
| "`xterm` \| 5.3.0 \| 2023-09-07 \| **DEPRECATED**" and the exact notice "This package is now deprecated. Move to @xterm/xterm instead." | CONFIRMED verbatim | 5.3.0, 2023-09-07T17:56:00Z. Deprecation string matches character for character. **1092 of its published versions carry the notice**, so the whole package is deprecated, not just the latest. | https://registry.npmjs.org/xterm | 2026-08-01 |
| "`xterm-addon-fit` says: 'This package is now deprecated. Move to @xterm/addon-fit instead.'" | CONFIRMED verbatim | 0.8.0, 2023-09-07T17:56:30Z, notice matches. | https://registry.npmjs.org/xterm-addon-fit | 2026-08-01 |
| "The scoped packages appeared with `@xterm/xterm` 5.4.0 on 2024-03-01, then 5.5.0 on 2024-04-05, then the 6.0.0 major on 2025-12-22" | CONFIRMED | All three dates exact. Upstream 5.4.0 release notes confirm intent: "the old `xterm` and `xterm-*` packages are now deprecated and will no longer be maintained ... mainly for security reasons to remove ambiguity around the package names and to prevent potential typosquatting attacks." | https://api.github.com/repos/xtermjs/xterm.js/releases | 2026-08-01 |
| "beta tags are being cut actively (for example `@xterm/xterm` 6.1.0-beta.291 on 2026-07-19)" | STALE (harmless) | Beta tag is now **6.1.0-beta.292**; registry `modified` 2026-07-27. Point stands, number moved. | https://registry.npmjs.org/@xterm%2fxterm | 2026-08-01 |
| "the package ships both a CommonJS entry (`lib/xterm.js`) and an ESM entry (`lib/xterm.mjs`) ... so modern bundlers resolve the ESM build without special configuration" | CONFIRMED with a caveat the spike missed | `main` = `lib/xterm.js`, `module` = `lib/xterm.mjs`, `style` = `css/xterm.css`. But **`exports` is null**: the package has no `exports` map. Resolution relies on the legacy `module` field. Bundlers honor it; Node's native ESM resolver does not. Fine for Vite, a trap for any Node-side tooling. | https://registry.npmjs.org/@xterm%2fxterm | 2026-08-01 |

### JSPI and blocking INPUT

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| "shipped in Chrome 137" | CONFIRMED | MDN BCD `webassembly/api/Suspending.json`: `chrome: {"version_added":"137"}`. Same for `SuspendError`. | https://raw.githubusercontent.com/mdn/browser-compat-data/main/webassembly/api/Suspending.json | 2026-08-01 |
| "shipped in Chrome 137 **and Firefox 139**" | **REFUTED** | MDN BCD: `firefox: {"version_added":"153"}`. Firefox 139 (2025-05-27) had it behind a flag, not shipped. | same | 2026-08-01 |
| "**It remains behind a flag in Firefox.**" | **REFUTED / STALE** | Firefox shipped JSPI **unflagged in 153**. Firefox 153.0 was released **2026-07-21**, one day before the spike was written. `LATEST_FIREFOX_VERSION` = 153.0.1. caniuse agrees: "Firefox: Supported in version 153 and later". | https://product-details.mozilla.org/1.0/firefox_versions.json + https://caniuse.com/wf-wasm-jspi | 2026-08-01 |
| "Safari removed its objection to the proposal in late 2025 and has an assigned implementer, but has not shipped it" | CONFIRMED | MDN BCD `safari: {"version_added": false}`, `safari_ios: mirror` (false). caniuse: "Safari: Not supported in any version through 26.5 and Technical Preview"; "Safari on iOS: Not supported in any version through 26.5". | BCD + caniuse | 2026-08-01 |
| "the specification reached Phase 4 at the W3C WebAssembly CG in April 2025" | UNVERIFIED (not re-checked this pass) | Not independently confirmed here. BCD now marks the feature `experimental: false, standard_track: true`, which is consistent with Phase 4. Immaterial to the recommendation. | https://raw.githubusercontent.com/mdn/browser-compat-data/main/webassembly/api/Suspending.json | 2026-08-01 |
| "JSPI is one of four focus areas comprising twenty percent of the Interop 2026 score" | CONFIRMED | "advanced `attr()`, the `getAllRecords()` method for IndexedDB, WebTransport, and the JavaScript Promise Integration API for Wasm. Together, these four areas make up 20% of the Interop 2026 score." (Twenty focus areas total; JSPI is in the four-item "adding support" group.) | https://webkit.org/blog/17818/announcing-interop-2026/ | 2026-08-01 |
| "**Verdict: not shippable in July 2026 for a course that must work in Safari.**" | **CONFIRMED. Architecture unchanged.** | Safari desktop and iOS both `false`. Global support 66.83% per caniuse. The Safari gap is the binding constraint and it has not moved. Option A stands. | BCD + caniuse | 2026-08-01 |
| (not claimed) Chrome for Android | NEW, SOURCES CONFLICT | MDN BCD says `chrome_android: {"version_added": false}`. caniuse says "Chrome for Android: Supported as of version 150". These disagree. Either way it does not change the verdict, because Safari iOS is a hard no. Worth recording so nobody later cites one source as settled. | BCD vs caniuse | 2026-08-01 |
| "`Atomics.wait()` throws a `TypeError` if the current thread cannot be blocked, explicitly including the main thread, and requires an `Int32Array` or `BigInt64Array` over a `SharedArrayBuffer`" | CONFIRMED verbatim | MDN Exceptions section: "If `typedArray` is not an `Int32Array` or `BigInt64Array` that views a `SharedArrayBuffer`." and "If the current thread cannot be blocked (for example, because it's the main thread)." | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait | 2026-08-01 |
| "`SharedArrayBuffer` ... requires two response headers ... `Cross-Origin-Opener-Policy: same-origin` / `Cross-Origin-Embedder-Policy: require-corp`" | CONFIRMED | Standard cross-origin-isolation requirement, unchanged. | https://web.dev/articles/coop-coep | 2026-08-01 |
| "GitHub Pages \| **No.** Long-standing feature request (community discussion 13309) still open" | CONFIRMED | Discussion still open and unanswered. GitHub staff (@yoannchaudet), 2023-07-10: "This is a scenario we would support with custom headers. No ETA at the moment." Most recent activity 2025-08-26. No custom-header support has shipped. | https://github.com/orgs/community/discussions/13309 | 2026-08-01 |

### Decimal arithmetic

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| "`rust_decimal` 1.42.1 was published 2026-06-12, with 1.42.0 on 2026-05-06, 1.41.0 on 2026-03-27, and 1.40.0 on 2026-01-14" | CONFIRMED | All four exact. 1.42.1 still `max_stable_version`. | https://crates.io/api/v1/crates/rust_decimal | 2026-08-01 |
| "the crate reports over 121 million total downloads" | STALE (harmless) | Now **125,300,817**. | same | 2026-08-01 |
| "It is pure Rust with no C dependencies" | CONFIRMED | Confirmed by dependency graph and by the local wasm build below (`arrayvec`, `num-traits` only, no build-script C). | Cargo.lock + README | 2026-08-01 |
| "usable on bare-metal targets such as `x86_64-unknown-none`" | CONFIRMED verbatim | README: "The no-allocator configuration is suitable for bare-metal targets such as `x86_64-unknown-none`." | https://raw.githubusercontent.com/paupino/rust-decimal/master/README.md | 2026-08-01 |
| "a 96-bit mantissa plus a scale, giving roughly 28 base-10 significant digits (29 in some cases)" | CONFIRMED verbatim | README: "a mantissa of 96 bits made up of three 32-bit unsigned integers with a fourth 32-bit unsigned integer to represent the scale/sign ... the maximum number of significant digits that can be represented is roughly 28 base-10 digits (29 in some cases)." docs.rs: "m is an integer such that -2^96 < m < 2^96, and e is an integer between 0 and 28 inclusive." | README + https://docs.rs/rust_decimal/1.42.1/rust_decimal/struct.Decimal.html | 2026-08-01 |
| "The stated MSRV is 1.67.1" | CONFIRMED for 1.42.1 | Tag `1.42.1` `Cargo.toml`: `rust-version = "1.67.1"`. README: "The current minimum compiler version is `1.67.1` which was released on `2023-02-09`." **Forward-looking note the spike could not have had: `master` is now `2.0.0-alpha.0` with `rust-version = "1.85.0"`.** A future 2.x will raise the floor. | https://raw.githubusercontent.com/paupino/rust-decimal/1.42.1/Cargo.toml | 2026-08-01 |
| "There is an optional `wasm` feature ... it enables `wasm-bindgen` support to expose `fromNumber`, `toNumber`, `fromString`, and `toString` across the JS boundary. **It is not required in order to compile to wasm.**" | CONFIRMED, and MEASURED | `Cargo.toml`: `wasm = ["dep:wasm-bindgen"]`, nothing else. README documents exactly those four methods and nothing more. Not-required half proven by building: see below. | README + Cargo.toml + local build | 2026-08-01 |
| "`fastnum` 0.7.5, published 2026-06-11" | CONFIRMED | 0.7.5, 2026-06-11T11:31:01Z, still latest. | https://crates.io/api/v1/crates/fastnum | 2026-08-01 |
| "`bigdecimal` 0.4.10, published 2025-12-27" | CONFIRMED | 0.4.10, 2025-12-27T23:58:11Z. | https://crates.io/api/v1/crates/bigdecimal | 2026-08-01 |
| "`fixed` 1.31.0, published 2026-03-20 ... binary fixed-point" | CONFIRMED | 1.31.0, 2026-03-20T09:00:22Z. | https://crates.io/api/v1/crates/fixed | 2026-08-01 |
| "`rusty-money` 0.5.0, published 2026-01-14" | CONFIRMED | 0.5.0, 2026-01-14T05:46:43Z. | https://crates.io/api/v1/crates/rusty-money | 2026-08-01 |

### Alternatives not chosen

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| "**Trunk** (0.21.14 stable, published 2025-05-08; a 0.22.0-beta.1 exists from 2026-03-10)" | STALE | Stable still 0.21.14 (2025-05-08), so the argument holds. But `newest_version` is now **0.22.0-beta.2, published 2026-07-24**. The beta line moved. | https://crates.io/api/v1/crates/trunk | 2026-08-01 |
| "**cargo-component** (0.21.1, published 2025-03-18)" | CONFIRMED | 0.21.1, 2025-03-18T16:10:46Z. Unchanged, now ~16.5 months stale. Argument strengthened. | https://crates.io/api/v1/crates/cargo-component | 2026-08-01 |

### Serving, CSP, MIME, Vite

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| "For this to work, `.wasm` files should be returned with an `application/wasm` MIME type by the server." | CONFIRMED verbatim | Quote matches MDN exactly. | https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static | 2026-08-01 |
| "If a page has a CSP header and `'wasm-unsafe-eval'` isn't specified in the `script-src` directive, WebAssembly is blocked from loading and executing on the page." | CONFIRMED verbatim | Quote matches MDN exactly. | https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/script-src | 2026-08-01 |
| "If `'unsafe-eval'` is present it supersedes `'wasm-unsafe-eval'`, so do not list both." | CONFIRMED | MDN: "If the `'unsafe-eval'` source keyword is used, then this overrides any occurrence of `'wasm-unsafe-eval'` in the CSP policy." | same | 2026-08-01 |
| "Vite natively supports importing `.wasm` as an ES module and with an `?init` suffix" | CONFIRMED | Vite docs confirm both. | https://vite.dev/guide/features | 2026-08-01 |
| "a directly imported `.wasm` module is async and needs top-level await ... **verified against community issue reports rather than official docs**" | CONFIRMED, upgrade the citation | This is now in the **official** Vite docs: "Because a WebAssembly module is instantiated asynchronously, a directly imported `.wasm` file behaves as an async module and requires top-level `await` support." No longer a community-only claim. | https://vite.dev/guide/features | 2026-08-01 |

### thiserror (in the repo, absent from both docs)

| Claim (quoted) | Verdict | Current actual value | Source URL | Accessed |
|---|---|---|---|---|
| `thiserror = "2"` in `crates/natural-core/Cargo.toml` | CONFIRMED | Major 2 is current. Latest **2.0.19**, published 2026-07-18. `"2"` resolves to 2.0.19 in the lockfile. No 3.x exists. Compiles clean to wasm32. Neither doc mentions thiserror; that is a documentation gap, not an error. | https://crates.io/api/v1/crates/thiserror | 2026-08-01 |

---

## Previously-unverified items: resolution attempts

The spike listed five items under "Explicitly unverified". Four were in scope. Results:

### 1. rust_decimal on `wasm32-unknown-unknown`: RESOLVED, both ways

**CI coverage exists.** The spike said it could not find a `wasm32-unknown-unknown` entry
in the published CI matrix. It is there. `.github/workflows/main.yml` has a dedicated
`wasm_tests` job named "WASM Tests":

```yaml
targets: wasm32-unknown-unknown
- name: Check wasm build
  run: cargo check --target wasm32-unknown-unknown --features wasm
- name: Run clippy for wasm
  run: cargo clippy --target wasm32-unknown-unknown --features wasm
- name: Run wasm tests
  run: wasm-pack test --node --features wasm
```

One caveat worth keeping: every step passes `--features wasm`, so upstream CI exercises
the wasm-bindgen-glue configuration, **not** the `default-features = false, features = ["std"]`
configuration this project uses. Upstream CI does not prove our config builds.

(Aside, not our problem but notable: that job installs wasm-pack from
`rustwasm.github.io/wasm-pack/installer/init.sh`, a domain the sunset moved off. Expect it
to rot.)

**So I built ours.** Rust 1.97.1, `wasm32-unknown-unknown` target installed:

```
$ cargo build -p natural-core --target wasm32-unknown-unknown --release
   Compiling thiserror v2.0.19
   Compiling rust_decimal v1.42.1
   Compiling arrayvec v0.7.8
   Compiling num-traits v0.2.19
   Compiling natural-core v0.1.0
    Finished `release` profile [optimized] target(s) in 4.69s
```

Exit 0. Zero warnings. This settles it: `rust_decimal` 1.42.1 with
`default-features = false, features = ["std"]`, plus `thiserror` 2.0.19, compiles cleanly
to `wasm32-unknown-unknown` on Rust 1.97.1 **without** the `wasm` feature. The "verify on
day one" task in the gotchas doc is done and should be marked done rather than left as an
open action.

### 2. Is `@xterm/addon-canvas` formally deprecated? RESOLVED: no, but it was deleted

**Not npm-deprecated.** No version of `@xterm/addon-canvas` carries a `deprecated` field,
including the 0.8.0-beta line. Contrast with `xterm`, where 1092 versions carry one. The
spike was right to refuse to call it deprecated.

**But the evidence is stronger than "unmaintained".** The addon was **removed from the
xterm.js repository** in the 6.0.0 release:

- `addons/` at tag `5.5.0`: includes `addon-canvas`.
- `addons/` at tag `6.0.0`: **no `addon-canvas`**.
- `addons/` on `main` today: **no `addon-canvas`**.

It did not merely miss the 6.0.0 train. It was dropped from the monorepo as part of it.
The recommendation (use WebGL, fall back to the built-in DOM renderer, do not add canvas)
is correct and can now be stated with a hard citation instead of a hedge.

Also worth noting: 6.0.0 added addons the spike does not list, including
`addon-progress` and `addon-image`, and `main` now carries `addon-web-fonts`.

### 3. Does xterm.js 6.0.0 still need `style-src 'unsafe-inline'`? RESOLVED: yes

Settled by inspecting the shipped bundle (`@xterm/xterm@6.0.0/lib/xterm.js`, 488,663 bytes
via jsDelivr):

| Pattern | Count | CSP-restricted? |
|---|---|---|
| `document.createElement("style")` with `.textContent` assigned | 4 | **Yes.** A dynamically created `<style>` element is checked against `style-src` on insertion. Needs `'unsafe-inline'` or a nonce, and xterm.js exposes no nonce hook. |
| `setAttribute("style", ...)` | 1 | **Yes** (`style-src-attr` / `'unsafe-inline'`). |
| `element.style.prop = ...` (CSSOM) | 127 | **No.** CSP does not restrict CSSOM property assignment. |
| `insertRule` | 2 | No, once the sheet itself is allowed. |

The three style elements are the theme sheet, the dimensions sheet, and the scrollbar
sheet, all populated at runtime with interpolated colors and pixel dimensions. Nonces
cannot be applied to them without upstream support.

**Verdict: keep `style-src 'unsafe-inline'`.** The spike's defensive inclusion was correct.
Change the wording from "unverified, test and tighten" to "required, do not attempt to
tighten without an upstream nonce API." This is source inspection, not a live browser test.
A browser test would be the final word, but the mechanism is unambiguous.

### 4. Railway static hosting and custom headers: PARTIALLY RESOLVED

Railway has **no declarative headers file**. There is no `_headers` equivalent, and no
`railway.json` header block. Railway's own docs for static hosting and for SPA routing do
not document any custom-response-header mechanism for the zero-config Railpack static path,
and Railpack's config-file docs do not either.

What Railway does document is deploying your own web server. Its SPA guide states "Caddy is
a lightweight web server that works well for serving SPAs on Railway. Most Railway SPA
templates use Caddy," with configuration via a `Caddyfile` in the project root, and also
covers nginx via `nginx.conf`. Taking over the server that way gives full header control
through Caddy's `header` directive, so COOP and COEP **are** reachable on Railway, just not
through zero-config static hosting.

Corrected row for the hosting table: "Yes, but only by deploying your own server (Caddy or
nginx). No declarative headers file. Not available on zero-config static hosting."

This remains off the critical path, because Option A needs no headers.

### 5. Natural's maximum digit count

Out of scope for this pass. Correctly deferred to the language-subset spec.

---

## Corrections required

### `research/06-rust-wasm-toolchain.md`

**C1. Section 1, "The organizational change training data will miss". REFUTED claim.**

Replace:

> The announcement is explicit that neither wasm-bindgen, web-sys, wasm-pack, nor gloo are
> being deprecated. wasm-bindgen moved to a new project-specific `wasm-bindgen` GitHub
> organization with additional maintainers.

With:

> The announcement is narrower than it is often summarized. Only wasm-bindgen was
> transferred, to a new project-specific `wasm-bindgen` GitHub organization with additional
> maintainers; web-sys rides along because it lives in the same monorepo. Everything else
> was archived: "All other repositories in the rustwasm organization are going to be
> archived in place or transferred to their existing maintainers if they elect to do so."
> The post names `rustwasm/wasm-pack` and `rustwasm/gloo` in the archive list and tells
> readers "If your use case critically relies on these repositories it is recommended to
> fork the repository." wasm-pack's survival was a community rescue after the fact, not the
> Rust project's plan. That distinction matters, because it is why wasm-pack's revival
> arrived through a maintainer handoff and a takeover rather than a transfer.

**C2. Section 1, wasm-bindgen MSRV bullet. One stale, one refuted.**

Replace:

> The library MSRV rose to 1.71 in 0.2.106, with a stated policy that library MSRV changes
> come with a minor version bump. CLI MSRV was raised to 1.82 in 0.2.118 and adjusted again
> around 0.2.122.

With:

> MSRV has moved repeatedly. The library floor rose to 1.71 in 0.2.106 (which also
> introduced the policy that library MSRV changes come with a minor version bump), then to
> 1.76 in 0.2.118; the library's `rust-version` is 1.77 today. The CLI floor rose to 1.82 in
> 0.2.101 and to 1.86 in 0.2.118; the CLI's `rust-version` is 1.86 today.

**C3. Section 1, JsStatic bullet. Stale replacement.**

Replace:

> `JsStatic` was deprecated in 0.2.93 in favor of `#[wasm_bindgen(thread_local)]`.

With:

> `JsStatic` was deprecated in 0.2.93 in favor of `#[wasm_bindgen(thread_local)]`, which was
> itself deprecated in 0.2.96 in favor of `#[wasm_bindgen(thread_local_v2)]`. Use
> `thread_local_v2`; it is the only one that supports `no_std`, and static strings require
> it outright. Tutorials citing either earlier form are two deprecations behind.

**C4. Section 1, wasm-pack repository move. Timing.**

Replace "The repository moved **again in 0.15.0**, from `drager/wasm-pack` to
`wasm-bindgen/wasm-pack`" with "The repository moved from `drager/wasm-pack` to
`wasm-bindgen/wasm-pack` between 0.14.0 and 0.15.0; 0.15.0 shipped the cleanup of URLs left
pointing at the old location."

**C5. Section 1, the npm caveat. Now historical.**

Append to the "Caveat to record honestly" paragraph:

> As of 0.15.0 the npm channel is repaired: the same PR that inlined the installer also
> added release automation so the npm package publishes without a manual step. The
> recommendation to use `cargo install wasm-pack --locked` stands on reproducibility
> grounds, not because npm is currently broken.

**C6. Section 1, target selection. Wrong tier.**

Replace "`wasm32v1-none` is Tier 3 and `no_std` only" with "`wasm32v1-none` is Tier 2
without host tools and `no_std` only".

**C7. Section 2, addon-canvas. Upgrade the hedge to a fact.**

Replace:

> Flagged as partially unverified: I did not find an official statement that
> `@xterm/addon-canvas` is deprecated, only that it has not been updated in over two years.

With:

> Resolved: it is not npm-deprecated (no version carries a `deprecated` field, unlike
> `xterm`, where 1092 versions do). It is something stronger. `addon-canvas` was removed
> from the xterm.js repository in 6.0.0: it is present in `addons/` at tag 5.5.0 and absent
> at tag 6.0.0 and on `main`. It did not miss the release train; it was dropped.

**C8. Section 2, package entry points. Add the missing caveat.**

Append to the paragraph ending "so modern bundlers resolve the ESM build without special
configuration":

> One caveat: `@xterm/xterm` 6.0.0 has **no `exports` map**. Resolution depends on the
> legacy `module` field. Bundlers such as Vite honor it; Node's native ESM resolver does
> not. Irrelevant for the browser build, relevant if any Node-side tooling ever imports the
> package directly.

**C9. Section 3, Option C JSPI. Two refuted statements.**

Replace:

> the specification reached Phase 4 at the W3C WebAssembly CG in April 2025 and shipped in
> Chrome 137 and Firefox 139. It remains behind a flag in Firefox.

With:

> the specification reached Phase 4 at the W3C WebAssembly CG in April 2025, shipped in
> Chrome 137, and shipped unflagged in Firefox 153 (released 2026-07-21). Firefox 139 had it
> behind a flag; that flag is gone.

Then replace the Safari sentence's surrounding verdict framing so the argument rests where
it actually rests:

> Safari removed its objection in late 2025 and has an assigned implementer, but has not
> shipped it: MDN browser-compat-data records `safari: false` and `safari_ios: false`, and
> caniuse shows no support through Safari 26.5 or Technical Preview, with global support at
> 66.83%.

Keep the verdict. **Change the reason.** The blocker is Safari alone now, not "Firefox and
Safari". Also add:

> Mobile Chrome is unclear: MDN browser-compat-data records `chrome_android: false` while
> caniuse claims support from Chrome for Android 150. The sources disagree. It does not
> change the verdict, because Safari on iOS is a hard no and iOS is not optional for a
> course.

**C10. Section 3, hosting table. Resolve the Railway row.**

Replace the Railway cell with:

> Yes, but only by deploying your own server. Railway has no declarative headers file. Its
> SPA guide directs users to Caddy with a `Caddyfile` (or nginx with `nginx.conf`), which
> gives full `header` control. Zero-config Railpack static hosting documents no
> custom-header mechanism. Verified 2026-08-01.

**C11. Section 4, rust_decimal wasm CI. Resolve.**

Replace the entire "One thing I could not fully verify" paragraph with:

> Resolved. `rust_decimal` does have `wasm32-unknown-unknown` CI: `.github/workflows/main.yml`
> carries a `wasm_tests` job running `cargo check`, `cargo clippy`, and `wasm-pack test --node`
> against that target. One gap remains: every step passes `--features wasm`, so upstream CI
> covers the wasm-bindgen-glue configuration and not ours. Ours was verified directly on
> 2026-08-01: `cargo build -p natural-core --target wasm32-unknown-unknown --release` on Rust
> 1.97.1 compiles `rust_decimal` 1.42.1 (`default-features = false, features = ["std"]`) and
> `thiserror` 2.0.19 clean in 4.69s with zero warnings.

**C12. Section 4, rust_decimal MSRV. Add the forward-looking note.**

Append after "The stated MSRV is 1.67.1, well below our 1.97.1":

> That is the floor for the 1.x line. `master` is now `2.0.0-alpha.0` with `rust-version = "1.85.0"`,
> so a future 2.x will raise it. Still well below our pin.

**C13. Section 4, download count.** "over 121 million" becomes "over 125 million".

**C14. Section 4, Trunk.** "a 0.22.0-beta.1 exists from 2026-03-10" becomes "a 0.22.0-beta.2
exists from 2026-07-24". The stable release is still 0.21.14 from 2025-05-08, so the argument
is unchanged.

**C15. Section 6, CSP. Resolve the style-src hedge.**

Replace:

> **I did not verify against xterm.js documentation whether v6.0.0 still requires inline
> styles**, so treat that as a starting point to test and tighten rather than a verified
> requirement.

With:

> Verified by inspecting the shipped bundle on 2026-08-01: `@xterm/xterm@6.0.0/lib/xterm.js`
> creates four `<style>` elements at runtime and assigns their `textContent`, plus one
> `setAttribute("style", ...)`. Dynamically created `<style>` elements are subject to
> `style-src`, and xterm.js exposes no nonce hook, so `'unsafe-inline'` is required. Do not
> try to tighten it. The 127 `element.style.prop = ...` assignments are CSSOM and are not
> CSP-restricted, which is why the requirement is narrower than it looks.

**C16. Section 6, Vite top-level await. Upgrade the citation.**

The top-level-await requirement is now stated in the official Vite docs, not just community
issues. Move that bullet out of the "verified against community issue reports" framing. The
`optimizeDeps.exclude` bullet stays community-sourced.

**C17. Sources section.** Remove Railway, addon-canvas, xterm.js style-src, and rust_decimal
wasm CI from "Explicitly unverified" and record the resolutions. Only Natural's maximum digit
count stays.

**C18. Section 7, tier 4 note.** Adjust the `println!` sentence to match the changelog's actual
wording: 0.2.100 intercepts `console.*()` calls, which is the same output path `println!` uses
on this target. Practical advice is unchanged.

**C19. Add thiserror.** Neither document mentions `thiserror`, which is one of only two
dependencies in `natural-core`. Add a line to the section 4 or section 6 dependency discussion:
`thiserror = "2"` (latest 2.0.19, published 2026-07-18); major 2 is current, no 3.x exists, and
it compiles clean to wasm32.

### `docs/gotchas-rust-wasm.md`

**G1. Header.** "All versions below verified as of 2026-07-22" becomes "as of 2026-08-01
(re-verified; see `research/verification/v06-rust-wasm-toolchain.md`)".

**G2. "The documentation moved" bullet. REFUTED clause.**

Replace:

> The `rustwasm` GitHub org was sunset on 2025-07-21; wasm-bindgen, web-sys, wasm-pack, and
> gloo were all transferred, not deprecated.

With:

> The `rustwasm` GitHub org was sunset on 2025-07-21 and every repository still in it is now
> archived. Only wasm-bindgen was transferred to a new org (web-sys came with it, same
> monorepo). wasm-pack and gloo were archived in place or handed back to their maintainers;
> the announcement told users to fork if they depended on them. wasm-pack was rescued
> afterward by a community takeover and now lives in the `wasm-bindgen` org. Do not repeat
> the shorthand that all four were "transferred, not deprecated".

**G3. Deprecated-patterns bullet. Stale replacement.**

Replace:

> * `JsStatic` (deprecated 0.2.93). Use `#[wasm_bindgen(thread_local)]`.

With:

> * `JsStatic` (deprecated 0.2.93). Its replacement `#[wasm_bindgen(thread_local)]` was
>   itself deprecated in 0.2.96. Use `#[wasm_bindgen(thread_local_v2)]`.

**G4. Wasm target table. Wrong tier.**

`| wasm32v1-none | Tier 3, no_std only. |` becomes
`| wasm32v1-none | Tier 2 without host tools, no_std only. |`

**G5. wasm-pack npm bullet. Now historical.**

Replace:

> **Install wasm-pack with cargo, not npm.** Wrong: `npm install -g wasm-pack`, which was
> returning a 404 until 0.15.0 fixed a stale installer URL from a previous maintainer.
> Correct: `cargo install wasm-pack --locked`.

With:

> **Install wasm-pack with cargo, not npm.** `cargo install wasm-pack --locked` gives a
> pinned, reproducible install. Historical context, since older advice still circulates:
> `npm install -g wasm-pack` returned a 404 from 0.14.0 until 0.15.0, which inlined the
> installer and automated npm publishing. The npm channel works again; cargo is still the
> right choice here.

**G6. addon-canvas bullet. Strengthen.**

Replace:

> **Do not add `@xterm/addon-canvas`.** Its latest is 0.7.0 from 2024-04-05 and it did not
> ride the 6.0.0 release train.

With:

> **Do not add `@xterm/addon-canvas`.** It was removed from the xterm.js repository in
> 6.0.0: present in `addons/` at tag 5.5.0, absent at tag 6.0.0 and on `main`. npm still
> serves 0.7.0 from 2024-04-05 and it is not formally deprecated, so nothing will warn you.
> Use the WebGL addon and let xterm.js fall back to its built-in DOM renderer.

**G7. JSPI bullet. REFUTED clause.**

Replace:

> it reached Phase 4 in April 2025 and shipped in Chrome 137, but it is still behind a flag
> in Firefox and unshipped in Safari as of 2026-07-22.

With:

> it reached Phase 4 in April 2025, shipped in Chrome 137, and shipped unflagged in Firefox
> 153 on 2026-07-21. Safari is the remaining blocker: unsupported on desktop and iOS through
> 26.5 and Technical Preview, with global support at 66.83% as of 2026-08-01. Mobile Chrome
> is disputed between MDN and caniuse. Since the course must work in Safari on iOS, the
> conclusion is unchanged. Revisit when Safari ships, not when Interop 2026 closes.

**G8. rust_decimal "verify the wasm build on day one" bullet. DONE, replace it.**

Replace the whole bullet with:

> **wasm build verified.** `cargo build -p natural-core --target wasm32-unknown-unknown --release`
> on Rust 1.97.1 compiles `rust_decimal` 1.42.1 (`default-features = false, features = ["std"]`)
> and `thiserror` 2.0.19 clean in 4.69s, zero warnings (2026-08-01). Upstream also runs a
> `wasm_tests` CI job against `wasm32-unknown-unknown`, though only with `--features wasm`.
> No action outstanding.

**G9. CSP style-src bullet. Resolve.**

Replace:

> xterm.js may need `style-src 'unsafe-inline'`. Unverified for 6.0.0; start permissive and
> tighten with a test.

With:

> xterm.js **requires** `style-src 'unsafe-inline'`. Verified against the shipped 6.0.0
> bundle on 2026-08-01: it creates four `<style>` elements at runtime and assigns their
> `textContent`, plus one `setAttribute("style", ...)`. There is no nonce hook. Do not try
> to tighten this.

**G10. Blocking-INPUT bullet, Railway row.**

"Railway static: unverified" becomes "Railway: only by deploying your own Caddy or nginx.
No declarative headers file, and no documented mechanism on zero-config static hosting."

**G11. Add thiserror.** Add to the decimal-arithmetic or toolchain section: `thiserror = "2"`
is current (2.0.19, 2026-07-18), no 3.x exists, compiles clean to wasm32. It is half of
`natural-core`'s dependency surface and appears nowhere in the gotchas.

**G12. New bullet, npm resolution.** Under "npm package names", add: `@xterm/xterm` 6.0.0
ships no `exports` map. Resolution goes through the legacy `module` field, which bundlers
honor and Node's native ESM resolver does not.

---

## Net effect on architecture

**None. Option A stands unchanged.**

The one finding with the power to move it was JSPI, and it moved in the right direction
without moving far enough. Firefox 153 shipping it unflagged on 2026-07-21 means two of
three major engines now support JSPI. Safari does not, on desktop or iOS, and shows no
version in MDN browser-compat-data at all. Global support is 66.83%. A browser-based course
that must work on an iPad cannot depend on it.

Everything else that touches the architecture held: `Atomics.wait()` still throws on the main
thread, SharedArrayBuffer still requires cross-origin isolation, and GitHub Pages still cannot
set the headers (discussion 13309 open since 2021, GitHub's last word in 2023 was "no ETA",
last activity 2025-08-26). The reasons to avoid Option B are all intact.

The non-negotiable constraint in `CLAUDE.md`, that statement execution be an explicit loop
with an explicit frame stack, is unaffected and should not be revisited.

One thing got easier rather than harder: `rust_decimal` on wasm32 is no longer an open risk.
It is built and measured. The "first task of the build spike" is retired.

---

## Sources

All accessed 2026-08-01 unless noted. Verification pass began 2026-07-31.

**Registries and manifests**

- https://static.rust-lang.org/dist/channel-rust-stable.toml
- https://api.github.com/repos/rust-lang/rust/releases
- https://api.github.com/repos/rust-lang/rust/releases/tags/1.85.0
- https://crates.io/api/v1/crates/wasm-bindgen
- https://crates.io/api/v1/crates/wasm-bindgen-cli
- https://crates.io/api/v1/crates/wasm-bindgen-test
- https://crates.io/api/v1/crates/wasm-pack
- https://crates.io/api/v1/crates/rust_decimal
- https://crates.io/api/v1/crates/rust_decimal/1.42.1
- https://crates.io/api/v1/crates/thiserror
- https://crates.io/api/v1/crates/fastnum
- https://crates.io/api/v1/crates/bigdecimal
- https://crates.io/api/v1/crates/fixed
- https://crates.io/api/v1/crates/rusty-money
- https://crates.io/api/v1/crates/trunk
- https://crates.io/api/v1/crates/cargo-component
- https://registry.npmjs.org/@xterm%2fxterm
- https://registry.npmjs.org/@xterm%2faddon-fit
- https://registry.npmjs.org/@xterm%2faddon-web-links
- https://registry.npmjs.org/@xterm%2faddon-webgl
- https://registry.npmjs.org/@xterm%2faddon-canvas
- https://registry.npmjs.org/@xterm%2faddon-clipboard
- https://registry.npmjs.org/xterm
- https://registry.npmjs.org/xterm-addon-fit
- https://product-details.mozilla.org/1.0/firefox_versions.json
- https://product-details.mozilla.org/1.0/firefox.json

**Repositories and changelogs**

- https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/CHANGELOG.md
- https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/Cargo.toml
- https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/crates/cli/Cargo.toml
- https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/crates/macro-support/src/parser.rs
- https://raw.githubusercontent.com/wasm-bindgen/wasm-pack/master/CHANGELOG.md
- https://api.github.com/repos/wasm-bindgen/wasm-bindgen
- https://api.github.com/repos/wasm-bindgen/wasm-pack
- https://api.github.com/repos/drager/wasm-pack
- https://api.github.com/orgs/rustwasm/repos
- https://api.github.com/repos/xtermjs/xterm.js/releases
- https://api.github.com/repos/xtermjs/xterm.js/contents/addons (refs: `main`, `6.0.0`, `5.5.0`)
- https://raw.githubusercontent.com/paupino/rust-decimal/master/README.md
- https://raw.githubusercontent.com/paupino/rust-decimal/master/Cargo.toml
- https://raw.githubusercontent.com/paupino/rust-decimal/1.42.1/Cargo.toml
- https://raw.githubusercontent.com/paupino/rust-decimal/master/.github/workflows/main.yml
- https://cdn.jsdelivr.net/npm/@xterm/xterm@6.0.0/lib/xterm.js

**Official documentation**

- https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/
- https://doc.rust-lang.org/nightly/rustc/platform-support.html
- https://doc.rust-lang.org/edition-guide/rust-2024/index.html
- https://wasm-bindgen.github.io/wasm-bindgen/
- https://rustwasm.github.io/docs/wasm-bindgen/ (carries the "no longer maintained at this domain" banner)
- https://docs.rs/rust_decimal/1.42.1/rust_decimal/struct.Decimal.html
- https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait
- https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static
- https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/script-src
- https://raw.githubusercontent.com/mdn/browser-compat-data/main/webassembly/api/Suspending.json
- https://raw.githubusercontent.com/mdn/browser-compat-data/main/webassembly/api/SuspendError.json
- https://caniuse.com/wf-wasm-jspi
- https://webkit.org/blog/17818/announcing-interop-2026/
- https://vite.dev/guide/features
- https://docs.railway.com/guides/static-hosting
- https://docs.railway.com/guides/spa-routing-configuration
- https://railpack.com/config/file
- https://www.firefox.com/en-US/firefox/153.0/releasenotes/

**Community sources (weaker, used where no official source exists)**

- https://github.com/orgs/community/discussions/13309

**Local measurement**

- `cargo build -p natural-core --target wasm32-unknown-unknown --release`, Rust 1.97.1
  (8bab26f4f 2026-07-14), run 2026-08-01 in this repository. Exit 0, 4.69s, zero warnings.
