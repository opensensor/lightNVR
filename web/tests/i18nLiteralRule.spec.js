/**
 * The i18n literal rule and the TypeScript lint settings, exercised through the
 * real eslint.config.js. ESLint runs in a child process because the config and
 * rule are ES modules that use import.meta, which Jest's CommonJS runtime cannot
 * load. Files are virtual: lintText never reads the given path.
 */
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const webRoot = path.resolve(__dirname, '..');
const RULE_ID = 'lightnvr-i18n/no-literal-ui-string';
const baseline = JSON.parse(fs.readFileSync(path.join(webRoot, 'i18n-literal-baseline.json'), 'utf8'));

const RUNNER = `
  import { ESLint } from 'eslint';
  import { baselineKey } from './eslint-rules/no-literal-ui-string.js';
  let input = '';
  for await (const chunk of process.stdin) input += chunk;
  const { cases, keys } = JSON.parse(input);
  const eslint = new ESLint({ cwd: process.cwd() });
  const results = [];
  for (const { file, code } of cases) {
    const [result] = await eslint.lintText(code, { filePath: file });
    results.push(result ? result.messages : null);
  }
  process.stdout.write(JSON.stringify({ results, keys: keys.map(baselineKey) }));
`;

function runEslint(cases, keys = []) {
  const { I18N_LITERAL_BASELINE: _ignored, ...env } = process.env;
  const child = spawnSync(process.execPath, ['--input-type=module', '-e', RUNNER], {
    cwd: webRoot,
    env,
    input: JSON.stringify({ cases, keys }),
    encoding: 'utf8',
    maxBuffer: 64 * 1024 * 1024,
  });
  if (child.status !== 0) throw new Error(child.stderr);
  return JSON.parse(child.stdout);
}

// Fails on a file ESLint skipped or could not parse, so "no errors" cannot pass
// vacuously.
function lint(file, code) {
  const [messages] = runEslint([{ file, code }]).results;
  if (messages === null) throw new Error(`${file} is not covered by the ESLint config`);
  const fatal = messages.find((message) => message.fatal);
  if (fatal) throw new Error(`${file} failed to parse: ${fatal.message}`);
  return messages;
}

const i18nReports = (messages) => messages.filter((message) => message.ruleId === RULE_ID);
const errorsFor = (messages, ruleId) => messages.filter((message) => message.ruleId === ruleId && message.severity === 2);
const component = (body) => `export const C = () => <div>${body}</div>;\n`;
const NEW_TSX = 'js/components/preact/NotInBaseline.tsx';

describe('no-literal-ui-string on TSX', () => {
  test.each([
    ['as', `<a>{('Hello' as string)}</a>`],
    ['satisfies', `<a>{('Hello' satisfies string)}</a>`],
    ['non-null', `<a>{'Hello'!}</a>`],
    ['nested wrappers', `<a>{('Hello'! as string)}</a>`],
    ['wrapped conditional branch', `<a>{(ok ? ('Hello' as string) : '') as string}</a>`],
    ['wrapped logical operand', `<a>{ok && ('Hello' as string)}</a>`],
  ])('reports a literal behind %s', (_name, body) => {
    const reports = i18nReports(lint(NEW_TSX, component(body)));
    expect(reports.map((report) => report.message)).toEqual(['Untranslated UI string: "Hello"']);
    expect(reports[0].severity).toBe(2);
  });

  test('still reports plain text and visible attributes', () => {
    const reports = i18nReports(lint(NEW_TSX, component(`<a title="Tip">Hello</a>`)));
    expect(reports.map((report) => report.message).sort()).toEqual([
      'Untranslated UI string: "Hello"',
      'Untranslated UI string: "Tip"',
    ]);
  });

  test('ignores translated calls', () => {
    expect(i18nReports(lint(NEW_TSX, component(`<a>{t('key') as string}</a>`)))).toEqual([]);
  });
});

describe('i18n baseline across the .jsx to .tsx rename', () => {
  // i18n-literal-baseline.json allows "HLS" twice in this file.
  const baselinedFile = (extension) => `js/components/preact/HLSVideoCell.${extension}`;

  test.each(['jsx', 'tsx'])('a .%s file may use its baselined count', (extension) => {
    expect(i18nReports(lint(baselinedFile(extension), component('<a>HLS</a><b>HLS</b>')))).toEqual([]);
  });

  test.each(['jsx', 'tsx'])('a .%s file reports only the occurrence past the baseline', (extension) => {
    const reports = i18nReports(lint(baselinedFile(extension), component('<a>HLS</a><b>HLS</b><i>HLS</i>')));
    expect(reports.map((report) => report.message)).toEqual(['Untranslated UI string: "HLS"']);
  });

  test('unwrapped literals draw on the same baseline count', () => {
    const body = `<a>{'HLS' as string}</a><b>{'HLS'!}</b>`;
    expect(i18nReports(lint(baselinedFile('tsx'), component(body)))).toEqual([]);
    expect(i18nReports(lint(baselinedFile('tsx'), component(`${body}<i>HLS</i>`)))).toHaveLength(1);
  });

  test('the same literal in a file without a baseline entry is reported', () => {
    expect(i18nReports(lint(NEW_TSX, component('<a>HLS</a>')))).toHaveLength(1);
  });

  test('baseline keys map .tsx to .jsx and .ts to .js', () => {
    const inputs = ['preact/A.tsx', 'preact/b.ts', 'preact/A.jsx', 'preact/b.js', 'preact/tsx.ts/c.tsx'];
    expect(runEslint([], inputs).keys).toEqual([
      'preact/A.jsx', 'preact/b.js', 'preact/A.jsx', 'preact/b.js', 'preact/tsx.ts/c.jsx',
    ]);
  });

  test('every real baselined component stays within its counts when linted as .tsx', () => {
    const componentsRoot = path.join(webRoot, 'js/components');
    const cases = Object.keys(baseline).map((key) => {
      const stem = key.replace(/\.jsx$/, '');
      const source = [`${stem}.tsx`, `${stem}.jsx`].find((name) => fs.existsSync(path.join(componentsRoot, name)));
      if (!source) throw new Error(`baseline entry ${key} has no source file`);
      return { file: `js/components/${stem}.tsx`, code: fs.readFileSync(path.join(componentsRoot, source), 'utf8') };
    });
    const { results } = runEslint(cases);
    const problems = results.flatMap((messages, index) => {
      if (messages === null) return [`${cases[index].file}: not linted`];
      return messages
        .filter((message) => message.fatal || message.ruleId === RULE_ID)
        .map((message) => `${cases[index].file}: ${message.message}`);
    });
    expect(problems).toEqual([]);
  });
});

describe('TypeScript lint settings', () => {
  const MODULE_TS = 'js/utils/lintProbe.ts';

  test.each([[MODULE_TS], ['js/pages/LintProbe.tsx']])('no-explicit-any is an error in %s', (file) => {
    const messages = lint(file, 'export const value: any = 1;\n');
    expect(errorsFor(messages, '@typescript-eslint/no-explicit-any')).toHaveLength(1);
  });

  test('ts-nocheck stays allowed until the migration closes', () => {
    const messages = lint(MODULE_TS, '// @ts-nocheck\nexport const value = 1;\n');
    expect(errorsFor(messages, '@typescript-eslint/ban-ts-comment')).toEqual([]);
  });

  test('ts-ignore is an error', () => {
    const messages = lint(MODULE_TS, '// @ts-ignore\nexport const value: number = "x";\n');
    expect(errorsFor(messages, '@typescript-eslint/ban-ts-comment')).toHaveLength(1);
  });

  test('ts-expect-error needs a reason on the same line', () => {
    const bare = lint(MODULE_TS, '// @ts-expect-error\nexport const value: number = "x";\n');
    const explained = lint(MODULE_TS, '// @ts-expect-error -- the API sends a string here\nexport const value: number = "x";\n');
    expect(errorsFor(bare, '@typescript-eslint/ban-ts-comment')).toHaveLength(1);
    expect(errorsFor(explained, '@typescript-eslint/ban-ts-comment')).toEqual([]);
  });

  test('a11y rules stay on for TSX as warnings', () => {
    const alt = lint('js/pages/LintProbe.tsx', 'export const C = () => <img src="a.png" />;\n')
      .filter((message) => message.ruleId === 'jsx-a11y/alt-text');
    expect(alt.map((message) => message.severity)).toEqual([1]);
  });

  test('TS syntax parses in .ts and .tsx, and angle-bracket casts only in .ts', () => {
    expect(lint(MODULE_TS, 'export const value = <number>(1 as unknown);\n')).toEqual([]);
    expect(lint('js/pages/LintProbe.tsx', 'export const C = <T,>(value: T): T => value;\n')).toEqual([]);
  });
});
