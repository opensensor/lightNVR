// E2E suite: Selenium tests driving a real browser.
//
// Requires a LightNVR server reachable at E2E_BASE_URL (default
// http://localhost:8080). Run with `npm run test:e2e`.
//
// Env:
//   E2E_BASE_URL   base URL of the running server (default http://localhost:8080)
//   E2E_BROWSER    'chrome' (default) or 'firefox'
//   E2E_HEADLESS   'false' to watch the browser; headless otherwise
module.exports = {
  testMatch: [
    '**/tests/e2e/**/*.spec.js'
  ],
  testEnvironment: 'node',
  setupFilesAfterEnv: ['./tests/setup.cjs'],
  testTimeout: 60000,
  transform: {
    '^.+\\.jsx?$': ['babel-jest', { configFile: './babel.config.cjs' }]
  }
};
