import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile, rm } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import config from '../../web/eslint.config.js';

const web = fileURLToPath(new URL('../../web/', import.meta.url));
const require = createRequire(new URL('../../web/package.json', import.meta.url));
const { ESLint } = require('eslint');
const lint = new ESLint({ cwd: web, overrideConfigFile: true, overrideConfig: config });
const literalRule = 'lightnvr-i18n/no-literal-ui-string';
const duplicateRule = 'lightnvr-source/no-duplicate-implementation';
async function messages(source, filename = 'js/components/preact/T10Probe.tsx') {
  return (await lint.lintText(source, { filePath: path.join(web, filename) }))[0].messages;
}
function forRule(result, rule) { return result.filter((message) => message.ruleId === rule); }

for (const extension of ['jsx', 'tsx']) {
  test(`${extension} retains the Footer allowance exactly once`, async () => {
    const filename = `js/components/preact/Footer.${extension}`;
    assert.equal(forRule(await messages('export const Footer = () => <footer>LightNVR ©</footer>;', filename), literalRule).length, 0);
    const result = await messages('export const Footer = () => <footer><span>LightNVR ©</span><span>LightNVR ©</span></footer>;', filename);
    assert.equal(forRule(result, literalRule).length, 1);
  });
}

for (const expression of [
  "('Untranslated greeting' as string)",
  "('Untranslated greeting' satisfies string)",
  "('Untranslated greeting' as string)!",
  "(true ? ('Untranslated greeting' as string) : t('key'))",
  "`${('Untranslated greeting' as string)}`",
  "[('Untranslated greeting' as string)]",
]) {
  test(`typed JSX child is checked: ${expression}`, async () => {
    const result = await messages(`export const View = () => <div>{${expression}}</div>;`);
    assert.equal(forRule(result, literalRule).length, 1, JSON.stringify(result));
  });
}

for (const attribute of ['title', 'aria-label', 'placeholder', 'alt']) {
  test(`typed visible attribute ${attribute} is checked`, async () => {
    const result = await messages(`export const View = () => <input ${attribute}={('Untranslated greeting' as string)} />;`);
    assert.equal(forRule(result, literalRule).length, 1);
  });
}

test('translated calls and non-visible strings remain allowed', async () => {
  const result = await messages("export const View = () => <div className={'layout-name' as string} data-url='/api/streams' title={t('label')}>{t('message')}</div>;");
  assert.equal(forRule(result, literalRule).length, 0);
});

test('nested wrapper occurrences consume the same canonical allowance', async () => {
  const result = await messages("export const Footer = () => <footer>{('LightNVR ©' as string)}{('LightNVR ©' satisfies string)}</footer>;", 'js/components/preact/Footer.tsx');
  assert.equal(forRule(result, literalRule).length, 1);
});

test('TSX hook order and accessibility violations are blocking', async () => {
  const result = await messages("import {useState} from 'preact/hooks'; export function View({enabled}: {enabled: boolean}) { if(enabled) useState(0); return <img src='/image.png'/>; }");
  assert.equal(forRule(result, 'react-hooks/rules-of-hooks')[0]?.severity, 2, JSON.stringify(result));
  assert.equal(forRule(result, 'jsx-a11y/alt-text')[0]?.severity, 2);
});

test('TSX missing effect dependencies are blocking', async () => {
  const result = await messages("import {useEffect} from 'preact/hooks'; export function View({value}: {value: string}) { useEffect(() => { console.log(value); }, []); return null; }");
  assert.equal(forRule(result, 'react-hooks/exhaustive-deps')[0]?.severity, 2, JSON.stringify(result));
});

test('legacy accessibility remains visible at warning severity', async () => {
  const result = await messages("export const View = () => <img src='/image.png'/>;", 'js/components/preact/T10Probe.jsx');
  assert.equal(forRule(result, 'jsx-a11y/alt-text')[0]?.severity, 1);
});

test('explicit any is blocked in migrated source', async () => {
  const result = await messages('export const decode = (value: any) => value;', 'js/api/T10Probe.ts');
  assert.equal(forRule(result, '@typescript-eslint/no-explicit-any')[0]?.severity, 2);
});

for (const directive of ['ts-nocheck', 'ts-ignore', 'ts-expect-error']) {
  test(`@${directive} cannot bypass migrated checks`, async () => {
    const result = await messages(`// @${directive}\nexport const number: number = 'invalid';`, 'js/api/T10Probe.ts');
    assert.equal(forRule(result, '@typescript-eslint/ban-ts-comment')[0]?.severity, 2);
  });
}

async function siblings(files, callback) {
  const root = await mkdtemp(path.join(os.tmpdir(), 'lightnvr-t10-'));
  try {
    const dir = path.join(root, 'js', 'components');
    await mkdir(dir, { recursive: true });
    for (const [name, body] of Object.entries(files)) await writeFile(path.join(dir, name), body);
    const eslint = new ESLint({ cwd: root, overrideConfigFile: true, overrideConfig: config });
    await callback((await eslint.lintFiles(Object.keys(files).map((name) => path.join(dir, name)))).flatMap((result) => result.messages));
  } finally { await rm(root, { recursive: true, force: true }); }
}

test('coexisting JS and TS implementations cannot double their identity', async () => {
  await siblings({ 'Probe.jsx': 'export const value = 1;', 'Probe.tsx': 'export const value = 2;' }, (result) => {
    assert.ok(forRule(result, duplicateRule).length >= 1, JSON.stringify(result));
  });
});

test('a checked same-module JS re-export bridge is permitted', async () => {
  await siblings({ 'Probe.js': '// @ts-check\nexport { value } from "./Probe.ts";', 'Probe.ts': 'export const value = 1;' }, (result) => {
    assert.equal(forRule(result, duplicateRule).length, 0, JSON.stringify(result));
  });
});

for (const bridge of [
  'export { value } from "./Probe.ts";',
  '// @ts-check\nexport { value } from "./Other.ts";',
  '// @ts-check\nexport { value } from "./Probe.ts"; export const duplicate = 2;',
  '// @ts-check\n// @ts-nocheck\nexport { value } from "./Probe.ts";',
]) {
  test(`an unchecked or implementing bridge is rejected: ${bridge}`, async () => {
    await siblings({ 'Probe.js': bridge, 'Probe.ts': 'export const value = 1;' }, (result) => {
      assert.ok(forRule(result, duplicateRule).length >= 1, JSON.stringify(result));
    });
  });
}

test('inline eslint disabling cannot hide a migrated type escape', async () => {
  const result = await messages('/* eslint-disable @typescript-eslint/no-explicit-any */\nexport const decode = (value: any) => value;', 'js/api/T10Probe.ts');
  assert.equal(forRule(result, '@typescript-eslint/no-explicit-any')[0]?.severity, 2);
});
