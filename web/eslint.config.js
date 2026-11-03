/**
 * ESLint configuration for the LightNVR web UI.
 *
 * Adds eslint-plugin-jsx-a11y (recommended ruleset) as part of the WCAG 2.2 AA
 * audit in https://github.com/opensensor/lightNVR/issues/647. All rules start
 * in warn mode so CI reports violations without failing the build; switch rules
 * to error as each finding category in the issue is closed.
 */
import jsxA11y from 'eslint-plugin-jsx-a11y';
import reactHooks from 'eslint-plugin-react-hooks';
import tseslint from 'typescript-eslint';
import noLiteralUiString from './eslint-rules/no-literal-ui-string.js';

const recommendedRules = jsxA11y.flatConfigs.recommended.rules;

const a11yWarnRules = Object.fromEntries(
  Object.keys(recommendedRules).map((name) => [name, 'warn']),
);

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
      ...a11yWarnRules,
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
