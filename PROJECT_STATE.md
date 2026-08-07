# Project State: course-natural-the-mainframe-language

Phase: 3.3 Promote (content audit remediation shipped; modules 1 to 15 complete)
Approved: 2026-07-22T18:45:25Z by Michael (sha256:135613afc4c9)

## Lifecycle
- [x] 1.1 Research
- [x] 1.2 Plan
- [x] 1.3 Approve
- [x] 2.1 Test
- [x] 2.2 Implement
- [x] 2.3 Verify
- [x] 3.1 Stage
- [x] 3.2 Confirm CI (local gate; hosted CI intentionally not used)
- [x] 3.3 Promote

## Contracts
- 2026-07-22T18:45:25Z (sha256:135613afc4c9) `spec/course-spec.md` approved by
  Michael. Binding terms:
  - v1 scope is **Tier 1 only, modules 1 to 9**. Tier 2 is a later release.
  - Interpreter is **Rust**, compiled to WASM, client-side against xterm.js. A
    scoped Rust-to-WASM spike runs **before** P1 implementation.
  - Product intent is a **revenue course** sold B2B.
  - v1 is positioned and priced as **release 1**, and must **not** promise
    job-readiness until Tier 2 ships.
  - Repo stays private through the build.
  - Do not host the free Community Edition as the course backend (license wall).

  Changes to these terms require /prd-amend and re-approval.

## Current Plan
Build an interactive, browser-based course for the Natural mainframe language,
using the VTT (instructions-left / terminal-right) model extracted from the
"Unleash an Agent, Watch It Burn" workshop.

The full plan is `spec/course-spec.md` (DRAFT, awaiting approval). Headline
decisions:
- Terminal = a custom Natural-subset interpreter in Rust, compiled to WASM, running
  100% client-side against xterm.js. Zero backend, zero per-student cost. Native CLI
  in a ttyd container is the fallback.
- Free vendor Community Edition exists but is personal-use-only, so we own the
  runtime rather than host theirs.
- 16 modules in three delivery tiers by what the browser interpreter can honestly
  execute. Tier 1 (modules 1 to 9) is the fastest path to a live course.

Phase 1.1 (done): VTT extracted, five research spikes complete and committed.
Phase 1.2 (done): spec drafted.
Phase 1.3 (pending): Michael's approval, plus answers to the four open questions in
spec section 8 (v1 scope depth, confirm Rust, product intent, public-repo timing).

## Branch & Tests
- Branch: staging
- Working tree: clean after the M-A commit
- Tests: 270 passing (`cargo test --workspace`), clippy clean under `-D warnings`,
  `cargo fmt --check` clean
- Toolchain: rustc 1.97.1 via the pinned `rust-toolchain.toml`, which independently
  confirmed the version claim in spike 06
- Wasm: `cargo build -p natural-core --target wasm32-unknown-unknown` succeeds with
  `rust_decimal` and `thiserror` in the tree, which settles both the day-one wasm check
  and the open `rust_decimal` wasm question from the gotchas doc.
- Verification: `scripts/verify.sh` is the gate, enforced by a pre-push hook.
  No hosted CI by decision. Install the hook on a fresh clone with
  `scripts/install-hooks.sh`. The gate now has six stages: the added one extracts every
  code sample from `web/lessons.js` so `tests/lesson_samples.rs` can run the source a
  learner is actually shown. Nothing had been checking published samples before, which is
  how a course full of invalid syntax shipped.

## Phase History
- 2026-07-19 repo initialized, VTT model extracted, research spikes launched
- 2026-08-04 content audit across all 15 lessons (7 adversarial passes, 118 recorded
  claims). 79 confirmed, 20 misleading, 15 refuted, 4 unverified. All 11 remediation items
  closed and promoted to main. Highest-severity finding: DEFINE MAP, END-MAP, TEXT and
  FIELD do not exist in Natural, so lessons 14 and 15 were rebuilt on real map objects.
- 2026-08-07 corrected the audit tallies everywhere they appeared. The earlier figures
  came from a word grep rather than a row count. Verdict tables now committed under
  `docs/content-audit/` so the counts are checkable.
