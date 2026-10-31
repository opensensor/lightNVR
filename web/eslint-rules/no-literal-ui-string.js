import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { canonicalAllowances, sourceIdentity } from './source-identity.js';

const baseline = canonicalAllowances(JSON.parse(fs.readFileSync(new URL('../i18n-literal-baseline.json', import.meta.url), 'utf8')));
const componentsRoot = fileURLToPath(new URL('../js/components/', import.meta.url));
const visibleAttributes = new Set(['title', 'aria-label', 'placeholder', 'alt']);

function candidate(value) {
  if (typeof value !== 'string') return null;
  const normalized = value.replace(/\s+/g, ' ').trim();
  return /[A-Za-z]/.test(normalized) ? normalized : null;
}

// Only inspect string branches that become JSX children. Calls such as t('key'),
// class names, API paths, and logging arguments are outside this rule's scope.
function expressionStrings(node) {
  if (!node) return [];
  if (node.type === 'Literal' && typeof node.value === 'string') return [node.value];
  if (['TSAsExpression', 'TSTypeAssertion', 'TSNonNullExpression', 'TSSatisfiesExpression', 'ChainExpression'].includes(node.type)) {
    return expressionStrings(node.expression);
  }
  if (node.type === 'TemplateLiteral') {
    return [...node.quasis.map((part) => part.value.cooked || ''), ...node.expressions.flatMap(expressionStrings)];
  }
  if (node.type === 'ArrayExpression') return node.elements.flatMap(expressionStrings);
  if (node.type === 'SequenceExpression') return expressionStrings(node.expressions.at(-1));
  if (node.type === 'ConditionalExpression') {
    return [...expressionStrings(node.consequent), ...expressionStrings(node.alternate)];
  }
  if (node.type === 'LogicalExpression') return expressionStrings(node.right);
  if (node.type === 'BinaryExpression' && node.operator === '+') {
    return [...expressionStrings(node.left), ...expressionStrings(node.right)];
  }
  return [];
}

export default {
  meta: {
    type: 'problem',
    docs: { description: 'Disallow new user-visible JSX literals' },
    schema: [],
  },
  create(context) {
    const file = path.relative(componentsRoot, context.filename).replaceAll(path.sep, '/');
    const allowed = baseline.get(sourceIdentity(file)) || {};
    const seen = new Map();

    function check(node, text) {
      const value = candidate(text);
      if (!value) return;
      const count = (seen.get(value) || 0) + 1;
      seen.set(value, count);
      if (process.env.I18N_LITERAL_BASELINE === '1' || count > (allowed[value] || 0)) {
        context.report({ node, message: `Untranslated UI string: ${JSON.stringify(value)}` });
      }
    }

    return {
      JSXText(node) { check(node, node.value); },
      JSXAttribute(node) {
        if (!visibleAttributes.has(node.name.name)) return;
        if (node.value?.type === 'Literal') check(node, node.value.value);
        if (node.value?.type === 'JSXExpressionContainer') {
          for (const value of expressionStrings(node.value.expression)) check(node, value);
        }
      },
      JSXExpressionContainer(node) {
        if (node.parent.type === 'JSXAttribute') return;
        for (const value of expressionStrings(node.expression)) check(node, value);
      },
    };
  },
};
