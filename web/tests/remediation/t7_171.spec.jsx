/** Names reuse existing localized control text; callbacks retain native semantics. */
const mockHooks = { cursor: 0, values: {}, setters: [] };
jest.mock('preact/hooks', () => ({
  useState: initial => { const i = mockHooks.cursor++; const setter = jest.fn(); mockHooks.setters[i] = setter;
    return [Object.hasOwn(mockHooks.values, i) ? mockHooks.values[i] : typeof initial === 'function' ? initial() : initial, setter]; },
  useEffect: jest.fn(), useCallback: callback => callback, useMemo: fn => fn(), useRef: current => ({ current }), useId: () => 'fixture',
}));
jest.mock('../../js/i18n.js', () => ({ t: key => `translated:${key}`, useI18n: () => ({ t: key => `translated:${key}` }) }));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn(), useQuery: jest.fn(), useQueryClient: () => ({ invalidateQueries: jest.fn() }) }));
jest.mock('../../js/request-queue.js', () => ({ queueThumbnailLoad: jest.fn(async () => {}), invalidateThumbnailLoad: jest.fn(), Priority: {} }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
const { LiveBuildingPlan } = require('../../js/components/preact/live/LiveBuildingPlan.jsx');
const { LiveOperatorNavigator } = require('../../js/components/preact/live/LiveOperatorNavigator.jsx');
const { PlanSketchInspector } = require('../../js/components/preact/live/PlanSketchControls.jsx');
const { FiltersSidebar } = require('../../js/components/preact/recordings/FiltersSidebar.jsx');
const { RecordingsGrid } = require('../../js/components/preact/recordings/RecordingsGrid.jsx');
const { RecordingsTable } = require('../../js/components/preact/recordings/RecordingsTable.jsx');
const { useQuery } = require('../../js/query-client.js');
const t = key => `translated:${key}`;
const stream = { name: 'Fixture camera', camera_uuid: 'camera', availability: 'live', location_path: 'East' };
const placement = { camera_uuid: 'camera', x: 0.3, y: 0.4, rotation: 0, fov: 70 };
const plan = { uuid: 'plan', name: 'Fixture plan', canvas_width: 1200, canvas_height: 800, cameras: [placement] };
const recording = { id: 7, stream: 'Fixture camera', start_time_unix: 1700000000, duration: 5, size: '1 MB' };
const filters = { dateRange: 'custom', streamIds: [], detectionLabels: [], tags: [], captureMethods: [], protectedStatus: 'all', recordingType: 'all', startDate: '2026-10-01', endDate: '2026-10-31', startTime: '00:00', endTime: '23:59' };
const savedGlobals = Object.fromEntries(['document', 'window', 'localStorage'].map(key => [key, Object.getOwnPropertyDescriptor(globalThis, key)]));
function nodes(node) { if (Array.isArray(node)) return node.flatMap(nodes); return !node || typeof node !== 'object' ? [] : [node, ...nodes(node.props?.children)]; }
function render(Component, props, values = {}) { mockHooks.cursor = 0; mockHooks.values = values; mockHooks.setters = []; return Component(props); }
function name(node, key) { expect(node).toBeDefined(); expect(node.props['aria-label']).toBe(t(key)); }
function input(node, value) { (node.props.onInput || node.props.oninput || node.props.onChange)({ target: { value }, currentTarget: { value } }); }
beforeEach(() => {
  jest.clearAllMocks();
  Object.defineProperty(globalThis, 'document', { configurable: true, value: { body: {} } });
  Object.defineProperty(globalThis, 'window', { configurable: true, value: { location: { search: '', href: 'http://fixture.invalid/' } } });
  Object.defineProperty(globalThis, 'localStorage', { configurable: true, value: { getItem: () => null } });
  useQuery.mockImplementation(({ queryKey }) => ({ data: queryKey[0] === 'operator-floor-plans' ? { plans: [plan], can_modify: true } : {}, isLoading: false }));
});
afterEach(() => { for (const [key, descriptor] of Object.entries(savedGlobals)) {
  if (descriptor) Object.defineProperty(globalThis, key, descriptor); else delete globalThis[key];
} });
it('plan dimensions and both camera-coverage sliders retain localized names and bounded updates', () => {
  const tree = render(LiveBuildingPlan, { streams: [stream], selectedPlanUuid: 'plan', onSelectPlan: jest.fn() }, { 0: true, 1: [placement], 4: 'camera' });
  const dimensions = nodes(tree).filter(node => node.type === 'input' && node.props.step === '10');
  expect(dimensions).toHaveLength(2);
  dimensions.forEach((field, i) => { name(field, i ? 'live.plan.height' : 'live.plan.width'); input(field, '900'); });
  expect(mockHooks.setters[2].mock.calls).toEqual([[{ canvas_width: '900', canvas_height: 800 }], [{ canvas_width: 1200, canvas_height: '900' }]]);
  const controls = nodes(tree).filter(node => typeof node.type === 'function' && node.type.name === 'PlanCoverageControl');
  expect(controls).toHaveLength(2);
  controls.forEach(control => {
    const change = jest.fn(); const field = nodes(render(control.type, { ...control.props, onChange: change })).find(node => node.props?.type === 'range');
    expect(field.props['aria-label']).toBe(control.props.label); input(field, '42'); expect(change).toHaveBeenCalledWith(42);
    expect(field.props.min).toBe(control.props.minimum); expect(field.props.max).toBe(control.props.maximum);
  });
});
it('navigator search and camera name preserve filtering, native activation and drag payload', () => {
  const onApplyLayout = jest.fn();
  const tree = render(LiveOperatorNavigator, { streams: [stream], orderedStreams: [stream], onFilterChange: jest.fn(), onApplyLayout });
  const search = nodes(tree).find(node => node.props?.type === 'search'); name(search, 'live.navigator.search'); input(search, 'East');
  expect(mockHooks.setters[5]).toHaveBeenCalledWith('East');
  const camera = nodes(tree).find(node => node.props?.className === 'live-camera-row');
  expect(camera.props['aria-label']).toBe('Fixture camera East'); camera.props.onClick();
  expect(onApplyLayout).toHaveBeenCalledWith({ cameraUuids: ['camera'], columns: 1, rows: 1 });
  const dataTransfer = { setData: jest.fn() }; camera.props.onDragStart({ dataTransfer });
  expect(dataTransfer.setData).toHaveBeenCalledWith('application/x-lightnvr-camera', 'camera'); expect(dataTransfer.effectAllowed).toBe('copy');
});
it.each(['rect', 'label', 'wall', 'area'])('sketch %s names editable fields and tones while retaining normalized geometry updates', type => {
  const shape = { id: 'shape', type, tone: 'blue', x: 0.1, y: 0.2, w: 0.3, h: 0.4, label: 'Lobby', text: 'Lobby', size: 'md', filled: true, points: [[0.1, 0.1], [0.6, 0.1], [0.3, 0.6]] };
  const onChange = jest.fn(); const tree = render(PlanSketchInspector, { uid: 'sketch', shapes: [shape], selectedShape: shape, t, onChange });
  const tones = nodes(tree).filter(node => node.type === 'button' && node.props.title?.startsWith('translated:live.plan.sketch.tone.'));
  expect(tones.length).toBeGreaterThan(1);
  tones.forEach(tone => { expect(tone.props['aria-label']).toBe(tone.props.title); tone.props.onClick(); });
  expect(onChange).toHaveBeenCalledTimes(tones.length);
  for (const field of nodes(tree).filter(node => node.type === 'input')) {
    const key = field.props.type === 'checkbox' ? 'filled' : type === 'label' ? 'text' : 'labelText';
    name(field, 'live.plan.sketch.' + key);
    if (field.props.type === 'checkbox') field.props.onChange({ currentTarget: { checked: false } }); else input(field, 'New label');
  }
  const percents = nodes(tree).filter(node => typeof node.type === 'function' && node.type.name === 'PercentField');
  expect(percents.length).toBeGreaterThan(0);
  percents.forEach(percent => {
    const commit = jest.fn(); const field = nodes(percent.type({ ...percent.props, onCommit: commit })).find(node => node.type === 'input');
    expect(field.props['aria-label']).toBe(percent.props.label); input(field, '37.5'); expect(commit).toHaveBeenCalledWith(0.375);
    input(field, '150'); expect(commit).toHaveBeenLastCalledWith(1);
  });
});
it('custom filter names preserve independent start/end date/time updates and hide action', () => {
  const setFilters = jest.fn(); const toggleCollapsed = jest.fn();
  const tree = render(FiltersSidebar, { filters, setFilters, streams: [], pagination: { pageSize: 20 }, toggleCollapsed });
  const hide = nodes(tree).find(node => node.props?.title === t('recordings.hideFilters')); name(hide, 'recordings.hideFilters'); hide.props.onClick(); expect(toggleCollapsed).toHaveBeenCalledTimes(1);
  for (const [id, key, prop] of [['start-date', 'start', 'startDate'], ['start-time', 'start', 'startTime'], ['end-date', 'end', 'endDate'], ['end-time', 'end', 'endTime']]) {
    const field = nodes(tree).find(node => node.props?.id === id); name(field, 'recordings.' + key); input(field, 'fixture');
    expect(setFilters.mock.lastCall[0](filters)).toEqual({ ...filters, [prop]: 'fixture' });
  }
});
function recordingProps() { return { recordings: [recording], selectedRecordings: {}, getSelectedCount: () => 0, collapsed: true, pagination: { totalItems: 1 },
  playRecording: jest.fn(), downloadRecording: jest.fn(), deleteRecording: jest.fn(), toggleCollapsed: jest.fn(), toggleSelectAll: jest.fn(), toggleColumn: jest.fn(), hiddenColumns: {} }; }
it.each([['grid', RecordingsGrid], ['table', RecordingsTable]])('%s toolbar/configuration/recording actions retain names and exact callbacks', (kind, Component) => {
  const props = recordingProps(); const tree = render(Component, props, { 0: kind === 'grid' });
  const show = nodes(tree).find(node => node.props?.title === t('recordings.showFilters')); name(show, 'recordings.showFilters'); show.props.onClick(); expect(props.toggleCollapsed).toHaveBeenCalledTimes(1);
  if (kind === 'grid') { const all = nodes(tree).find(node => node.type === 'input'); name(all, 'recordings.selectAll'); all.props.onChange(); expect(props.toggleSelectAll).toHaveBeenCalledTimes(1); }
  const dropdown = nodes(tree).find(node => typeof node.type === 'function' && node.type.name.endsWith('ConfigDropdown'));
  const config = render(dropdown.type, dropdown.props, { 0: true });
  if (kind === 'grid') name(nodes(config).find(node => node.type === 'button'), 'recordings.configureElements');
  const checks = nodes(config).filter(node => node.type === 'input'); expect(checks.length).toBeGreaterThan(4);
  checks.forEach(check => { expect(check.props['aria-label']).toMatch(/^translated:/); expect(check.props.checked).toBe(true); check.props.onChange(); });
  expect(props.toggleColumn).toHaveBeenCalledTimes(checks.length);
  const card = nodes(tree).find(node => typeof node.type === 'function' && node.type.name === 'RecordingCard');
  const actions = kind === 'grid' ? render(card.type, card.props) : tree;
  for (const [key, callback] of [['recordings.play', 'playRecording'], [kind === 'grid' ? 'recordings.download' : 'common.download', 'downloadRecording'], ['common.delete', 'deleteRecording']]) {
    const button = nodes(actions).find(node => node.type === 'button' && node.props.title?.startsWith(t(key))); name(button, key); button.props.onClick(); expect(props[callback]).toHaveBeenCalledTimes(1); expect(props[callback]).toHaveBeenCalledWith(recording);
  }
  const link = nodes(actions).find(node => node.type === 'a' && node.props.title === t('live.viewInTimeline')); name(link, 'live.viewInTimeline'); expect(link.props.href).toContain('timeline.html?'); expect(link.props.href).toContain('Fixture%20camera');
});

async function openBrowser() {
  const { chromium } = require('../../../node_modules/playwright'); const { build } = require('esbuild');
  const path = require('node:path'); const fs = require('node:fs'); const webRoot = path.resolve(__dirname, '../..');
  const result = await build({ stdin: { resolveDir: webRoot, loader: 'jsx', contents: `
    import { h, render } from 'preact'; import { useState } from 'preact/hooks';
    import { LiveBuildingPlan } from './js/components/preact/live/LiveBuildingPlan.jsx';
    import { LiveOperatorNavigator } from './js/components/preact/live/LiveOperatorNavigator.jsx';
    import { PlanSketchInspector } from './js/components/preact/live/PlanSketchControls.jsx';
    import { FiltersSidebar } from './js/components/preact/recordings/FiltersSidebar.jsx';
    import { RecordingsGrid } from './js/components/preact/recordings/RecordingsGrid.jsx';
    import { RecordingsTable } from './js/components/preact/recordings/RecordingsTable.jsx';
    const t = key => 'translated:' + key; const root = document.querySelector('#root');
    const streams = [${JSON.stringify(stream)}]; const plan = ${JSON.stringify(plan)};
    window.fixtureQuery = ({ queryKey }) => ({ data: queryKey[0] === 'operator-floor-plans' ? { plans: [plan], can_modify: true } : {}, isLoading: false });
    window.fixtureJSON = async (url, options) => { window.calls.requests.push({ url, options }); return plan; };
    window.fetch = async () => ({ ok: true, json: async () => ({}) });
    function Host({ kind, options }) {
      const [selected, setSelected] = useState({}); const [hidden, setHidden] = useState({});
      const [shape, setShape] = useState({ id: 'rect', type: 'rect', tone: 'blue', x: 0.1, y: 0.2, w: 0.3, h: 0.4, label: 'Lobby', filled: true });
      const [filters, setFilters] = useState(${JSON.stringify(filters)});
      if (kind === 'plan') return h(LiveBuildingPlan, { streams, selectedPlanUuid: 'plan', onSelectPlan: () => {}, onOpenCamera: () => window.calls.open++ });
      if (kind === 'navigator') return h(LiveOperatorNavigator, { streams, orderedStreams: streams, onFilterChange: () => {}, onApplyLayout: value => window.calls.layouts.push(value), onSurfaceChange: () => {} });
      if (kind === 'sketch') return h(PlanSketchInspector, { uid: 'sketch', shapes: [shape], selectedShape: shape, t,
        onChange: value => { setShape(value); window.calls.shape = value; }, onSelect: () => {}, onDelete: () => {}, onDuplicate: () => {}, onDeselect: () => {} });
      if (kind === 'filters') return h(FiltersSidebar, { filters, setFilters: update => setFilters(old => { const value = update(old); window.calls.filters = value; return value; }),
        streams: [], pagination: { pageSize: 20 }, setPagination: () => {}, toggleCollapsed: () => window.calls.collapse++, handleDateRangeChange: () => {} });
      return h(kind === 'grid' ? RecordingsGrid : RecordingsTable, { recordings: [${JSON.stringify(recording)}], collapsed: true,
        toggleCollapsed: () => window.calls.collapse++, selectedRecordings: selected, selectAll: Boolean(selected[7]), getSelectedCount: () => selected[7] ? 1 : 0,
        toggleRecordingSelection: id => { window.calls.select++; setSelected(old => ({ ...old, [id]: !old[id] })); },
        toggleSelectAll: () => { window.calls.all++; setSelected(old => ({ 7: !old[7] })); }, clearSelections: () => setSelected({}),
        pagination: { totalItems: 1 }, canDelete: !options.readonly, thumbnailsPerRecording: 1, hiddenColumns: hidden,
        toggleColumn: key => { window.calls.columns.push(key); setHidden(old => ({ ...old, [key]: !old[key] })); },
        playRecording: value => window.calls.play.push(value.id), downloadRecording: value => window.calls.download.push(value.id),
        deleteRecording: value => window.calls.delete.push(value.id), toggleProtection: () => {}, sortBy: () => {}, openDeleteModal: () => {}, openDownloadModal: () => {} });
    }
    window.mount = (kind, options = {}) => { render(null, root); localStorage.clear(); window.calls = { open: 0, requests: [], layouts: [], collapse: 0, play: [], download: [], delete: [], columns: [], select: 0, all: 0 }; render(h(Host, { kind, options }), root); };
  ` }, bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact', plugins: [{ name: 'fixtures', setup(builder) {
    builder.onResolve({ filter: /(?:i18n|query-client|request-queue|ToastContainer)\.(?:js|jsx)$/ }, args => ({ path: args.path, namespace: 'fixture' }));
    builder.onLoad({ filter: /.*/, namespace: 'fixture' }, () => ({ loader: 'js', contents: `export const t = key => 'translated:' + key; export const useI18n = () => ({ t });
      export const fetchJSON = (url, options) => window.fixtureJSON(url, options); export const useQuery = options => window.fixtureQuery(options);
      export const useQueryClient = () => ({ invalidateQueries: () => {} }); export const useMutation = () => ({});
      export const queueThumbnailLoad = async () => {}; export const invalidateThumbnailLoad = () => {}; export const Priority = {}; export const showStatusMessage = () => {};` }));
  } }] });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } }); page.setDefaultTimeout(4000);
    const errors = []; page.on('pageerror', error => errors.push(error.message));
    await page.route('http://fixture.invalid/**', route => route.fulfill({ contentType: 'text/html', body: '<div id="root"></div>' }));
    await page.goto('http://fixture.invalid/');
    const styles = new Set(); for (const file of ['index.html', 'recordings.html']) {
      const html = fs.readFileSync(path.join(webRoot, 'dist', file), 'utf8');
      for (const tag of html.match(/<link[^>]*rel="stylesheet"[^>]*>/g) || []) styles.add(tag.match(/href="([^"]+)"/)[1]);
    }
    for (const href of styles) await page.addStyleTag({ content: fs.readFileSync(path.join(webRoot, 'dist', href), 'utf8') });
    await page.addScriptTag({ content: result.outputFiles[0].text }); return { browser, page, errors };
  } catch (error) { await browser.close(); throw error; }
}
const browserTest = process.env.T7_171_BROWSER === '1' ? it : it.skip;
async function removeTitles(page) { await page.locator('[title]').evaluateAll(elements => elements.forEach(element => element.removeAttribute('title'))); }
async function settle(page) { await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(resolve, 0))))); }
browserTest('plan and sketch names preserve numeric edits, slider keys, saved geometry and tone activation', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('plan')); await settle(page);
    await page.getByRole('button', { name: t('live.plan.editLayout'), exact: true }).click();
    await page.getByRole('spinbutton', { name: t('live.plan.width'), exact: true }).fill('1500');
    await page.getByRole('spinbutton', { name: t('live.plan.height'), exact: true }).fill('900');
    await page.locator('g.live-plan-camera').click();
    const direction = page.getByRole('slider', { name: t('live.plan.direction'), exact: true }); await direction.press('ArrowRight'); expect(await direction.inputValue()).toBe('1');
    const fov = page.getByRole('slider', { name: t('live.plan.fov'), exact: true }); await fov.press('ArrowLeft'); expect(await fov.inputValue()).toBe('69');
    await page.getByRole('button', { name: t('live.plan.save'), exact: true }).click();
    const requests = await page.evaluate(() => window.calls.requests); expect(requests).toHaveLength(1);
    const payload = JSON.parse(requests[0].options.body); expect(payload).toMatchObject({ canvas_width: 1500, canvas_height: 900 }); expect(payload.cameras[0]).toMatchObject({ rotation: 1, fov: 69 });
    await page.evaluate(() => window.mount('sketch')); await removeTitles(page);
    await page.getByRole('button', { name: t('live.plan.sketch.tone.green'), exact: true }).press('Enter');
    await page.getByRole('textbox', { name: t('live.plan.sketch.labelText'), exact: true }).fill('Entry');
    await page.getByRole('checkbox', { name: t('live.plan.sketch.filled'), exact: true }).uncheck();
    const left = page.getByRole('spinbutton', { name: t('live.plan.sketch.left'), exact: true }); await left.fill('25'); await left.press('Tab');
    expect(await page.evaluate(() => window.calls.shape)).toMatchObject({ tone: 'green', label: 'Entry', filled: false, x: 0.25 });
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('navigator names preserve search, native camera action and drag payload', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('navigator')); await removeTitles(page);
    const search = page.getByRole('searchbox', { name: t('live.navigator.search'), exact: true }); await search.fill('missing'); expect(await page.locator('.live-camera-row').count()).toBe(0); await search.fill('Fixture');
    const camera = page.getByRole('button', { name: 'Fixture camera East', exact: true }); await camera.press('Enter'); await camera.press('Space');
    expect(await page.evaluate(() => window.calls.layouts)).toEqual([{ cameraUuids: ['camera'], columns: 1, rows: 1 }, { cameraUuids: ['camera'], columns: 1, rows: 1 }]);
    const transfer = await page.evaluateHandle(() => new DataTransfer()); await camera.dispatchEvent('dragstart', { dataTransfer: transfer });
    expect(await transfer.evaluate(value => value.getData('application/x-lightnvr-camera'))).toBe('camera');
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('custom date/time fields retain independent values and named hide action', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('filters')); await removeTitles(page);
    for (const [id, label, value] of [['start-date', 'start', '2026-10-02'], ['start-time', 'start', '03:15'], ['end-date', 'end', '2026-10-30'], ['end-time', 'end', '20:45']]) {
      const field = page.locator('#' + id); expect(await field.getAttribute('aria-label')).toBe(t('recordings.' + label));
      expect(await page.getByLabel(t('recordings.' + label), { exact: true }).count()).toBe(2); await field.fill(value); await field.press('Tab');
    }
    expect(await page.evaluate(() => window.calls.filters)).toMatchObject({ startDate: '2026-10-02', startTime: '03:15', endDate: '2026-10-30', endTime: '20:45' });
    await page.getByRole('button', { name: t('recordings.hideFilters'), exact: true }).press('Space'); expect(await page.evaluate(() => window.calls.collapse)).toBe(1);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest.each(['grid', 'table'])('%s named actions remain independent from selection and configuration', async kind => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(kind => window.mount(kind), kind); await removeTitles(page);
    const play = page.getByRole('button', { name: t('recordings.play'), exact: true }); await play.press('Enter'); await play.press('Space');
    await page.getByRole('button', { name: t(kind === 'grid' ? 'recordings.download' : 'common.download'), exact: true }).click();
    await page.getByRole('button', { name: t('common.delete'), exact: true }).press('Space');
    expect(await page.evaluate(() => [window.calls.play, window.calls.download, window.calls.delete, window.calls.select])).toEqual([[7, 7], [7], [7], 0]);
    const link = page.getByRole('link', { name: t('live.viewInTimeline'), exact: true });
    expect(await link.getAttribute('href')).toContain('timeline.html?'); expect(await link.getAttribute('href')).toContain('Fixture%20camera');
    await page.getByRole('button', { name: t('recordings.showFilters'), exact: true }).press('Enter'); expect(await page.evaluate(() => window.calls.collapse)).toBe(1);
    if (kind === 'grid') await page.getByRole('button', { name: t('streams.select'), exact: true }).click();
    const all = page.getByRole('checkbox', { name: t('recordings.selectAll'), exact: true }); await all.press('Space'); expect(await all.isChecked()).toBe(true); expect(await page.evaluate(() => window.calls.all)).toBe(1);
    // Table gear is outside the reported bucket; its existing title was removed.
    const gear = kind === 'grid' ? page.getByRole('button', { name: t('recordings.configureElements'), exact: true }) : page.locator('.relative.inline-block > button[type=button]');
    await gear.click(); const streamToggle = page.getByRole('checkbox', { name: t(kind === 'grid' ? 'recordings.columnStreamName' : 'live.stream'), exact: true }); await streamToggle.uncheck();
    expect(await page.evaluate(() => window.calls.columns)).toEqual(['stream']);
    await page.evaluate(kind => window.mount(kind, { readonly: true }), kind); await removeTitles(page); expect(await page.getByRole('button', { name: t('common.delete'), exact: true }).count()).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
