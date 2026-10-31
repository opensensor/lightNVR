/** Native sibling actions and presentation surfaces. Browser supplements use
 * actual Preact/components, with explicit external service/media fixtures. */
const mockHooks = { cursor: 0, values: {}, setters: [] };
jest.mock('preact/hooks', () => ({
  useState: initial => {
    const index = mockHooks.cursor++;
    const setter = jest.fn(); mockHooks.setters[index] = setter;
    return [Object.hasOwn(mockHooks.values, index) ? mockHooks.values[index] : typeof initial === 'function' ? initial() : initial, setter];
  },
  useEffect: jest.fn(), useCallback: callback => callback,
  useMemo: factory => factory(), useRef: current => ({ current }),
}));
jest.mock('preact/compat', () => ({ ...jest.requireActual('preact/compat'), createPortal: child => child }));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: key => `translated:${key}` }) }));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn(), useQuery: jest.fn(), useQueryClient: () => ({ invalidateQueries: jest.fn() }) }));
jest.mock('../../js/request-queue.js', () => ({ queueThumbnailLoad: jest.fn(async () => {}), invalidateThumbnailLoad: jest.fn(), Priority: { HIGH: 'high', LOW: 'low' } }));
jest.mock('../../js/components/preact/timeline/TimelinePage.jsx', () => ({ timelineState: {} }));
jest.mock('../../js/components/preact/UI.jsx', () => ({ ConfirmDialog: () => null }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
const { LiveOperatorNavigator } = require('../../js/components/preact/live/LiveOperatorNavigator.jsx');
const { RecordingsGrid } = require('../../js/components/preact/recordings/RecordingsGrid.jsx');
const { Go2rtcTab } = require('../../js/components/preact/settings/Go2rtcTab.jsx');
const { ClearLogsModal } = require('../../js/components/preact/system/ClearLogsModal.jsx');
const { RestartModal } = require('../../js/components/preact/system/RestartModal.jsx');
const { TimelinePlayer } = require('../../js/components/preact/timeline/TimelinePlayer.jsx');
const { useQuery } = require('../../js/query-client.js');
const { queueThumbnailLoad } = require('../../js/request-queue.js');
const savedGlobals = Object.fromEntries(['document', 'window', 'localStorage'].map(key => [key, Object.getOwnPropertyDescriptor(globalThis, key)]));
function nodes(node) {
  if (Array.isArray(node)) return node.flatMap(nodes);
  if (!node || typeof node !== 'object') return [];
  return [node, ...nodes(node.props?.children)];
}
function render(Component, props, values = {}) {
  mockHooks.cursor = 0; mockHooks.values = values; mockHooks.setters = [];
  return Component(props);
}
beforeEach(() => {
  jest.clearAllMocks();
  Object.defineProperty(globalThis, 'document', { configurable: true, value: { body: {} } });
  Object.defineProperty(globalThis, 'window', { configurable: true, value: { location: { search: '', href: 'http://fixture.invalid/' } } });
  Object.defineProperty(globalThis, 'localStorage', { configurable: true, value: { getItem: () => null } });
});
afterEach(() => {
  for (const [key, value] of Object.entries(savedGlobals)) {
    if (value) Object.defineProperty(globalThis, key, value); else delete globalThis[key];
  }
});
it.each([['clear', ClearLogsModal], ['restart', RestartModal]])('%s structural backdrop retains target-only dismissal without key activation', (_kind, Component) => {
  const onClose = jest.fn(); const onConfirm = jest.fn();
  const tree = render(Component, { isOpen: true, isRestarting: false, onClose, onConfirm });
  expect(tree.props.role).toBe('presentation');
  expect(tree.props.onKeyDown).toBeUndefined();
  const backdrop = {}; tree.ref.current = backdrop;
  tree.props.onClick({ target: {}, currentTarget: backdrop }); expect(onClose).not.toHaveBeenCalled();
  tree.props.onClick({ target: backdrop, currentTarget: backdrop }); expect(onClose).toHaveBeenCalledTimes(1);
});
it('restart backdrop remains blocked during restart and closed portals stay absent', () => {
  const onClose = jest.fn();
  const tree = render(RestartModal, { isOpen: true, isRestarting: true, onClose });
  const target = {}; tree.props.onClick({ target, currentTarget: target }); expect(onClose).not.toHaveBeenCalled();
  expect(render(RestartModal, { isOpen: false })).toBeNull();
  expect(render(ClearLogsModal, { isOpen: false })).toBeNull();
});
it('go2rtc preview uses target-only presentation instead of an inner stop-click handler', () => {
  const tree = render(Go2rtcTab, { settings: { go2rtcEnabled: true }, canModifySettings: true, t: key => `translated:${key}` }, { 3: true });
  const outer = nodes(tree).find(node => (node.props?.class || '').includes('fixed inset-0'));
  expect(outer.props.role).toBe('presentation');
  expect(outer.props.onKeyDown).toBeUndefined();
  expect(outer.props.children.props.onClick).toBeUndefined();
  outer.props.onClick({ target: {}, currentTarget: {}, stopPropagation: jest.fn() });
  expect(mockHooks.setters[3]).not.toHaveBeenCalled();
  const target = {}; outer.props.onClick({ target, currentTarget: target, stopPropagation: jest.fn() });
  expect(mockHooks.setters[3]).toHaveBeenCalledWith(false);
});
function recordingCard(loadState, inProgress = false) {
  const recording = { id: 7, stream: 'Fixture camera', duration: 5, in_progress: inProgress };
  const props = { recordings: [recording], selectedRecordings: {}, getSelectedCount: () => 0, pagination: { totalItems: 1 },
    playRecording: jest.fn(), toggleRecordingSelection: jest.fn(), canDelete: true, hiddenColumns: {}, thumbnailsPerRecording: 1 };
  const grid = render(RecordingsGrid, props, { 0: true });
  const card = nodes(grid).find(node => typeof node.type === 'function' && node.type.name === 'RecordingCard');
  return { tree: render(card.type, card.props, { 4: loadState }), props, recording };
}
it.each([['loaded', false], ['loading', false], ['loaded', true]])('thumbnail play is native and retains %s/in-progress=%s callback contract', (state, inProgress) => {
  const { tree, props, recording } = recordingCard(state, inProgress);
  const play = nodes(tree).find(node => node.type === 'button' && node.props['aria-label']?.startsWith('translated:recordings.play'));
  expect(play).toBeDefined(); expect(play.props.type).toBe('button'); expect(play.props.onKeyDown).toBeUndefined();
  expect(play.props.disabled).toBeFalsy();
  play.props.onClick(); expect(props.playRecording).toHaveBeenCalledTimes(1); expect(props.playRecording).toHaveBeenCalledWith(recording);
  const checkbox = nodes(tree).find(node => node.type === 'input' && node.props.type === 'checkbox');
  expect(nodes(play.props.children)).not.toContain(checkbox);
  checkbox.props.onChange(); expect(props.toggleRecordingSelection).toHaveBeenCalledWith(7);
  expect(props.playRecording).toHaveBeenCalledTimes(1);
});
it('retry is a separate native action and never activates playback', () => {
  const { tree, props } = recordingCard('error');
  const retry = nodes(tree).find(node => node.type === 'button' && node.props.title === 'translated:recordings.clickToRetry');
  expect(retry).toBeDefined(); expect(retry.props.type).toBe('button'); expect(retry.props.onKeyDown).toBeUndefined();
  retry.props.onClick(); expect(queueThumbnailLoad).toHaveBeenCalledTimes(1); expect(props.playRecording).not.toHaveBeenCalled();
  const selectionSurface = nodes(tree).find(node => (node.props?.class || '').includes('absolute top-2 left-2'));
  expect(selectionSurface.props.onClick).toBeUndefined();
});
it('fullscreen prompt has native enter and dismiss siblings without nested interactive descendants', () => {
  const tree = render(TimelinePlayer, { autoFullscreen: true });
  const enter = nodes(tree).find(node => node.type === 'button' && node.props['aria-label'] === 'translated:timeline.clickToEnterFullscreen');
  expect(enter).toBeDefined(); expect(enter.props.type).toBe('button'); expect(enter.props.onKeyDown).toBeUndefined();
  expect(nodes(enter.props.children).some(node => node.type === 'button')).toBe(false);
  const dismiss = nodes(tree).find(node => node.type === 'button' && node.props['aria-label'] === 'translated:timeline.exitFullscreen');
  dismiss.props.onClick({ stopPropagation: jest.fn() }); expect(mockHooks.setters[8]).toHaveBeenCalledWith(false);
  const video = nodes(tree).find(node => node.type === 'video'); expect(video.props.controls).toBe(true);
});
it('location expansion is a native sibling action, not an interactive descendant of selection', () => {
  const locations = [{ uuid: 'east', name: 'East', type: 'building' }, { uuid: 'child', name: 'Child', parent_uuid: 'east', type: 'floor' }];
  useQuery.mockImplementation(({ queryKey }) => ({ data: queryKey[0] === 'fleet-locations' ? { locations } : {}, isLoading: false }));
  const tree = render(LiveOperatorNavigator, { streams: [], orderedStreams: [], onFilterChange: jest.fn() });
  const expand = nodes(tree).find(node => node.type === 'button' && node.props['aria-label'] === 'East' && 'aria-expanded' in node.props);
  expect(expand).toBeDefined(); expect(expand.props.type).toBe('button'); expect(expand.props.onKeyDown).toBeUndefined();
  const containingButtons = nodes(tree).filter(node => node.type === 'button' && nodes(node.props.children).includes(expand));
  expect(containingButtons).toHaveLength(0);
  expand.props.onClick(); expect(mockHooks.setters[4]).toHaveBeenCalledTimes(1);
});

async function openBrowser() {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild'); const fs = require('node:fs'); const path = require('node:path');
  const webRoot = path.resolve(__dirname, '../..');
  const result = await build({
    stdin: { resolveDir: webRoot, loader: 'jsx', contents: `
      import { h, render } from 'preact'; import { useState } from 'preact/hooks';
      import { LiveOperatorNavigator } from './js/components/preact/live/LiveOperatorNavigator.jsx';
      import { RecordingsGrid } from './js/components/preact/recordings/RecordingsGrid.jsx';
      import { Go2rtcTab } from './js/components/preact/settings/Go2rtcTab.jsx';
      import { ClearLogsModal } from './js/components/preact/system/ClearLogsModal.jsx';
      import { RestartModal } from './js/components/preact/system/RestartModal.jsx';
      import { TimelinePlayer } from './js/components/preact/timeline/TimelinePlayer.jsx';
      const root = document.querySelector('#root');
      const translate = key => 'translated:' + key;
      const locations = [{ uuid: 'east', name: 'East', type: 'building' },
        { uuid: 'child', name: 'East child', type: 'floor', parent_uuid: 'east' },
        { uuid: 'west', name: 'West', type: 'building' }];
      const streams = [{ name: 'Fixture camera', camera_uuid: 'camera', location_uuid: 'child', availability: 'live' }];
      const queryData = { 'fleet-locations': { locations }, 'ui-workspaces': {}, 'live-layouts': { layouts: [] }, 'operator-floor-plans': { plans: [] } };
      window.fixtureQuery = ({ queryKey }) => ({ data: queryData[queryKey[0]] || {}, isLoading: false });
      window.fixtureJSON = async url => { window.calls.requests++; return url.endsWith('/effective-config')
        ? { base: 'synthetic: config', override: '', redaction_available: true } : {}; };
      window.thumbnail = async () => { window.calls.thumbnails++; if (window.failThumb) throw Error('fixture thumbnail unavailable'); };
      window.timelineFixture = { timelineSegments: [], currentSegmentIndex: -1, currentTime: null, playbackSpeed: 1, isPlaying: false,
        subscribe: () => () => {}, setState(change) { Object.assign(this, change); window.calls.timeline++; } };
      Element.prototype.requestFullscreen = async function() {
        window.calls.fullscreen++; window.calls.fullscreenTarget = this.dataset.testid;
        window.calls.userActivation = navigator.userActivation.isActive;
      };
      window.fetch = async () => ({ ok: false, json: async () => ({}) });
      const onFilterChange = value => { window.calls.filters++; window.calls.selected = value?.selectedLocation; };
      const onApplyLayout = value => { window.calls.layouts++; window.calls.layout = value; };
      function Host({ kind, options }) {
        const [selected, setSelected] = useState({});
        const [settings, setSettings] = useState({ go2rtcEnabled: true });
        const close = () => { window.calls.close++; render(null, root); };
        if (kind === 'live') return h(LiveOperatorNavigator, { streams, orderedStreams: streams, columns: 2, rows: 2,
          onFilterChange, onApplyLayout, onSurfaceChange: () => {}, onSelectPlan: () => {} });
        if (kind === 'grid') return h(RecordingsGrid, { recordings: [{ id: 7, stream: 'Fixture camera', start_time_unix: 1700000000, duration: 5, in_progress: options.inProgress }],
          selectedRecordings: selected, getSelectedCount: () => Object.values(selected).filter(Boolean).length,
          toggleRecordingSelection: id => { window.calls.select++; setSelected(current => ({ ...current, [id]: !current[id] })); },
          toggleSelectAll: () => {}, pagination: { totalItems: 1 }, playRecording: value => { window.calls.play++; window.calls.playId = value.id; },
          clearSelections: () => setSelected({}), canDelete: true, hiddenColumns: { actions: true, tags: true }, thumbnailsPerRecording: options.frames || 1 });
        if (kind === 'go2rtc') return h(Go2rtcTab, { settings, setSettings, handleInputChange: () => {}, canModifySettings: !options.readonly, t: translate });
        if (kind === 'timeline') return h(TimelinePlayer, { autoFullscreen: true });
        return h(kind === 'clear' ? ClearLogsModal : RestartModal, { isOpen: true, isRestarting: options.restarting,
          onClose: close, onConfirm: () => { window.calls.confirm++; return new Promise(resolve => { window.finishConfirm = resolve; }); } });
      }
      window.mount = (kind, options = {}) => {
        render(null, root); localStorage.clear();
        window.calls = { close: 0, confirm: 0, requests: 0, thumbnails: 0, play: 0, select: 0, filters: 0, layouts: 0, fullscreen: 0, timeline: 0 };
        window.failThumb = Boolean(options.failThumb); render(h(Host, { kind, options }), root);
      };
    ` },
    bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact',
    plugins: [{ name: 'test-service', setup(builder) {
      builder.onResolve({ filter: /(?:i18n|query-client|request-queue|ToastContainer|TimelinePage)\.(?:js|jsx)$/ }, args => ({ path: args.path, namespace: 'service' }));
      builder.onLoad({ filter: /.*/, namespace: 'service' }, args => ({ loader: 'js', contents: args.path.includes('TimelinePage')
        ? 'export const timelineState = new Proxy({}, { get: (_target, key) => window.timelineFixture[key], set: (_target, key, value) => (window.timelineFixture[key] = value, true) });'
        : `export const t = key => 'translated:' + key; export const useI18n = () => ({ t });
          export const fetchJSON = (url, options) => window.fixtureJSON(url, options);
          export const useQuery = options => window.fixtureQuery(options);
          export const useQueryClient = () => ({ invalidateQueries: () => {} }); export const useMutation = () => ({});
          export const queueThumbnailLoad = () => window.thumbnail(); export const invalidateThumbnailLoad = () => {};
          export const Priority = { HIGH: 'high', LOW: 'low' }; export const showStatusMessage = () => {};` }));
    } }],
  });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } }); page.setDefaultTimeout(5000);
    const errors = []; page.on('pageerror', error => errors.push(error.message));
    await page.route('http://fixture.invalid/**', route => route.fulfill({ contentType: route.request().url().includes('/thumbnail/') ? 'image/svg+xml' : 'text/html',
      body: route.request().url().includes('/thumbnail/') ? '<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect width="10" height="10" fill="green"/></svg>' : '<button id="launcher">Launch</button><div id="root"></div>' }));
    await page.goto('http://fixture.invalid/');
    const styles = new Set();
    for (const file of ['index.html', 'recordings.html', 'timeline.html', 'settings.html', 'system.html']) {
      const html = fs.readFileSync(path.join(webRoot, 'dist', file), 'utf8');
      for (const tag of html.match(/<link[^>]*rel="stylesheet"[^>]*>/g) || []) styles.add(tag.match(/href="([^"]+)"/)[1]);
    }
    expect(styles.size).toBeGreaterThan(0);
    for (const href of styles) await page.addStyleTag({ content: fs.readFileSync(path.join(webRoot, 'dist', href), 'utf8') });
    await page.addScriptTag({ content: result.outputFiles[0].text });
    return { browser, page, errors };
  } catch (error) { await browser.close(); throw error; }
}
const browserTest = process.env.T7_163_BROWSER === '1' ? it : it.skip;
async function flushEffects(page) { await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(resolve, 0))))); }
async function focusVisible(page, control) {
  await page.keyboard.press('Tab'); await control.focus();
  expect(await control.evaluate(element => { const style = getComputedStyle(element); return element.matches(':focus-visible')
    && ((style.outlineStyle !== 'none' && parseFloat(style.outlineWidth) > 0) || style.boxShadow !== 'none'); })).toBe(true);
}
browserTest('location expansion remains separate from selection, double-click layout and camera drag', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('live')); await flushEffects(page);
    const expand = page.getByRole('button', { name: 'East', exact: true });
    await expand.waitFor(); expect(await expand.getAttribute('aria-expanded')).toBe('true');
    await focusVisible(page, expand);
    const count = await page.evaluate(() => window.calls.filters);
    await expand.press('Enter'); expect(await expand.getAttribute('aria-expanded')).toBe('false');
    expect(await page.getByText('East child', { exact: true }).count()).toBe(0);
    await expand.press('Space'); expect(await expand.getAttribute('aria-expanded')).toBe('true');
    await expand.click(); expect(await expand.getAttribute('aria-expanded')).toBe('false');
    await expand.click(); expect(await expand.getAttribute('aria-expanded')).toBe('true');
    expect(await page.evaluate(() => window.calls.filters)).toBe(count);
    expect(await page.evaluate(() => window.calls.layouts)).toBe(0);
    const select = page.locator('.live-location-tree button.live-location-row').filter({ hasText: 'East' }).first();
    await select.press('Enter'); await page.waitForFunction(() => window.calls.selected === 'east');
    await select.dblclick(); expect(await page.evaluate(() => window.calls.layouts)).toBe(1);
    expect(await page.evaluate(() => window.calls.layout.cameraUuids)).toEqual(['camera']);
    const camera = page.locator('.live-camera-row').first(); expect(await camera.getAttribute('draggable')).toBe('true');
    const transfer = await page.evaluateHandle(() => new DataTransfer());
    await camera.dispatchEvent('dragstart', { dataTransfer: transfer });
    expect(await transfer.evaluate(value => value.types.length)).toBeGreaterThan(0);
    expect(await page.locator('button button').count()).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('thumbnail play, retry and selection dispatch independently by keyboard and pointer', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('grid'));
    const play = page.getByRole('button', { name: 'translated:recordings.play Fixture camera', exact: true });
    await play.waitFor(); await focusVisible(page, play);
    await play.press('Enter'); await play.press('Space'); await play.click();
    expect(await page.evaluate(() => window.calls.play)).toBe(3);
    expect(await page.evaluate(() => window.calls.playId)).toBe(7);
    await page.getByRole('button', { name: 'translated:streams.select', exact: true }).click();
    const select = page.getByRole('checkbox', { name: 'translated:recordings.selectRecording', exact: true });
    await select.press('Space'); expect(await select.isChecked()).toBe(true);
    await select.click(); expect(await select.isChecked()).toBe(false);
    expect(await page.evaluate(() => [window.calls.select, window.calls.play])).toEqual([2, 3]);
    expect(await page.locator('button input, button button').count()).toBe(0);
    for (const key of ['Enter', 'Space', 'pointer']) {
      await page.evaluate(() => window.mount('grid', { failThumb: true }));
      const retry = page.getByRole('button', { name: 'translated:recordings.tapToRetry', exact: true });
      await retry.waitFor(); await focusVisible(page, retry);
      const before = await page.evaluate(() => window.calls.thumbnails);
      await page.evaluate(() => { window.failThumb = false; });
      if (key === 'pointer') await retry.click(); else await retry.press(key);
      await page.locator('.recording-card img').waitFor();
      expect(await page.evaluate(() => window.calls.thumbnails)).toBe(before + 1);
      expect(await page.evaluate(() => window.calls.play)).toBe(0);
    }
    await page.evaluate(() => window.mount('grid', { frames: 3 }));
    await page.locator('.recording-card').hover();
    await page.waitForFunction(() => window.calls.thumbnails === 3);
    await page.waitForFunction(() => document.querySelector('.recording-card img')?.src.endsWith('/1'));
    await page.mouse.move(1, 1);
    await page.waitForFunction(() => document.querySelector('.recording-card img')?.src.endsWith('/0'));
    expect(await page.evaluate(() => window.calls.play)).toBe(0);
    await page.evaluate(() => window.mount('grid', { inProgress: true }));
    await play.press('Enter'); expect(await page.evaluate(() => window.calls.play)).toBe(1);
    expect(await page.evaluate(() => window.calls.thumbnails)).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('fullscreen entry and dismissal are independent native actions; media guard remains intact', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    for (const key of ['Enter', 'Space', 'pointer']) {
      await page.evaluate(() => window.mount('timeline'));
      const enter = page.getByRole('button', { name: 'translated:timeline.clickToEnterFullscreen', exact: true });
      await enter.waitFor(); await focusVisible(page, enter);
      if (key === 'pointer') await enter.click(); else await enter.press(key);
      expect(await page.evaluate(() => window.calls.fullscreen)).toBe(1);
      expect(await page.evaluate(() => window.calls.fullscreenTarget)).toBe('timeline-video-container');
      expect(await page.evaluate(() => window.calls.userActivation)).toBe(true);
      expect(await enter.count()).toBe(0);
    }
    for (const key of ['Enter', 'Space', 'pointer']) {
      await page.evaluate(() => window.mount('timeline'));
      const dismiss = page.getByRole('button', { name: 'translated:timeline.exitFullscreen', exact: true });
      if (key === 'pointer') await dismiss.click(); else await dismiss.press(key);
      expect(await page.evaluate(() => window.calls.fullscreen)).toBe(0);
    }
    const video = page.locator('video'); expect(await video.evaluate(element => element.controls)).toBe(true);
    // Locate the unchanged mouse guard by its reserved native-controls band.
    const band = page.locator('[data-testid="timeline-video-container"] > div[style*="max(64px"]');
    expect(await band.count()).toBe(1);
    await band.dispatchEvent('pointerdown'); await band.dispatchEvent('dblclick');
    expect(await page.evaluate(() => window.calls.fullscreen)).toBe(1);
    expect(await page.locator('button button').count()).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('settings/system backdrops preserve native close/confirm, disabled preview and restart guard', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    for (const kind of ['clear', 'restart']) {
      await page.evaluate(value => window.mount(value), kind);
      const cancel = page.getByRole('button', { name: 'translated:common.cancel', exact: true });
      await cancel.waitFor(); await focusVisible(page, cancel); await cancel.press('Enter');
      expect(await page.evaluate(() => window.calls.close)).toBe(1);
      await page.evaluate(value => window.mount(value), kind);
      await page.getByRole('button', { name: 'translated:common.cancel', exact: true }).press('Space');
      expect(await page.evaluate(() => window.calls.close)).toBe(1);
      await page.evaluate(value => window.mount(value), kind);
      const background = page.locator('body > div.fixed').last();
      await background.locator('h3').click(); expect(await page.evaluate(() => window.calls.close)).toBe(0);
      await background.click({ position: { x: 1, y: 1 } }); expect(await page.evaluate(() => window.calls.close)).toBe(1);
      for (const key of ['Enter', 'Space', 'pointer']) {
        await page.evaluate(value => window.mount(value), kind);
        const confirm = page.locator(kind === 'clear' ? 'body > div.fixed .btn-danger' : 'body > div.fixed .btn-warning');
        if (key === 'pointer') await confirm.click(); else await confirm.press(key);
        expect(await page.evaluate(() => window.calls.confirm)).toBe(1);
        if (kind === 'clear') expect(await page.evaluate(() => window.calls.close)).toBe(1);
        else {
          expect(await confirm.isDisabled()).toBe(true);
          await page.keyboard.press('Enter'); await page.keyboard.press('Space');
          expect(await page.evaluate(() => window.calls.confirm)).toBe(1);
          await page.evaluate(() => window.finishConfirm());
        }
      }
    }
    await page.evaluate(() => window.mount('restart', { restarting: true })); await flushEffects(page);
    await page.keyboard.press('Escape'); await page.locator('body > div.fixed').last().click({ position: { x: 1, y: 1 } });
    expect(await page.evaluate(() => window.calls.close)).toBe(0);
    await page.evaluate(() => window.mount('go2rtc', { readonly: true }));
    const preview = page.getByRole('button', { name: 'translated:settings.go2rtcShowEffectiveConfig', exact: true });
    expect(await preview.isDisabled()).toBe(true); expect(await page.evaluate(() => window.calls.requests)).toBe(0);
    await page.evaluate(() => window.mount('go2rtc'));
    await preview.press('Enter');
    const outer = page.locator('#root .fixed');
    await outer.locator('pre').first().waitFor();
    await outer.locator('pre').first().click(); expect(await outer.count()).toBe(1);
    await outer.getByRole('button', { name: 'translated:common.close', exact: true }).press('Space'); expect(await outer.count()).toBe(0);
    await preview.click(); await outer.waitFor(); await outer.click({ position: { x: 1, y: 1 } }); expect(await outer.count()).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
