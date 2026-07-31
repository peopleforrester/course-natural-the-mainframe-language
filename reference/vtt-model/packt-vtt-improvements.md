# What the Packt VTT Adds Over the Unleashed One

The Natural course's split-pane frontend should be based on the **Packt** version, not the
Unleashed one. Both are in this directory for comparison:

- `unleashed-lab.html` (268 lines) from `~/repos/events/Unleash_an_Agent_Watch_It_Burn`
- `packt/lab.html` (475 lines) from `~/repos/events/Packt-agentic-devops`
- `packt/web-terminal.yaml` and `packt/Dockerfile.web-terminal` for the server-side path

The base architecture is documented in `vtt-architecture.md`. This file records only what
the Packt revision improved, because those improvements were bought with debugging time
and should not be rediscovered.

## 1. Commands land in the terminal on click

The single biggest usability gain. In the Unleashed version, clicking a command copied it
and the student then had to click the terminal and press Ctrl+Shift+V. In the Packt
version the command is **injected straight into the running terminal**, ready to run.

The mechanism, from `packt/lab.html`:

```js
// ttyd exposes the xterm Terminal instance as window.term, and the lab page and terminal
// are same-origin, so call xterm's own paste() API directly.
var w = iframe.contentWindow;
if (w && w.term && typeof w.term.paste === 'function') {
  w.term.focus();
  w.term.paste(text);
}
```

Two failure modes are recorded in that file's own comments, and both are worth keeping in
mind because they are not obvious:

- Dispatching a synthetic `ClipboardEvent` on two targets **double-pasted in Chrome**.
- **Firefox strips `clipboardData`** from a synthetic paste event, so the same approach
  silently did nothing there.

The fallback, used only when a ttyd build does not expose `window.term`, is a **single**
synthetic paste dispatched on `.xterm-helper-textarea`, never two. The page also copies to
the clipboard regardless, so manual paste still works if the bridge is blocked.

The toast text is honest about the fallback: "Sent to your terminal, press Enter. If
nothing appears, click the terminal and press Ctrl+Shift+V."

## 2. Mixed tab types in one tab bar

The tab strip holds terminal tabs and non-terminal panes together. Terminal panes carry a
numeric `data-i`; the others do not, which is how the paste bridge tells them apart:

```js
if (active && (active.classList.contains('mappane') || isNaN(parseInt(active.dataset.i,10)))) {
  showTerm(0);   // flip to a real terminal so the command lands somewhere visible
}
```

Packt uses this for an architecture blueprint, a component endpoints page, and a tutor
pane. The Natural course would use it for a **reference card** (statement syntax, format
letters) and later a **map/screen preview**.

## 3. Embedded panels can drive the terminal

Non-terminal panes post a message to the parent, which routes it to the terminal:

```js
window.addEventListener('message', function(e){
  if (e && e.data && e.data.type === 'vtt-run' && typeof e.data.text === 'string') {
    sendToTerminal(e.data.text);
  }
});
```

Useful for a clickable syntax reference where an example can be run without retyping.

## 4. First step open by default

Small but real. The Unleashed page started with every step collapsed, which reads as an
empty page. Packt opens step 0 and remembers per-step state after that:

```js
var saved = localStorage.getItem('packt-step-' + idx);
var open = saved === null ? (idx === 0) : (saved === '1');
```

## 5. One pod instead of several services

`packt/web-terminal.yaml` runs nginx and ttyd as two containers in a single pod, with
nginx proxying `/terminal/` to ttyd on loopback. The Unleashed version split these across
separate Services. The single-pod shape is simpler to host and is the better starting
point for the server-side fallback path.

## What changes for the Natural course

Our primary architecture has **no ttyd and no iframe**: the interpreter is WASM on the
same page as xterm.js. That makes the hardest part of the above trivial. There is no
cross-document boundary, so "send this command to the terminal" is a direct call into our
own terminal instance, and the Chrome double-paste and Firefox `clipboardData` problems
cannot occur at all.

What still carries over:

- The split-pane layout, collapsible steps, and per-step `localStorage` state.
- Click-to-run rather than click-to-copy, as the interaction model.
- Mixed tab types, for a syntax reference pane beside the terminal.
- Opening the first step by default.
- Keeping clipboard copy as a visible fallback.

The ttyd container files are kept here for the documented server-side fallback path, which
is only used if a lesson outgrows the browser sandbox.
