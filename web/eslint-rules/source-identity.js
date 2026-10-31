import fs from 'node:fs';
import path from 'node:path';
import tseslint from 'typescript-eslint';

const extensions = ['.js', '.jsx', '.ts', '.tsx'];

// Extensions describe the compiler, not a new implementation or allowance.
export function sourceIdentity(filename) {
  return filename.replaceAll(path.sep, '/').replace(/\.[jt]sx?$/, '');
}

export function canonicalAllowances(baseline) {
  const result = new Map();
  for (const [filename, counts] of Object.entries(baseline)) {
    const identity = sourceIdentity(filename);
    if (result.has(identity)) throw new Error(`Duplicate i18n baseline identity: ${identity}`);
    result.set(identity, counts);
  }
  return result;
}

function checkedReExport(filename, siblings) {
  if (!/\.jsx?$/.test(filename)) return false;
  try {
    const ast = tseslint.parser.parseForESLint(fs.readFileSync(filename, 'utf8'), {
      filePath: filename,
      sourceType: 'module',
      ecmaVersion: 'latest',
      ecmaFeatures: { jsx: true },
      comment: true,
    }).ast;
    const comments = ast.comments || [];
    if (!comments.some(({ value }) => /^\s*@ts-check\b/.test(value))) return false;
    if (comments.some(({ value }) => /@ts-(?:nocheck|ignore|expect-error)\b/.test(value))) return false;
    return ast.body.length > 0 && ast.body.every((statement) => {
      if (!['ExportNamedDeclaration', 'ExportAllDeclaration'].includes(statement.type)
          || statement.declaration || typeof statement.source?.value !== 'string') return false;
      const target = path.resolve(path.dirname(filename), statement.source.value);
      return /\.tsx?$/.test(target) && siblings.includes(target)
        && sourceIdentity(filename) === sourceIdentity(target);
    });
  } catch {
    // A broken or unreadable sibling cannot be treated as a safe bridge.
    return false;
  }
}

export function duplicateImplementations(filename) {
  const identity = filename.replace(/\.[jt]sx?$/, '');
  const siblings = extensions.map((extension) => identity + extension).filter((file) => fs.existsSync(file));
  if (siblings.length < 2) return [];
  const implementations = siblings.filter((file) => !checkedReExport(file, siblings));
  return implementations.length > 1 ? implementations : [];
}
