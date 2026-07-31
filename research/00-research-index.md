# Research Index (July 2026)

Deep research spikes grounding the Natural course spec. Each spike is dated and
cites primary sources per the recency-verification discipline: version numbers,
ownership, and "current" claims are verified against live sources, not memory.

| # | Spike | Question it answers |
|---|-------|---------------------|
| 01 | Identity, vendor, history | What is Natural, who made/owns it now, current version, licensing |
| 02 | Technical training curriculum | Language model, syntax, the beginner-to-competent path |
| 03 | Academia and job market | The UT connection, mainframe skills gap, who hires, demand, pay |
| 04 | Existing courses and resources | Official docs, books, tutorials, competing courses, community |
| 05 | Emulator and WASM feasibility | Can Natural run locally / in a CLI / in the browser (WASM) |
| 06 | Rust and WASM toolchain | New-technology adoption spike: current toolchain, xterm.js, INPUT strategy, decimals |
| 07 | Output formatting semantics | Exact WRITE and DISPLAY widths, spacing, and edit masks |
| 08 | Mainframe emulators and 3270 | Hercules, turnkey distros, browser 3270, green-screen look, map model |
| 09 | Curriculum validation | Our module order against seven real published Natural syllabi |

Spike 06 also produced `docs/gotchas-rust-wasm.md`, which is required reading before
writing any Rust or WASM code in this repo.

Cross-cutting reference (not a web spike): `reference/vtt-model/vtt-architecture.md`
documents the reusable instructions-left / terminal-right VTT extracted from the
Unleash workshop.

## Synthesis

After all five land, the synthesis and the resulting course spec go in `spec/`.


## Verification pass (2026-08-01)

Every spike above was adversarially re-verified against primary sources, with agents
instructed to refute rather than confirm. Findings and required corrections are in
`research/verification/`, and the corrections have been applied back to the spikes, the
spec, the lesson outline, `CLAUDE.md`, and `docs/gotchas-rust-wasm.md`.

The verification files are the audit trail. Where a spike and its verification file
disagree, the verification file is newer and wins.
