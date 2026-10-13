import { test, expect } from '@playwright/test';

test('snapshot requests stay stable until a camera is re-enabled @ui @streams', async ({ page }) => {
  const snapshotUrls: string[] = [];
  const streams = ['cam_a', 'cam_b', 'cam_c'].map(name => ({
    name, camera_uuid: name, enabled: true, streaming_enabled: true,
    status: 'Running', availability: 'live', can_configure: true,
    width: 640, height: 480, fps: 15, codec: 'h264', protocol: 0,
    record: false, tags: '', playback_transport: 'mse_only',
  }));
  await page.route('**/api/**', async route => {
    const path = new URL(route.request().url()).pathname;
    if (path === '/api/streams/cam_a') {
      streams[0].enabled = route.request().method() === 'PUT';
      await route.fulfill({ json: { success: true } });
      return;
    }
    if (path === '/go2rtc/api/frame.jpeg') {
      snapshotUrls.push(route.request().url());
      await route.fulfill({
        contentType: 'image/png', headers: { 'Cache-Control': 'no-store' },
        body: Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+a6V8AAAAASUVORK5CYII=', 'base64'),
      });
      return;
    }
    const responses: Record<string, unknown> = {
      '/api/client-config': { go2rtc_enabled: true, web_auth_enabled: false },
      '/api/auth/verify': {
        authenticated: true, username: 'admin', role: 'admin', role_id: 0,
        auth_enabled: false,
      },
      '/api/streams': { streams, total: 3, total_pages: 1, page: 1, page_size: 50 },
      '/api/detection/models': { models: [] },
    };
    await route.fulfill({ json: responses[path] || {} });
  });
  await page.goto('/streams.html');
  const cards = page.locator('.stream-card');
  await expect(cards).toHaveCount(3);
  await expect.poll(() => snapshotUrls.length).toBe(3);
  await expect.poll(() => cards.locator('img').evaluateAll(images =>
    images.every(image => (image as HTMLImageElement).complete &&
      (image as HTMLImageElement).naturalWidth > 0))).toBe(true);
  const initialSources = await cards.locator('img').evaluateAll(images =>
    images.map(image => (image as HTMLImageElement).src));

  await page.getByRole('button', { name: 'Select', exact: true }).click();
  await cards.first().getByRole('checkbox').check();
  await expect(cards.first()).toHaveClass(/ring-2/);
  const updatedSources = await cards.locator('img').evaluateAll(images =>
    images.map(image => (image as HTMLImageElement).src));
  expect(updatedSources).toEqual(initialSources);
  expect(snapshotUrls).toHaveLength(3);

  const camera = page.locator('.stream-card[data-stream="cam_a"]');
  const disable = camera.getByRole('button', { name: 'Disable stream', exact: true });
  await disable.click();
  await disable.click();
  const enable = camera.getByRole('button', { name: 'Enable stream', exact: true });
  await expect(enable).toBeVisible();
  expect(snapshotUrls).toHaveLength(3);
  await enable.click();
  await expect(disable).toBeVisible();
  await expect.poll(() => snapshotUrls.length).toBe(4);
  await expect(camera.locator('img')).toBeVisible();
  expect(new URL(snapshotUrls[3]).searchParams.get('src')).toBe('cam_a');
  expect(snapshotUrls[3]).not.toBe(snapshotUrls[0]);
});
