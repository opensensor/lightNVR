/**
 * Sub-stream ↔ main-stream swap on per-cell fullscreen (#366)
 *
 * When a stream is configured with sub_stream_url and rendered in a grid
 * (>1 cell), the video cell uses the `{name}_sub` go2rtc source. Entering
 * native fullscreen on that specific cell must swap it to the main stream
 * so the user sees full-resolution video, not the low-res dashboard feed.
 *
 * We assert this via the `data-sub-stream` attribute on each `.video-cell`,
 * which mirrors the `useSubStream` prop resolved by LiveView / WebRTCView.
 * Fullscreen is simulated by overriding `document.fullscreenElement` and
 * dispatching `fullscreenchange` — headless Chromium's real Fullscreen API
 * is gated on user activation and flaky enough in CI that a simulated
 * event gives us deterministic coverage of the behavior we actually care
 * about (the state wiring from `fullscreenchange` → `useSubStream`).
 *
 * @tags @ui @liveview
 */

import { test, expect } from '@playwright/test';
import { USERS, login, sleep } from '../fixtures/test-fixtures';

test.describe('Sub-stream swap on per-cell fullscreen @ui @liveview', () => {
  const STREAMS = [
    { name: 'cam_a', sub_stream_url: 'rtsp://192.168.1.50/sub' },
    { name: 'cam_b', sub_stream_url: 'rtsp://192.168.1.51/sub' },
  ];

  /**
   * Produce a stream-details object that satisfies LiveView's filter:
   * enabled + streaming_enabled + not soft-deleted, with sub_stream_url set.
   */
  function fullStreamDetails(name: string, subUrl: string) {
    return {
      name,
      url: `rtsp://192.168.1.100/${name}`,
      sub_stream_url: subUrl,
      enabled: true,
      streaming_enabled: true,
      is_deleted: false,
      status: 'Running',
      width: 1920,
      height: 1080,
      fps: 15,
      codec: 'h264',
      protocol: 0,
      priority: 5,
      segment_duration: 30,
      record: false,
      record_audio: false,
      go2rtc_hls_available: false,
    };
  }

  test.beforeEach(async ({ page }) => {
    // Short-circuit go2rtc snapshot probes so the grid can render without
    // waiting on an external service.
    await page.route('**/go2rtc/api/frame.jpeg*', route => {
      route.fulfill({ status: 204, body: '' }).catch(() => {});
    });

    // List endpoint
    await page.route('**/api/streams', route => route.fulfill({
      json: STREAMS.map(s => ({ name: s.name, enabled: true, streaming_enabled: true })),
    }));

    // Per-stream details endpoint (called by LiveView for each listed stream)
    for (const s of STREAMS) {
      await page.route(`**/api/streams/${s.name}`, route => route.fulfill({
        json: fullStreamDetails(s.name, s.sub_stream_url),
      }));
    }

    // Stub HLS / go2rtc endpoints so the video cells don't spam 404s.
    await page.route('**/hls/**', route => route.fulfill({ status: 204, body: '' }));
    await page.route('**/go2rtc/api/streams*', route => route.fulfill({ json: {} }));

    await login(page, USERS.admin);
  });

  test('cell entering fullscreen switches from sub to main stream', async ({ page }) => {
    await page.goto('/index.html?cols=2&rows=2', { waitUntil: 'domcontentloaded' });

    const cellA = page.locator('.video-cell[data-stream-name="cam_a"]').first();
    const cellB = page.locator('.video-cell[data-stream-name="cam_b"]').first();

    await expect(cellA).toBeVisible({ timeout: 15000 });
    await expect(cellB).toBeVisible({ timeout: 15000 });

    // Both cells start in grid mode → both should be using the sub-stream.
    await expect(cellA).toHaveAttribute('data-sub-stream', 'true');
    await expect(cellB).toHaveAttribute('data-sub-stream', 'true');

    // Simulate cam_a entering per-cell native fullscreen. We override
    // `document.fullscreenElement` with a configurable getter so the
    // `useFullscreenCellStream` hook's `fullscreenchange` handler picks
    // up cam_a as the active cell.
    await page.evaluate(() => {
      const target = document.querySelector('.video-cell[data-stream-name="cam_a"]');
      Object.defineProperty(document, 'fullscreenElement', {
        configurable: true,
        get: () => target,
      });
      document.dispatchEvent(new Event('fullscreenchange'));
    });

    // cam_a must swap to the main stream; cam_b stays on sub.
    await expect(cellA).toHaveAttribute('data-sub-stream', 'false');
    await expect(cellB).toHaveAttribute('data-sub-stream', 'true');

    // Exit fullscreen → both cells back on sub.
    await page.evaluate(() => {
      Object.defineProperty(document, 'fullscreenElement', {
        configurable: true,
        get: () => null,
      });
      document.dispatchEvent(new Event('fullscreenchange'));
    });

    await expect(cellA).toHaveAttribute('data-sub-stream', 'true');
    await expect(cellB).toHaveAttribute('data-sub-stream', 'true');
  });

  test('single-stream layout never uses sub-stream', async ({ page }) => {
    await page.goto('/index.html?cols=1&rows=1&stream=cam_a', { waitUntil: 'domcontentloaded' });

    const cellA = page.locator('.video-cell[data-stream-name="cam_a"]').first();
    await expect(cellA).toBeVisible({ timeout: 15000 });

    // 1×1 layout always shows the main stream (full-screen/recording quality).
    await expect(cellA).toHaveAttribute('data-sub-stream', 'false');
  });
});
