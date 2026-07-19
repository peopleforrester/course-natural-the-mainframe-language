# Natural: The Mainframe Language (Interactive Course)

An interactive, browser-based course teaching the **Natural** programming language
(the Software AG 4GL used with ADABAS on IBM mainframes and still taught at
institutions such as the University of Texas).

The course uses the **virtual terminal (VTT) model**: student instructions on the
left, a live terminal on the right, so learners write and run Natural code without
installing anything locally. The VTT pattern is adapted from the "Unleash an Agent,
Watch It Burn" workshop (ttyd + nginx split-pane). See
[`reference/vtt-model/`](reference/vtt-model/).

## Status

Phase 1 (Inception): research and specification. Nothing is built yet. See
[`PROJECT_STATE.md`](PROJECT_STATE.md) for the current phase and
[`research/`](research/) for the July 2026 research spikes that ground the spec.

## Repository layout

| Path | What lives here |
|------|-----------------|
| `research/` | Dated July 2026 research spikes: language history, vendor, training path, market, emulator feasibility |
| `reference/vtt-model/` | Extraction of the reusable VTT (instructions-left / terminal-right) architecture |
| `spec/` | The course specification (written after research synthesis) |
| `PROJECT_STATE.md` | Current lifecycle phase and plan |
| `decisions.md` | Append-only decision log |

## Open questions the research must answer

1. Who owns and maintains Natural as of July 2026, and what is the current version?
2. Can Natural run without a physical mainframe (local runtime, community edition,
   Docker), and is a CLI emulator or a WASM/browser build feasible?
3. What is the realistic beginner-to-competent training path?
4. Is there a market and audience worth publishing a course for?
