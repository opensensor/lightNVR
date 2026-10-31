/** Native action semantics and media gesture boundaries; external services are fixtures. */
const mockHooks = { cursor: 0, values: {}, setters: [] };
const mockZoom = { isZoomed: false, scale: 1, reset: jest.fn(), containerRef: {}, touchAction: 'auto' };
const mockMobile = { onClick: jest.fn(() => false), chromeVisible: true };
jest.mock('preact/hooks', () => ({
  useState: initial => { const i = mockHooks.cursor++; const setter = jest.fn(); mockHooks.setters[i] = setter;
    return [Object.hasOwn(mockHooks.values, i) ? mockHooks.values[i] : typeof initial === 'function' ? initial() : initial, setter]; },
  useEffect: jest.fn(), useCallback: callback => callback, useMemo: fn => fn(), useRef: current => ({ current }), useId: () => 'fixture',
}));
jest.mock('../../js/i18n.js', () => ({ t: key => `translated:${key}`, useI18n: () => ({ t: key => `translated:${key}` }) }));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn(), useQuery: jest.fn(() => ({ data: [], isLoading: false })), useQueryClient: () => ({ invalidateQueries: jest.fn() }), useMutation: () => ({}), usePostMutation: () => ({}) }));
jest.mock('../../js/components/preact/useVideoZoom.js', () => ({ useVideoZoom: () => mockZoom }));
jest.mock('../../js/components/preact/useAutoRetry.js', () => ({ useAutoRetry: () => null }));
jest.mock('../../js/components/preact/MobileTileGestures.jsx', () => ({ useMobileTileGestures: () => mockMobile, MobileTileContextMenu: () => null }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
jest.mock('hls.js', () => ({}));
const { GridPicker } = require('../../js/components/preact/GridPicker.jsx');
const { HLSVideoCell } = require('../../js/components/preact/HLSVideoCell.jsx');
const { MSEVideoCell } = require('../../js/components/preact/MSEVideoCell.jsx');
const { HealthView } = require('../../js/components/preact/HealthView.jsx');
const { StreamCard } = require('../../js/components/preact/StreamCard.jsx');
const { StreamsView } = require('../../js/components/preact/StreamsView.jsx');
const { useQuery } = require('../../js/query-client.js');
const t = key => `translated:${key}`;
const stream = { name: 'Fixture camera', enabled: true, status: 'Running', url: 'rtsp://fixture.invalid/camera', can_control_privacy: false };
function nodes(node) { if (Array.isArray(node)) return node.flatMap(nodes); return !node || typeof node !== 'object' ? [] : [node, ...nodes(node.props?.children)]; }
function render(Component, props = {}, values = {}) { mockHooks.cursor = 0; mockHooks.values = values; mockHooks.setters = []; return Component(props); }
function native(node) { expect(node).toBeDefined(); expect(node.type).toBe('button'); expect(node.props.type).toBe('button'); expect(node.props.onKeyDown).toBeUndefined(); }
beforeEach(() => { jest.clearAllMocks(); mockZoom.isZoomed = false; mockMobile.onClick.mockReturnValue(false); });
it('grid cells are named native buttons with the same selection coordinates and hover preview', () => {
  const onSelect = jest.fn(); const tree = render(GridPicker, { cols: 1, rows: 1, maxCells: 9, onSelect }, { 1: true });
  const cells = nodes(tree).filter(node => node.props?.style?.width === '20px'); expect(cells).toHaveLength(9);
  cells.forEach((cell, i) => { native(cell); expect(cell.props['aria-label']).toBe(t('live.gridLayoutAria')); expect(cell.props.disabled).toBe(false);
    cell.props.onMouseEnter(); cell.props.onClick(); expect(onSelect).toHaveBeenLastCalledWith(i % 3 + 1, Math.floor(i / 3) + 1); });
  expect(onSelect).toHaveBeenCalledTimes(9); expect(mockHooks.setters[1]).toHaveBeenCalledTimes(9);
});
it('grid allocation stays capped at 36 native actions when stream count exceeds the hard limit', () => {
  const tree = render(GridPicker, { cols: 1, rows: 1, maxCells: 1000, onSelect: jest.fn() }, { 1: true });
  const cells = nodes(tree).filter(node => node.props?.style?.width === '20px');
  expect(cells).toHaveLength(36); cells.forEach(native);
});
it('health disclosure is a native sibling of readable metric content', () => {
  useQuery.mockReturnValue({ data: { streams_detail: [{ name: stream.name, status: 'up' }] }, isLoading: false });
  const view = render(HealthView); const card = nodes(view).find(node => typeof node.type === 'function' && node.type.name === 'StreamCard'); expect(card).toBeDefined();
  const onToggle = jest.fn(); const tree = render(card.type, { ...card.props, expanded: true, onToggle });
  expect(tree.props.onClick).toBeUndefined(); const button = nodes(tree).find(node => node.type === 'button'); native(button);
  expect(button.props['aria-expanded']).toBe(true); expect(button.props['aria-label']).toBe(stream.name); button.props.onClick(); expect(onToggle).toHaveBeenCalledTimes(1);
  expect(nodes(button.props.children).filter(node => node.type === 'h3' || node.type === 'div')).toHaveLength(0);
});
it.each([false, true])('snapshot selection uses a native sibling action only in selection mode=%s', selectionMode => {
  const onToggleSelect = jest.fn(); const tree = render(StreamCard, { stream, t, selectionMode, canModifyStreams: true, isSelected: true, onToggleSelect });
  const snapshot = nodes(tree).find(node => (node.props?.class || '').includes('aspect-video'));
  expect(snapshot.props.onClick).toBeUndefined();
  const select = nodes(snapshot).find(node => node.type === 'button');
  if (!selectionMode) { expect(select).toBeUndefined(); return; }
  native(select); expect(select.props['aria-pressed']).toBe(true); expect(select.props['aria-label']).toBe(t('streams.selectStream'));
  const event = {}; select.props.onClick(event); expect(onToggleSelect).toHaveBeenCalledWith(event, stream.name);
  const checkbox = nodes(snapshot).find(node => node.type === 'input'); expect(nodes(select.props.children)).not.toContain(checkbox); expect(checkbox.props.onClick).toBeUndefined();
  checkbox.props.onChange(event); expect(onToggleSelect).toHaveBeenCalledTimes(2);
});
it.each([['hls', HLSVideoCell], ['mse', MSEVideoCell]])('%s gesture boundary preserves tap/double-click targets and supplies a native fullscreen fallback', (_kind, Component) => {
  const onToggleFullscreen = jest.fn(); const tree = render(Component, { stream, streamId: 'camera', onToggleFullscreen, showControls: false, alwaysFullscreenOnTap: true });
  expect(tree.props.role).toBe('presentation'); expect(tree.props.onKeyDown).toBeUndefined();
  const fallback = nodes(tree).find(node => node.type === 'button' && node.props['aria-label'] === t('live.toggleFullscreen')); native(fallback);
  const cell = {}; tree.ref(cell); const controlTarget = { closest: () => ({}) }; const videoTarget = { closest: () => null };
  const controlEvent = { target: controlTarget }; fallback.props.onClick(controlEvent); tree.props.onClick(controlEvent); expect(onToggleFullscreen).toHaveBeenCalledTimes(1);
  tree.props.onClick({ target: videoTarget }); expect(onToggleFullscreen).toHaveBeenCalledTimes(2);
  mockMobile.onClick.mockReturnValue(true); tree.props.onClick({ target: videoTarget }); expect(onToggleFullscreen).toHaveBeenCalledTimes(2); mockMobile.onClick.mockReturnValue(false);
  mockZoom.isZoomed = true; tree.props.onClick({ target: videoTarget }); expect(onToggleFullscreen).toHaveBeenCalledTimes(2);
  mockZoom.isZoomed = false; const double = render(Component, { stream, streamId: 'camera', onToggleFullscreen, alwaysFullscreenOnTap: false });
  double.props.onClick({ target: videoTarget }); double.props.onDblClick({ target: controlTarget }); expect(onToggleFullscreen).toHaveBeenCalledTimes(2);
  double.props.onDblClick({ target: videoTarget }); expect(onToggleFullscreen).toHaveBeenCalledTimes(3);
  expect(nodes(tree).filter(node => node.type === 'video')).toHaveLength(1);
});
it('HLS autoplay recovery is native and calls play once without adding keyboard handlers', async () => {
  const tree = render(HLSVideoCell, { stream, streamId: 'camera', onToggleFullscreen: jest.fn() }, { 0: false });
  const play = nodes(tree).find(node => (node.props?.className || '').includes('play-overlay')); native(play);
  expect(play.props['aria-label']).toBe(t('recordings.play')); const video = nodes(tree).find(node => node.type === 'video'); video.ref.current = { play: jest.fn(async () => {}) };
  await play.props.onClick(); await Promise.resolve(); expect(video.ref.current.play).toHaveBeenCalledTimes(1); expect(mockHooks.setters[2]).toHaveBeenCalledWith(true);
});
it.each([['onvif-modal', 4], ['custom-name-modal', 5]])('%s header close is native and targets only its own modal', (id, index) => {
  useQuery.mockReturnValue({ data: [], isLoading: false }); const tree = render(StreamsView, {}, { [index]: true });
  const modal = nodes(tree).find(node => node.props?.id === id); expect(modal).toBeDefined();
  const close = nodes(modal).find(node => node.type === 'button' && node.props['aria-label'] === t('common.close')); native(close); close.props.onClick();
  expect(mockHooks.setters[index]).toHaveBeenCalledWith(false); expect(mockHooks.setters[index === 4 ? 5 : 4]).not.toHaveBeenCalled();
});

async function openBrowser({ coarse = false } = {}) {
  const { chromium } = require('../../../node_modules/playwright'); const { build } = require('esbuild');
  const path = require('node:path'); const fs = require('node:fs'); const webRoot = path.resolve(__dirname, '../..');
  const result = await build({ stdin: { resolveDir: webRoot, loader: 'jsx', contents: `
    import { h, render } from 'preact'; import { useState } from 'preact/hooks';
    import { GridPicker } from './js/components/preact/GridPicker.jsx';
    import { HealthView } from './js/components/preact/HealthView.jsx';
    import { StreamCard } from './js/components/preact/StreamCard.jsx';
    import { StreamsView } from './js/components/preact/StreamsView.jsx';
    import { HLSVideoCell } from './js/components/preact/HLSVideoCell.jsx';
    import { MSEVideoCell } from './js/components/preact/MSEVideoCell.jsx';
    const root = document.querySelector('#root');
    const t = (key, values) => 'translated:' + key + (key === 'live.gridLayoutAria' ? ':' + values.cols + 'x' + values.rows : '');
    window.fixtureTranslate = t;
    window.fixtureQuery = key => ({ data: key[0] === 'streamHealth' ? { streams_detail: [{ name: 'Fixture camera', status: 'up', observed_fps: 24, expected_fps: 30 }] } : [], isLoading: false });
    HTMLMediaElement.prototype.play = async function() { window.calls.play++; };
    HTMLMediaElement.prototype.pause = function() {};
    window.fixtureMobile = { get chromeVisible() { return !window.options.chromeHidden; }, onClick: () => Boolean(window.options.consumeTap), onPointerDown: () => window.calls.pointer++, onPointerMove: () => {}, onPointerUp: () => {}, onPointerCancel: () => {}, onContextMenu: () => {} };
    window.fixtureZoom = { get isZoomed() { return Boolean(window.options.zoomed); }, scale: 2, containerRef: {}, touchAction: 'none', reset: () => window.calls.reset++ };
    function Host({ kind }) {
      const [selected, setSelected] = useState(false);
      const stream = ${JSON.stringify(stream)};
      if (kind === 'grid') return h(GridPicker, { cols: 1, rows: 1, maxCells: 9, onSelect: (...value) => window.calls.grid.push(value) });
      if (kind === 'health') return h(HealthView);
      if (kind === 'card') return h(StreamCard, { stream, t, selectionMode: !window.options.noSelection, canModifyStreams: !window.options.readonly, isSelected: selected,
        onToggleSelect: (event, name) => { event.stopPropagation(); window.calls.select.push(name); setSelected(old => !old); }, onEdit: () => window.calls.edit++, onClone: () => {}, onOpenDelete: () => {}, onEnable: () => {}, onDisable: () => {} });
      if (kind === 'streams') return h(StreamsView);
      return h(kind === 'hls' ? HLSVideoCell : MSEVideoCell, { stream, streamId: 'fixture', showControls: window.options.controls !== false,
        alwaysFullscreenOnTap: Boolean(window.options.tap), onToggleFullscreen: (name, event, cell) => { window.calls.fullscreen.push({ name, target: cell?.dataset.streamName }); } });
    }
    window.mount = (kind, options = {}) => { render(null, root); window.options = options; window.calls = { grid: [], select: [], edit: 0, play: 0, fullscreen: [], pointer: 0, reset: 0 }; render(h(Host, { kind }), root); };
  ` }, bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact', plugins: [{ name: 'fixtures', setup(builder) {
    // Fixture only transport/telemetry effects and initial transient states. Rendering,
    // DOM event handlers and state updates remain the actual component code.
    builder.onLoad({ filter: /(?:HLSVideoCell|MSEVideoCell|StreamsView)\.jsx$/ }, async args => {
      let source = fs.readFileSync(args.path, 'utf8');
      if (!args.path.endsWith('StreamsView.jsx')) {
        source = source.replace('useState, useEffect, useRef', 'useState, useRef');
        source = 'const useEffect = () => {};\n' + source;
        source = source.replace('useState(true);', 'useState(Boolean(window.options.loading));');
        source = source.replace('const [error, setError] = useState(null)', "const [error, setError] = useState(window.options.error ? 'fixture-error' : null)");
        source = source.replace('const [isPlaying, setIsPlaying] = useState(false)', 'const [isPlaying, setIsPlaying] = useState(Boolean(window.options.playing))');
      } else {
        source = source.replace('const [onvifModalVisible, setOnvifModalVisible] = useState(false)', 'const [onvifModalVisible, setOnvifModalVisible] = useState(Boolean(window.options.onvif))');
        source = source.replace('const [showCustomNameInput, setShowCustomNameInput] = useState(false)', 'const [showCustomNameInput, setShowCustomNameInput] = useState(Boolean(window.options.custom))');
      }
      return { contents: source, loader: 'jsx', resolveDir: path.dirname(args.path) };
    });
    builder.onResolve({ filter: /(?:i18n|query-client|auth-utils|useVideoZoom|useAutoRetry|MobileTileGestures|ToastContainer)\.(?:js|jsx)$/ }, args => ({ path: args.path, namespace: 'fixture' }));
    builder.onLoad({ filter: /.*/, namespace: 'fixture' }, () => ({ loader: 'js', contents: `
      export const t = (key, values) => window.fixtureTranslate(key, values); export const useI18n = () => ({ t });
      export const useQuery = key => window.fixtureQuery(key); export const fetchJSON = async () => ({});
      export const useQueryClient = () => ({ invalidateQueries: () => {} }); export const useMutation = () => ({}); export const usePostMutation = () => ({});
      export const validateSession = async () => ({ valid: true, role: 'admin' }); export const useVideoZoom = () => window.fixtureZoom;
      export const useAutoRetry = () => null; export const useMobileTileGestures = () => window.fixtureMobile;
      export const MobileTileContextMenu = () => null; export const showStatusMessage = () => {};` }));
    const stub = /(?:DetectionOverlay|SnapshotManager|ManualRecordingButton|PTZControls|DayNightControl|UI|LiveTileStatus|PictureInPictureButton|TileAudioButton|FisheyeEptzCanvas|StreamConfigModal|FleetView|EventRoutingView)\.jsx$/;
    builder.onResolve({ filter: stub }, args => ({ path: args.path, namespace: 'child' }));
    builder.onLoad({ filter: /.*/, namespace: 'child' }, () => ({ loader: 'js', contents: `
      export const DetectionOverlay = () => null; export const drawDetectionsOnCanvas = () => {}; export const SnapshotButton = () => null;
      export const ManualRecordingButton = () => null; export const PTZControls = () => null; export const DayNightControl = () => null; export const ConfirmDialog = () => null;
      export const LiveTileStatus = () => null; export const PictureInPictureButton = () => null; export const TileAudioButton = () => null; export const FisheyeEptzCanvas = () => null;
      export const StreamConfigModal = () => null; export const FleetView = () => null; export const EventRoutingView = () => null;` }));
  } }] });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, hasTouch: coarse }); page.setDefaultTimeout(4000);
    const errors = []; page.on('pageerror', error => errors.push(error.message));
    await page.route('http://fixture.invalid/**', route => route.fulfill({ contentType: 'text/html', body: '<button id="launcher">Launch</button><div id="root"></div>' }));
    await page.goto('http://fixture.invalid/');
    const styles = new Set(); for (const file of ['index.html', 'streams.html']) {
      const html = fs.readFileSync(path.join(webRoot, 'dist', file), 'utf8');
      for (const tag of html.match(/<link[^>]*rel="stylesheet"[^>]*>/g) || []) styles.add(tag.match(/href="([^"]+)"/)[1]);
    }
    for (const href of styles) await page.addStyleTag({ content: fs.readFileSync(path.join(webRoot, 'dist', href), 'utf8') });
    await page.addStyleTag({ content: '.video-cell { width: 640px; height: 360px; }' });
    await page.addScriptTag({ content: result.outputFiles[0].text }); return { browser, page, errors };
  } catch (error) { await browser.close(); throw error; }
}
const browserTest = process.env.T7_161_BROWSER === '1' ? it : it.skip;
async function focusVisible(page, locator) {
  await page.keyboard.press('Tab'); await locator.focus();
  expect(await locator.evaluate(element => { const style = getComputedStyle(element); return element.matches(':focus-visible') && ((style.outlineStyle !== 'none' && parseFloat(style.outlineWidth) > 0) || style.boxShadow !== 'none'); })).toBe(true);
}
browserTest('grid cells preview on hover and select once by Enter, Space or pointer', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    for (const key of ['Enter', 'Space', 'pointer']) {
      await page.evaluate(() => window.mount('grid')); await page.getByRole('button', { name: 'translated:live.gridLayoutAria:1x1' }).click();
      const cell = page.getByRole('button', { name: 'translated:live.gridLayoutAria:2x3' }); await cell.hover(); expect(await page.getByText('2 × 3', { exact: true }).count()).toBe(1);
      await focusVisible(page, cell); if (key === 'pointer') await cell.click(); else await cell.press(key);
      expect(await page.evaluate(() => window.calls.grid)).toEqual([[2, 3]]); expect(await cell.count()).toBe(0);
    }
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('health disclosure preserves readable metrics and toggles once per native activation', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('health')); const toggle = page.getByRole('button', { name: 'Fixture camera', exact: true }); await focusVisible(page, toggle);
    await toggle.evaluate(element => { window.disclosureClicks = 0; element.addEventListener('click', () => window.disclosureClicks++); });
    expect(await toggle.getAttribute('aria-expanded')).toBe('false'); await toggle.press('Enter'); expect(await toggle.getAttribute('aria-expanded')).toBe('true');
    expect(await page.getByText('translated:streamHealth.framesTotal:', { exact: false }).count()).toBeGreaterThan(0);
    await toggle.press('Space'); expect(await toggle.getAttribute('aria-expanded')).toBe('false'); await toggle.click();
    expect(await page.evaluate(() => window.disclosureClicks)).toBe(3);
    expect(await toggle.getAttribute('aria-expanded')).toBe('true');
    expect(await page.getByRole('heading', { name: 'Fixture camera' }).count()).toBe(1); expect(await page.locator('button h3').count()).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('snapshot selection and checkbox are independent; action controls and readonly mode retain behavior', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('card')); const select = page.getByRole('button', { name: t('streams.selectStream'), exact: true }); await focusVisible(page, select);
    await select.press('Enter'); expect(await select.getAttribute('aria-pressed')).toBe('true'); await select.press('Space'); expect(await select.getAttribute('aria-pressed')).toBe('false'); await select.click();
    const checkbox = page.getByRole('checkbox', { name: t('streams.selectStream') }); await checkbox.uncheck(); expect(await select.getAttribute('aria-pressed')).toBe('false');
    expect(await page.evaluate(() => window.calls.select)).toHaveLength(4); expect(await page.locator('button input,button button').count()).toBe(0);
    await page.evaluate(() => window.mount('card', { noSelection: true, readonly: true })); expect(await page.getByRole('checkbox').count()).toBe(0); expect(await page.getByRole('button', { name: t('streams.selectStream'), exact: true }).count()).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest.each(['onvif', 'custom'])('%s header close works from keyboard without activating fields or another modal', async kind => {
  const { browser, page, errors } = await openBrowser();
  try {
    for (const key of ['Enter', 'Space', 'pointer']) {
      await page.evaluate(kind => window.mount('streams', { [kind]: true }), kind);
      const id = kind === 'onvif' ? '#onvif-modal' : '#custom-name-modal'; const modal = page.locator(id);
      const field = modal.locator('input').first(); await field.fill('fixture'); expect(await modal.count()).toBe(1);
      const close = modal.locator('.border-b').first().getByRole('button', { name: t('common.close'), exact: true }); await focusVisible(page, close);
      if (key === 'pointer') await close.click(); else await close.press(key); expect(await modal.count()).toBe(0);
    }
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest.each(['hls', 'mse'])('%s fullscreen keyboard alternatives preserve tap, double-click, nested controls and zoom guards', async kind => {
  const { browser, page, errors } = await openBrowser();
  try {
    for (const options of [{ controls: false }, { loading: true }, { error: true }, { playing: true }]) {
      await page.evaluate(({ kind, options }) => window.mount(kind, options), { kind, options });
      const action = page.getByRole('button', { name: t('live.toggleFullscreen'), exact: true }); expect(await action.count()).toBe(1);
      const fallback = (await action.getAttribute('class')).includes('sr-only');
      if (fallback) expect((await action.boundingBox()).width).toBe(1);
      await focusVisible(page, action); if (fallback) expect((await action.boundingBox()).width).toBeGreaterThan(20);
      if (kind === 'hls' && (options.loading || options.error || options.playing)) expect(await page.getByRole('button', { name: t('recordings.play'), exact: true }).count()).toBe(0);
      await action.press('Enter'); await action.press('Space'); expect(await page.evaluate(() => window.calls.fullscreen)).toHaveLength(2);
    }
    await page.evaluate(kind => window.mount(kind, { controls: false, playing: true, tap: true }), kind);
    const video = page.locator('video'); await video.click(); expect(await page.evaluate(() => window.calls.fullscreen)).toHaveLength(1);
    await page.getByRole('button', { name: t('live.toggleFullscreen'), exact: true }).press('Enter'); expect(await page.evaluate(() => window.calls.fullscreen)).toHaveLength(2);
    await page.evaluate(() => { window.options.consumeTap = true; }); await video.click(); expect(await page.evaluate(() => window.calls.fullscreen)).toHaveLength(2);
    await page.evaluate(kind => window.mount(kind, { controls: false, playing: true }), kind); await video.dblclick(); expect(await page.evaluate(() => window.calls.fullscreen)).toHaveLength(1);
    await page.evaluate(kind => window.mount(kind, { controls: false, playing: true, zoomed: true }), kind); await video.dblclick(); expect(await page.evaluate(() => window.calls.fullscreen)).toHaveLength(0);
    await page.getByRole('button', { name: t('live.resetZoom'), exact: true }).press('Enter'); expect(await page.evaluate(() => window.calls.reset)).toBe(1); expect(await page.evaluate(() => window.calls.fullscreen)).toHaveLength(0);
    if (kind === 'hls') {
      await page.evaluate(() => window.mount('hls', { controls: false, tap: true })); const play = page.getByRole('button', { name: t('recordings.play'), exact: true }); await focusVisible(page, play); await play.press('Space');
      expect(await page.evaluate(() => window.calls.play)).toBe(1); expect(await page.evaluate(() => window.calls.fullscreen)).toHaveLength(0);
    }
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest.each(['hls', 'mse'])('%s coarse-pointer controls reveal for keyboard focus and hide again on blur', async kind => {
  const { browser, page, errors } = await openBrowser({ coarse: true });
  try {
    expect(await page.evaluate(() => matchMedia('(hover: none) and (pointer: coarse)').matches)).toBe(true);
    await page.evaluate(kind => window.mount(kind, { playing: true, chromeHidden: true }), kind);
    const group = page.locator(kind === 'hls' ? '.stream-controls' : '.video-controls');
    await page.waitForFunction(selector => getComputedStyle(document.querySelector(selector)).opacity === '0', kind === 'hls' ? '.stream-controls' : '.video-controls');
    expect(await group.evaluate(element => getComputedStyle(element).pointerEvents)).toBe('none');
    const action = page.getByRole('button', { name: t('live.toggleFullscreen'), exact: true }); await focusVisible(page, action);
    await page.waitForFunction(selector => getComputedStyle(document.querySelector(selector)).opacity === '1', kind === 'hls' ? '.stream-controls' : '.video-controls');
    expect(await group.evaluate(element => getComputedStyle(element).pointerEvents)).toBe('auto');
    await action.press('Enter'); expect(await page.evaluate(() => window.calls.fullscreen)).toHaveLength(1);
    await page.locator('#launcher').focus(); await page.waitForFunction(selector => getComputedStyle(document.querySelector(selector)).opacity === '0', kind === 'hls' ? '.stream-controls' : '.video-controls');
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
