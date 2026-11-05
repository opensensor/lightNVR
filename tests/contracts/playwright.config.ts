import {defineConfig, devices} from '@playwright/test';
export default defineConfig({
  testDir: '.', fullyParallel: false, workers: 1, retries: 0,
  timeout: 90000, expect: {timeout: 10000},
  outputDir: '../../test-results/contracts',
  reporter: [['list']],
  use: {trace: 'retain-on-failure', screenshot: 'only-on-failure'},
  projects: [
    {name: 'api', testMatch: [/baseline\.spec\.ts$/, /.*\.api\.spec\.ts$/]},
    {name: 'ui', testMatch: /.*\.ui\.spec\.ts$/, use: {...devices['Desktop Chrome']}},
  ],
});
