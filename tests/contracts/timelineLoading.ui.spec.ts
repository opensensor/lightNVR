import {test, expect} from './permissionCases';

test('timeline waits for initial load and never overlaps slow polls', async ({server, browser}) => {
  const context = await server.browserContext(browser);
  const page = await context.newPage();
  let releaseInitial!: () => void;
  let releasePoll!: () => void;
  const initial = new Promise<void>(resolve => { releaseInitial = resolve; });
  const poll = new Promise<void>(resolve => { releasePoll = resolve; });
  let requests = 0;
  try {
    await page.clock.install();
    await page.route('**/api/timeline/segments?*', async route => {
      requests++;
      await (requests === 1 ? initial : poll);
      await route.fulfill({json: {segments: [], detection_intervals: []}});
    });
    await page.goto(`${server.baseURL}/timeline.html?stream=A&date=2026-11-07`);
    await expect.poll(() => requests).toBe(1);
    await expect(page.getByText('Loading timeline data...', {exact: true})).toBeVisible();
    await page.clock.runFor(20000);
    expect(requests).toBe(1);
    releaseInitial();
    await expect(page.getByText('Loading timeline data...', {exact: true})).toBeHidden();
    await page.clock.runFor(11000);
    await expect.poll(() => requests).toBe(2);
    await page.clock.runFor(20000);
    expect(requests).toBe(2);
    releasePoll();
  } finally {
    releaseInitial(); releasePoll();
    await context.close();
  }
});

test('failed timeline load exits spinner and recovers through Retry', async ({server, browser}) => {
  const context = await server.browserContext(browser);
  const page = await context.newPage();
  let requests = 0;
  try {
    await page.route('**/api/timeline/segments?*', route => {
      requests++;
      return requests === 1
        ? route.fulfill({status: 500, json: {error: 'Timeline query failed'}})
        : route.fulfill({json: {segments: [], detection_intervals: []}});
    });
    await page.goto(`${server.baseURL}/timeline.html?stream=A&date=2026-11-07`);
    const error = page.locator('.timeline-page [role="alert"]');
    await expect(error).toContainText('Error loading timeline data');
    await expect(page.getByText('Loading timeline data...', {exact: true})).toBeHidden();
    expect(requests).toBe(1);
    await error.getByRole('button', {name: 'Retry', exact: true}).click();
    await expect(error).toBeHidden();
    expect(requests).toBe(2);
  } finally {
    await context.close();
  }
});

test('a timed-out initial timeline request does not start retries or background polls', async ({server, browser}) => {
  const context = await server.browserContext(browser);
  const page = await context.newPage();
  let release!: () => void;
  const pending = new Promise<void>(resolve => { release = resolve; });
  let requests = 0;
  try {
    await page.clock.install();
    await page.route('**/api/timeline/segments?*', async route => {
      requests++;
      await pending;
      await route.fulfill({json: {segments: [], detection_intervals: []}});
    });
    await page.goto(`${server.baseURL}/timeline.html?stream=A&date=2026-11-07`);
    await expect.poll(() => requests).toBe(1);
    await page.clock.runFor(31000);
    await expect(page.locator('.timeline-page [role="alert"]')).toBeVisible();
    await expect(page.getByText('Loading timeline data...', {exact: true})).toBeHidden();
    await page.clock.runFor(60000);
    expect(requests).toBe(1);
  } finally {
    release();
    await context.close();
  }
});
