import { test, expect, Page } from '@playwright/test';
import { login, USERS } from '../fixtures/test-fixtures';
import { TimelinePage } from '../pages/TimelinePage';

type Segment = { id: number; stream: string; start_timestamp: number; end_timestamp: number };

function localTimestamp(date: string, time: string): number {
  const [y, m, d] = date.split('-').map(Number);
  const [hh, mm, ss] = time.split(':').map(Number);
  return Math.floor(new Date(y, m - 1, d, hh, mm, ss).getTime() / 1000);
}

function utcTimestamp(isoString: string): number {
  return Math.floor(Date.parse(isoString) / 1000);
}

async function mockTimelineApis(page: Page, stream: string, segments: Segment[], tagsById: Record<number, string[]> = {}) {
  await page.route('**/api/streams', route => route.fulfill({ json: [{ name: stream }] }));
  await page.route('**/api/timeline/segments?**', route => route.fulfill({ json: { segments } }));
  await page.route('**/api/timeline/segments-by-ids?**', route => route.fulfill({ json: {
    segments,
    start_time: new Date(segments[0].start_timestamp * 1000).toISOString(),
    end_time: new Date(segments[segments.length - 1].end_timestamp * 1000).toISOString(),
    multi_stream: new Set(segments.map(segment => segment.stream)).size > 1
  } }));
  await page.route('**/api/detection/results/**', route => route.fulfill({ json: { detections: [] } }));
  await page.route('**/api/recordings/**', async route => {
    const pathname = new URL(route.request().url()).pathname;
    const play = pathname.match(/\/api\/recordings\/play\/(\d+)$/);
    if (play) return route.fulfill({ status: 204, body: '' });

    const tags = pathname.match(/\/api\/recordings\/(\d+)\/tags$/);
    if (tags) return route.fulfill({ json: { tags: tagsById[Number(tags[1])] || [] } });

    const info = pathname.match(/\/api\/recordings\/(\d+)$/);
    if (info) {
      const segment = segments.find(s => s.id === Number(info[1]));
      if (segment) {
        return route.fulfill({ json: {
          id: segment.id,
          stream: segment.stream,
          start_time: new Date(segment.start_timestamp * 1000).toISOString(),
          end_time: new Date(segment.end_timestamp * 1000).toISOString(),
          protected: false
        } });
      }
    }

    return route.continue();
  });
}

test.describe('Timeline boundary flows @ui @timeline', () => {
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test('loads the intended recording from Recordings view at a boundary timestamp', async ({ page }) => {
    const stream = 'front_door';
    const date = '2026-03-08';
    const segments: Segment[] = [
      { id: 101, stream, start_timestamp: localTimestamp(date, '12:00:00'), end_timestamp: localTimestamp(date, '12:10:00') },
      { id: 102, stream, start_timestamp: localTimestamp(date, '12:10:00'), end_timestamp: localTimestamp(date, '12:20:00') }
    ];

    await page.addInitScript(() => localStorage.setItem('recordings_view_mode', 'table'));
    await mockTimelineApis(page, stream, segments, { 102: ['person'] });
    await page.route('**/api/recordings?**', route => route.fulfill({ json: {
      recordings: [{ id: 102, stream, start_time_unix: segments[1].start_timestamp, duration: 600, size: '1 MB', protected: false, tags: [] }],
      pagination: { total: 1, pages: 1, limit: 20 }
    } }));

    await page.goto('/recordings.html', { waitUntil: 'domcontentloaded' });
    const link = page.locator('a[title="View in Timeline"]').first();
    await expect(link).toBeVisible();

    const href = await link.getAttribute('href');
    if (!href) {
      throw new Error('Expected "View in Timeline" link to have an href attribute');
    }
    const expectedTime = new URL(href, 'http://localhost').searchParams.get('time');
    if (expectedTime === null) {
      throw new Error('Expected "time" search parameter in timeline link href but none was found');
    }

    await Promise.all([
      page.waitForURL('**/timeline.html**'),
      link.click()
    ]);

    const timelinePage = new TimelinePage(page);
    await expect(timelinePage.timelineContainer).toBeVisible();
    await expect(timelinePage.timeDisplay).toHaveText(`${stream} - ${expectedTime}`);
    await expect(page.locator('button[title="Manage Recording Tags"]')).toContainText('Tags (1)');
    await expect(timelinePage.videoPlayer).toHaveAttribute('src', /\/api\/recordings\/play\/102(?:\?|$)/);
  });

  test('renders a recording that spans midnight on the selected day', async ({ page }) => {
    const stream = 'overnight_drive';
    const selectedDate = '2026-03-08';
    const segments: Segment[] = [
      { id: 201, stream, start_timestamp: localTimestamp('2026-03-07', '23:58:00'), end_timestamp: localTimestamp(selectedDate, '00:05:00') }
    ];

    await mockTimelineApis(page, stream, segments, { 201: ['night'] });
    await page.goto(`/timeline.html?stream=${stream}&date=${selectedDate}&time=00:02:00`, { waitUntil: 'domcontentloaded' });

    const timelinePage = new TimelinePage(page);
    const segmentBar = page.locator('#timeline-container .timeline-segment').first();

    await expect(timelinePage.timelineContainer).toBeVisible();
    await expect(page.locator('button[title="Manage Recording Tags"]')).toContainText('Tags (1)');
    await expect(timelinePage.videoPlayer).toHaveAttribute('src', /\/api\/recordings\/play\/201(?:\?|$)/);
    await expect(segmentBar).toBeVisible();
    expect((await segmentBar.boundingBox())?.width ?? 0).toBeGreaterThan(0);
  });

  test('falls back to the nearest recording when the requested time is in a gap', async ({ page }) => {
    const stream = 'garage';
    const date = '2026-03-08';
    const segments: Segment[] = [
      { id: 301, stream, start_timestamp: localTimestamp(date, '10:00:00'), end_timestamp: localTimestamp(date, '10:05:00') },
      { id: 302, stream, start_timestamp: localTimestamp(date, '10:10:00'), end_timestamp: localTimestamp(date, '10:15:00') }
    ];

    await mockTimelineApis(page, stream, segments);
    await page.goto(`/timeline.html?stream=${stream}&date=${date}&time=10:08:00`, { waitUntil: 'domcontentloaded' });

    const timelinePage = new TimelinePage(page);
    await expect(timelinePage.timelineContainer).toBeVisible();
    await expect(timelinePage.timeDisplay).toHaveText('garage - 10:10:00');
    await expect(timelinePage.videoPlayer).toHaveAttribute('src', /\/api\/recordings\/play\/302(?:\?|$)/);
  });

  test('loads selected recordings mode, reuses batch download modal, and restores selections when refining', async ({ page }) => {
    const stream = 'front_door';
    const date = '2026-03-08';
    const returnUrl = `/recordings.html?dateRange=custom&startDate=${date}&endDate=${date}&startTime=00:00&endTime=23:59&page=1`;
    const segments: Segment[] = [
      { id: 401, stream, start_timestamp: localTimestamp(date, '09:00:00'), end_timestamp: localTimestamp(date, '09:10:00') },
      { id: 402, stream: 'garage', start_timestamp: localTimestamp(date, '09:15:00'), end_timestamp: localTimestamp(date, '09:25:00') }
    ];

    await page.addInitScript(({ selectedIds, url }) => {
      localStorage.setItem('recordings_view_mode', 'table');
      sessionStorage.setItem('lightnvr_selected_recording_ids', JSON.stringify(selectedIds));
      sessionStorage.setItem('lightnvr_restore_recording_selection', 'true');
      sessionStorage.setItem('lightnvr_recordings_return_url', url);
    }, { selectedIds: ['401', '402'], url: returnUrl });

    await mockTimelineApis(page, stream, segments, { 401: ['vehicle'], 402: ['person'] });
    await page.route('**/api/recordings?**', route => route.fulfill({ json: {
      recordings: segments.map(segment => ({
        id: segment.id,
        stream: segment.stream,
        start_time_unix: segment.start_timestamp,
        duration: segment.end_timestamp - segment.start_timestamp,
        size: '1 MB',
        protected: false,
        tags: []
      })),
      pagination: { total: segments.length, pages: 1, limit: 20 }
    } }));

    await page.goto('/timeline.html?ids=401,402', { waitUntil: 'domcontentloaded' });

    const timelinePage = new TimelinePage(page);
    await expect(timelinePage.timelineContainer).toBeVisible();
    await expect(page.getByText('Loading selected recordings...')).toHaveCount(0);
    await expect(timelinePage.timeDisplay).toHaveText('front_door - 09:00:00');

    // Click near the left edge to keep the selected timestamp close to the
    // recording start while still exercising direct segment selection.
    await timelinePage.timelineSegments.nth(1).click({ position: { x: 3, y: 3 } });
    await expect(timelinePage.timeDisplay).toContainText('garage - 09:15:');
    await expect(timelinePage.videoPlayer).toHaveAttribute('src', /\/api\/recordings\/play\/402(?:\?|$)/);

    await page.getByRole('button', { name: '↓ Download All (2)' }).click();
    await expect(page.getByRole('heading', { name: 'Download Selected Recordings' })).toBeVisible();
    await page.getByRole('button', { name: 'Cancel' }).click();

    await Promise.all([
      page.waitForURL('**/recordings.html**'),
      page.getByRole('link', { name: '← Refine Selections' }).click()
    ]);

    await expect(page.getByRole('button', { name: '▶ Timeline (2)' })).toBeVisible();
  });

  test('uses container fullscreen so detection overlays remain part of the timeline player', async ({ page }) => {
    const stream = 'front_door';
    const date = '2026-03-08';
    const segments: Segment[] = [
      { id: 501, stream, start_timestamp: localTimestamp(date, '14:00:00'), end_timestamp: localTimestamp(date, '14:05:00') }
    ];

    await mockTimelineApis(page, stream, segments);
    await page.route('**/api/detection/results/**', route => route.fulfill({ json: { detections: [{
      timestamp: segments[0].start_timestamp + 1,
      x: 0.1,
      y: 0.1,
      width: 0.25,
      height: 0.25,
      label: 'person',
      confidence: 0.92
    }] } }));

    await page.goto(`/timeline.html?stream=${stream}&date=${date}&time=14:00:01`, { waitUntil: 'domcontentloaded' });

    const timelinePage = new TimelinePage(page);
    await expect(timelinePage.timelineContainer).toBeVisible();
    await expect.poll(() => timelinePage.videoPlayer.evaluate(video => {
      interface VideoWithControlsList extends HTMLVideoElement {
        controlsList?: { contains?(value: string): boolean };
      }

      if (!video) {
        return false;
      }

      const typedVideo = video as VideoWithControlsList;
      const controlsList = typedVideo.controlsList;
      if (!controlsList || typeof controlsList.contains !== 'function') {
        return false;
      }
      return controlsList.contains('nofullscreen');
    })).toBe(true);

    await page.locator('#timeline-detection-overlay').check();
    await expect(page.locator('[data-testid="timeline-video-container"] canvas')).toBeVisible();

    await timelinePage.fullscreenButton.click();

    await expect.poll(() => page.evaluate(() => document.fullscreenElement?.getAttribute('data-testid'))).toBe('timeline-video-container');
    await expect.poll(() => page.evaluate(() => {
      const fullscreenElement = document.fullscreenElement;
      return Boolean(fullscreenElement?.querySelector('video') && fullscreenElement?.querySelector('canvas'));
    })).toBe(true);

    await page.evaluate(() => document.exitFullscreen());
    await expect.poll(() => page.evaluate(() => document.fullscreenElement)).toBeNull();
  });

  test('keeps the stream name prefix while dragging the timeline playhead', async ({ page }) => {
    const stream = 'front_door';
    const date = '2026-03-08';
    const segments: Segment[] = [
      { id: 801, stream, start_timestamp: localTimestamp(date, '09:00:00'), end_timestamp: localTimestamp(date, '09:10:00') }
    ];

    await mockTimelineApis(page, stream, segments);
    await page.goto(`/timeline.html?stream=${stream}&date=${date}&time=09:00:00`, { waitUntil: 'domcontentloaded' });

    const timelinePage = new TimelinePage(page);
    await expect(timelinePage.timelineContainer).toBeVisible();
    await expect(timelinePage.timeDisplay).toContainText(`${stream} - 09:00:00`);

    const playheadBox = await timelinePage.playhead.boundingBox();
    if (!playheadBox) {
      throw new Error('Expected timeline playhead to have a bounding box');
    }

    await page.mouse.move(playheadBox.x + (playheadBox.width / 2), playheadBox.y + (playheadBox.height / 2));
    await page.mouse.down();
    await page.mouse.move(playheadBox.x + 80, playheadBox.y + (playheadBox.height / 2), { steps: 4 });

    await expect(timelinePage.timeDisplay).toContainText(`${stream} -`);

    await page.mouse.up();
    await expect(timelinePage.timeDisplay).toContainText(`${stream} -`);
  });
});

test.describe('Timeline DST rendering @ui @timeline', () => {
  test.use({ timezoneId: 'America/New_York' });

  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test('skips the nonexistent 2am ruler label on spring-forward days while preserving playback selection', async ({ page }) => {
    const stream = 'front_door';
    const selectedDate = '2026-03-08';
    // America/New_York spring-forward day:
    // 06:50:00Z = 01:50:00 EST, 06:55:00Z = 01:55:00 EST,
    // 07:10:00Z = 03:10:00 EDT, 07:15:00Z = 03:15:00 EDT.
    const preSpringForward0150EST = utcTimestamp('2026-03-08T06:50:00Z');
    const preSpringForward0155EST = utcTimestamp('2026-03-08T06:55:00Z');
    const postSpringForward0310EDT = utcTimestamp('2026-03-08T07:10:00Z');
    const postSpringForward0315EDT = utcTimestamp('2026-03-08T07:15:00Z');
    const segments: Segment[] = [
      { id: 601, stream, start_timestamp: preSpringForward0150EST, end_timestamp: preSpringForward0155EST },
      { id: 602, stream, start_timestamp: postSpringForward0310EDT, end_timestamp: postSpringForward0315EDT }
    ];

    await mockTimelineApis(page, stream, segments);
    await page.goto(`/timeline.html?stream=${stream}&date=${selectedDate}&time=03:10:00`, { waitUntil: 'domcontentloaded' });

    const timelinePage = new TimelinePage(page);
    const ruler = page.locator('.timeline-ruler');

    await expect(timelinePage.timelineContainer).toBeVisible();
    await expect(timelinePage.timeDisplay).toHaveText('front_door - 03:10:00');
    await expect(timelinePage.videoPlayer).toHaveAttribute('src', /\/api\/recordings\/play\/602(?:\?|$)/);
    await expect(ruler.getByText('3:00', { exact: true })).toBeVisible();
    await expect(ruler.getByText('2:00', { exact: true })).toHaveCount(0);
  });
});

test.describe('Timeline DST fall-back rendering @ui @timeline', () => {
  test.use({ timezoneId: 'Europe/Berlin' });

  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test('shows the repeated 2am hour twice on fall-back days', async ({ page }) => {
    const stream = 'front_door';
    const selectedDate = '2026-10-25';
    // Europe/Berlin fall-back day:
    // 00:10:00Z = first 02:10 local time (CEST), 01:10:00Z = second 02:10 local time (CET).
    const first0210Berlin = utcTimestamp('2026-10-25T00:10:00Z');
    const second0210Berlin = utcTimestamp('2026-10-25T01:10:00Z');
    const segmentDurationSeconds = 5 * 60;
    const segments: Segment[] = [
      { id: 701, stream, start_timestamp: first0210Berlin, end_timestamp: first0210Berlin + segmentDurationSeconds },
      { id: 702, stream, start_timestamp: second0210Berlin, end_timestamp: second0210Berlin + segmentDurationSeconds }
    ];

    await mockTimelineApis(page, stream, segments);
    await page.goto(`/timeline.html?stream=${stream}&date=${selectedDate}`, { waitUntil: 'domcontentloaded' });

    const timelinePage = new TimelinePage(page);
    const ruler = page.locator('.timeline-ruler');

    await expect(timelinePage.timelineContainer).toBeVisible();
    await expect(ruler.getByText('2:00', { exact: true })).toHaveCount(2);
    await expect(ruler.getByText('3:00', { exact: true })).toBeVisible();
  });
});