// ABOUTME: Extracts every runnable code sample and exercise from web/lessons.js into JSON so
// ABOUTME: the Rust test suite can compile and run the exact source a learner is shown.

import { writeFileSync, mkdirSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, '..');
const out = resolve(root, 'crates/natural-core/tests/fixtures/lesson-code.json');

const { LESSONS, LIBRARY } = await import(resolve(root, 'web/lessons.js'));

const samples = [];
LESSONS.forEach((lesson, li) => {
  lesson.steps.forEach((step, si) => {
    const where = `${li + 1}.${si + 1} ${lesson.title} / ${step.title}`;
    if (step.code) {
      // A step that teaches by failing declares it, so the harness can require the
      // failure instead of tripping over it.
      samples.push({
        where,
        kind: step.expectError ? 'demo-error' : 'demo',
        source: step.code,
      });
    }
    // An exercise starter is shown in the editor, so it must at least parse. It is
    // deliberately incomplete, which is why it is recorded with its own kind.
    if (step.exercise?.starter) {
      samples.push({ where, kind: 'starter', source: step.exercise.starter });
    }
    if (step.exercise?.solution) {
      samples.push({ where, kind: 'solution', source: step.exercise.solution });
    }
  });
});

const library = Object.entries(LIBRARY ?? {}).map(([name, source]) => ({ name, source }));

mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, `${JSON.stringify({ samples, library }, null, 2)}\n`);
process.stdout.write(
  `extracted ${samples.length} samples and ${library.length} library objects\n`,
);
