# Project State: course-natural-the-mainframe-language

Phase: 1.2 Plan
Approved: pending

## Lifecycle
- [x] 1.1 Research
- [x] 1.2 Plan
- [ ] 1.3 Approve  ← you are here
- [ ] 2.1 Test
- [ ] 2.2 Implement
- [ ] 2.3 Verify
- [ ] 3.1 Stage
- [ ] 3.2 Confirm CI
- [ ] 3.3 Promote

## Contracts
None sealed yet. Phase 1.3 approval will record the first.

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
- Branch: main (bootstrap), staging created for ongoing work
- Working tree: scaffolding
- Last CI: none yet

## Phase History
- 2026-07-19 repo initialized, VTT model extracted, research spikes launched
