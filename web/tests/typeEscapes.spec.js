import fs from 'node:fs';
import path from 'node:path';
import { parse } from '@babel/parser';

const SOURCE_ROOT = path.resolve(__dirname, '../js');

// The only non-TS source allowed under web/js. See "TypeScript conventions" in
// web/VITE.md for why each one must keep its name.
const ALLOWED_JS = new Set(['version.js', 'utils/theme-init.js']);

// Matches the directive forms TypeScript honors, not text that merely names them:
// the directive has to open a line comment, or a line of a block comment.
const DIRECTIVE = /^[/*\s]*@ts-(nocheck|ignore)\b/;

function allFiles(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const absolutePath = path.join(directory, entry.name);
    return entry.isDirectory() ? allFiles(absolutePath) : [absolutePath];
  });
}

function parserPlugins(file) {
  if (file.endsWith('.d.ts')) return [['typescript', { dts: true }]];
  if (file.endsWith('.ts')) return ['typescript'];
  return ['typescript', 'jsx'];
}

function walkAst(value, visit) {
  if (!value || typeof value !== 'object') return;
  if (Array.isArray(value)) {
    value.forEach((item) => walkAst(item, visit));
    return;
  }
  if (typeof value.type === 'string') visit(value);
  Object.entries(value).forEach(([key, child]) => {
    // Comments are checked separately and hold no type nodes.
    if (!['loc', 'start', 'end', 'comments', 'leadingComments', 'trailingComments', 'innerComments', 'extra'].includes(key)) {
      walkAst(child, visit);
    }
  });
}

/** Every banned escape in one source text, as `line: description`. */
function findEscapes(source, file) {
  const ast = parse(source, {
    plugins: parserPlugins(file),
    sourceFilename: file,
    sourceType: 'module',
  });
  const found = [];

  ast.comments.forEach((comment) => {
    comment.value.split('\n').forEach((line, offset) => {
      const match = DIRECTIVE.exec(line);
      if (match) found.push(`${comment.loc.start.line + offset}: @ts-${match[1]}`);
    });
  });

  // TSAnyKeyword covers every spelling: `: any`, `as any`, `<any>`, `any[]`,
  // `Array<any>`, `Record<string, any>`.
  walkAst(ast, (node) => {
    if (node.type === 'TSAnyKeyword') found.push(`${node.loc.start.line}: any`);
  });

  return found;
}

describe('type escape scanner', () => {
  test.each([
    ['a @ts-nocheck header', '// @ts-nocheck\nexport const a = 1;\n', '1: @ts-nocheck'],
    ['a triple-slash @ts-nocheck', '/// @ts-nocheck\nexport const a = 1;\n', '1: @ts-nocheck'],
    ['@ts-ignore', 'export const a = 1;\n// @ts-ignore\nexport const b: string = a;\n', '2: @ts-ignore'],
    ['@ts-ignore in a block comment', '/* @ts-ignore */\nexport const a: string = 1;\n', '1: @ts-ignore'],
    ['@ts-ignore on a later block comment line', '/**\n * @ts-ignore\n */\nexport const a: string = 1;\n', '2: @ts-ignore'],
    [': any', 'export const a: any = 1;\n', '1: any'],
    ['as any', 'export const a = 1 as any;\n', '1: any'],
    ['<any> assertion', 'export const a = <any>1;\n', '1: any'],
    ['<any> type argument', 'export const a = new Set<any>();\n', '1: any'],
    ['any[]', 'export const a: any[] = [];\n', '1: any'],
    ['an any parameter', 'export function f(a: any) { return a; }\n', '1: any'],
  ])('reports %s', (_name, source, expected) => {
    expect(findEscapes(source, 'probe.ts')).toEqual([expected]);
  });

  test('reports escapes in TSX, including JSX comments and type arguments', () => {
    const source = 'export const a = <div>{/* @ts-ignore */}{(1 as any)}</div>;\n';
    expect(findEscapes(source, 'probe.tsx').sort()).toEqual(['1: @ts-ignore', '1: any']);
  });

  test('reports any in a declaration file', () => {
    expect(findEscapes('declare const a: any;\nexport { a };\n', 'probe.d.ts')).toEqual(['1: any']);
  });

  test.each([
    ['strings', "export const a = '// @ts-nocheck as any : any <any>';\n", 'probe.ts'],
    ['template literals', 'export const a = `// @ts-ignore\n as any`;\n', 'probe.ts'],
    ['mentions in comments', '// This must not use @ts-ignore or `as any`.\n// see : any\nexport const a = 1;\n', 'probe.ts'],
    ['JSX text', 'export const a = <p>as any : any // @ts-nocheck</p>;\n', 'probe.tsx'],
    ['identifiers named any', 'const any = 1;\nexport const a = { any };\n', 'probe.ts'],
    ['the allowed escapes', '// @ts-expect-error reason\nexport const a: string = 1 as unknown as string;\n', 'probe.ts'],
  ])('ignores %s', (_name, source, file) => {
    expect(findEscapes(source, file)).toEqual([]);
  });
});

describe('type escape policy', () => {
  const files = allFiles(SOURCE_ROOT);

  test('web/js has no @ts-nocheck, @ts-ignore, or any', () => {
    const violations = files
      .filter((file) => /\.tsx?$/.test(file))
      .flatMap((file) => findEscapes(fs.readFileSync(file, 'utf8'), file)
        .map((finding) => `${path.relative(SOURCE_ROOT, file)}:${finding}`));

    expect(violations).toEqual([]);
  });

  test('only the documented .js exceptions remain in web/js', () => {
    const unexpected = files
      .filter((file) => /\.(?:js|jsx|mjs|cjs)$/.test(file))
      .map((file) => path.relative(SOURCE_ROOT, file).split(path.sep).join('/'))
      .filter((file) => !ALLOWED_JS.has(file));

    expect(unexpected).toEqual([]);
  });
});
