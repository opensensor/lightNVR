import fs from 'node:fs';
import path from 'node:path';
import { parse } from '@babel/parser';

const SOURCE_ROOT = path.resolve(__dirname, '../js');
const BROWSER_DIALOG_NAMES = new Set(['alert', 'confirm', 'prompt']);

function sourceFiles(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const absolutePath = path.join(directory, entry.name);
    if (entry.isDirectory()) return sourceFiles(absolutePath);
    return /\.(?:js|jsx)$/.test(entry.name) ? [absolutePath] : [];
  });
}

function walkAst(value, visit) {
  if (!value || typeof value !== 'object') return;
  if (Array.isArray(value)) {
    value.forEach((item) => walkAst(item, visit));
    return;
  }
  if (typeof value.type === 'string') visit(value);
  Object.entries(value).forEach(([key, child]) => {
    if (!['loc', 'start', 'end'].includes(key)) walkAst(child, visit);
  });
}

describe('browser dialog policy', () => {
  test('first-party UI code does not call alert, confirm, or prompt', () => {
    const violations = [];

    sourceFiles(SOURCE_ROOT).forEach((file) => {
      const ast = parse(fs.readFileSync(file, 'utf8'), {
        plugins: ['jsx'],
        sourceFilename: file,
        sourceType: 'module',
      });

      walkAst(ast, (node) => {
        if (node.type !== 'CallExpression') return;
        const directName = node.callee?.type === 'Identifier' ? node.callee.name : '';
        const windowName = node.callee?.type === 'MemberExpression' &&
          !node.callee.computed && node.callee.object?.name === 'window'
          ? node.callee.property?.name : '';
        const name = directName || windowName;
        if (BROWSER_DIALOG_NAMES.has(name)) {
          violations.push(`${path.relative(SOURCE_ROOT, file)}:${node.loc?.start.line || '?'}`);
        }
      });
    });

    expect(violations).toEqual([]);
  });
});
