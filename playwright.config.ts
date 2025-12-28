import { defineConfig, devices } from '@playwright/test';

/**
 * LightNVR Integration Test Configuration
 *
 * Usage:
 *   npx playwright test                    # Run all tests headless
 *   npx playwright test --headed           # Run with visible browser
 *   npx playwright test --ui               # Run with Playwright UI mode
 *   npx playwright test --project=ui       # Run only UI tests
 *   npx playwright test --project=api      # Run only API tests
 *   npx playwright test --debug            # Run in debug mode with inspector
 *
 * Test categories:
 *   @api     - API tests (no browser needed)
 *   @ui      - Browser UI tests
 *   @go2rtc  - go2rtc streaming tests
 */

// Check if running in headed/debug mode
const isHeaded = process.env.HEADED === '1' || process.argv.includes('--headed');
const isDebug = process.env.PWDEBUG === '1' || process.argv.includes('--debug');

export default defineConfig({
  testDir: './tests/integration',

  // Run tests in parallel (disabled for integration tests)
  fullyParallel: false,

  // Fail the build on CI if you accidentally left test.only in the source code
  forbidOnly: !!process.env.CI,

  // Retry on CI only
  retries: process.env.CI ? 2 : 0,

  // Limit workers for integration tests
  workers: 1,

  // Reporter - use html in CI for artifact, list for headed/debug mode
  reporter: process.env.CI
    ? [['github'], ['html', { open: 'never' }]]
    : (isHeaded || isDebug)
      ? 'list'
      : 'html',

  // Timeout for each test (longer for headed mode)
  timeout: isHeaded || isDebug ? 120000 : 60000,

  // Expect timeout
  expect: {
    timeout: isHeaded || isDebug ? 15000 : 5000,
  },

  // Output directory for test artifacts
  outputDir: './test-results',

  // Global setup and teardown
  globalSetup: './tests/integration/global-setup.ts',
  globalTeardown: './tests/integration/global-teardown.ts',

  use: {
    // Base URL for tests
    baseURL: process.env.LIGHTNVR_URL || 'http://localhost:18080',

    // Collect trace when retrying the failed test
    trace: 'on-first-retry',

    // Screenshot on failure
    screenshot: 'only-on-failure',

    // Video on failure
    video: 'on-first-retry',

    // Slow down actions in headed mode for visibility
    ...(isHeaded && {
      launchOptions: {
        slowMo: 500,
      },
    }),
  },

  // Configure projects for different test types
  projects: [
    {
      name: 'api',
      testMatch: /.*\.api\.spec\.ts/,
      use: {
        // API tests don't need a browser
      },
    },
    {
      name: 'ui',
      testMatch: /.*\.ui\.spec\.ts/,
      use: {
        ...devices['Desktop Chrome'],
        // Always use headed mode for UI tests when HEADED=1
        headless: !isHeaded && !isDebug,
        viewport: { width: 1280, height: 720 },
      },
    },
    {
      name: 'go2rtc',
      testMatch: /.*\.go2rtc\.spec\.ts/,
      use: {
        // go2rtc tests don't need a browser
      },
    },
  ],
});

