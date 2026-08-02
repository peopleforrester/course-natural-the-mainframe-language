// ABOUTME: Drives the WASM interpreter against an xterm.js 24x80 screen, pumping the
// ABOUTME: resumable state machine so a suspended INPUT hands control back to the page.

import { Terminal } from './vendor/xterm.js';
import init, { NaturalSession } from './pkg/natural_wasm.js';
import { LESSONS, LIBRARY } from './lessons.js';

// A Model 2 screen. Fixed, with no scrollback and no fit addon, because a real 3270
// neither scrolls nor reflows. See research/08-mainframe-emulators-3270.md.
const COLS = 80;
const ROWS = 24;

const term = new Terminal({
  cols: COLS,
  rows: ROWS,
  scrollback: 0,
  cursorBlink: true,
  convertEol: true, // emit "\n" from Rust and let xterm supply the carriage return
  fontFamily: '"3270", ui-monospace, Menlo, Consolas, monospace',
  fontSize: 15,
  theme: {
    background: '#0b1207',
    foreground: '#33ff66',
    cursor: '#33ff66',
    selectionBackground: 'rgba(51,255,102,0.28)',
  },
});

const PALETTES = {
  green: { background: '#0b1207', foreground: '#33ff66', cursor: '#33ff66' },
  amber: { background: '#140d02', foreground: '#ffb000', cursor: '#ffb000' },
};

/** Interpreter state, and the input line being collected while suspended. */
const state = {
  session: null,
  awaitingInput: false,
  buffer: '',
  running: false,
  /// The map currently displayed: its entry fields and which one is being filled.
  screen: null,
  screenValues: [],
  screenIndex: 0,
};

// ---- the Operator Information Area ----

const oiaSys = document.getElementById('oiaSys');
const oiaMsg = document.getElementById('oiaMsg');

/**
 * Sets the OIA. On a 3270 this strip reports what the terminal is doing; here it makes
 * otherwise invisible interpreter state visible, which is a teaching surface in itself.
 */
function setOia(system, message) {
  oiaSys.textContent = system;
  oiaMsg.textContent = message || '';
}

// ---- terminal plumbing ----

function writeLine(text) {
  term.write(text + '\n');
}

/**
 * Advances the interpreter until it needs the learner or finishes.
 *
 * This is the loop the whole architecture exists for. Nothing blocks: when the program
 * wants input the pump returns and the page waits for keystrokes, so the browser stays
 * responsive with no cross-origin isolation headers.
 */
function pump() {
  if (!state.session) return;
  for (;;) {
    const step = state.session.step();
    switch (step.kind) {
      case 'output':
        writeLine(step.text);
        break;
      case 'input':
        state.awaitingInput = true;
        state.buffer = '';
        term.write(step.text + ' ');
        // "X Protected" style convention: the machine is waiting on the operator.
        setOia('4  A', 'Input Inhibited: waiting');
        return;
      case 'screen':
        presentScreen(step.text);
        return;
      case 'done':
        state.running = false;
        writeLine('');
        writeLine('*** Program finished. ***');
        setOia('4  A', 'Ready');
        return;
      case 'error':
        state.running = false;
        writeLine('');
        writeLine('*** ' + step.text);
        setOia('4  A', 'X  Program check');
        return;
      default:
        return;
    }
  }
}

/**
 * Draws a map and starts collecting its entry fields.
 *
 * The grid arrives pre-rendered from the interpreter, so the page does not need to know
 * the 3270 field model to display a panel. The field list is fetched separately because
 * the page does need it to know what to collect.
 */
function presentScreen(rendered) {
  term.reset();
  const rows = rendered.split('\n');
  rows.forEach((row, i) => {
    term.write(row.replace(/\s+$/, ''));
    if (i < rows.length - 1) term.write('\n');
  });

  const fields = state.session
    .screenFields()
    .split('\n')
    .filter(Boolean)
    .map((row) => {
      const [name, r, c, width, kind] = row.split('|');
      return { name, row: Number(r), column: Number(c), width: Number(width), kind };
    });

  state.screen = fields;
  state.screenValues = fields.map(() => '');
  state.screenIndex = 0;
  state.awaitingInput = false;
  setOia('4  A', 'Input Inhibited: waiting');
  showAidBar(true);
  focusScreenField();
}

/** Positions the cursor on the field being filled and echoes what has been typed. */
function focusScreenField() {
  const field = state.screen[state.screenIndex];
  if (!field) return;
  // xterm rows and columns are one-based in the CUP escape, matching a map definition.
  term.write('\u001b[' + field.row + ';' + field.column + 'H');
  const typed = state.screenValues[state.screenIndex];
  term.write(field.kind === 'hidden' ? '*'.repeat(typed.length) : typed);
}

/** Sends the completed screen back to the interpreter with the AID key pressed. */
function submitScreen(aid) {
  if (!state.screen) return;
  const payload = state.screen
    .map((f, i) => f.name + '=' + state.screenValues[i])
    .join('\n');
  state.screen = null;
  showAidBar(false);
  term.write('\u001b[24;1H\n');
  setOia('4  A', 'X SYSTEM');
  const complaint = state.session.provideScreen(payload, aid);
  if (complaint) {
    writeLine('*** ' + complaint);
    setOia('4  A', 'X  Program check');
    return;
  }
  pump();
}

function showAidBar(visible) {
  document.getElementById('aidbar').style.display = visible ? 'flex' : 'none';
}

term.onData((data) => {
  // A map is being filled in: keystrokes go to the field under the cursor.
  if (state.screen) {
    for (const ch of data) {
      const field = state.screen[state.screenIndex];
      if (!field) return;
      if (ch === '\r' || ch === '\n') {
        if (state.screenIndex < state.screen.length - 1) {
          state.screenIndex += 1;
          focusScreenField();
        } else {
          submitScreen('ENTR');
        }
        return;
      }
      if (ch === '\u007f') {
        if (state.screenValues[state.screenIndex].length > 0) {
          state.screenValues[state.screenIndex] =
            state.screenValues[state.screenIndex].slice(0, -1);
          term.write('\b \b');
        }
        continue;
      }
      if (ch < ' ') continue;
      // A numeric field accepts digits only, which is what its attribute byte means.
      if (field.kind === 'numeric' && !/[0-9.\-]/.test(ch)) {
        setOia('4  A', 'X  Numeric field');
        continue;
      }
      if (state.screenValues[state.screenIndex].length >= field.width) continue;
      state.screenValues[state.screenIndex] += ch;
      term.write(field.kind === 'hidden' ? '*' : ch);
    }
    return;
  }

  if (!state.awaitingInput) return;
  for (const ch of data) {
    if (ch === '\r' || ch === '\n') {
      term.write('\n');
      const value = state.buffer;
      state.buffer = '';
      state.awaitingInput = false;
      setOia('4  A', 'X SYSTEM');
      const complaint = state.session.provideInput(value);
      if (complaint) {
        // A rejected value re-prompts rather than ending the lesson.
        writeLine('*** ' + complaint);
        state.awaitingInput = true;
        term.write('Try again: ');
        setOia('4  A', 'Input Inhibited: waiting');
        return;
      }
      pump();
      return;
    }
    if (ch === '') {
      if (state.buffer.length > 0) {
        state.buffer = state.buffer.slice(0, -1);
        term.write('\b \b');
      }
      continue;
    }
    if (ch >= ' ') {
      state.buffer += ch;
      term.write(ch);
    }
  }
});

// ---- running a program ----

function runSource(source) {
  term.reset();
  state.session = new NaturalSession(source);
  // The lesson library travels with every run, so any lesson may CALLNAT these.
  for (const [name, body] of Object.entries(LIBRARY)) {
    state.session.addObject(name, body);
  }
  state.screen = null;
  state.screenValues = [];
  state.screenIndex = 0;
  state.awaitingInput = false;
  state.buffer = '';
  state.running = true;
  setOia('4  A', 'X SYSTEM');
  pump();
}

/** The editor is the lesson's code block; this is what Run executes. */
let currentSource = '';

function loadSource(source) {
  currentSource = source;
}

// ---- lesson rendering ----

const guideSteps = document.getElementById('steps');
const lessonTitle = document.getElementById('lessontitle');
const lessonLede = document.getElementById('lessonlede');
const lessonSelect = document.getElementById('lessonsel');

function renderLesson(lesson) {
  lessonTitle.textContent = lesson.title;
  lessonLede.textContent = lesson.lede;
  guideSteps.innerHTML = '';

  lesson.steps.forEach((step, index) => {
    const card = document.createElement('div');
    card.className = 'step' + (index === 0 ? ' open' : '');

    const head = document.createElement('div');
    head.className = 'h';
    head.innerHTML =
      '<div class="num">' + (index + 1) + '</div><div class="t"></div>';
    head.querySelector('.t').textContent = step.title;
    const chev = document.createElement('div');
    chev.className = 'chev';
    chev.textContent = index === 0 ? '▾' : '▸';
    head.appendChild(chev);

    const body = document.createElement('div');
    body.className = 'b';
    body.innerHTML = step.body;

    // Every code block runs on click, rather than only copying. The Packt VTT showed
    // click-to-run is the interaction that keeps a learner moving.
    if (step.code) {
      const block = document.createElement('div');
      block.className = 'cmd';
      block.textContent = step.code;
      const go = document.createElement('button');
      go.className = 'go';
      go.textContent = 'Run ▶';
      block.appendChild(go);
      block.addEventListener('click', () => {
        loadSource(step.code);
        runSource(step.code);
        block.classList.add('sent');
        setTimeout(() => block.classList.remove('sent'), 1200);
      });
      body.appendChild(block);
    }

    head.addEventListener('click', () => {
      const open = card.classList.toggle('open');
      chev.textContent = open ? '▾' : '▸';
    });

    card.appendChild(head);
    card.appendChild(body);
    guideSteps.appendChild(card);
  });

  // Preload the lesson's first runnable example so Run works immediately.
  const first = lesson.steps.find((s) => s.code);
  loadSource(first ? first.code : '');
}

// ---- startup ----

async function main() {
  await init();

  term.open(document.getElementById('screen'));

  // Exposed deliberately. Renderers may draw to a canvas rather than the DOM, so the
  // buffer API is the only reliable way to assert on screen content from a test, and it
  // is also the hook a lesson checker uses to read what the learner saw.
  window.term = term;
  window.naturalRun = runSource;
  window.naturalSubmitScreen = submitScreen;
  window.naturalState = state;

  LESSONS.forEach((lesson, index) => {
    const option = document.createElement('option');
    option.value = String(index);
    option.textContent = lesson.title;
    lessonSelect.appendChild(option);
  });
  lessonSelect.addEventListener('change', () => {
    renderLesson(LESSONS[Number(lessonSelect.value)]);
    term.reset();
    setOia('4  A', 'Ready');
  });

  document.getElementById('runbtn').addEventListener('click', () => {
    if (currentSource) runSource(currentSource);
  });
  document.getElementById('resetbtn').addEventListener('click', () => {
    term.reset();
    state.session = null;
    state.awaitingInput = false;
    state.running = false;
    state.screen = null;
    showAidBar(false);
    setOia('4  A', 'Ready');
  });

  // The AID keys a map can be ended with. ENTER confirms; PF3 is the near-universal
  // mainframe convention for "go back" and every lesson that offers a choice uses it.
  document.querySelectorAll('#aidbar button').forEach((button) => {
    button.addEventListener('click', () => submitScreen(button.dataset.aid));
  });
  showAidBar(false);
  document.getElementById('palette').addEventListener('change', (event) => {
    const name = event.target.value;
    document.body.classList.toggle('amber', name === 'amber');
    term.options.theme = { ...term.options.theme, ...PALETTES[name] };
  });

  renderLesson(LESSONS[0]);
  setOia('4  A', 'Ready');
  writeLine('Natural teaching interpreter ready.');
  writeLine('Pick a lesson on the left and press Run.');
}

main();
