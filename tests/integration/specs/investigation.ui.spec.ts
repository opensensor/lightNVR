import { test, expect } from '@playwright/test';

import { USERS, login } from '../fixtures/test-fixtures';

/* One solid-blue 320x240 VP8 frame. Keeping the fixture inline makes the
 * metadata transition deterministic without depending on a camera or ffmpeg. */
const FOUR_BY_THREE_WEBM = Buffer.from(
  'GkXfo59ChoEBQveBAULygQRC84EIQoKEd2VibUKHgQJChYECGFOAZwEAAAAAAAKEEU2bdLpNu4tTq4QVSalmU6yBoU27i1OrhBZUrmtTrIHWTbuMU6uEElTDZ1OsggEkTbuMU6uEHFO7a1OsggJu7AEAAAAAAABZAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAVSalmsCrXsYMPQkBNgIxMYXZmNjIuMy4xMDBXQYxMYXZmNjIuMy4xMDBEiYhAj0AAAAAAABZUrmvJrgEAAAAAAABA14EBc8WItLrAEf7lCg2cgQAitZyDdW5kiIEAhoVWX1ZQOIOBASPjg4Q7msoA4JGwggFAuoHwmoECVbCEVbmBARJUw2f7c3OfY8CAZ8iZRaOHRU5DT0RFUkSHjExhdmY2Mi4zLjEwMHNz1mPAi2PFiLS6wBH+5QoNZ8ihRaOHRU5DT0RFUkSHlExhdmM2Mi4xMS4xMDAgbGlidnB4Z8ihRaOIRFVSQVRJT05Eh5MwMDowMDowMS4wMDAwMDAwMDAAH0O2dUDE54EAo0C+gQAAgNASAJ0BKkAB8AAARwiFhYiZhIgCAgJ1qgP4AgaGhajD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnD2TnDyAD+/00S//xKp/Eqn8Sqf/Eqn/0Yfww/hh/eEBxTu2uRu4+zgQC3iveBAfGCAaTwgQM=',
  'base64',
);

const CAMERA_UUID = '56856856-8568-4568-8568-568568568568';
const START_TIME = 1_787_750_400;
const END_TIME = START_TIME + 60;

test.describe('Investigation player regressions @ui @investigation', () => {
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test('keeps controls visible and preserves a 4:3 recording after metadata loads', async ({ page }) => {
    await page.route('**/api/streams?**', route => route.fulfill({
      json: {
        streams: [{
          camera_uuid: CAMERA_UUID,
          name: 'front-door',
          enabled: true,
          eptz_config: '',
        }],
        page: 1,
        total_pages: 1,
      },
    }));
    await page.route('**/api/investigations/timeline', route => route.fulfill({
      json: {
        start_time: START_TIME,
        end_time: END_TIME,
        max_active_decoders: 4,
        tracks: [{
          camera_uuid: CAMERA_UUID,
          name: 'front-door',
          segment_count: 1,
          truncated: false,
          aggregated: false,
          segments: [{
            id: 568,
            start_time: START_TIME,
            end_time: END_TIME,
            has_detection: true,
          }],
        }],
      },
    }));
    await page.route('**/api/investigations/search', route => route.fulfill({
      json: {
        results: [],
        page: { total: 0, has_more: false, next_cursor: null },
        facets: {
          event_types: [], locations: [], labels: [], zones: [], sources: [],
          capture_methods: [], recording_tags: [],
        },
        histogram: { buckets: [] },
        coverage: {
          unresolved_legacy_rows: 0,
          spatial_metadata: {
            requested: false, rows_with_boxes: 0, rows_without_boxes: 0,
          },
        },
      },
    }));
    await page.route('**/api/recordings/play/568*', route => route.fulfill({
      status: 200,
      contentType: 'video/webm',
      headers: { 'Accept-Ranges': 'bytes' },
      body: FOUR_BY_THREE_WEBM,
    }));

    await page.goto(
      `/investigation.html?cameras=${CAMERA_UUID}&start=${START_TIME}` +
      `&end=${END_TIME}&cursor=${START_TIME + 1}`,
      { waitUntil: 'domcontentloaded' },
    );

    const player = page.locator('.investigation-player').first();
    const frame = player.locator('.investigation-video-frame');
    const video = frame.locator('video');
    const controls = player.locator('.investigation-player-controls');

    await expect(video).toBeVisible();
    await expect.poll(() => video.evaluate(element => ({
      width: element.videoWidth,
      height: element.videoHeight,
      metadataLoaded: element.readyState >= HTMLMediaElement.HAVE_METADATA,
    }))).toEqual({ width: 320, height: 240, metadataLoaded: true });

    await expect(frame).toHaveAttribute(
      'style',
      /aspect-ratio:\s*320\s*\/\s*240/,
    );
    await expect.poll(() => frame.evaluate(element => {
      const bounds = element.getBoundingClientRect();
      return bounds.width / bounds.height;
    })).toBeCloseTo(4 / 3, 2);
    await expect.poll(() => video.evaluate(element => element.controls)).toBe(true);
    await expect.poll(() => video.evaluate(element =>
      getComputedStyle(element).objectFit)).toBe('contain');

    await expect(controls).toBeVisible();
    await expect(controls.getByRole('button', { name: 'Play' })).toBeVisible();
    await expect(controls.getByRole('slider')).toBeVisible();
    await expect(controls.getByRole('button', { name: 'Fullscreen' })).toBeVisible();
    await expect(page.getByTestId('fisheye-eptz-canvas')).toHaveCount(0);
  });
});
