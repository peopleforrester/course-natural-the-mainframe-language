// ABOUTME: End-to-end walkthrough. Serves web/, drives every lesson step in a real Chrome,
// ABOUTME: and asserts on the terminal buffer, because the DOM renderer does not paint headless.

import { execFileSync, spawn } from 'node:child_process';
import { createRequire } from 'node:module';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, '..');
const PORT = Number(process.env.PORT || 8791);
const BASE = `http://127.0.0.1:${PORT}`;

// Puppeteer is not a project dependency; this script is opt-in and resolves it from wherever
// it is installed rather than adding a browser download to every clone.
const require = createRequire(import.meta.url);

// Resolve the global npm prefix rather than hardcoding one machine's layout.
let globalPuppeteer = null;
try {
  const prefix = execFileSync('npm', ['root', '-g'], { encoding: 'utf8' }).trim();
  if (prefix) globalPuppeteer = resolve(prefix, 'puppeteer');
} catch {
  /* npm may not be on PATH; the other candidates still apply */
}

let puppeteer;
for (const candidate of [process.env.PUPPETEER_PATH, 'puppeteer', globalPuppeteer].filter(
  Boolean,
)) {
  try {
    puppeteer = require(candidate);
    break;
  } catch {
    /* try the next */
  }
}
if (!puppeteer) {
  console.error(
    'puppeteer not found. Install it (npm i -g puppeteer) or set PUPPETEER_PATH.\n' +
      'This walkthrough is deliberately not part of scripts/verify.sh, because it needs a browser.',
  );
  process.exit(2);
}

const findings = [];
const note = (where, kind, detail) => findings.push({ where, kind, detail });

const server = spawn('python3', ['-m', 'http.server', String(PORT)], {
  cwd: resolve(root, 'web'),
  stdio: 'ignore',
});
const stop = () => server.kill();
process.on('exit', stop);

await new Promise((r) => setTimeout(r, 1200));

const browser = await puppeteer.launch({
  headless: 'new',
  args: ['--no-sandbox', '--disable-dev-shm-usage'],
});
const page = await browser.newPage();

const pageErrors = [];
const consoleErrors = [];
const failedRequests = [];

await page.evaluateOnNewDocument(() => {
  window.__problems = [];
  window.addEventListener('unhandledrejection', (e) =>
    window.__problems.push('REJECTION: ' + String(e.reason?.stack ?? e.reason)),
  );
  window.addEventListener('error', (e) =>
    window.__problems.push('ERROR: ' + String(e.error?.stack ?? e.message)),
  );
});
page.on('pageerror', (e) => pageErrors.push(String(e.stack ?? e)));
page.on('console', (m) => {
  if (m.type() === 'error') consoleErrors.push(m.text());
});
page.on('requestfailed', (r) => failedRequests.push(`${r.url()} :: ${r.failure()?.errorText}`));
page.on('response', (r) => {
  if (r.status() >= 400) failedRequests.push(`${r.url()} :: HTTP ${r.status()}`);
});

await page.goto(`${BASE}/index.html`, { waitUntil: 'networkidle2', timeout: 60000 });
await page.waitForFunction(
  () => typeof window.term === 'object' && typeof window.naturalRun === 'function',
  { timeout: 30000 },
);

/**
 * Reads the terminal.
 *
 * Through the buffer, never the DOM: xterm's renderer may draw to a canvas, and headless
 * Chrome paints nothing into `.xterm-rows` even when the buffer is correct. Scraping the DOM
 * here produced two confident false diagnoses before this was written down.
 *
 * Lines are joined rather than returned per row, because the 80-column grid wraps a long
 * diagnostic mid-word and a per-row match would miss it.
 */
const readTerm = () =>
  page.evaluate(() => {
    const buf = window.term.buffer.active;
    const rows = [];
    for (let i = 0; i < buf.length; i++) {
      const line = buf.getLine(i);
      rows.push({ text: line?.translateToString(true) ?? '', wrapped: !!line?.isWrapped });
    }
    // Rejoin wrapped continuations so a diagnostic reads as one string.
    const out = [];
    for (const row of rows) {
      if (row.wrapped && out.length) out[out.length - 1] += row.text;
      else out.push(row.text);
    }
    return out.map((l) => l.replace(/\s+$/, '')).filter((l) => l.length);
  });

const ERROR_SHAPE =
  /is not a Natural statement|has not been declared|must come before|there is no |never closed|must be the first|does not fit|is already declared|only works|Natural needs a space|is 1 to 8 |reserved word|expects \d+ parameter|was stopped|not a field of|is not a view|is not a valid value|no such statement|cannot put its result|does not belong here|has no END statement|is not valid: that field/i;

const lessons = await page.evaluate(() =>
  import('./lessons.js').then((m) =>
    m.LESSONS.map((l, i) => ({
      index: i,
      title: l.title,
      steps: l.steps.map((s, j) => ({
        index: j,
        title: s.title,
        code: s.code ?? null,
        expectError: !!s.expectError,
        hasExercise: !!s.exercise,
      })),
    })),
  ),
);

let ran = 0;
let produced = 0;

for (const lesson of lessons) {
  const shown = await page.evaluate((li) => {
    const sel = document.getElementById('lessonsel');
    sel.value = String(li);
    sel.dispatchEvent(new Event('change', { bubbles: true }));
    return document.getElementById('lessontitle')?.textContent ?? '';
  }, lesson.index);
  if (shown.trim() !== lesson.title.trim()) {
    note(lesson.title, 'navigation', `selecting lesson ${lesson.index} showed "${shown}"`);
  }

  for (const step of lesson.steps) {
    if (!step.code) continue;
    const where = `${lesson.index + 1}.${step.index + 1} ${step.title}`;
    ran++;

    await page.evaluate(() => window.term.reset());
    await page.evaluate((src) => window.naturalRun(src), step.code);
    await new Promise((r) => setTimeout(r, 220));

    // A step that shows a map suspends until the operator transmits.
    for (let guard = 0; guard < 6; guard++) {
      const onScreen = await page.evaluate(() => !!window.naturalState.screen);
      if (!onScreen) break;
      await page.evaluate(() => window.naturalSubmitScreen('ENTR'));
      await new Promise((r) => setTimeout(r, 200));
    }

    const out = await readTerm();
    const text = out.join('\n');
    if (out.length) produced++;
    const errored = ERROR_SHAPE.test(text);

    if (step.expectError && !errored) {
      note(where, 'expected-error-missing', text.slice(0, 300) || '(no output)');
    } else if (!step.expectError && errored) {
      note(where, 'unexpected-error', text.slice(0, 400));
    } else if (!step.expectError && out.length === 0) {
      note(where, 'no-output', 'the step produced nothing');
    }
  }
}

// ---- the map path, which suspends on a screen rather than printing ----

await page.evaluate(() => window.term.reset());
await page.evaluate(() =>
  window.naturalRun(
    "DEFINE DATA LOCAL\n1 #NAME (A20)\n1 #DEPT (A6)\nEND-DEFINE\n" +
      "INPUT USING MAP 'EMPENTRY'\nWRITE 'entered' #NAME 'in' #DEPT\nEND",
  ),
);
await new Promise((r) => setTimeout(r, 400));

const mapUp = await page.evaluate(() => ({
  suspended: !!window.naturalState.screen,
  aidbar: document.getElementById('aidbar')?.style.display,
  oia: document.getElementById('oiaMsg')?.textContent ?? '',
}));
if (!mapUp.suspended) note('maps', 'map-suspension', 'INPUT USING MAP did not suspend on a screen');
if (mapUp.aidbar === 'none') note('maps', 'aid-bar', 'the AID key bar stayed hidden while a screen was up');

const panel = await readTerm();
if (!panel.some((l) => l.includes('EMPLOYEE MAINTENANCE'))) {
  note('maps', 'map-render', `the map title did not render. saw: ${panel.slice(0, 3).join(' | ')}`);
}

// Type the way an operator does, through the terminal's own input path. Setting
// screenValues directly would skip the modified data tag, and Read Modified would then
// correctly discard the values, which looks like a bug and is not one.
await page.evaluate(() => {
  window.term.input('GARRET');
  window.term.input('\r');
  window.term.input('SALES');
  window.naturalSubmitScreen('ENTR');
});
await new Promise((r) => setTimeout(r, 400));
const resumed = (await readTerm()).join('\n');
if (!resumed.includes('GARRET') || !resumed.includes('SALES')) {
  note('maps', 'map-resume', `transmitting did not return the typed values. tail: ${resumed.slice(-160)}`);
}

// ---- Read Modified: an untouched field is not transmitted ----

await page.evaluate(() => window.term.reset());
await page.evaluate(() =>
  window.naturalRun(
    "DEFINE DATA LOCAL\n1 #USER (A10)\n1 #PIN (A4)\n1 #AMOUNT (N7)\nEND-DEFINE\n" +
      "MOVE 500 TO #AMOUNT\nINPUT USING MAP 'SIGNON'\nWRITE 'amount' #AMOUNT\nEND",
  ),
);
await new Promise((r) => setTimeout(r, 300));
// Transmit without typing anything at all.
await page.evaluate(() => window.naturalSubmitScreen('ENTR'));
await new Promise((r) => setTimeout(r, 300));
const untouched = (await readTerm()).join('\n');
if (/is not valid|not a valid value/i.test(untouched)) {
  note('maps', 'read-modified', `an untouched field was transmitted and rejected: ${untouched.slice(-200)}`);
}
if (!untouched.includes('500')) {
  note('maps', 'read-modified', `an untouched numeric field lost its value. saw: ${untouched.slice(-160)}`);
}

// ---- an unsensitized PF key must arrive as ENTR ----

await page.evaluate(() => window.term.reset());
await page.evaluate(() =>
  window.naturalRun(
    "DEFINE DATA LOCAL\n1 #NAME (A20)\nEND-DEFINE\nINPUT USING MAP 'CONFIRM'\n" +
      "IF *PF-KEY = 'PF3'\nWRITE 'cancelled'\nELSE\nWRITE 'confirmed'\nEND-IF\nEND",
  ),
);
await new Promise((r) => setTimeout(r, 300));
await page.evaluate(() => window.naturalSubmitScreen('PF3'));
await new Promise((r) => setTimeout(r, 300));
const unsensitized = (await readTerm()).join('\n');
if (!unsensitized.includes('confirmed')) {
  note('aid keys', 'unsensitized-pf', `PF3 without SET KEY should arrive as ENTR. saw: ${unsensitized.slice(-120)}`);
}

const problems = await page.evaluate(() => window.__problems || []);
await browser.close();
stop();

const failures =
  findings.length + problems.length + pageErrors.length + consoleErrors.length + failedRequests.length;

console.log('================ END-TO-END WALKTHROUGH ================');
console.log(`lessons ${lessons.length} | code steps run ${ran} | produced output ${produced}`);
for (const f of findings) console.log(`\n[${f.kind}] ${f.where}\n    ${f.detail.replace(/\n/g, '\n    ')}`);
if (problems.length) console.log('\nin-page errors:\n  ' + problems.join('\n  '));
if (pageErrors.length) console.log('\npage errors:\n  ' + pageErrors.join('\n  '));
if (consoleErrors.length) console.log('\nconsole errors:\n  ' + [...new Set(consoleErrors)].join('\n  '));
if (failedRequests.length) console.log('\nfailed requests:\n  ' + [...new Set(failedRequests)].join('\n  '));

if (failures === 0) {
  console.log('\nPASS: every step ran, nothing errored.');
  process.exit(0);
}
console.log(`\nFAIL: ${failures} problem(s).`);
process.exit(1);
