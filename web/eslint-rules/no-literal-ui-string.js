import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const baseline = JSON.parse(fs.readFileSync(new URL('../i18n-literal-baseline.json', import.meta.url), 'utf8'));
const componentsRoot = fileURLToPath(new URL('../js/components/', import.meta.url));
const visibleAttributes = new Set(['title', 'aria-label', 'placeholder', 'alt']);

function candidate(value) {
  const normalized = value.replace(/\s+/g, ' ').trim();
  return /[A-Za-z]/.test(normalized) ? normalized : null;
}

// Only inspect string branches that become JSX children. Calls such as t('key'),
// class names, API paths, and logging arguments are outside this rule's scope.
function expressionStrings(node) {
  if (!node) return [];
  if (node.type === 'Literal' && typeof node.value === 'string') return [node.value];
  if (node.type === 'TemplateLiteral') return node.quasis.map((part) => part.value.cooked || '');
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
    const allowed = baseline[file] || {};
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
        if (visibleAttributes.has(node.name.name) && node.value?.type === 'Literal') {
          check(node, node.value.value);
        }
      },
      JSXExpressionContainer(node) {
        if (node.parent.type === 'JSXAttribute') return;
        for (const value of expressionStrings(node.expression)) check(node, value);
      },
    };
  },
};
