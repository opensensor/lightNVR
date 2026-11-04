const jsxPlugin = ['@babel/plugin-transform-react-jsx', { runtime: 'automatic', importSource: 'preact' }];
const typescriptPreset = ['@babel/preset-typescript', { onlyRemoveTypeImports: true }];

module.exports = {
  presets: [
    [
      '@babel/preset-env',
      {
        targets: {
          node: 'current',
        },
      },
    ],
  ],
  // Plain .js keeps compiling as it always has: no TS, no JSX transform.
  overrides: [
    { test: /\.ts$/, presets: [typescriptPreset] },
    { test: /\.tsx$/, presets: [typescriptPreset], plugins: [jsxPlugin] },
    { test: /\.jsx$/, plugins: [jsxPlugin] },
  ],
};
