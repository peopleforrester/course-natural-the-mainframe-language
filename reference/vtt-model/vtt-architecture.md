# VTT Model: Instructions-Left / Terminal-Right

Extraction of the reusable virtual-terminal architecture from the
"Unleash an Agent, Watch It Burn" workshop
(`~/repos/events/Unleash_an_Agent_Watch_It_Burn`). This is the pattern the Natural
course reuses: a split page with student instructions on the left and a live,
in-browser terminal on the right.

Source files in the Unleash repo:

- `gitops/ai-layer/web/lab.html`: the split-pane frontend
- `images/web-terminal/Dockerfile`: the ttyd terminal container
- `images/web-terminal/entrypoint.sh`: environment wiring + ttyd launch
- `gitops/ai-layer/console.conf`: nginx front door that proxies the terminal

## The three layers

### 1. Frontend split page (`lab.html`)

A flexbox page, full viewport height, no body scroll:

- `header`: brand bar with external links (buttons that open sibling tabs).
- `.crumb`: breadcrumb with identity pulled from URL query params
  (`?cluster=&user=&role=`), so a link can carry per-student context.
- `.split`: the two-column body:
  - `.guide` (left, ~40%, `max-width:560px`, `min-width:340px`, scrolls
    independently). Holds `.step` cards that are **collapsible** (closed by
    default, open state persisted per step in `localStorage`), and `.cmd`
    blocks that **copy to clipboard on click** (`navigator.clipboard` with a
    `document.execCommand('copy')` fallback).
  - `.termcol` (right, flexes to fill). Holds a **tab bar** plus one or more
    `<iframe class="tpane" src="/terminal/">`. A `+` button appends another
    iframe, so each terminal tab is an **independent** terminal session
    (Katacoda/KodeKloud style). An `x` closes a tab.
- Mobile: at `max-width:820px` the split stacks vertically (guide on top,
  terminal below at 70vh).

Everything is a single self-contained HTML file: inline CSS, inline JS, no build
step, no external dependencies. That matters for portability.

### 2. Terminal backend (`ttyd`)

`ttyd` (https://github.com/tsl0922/ttyd) is a small C program that exposes a
command over a WebSocket to an `xterm.js` frontend it serves itself. It is the
whole terminal: no separate xterm.js wiring, no custom websocket code.

From the Dockerfile (`images/web-terminal/Dockerfile`):

- Base `debian:bookworm-slim` (glibc, needed because some baked-in tools have no
  musl build).
- Downloads the **ttyd 1.7.7 static binary** (arch-aware: `x86_64` / `aarch64`).
- Bakes in the domain toolset (in the Unleash case: kubectl, aws, helm, etc.).
  For Natural this is where the Natural runtime/emulator goes.
- Creates a non-root `student` user (uid 1000) so the pod can enforce
  `runAsNonRoot`.
- `EXPOSE 7681`; entrypoint launches ttyd.

From the entrypoint (`images/web-terminal/entrypoint.sh`), the launch line is:

```bash
exec ttyd -p 7681 -W -b /terminal -t fontSize=14 \
  -t 'theme={"background":"#0f1117"}' bash --rcfile "$HOME/.bashrc"
```

Key flags:

- `-p 7681`: listen port.
- `-W`: **writable** terminal (interactive input allowed). Without it the
  terminal is read-only.
- `-b /terminal`: serve under the base path `/terminal` so the front door can
  reverse-proxy it on a subpath.
- `-t ...`: xterm theming (font size, colors).
- Trailing `bash --rcfile ...`: the command ttyd runs. **This is the swap
  point**: replace `bash` (or the rcfile it sources) with the Natural
  runtime/REPL to make the terminal a Natural session.

The entrypoint also seeds a custom `.bashrc` (MOTD, prompt, PATH) before
launching. That is where a Natural course would auto-start the emulator or print
a "type `natural` to begin" banner.

### 3. nginx front door (`console.conf`)

An `nginx-unprivileged` server on :8080 that:

- Serves the static HTML (`lab.html`, etc.) from a ConfigMap-mounted root.
- Reverse-proxies the terminal with **WebSocket upgrade** headers:

```nginx
location /terminal/ {
  proxy_pass http://web-terminal:7681;
  proxy_http_version 1.1;
  proxy_set_header Upgrade $http_upgrade;
  proxy_set_header Connection "upgrade";
  proxy_set_header Host $host;
  proxy_read_timeout 3600s;   # keep long-lived terminals alive
}
```

The `Upgrade`/`Connection` headers and `proxy_http_version 1.1` are mandatory or
the WebSocket handshake fails and the terminal never connects. The long
`proxy_read_timeout` keeps idle terminals from being dropped.

## What this means for the Natural course

**Server-hosted path (proven, low risk):** one container that bakes in the
Natural runtime instead of the k8s toolset, ttyd launching the Natural REPL/shell
instead of bash, and the same nginx split-page front door. The `lab.html`
frontend is reusable almost verbatim (swap the instruction content). This is the
default and it works today.

**Client-side-only path (Michael's stretch goal, unproven):** drop ttyd and the
container entirely. Compile a Natural interpreter to WebAssembly and run it
against `xterm.js` directly in the browser, with an in-memory filesystem for the
student's source. No backend, infinitely scalable, zero per-student cost. Whether
this is feasible depends entirely on whether a Natural interpreter can be built
or ported to WASM. That is an explicit research question
(`research/05-emulator-and-wasm-feasibility.md`).

## Hosting options for the server path

- A cluster (as Unleash does) with one console Service per environment.
- A single container per student or a shared multiplexed host.
- The whole thing is static-HTML + one small container, so it can also run on a
  cheap VPS, Railway, Fly.io, or similar, without Kubernetes.
