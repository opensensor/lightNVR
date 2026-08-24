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

async function readTimelineRange(page: Page): Promise<{ start: number; end: number }> {
  return page.locator('.timeline-ruler').evaluate(element => ({
    start: Number(element.getAttribute('data-timeline-start-hour')),
    end: Number(element.getAttribute('data-timeline-end-hour'))
  }));
}

async function dispatchTouchGesture(
  page: Page,
  start: { x: number; y: number },
  moved: { x: number; y: number },
  finish: 'touchend' | 'touchcancel'
): Promise<{ start: number; end: number }> {
  return page.locator('#timeline-container').evaluate(async (element, gesture) => {
    const dispatch = (type: 'touchstart' | 'touchmove' | 'touchend' | 'touchcancel', point: { x: number; y: number }) => {
      const touch = new Touch({
        identifier: 1,
        target: element,
        clientX: point.x,
        clientY: point.y,
        screenX: point.x,
        screenY: point.y,
        radiusX: 2,
        radiusY: 2,
        force: 1
      });
      const active = type === 'touchstart' || type === 'touchmove';
      element.dispatchEvent(new TouchEvent(type, {
        bubbles: true,
        cancelable: true,
        touches: active ? [touch] : [],
        targetTouches: active ? [touch] : [],
        changedTouches: [touch]
      }));
    };

    dispatch('touchstart', gesture.start);
    await new Promise(resolve => setTimeout(resolve, 50));
    dispatch('touchmove', gesture.moved);
    // Let Preact commit the range produced directly by touchmove, but keep the
    // sample/release interval inside the launch-velocity window.
    await new Promise(resolve => requestAnimationFrame(() => resolve(undefined)));
    const ruler = element.querySelector('.timeline-ruler');
    const rangeAfterMove = {
      start: Number(ruler?.getAttribute('data-timeline-start-hour')),
      end: Number(ruler?.getAttribute('data-timeline-end-hour'))
    };
    dispatch(gesture.finish, gesture.moved);
    return rangeAfterMove;
  }, {
    start,
    moved,
    finish
  });
}

async function dispatchTwoFingerGesture(
  page: Page,
  starts: [{ x: number; y: number }, { x: number; y: number }],
  moves: [{ x: number; y: number }, { x: number; y: number }]
): Promise<void> {
  await page.locator('#timeline-container').evaluate(async (element, gesture) => {
    const makeTouch = (identifier: number, point: { x: number; y: number }) => new Touch({
      identifier,
      target: element,
      clientX: point.x,
      clientY: point.y,
      screenX: point.x,
      screenY: point.y,
      radiusX: 2,
      radiusY: 2,
      force: 1
    });
    const dispatch = (type: 'touchstart' | 'touchmove' | 'touchend', points: Array<{ x: number; y: number }>) => {
      const touches = points.map((point, index) => makeTouch(index + 1, point));
      element.dispatchEvent(new TouchEvent(type, {
        bubbles: true,
        cancelable: true,
        touches: type === 'touchend' ? [] : touches,
        targetTouches: type === 'touchend' ? [] : touches,
        changedTouches: touches
      }));
    };

    dispatch('touchstart', gesture.starts);
    dispatch('touchmove', gesture.moves);
    await new Promise(resolve => requestAnimationFrame(() => resolve(undefined)));
    dispatch('touchend', gesture.moves);
  }, { starts, moves });
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
    await expect(timelinePage.tagsRecordingButton).toHaveAttribute('aria-label', 'Manage Recording Tags (1)');
    await expect(timelinePage.protectRecordingButton).toBeVisible();
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
    await expect(timelinePage.tagsRecordingButton).toHaveAttribute('aria-label', 'Manage Recording Tags (1)');
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

  test('jumps sequentially between recordings using previous and next recording buttons', async ({ page }) => {
    const stream = 'front_door';
    const date = '2026-03-08';
    const segments: Segment[] = [
      { id: 311, stream, start_timestamp: localTimestamp(date, '11:00:00'), end_timestamp: localTimestamp(date, '11:05:00') },
      { id: 312, stream, start_timestamp: localTimestamp(date, '11:10:00'), end_timestamp: localTimestamp(date, '11:15:00') },
      { id: 313, stream, start_timestamp: localTimestamp(date, '11:20:00'), end_timestamp: localTimestamp(date, '11:25:00') }
    ];

    await mockTimelineApis(page, stream, segments);
    await page.goto(`/timeline.html?stream=${stream}&date=${date}&time=11:10:00`, { waitUntil: 'domcontentloaded' });

    const timelinePage = new TimelinePage(page);
    await expect(timelinePage.timelineContainer).toBeVisible();
    await expect(timelinePage.timeDisplay).toHaveText('front_door - 11:10:00');
    await expect(timelinePage.videoPlayer).toHaveAttribute('src', /\/api\/recordings\/play\/312(?:\?|$)/);

    await expect(timelinePage.protectRecordingButton).toBeVisible();
    await expect(timelinePage.tagsRecordingButton).toBeVisible();
    await expect(timelinePage.previousRecordingButton).toBeEnabled();
    await expect(timelinePage.nextRecordingButton).toBeEnabled();

    await timelinePage.previousRecordingButton.click();
    await expect(timelinePage.timeDisplay).toHaveText('front_door - 11:00:00');
    await expect(timelinePage.videoPlayer).toHaveAttribute('src', /\/api\/recordings\/play\/311(?:\?|$)/);
    await expect(timelinePage.previousRecordingButton).toBeDisabled();

    await timelinePage.nextRecordingButton.click();
    await expect(timelinePage.timeDisplay).toHaveText('front_door - 11:10:00');
    await expect(timelinePage.videoPlayer).toHaveAttribute('src', /\/api\/recordings\/play\/312(?:\?|$)/);

    await timelinePage.nextRecordingButton.click();
    await expect(timelinePage.timeDisplay).toHaveText('front_door - 11:20:00');
    await expect(timelinePage.videoPlayer).toHaveAttribute('src', /\/api\/recordings\/play\/313(?:\?|$)/);
    await expect(timelinePage.nextRecordingButton).toBeDisabled();
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
    expect(playheadBox.width).toBeGreaterThanOrEqual(36);
    expect(playheadBox.height).toBeGreaterThanOrEqual(36);

    await page.mouse.move(playheadBox.x + (playheadBox.width / 2), playheadBox.y + (playheadBox.height / 2));
    await page.mouse.down();
    await page.mouse.move(playheadBox.x + 80, playheadBox.y + (playheadBox.height / 2), { steps: 4 });

    await expect(timelinePage.timeDisplay).toContainText(`${stream} -`);

    await page.mouse.up();
    await expect(timelinePage.timeDisplay).toContainText(`${stream} -`);
  });

  test('rebinds gestures and clears cursor locks when loading remounts the same-size timeline', async ({ page }) => {
    const date = '2026-03-08';
    const segments: Segment[] = [
      { id: 811, stream: 'front_door', start_timestamp: localTimestamp(date, '09:00:00'), end_timestamp: localTimestamp(date, '10:00:00') }
    ];

    await mockTimelineApis(page, 'front_door', segments);
    await page.route('**/api/streams', route => route.fulfill({ json: [
      { name: 'front_door' },
      { name: 'garage' }
    ] }));
    await page.route('**/api/timeline/segments?**', async route => {
      const stream = new URL(route.request().url()).searchParams.get('stream');
      if (stream === 'garage') {
        await new Promise(resolve => setTimeout(resolve, 150));
      }
      await route.fulfill({ json: { segments: segments.map(segment => ({ ...segment, stream: stream || segment.stream })) } });
    });

    await page.goto(`/timeline.html?stream=front_door&date=${date}&time=09:10:00`, { waitUntil: 'domcontentloaded' });
    const timelinePage = new TimelinePage(page);
    await expect(timelinePage.timelineContainer).toBeVisible();

    const oldContainer = await timelinePage.timelineContainer.elementHandle();
    const oldPlayheadBox = await timelinePage.playhead.boundingBox();
    if (!oldContainer || !oldPlayheadBox) {
      throw new Error('Expected mounted timeline and playhead before remount');
    }

    // Keep a cursor drag active while the loading branch removes the timeline.
    await page.mouse.move(
      oldPlayheadBox.x + (oldPlayheadBox.width / 2),
      oldPlayheadBox.y + (oldPlayheadBox.height / 2)
    );
    await page.mouse.down();
    await page.locator('#stream-selector').selectOption('garage');
    await expect(page.getByText('Loading timeline data...')).toBeVisible();
    await expect(timelinePage.timelineContainer).toBeVisible();
    expect(await oldContainer.evaluate(element => element.isConnected)).toBe(false);
    await page.mouse.up();

    // If unmount cleanup left userControllingCursor latched, this seek would
    // update global time but the newly-mounted playhead would stay in place.
    const beforeSeek = await timelinePage.playhead.boundingBox();
    const track = page.locator('.timeline-segments');
    const trackBox = await track.boundingBox();
    if (!beforeSeek || !trackBox) {
      throw new Error('Expected remounted timeline geometry');
    }
    await track.click({ position: { x: trackBox.width * 0.8, y: trackBox.height / 2 } });
    await expect.poll(async () => (await timelinePage.playhead.boundingBox())?.x ?? 0)
      .toBeGreaterThan(beforeSeek.x + 20);

    // Wheel zoom must be owned by the new node rather than the detached one.
    const beforeWheel = await readTimelineRange(page);
    const containerBox = await timelinePage.timelineContainer.boundingBox();
    if (!containerBox) {
      throw new Error('Expected remounted timeline bounding box');
    }
    await timelinePage.timelineContainer.dispatchEvent('wheel', {
      clientX: containerBox.x + (containerBox.width / 2),
      clientY: containerBox.y + 10,
      deltaY: -100,
      ctrlKey: true
    });
    await expect.poll(async () => {
      const range = await readTimelineRange(page);
      return range.end - range.start;
    }).toBeLessThan(beforeWheel.end - beforeWheel.start);
  });

  test('handles browser touch fling and cancel while honoring the explicit reduced-motion override', async ({ page }) => {
    const stream = 'mobile_cam';
    const date = '2026-03-08';
    const segments: Segment[] = [
      { id: 821, stream, start_timestamp: localTimestamp(date, '09:00:00'), end_timestamp: localTimestamp(date, '10:00:00') }
    ];

    await page.evaluate(() => localStorage.setItem('lightnvr.reduceMotion', 'off'));
    await mockTimelineApis(page, stream, segments);
    await page.goto(`/timeline.html?stream=${stream}&date=${date}&time=09:10:00`, { waitUntil: 'domcontentloaded' });
    const timelinePage = new TimelinePage(page);
    await expect(timelinePage.timelineContainer).toBeVisible();
    const box = await timelinePage.timelineContainer.boundingBox();
    if (!box) {
      throw new Error('Expected timeline geometry for touch test');
    }
    const start = { x: box.x + (box.width * 0.65), y: box.y + (box.height * 0.75) };
    const moved = { x: start.x - 90, y: start.y };

    // This project runs in a desktop Chromium context, where CDP touch input is
    // not delivered consistently. Constructed browser TouchEvents still cover
    // the component's real listener/state-machine path deterministically; the
    // physical-device acceptance pass remains a separate manual check.
    await timelinePage.timelineContainer.evaluate((element) => {
      ['touchstart', 'touchmove', 'touchend', 'touchcancel'].forEach(type => {
        element.addEventListener(type, () => {
          const key = `test-${type}`;
          element.setAttribute(key, String(Number(element.getAttribute(key) || '0') + 1));
        });
      });
    });

    const afterDrag = await dispatchTouchGesture(page, start, moved, 'touchend');
    await expect(timelinePage.timelineContainer).toHaveAttribute('test-touchmove', '1');
    await page.waitForTimeout(220);
    const afterFling = await readTimelineRange(page);
    expect(afterFling.start).toBeGreaterThan(afterDrag.start + 0.01);

    // A cancelled touch may pan directly, but must never launch inertia.
    await page.waitForTimeout(700);
    const cancelPoint = { x: start.x + 60, y: start.y };
    const beforeCancel = await dispatchTouchGesture(page, start, cancelPoint, 'touchcancel');
    await page.waitForTimeout(220);
    const afterCancel = await readTimelineRange(page);
    expect(afterCancel.start).toBeCloseTo(beforeCancel.start, 9);
    expect(afterCancel.end - afterCancel.start)
      .toBeCloseTo(beforeCancel.end - beforeCancel.start, 9);

    // LightNVR's explicit preference must win over the OS media query.
    await page.evaluate(() => localStorage.setItem('lightnvr.reduceMotion', 'on'));
    await page.reload({ waitUntil: 'domcontentloaded' });
    await expect(timelinePage.timelineContainer).toBeVisible();
    const reducedBox = await timelinePage.timelineContainer.boundingBox();
    if (!reducedBox) {
      throw new Error('Expected timeline geometry after reduced-motion reload');
    }
    const reducedStart = {
      x: reducedBox.x + (reducedBox.width * 0.65),
      y: reducedBox.y + (reducedBox.height * 0.75)
    };
    const reducedMoved = { x: reducedStart.x - 90, y: reducedStart.y };
    const reducedAfterDrag = await dispatchTouchGesture(page, reducedStart, reducedMoved, 'touchend');
    await page.waitForTimeout(220);
    const reducedAfterRelease = await readTimelineRange(page);
    expect(reducedAfterRelease.start).toBeCloseTo(reducedAfterDrag.start, 9);
    expect(reducedAfterRelease.end - reducedAfterRelease.start)
      .toBeCloseTo(reducedAfterDrag.end - reducedAfterDrag.start, 9);
  });

  test('handles browser two-finger pinch zoom and pan', async ({ page }) => {
    const stream = 'pinch_cam';
    const date = '2026-03-08';
    const segments: Segment[] = [
      { id: 825, stream, start_timestamp: localTimestamp(date, '11:00:00'), end_timestamp: localTimestamp(date, '13:00:00') }
    ];

    await mockTimelineApis(page, stream, segments);
    await page.goto(`/timeline.html?stream=${stream}&date=${date}&time=12:00:00`, { waitUntil: 'domcontentloaded' });
    const timelinePage = new TimelinePage(page);
    await expect(timelinePage.timelineContainer).toBeVisible();
    const box = await timelinePage.timelineContainer.boundingBox();
    if (!box) throw new Error('Expected timeline geometry for pinch test');

    const y = box.y + (box.height * 0.75);
    const center = box.x + (box.width / 2);
    const beforePinch = await readTimelineRange(page);
    await dispatchTwoFingerGesture(
      page,
      [{ x: center - 30, y }, { x: center + 30, y }],
      [{ x: center - 90, y }, { x: center + 90, y }]
    );
    const afterPinch = await readTimelineRange(page);
    expect(afterPinch.end - afterPinch.start).toBeLessThan(beforePinch.end - beforePinch.start);

    await dispatchTwoFingerGesture(
      page,
      [{ x: center - 60, y }, { x: center + 60, y }],
      [{ x: center - 10, y }, { x: center + 110, y }]
    );
    const afterPan = await readTimelineRange(page);
    expect(afterPan.start).toBeLessThan(afterPinch.start);
    expect(afterPan.end - afterPan.start).toBeCloseTo(afterPinch.end - afterPinch.start, 9);
  });

  test('keeps the accessible playhead target inside the day edge and preserves keyboard navigation', async ({ page }) => {
    const stream = 'edge_cam';
    const date = '2026-03-08';
    const segments: Segment[] = [
      { id: 831, stream, start_timestamp: localTimestamp(date, '23:40:00'), end_timestamp: localTimestamp(date, '23:45:00') },
      { id: 832, stream, start_timestamp: localTimestamp(date, '23:50:00'), end_timestamp: localTimestamp('2026-03-09', '00:02:00') }
    ];

    await mockTimelineApis(page, stream, segments);
    await page.goto(`/timeline.html?stream=${stream}&date=${date}&time=23:59:59`, { waitUntil: 'domcontentloaded' });
    const timelinePage = new TimelinePage(page);
    await expect(timelinePage.timelineContainer).toBeVisible();

    const slider = page.getByRole('slider');
    await expect(slider).toHaveAttribute('aria-orientation', 'horizontal');
    await expect(slider).toHaveAttribute('aria-valuetext', /edge_cam - 23:59:59/);
    await expect(slider).toHaveAttribute('tabindex', '0');

    const [containerBox, sliderBox] = await Promise.all([
      timelinePage.timelineContainer.boundingBox(),
      slider.boundingBox()
    ]);
    if (!containerBox || !sliderBox) {
      throw new Error('Expected accessible edge geometry');
    }
    expect(sliderBox.width).toBeGreaterThanOrEqual(36);
    expect(sliderBox.x).toBeGreaterThanOrEqual(containerBox.x);
    expect(sliderBox.x + sliderBox.width).toBeLessThanOrEqual(containerBox.x + containerBox.width + 0.5);

    // The clamped target is offset from the exact line near an edge. A grab
    // and release at its center must preserve the timestamp rather than seek
    // backward by half the hit-target width.
    const valueBeforeTap = await slider.getAttribute('aria-valuenow');
    await page.mouse.move(sliderBox.x + (sliderBox.width / 2), sliderBox.y + (sliderBox.height / 2));
    await page.mouse.down();
    await page.mouse.up();
    await expect(slider).toHaveAttribute('aria-valuenow', valueBeforeTap || '');

    // Zooming around the opposite edge pans the current timestamp out of the
    // viewport. All three cursor layers must disappear together.
    await timelinePage.timelineContainer.dispatchEvent('wheel', {
      clientX: containerBox.x + 1,
      clientY: containerBox.y + 10,
      deltaY: -100,
      ctrlKey: true
    });
    await expect(slider).toBeHidden();
    await expect(page.getByTestId('timeline-cursor-line')).toBeHidden();
    await expect(page.getByTestId('timeline-cursor-thumb')).toBeHidden();

    // Restore the full day before checking the existing keyboard behavior.
    await timelinePage.timelineContainer.dispatchEvent('wheel', {
      clientX: containerBox.x + 1,
      clientY: containerBox.y + 10,
      deltaY: 100,
      ctrlKey: true
    });
    await expect(slider).toBeVisible();
    await slider.focus();
    await page.keyboard.press('ArrowLeft');
    await expect(timelinePage.timeDisplay).toHaveText('edge_cam - 23:40:00');
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
