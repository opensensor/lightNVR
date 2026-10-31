// Default suite: unit tests only.
// The E2E specs need a running LightNVR server and a real browser, so they are
// excluded here and run via `npm run test:e2e` (jest.e2e.config.cjs).
module.exports = {
  testMatch: [
    '**/tests/**/*.spec.[jt]s?(x)'
  ],
  testPathIgnorePatterns: [
    '/node_modules/',
    '/tests/e2e/'
  ],
  testEnvironment: 'node',
  setupFilesAfterEnv: ['./tests/setup.cjs'],
  testTimeout: 30000,
  transform: {
    '^.+\\.[jt]sx?$': ['babel-jest', { configFile: './babel.config.cjs' }]
  }
};
