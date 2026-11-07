import { test, expect } from './permissionCases';

for (const status of [200, 503]) {
  test(`restart observation ends after 120 seconds without an observed outage (HTTP ${status})`, async ({ server, browser }) => {
    const context = await server.browserContext(browser);
    const page = await context.newPage();
    try {
      let writes = 0;
      let statusReads = 0;
      let completedReads = 0;
      page.on('requestfinished', request => {
        if (new URL(request.url()).pathname === '/api/system/status') completedReads++;
      });
      let reloads = 0;
      await page.route('**/api/system/restart', route => {
        writes++;
        return route.fulfill({
          status,
          json: status === 200
            ? { success: true, message: 'Restart requested' }
            : { error: 'Ambiguous restart acknowledgment' }
        });
      });
      await page.route('**/api/system/status', route => {
        statusReads++;
        return route.fulfill({ json: { status: 'ok', message: 'System running' } });
      });
      await page.goto(`${server.baseURL}/system.html`);
      const clockStart = new Date('2026-01-01T00:00:00Z');
      await page.clock.install({ time: clockStart });
      page.on('framenavigated', frame => {
        if (frame === page.mainFrame()) reloads++;
      });
      await page.locator('#restart-btn').click();
      await page.getByRole('button', { name: 'Restart', exact: true }).click();
      await expect(page.getByRole('heading', { name: 'Restarting LightNVR', exact: true })).toBeVisible();
      await page.clock.pauseAt(await page.evaluate(() => Date.now() + 100));
      await page.clock.runFor(500);
      await expect.poll(() => statusReads).toBeGreaterThan(0);
      await expect.poll(() => completedReads).toBe(statusReads);
      const readsBeforeDeadline = statusReads;
      await page.clock.fastForward(118500);
      await expect.poll(() => statusReads).toBeGreaterThan(readsBeforeDeadline);
      // Settle real network responses before advancing their synthetic timeout.
      await expect.poll(() => completedReads).toBe(statusReads);
      await expect(page.getByRole('button', { name: 'Refresh Page', exact: true })).toHaveCount(0);
      expect(reloads).toBe(0);
      expect(writes).toBe(1);

      await page.clock.fastForward(1000);
      await expect(page.getByText('Server is taking longer than expected. Please refresh manually.', { exact: true })).toBeVisible();
      const refresh = page.getByRole('button', { name: 'Refresh Page', exact: true });
      await expect(refresh).toBeEnabled();
      expect(reloads).toBe(0);
      expect(writes).toBe(1);
      const readsAtDeadline = statusReads;
      await page.clock.fastForward(60000);
      expect(statusReads).toBe(readsAtDeadline);

      await page.clock.resume();
      await refresh.click();
      await expect.poll(() => reloads).toBe(1);
      expect(writes).toBe(1);
    } finally {
      await context.close();
    }
  });
}
