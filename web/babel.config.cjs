module.exports = {
  presets: [
    '@babel/preset-typescript',
    ['@babel/preset-react', { runtime: 'automatic', importSource: 'preact' }],
    [
      '@babel/preset-env',
      {
        targets: {
          node: 'current',
        },
      },
    ],
  ],
};
