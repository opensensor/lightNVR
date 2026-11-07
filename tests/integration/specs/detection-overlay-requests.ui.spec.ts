import { test, expect } from '@playwright/test';
import { cameraQueryFixture, collectionsFixture } from '../fixtures/fleet-contracts';
import { streamSummary, streamSummaryPage } from '../fixtures/stream-summary';

test('slow detection polls do not overlap and stop when overlays close @ui @liveview', async ({ page }) => {
  let detectionRequests = 0;
  let abortedRequests = 0;
  let releaseResponse: () => void = () => {};
  const heldResponse = new Promise<void>(resolve => { releaseResponse = resolve; });
  await page.clock.install();
  page.on('requestfailed', request => {
    if (request.url().includes('/api/detection/results/')) abortedRequests++;
  });
  await page.route('**/api/**', async route => {
    const path = new URL(route.request().url()).pathname;
    if (path === '/api/detection/results/cam_a') {
      detectionRequests++;
      await heldResponse;
      // Closing the overlay aborts this request before the upstream responds.
      await route.fulfill({ json: { detections: [] } }).catch(() => {});
      return;
    }
    const responses: Record<string, unknown> = {
      '/api/client-config': {
        go2rtc_enabled: true, go2rtc_available: true,
        webrtc_disabled: true, web_auth_enabled: false,
      },
      '/api/auth/verify': {
        authenticated: true, username: 'admin', role: 'admin', role_id: 0,
        auth_enabled: false,
      },
      '/api/setup/status': { complete: true },
      '/api/camera-collections': collectionsFixture([]),
      '/api/cameras/query': cameraQueryFixture([{ name: 'cam_a', camera_uuid: '00000000-0000-4000-8000-000000000001' }]),
      '/api/locations': { locations: [], count: 0 },
      '/api/streams': streamSummaryPage([streamSummary('cam_a', {
        camera_uuid: '00000000-0000-4000-8000-000000000001',
        streaming_enabled: true, playback_transport: 'mse_only',
        detection_based_recording: true, detection_model: 'api-detection',
        detection_interval: 10,
      })]),
    };
    await route.fulfill({ json: responses[path] || {} });
  });
  await page.routeWebSocket('**/go2rtc/api/ws*', () => {});
  try {
    await page.goto('/index.html?mode=mse&cols=1&rows=1');
    const cell = page.locator('.video-cell[data-stream-name="cam_a"]').first();
    await expect(cell).toBeVisible();
    // Let bootstrap finish before freezing time. Explicit advances below keep
    // CI interaction latency from exhausting the five-second request timeout.
    await page.clock.pauseAt(await page.evaluate(() => Date.now() + 100));
    // Simulate a decoded frame: MSE enables overlays only after `playing`
    // with current video data and an unpaused media element.
    await cell.locator('video').evaluate(video => {
      Object.defineProperties(video, {
        videoWidth: { value: 640 },
        videoHeight: { value: 480 },
        readyState: { value: HTMLMediaElement.HAVE_CURRENT_DATA },
        paused: { value: false },
      });
      video.dispatchEvent(new Event('playing'));
    });
    await page.clock.runFor(4000);
    await expect.poll(() => detectionRequests).toBe(1);
    expect(abortedRequests).toBe(0);
    await page.getByRole('button', { name: 'Hide All Detection Overlays', exact: true }).click();
    // Flush the component's scheduled render/unmount without reaching timeout.
    await page.clock.runFor(100);
    await expect.poll(() => abortedRequests).toBe(1);
    await page.clock.runFor(10000);
    expect(detectionRequests).toBe(1);
  } finally {
    releaseResponse();
  }
});
