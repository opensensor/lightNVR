/**
 * Building-plan layout editor: undo/redo reachability, direct edge resizing,
 * right-click menus and camera aiming, driven in a real browser.
 * Run with: npm --prefix web run test:e2e -- --testPathPatterns='planEditor\.browser\.spec\.js$'
 */
const path = require('node:path');
const { fleetLocations } = require('./fleetFixtures');

const fixturePlan = {
  uuid: 'plan-1', name: 'Orchard', revision: 1, canvas_width: 1200, canvas_height: 800,
  cameras: [], sketch: null, background_mime: null, updated_at: 1,
};
const fixtureStreams = [{ camera_uuid: 'cam-1', name: 'Front gate', availability: 'live' }];

async function openPlanBrowser() {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const webRoot = path.resolve(__dirname, '../..');
  const output = await build({
    stdin: {
      resolveDir: webRoot,
      loader: 'jsx',
      contents: `
  import { h, render } from 'preact';
  import { QueryClient, QueryClientProvider } from './js/query-client.ts';
  import { LiveBuildingPlan } from './js/components/preact/live/LiveBuildingPlan.tsx';
  const plan = ${JSON.stringify(fixturePlan)};
  window.calls = { puts: [], locations: [] };
  const json = (body) => new Response(JSON.stringify(body), { status: 200, headers: { 'Content-Type': 'application/json' } });
  window.fetch = async (url, options = {}) => {
    const target = String(url);
    if (target.startsWith('/api/live/plans/plan-1') && options.method === 'PUT') {
      const body = JSON.parse(options.body);
      window.calls.puts.push(body);
      return json({ ...plan, ...body, revision: plan.revision + 1 });
    }
    if (target.startsWith('/api/live/plans')) return json({ plans: [plan], can_modify: true });
    if (target === '/api/locations') return json(${JSON.stringify(fleetLocations([{ uuid: '00000000-0000-4000-8000-000000000001', name: 'Orchard' }]))});
    if (target.startsWith('/api/cameras/') && options.method === 'PUT') {
      window.calls.locations.push({ url: target, body: JSON.parse(options.body) });
      return json({});
    }
    return new Response('', { status: 404 });
  };
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
  window.mount = () => render(h(QueryClientProvider, { client },
    h(LiveBuildingPlan, { streams: ${JSON.stringify(fixtureStreams)}, selectedPlanUuid: 'plan-1',
      onSelectPlan: () => {}, onOpenCamera: () => {} })), document.getElementById('root'));
 `,
    },
    bundle: true,
    write: false,
    format: 'iife',
    jsx: 'automatic',
    jsxImportSource: 'preact',
    loader: { '.js': 'jsx' },
    // Same aliases Vite applies for the production bundle.
    alias: { react: 'preact/compat', 'react-dom': 'preact/compat' },
    plugins: [{
      name: 'fixture',
      setup(builder) {
        builder.onResolve({ filter: /(?:\/i18n|ToastContainer)\.tsx?$/ }, (args) => ({
          path: args.path, namespace: 'fixture',
        }));
        builder.onLoad({ filter: /.*/, namespace: 'fixture' }, () => ({
          contents: "export const t=(key)=>key;export const useI18n=()=>({t});export const showStatusMessage=()=>{};",
          loader: 'js',
        }));
      },
    }],
  });
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1400, height: 1000 } });
  page.setDefaultTimeout(5000);
  await page.route('http://fixture.invalid/**', (route) => route.fulfill({
    contentType: 'text/html', body: '<div id="root"></div>',
  }));
  await page.goto('http://fixture.invalid/');
  // Minimal layout so the SVG has a stable on-screen size.
  await page.addStyleTag({
    content: `.live-plan-workspace{display:flex}.live-plan-canvas-wrap{position:relative;width:900px;height:600px}
      .live-plan-canvas{width:900px;height:600px;display:block;touch-action:none}
      .live-plan-sketch-tools{position:static}
      .live-plan-sketch:not(.is-editing){pointer-events:none}
      .live-plan-shape-edge{stroke:transparent;fill:none;pointer-events:stroke}
      .live-plan-shape-hit{fill:none;stroke:transparent;stroke-width:26;pointer-events:stroke}
      .live-plan-shape-rect{fill:none;stroke:#888;stroke-width:5}.live-plan-shape-rect.is-filled{fill:#888;fill-opacity:.2}
      .live-plan-context-menu{position:fixed;background:#fff;border:1px solid #000;z-index:60}
      .live-plan-context-menu>button{display:block;width:100%}`,
  });
  await page.addScriptTag({ content: output.outputFiles[0].text });
  await page.evaluate(() => window.mount());
  await page.getByRole('button', { name: 'live.plan.editLayout', exact: true }).click();
  return { browser, page };
}

/** Plan fraction -> page coordinates (canvas is 1200x800 drawn into 900x600). */
async function planPoint(page, x, y) {
  const box = await page.locator('svg.live-plan-canvas').boundingBox();
  return { x: box.x + x * box.width, y: box.y + y * box.height };
}

async function drag(page, from, to) {
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  await page.mouse.move((from.x + to.x) / 2, (from.y + to.y) / 2, { steps: 4 });
  await page.mouse.move(to.x, to.y, { steps: 4 });
  await page.mouse.up();
}

/** Preact renders asynchronously after input; poll instead of reading once. */
async function eventually(read, check, timeout = 2000) {
  const deadline = Date.now() + timeout;
  let value = await read();
  while (!check(value) && Date.now() < deadline) {
    await new Promise((resolve) => setTimeout(resolve, 25));
    value = await read();
  }
  return value;
}
const TOOL_LABELS = { v: 'select', r: 'room', w: 'wall', a: 'area', l: 'label' };
/** Press a tool shortcut and wait until the toolbar shows it active. */
async function pressTool(page, key) {
  const button = page.locator(`.live-plan-sketch-tools button[title^="live.plan.sketch.tool.${TOOL_LABELS[key]} "]`);
  // The document key listener attaches in an effect right after entering
  // edit mode, so the very first press may need a retry.
  for (let attempt = 0; attempt < 5; attempt++) {
    await page.keyboard.press(key);
    if (await eventually(() => button.getAttribute('aria-pressed'),
      (value) => value === 'true', 300) === 'true') return;
  }
  throw new Error(`tool shortcut ${key} did not activate`);
}
const shapeCount = (page, expected) => eventually(
  () => page.locator('g.live-plan-shape').count(), (value) => value === expected);
const fieldNumber = (page, suffix, expected, digits = 0) => eventually(
  async () => Number(await page.locator(`input[id$="${suffix}"]`).inputValue()),
  (value) => Math.abs(value - expected) < 0.5 * 10 ** -digits);

it('edits a drawn room directly and undoes it even when focus is on body', async () => {
  const { browser, page } = await openPlanBrowser();
  try {
    await pressTool(page, 'r');
    await drag(page, await planPoint(page, 0.2, 0.2), await planPoint(page, 0.5, 0.5));
    expect(await shapeCount(page, 1)).toBe(1);
    // Drawing returns to Select with the new room selected.
    const selectButton = page.locator('.live-plan-sketch-tools button[title="live.plan.sketch.tool.select (V)"]');
    expect(await eventually(() => selectButton.getAttribute('aria-pressed'),
      (value) => value === 'true')).toBe('true');
    expect(await fieldNumber(page, '-sketch-w', 30)).toBeCloseTo(30, 0);

    // Drag the room's east edge (no handle needed) to widen it.
    await drag(page, await planPoint(page, 0.5, 0.35), await planPoint(page, 0.7, 0.35));
    expect(await fieldNumber(page, '-sketch-w', 50)).toBeCloseTo(50, 0);
    expect(await fieldNumber(page, '-sketch-x', 20)).toBeCloseTo(20, 0);

    // Undo works from <body> focus: it used to be swallowed there.
    await page.evaluate(() => document.activeElement?.blur());
    await page.keyboard.press('Control+z');
    expect(await fieldNumber(page, '-sketch-w', 30)).toBeCloseTo(30, 0);
    await page.keyboard.press('Control+z');
    expect(await shapeCount(page, 0)).toBe(0);
    await page.keyboard.press('Control+Shift+z');
    await page.keyboard.press('Control+Shift+z');
    expect(await shapeCount(page, 1)).toBe(1);

    // Drag the body to move it without switching tools first.
    await pressTool(page, 'r');
    await pressTool(page, 'v');
    await drag(page, await planPoint(page, 0.3, 0.3), await planPoint(page, 0.4, 0.4));
    expect(await fieldNumber(page, '-sketch-x', 30)).toBeCloseTo(30, 0);

    // Right-click opens the shape menu; Duplicate adds a copy.
    const center = await planPoint(page, 0.5, 0.45);
    await page.mouse.click(center.x, center.y, { button: 'right' });
    const menu = page.getByRole('menu', { name: 'live.plan.menu.label' });
    await menu.getByRole('menuitem', { name: /live\.plan\.sketch\.duplicate/ }).click();
    expect(await shapeCount(page, 2)).toBe(2);
    expect(await eventually(() => menu.count(), (value) => value === 0)).toBe(0);

    // Right-click mid-wall finishes it from the drawing menu.
    await pressTool(page, 'w');
    for (const [x, y] of [[0.1, 0.8], [0.4, 0.8], [0.4, 0.9]]) {
      const point = await planPoint(page, x, y);
      await page.mouse.click(point.x, point.y);
    }
    await page.keyboard.press('Backspace');
    const last = await planPoint(page, 0.3, 0.9);
    await page.mouse.click(last.x, last.y, { button: 'right' });
    await page.getByRole('menuitem', { name: /live\.plan\.menu\.finishDrawing/ }).click();
    expect(await shapeCount(page, 3)).toBe(3);
    const vertices = page.locator('g.live-plan-shape.is-wall.is-selected .live-plan-shape-handle.is-vertex');
    expect(await eventually(() => vertices.count(), (value) => value === 2)).toBe(2);
  } finally {
    await browser.close();
  }
}, 30000);

it('places a camera by pressing and aims it by dragging, then re-aims with the knob', async () => {
  const { browser, page } = await openPlanBrowser();
  try {
    await page.locator('.live-plan-unplaced button', { hasText: 'Front gate' }).click();
    // Drag from the placement point toward the lower right: 45°.
    const origin = await planPoint(page, 0.3, 0.3);
    await drag(page, origin, { x: origin.x + 120, y: origin.y + 120 });
    expect(await fieldNumber(page, '-rotation-number', 45)).toBe(45);
    expect(await eventually(() => page.locator('.live-plan-placement-hint').count(),
      (value) => value === 0)).toBe(0);

    // The aim knob on the selected camera re-aims it straight down (90°).
    const knob = await page.locator('.live-plan-aim-handle').boundingBox();
    await drag(page, { x: knob.x + knob.width / 2, y: knob.y + knob.height / 2 },
      { x: origin.x, y: origin.y + 200 });
    expect(await fieldNumber(page, '-rotation-number', 90)).toBe(90);

    // Keyboard rotation and undo cover cameras too.
    await page.evaluate(() => document.activeElement?.blur());
    await page.keyboard.press(']');
    expect(await fieldNumber(page, '-rotation-number', 105)).toBe(105);
    await page.keyboard.press('Control+z');
    expect(await fieldNumber(page, '-rotation-number', 90)).toBe(90);

    await page.getByRole('button', { name: 'live.plan.save', exact: true }).click();
    await page.waitForFunction(() => window.calls.puts.length === 1);
    const [body] = await page.evaluate(() => window.calls.puts);
    expect(body.cameras).toEqual([expect.objectContaining({
      camera_uuid: 'cam-1', rotation: 90, x: expect.closeTo(0.3, 1), y: expect.closeTo(0.3, 1),
    })]);
  } finally {
    await browser.close();
  }
}, 30000);

const rectXs = (page) => page.locator('g.live-plan-shape.is-rect rect.live-plan-shape-rect')
  .evaluateAll((rects) => rects.map((rect) => Math.round(Number(rect.getAttribute('x')))));
const selectedCount = (page, expected) => eventually(
  () => page.locator('g.live-plan-shape.is-selected').count(), (value) => value === expected);

it('box-selects, shift-clicks and moves several shapes together', async () => {
  const { browser, page } = await openPlanBrowser();
  try {
    await pressTool(page, 'r');
    await drag(page, await planPoint(page, 0.1, 0.1), await planPoint(page, 0.25, 0.25));
    await shapeCount(page, 1);
    await pressTool(page, 'r');
    await drag(page, await planPoint(page, 0.3, 0.1), await planPoint(page, 0.45, 0.25));
    expect(await shapeCount(page, 2)).toBe(2);
    await page.keyboard.press('Escape');
    expect(await selectedCount(page, 0)).toBe(0);

    // Dragging from empty canvas draws a marquee around both rooms.
    await drag(page, await planPoint(page, 0.05, 0.05), await planPoint(page, 0.5, 0.3));
    expect(await selectedCount(page, 2)).toBe(2);
    expect(await page.getByText('live.plan.sketch.multiSelected').count()).toBe(1);

    // Dragging either one moves the whole group by the same amount.
    const before = await rectXs(page);
    await drag(page, await planPoint(page, 0.175, 0.175), await planPoint(page, 0.275, 0.175));
    const after = await eventually(() => rectXs(page), (xs) => xs[0] !== before[0]);
    expect(after.map((x, index) => x - before[index])).toEqual([120, 120]);
    expect(await selectedCount(page, 2)).toBe(2);

    // Plain click narrows to one; Shift-click adds the other back.
    const first = await planPoint(page, 0.275, 0.175);
    await page.mouse.click(first.x, first.y);
    expect(await selectedCount(page, 1)).toBe(1);
    const second = await planPoint(page, 0.475, 0.175);
    await page.keyboard.down('Shift');
    await page.mouse.click(second.x, second.y);
    await page.keyboard.up('Shift');
    expect(await selectedCount(page, 2)).toBe(2);

    // Group delete is one undo step; Ctrl+A selects everything.
    await page.keyboard.press('Delete');
    expect(await shapeCount(page, 0)).toBe(0);
    await page.keyboard.press('Control+z');
    expect(await shapeCount(page, 2)).toBe(2);
    await page.keyboard.press('Escape');
    await page.keyboard.press('Control+a');
    expect(await selectedCount(page, 2)).toBe(2);

    // Group right-click menu.
    const center = await planPoint(page, 0.275, 0.175);
    await page.mouse.click(center.x, center.y, { button: 'right' });
    await page.getByRole('menuitem', { name: /live\.plan\.sketch\.duplicate/ }).click();
    expect(await shapeCount(page, 4)).toBe(4);
    expect(await selectedCount(page, 2)).toBe(2);
  } finally {
    await browser.close();
  }
}, 30000);

it('widens the field of view from the wedge handle and links the plan to a location', async () => {
  const { browser, page } = await openPlanBrowser();
  try {
    await page.locator('.live-plan-unplaced button', { hasText: 'Front gate' }).click();
    const origin = await planPoint(page, 0.4, 0.4);
    await page.mouse.click(origin.x, origin.y);
    expect(await fieldNumber(page, '-fov-number', 65)).toBe(65);

    // Drag the lower wedge-edge handle to 60° off the heading: 120° FOV.
    const handles = page.locator('.live-plan-fov-handle');
    expect(await eventually(() => handles.count(), (value) => value === 2)).toBe(2);
    const handle = await handles.nth(1).boundingBox();
    // Read the camera's on-screen center now: hints above the canvas can
    // shift it while placement is armed.
    const ring = await page.locator('.live-plan-camera-ring').boundingBox();
    const center = { x: ring.x + ring.width / 2, y: ring.y + ring.height / 2 };
    const radius = 150;
    await drag(page, { x: handle.x + handle.width / 2, y: handle.y + handle.height / 2 }, {
      x: center.x + radius * Math.cos(Math.PI / 3), y: center.y + radius * Math.sin(Math.PI / 3),
    });
    expect(await fieldNumber(page, '-fov-number', 120)).toBe(120);
    await page.evaluate(() => document.activeElement?.blur());
    await page.keyboard.press('=');
    expect(await fieldNumber(page, '-fov-number', 125)).toBe(125);

    await page.locator('.live-plan-location-field select').selectOption('00000000-0000-4000-8000-000000000001');
    await page.getByRole('button', { name: 'live.plan.save', exact: true }).click();
    await page.waitForFunction(() => window.calls.puts.length === 1);
    const [body] = await page.evaluate(() => window.calls.puts);
    expect(body.location_uuid).toBe('00000000-0000-4000-8000-000000000001');
    expect(body.cameras[0]).toMatchObject({ camera_uuid: 'cam-1', fov: 125 });

    // Back in view mode, offer to put the plan's cameras in that location.
    const assign = page.getByRole('button', { name: 'live.plan.assignLocation', exact: true });
    await assign.click();
    await page.waitForFunction(() => window.calls.locations.length === 1);
    expect(await page.evaluate(() => window.calls.locations)).toEqual([{
      url: '/api/cameras/cam-1/location', body: { location_uuid: '00000000-0000-4000-8000-000000000001' },
    }]);
  } finally {
    await browser.close();
  }
}, 30000);

/** Browser-initiated cancel (touch scroll/pinch, OS gesture) mid-drag. */
async function cancelPointer(page) {
  await page.evaluate(() => {
    document.querySelector('svg.live-plan-canvas').dispatchEvent(new PointerEvent('pointercancel', {
      pointerId: 1, pointerType: 'mouse', bubbles: true, cancelable: false,
    }));
  });
}

it('rolls back a cancelled drag or camera placement without an undo step', async () => {
  const { browser, page } = await openPlanBrowser();
  try {
    const errors = [];
    page.on('pageerror', (error) => errors.push(error.message));
    await pressTool(page, 'r');
    await drag(page, await planPoint(page, 0.2, 0.2), await planPoint(page, 0.4, 0.4));
    expect(await shapeCount(page, 1)).toBe(1);
    expect(await fieldNumber(page, '-sketch-x', 20)).toBeCloseTo(20, 0);

    // Start moving the room, then the browser cancels the pointer.
    const from = await planPoint(page, 0.3, 0.3);
    await page.mouse.move(from.x, from.y);
    await page.mouse.down();
    await page.mouse.move(from.x + 90, from.y, { steps: 4 });
    expect(await fieldNumber(page, '-sketch-x', 30)).toBeCloseTo(30, 0);
    await cancelPointer(page);
    await page.mouse.up();
    expect(await fieldNumber(page, '-sketch-x', 20)).toBeCloseTo(20, 0);
    // Only the room creation is on the undo stack.
    await page.evaluate(() => document.activeElement?.blur());
    await page.keyboard.press('Control+z');
    expect(await shapeCount(page, 0)).toBe(0);

    // Arming a camera adds it at the centre; the press then moves it to the
    // pointer. A cancelled press puts it back and leaves placement armed.
    await page.locator('.live-plan-unplaced button', { hasText: 'Front gate' }).click();
    const marker = page.locator('g.live-plan-camera');
    const armed = await eventually(() => marker.getAttribute('transform').catch(() => null),
      (value) => Boolean(value));
    const spot = await planPoint(page, 0.2, 0.8);
    await page.mouse.move(spot.x, spot.y);
    await page.mouse.down();
    expect(await eventually(() => marker.getAttribute('transform'),
      (value) => value !== armed)).not.toBe(armed);
    await cancelPointer(page);
    await page.mouse.up();
    expect(await eventually(() => marker.getAttribute('transform'),
      (value) => value === armed)).toBe(armed);
    expect(await page.locator('svg.live-plan-canvas.is-placing').count()).toBe(1);
    expect(errors).toEqual([]);
  } finally {
    await browser.close();
  }
}, 30000);

it('activates camera markers from the keyboard without blocking edit shortcuts', async () => {
  const { browser, page } = await openPlanBrowser();
  try {
    await page.locator('.live-plan-unplaced button', { hasText: 'Front gate' }).click();
    const origin = await planPoint(page, 0.4, 0.4);
    await page.mouse.click(origin.x, origin.y);
    const marker = page.locator('g.live-plan-camera');
    expect(await eventually(() => marker.count(), (value) => value === 1)).toBe(1);
    await page.keyboard.press('Escape');
    expect(await eventually(() => page.locator('g.live-plan-camera.is-selected').count(),
      (value) => value === 0)).toBe(0);

    // Enter on the focused marker selects it, like a click.
    await marker.focus();
    await page.keyboard.press('Enter');
    expect(await eventually(() => page.locator('g.live-plan-camera.is-selected').count(),
      (value) => value === 1)).toBe(1);

    // Space with the marker focused still arms hold-to-pan in edit mode.
    await marker.focus();
    await page.keyboard.down(' ');
    expect(await eventually(() => page.locator('svg.live-plan-canvas.is-pan-ready').count(),
      (value) => value === 1)).toBe(1);
    await page.keyboard.up(' ');
  } finally {
    await browser.close();
  }
}, 30000);
