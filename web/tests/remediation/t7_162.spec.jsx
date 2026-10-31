/** Native actions, structural dialog boundaries and scoped media alternatives. */
const mockHooks = { cursor: 0, values: {}, setters: [] };
const mockZoom = { isZoomed: false, scale: 1, reset: jest.fn(), containerRef: {}, touchAction: 'auto' };
const mockMobile = { onClick: jest.fn(() => false), chromeVisible: true };
const mockLocale = jest.fn();
jest.mock('preact/hooks', () => ({
  useState: initial => { const i = mockHooks.cursor++; const setter = jest.fn(); mockHooks.setters[i] = setter;
    return [Object.hasOwn(mockHooks.values, i) ? mockHooks.values[i] : typeof initial === 'function' ? initial() : initial, setter]; },
  useEffect: jest.fn(), useCallback: fn => fn, useMemo: fn => fn(), useRef: current => ({ current }),
}));
jest.mock('preact/compat', () => ({ ...jest.requireActual('preact/compat'), createPortal: child => child }));
jest.mock('../../js/i18n.js', () => ({ t: key => `translated:${key}`, getLocalesUrl: file => '/locales/' + file,
  useI18n: () => ({ t: key => `translated:${key}`, locale: 'en', localePreference: null, setLocalePreference: mockLocale, availableLocales: [{ code: 'en', nativeName: 'English' }, { code: 'fr', nativeName: 'Français' }] }) }));
jest.mock('../../js/query-client.js', () => ({ useQueryClient: () => ({ invalidateQueries: jest.fn() }) }));
jest.mock('../../js/components/preact/useVideoZoom.js', () => ({ useVideoZoom: () => mockZoom }));
jest.mock('../../js/components/preact/useAutoRetry.js', () => ({ useAutoRetry: () => null }));
jest.mock('../../js/components/preact/MobileTileGestures.jsx', () => ({ useMobileTileGestures: () => mockMobile, MobileTileContextMenu: () => null }));
jest.mock('webrtc-adapter', () => ({}));
const { ToastContainer } = require('../../js/components/preact/ToastContainer.jsx');
const { SnapshotPreviewModal, VideoModal } = require('../../js/components/preact/UI.jsx');
const { ConfirmDialog, TextInputDialog } = require('../../js/components/preact/common/ModalDialog.jsx');
const { ZoneEditor } = require('../../js/components/preact/ZoneEditor.jsx');
const { WebRTCVideoCell } = require('../../js/components/preact/WebRTCVideoCell.jsx');
const LanguageSelector = require('../../js/components/preact/common/LanguageSelector.jsx').default;
const { useEffect } = require('preact/hooks');
const t = key => `translated:${key}`;
const stream = { name: 'Fixture camera', status: 'playing', can_control_privacy: false };
const zones = [{ id: 'one', name: 'Entry', enabled: true, color: '#3b82f6', polygon: [{ x: 0.1, y: 0.1 }, { x: 0.4, y: 0.1 }, { x: 0.2, y: 0.4 }] }];
const savedDocument = Object.getOwnPropertyDescriptor(globalThis, 'document');
function nodes(node) { if (Array.isArray(node)) return node.flatMap(nodes); return !node || typeof node !== 'object' ? [] : [node, ...nodes(node.props?.children)]; }
function render(Component, props = {}, values = {}) { mockHooks.cursor = 0; mockHooks.values = values; mockHooks.setters = []; return Component(props); }
function native(node) { expect(node).toBeDefined(); expect(node.type).toBe('button'); expect(node.props.type).toBe('button'); expect(node.props.onKeyDown).toBeUndefined(); }
beforeEach(() => { jest.clearAllMocks(); Object.defineProperty(globalThis, 'document', { configurable: true, value: { body: {}, activeElement: { blur: jest.fn() } } }); });
afterEach(() => { if (savedDocument) Object.defineProperty(globalThis, 'document', savedDocument); else delete globalThis.document; });
it('toast dismissal is a named native action retaining its message and one state update', () => {
  const tree = render(ToastContainer, {}, { 0: [{ id: 7, message: 'Fixture notice', type: 'info' }] });
  const cleanup = useEffect.mock.calls[0][0]();
  const button = nodes(tree).find(node => node.props?.children === 'Fixture notice'); native(button);
  expect(button.props['aria-label']).toBe('translated:common.close: Fixture notice'); button.props.onClick(); expect(mockHooks.setters[0]).toHaveBeenCalledTimes(1); expect(mockHooks.setters[0]).toHaveBeenCalledWith([]); cleanup();
});
it.each([['snapshot', SnapshotPreviewModal], ['video', VideoModal]])('%s backdrop remains target-only presentation with no artificial key action', (_kind, Component) => {
  const onClose = jest.fn(); const tree = render(Component, { isOpen: true, onClose });
  expect(tree.props.role).toBe('presentation'); expect(tree.props.onKeyDown).toBeUndefined();
  tree.props.onClick({ target: {}, currentTarget: {} }); expect(onClose).not.toHaveBeenCalled();
  const target = {}; tree.props.onClick({ target, currentTarget: target }); expect(onClose).toHaveBeenCalledTimes(1);
});
it.each([false, true])('shared ModalShell preserves backdrop mouse-down and click propagation guards (disabled=%s)', closeDisabled => {
  const onClose = jest.fn(); const dialog = render(ConfirmDialog, { isOpen: true, title: 'Fixture', onClose, onConfirm: jest.fn() });
  const tree = render(dialog.type, { ...dialog.props, closeDisabled }); expect(tree.props.role).toBe('presentation'); expect(tree.props.onKeyDown).toBeUndefined();
  const stopPropagation = jest.fn(); tree.props.onMouseDown({ target: {}, currentTarget: {}, stopPropagation }); expect(onClose).not.toHaveBeenCalled();
  const target = {}; tree.props.onMouseDown({ target, currentTarget: target, stopPropagation }); expect(onClose).toHaveBeenCalledTimes(closeDisabled ? 0 : 1);
  tree.props.onClick({ stopPropagation }); expect(stopPropagation).toHaveBeenCalledTimes(3);
  expect(nodes(tree).find(node => node.type === 'section').props.role).toBe('dialog');
});
it('zone selection has a native delegate while edits and canvas handlers retain their paths', () => {
  const tree = render(ZoneEditor, { zones, onClose: jest.fn(), onZonesChange: jest.fn() }, { 1: 0 });
  const card = nodes(tree).find(node => node.key === 'one'); expect(card.props.role).toBe('presentation');
  const button = nodes(card).find(node => node.type === 'button'); native(button); expect(button.props['aria-pressed']).toBe(true);
  expect(button.props.onClick).toBeUndefined(); card.props.onClick(); expect(mockHooks.setters[1]).toHaveBeenCalledTimes(1);
  const checkbox = nodes(card).find(node => node.props?.type === 'checkbox'); checkbox.props.onChange({ target: { checked: false } }); card.props.onClick();
  expect(mockHooks.setters[6]).toHaveBeenCalledTimes(1); expect(mockHooks.setters[6].mock.calls[0][0][0].enabled).toBe(false); expect(mockHooks.setters[1]).toHaveBeenCalledTimes(2);
  const canvas = nodes(tree).find(node => node.type === 'canvas'); for (const key of ['onClick', 'onMouseDown', 'onMouseMove', 'onMouseUp']) expect(typeof canvas.props[key]).toBe('function');
});
it.each([false, true])('language options are native actions on desktop/mobile=%s, keeping locale payload and blur', mobile => {
  const tree = render(LanguageSelector, { mobile });
  const buttons = nodes(tree).filter(node => node.type === 'button'); expect(buttons).toHaveLength(4); buttons.forEach(native);
  expect(buttons[0].props['aria-label']).toBe(t('language.label')); expect(nodes(tree).filter(node => node.type === 'li' && node.props.onClick)).toHaveLength(0);
  buttons.slice(1).forEach(button => button.props.onClick()); expect(mockLocale.mock.calls).toEqual([[null], ['en'], ['fr']]); expect(document.activeElement.blur).toHaveBeenCalledTimes(3);
});
it('WebRTC delegates gestures structurally and retains a native hidden-toolbar fullscreen action', () => {
  const onToggleFullscreen = jest.fn(); const tree = render(WebRTCVideoCell, { stream, streamId: 'fixture', showControls: false, onToggleFullscreen, alwaysFullscreenOnTap: true });
  expect(tree.props.role).toBe('presentation'); expect(tree.props.onKeyDown).toBeUndefined(); const action = nodes(tree).find(node => node.type === 'button' && node.props['aria-label'] === t('live.toggleFullscreen')); native(action);
  const cell = {}; tree.ref(cell); const event = { target: { closest: () => ({}) } }; action.props.onClick(event); tree.props.onClick(event); expect(onToggleFullscreen).toHaveBeenCalledTimes(1);
  tree.props.onClick({ target: { closest: () => null } }); expect(onToggleFullscreen).toHaveBeenCalledTimes(2);
});

async function openBrowser({ coarse = false } = {}) {
  const { chromium } = require('../../../node_modules/playwright'); const { build } = require('esbuild');
  const path = require('node:path'); const fs = require('node:fs'); const webRoot = path.resolve(__dirname, '../..');
  const result = await build({ stdin: { resolveDir: webRoot, loader: 'jsx', contents: `
    import { h, render } from 'preact';
    import { ToastContainer, addToast } from './js/components/preact/ToastContainer.jsx';
    import { SnapshotPreviewModal, VideoModal } from './js/components/preact/UI.jsx';
    import { ConfirmDialog, TextInputDialog } from './js/components/preact/common/ModalDialog.jsx';
    import LanguageSelector from './js/components/preact/common/LanguageSelector.jsx';
    import { ZoneEditor } from './js/components/preact/ZoneEditor.jsx';
    import { WebRTCVideoCell } from './js/components/preact/WebRTCVideoCell.jsx';
    const root = document.querySelector('#root'); const zones = ${JSON.stringify(zones)};
    window.fixtureTranslate = key => 'translated:' + key;
    window.fixtureMobile = { get chromeVisible() { return !window.options.chromeHidden; }, onClick: () => Boolean(window.options.consumeTap), onPointerDown: () => {}, onPointerMove: () => {}, onPointerUp: () => {}, onPointerCancel: () => {}, onContextMenu: () => {} };
    window.fixtureZoom = { get isZoomed() { return Boolean(window.options.zoomed); }, scale: 2, containerRef: {}, touchAction: 'none', reset: () => window.calls.reset++ };
    HTMLMediaElement.prototype.play = async function() { window.calls.play++; }; HTMLMediaElement.prototype.pause = function() {}; HTMLMediaElement.prototype.load = function() {};
    Element.prototype.requestFullscreen = async function() { window.calls.fullscreen++; };
    window.fetch = async (url, options = {}) => { if (options.method === 'POST') window.calls.requests.push(JSON.parse(options.body)); return { ok: true, json: async () => ({ zones }) }; };
    window.addFixtureToast = message => addToast(message, 'info', 60000);
    window.mount = (kind, options = {}) => {
      render(null, root); window.options = options;
      window.calls = { close: 0, confirm: 0, submit: [], locale: [], download: 0, requests: [], zones: [], fullscreen: 0, reset: 0, play: 0, outer: 0 };
      const close = () => { window.calls.close++; render(null, root); };
      let child;
      if (kind === 'toast') child = h(ToastContainer);
      if (kind === 'language') child = options.mobile ? h(LanguageSelector, { mobile: true }) : h('ul', null, h(LanguageSelector));
      if (kind === 'confirm') child = h(ConfirmDialog, { isOpen: true, onClose: close, title: 'Fixture confirmation', message: 'Confirm fixture', confirmLabel: 'Proceed', cancelLabel: 'Cancel', confirmDisabled: options.disabled, onConfirm: () => window.calls.confirm++ });
      if (kind === 'input') child = h(TextInputDialog, { isOpen: true, onClose: close, title: 'Fixture input', inputLabel: 'Name', initialValue: 'Entry', checkboxLabel: 'Flag', onSubmit: (...value) => { window.calls.submit.push(value); return new Promise(resolve => { window.finishSubmit = resolve; }); } });
      if (kind === 'snapshot') child = h(SnapshotPreviewModal, { isOpen: true, onClose: close, imageData: '/fixture.svg', streamName: 'Fixture', onDownload: () => window.calls.download++ });
      if (kind === 'video') child = h(VideoModal, { isOpen: true, onClose: close, videoUrl: '/fixture.mp4', title: 'Fixture video' });
      if (kind === 'zone') child = h(ZoneEditor, { streamName: 'Fixture', streamWidth: 640, streamHeight: 360, zones, onClose: close, onZonesChange: value => window.calls.zones.push(value) });
      if (kind === 'webrtc') child = h(WebRTCVideoCell, { stream: { ...${JSON.stringify(stream)}, status: options.loading ? 'connecting' : 'playing' }, streamId: 'fixture', showControls: options.controls !== false, alwaysFullscreenOnTap: Boolean(options.tap), onToggleFullscreen: () => window.calls.fullscreen++ });
      render(h('div', { onClick: () => window.calls.outer++ }, child), root);
    };
  ` }, bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact', plugins: [{ name: 'fixtures', setup(builder) {
    builder.onLoad({ filter: /WebRTCVideoCell\.jsx$/ }, args => {
      let source = fs.readFileSync(args.path, 'utf8').replace('useState, useEffect, useRef, useCallback', 'useState, useRef, useCallback');
      source = 'const useEffect = () => {};\n' + source;
      source = source.replace('const [error, setError] = useState(null)', "const [error, setError] = useState(window.options.error ? 'fixture-error' : null)");
      source = source.replace('const [isPlaying, setIsPlaying] = useState(false)', 'const [isPlaying, setIsPlaying] = useState(Boolean(window.options.playing))');
      return { contents: source, loader: 'jsx', resolveDir: path.dirname(args.path) };
    });
    builder.onResolve({ filter: /(?:i18n|query-client|useVideoZoom|useAutoRetry|MobileTileGestures|settings-utils|recording-playback)\.(?:js|jsx)$/ }, args => ({ path: args.path, namespace: 'fixture' }));
    builder.onLoad({ filter: /.*/, namespace: 'fixture' }, () => ({ loader: 'js', contents: `
      export const t = key => window.fixtureTranslate(key); export const getLocalesUrl = file => '/locales/' + file;
      export const useI18n = () => ({ t, locale: 'en', localePreference: null, setLocalePreference: value => window.calls.locale.push(value), availableLocales: [{ code: 'en', nativeName: 'English' }, { code: 'fr', nativeName: 'Français' }] });
      export const useQueryClient = () => ({ invalidateQueries: () => {} }); export const getGo2rtcBaseUrl = async () => '/go2rtc'; export const getWebRTCTimeouts = () => ({});
      export const useVideoZoom = () => window.fixtureZoom; export const useAutoRetry = () => null; export const useMobileTileGestures = () => window.fixtureMobile;
      export const MobileTileContextMenu = () => null; export const loadRecordingPlayback = () => () => {};` }));
    builder.onResolve({ filter: /(?:DetectionOverlay|SnapshotManager|ManualRecordingButton|PTZControls|DayNightControl|LiveTileStatus|PictureInPictureButton|FisheyeEptzCanvas)\.jsx$/ }, args => ({ path: args.path, namespace: 'child' }));
    builder.onLoad({ filter: /.*/, namespace: 'child' }, () => ({ loader: 'js', contents: `
      export const DetectionOverlay = () => null; export const drawDetectionsOnCanvas = () => {}; export const SnapshotButton = () => null;
      export const ManualRecordingButton = () => null; export const PTZControls = () => null; export const DayNightControl = () => null;
      export const LiveTileStatus = () => null; export const PictureInPictureButton = () => null; export const FisheyeEptzCanvas = () => null;` }));
  } }] });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 1000 }, hasTouch: coarse }); page.setDefaultTimeout(4000);
    const errors = []; page.on('pageerror', error => errors.push(error.message));
    await page.route('http://fixture.invalid/**', route => route.fulfill({ contentType: route.request().url().endsWith('/') ? 'text/html' : 'image/svg+xml', body: route.request().url().endsWith('/')
      ? '<button id="launcher">Launch</button><div id="root"></div>' : '<svg xmlns="http://www.w3.org/2000/svg" width="640" height="360"><rect width="640" height="360" fill="gray"/></svg>' }));
    await page.goto('http://fixture.invalid/');
    const styles = new Set(); for (const file of ['index.html', 'streams.html', 'recordings.html']) {
      const html = fs.readFileSync(path.join(webRoot, 'dist', file), 'utf8');
      for (const tag of html.match(/<link[^>]*rel="stylesheet"[^>]*>/g) || []) styles.add(tag.match(/href="([^"]+)"/)[1]);
    }
    for (const href of styles) await page.addStyleTag({ content: fs.readFileSync(path.join(webRoot, 'dist', href), 'utf8') });
    await page.addStyleTag({ content: '.video-cell { width: 640px; height: 360px; }' });
    await page.addScriptTag({ content: result.outputFiles[0].text }); return { browser, page, errors };
  } catch (error) { await browser.close(); throw error; }
}
const browserTest = process.env.T7_162_BROWSER === '1' ? it : it.skip;
async function focusVisible(page, locator) { await page.keyboard.press('Tab'); await locator.focus();
  expect(await locator.evaluate(element => { const style = getComputedStyle(element); return element.matches(':focus-visible') && ((style.outlineStyle !== 'none' && parseFloat(style.outlineWidth) > 0) || style.boxShadow !== 'none'); })).toBe(true); }
async function settle(page) { await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(resolve, 0))))); }
browserTest('each toast dismisses independently with Enter, Space or pointer while preserving remaining notices', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('toast')); await settle(page);
    await page.evaluate(() => ['One', 'Two', 'Three'].forEach(window.addFixtureToast));
    for (const [message, key] of [['One', 'Enter'], ['Two', 'Space'], ['Three', 'pointer']]) {
      const button = page.getByRole('button', { name: t('common.close') + ': ' + message }); await focusVisible(page, button);
      if (key === 'pointer') await button.click(); else await button.press(key); expect(await button.count()).toBe(0);
    }
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest.each([false, true])('language choices work by keyboard and pointer, closing after selection (mobile=%s)', async mobile => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(mobile => window.mount('language', { mobile }), mobile);
    for (const [name, key, value] of [['FR (Français)', 'Enter', 'fr'], ['EN (English)', 'Space', 'en'], [t('language.browserDefault'), 'pointer', null]]) {
      const trigger = page.getByRole('button', { name: t('language.label'), exact: true }); await focusVisible(page, trigger); await trigger.press('Enter');
      const option = page.getByRole('button', { name, exact: true }); await focusVisible(page, option); if (key === 'pointer') await option.click(); else await option.press(key);
      expect(await page.evaluate(() => window.calls.locale.at(-1))).toBe(value); expect(await page.locator('.dropdown').evaluate(element => element.matches(':focus-within'))).toBe(false);
    }
    expect(await page.evaluate(() => window.calls.locale)).toEqual(['fr', 'en', null]); expect(await page.locator('li[onclick], button button').count()).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('shared confirmation preserves initial focus, wrapping, restoration, native confirm and disabled guard', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.locator('#launcher').focus(); await page.evaluate(() => window.mount('confirm'));
    await page.waitForFunction(() => document.activeElement.textContent === 'Cancel'); const confirm = page.getByRole('button', { name: 'Proceed' });
    await page.keyboard.press('Shift+Tab'); expect(await confirm.evaluate(element => element === document.activeElement)).toBe(true);
    await page.keyboard.press('Tab'); expect(await page.getByRole('button', { name: 'Cancel' }).evaluate(element => element === document.activeElement)).toBe(true);
    await confirm.press('Enter'); expect(await page.evaluate(() => [window.calls.confirm, window.calls.close, window.calls.outer])).toEqual([1, 1, 0]);
    expect(await page.locator('#launcher').evaluate(element => element === document.activeElement)).toBe(true);
    await page.evaluate(() => window.mount('confirm', { disabled: true })); await settle(page); expect(await confirm.isDisabled()).toBe(true); await page.keyboard.press('Escape'); expect(await page.getByRole('dialog').count()).toBe(0); expect(await page.evaluate(() => window.calls.confirm)).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('shared input preserves inside-to-backdrop drag, pending Escape/backdrop guards and exact submit payload', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('input'));
    await page.waitForFunction(() => document.activeElement?.tagName === 'INPUT' && document.activeElement.value === 'Entry');
    const dialog = page.getByRole('dialog'); const input = page.getByRole('textbox', { name: 'Name' }); await input.fill(' Renamed ');
    const box = await input.boundingBox(); await page.mouse.move(box.x + 3, box.y + 3); await page.mouse.down(); await page.mouse.move(2, 2); await page.mouse.up(); expect(await dialog.count()).toBe(1);
    await page.getByRole('checkbox', { name: 'Flag' }).check(); await page.getByRole('button', { name: 'Save', exact: true }).press('Enter');
    expect(await input.isDisabled()).toBe(true); await settle(page); expect(await page.getByRole('button', { name: 'Cancel' }).isDisabled()).toBe(true);
    await page.keyboard.press('Escape'); await page.mouse.click(2, 2); expect(await dialog.count()).toBe(1); expect(await page.evaluate(() => window.calls.close)).toBe(0);
    expect(await page.evaluate(() => window.calls.submit)).toEqual([['Renamed', true]]);
    await page.evaluate(() => window.finishSubmit(false)); await page.waitForFunction(() => !document.querySelector('input[type=text]').disabled); await settle(page); await page.keyboard.press('Escape'); expect(await dialog.count()).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest.each(['snapshot', 'video'])('%s backdrop retains nested controls, Escape and target-only dismissal', async kind => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(kind => window.mount(kind), kind); await settle(page); const root = page.locator('#' + kind + '-preview-modal');
    if (kind === 'snapshot') { await page.getByRole('button', { name: t('common.download'), exact: true }).press('Enter'); expect(await page.evaluate(() => window.calls.download)).toBe(1); }
    else { expect(await page.locator('video').evaluate(element => element.controls)).toBe(true); await page.locator('video').click(); }
    expect(await root.count()).toBe(1); await page.keyboard.press('Escape'); expect(await root.count()).toBe(0); expect(await page.evaluate(() => window.calls.close)).toBe(1);
    await page.evaluate(kind => window.mount(kind), kind); await root.click({ position: { x: 2, y: 2 } }); expect(await root.count()).toBe(0); expect(await page.evaluate(() => window.calls.close)).toBe(1);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('zone native selection and field clicks keep one selection path, form edits and saved polygon', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('zone')); await page.locator('canvas').waitFor({ state: 'visible' });
    const select = page.getByRole('button', { name: t('zoneEditor.selectZone') + ' Entry', exact: true }); await focusVisible(page, select);
    await select.evaluate(element => { window.selectionClicks = 0; element.parentElement.addEventListener('click', () => window.selectionClicks++); });
    await select.press('Enter'); expect(await select.getAttribute('aria-pressed')).toBe('true'); await select.press('Space'); await select.click(); expect(await page.evaluate(() => window.selectionClicks)).toBe(3);
    const card = page.locator('div.mb-3.p-3.rounded.border').first(); await card.locator('input[type=text]').click(); await card.locator('input[type=text]').fill('Renamed'); await card.getByRole('checkbox').uncheck();
    expect(await page.evaluate(() => window.selectionClicks)).toBe(5); expect(await select.count()).toBe(0);
    await page.getByRole('button', { name: t('zoneEditor.saveZones'), exact: true }).click();
    const requests = await page.evaluate(() => window.calls.requests); expect(requests).toHaveLength(1); expect(requests[0].zones[0]).toMatchObject({ name: 'Renamed', enabled: false, polygon: zones[0].polygon });
    expect(await page.evaluate(() => [window.calls.zones.length, window.calls.close])).toEqual([1, 1]);
    await page.evaluate(() => window.mount('zone')); await page.locator('canvas').waitFor({ state: 'visible' });
    await page.getByRole('button', { name: /translated:common.delete/ }).click();
    const point = await page.locator('canvas').evaluate(canvas => {
      const rect = canvas.getBoundingClientRect(); const scale = Math.min(rect.width / 640, rect.height / 360);
      return { x: rect.left + (rect.width - 640 * scale) / 2 + 0.2 * 640 * scale,
        y: rect.top + (rect.height - 360 * scale) / 2 + 0.2 * 360 * scale };
    });
    await page.mouse.click(point.x, point.y); expect(await page.getByRole('button', { name: t('zoneEditor.selectZone') + ' Entry', exact: true }).count()).toBe(0);
    await page.getByRole('button', { name: t('zoneEditor.saveZones'), exact: true }).click();
    expect(await page.evaluate(() => window.calls.requests)).toEqual([{ zones: [] }]);

  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('WebRTC native fullscreen alternatives preserve hidden/loading/error states, tap/double-click and zoom', async () => {
  const { browser, page, errors } = await openBrowser({ coarse: true });
  try {
    for (const options of [{ controls: false }, { loading: true }, { error: true }, { playing: true }]) {
      await page.evaluate(options => window.mount('webrtc', options), options); const action = page.getByRole('button', { name: t('live.toggleFullscreen'), exact: true }); expect(await action.count()).toBe(1); await focusVisible(page, action);
      await action.press('Enter'); await action.press('Space'); expect(await page.evaluate(() => window.calls.fullscreen)).toBe(2);
    }
    await page.evaluate(() => window.mount('webrtc', { controls: false, playing: true, tap: true })); await page.locator('video').click(); expect(await page.evaluate(() => window.calls.fullscreen)).toBe(1);
    await page.getByRole('button', { name: t('live.toggleFullscreen'), exact: true }).press('Enter'); expect(await page.evaluate(() => window.calls.fullscreen)).toBe(2);
    await page.evaluate(() => window.mount('webrtc', { controls: false, playing: true, zoomed: true })); await page.locator('video').dblclick(); expect(await page.evaluate(() => window.calls.fullscreen)).toBe(0);
    await page.evaluate(() => window.mount('webrtc', { controls: false, playing: true })); await page.locator('video').dblclick(); expect(await page.evaluate(() => window.calls.fullscreen)).toBe(1);
    await page.evaluate(() => window.mount('webrtc', { playing: true, chromeHidden: true })); const group = page.locator('.stream-controls'); await page.waitForFunction(() => getComputedStyle(document.querySelector('.stream-controls')).opacity === '0');
    const action = page.getByRole('button', { name: t('live.toggleFullscreen'), exact: true }); await focusVisible(page, action); await page.waitForFunction(() => getComputedStyle(document.querySelector('.stream-controls')).opacity === '1');
    expect(await group.evaluate(element => getComputedStyle(element).pointerEvents)).toBe('auto'); await page.locator('#launcher').focus(); await page.waitForFunction(() => getComputedStyle(document.querySelector('.stream-controls')).opacity === '0');
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
