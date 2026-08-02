// ABOUTME: Drives the WASM interpreter against an xterm.js 24x80 screen, pumping the
// ABOUTME: resumable state machine so a suspended INPUT or map hands control back to the page.

import { Terminal } from './vendor/xterm.js';
import init, { NaturalSession } from './pkg/natural_wasm.js';
import { LESSONS, LIBRARY } from './lessons.js';

// A Model 2 screen. Fixed, with no scrollback and no fit addon, because a real 3270
// neither scrolls nor reflows. See research/08-mainframe-emulators-3270.md.
const COLS = 80;
const ROWS = 24;

const PALETTES = {
  green: { background: '#0b1207', foreground: '#33ff66', cursor: '#33ff66' },
  amber: { background: '#140d02', foreground: '#ffb000', cursor: '#ffb000' },
};

const term = new Terminal({
  cols: COLS,
  rows: ROWS,
  scrollback: 0,
  cursorBlink: true,
  convertEol: true, // emit "\n" from Rust and let xterm supply the carriage return
  fontFamily: '"3270", ui-monospace, Menlo, Consolas, monospace',
  fontSize: 15,
  theme: { ...PALETTES.green, selectionBackground: 'rgba(51,255,102,0.28)' },
});

/** Interpreter state, plus whatever is being collected while the program is suspended. */
const state = {
  session: null,
  awaitingInput: false,
  buffer: '',
  running: false,
  /** The map on screen: its entry fields, the values typed, and which field has focus. */
  screen: null,
  screenValues: [],
  screenIndex: 0,
  /** Everything the current run produced, so an exercise check can inspect it. */
  transcript: [],
  lastError: null,
};

const els = {
  editor: document.getElementById('editor'),
  gutter: document.getElementById('gutter'),
  oiaSys: document.getElementById('oiaSys'),
  oiaMsg: document.getElementById('oiaMsg'),
  aidbar: document.getElementById('aidbar'),
  steps: document.getElementById('steps'),
  title: document.getElementById('lessontitle'),
  lede: document.getElementById('lessonlede'),
  select: document.getElementById('lessonsel'),
  progressFill: document.getElementById('progressfill'),
  progressText: document.getElementById('progresstext'),
};

/**
 * Sets the Operator Information Area. On a 3270 this strip reports what the terminal is
 * doing; here it makes otherwise invisible interpreter state visible.
 */
function setOia(system, message) {
  els.oiaSys.textContent = system;
  els.oiaMsg.textContent = message || '';
}

function showPane(which) {
  document.getElementById('editorpane').hidden = which !== 'editor';
  document.getElementById('termpane').hidden = which !== 'terminal';
  document.getElementById('tabeditor').classList.toggle('active', which === 'editor');
  document.getElementById('tabterminal').classList.toggle('active', which === 'terminal');
}

// ---- the editor ----

function syncGutter() {
  const lines = els.editor.value.split('\n').length;
  let text = '';
  for (let i = 1; i <= Math.max(lines, 1); i++) text += i + '\n';
  els.gutter.textContent = text;
  els.gutter.scrollTop = els.editor.scrollTop;
}

els.editor.addEventListener('input', syncGutter);
els.editor.addEventListener('scroll', () => {
  els.gutter.scrollTop = els.editor.scrollTop;
});
// Tab indents rather than leaving the editor, which is what a code editor should do.
els.editor.addEventListener('keydown', (event) => {
  if (event.key !== 'Tab') return;
  event.preventDefault();
  const { selectionStart: start, selectionEnd: end, value } = els.editor;
  els.editor.value = value.slice(0, start) + '  ' + value.slice(end);
  els.editor.selectionStart = els.editor.selectionEnd = start + 2;
  syncGutter();
});

function loadSource(source) {
  els.editor.value = source;
  syncGutter();
}

// ---- running ----

function writeLine(text) {
  state.transcript.push(text);
  term.write(text + '\n');
}

/**
 * Advances the interpreter until it needs the learner or finishes.
 *
 * This is the loop the whole architecture exists for. Nothing blocks: when the program
 * wants input the pump returns and the page waits, so the browser stays responsive with no
 * cross-origin isolation headers.
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
        state.lastError = step.text;
        writeLine('');
        writeLine('*** ' + step.text);
        setOia('4  A', 'X  Program check');
        return;
      default:
        return;
    }
  }
}

function runSource(source) {
  showPane('terminal');
  term.reset();
  state.session = new NaturalSession(source);
  // The lesson library travels with every run, so any lesson may CALLNAT these.
  for (const [name, body] of Object.entries(LIBRARY)) {
    state.session.addObject(name, body);
  }
  state.awaitingInput = false;
  state.buffer = '';
  state.running = true;
  state.screen = null;
  state.screenValues = [];
  state.screenIndex = 0;
  state.transcript = [];
  state.lastError = null;
  setOia('4  A', 'X SYSTEM');
  pump();
}

// ---- maps ----

/**
 * Draws a map and starts collecting its entry fields.
 *
 * The grid arrives pre-rendered from the interpreter, so the page does not need to know the
 * 3270 field model to display a panel. The field list is fetched separately because the
 * page does need it to know what to collect and how to constrain it.
 */
function presentScreen(rendered) {
  term.reset();
  const rows = rendered.split('\n');
  rows.forEach((row, i) => {
    term.write(row.replace(/\s+$/, ''));
    if (i < rows.length - 1) term.write('\n');
  });

  state.screen = state.session
    .screenFields()
    .split('\n')
    .filter(Boolean)
    .map((row) => {
      const [name, r, c, width, kind] = row.split('|');
      return { name, row: Number(r), column: Number(c), width: Number(width), kind };
    });
  state.screenValues = state.screen.map(() => '');
  state.screenIndex = 0;
  state.awaitingInput = false;
  setOia('4  A', 'Input Inhibited: waiting');
  els.aidbar.style.display = 'flex';
  focusScreenField();
}

/** Positions the cursor on the field being filled and echoes what has been typed. */
function focusScreenField() {
  const field = state.screen && state.screen[state.screenIndex];
  if (!field) return;
  // The CUP escape is one-based in both axes, matching how a map declares positions.
  term.write('[' + field.row + ';' + field.column + 'H');
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
  els.aidbar.style.display = 'none';
  term.write('[24;1H\n');
  setOia('4  A', 'X SYSTEM');
  const complaint = state.session.provideScreen(payload, aid);
  if (complaint) {
    writeLine('*** ' + complaint);
    state.lastError = complaint;
    setOia('4  A', 'X  Program check');
    return;
  }
  pump();
}

// ---- keyboard ----

term.onData((data) => {
  // A map is on screen: keystrokes go to the field under the cursor.
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
      if (ch === '') {
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
        setOia('4  A', 'X  Numeric field only');
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

// ---- progress ----

const PROGRESS_KEY = 'natural-course-progress';

function loadProgress() {
  try {
    return new Set(JSON.parse(localStorage.getItem(PROGRESS_KEY) || '[]'));
  } catch {
    return new Set();
  }
}

const progress = loadProgress();

function totalExercises() {
  return LESSONS.reduce(
    (sum, lesson) => sum + lesson.steps.filter((s) => s.exercise).length,
    0
  );
}

function markDone(id) {
  progress.add(id);
  localStorage.setItem(PROGRESS_KEY, JSON.stringify([...progress]));
  renderProgress();
}

function renderProgress() {
  const total = totalExercises();
  const done = progress.size;
  els.progressText.textContent = done + ' / ' + total;
  els.progressFill.style.width = total ? (done / total) * 100 + '%' : '0%';
}

// ---- exercise checking ----

/**
 * Runs a program to completion without the terminal, so a check can inspect what it did.
 *
 * Answers come from the exercise rather than from the learner, because a check must not
 * depend on somebody typing at the right moment.
 */
function evaluate(source, answers) {
  const session = new NaturalSession(source);
  for (const [name, body] of Object.entries(LIBRARY)) session.addObject(name, body);

  const lines = [];
  let errored = null;
  let supplied = 0;
  let guard = 0;

  for (;;) {
    if (++guard > 40000) {
      errored = 'the program did not finish';
      break;
    }
    const step = session.step();
    if (step.kind === 'output') {
      lines.push(step.text);
      continue;
    }
    if (step.kind === 'input') {
      session.provideInput(answers[supplied++] ?? '');
      continue;
    }
    if (step.kind === 'screen') {
      const payload = session
        .screenFields()
        .split('\n')
        .filter(Boolean)
        .map((row) => row.split('|')[0] + '=' + (answers[supplied++] ?? ''))
        .join('\n');
      session.provideScreen(payload, 'ENTR');
      continue;
    }
    if (step.kind === 'error') {
      errored = step.text;
      break;
    }
    break;
  }

  return {
    lines,
    errored,
    text: lines.join('\n'),
    field: (name) => session.fieldValue(name),
    committedCount: session.committedRecordCount(),
  };
}

// ---- lesson rendering ----

function renderLesson(lessonIndex) {
  const lesson = LESSONS[lessonIndex];
  els.title.textContent = lesson.title;
  els.lede.textContent = lesson.lede;
  els.steps.innerHTML = '';

  lesson.steps.forEach((step, stepIndex) => {
    const id = lessonIndex + 1 + '|' + (stepIndex + 1);
    const card = document.createElement('div');
    card.className = 'step' + (stepIndex === 0 ? ' open' : '');
    if (progress.has(id)) card.classList.add('done');

    const head = document.createElement('div');
    head.className = 'h';
    head.innerHTML = '<div class="num"></div><div class="t"></div>';
    head.querySelector('.num').textContent = progress.has(id) ? '✓' : stepIndex + 1;
    head.querySelector('.t').textContent = step.title;
    const chev = document.createElement('div');
    chev.className = 'chev';
    chev.textContent = stepIndex === 0 ? '▾' : '▸';
    head.appendChild(chev);
    head.addEventListener('click', () => {
      const open = card.classList.toggle('open');
      chev.textContent = open ? '▾' : '▸';
    });

    const body = document.createElement('div');
    body.className = 'b';
    body.innerHTML = step.body;

    // Every code block loads into the editor and runs, rather than only copying.
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

    if (step.exercise) {
      body.appendChild(buildExercise(step.exercise, id, card, head));
    }

    card.appendChild(head);
    card.appendChild(body);
    els.steps.appendChild(card);
  });

  // Load the lesson's first runnable example. A lesson with nothing to run clears the
  // editor rather than leaving the previous lesson's code sitting there, which reads as
  // though it belongs to what is now on screen.
  const first = lesson.steps.find((s) => s.code || s.exercise);
  loadSource(first ? first.code || (first.exercise && first.exercise.starter) || '' : '');
}

function buildExercise(exercise, id, card, head) {
  const box = document.createElement('div');
  box.className = 'exercise';

  const heading = document.createElement('div');
  heading.className = 'xh';
  heading.textContent = 'Your turn';
  box.appendChild(heading);

  const task = document.createElement('p');
  task.className = 'xtask';
  task.innerHTML = exercise.task;
  box.appendChild(task);

  const actions = document.createElement('div');
  actions.className = 'xact';

  const load = document.createElement('button');
  load.className = 'load';
  load.textContent = 'Load starter';
  load.addEventListener('click', () => {
    loadSource(exercise.starter || '');
    showPane('editor');
  });

  const check = document.createElement('button');
  check.className = 'check';
  check.textContent = 'Check my answer';

  const result = document.createElement('div');
  result.className = 'xresult';

  check.addEventListener('click', () => {
    let verdict;
    try {
      verdict = exercise.check(evaluate(els.editor.value, exercise.answers || []));
    } catch (error) {
      verdict = { pass: false, message: 'The check could not run: ' + error.message };
    }
    result.className = 'xresult show ' + (verdict.pass ? 'pass' : 'fail');
    result.textContent = (verdict.pass ? '✓ ' : '✗ ') + verdict.message;
    if (verdict.pass) {
      markDone(id);
      card.classList.add('done');
      head.querySelector('.num').textContent = '✓';
    }
  });

  actions.appendChild(load);
  actions.appendChild(check);
  box.appendChild(actions);
  box.appendChild(result);
  return box;
}

// ---- startup ----

async function main() {
  await init();
  term.open(document.getElementById('screen'));

  // Exposed deliberately. Renderers may draw to a canvas rather than the DOM, so the
  // buffer API is the only reliable way to assert on screen content from a test.
  window.term = term;
  window.naturalRun = runSource;
  window.naturalEvaluate = evaluate;
  window.naturalState = state;
  window.naturalSubmitScreen = submitScreen;

  LESSONS.forEach((lesson, index) => {
    const option = document.createElement('option');
    option.value = String(index);
    option.textContent = lesson.title;
    els.select.appendChild(option);
  });
  els.select.addEventListener('change', () => {
    renderLesson(Number(els.select.value));
    term.reset();
    showPane('editor');
    setOia('4  A', 'Ready');
  });

  document.getElementById('runbtn').addEventListener('click', () => {
    if (els.editor.value.trim()) runSource(els.editor.value);
  });
  document.getElementById('resetbtn').addEventListener('click', () => {
    term.reset();
    state.session = null;
    state.awaitingInput = false;
    state.running = false;
    state.screen = null;
    els.aidbar.style.display = 'none';
    setOia('4  A', 'Ready');
  });
  document.getElementById('palette').addEventListener('change', (event) => {
    const name = event.target.value;
    document.body.classList.toggle('amber', name === 'amber');
    term.options.theme = { ...term.options.theme, ...PALETTES[name] };
  });
  document.querySelectorAll('.tab').forEach((tab) => {
    tab.addEventListener('click', () => showPane(tab.dataset.pane));
  });
  els.aidbar.querySelectorAll('button').forEach((button) => {
    button.addEventListener('click', () => submitScreen(button.dataset.aid));
  });
  els.aidbar.style.display = 'none';

  renderLesson(0);
  renderProgress();
  showPane('editor');
  setOia('4  A', 'Ready');
  writeLine('Natural teaching interpreter ready.');
  writeLine('Edit the program on the EDITOR tab, then press Run.');
}

main();
