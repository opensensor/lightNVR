/**
 * ESLint configuration for the LightNVR web UI.
 *
 * eslint-plugin-jsx-a11y's recommended ruleset came from the WCAG 2.2 AA audit in
 * https://github.com/opensensor/lightNVR/issues/647. Its findings are closed, so
 * the rules fail lint. Suppress a rule only on the specific element, with a reason.
 */
import jsxA11y from 'eslint-plugin-jsx-a11y';
import reactHooks from 'eslint-plugin-react-hooks';
import tseslint from 'typescript-eslint';
import noLiteralUiString from './eslint-rules/no-literal-ui-string.js';

const recommendedRules = jsxA11y.flatConfigs.recommended.rules;

// Keep the preset's severity and options. control-has-associated-label is off in
// the preset but finds unnamed buttons and links, so it is enabled here. Its preset
// options exclude inputs, which it can't resolve through an external <label>.
const a11yRules = {
  ...recommendedRules,
  'jsx-a11y/control-has-associated-label': [
    'error',
    { ...recommendedRules['jsx-a11y/control-has-associated-label'][1], depth: 5 },
  ],
  // Labels in this UI commonly nest their text a few elements deep.
  'jsx-a11y/label-has-associated-control': ['error', { depth: 5 }],
};

export default [
  {
    ignores: ['dist/**', 'node_modules/**', 'coverage/**'],
  },
  {
    files: ['js/**/*.js', 'js/**/*.jsx', 'js/**/*.ts', 'js/**/*.tsx'],
    linterOptions: {
      // Targeted disable comments (e.g. react-hooks) belong to configs outside
      // this a11y lint; don't flag them as unused here.
      reportUnusedDisableDirectives: 'off',
    },
    plugins: { 'jsx-a11y': jsxA11y, 'react-hooks': reactHooks },
    languageOptions: {
      ...jsxA11y.flatConfigs.recommended.languageOptions,
      sourceType: 'module',
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: 'module',
        ecmaFeatures: { jsx: true },
      },
    },
    rules: {
      ...a11yRules,
      // The plugin is registered only so pre-existing targeted
      // eslint-disable-next-line comments referencing react-hooks rules
      // resolve; its rules stay off while this config tracks the a11y audit.
      'react-hooks/exhaustive-deps': 'off',
      'react-hooks/rules-of-hooks': 'off',
    },
  },
  {
    // The a11y and i18n rules above and below apply here too; this block only
    // swaps the parser and adds the TS escape-hatch policy. The parser picks
    // JSX support from the extension: .tsx has it, .ts does not.
    files: ['js/**/*.ts', 'js/**/*.tsx'],
    plugins: { '@typescript-eslint': tseslint.plugin },
    languageOptions: { parser: tseslint.parser },
    rules: {
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/ban-ts-comment': ['error', {
        'ts-check': false,
        'ts-expect-error': 'allow-with-description',
        'ts-ignore': true,
        'ts-nocheck': true,
      }],
    },
  },
  {
    files: ['js/components/**/*.js', 'js/components/**/*.jsx', 'js/components/**/*.ts', 'js/components/**/*.tsx'],
    // Existing technical labels and examples are counted by file in the
    // baseline. New JSX text or visible-attribute literals fail lint.
    plugins: { 'lightnvr-i18n': { rules: { 'no-literal-ui-string': noLiteralUiString } } },
    rules: { 'lightnvr-i18n/no-literal-ui-string': 'error' },
  },
];
