/** Camera inventory failures must not disconnect mounted live players. */
import { test, expect } from '@playwright/test';

for (const webrtcDisabled of [true, false]) {
  test(`inventory refresh preserves players with WebRTC ${webrtcDisabled ? 'disabled' : 'enabled'} @ui @liveview`, async ({ page }) => {
    let failInventory = false;
    let failedRequests = 0;
    let successfulRequests = 0;
    await page.clock.install();
    await page.route('**/api/**', async route => {
      const path = new URL(route.request().url()).pathname;
      if (path === '/api/streams' && failInventory) {
        failedRequests++;
        await route.fulfill({ status: 503, json: { error: 'Inventory temporarily unavailable' } });
        return;
      }
      if (path === '/api/streams') successfulRequests++;
      const responses: Record<string, unknown> = {
        '/api/client-config': {
          go2rtc_enabled: true, go2rtc_available: true,
          webrtc_disabled: webrtcDisabled, web_auth_enabled: false,
        },
        '/api/auth/verify': {
          authenticated: true, username: 'admin', role: 'admin', role_id: 0,
          auth_enabled: false,
        },
        '/api/setup/status': { complete: true },
        '/api/streams': {
          streams: [{
            name: 'cam_a', camera_uuid: 'camera-a', enabled: true,
            streaming_enabled: true, status: 'Running', availability: 'live',
            playback_transport: 'mse_only',
          }],
          total_pages: 1,
        },
      };
      await route.fulfill({ json: responses[path] || {} });
    });
    // No camera connection is needed to verify whether the player is torn down.
    await page.routeWebSocket('**/go2rtc/api/ws*', () => {});
    await page.goto('/index.html?mode=mse&cols=1&rows=1');
    const cell = page.locator('.video-cell[data-stream-name="cam_a"]').first();
    await expect(cell).toBeVisible();
    await cell.evaluate(element => { (window as any).originalLiveCell = element; });

    failInventory = true;
    await page.clock.runFor(30000);
    await expect.poll(() => failedRequests).toBeGreaterThan(0);
    // Allow the query's own retry and error state to finish.
    for (let retry = 0; retry < 3; retry++) {
      await page.clock.runFor(1000);
      await expect.poll(() => failedRequests).toBeGreaterThan(retry + 1);
    }
    await expect(cell).toBeVisible();
    await expect(page.locator('#video-grid')).not.toContainText('Error loading streams');
    expect(await cell.evaluate(element => element === (window as any).originalLiveCell)).toBe(true);

    failInventory = false;
    const successfulBeforeRecovery = successfulRequests;
    await page.clock.runFor(30000);
    await expect.poll(() => successfulRequests).toBeGreaterThan(successfulBeforeRecovery);
    await expect(cell).toBeVisible();
    expect(await cell.evaluate(element => element === (window as any).originalLiveCell)).toBe(true);
  });
}
