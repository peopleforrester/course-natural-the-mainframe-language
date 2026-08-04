// ABOUTME: Tier 1 lesson content, modules 1 to 9, following spec/tier1-lesson-outline.md.
// ABOUTME: Every code block runs in the browser interpreter exactly as written.

export const LESSONS = [
  {
    title: '1. What Natural is',
    lede:
      'Natural is a fourth-generation language built as the native programming language ' +
      'of the ADABAS database. It still runs payroll, benefits, and licensing systems in ' +
      'government and insurance, which is why people are still hired to maintain it.',
    steps: [
      {
        title: 'Where it came from, and where it runs',
        body:
          '<p>Software AG developed Natural from 1975, under Peter Pagé with Margit ' +
          'Neumann. It was designed around ADABAS, so reading and writing database ' +
          'records is part of the language rather than a library bolted on.</p>' +
          '<p>Today it runs on <b>IBM z/OS</b> and <b>Linux</b>, plus Windows and ' +
          'containers. Two different things happened to the older platforms you may see ' +
          'mentioned online, and the distinction is worth keeping straight. ' +
          '<b>AIX, Solaris and HP-UX</b> reached end of maintenance on 31 December 2024. ' +
          '<b>z/VSE and BS2000</b> were dropped earlier and are simply not supported from ' +
          'version 9.2 onward, with no separate end-of-maintenance date published.</p>' +
          '<div class="tip">Adabas &amp; Natural has been a standalone business under ' +
          'Software GmbH since January 2025.</div>',
      },
      {
        title: 'Run your first program',
        body:
          '<p>Press <b>Run</b> below. You are executing real Natural syntax against an ' +
          'interpreter compiled to WebAssembly, running in this browser tab.</p>',
        code: "WRITE 'Hello from the mainframe.'\nEND",
      },
      {
        title: 'What this course is, honestly',
        body:
          '<p>The terminal runs a <b>teaching interpreter over sample data</b>. It is not ' +
          'a live ADABAS instance, and it implements the subset of Natural this course ' +
          'teaches.</p>' +
          '<p>Every <b>statement</b> you write here is real Natural, checked against the ' +
          'Software AG statement reference. Where the interpreter rejects something, it ' +
          'rejects it for the reason a real compiler would.</p>' +
          '<p>Two things around the edges are deliberately ours, and you should know ' +
          'which:</p>' +
          '<ul>' +
          '<li>The sample <code class="inl">EMPLOYEES</code> file is <b>flattened</b>. In ' +
          'the real Software AG demo file, salary lives inside a repeating group and needs ' +
          'index notation. We teach the loops first and the group structure later.</li>' +
          '<li>The <b>source format for maps</b> is this course\'s own. A real map is drawn ' +
          'in a screen editor and has no hand-written text form, so there is nothing ' +
          'authentic to copy. The program-side statements that <i>use</i> a map are real.</li>' +
          '</ul>' +
          '<div class="tip">Anywhere the course departs from production Natural, it says ' +
          'so on the page where it happens. You should never find out from a compiler.</div>',
      },
    ],
  },

  {
    title: '2. Your first program',
    lede: 'Every Natural program ends with END. WRITE puts a line on the screen.',
    steps: [
      {
        title: 'WRITE and END',
        body:
          '<p><code class="inl">WRITE</code> outputs a line. ' +
          '<code class="inl">END</code> terminates the program, and every program needs ' +
          'one.</p>',
        code: "WRITE 'First line'\nWRITE 'Second line'\nEND",
      },
      {
        title: 'Quotes inside text',
        body:
          '<p>Write a quote inside a text literal by doubling it. This is the documented ' +
          'Natural convention.</p>',
        code: "WRITE 'It''s a mainframe.'\nEND",
      },
      {
        title: 'Leave the END off and see what happens',
        body:
          '<p>Errors in this course name the Natural concept you missed, not a parser ' +
          'internal. Run this to see one.</p>',
        code: "WRITE 'I forgot something'",
      },
      {
        title: 'Your turn: write a program',
        body: '<p>Time to write one yourself rather than running mine.</p>',
        exercise: {
          task:
            'Write a program that outputs exactly three lines: ' +
            '<code class="inl">ONE</code>, <code class="inl">TWO</code>, ' +
            'then <code class="inl">THREE</code>. Remember the END.',
          starter: "WRITE 'ONE'\n",
          check: (r) => {
            if (r.errored) return { pass: false, message: r.errored };
            const got = r.lines.map((l) => l.trim());
            if (got.length !== 3)
              return { pass: false, message: 'Expected three lines, got ' + got.length + '.' };
            const want = ['ONE', 'TWO', 'THREE'];
            for (let i = 0; i < 3; i++)
              if (got[i] !== want[i])
                return { pass: false, message: 'Line ' + (i + 1) + ' should be ' + want[i] + ', got "' + got[i] + '".' };
            return { pass: true, message: 'Three lines, in order. That is a complete Natural program.' };
          },
        },
      },
    ],
  },

  {
    title: '3. Data and DEFINE DATA',
    lede:
      'Natural makes you declare your fields up front, with a format and a length. This ' +
      'is the conceptual spine of the language.',
    steps: [
      {
        title: 'The DEFINE DATA block',
        body:
          '<p>Declarations come first, before any executable statement. Each field has a ' +
          'level number, a name, and a format in parentheses.</p>' +
          '<p><b>A</b> is alphanumeric, <b>N</b> is numeric, <b>P</b> is packed numeric, ' +
          '<b>I</b> is a binary integer, and <b>L</b> is logical.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #NAME (A20)\n" +
          "1 #AGE (N3)\n" +
          "END-DEFINE\n" +
          "MOVE 'GRACE HOPPER' TO #NAME\n" +
          "MOVE 79 TO #AGE\n" +
          "WRITE #NAME #AGE\n" +
          "END",
      },
      {
        title: 'Reading the length notation correctly',
        body:
          '<p>This one trips up almost everyone. <code class="inl">(N7.2)</code> means ' +
          '<b>seven digits before</b> the decimal point and <b>two after</b>, so nine ' +
          'digit positions in total. It does not mean seven digits altogether.</p>' +
          '<div class="tip">Limits worth knowing: N and P allow at most 29 positions, ' +
          'and I accepts only lengths 1, 2, and 4.</div>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #BIG (N7.2)\n" +
          "END-DEFINE\n" +
          "MOVE 1234567.89 TO #BIG\n" +
          "WRITE 'All nine positions fit:' #BIG\n" +
          "END",
      },
      {
        title: 'Fields have a width when printed',
        body:
          '<p>A field always occupies its full print width. A numeric field also reserves ' +
          'one leading position for a sign, even when the value is positive, which is why ' +
          'numbers appear further right than you might expect.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #A (A10)\n" +
          "1 #N (N5)\n" +
          "END-DEFINE\n" +
          "MOVE 'LEFT' TO #A\n" +
          "MOVE 42 TO #N\n" +
          "WRITE '[' #A ']'\n" +
          "WRITE '[' #N ']'\n" +
          "END",
      },
    ],
  },

  {
    title: '4. Assignment and computation',
    lede:
      'Natural is a business language, so its arithmetic is exact decimal arithmetic. ' +
      'This is the reason it is still trusted with money.',
    steps: [
      {
        title: 'MOVE, the assignment operator, and COMPUTE',
        body:
          '<p><code class="inl">MOVE x TO y</code> and <code class="inl">y := x</code> ' +
          'are the same thing. <code class="inl">COMPUTE</code> evaluates an expression. ' +
          'Put spaces around operators.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #PRICE (N7.2)\n" +
          "1 #QTY (I4)\n" +
          "1 #TOTAL (N9.2)\n" +
          "END-DEFINE\n" +
          "MOVE 19.99 TO #PRICE\n" +
          "#QTY := 3\n" +
          "COMPUTE #TOTAL = #PRICE * #QTY\n" +
          "WRITE 'Total:' #TOTAL\n" +
          "END",
      },
      {
        title: 'Exact decimals, not floating point',
        body:
          '<p>In most languages <code class="inl">0.1 + 0.2</code> is ' +
          '0.30000000000000004. Here it is exactly 0.30, because Natural stores decimal ' +
          'digits rather than binary fractions.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #R (N5.2)\n" +
          "END-DEFINE\n" +
          "COMPUTE #R = 0.1 + 0.2\n" +
          "WRITE 'Exactly:' #R\n" +
          "END",
      },
      {
        title: 'Truncation is the default; ROUNDED opts in',
        body:
          '<p>Assigning a value with more decimals than the field holds ' +
          '<b>truncates</b> toward zero. Add <code class="inl">ROUNDED</code> when you ' +
          'want rounding instead.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #CUT (N3.1)\n" +
          "1 #ROUND (N3.1)\n" +
          "END-DEFINE\n" +
          "COMPUTE #CUT = 1.29\n" +
          "COMPUTE ROUNDED #ROUND = 1.29\n" +
          "WRITE 'Truncated:' #CUT\n" +
          "WRITE 'Rounded:  ' #ROUND\n" +
          "END",
      },
      {
        title: 'The arithmetic verbs',
        body:
          '<p>ADD, SUBTRACT, MULTIPLY, and DIVIDE each read and write one field. Watch ' +
          'the direction of DIVIDE: <code class="inl">DIVIDE 4 INTO #N</code> divides ' +
          '<b>the field</b> by four.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #N (N7.2)\n" +
          "END-DEFINE\n" +
          "MOVE 100 TO #N\n" +
          "ADD 50 TO #N\n" +
          "WRITE 'After ADD:     ' #N\n" +
          "SUBTRACT 25 FROM #N\n" +
          "WRITE 'After SUBTRACT:' #N\n" +
          "DIVIDE 5 INTO #N\n" +
          "WRITE 'After DIVIDE:  ' #N\n" +
          "END",
      },
      {
        title: 'Your turn: work out a total',
        body: '<p>Declare what you need, calculate, and print the answer.</p>',
        exercise: {
          task:
            'A part costs <b>12.50</b> and you are buying <b>7</b>. Compute the total ' +
            'into a field called <code class="inl">#TOTAL</code> declared as ' +
            '<code class="inl">(N7.2)</code>, then WRITE it. The answer should be 87.50.',
          starter:
            "DEFINE DATA LOCAL\n1 #PRICE (N7.2)\n1 #QTY (I4)\n1 #TOTAL (N7.2)\n" +
            "END-DEFINE\n\nEND",
          check: (r) => {
            if (r.errored) return { pass: false, message: r.errored };
            const total = r.field('#TOTAL');
            if (total === undefined || total === null)
              return { pass: false, message: 'No field called #TOTAL was declared.' };
            if (total.trim() !== '87.50')
              return { pass: false, message: '#TOTAL holds ' + total.trim() + ', expected 87.50.' };
            if (!r.lines.length)
              return { pass: false, message: '#TOTAL is right, but nothing was written out. Add a WRITE.' };
            return { pass: true, message: 'Exactly 87.50, and printed. Decimal arithmetic done properly.' };
          },
        },
      },
    ],
  },

  {
    title: '5. Input and decisions',
    lede:
      'Programs get more useful when they read a value and branch on it. INPUT pauses ' +
      'the program and waits for you.',
    steps: [
      {
        title: 'INPUT reads a value',
        body:
          '<p>Run this, then <b>type a name into the terminal and press Enter</b>. The ' +
          'program is genuinely suspended while it waits, and resumes where it left ' +
          'off.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #NAME (A20)\n" +
          "END-DEFINE\n" +
          "INPUT 'What is your name?' #NAME\n" +
          "WRITE 'Hello,' #NAME\n" +
          "END",
      },
      {
        title: 'IF, ELSE, END-IF',
        body:
          '<p>Comparisons accept symbols or their mnemonic forms: ' +
          '<code class="inl">=</code> or <code class="inl">EQ</code>, ' +
          '<code class="inl">&gt;</code> or <code class="inl">GT</code>, and so on. Type ' +
          'a number when it asks.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #AGE (N3)\n" +
          "END-DEFINE\n" +
          "INPUT 'How many years of service?' #AGE\n" +
          "IF #AGE >= 25\n" +
          "WRITE 'Eligible for the long service award.'\n" +
          "ELSE\n" +
          "WRITE 'Not yet eligible.'\n" +
          "END-IF\n" +
          "END",
      },
      {
        title: 'DECIDE FOR: a ladder of conditions',
        body:
          '<p><code class="inl">DECIDE FOR FIRST CONDITION</code> takes the first branch ' +
          'whose condition is true. It reads far better than nested IFs, and you will ' +
          'meet it constantly in real maintenance code.</p>' +
          '<p><code class="inl">WHEN NONE</code> is <b>required</b>, not a nicety. Natural ' +
          'will not compile a DECIDE without it. The optional clauses in the syntax are ' +
          'printed in square brackets; NONE is not one of them.</p>' +
          '<div class="tip">If a branch genuinely should do nothing, write ' +
          '<code class="inl">IGNORE</code>. Natural has no empty statement, so leaving the ' +
          'clause blank is an error. Being made to write IGNORE is the point: it proves you ' +
          'considered the case rather than forgot it.</div>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #SCORE (N3)\n" +
          "1 #GRADE (A1)\n" +
          "END-DEFINE\n" +
          "MOVE 87 TO #SCORE\n" +
          "DECIDE FOR FIRST CONDITION\n" +
          "WHEN #SCORE >= 90\n" +
          "MOVE 'A' TO #GRADE\n" +
          "WHEN #SCORE >= 80\n" +
          "MOVE 'B' TO #GRADE\n" +
          "WHEN NONE\n" +
          "MOVE 'F' TO #GRADE\n" +
          "END-DECIDE\n" +
          "WRITE 'Score' #SCORE 'is grade' #GRADE\n" +
          "END",
      },
      {
        title: 'DECIDE ON: branching on one value',
        body:
          '<p>When you are testing one field against a list of values, ' +
          '<code class="inl">DECIDE ON</code> is the clearer form. A clause can list ' +
          'several values.</p>' +
          '<p>The catch-all is spelled <code class="inl">NONE VALUE</code> here rather than ' +
          '<code class="inl">WHEN NONE</code>, and it is required in this form too.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #DAY (N1)\n" +
          "END-DEFINE\n" +
          "MOVE 6 TO #DAY\n" +
          "DECIDE ON FIRST VALUE OF #DAY\n" +
          "VALUE 1, 2, 3, 4, 5\n" +
          "WRITE 'A working day.'\n" +
          "VALUE 6, 7\n" +
          "WRITE 'The weekend.'\n" +
          "NONE VALUE\n" +
          "WRITE 'Not a day of the week.'\n" +
          "END-DECIDE\n" +
          "END",
      },
    ],
  },

  {
    title: '6. WRITE and DISPLAY',
    lede:
      'Natural has two output statements and they behave differently. Knowing which to ' +
      'reach for is the whole of this module.',
    steps: [
      {
        title: 'WRITE is free-format',
        body:
          '<p><code class="inl">WRITE</code> puts elements on a line separated by one ' +
          'blank, and never produces headers.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #CITY (A20)\n" +
          "1 #POP (N8)\n" +
          "END-DEFINE\n" +
          "MOVE 'DERBY' TO #CITY\n" +
          "MOVE 261000 TO #POP\n" +
          "WRITE 'City:' #CITY 'Population:' #POP\n" +
          "END",
      },
      {
        title: 'DISPLAY builds a report',
        body:
          '<p><code class="inl">DISPLAY</code> is column-oriented. It generates a header ' +
          'from each field name, underlines it, leaves one blank line, and then prints ' +
          'rows. Alphanumeric values sit left in their column, numerics sit right.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #I (I4)\n" +
          "1 #PRODUCT (A12)\n" +
          "1 #QTY (N4)\n" +
          "END-DEFINE\n" +
          "MOVE 'WIDGET' TO #PRODUCT\n" +
          "FOR #I = 1 TO 3\n" +
          "COMPUTE #QTY = #I * 10\n" +
          "DISPLAY #PRODUCT #QTY\n" +
          "END-FOR\n" +
          "END",
      },
      {
        title: 'The headers appear once',
        body:
          '<p>Notice that the DISPLAY above sat inside a loop and still produced a single ' +
          'header block. That is what makes it the reporting statement: one header, then ' +
          'a row per record.</p>',
      },
    ],
  },

  {
    title: '7. Loops',
    lede:
      'FOR counts. REPEAT keeps going until something stops it. ESCAPE is how you leave ' +
      'a loop early.',
    steps: [
      {
        title: 'FOR counts through a range',
        body: '<p>The control field holds the current value on each pass.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #I (I4)\n" +
          "END-DEFINE\n" +
          "FOR #I = 1 TO 5\n" +
          "WRITE 'Pass number' #I\n" +
          "END-FOR\n" +
          "END",
      },
      {
        title: 'REPEAT and ESCAPE BOTTOM',
        body:
          '<p><code class="inl">REPEAT</code> loops forever until something ends it. ' +
          '<code class="inl">ESCAPE BOTTOM</code> leaves the loop; ' +
          '<code class="inl">ESCAPE TOP</code> starts the next pass.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #N (N5)\n" +
          "END-DEFINE\n" +
          "REPEAT\n" +
          "ADD 1 TO #N\n" +
          "WRITE 'Count is' #N\n" +
          "IF #N >= 4\n" +
          "ESCAPE BOTTOM\n" +
          "END-IF\n" +
          "END-REPEAT\n" +
          "WRITE 'Out of the loop.'\n" +
          "END",
      },
      {
        title: 'A loop with no way out',
        body:
          '<p>Run this deliberately. A REPEAT with nothing to stop it would hang a real ' +
          'session; here it stops itself and tells you how to fix it.</p>' +
          '<div class="warn">This is the single most common beginner mistake with ' +
          'REPEAT. Give every REPEAT either an ESCAPE or an UNTIL condition.</div>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #N (N9)\n" +
          "END-DEFINE\n" +
          "REPEAT\n" +
          "ADD 1 TO #N\n" +
          "END-REPEAT\n" +
          "END",
      },
      {
        title: 'REPEAT UNTIL',
        body:
          '<p>The tidier form when you know the stopping condition up front.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #N (N5)\n" +
          "END-DEFINE\n" +
          "REPEAT UNTIL #N >= 3\n" +
          "ADD 1 TO #N\n" +
          "WRITE 'N is now' #N\n" +
          "END-REPEAT\n" +
          "END",
      },
      {
        title: 'Your turn: count with a loop',
        body: '<p>Use a loop rather than five WRITE statements.</p>',
        exercise: {
          task:
            'Write a FOR loop that outputs the numbers <b>1 to 5</b>, one per line. ' +
            'Declare the control field as <code class="inl">(I4)</code>.',
          starter: "DEFINE DATA LOCAL\n1 #I (I4)\nEND-DEFINE\n\nEND",
          check: (r) => {
            if (r.errored) return { pass: false, message: r.errored };
            const nums = r.lines.map((l) => l.trim()).filter(Boolean);
            if (nums.length !== 5)
              return { pass: false, message: 'Expected five lines, got ' + nums.length + '.' };
            for (let i = 0; i < 5; i++)
              if (nums[i] !== String(i + 1))
                return { pass: false, message: 'Line ' + (i + 1) + ' should be ' + (i + 1) + ', got "' + nums[i] + '".' };
            return { pass: true, message: 'Five lines from one loop. That is the point of FOR.' };
          },
        },
      },
    ],
  },

  {
    title: '8. Reading the database',
    lede:
      'This is what Natural exists for. A view is your window onto a file, and READ and ' +
      'FIND are loops over records.',
    steps: [
      {
        title: 'A database loop IS a loop',
        body:
          '<p>Before anything else: Natural calls READ and FIND <b>database loops</b>, as ' +
          'opposed to the non-database loops (FOR, REPEAT) you met in module 7. They are ' +
          'the same idea applied to records. Everything you learned about ESCAPE still ' +
          'applies.</p>',
      },
      {
        title: 'Declare a view, then READ it',
        body:
          '<p>A <code class="inl">VIEW OF</code> names the file and lists the fields you ' +
          'want. Those fields take their format from the file definition, so you do not ' +
          'declare formats for them.</p>' +
          '<p>The sample file holds eight employees.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 NAME\n" +
          "2 FIRST-NAME\n" +
          "2 CITY\n" +
          "END-DEFINE\n" +
          "READ EMPLOYEES-VIEW BY NAME\n" +
          "DISPLAY NAME FIRST-NAME CITY\n" +
          "END-READ\n" +
          "END",
      },
      {
        title: 'FIND searches',
        body:
          '<p><code class="inl">WITH</code> is the search the database performs. ' +
          '<code class="inl">WHERE</code> filters further, record by record, after they ' +
          'come back.</p>' +
          '<div class="tip"><code class="inl">*NUMBER</code> reports how many records the ' +
          'WITH search found, before WHERE narrowed them. That distinction tells you ' +
          'where the work happened.</div>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 NAME\n" +
          "2 COUNTRY\n" +
          "2 SALARY\n" +
          "END-DEFINE\n" +
          "FIND EMPLOYEES-VIEW WITH COUNTRY = 'USA' WHERE SALARY > 45000 SORTED BY NAME\n" +
          "DISPLAY *COUNTER NAME SALARY\n" +
          "END-FIND\n" +
          "WRITE 'The search matched' *NUMBER 'records before the WHERE filter.'\n" +
          "END",
      },
      {
        title: 'When nothing matches',
        body:
          '<p><code class="inl">IF NO RECORDS FOUND</code> is a clause of the FIND itself, ' +
          'and runs instead of the loop.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 NAME\n" +
          "2 CITY\n" +
          "END-DEFINE\n" +
          "FIND EMPLOYEES-VIEW WITH CITY = 'ATLANTIS'\n" +
          "IF NO RECORDS FOUND\n" +
          "WRITE 'Nobody works in ATLANTIS.'\n" +
          "END-NOREC\n" +
          "WRITE NAME\n" +
          "END-FIND\n" +
          "END",
      },
      {
        title: 'HISTOGRAM counts values',
        body:
          '<p>A histogram walks the distinct values of one field and tells you how many ' +
          'records carry each one, without reading the records themselves.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 COUNTRY\n" +
          "END-DEFINE\n" +
          "HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY\n" +
          "DISPLAY COUNTRY *NUMBER\n" +
          "END-HISTOGRAM\n" +
          "END",
      },
      {
        title: 'Your turn: query the file',
        body: '<p>Now use the database for real.</p>',
        exercise: {
          task:
            'Find every employee in <b>UK</b> and write each one\'s NAME. ' +
            'There are two of them, and they should come out in name order.',
          starter:
            "DEFINE DATA LOCAL\n1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n2 NAME\n2 COUNTRY\n" +
            "END-DEFINE\n\nEND",
          check: (r) => {
            if (r.errored) return { pass: false, message: r.errored };
            const names = r.lines.map((l) => l.trim()).filter(Boolean);
            if (names.length !== 2)
              return { pass: false, message: 'Expected two employees, got ' + names.length + '.' };
            if (names[0] !== 'GARRET' || names[1] !== 'JONES')
              return { pass: false, message: 'Expected GARRET then JONES, got ' + names.join(', ') + '.' };
            return { pass: true, message: 'Both UK employees, in order. You are reading a database in Natural.' };
          },
        },
      },
    ],
  },

  {
    title: '9. Changing the database',
    lede:
      'STORE adds, UPDATE changes, DELETE removes. None of them count until you commit ' +
      'with END TRANSACTION.',
    steps: [
      {
        title: 'STORE adds a record',
        body:
          '<p>Move the values you want into the view fields, then STORE. Note the ' +
          '<code class="inl">END TRANSACTION</code> at the bottom: that is the commit.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 PERSONNEL-ID\n" +
          "2 NAME\n" +
          "2 FIRST-NAME\n" +
          "2 CITY\n" +
          "2 COUNTRY\n" +
          "2 SALARY\n" +
          "END-DEFINE\n" +
          "MOVE '99999999' TO PERSONNEL-ID\n" +
          "MOVE 'TURING' TO NAME\n" +
          "MOVE 'ALAN' TO FIRST-NAME\n" +
          "MOVE 'LONDON' TO CITY\n" +
          "MOVE 'UK' TO COUNTRY\n" +
          "MOVE 71000 TO SALARY\n" +
          "STORE EMPLOYEES-VIEW\n" +
          "END TRANSACTION\n" +
          "READ EMPLOYEES-VIEW BY NAME\n" +
          "DISPLAY NAME CITY SALARY\n" +
          "END-READ\n" +
          "END",
      },
      {
        title: 'UPDATE changes the record you are on',
        body:
          '<p>Inside a READ or FIND loop, UPDATE writes the view fields back to the ' +
          'record the loop is holding.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 NAME\n" +
          "2 SALARY\n" +
          "END-DEFINE\n" +
          "FIND EMPLOYEES-VIEW WITH NAME = 'GARRET'\n" +
          "WRITE 'Before:' SALARY\n" +
          "COMPUTE SALARY = SALARY * 1.10\n" +
          "UPDATE\n" +
          "END-FIND\n" +
          "END TRANSACTION\n" +
          "FIND EMPLOYEES-VIEW WITH NAME = 'GARRET'\n" +
          "WRITE 'After: ' SALARY\n" +
          "END-FIND\n" +
          "END",
      },
      {
        title: 'Forget END TRANSACTION and lose the work',
        body:
          '<p>This is the classic beginner bug, and it is worth making on purpose. The ' +
          'update below happens, and the program can even see it, but it is never ' +
          'committed. The second FIND runs after a fresh start and shows the original ' +
          'value.</p>' +
          '<div class="warn">On a real system an uncommitted transaction is backed out ' +
          'when the program ends. The work is simply gone.</div>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 NAME\n" +
          "2 SALARY\n" +
          "END-DEFINE\n" +
          "FIND EMPLOYEES-VIEW WITH NAME = 'GARRET'\n" +
          "COMPUTE SALARY = 999999\n" +
          "UPDATE\n" +
          "WRITE 'In this run the field says:' SALARY\n" +
          "END-FIND\n" +
          "WRITE 'But nothing was committed, so nothing persists.'\n" +
          "END",
      },
      {
        title: 'DELETE and BACKOUT',
        body:
          '<p>DELETE removes the record the loop is holding. ' +
          '<code class="inl">BACKOUT TRANSACTION</code> throws away everything since the ' +
          'last commit, which is how you undo deliberately.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 NAME\n" +
          "END-DEFINE\n" +
          "FIND EMPLOYEES-VIEW WITH NAME = 'JONES'\n" +
          "DELETE\n" +
          "END-FIND\n" +
          "WRITE 'Deleted, but not committed. Now backing out:'\n" +
          "BACKOUT TRANSACTION\n" +
          "READ EMPLOYEES-VIEW BY NAME\n" +
          "WRITE NAME\n" +
          "END-READ\n" +
          "END",
      },
      {
        title: 'Capstone: put it all together',
        body:
          '<p>Everything from modules 1 through 9 in one program: a histogram summary, a ' +
          'read loop with grading, a report, a rounded calculation, a filtered update, ' +
          'and a commit you can verify.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 NAME\n" +
          "2 COUNTRY\n" +
          "2 SALARY\n" +
          "1 #TOTAL (P11)\n" +
          "1 #RAISE (N9.2)\n" +
          "1 #GRADE (A8)\n" +
          "END-DEFINE\n" +
          "WRITE 'STAFF REVIEW'\n" +
          "HISTOGRAM EMPLOYEES-VIEW FOR COUNTRY\n" +
          "WRITE COUNTRY 'has' *NUMBER 'staff'\n" +
          "END-HISTOGRAM\n" +
          "READ EMPLOYEES-VIEW BY NAME\n" +
          "ADD SALARY TO #TOTAL\n" +
          "DECIDE FOR FIRST CONDITION\n" +
          "WHEN SALARY >= 60000\n" +
          "MOVE 'SENIOR' TO #GRADE\n" +
          "WHEN SALARY >= 40000\n" +
          "MOVE 'MID' TO #GRADE\n" +
          "WHEN NONE\n" +
          "MOVE 'JUNIOR' TO #GRADE\n" +
          "END-DECIDE\n" +
          "DISPLAY NAME COUNTRY SALARY #GRADE\n" +
          "END-READ\n" +
          "WRITE 'Total payroll:' #TOTAL\n" +
          "COMPUTE ROUNDED #RAISE = #TOTAL * 0.035\n" +
          "WRITE 'Raise budget: ' #RAISE\n" +
          "END",
      },
    ],
  },

  {
    title: '10. Validating what the operator typed',
    lede:
      'A program that reads input has to cope with input it does not want. REINPUT sends ' +
      'the operator back to the screen with a message saying why.',
    steps: [
      {
        title: 'REINPUT re-asks',
        body:
          '<p>Run this and type a number under 18 the first time. The program rejects it, ' +
          'says why, and asks again. Type something 18 or over to get through.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #AGE (N3)\n" +
          "END-DEFINE\n" +
          "INPUT 'Age?' #AGE\n" +
          "IF #AGE < 18\n" +
          "REINPUT 'You must be at least 18. Try again.'\n" +
          "END-IF\n" +
          "WRITE 'Accepted:' #AGE\n" +
          "END",
      },
      {
        title: 'Validation is a loop you did not have to write',
        body:
          '<p>Notice there is no REPEAT here. REINPUT itself sends control back to the ' +
          'INPUT, so the validation loop is built into the statement. That is why Natural ' +
          'programs that read screens tend to be shorter than you would expect.</p>' +
          '<div class="tip">A REINPUT with no INPUT above it is an error, because there ' +
          'is nothing to go back to.</div>',
      },
    ],
  },

  {
    title: '11. Subroutines',
    lede:
      'A subroutine is a named block of statements in the same program. PERFORM runs it ' +
      'and execution comes back to where it left off.',
    steps: [
      {
        title: 'DEFINE SUBROUTINE and PERFORM',
        body:
          '<p>The definition is skipped during normal flow; only PERFORM enters it.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #N (N5)\n" +
          "END-DEFINE\n" +
          "WRITE 'starting'\n" +
          "PERFORM SAY-HELLO\n" +
          "WRITE 'finished'\n" +
          "DEFINE SUBROUTINE SAY-HELLO\n" +
          "WRITE 'hello from the subroutine'\n" +
          "END-SUBROUTINE\n" +
          "END",
      },
      {
        title: 'An inline subroutine shares the program data',
        body:
          '<p>This is the key property, and the one that separates a subroutine from the ' +
          'subprograms in module 13. The subroutine reads and writes the same fields the ' +
          'main program declared.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 NAME\n" +
          "2 SALARY\n" +
          "1 #TOTAL (P11)\n" +
          "1 #COUNT (I4)\n" +
          "1 #AVERAGE (N9.2)\n" +
          "END-DEFINE\n" +
          "PERFORM GATHER\n" +
          "PERFORM REPORT\n" +
          "DEFINE SUBROUTINE GATHER\n" +
          "READ EMPLOYEES-VIEW\n" +
          "ADD SALARY TO #TOTAL\n" +
          "ADD 1 TO #COUNT\n" +
          "END-READ\n" +
          "END-SUBROUTINE\n" +
          "DEFINE SUBROUTINE REPORT\n" +
          "COMPUTE ROUNDED #AVERAGE = #TOTAL / #COUNT\n" +
          "WRITE 'Employees:' #COUNT\n" +
          "WRITE 'Average:  ' #AVERAGE\n" +
          "END-SUBROUTINE\n" +
          "END",
      },
      {
        title: 'Subroutines can call subroutines',
        body:
          '<p>Nesting works, and each PERFORM returns to its own caller. Try to write a ' +
          'subroutine that performs itself and the interpreter will stop you rather than ' +
          'letting the program run out of stack.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #N (N5)\n" +
          "END-DEFINE\n" +
          "PERFORM OUTER\n" +
          "DEFINE SUBROUTINE OUTER\n" +
          "WRITE 'outer starts'\n" +
          "PERFORM INNER\n" +
          "WRITE 'outer resumes'\n" +
          "END-SUBROUTINE\n" +
          "DEFINE SUBROUTINE INNER\n" +
          "WRITE '  inner ran'\n" +
          "END-SUBROUTINE\n" +
          "END",
      },
      {
        title: 'Your turn: factor the work out',
        body: '<p>Take working code and give part of it a name.</p>',
        exercise: {
          task:
            'Write a subroutine called <code class="inl">TOTAL-PAY</code> that reads every ' +
            'employee and accumulates SALARY into <code class="inl">#TOTAL</code>. ' +
            'PERFORM it, then WRITE the total. It should come to 322100.',
          starter:
            "DEFINE DATA LOCAL\n1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n2 SALARY\n" +
            "1 #TOTAL (P11)\nEND-DEFINE\n\nEND",
          check: (r) => {
            if (r.errored) return { pass: false, message: r.errored };
            const total = r.field('#TOTAL');
            if (!total || total.trim() !== '322100')
              return { pass: false, message: '#TOTAL holds ' + (total || 'nothing').trim() + ', expected 322100.' };
            if (!r.text.includes('322100'))
              return { pass: false, message: 'The total is right but was never written out.' };
            return { pass: true, message: 'Correct, and the work lives in a named routine you could reuse.' };
          },
        },
      },
    ],
  },

  {
    title: '12. Data areas',
    lede:
      'Where a field lives decides who can see it. A parameter data area is the interface ' +
      'between two objects.',
    steps: [
      {
        title: 'LOCAL data belongs to one object',
        body:
          '<p>Everything you have declared so far has been ' +
          '<code class="inl">DEFINE DATA LOCAL</code>: fields belonging to this program ' +
          'alone. A subroutine in the same program shares them, because it is part of the ' +
          'same object.</p>',
      },
      {
        title: 'PARAMETER data is the call interface',
        body:
          '<p>A subprogram declares a ' +
          '<code class="inl">DEFINE DATA PARAMETER</code> block. Those fields, in that ' +
          'order, are what a caller passes. Nothing else crosses between them.</p>' +
          '<p>This course ships a small library of subprograms you can call. ' +
          '<code class="inl">DOUBLE-IT</code> takes a number and returns twice it; ' +
          '<code class="inl">COUNT-STAFF</code> takes a country code and returns how many ' +
          'employees are there.</p>' +
          '<div class="tip">A subprogram cannot see the caller\'s other fields, even ones ' +
          'with the same name. That isolation is the reason subprograms are safe to reuse.' +
          '</div>',
      },
      {
        title: 'See the isolation for yourself',
        body:
          '<p>This program has its own <code class="inl">#IN</code>, and so does the ' +
          '<code class="inl">DOUBLE-IT</code> subprogram it calls. They are different ' +
          'fields that happen to share a name.</p>' +
          '<p>Run it. The subprogram doubles what was passed to it, and the caller\'s ' +
          '<code class="inl">#IN</code> comes back untouched at 999.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #VALUE (N5)\n" +
          "1 #RESULT (N7)\n" +
          "1 #IN (N5)\n" +
          "END-DEFINE\n" +
          "MOVE 5 TO #VALUE\n" +
          "MOVE 999 TO #IN\n" +
          "CALLNAT 'DOUBLE-IT' #VALUE #RESULT\n" +
          "WRITE 'The subprogram returned:' #RESULT\n" +
          "WRITE 'My own #IN is still: ' #IN\n" +
          "END",
      },
    ],
  },

  {
    title: '13. Subprograms and CALLNAT',
    lede:
      'A subprogram is a separate object with its own data. CALLNAT runs it and passes ' +
      'values through its parameter list.',
    steps: [
      {
        title: 'CALLNAT passes values in and results back',
        body:
          '<p>DOUBLE-IT is a subprogram in this course library. Its parameter block is ' +
          '<code class="inl">1 #IN (N5)</code> then <code class="inl">1 #OUT (N7)</code>, ' +
          'so the call passes two arguments in that order.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #VALUE (N5)\n" +
          "1 #RESULT (N7)\n" +
          "END-DEFINE\n" +
          "MOVE 21 TO #VALUE\n" +
          "CALLNAT 'DOUBLE-IT' #VALUE #RESULT\n" +
          "WRITE 'Twice' #VALUE 'is' #RESULT\n" +
          "END",
      },
      {
        title: 'A subprogram can do real work',
        body:
          '<p>COUNT-STAFF reads the database on your behalf. You pass a country and get ' +
          'back a count, without needing to know how it searched.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #WHERE (A3)\n" +
          "1 #HOWMANY (N3)\n" +
          "END-DEFINE\n" +
          "MOVE 'USA' TO #WHERE\n" +
          "CALLNAT 'COUNT-STAFF' #WHERE #HOWMANY\n" +
          "WRITE 'Staff in' #WHERE 'is' #HOWMANY\n" +
          "CALLNAT 'COUNT-STAFF' 'UK' #HOWMANY\n" +
          "WRITE 'Staff in UK is' #HOWMANY\n" +
          "END",
      },
      {
        title: 'Get the parameter list wrong on purpose',
        body:
          '<p>The call has to match the subprogram\'s parameter block. Run this to see ' +
          'what happens when it does not.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #VALUE (N5)\n" +
          "END-DEFINE\n" +
          "CALLNAT 'DOUBLE-IT' #VALUE\n" +
          "END",
      },
      {
        title: 'Your turn: call a subprogram',
        body: '<p>Use a routine somebody else wrote, through its parameter list.</p>',
        exercise: {
          task:
            'Use <code class="inl">COUNT-STAFF</code> to find how many employees are in ' +
            '<b>ESP</b>, put the answer in <code class="inl">#HOWMANY (N3)</code>, and ' +
            'WRITE it. The answer is 1.',
          starter: "DEFINE DATA LOCAL\n1 #HOWMANY (N3)\nEND-DEFINE\n\nEND",
          check: (r) => {
            if (r.errored) return { pass: false, message: r.errored };
            const n = r.field('#HOWMANY');
            if (!n || n.trim() !== '1')
              return { pass: false, message: '#HOWMANY holds ' + (n || 'nothing').trim() + ', expected 1.' };
            if (!r.lines.length)
              return { pass: false, message: 'The count is right but nothing was written out.' };
            return { pass: true, message: 'You called a separate object and got a result back through its parameters.' };
          },
        },
      },
    ],
  },

  {
    title: '14. Maps: the green screen',
    lede:
      'A map is a screen layout. Reading one suspends the program exactly as INPUT does, ' +
      'except what is suspended is a whole panel of fields.',
    steps: [
      {
        title: 'DEFINE MAP lays out a screen',
        body:
          '<p><code class="inl">TEXT</code> places a label at a row and column. ' +
          '<code class="inl">FIELD</code> places a label followed by an entry field bound ' +
          'to one of your variables. Run this and the terminal shows the panel.</p>' +
          '<p>Type a value for each field and press Enter to move through them.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #NAME (A20)\n" +
          "1 #DEPT (A6)\n" +
          "END-DEFINE\n" +
          "DEFINE MAP EMPLOYEE-ENTRY\n" +
          "TEXT 2 25 'EMPLOYEE MAINTENANCE'\n" +
          "TEXT 4 25 '===================='\n" +
          "FIELD 7 10 'Name:' #NAME\n" +
          "FIELD 9 10 'Dept:' #DEPT\n" +
          "TEXT 22 10 'Enter to confirm'\n" +
          "END-MAP\n" +
          "INPUT USING MAP EMPLOYEE-ENTRY\n" +
          "WRITE 'You entered' #NAME 'in department' #DEPT\n" +
          "END",
      },
      {
        title: 'Attribute bytes make fields behave differently',
        body:
          '<p>Every field on a 3270 carries an attribute byte. A label is ' +
          '<b>protected</b>, so the operator cannot type into it. A numeric field accepts ' +
          'digits only. <code class="inl">(AD=I)</code> intensifies a field and ' +
          '<code class="inl">(AD=N)</code> hides what is typed into it, which is how ' +
          'password fields have always worked.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #USER (A10)\n" +
          "1 #PIN (A4)\n" +
          "1 #AMOUNT (N7)\n" +
          "END-DEFINE\n" +
          "DEFINE MAP SIGN-ON\n" +
          "TEXT 3 28 'SECURE TRANSFER'\n" +
          "FIELD 8 10 'User:  ' #USER (AD=I)\n" +
          "FIELD 10 10 'PIN:   ' #PIN (AD=N)\n" +
          "FIELD 12 10 'Amount:' #AMOUNT\n" +
          "END-MAP\n" +
          "INPUT USING MAP SIGN-ON\n" +
          "WRITE 'User' #USER 'moved' #AMOUNT\n" +
          "WRITE 'The PIN was never displayed on screen.'\n" +
          "END",
      },
      {
        title: 'PF keys tell the program what the operator wanted',
        body:
          '<p>The key that ends a screen is an <b>AID key</b>, and the program reads it ' +
          'from <code class="inl">*PF-KEY</code>. Every "PF3 to exit" convention in ' +
          'mainframe software is this one field.</p>' +
          '<div class="tip">Press <b>Enter</b> to confirm, or the <b>PF3</b> button below ' +
          'the screen to cancel, and watch which branch runs.</div>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 #NAME (A20)\n" +
          "END-DEFINE\n" +
          "DEFINE MAP CONFIRM\n" +
          "TEXT 2 25 'ADD AN EMPLOYEE'\n" +
          "FIELD 6 10 'Name:' #NAME\n" +
          "TEXT 22 10 'Enter to save, PF3 to cancel'\n" +
          "END-MAP\n" +
          "INPUT USING MAP CONFIRM\n" +
          "IF *PF-KEY = 'PF3'\n" +
          "WRITE 'Cancelled. Nothing was saved.'\n" +
          "ELSE\n" +
          "WRITE 'Would save' #NAME\n" +
          "END-IF\n" +
          "END",
      },
    ],
  },

  {
    title: '15. Capstone: a maintenance program',
    lede:
      'Everything together: a map to collect a search, a subprogram to do the counting, ' +
      'subroutines to organize the work, and a committed database change.',
    steps: [
      {
        title: 'Search and report',
        body:
          '<p>Fill in a country code (try USA, UK, ESP, F, or CZ) and press Enter.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 NAME\n" +
          "2 CITY\n" +
          "2 COUNTRY\n" +
          "2 SALARY\n" +
          "1 #WHERE (A3)\n" +
          "1 #HOWMANY (N3)\n" +
          "1 #TOTAL (P11)\n" +
          "END-DEFINE\n" +
          "DEFINE MAP SEARCH\n" +
          "TEXT 2 25 'STAFF ENQUIRY'\n" +
          "FIELD 6 10 'Country code:' #WHERE\n" +
          "TEXT 22 10 'Enter to search'\n" +
          "END-MAP\n" +
          "INPUT USING MAP SEARCH\n" +
          "CALLNAT 'COUNT-STAFF' #WHERE #HOWMANY\n" +
          "IF #HOWMANY = 0\n" +
          "WRITE 'No staff found in' #WHERE\n" +
          "ELSE\n" +
          "PERFORM LIST-THEM\n" +
          "END-IF\n" +
          "DEFINE SUBROUTINE LIST-THEM\n" +
          "FIND EMPLOYEES-VIEW WITH COUNTRY = #WHERE SORTED BY NAME\n" +
          "DISPLAY NAME CITY SALARY\n" +
          "ADD SALARY TO #TOTAL\n" +
          "END-FIND\n" +
          "WRITE 'Headcount:' #HOWMANY\n" +
          "WRITE 'Payroll:  ' #TOTAL\n" +
          "END-SUBROUTINE\n" +
          "END",
      },
      {
        title: 'A validated update, committed',
        body:
          '<p>The full shape of a maintenance program: read a screen, validate it, change ' +
          'the database, commit, and prove the change stuck.</p>',
        code:
          "DEFINE DATA LOCAL\n" +
          "1 EMPLOYEES-VIEW VIEW OF EMPLOYEES\n" +
          "2 NAME\n" +
          "2 SALARY\n" +
          "1 #WHO (A20)\n" +
          "1 #RISE (N5)\n" +
          "END-DEFINE\n" +
          "DEFINE MAP RAISE-MAP\n" +
          "TEXT 2 25 'AWARD A RAISE'\n" +
          "FIELD 6 10 'Employee:' #WHO\n" +
          "FIELD 8 10 'Amount:  ' #RISE\n" +
          "TEXT 22 10 'Try GARRET and 3000'\n" +
          "END-MAP\n" +
          "INPUT USING MAP RAISE-MAP\n" +
          "IF #RISE = 0\n" +
          "REINPUT 'Enter an amount greater than zero.'\n" +
          "END-IF\n" +
          "FIND EMPLOYEES-VIEW WITH NAME = #WHO\n" +
          "IF NO RECORDS FOUND\n" +
          "WRITE 'No employee called' #WHO\n" +
          "END-NOREC\n" +
          "WRITE 'Before:' SALARY\n" +
          "ADD #RISE TO SALARY\n" +
          "UPDATE\n" +
          "END-FIND\n" +
          "END TRANSACTION\n" +
          "FIND EMPLOYEES-VIEW WITH NAME = #WHO\n" +
          "WRITE 'After: ' SALARY\n" +
          "END-FIND\n" +
          "END",
      },
    ],
  },
];

/**
 * Subprograms the lessons can CALLNAT. In a real installation these live in the same
 * library as the program; here the course supplies them so a learner can call a routine
 * without also having to write it.
 */
export const LIBRARY = {
  'DOUBLE-IT': [
    'DEFINE DATA PARAMETER',
    '1 #IN (N5)',
    '1 #OUT (N7)',
    'END-DEFINE',
    'COMPUTE #OUT = #IN * 2',
    'END',
  ].join('\n'),

  'COUNT-STAFF': [
    'DEFINE DATA PARAMETER',
    '1 #COUNTRY (A3)',
    '1 #HOWMANY (N3)',
    'END-DEFINE',
    'DEFINE DATA LOCAL',
    '1 EMPLOYEES-VIEW VIEW OF EMPLOYEES',
    '2 COUNTRY',
    'END-DEFINE',
    'RESET #HOWMANY',
    'FIND EMPLOYEES-VIEW WITH COUNTRY = #COUNTRY',
    'ADD 1 TO #HOWMANY',
    'END-FIND',
    'END',
  ].join('\n'),
};
