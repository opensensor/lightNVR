import { test, expect } from '@playwright/test';

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
      '/api/streams': {
        streams: [{
          name: 'cam_a', camera_uuid: 'camera-a', enabled: true,
          streaming_enabled: true, status: 'Running', availability: 'live',
          playback_transport: 'mse_only', detection_based_recording: true,
          detection_model: 'api-detection', detection_interval: 10,
        }],
        total_pages: 1,
      },
    };
    await route.fulfill({ json: responses[path] || {} });
  });
  await page.routeWebSocket('**/go2rtc/api/ws*', () => {});
  try {
    await page.goto('/index.html?mode=mse&cols=1&rows=1');
    const cell = page.locator('.video-cell[data-stream-name="cam_a"]').first();
    await expect(cell).toBeVisible();
    // Trigger the video's playback state without requiring a camera source.
    await cell.locator('video').evaluate(video => {
      Object.defineProperty(video, 'videoWidth', { value: 640 });
      Object.defineProperty(video, 'videoHeight', { value: 480 });
      video.dispatchEvent(new Event('play'));
    });
    await page.clock.runFor(4000);
    await expect.poll(() => detectionRequests).toBe(1);
    await page.getByRole('button', { name: 'Hide All Detection Overlays', exact: true }).click();
    await expect.poll(() => abortedRequests).toBe(1);
    await page.clock.runFor(10000);
    expect(detectionRequests).toBe(1);
  } finally {
    releaseResponse();
  }
});
