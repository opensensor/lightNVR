/**
 * ESLint configuration for the LightNVR web UI.
 *
 * Adds eslint-plugin-jsx-a11y (recommended ruleset) as part of the WCAG 2.2 AA
 * audit in https://github.com/opensensor/lightNVR/issues/647. Legacy JS findings remain warnings recorded in the findings inventory.
 * Migrated TS/TSX sources enforce accessibility, hooks and type-escape rules.
 */
import tseslint from 'typescript-eslint';
import jsxA11y from 'eslint-plugin-jsx-a11y';
import reactHooks from 'eslint-plugin-react-hooks';
import noLiteralUiString from './eslint-rules/no-literal-ui-string.js';
import noDuplicateImplementation from './eslint-rules/no-duplicate-implementation.js';

const recommendedRules = jsxA11y.flatConfigs.recommended.rules;

const a11yWarnRules = Object.fromEntries(
  Object.keys(recommendedRules).map((name) => [name, 'warn']),
);

export default [
  {
    ignores: ['dist/**', 'node_modules/**', 'coverage/**'],
  },
  {
    files: ['js/**/*.{js,jsx,ts,tsx}'],
    linterOptions: {
      // Targeted disable comments (e.g. react-hooks) belong to configs outside
      // this a11y lint; don't flag them as unused here.
      reportUnusedDisableDirectives: 'off',
    },
    plugins: {
      'jsx-a11y': jsxA11y,
      'react-hooks': reactHooks,
      'lightnvr-source': { rules: { 'no-duplicate-implementation': noDuplicateImplementation } },
    },
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
      'lightnvr-source/no-duplicate-implementation': 'error',
      // The plugin is registered only so pre-existing targeted
      // eslint-disable-next-line comments referencing react-hooks rules
      // resolve; its rules stay off while this config tracks the a11y audit.
      'react-hooks/exhaustive-deps': 'off',
      'react-hooks/rules-of-hooks': 'off',
    },
  },
  {
    files: ['js/components/**/*.{js,jsx,ts,tsx}'],
    // Existing technical labels and examples are counted by file in the
    // baseline. New JSX text or visible-attribute literals fail lint.
    plugins: { 'lightnvr-i18n': { rules: { 'no-literal-ui-string': noLiteralUiString } } },
    rules: { 'lightnvr-i18n/no-literal-ui-string': 'error' },
  },
  {
    files: ['js/**/*.{ts,tsx}'],
    languageOptions: { parser: tseslint.parser },
    plugins: { '@typescript-eslint': tseslint.plugin },
    linterOptions: { noInlineConfig: true, reportUnusedDisableDirectives: 'error' },
    rules: {
      ...Object.fromEntries(Object.keys(recommendedRules).map((name) => [name, 'error'])),
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'error',
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/ban-ts-comment': ['error', {
        'ts-check': false, 'ts-nocheck': true, 'ts-ignore': true, 'ts-expect-error': true,
      }],
    },
  },
];
