/**
 * Keyboard access in a real browser: Escape on dialogs, the recording schedule
 * grid, Live navigator expansion, zone rows, and recording thumbnails.
 * Run with: npm --prefix web run test:e2e -- --testPathPatterns='keyboardAccess\.browser\.spec\.js$'
 */
const path = require('node:path');
const { fleetLocations } = require('./fleetFixtures');

const FIXTURE = `
  import { h, render, Fragment } from 'preact';
  import { useState } from 'preact/hooks';
  import { QueryClient, QueryClientProvider } from './js/query-client.ts';
  import { useEscapeKey } from './js/hooks/useEscapeKey.ts';
  import { ConfirmDialog } from './js/components/preact/UI.tsx';
  import { ClearLogsModal } from './js/components/preact/system/ClearLogsModal.tsx';
  import { StreamBulkActionModal } from './js/components/preact/StreamBulkActionModal.tsx';
  import { RecordingScheduleGrid } from './js/components/preact/StreamConfigModal.tsx';
  import { LiveOperatorNavigator } from './js/components/preact/live/LiveOperatorNavigator.tsx';
  import { RecordingsGrid } from './js/components/preact/recordings/RecordingsGrid.tsx';

  const root = document.getElementById('root');
  window.calls = { closed: [], played: [], applied: [] };

  function Layer({ name, onEscape, active }) {
    useEscapeKey(onEscape, active);
    return h('div', { 'data-layer': name }, name);
  }
  function Layers() {
    const [open, setOpen] = useState({ outer: true, inner: false });
    const [confirm, setConfirm] = useState(false);
    window.openInner = () => setOpen((o) => ({ ...o, inner: true }));
    window.openConfirm = () => setConfirm(true);
    return h(Fragment, null,
      open.outer && h(Layer, { name: 'outer', onEscape: () => setOpen((o) => ({ ...o, outer: false })) }),
      open.inner && h(Layer, { name: 'inner', onEscape: () => setOpen((o) => ({ ...o, inner: false })) }),
      h(ConfirmDialog, { isOpen: confirm, title: 'Confirm', message: 'Sure?', onClose: () => setConfirm(false), onConfirm: () => {} }));
  }
  function Grid() {
    const [schedule, setSchedule] = useState(() => new Array(168).fill(false));
    window.schedule = schedule;
    return h(RecordingScheduleGrid, { schedule, onChange: setSchedule });
  }
  const json = (body) => new Response(JSON.stringify(body), { status: 200, headers: { 'Content-Type': 'application/json' } });
  window.fetch = async (url) => {
    if (String(url) === '/api/locations') {
      return json(${JSON.stringify(fleetLocations([
        { uuid: '00000000-0000-4000-8000-000000000001', name: 'Orchard', child_count: 1 },
        { uuid: '00000000-0000-4000-8000-000000000002', name: 'Packing', parent_uuid: '00000000-0000-4000-8000-000000000001', type: 'building' },
      ]))});
    }
    return json({});
  };
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
  const recording = { id: 7, stream: 'Gate', start_time: 1700000000, end_time: 1700000060, size: 10, duration: 60, complete: true };
  const noop = () => {};
  const mounts = {
    clearLogs: () => h(ClearLogsModal, { isOpen: true, onClose: () => window.calls.closed.push('clearLogs'), onConfirm: noop }),
    bulkIdle: () => h(StreamBulkActionModal, { action: 'disable', streamNames: ['Gate'], onClose: () => window.calls.closed.push('bulk'), onConfirm: noop, isWorking: false }),
    bulkBusy: () => h(StreamBulkActionModal, { action: 'disable', streamNames: ['Gate'], onClose: () => window.calls.closed.push('bulk'), onConfirm: noop, isWorking: true }),
    layers: () => h(Layers),
    grid: () => h(Grid),
    navigator: () => h(QueryClientProvider, { client }, h(LiveOperatorNavigator, {
      streams: [], orderedStreams: [], columns: 2, rows: 1, operatorSurface: 'grid', selectedPlanUuid: '',
      onFilterChange: noop, onSurfaceChange: noop, onSelectPlan: noop, onApplyLayout: layout => window.calls.applied.push(layout),
    })),
    recordings: (hiddenColumns) => h(RecordingsGrid, {
      collapsed: false, toggleCollapsed: noop, recordings: [recording], selectedRecordings: {}, toggleRecordingSelection: noop,
      selectAll: false, toggleSelectAll: noop, getSelectedCount: () => 0, openDeleteModal: noop,
      playRecording: (r) => window.calls.played.push(r.id), downloadRecording: noop, deleteRecording: noop,
      pagination: { totalItems: 1 }, hiddenColumns,
    }),
  };
  window.mount = (name, arg) => render(mounts[name](arg), root);
  window.unmount = () => render(null, root);
`;

let browser;
let page;

beforeAll(async () => {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const output = await build({
    stdin: { resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx', contents: FIXTURE },
    bundle: true,
    write: false,
    format: 'iife',
    jsx: 'automatic',
    jsxImportSource: 'preact',
    loader: { '.js': 'jsx' },
    alias: { react: 'preact/compat', 'react-dom': 'preact/compat' },
    plugins: [{
      name: 'fixture',
      setup(builder) {
        builder.onResolve({ filter: /(?:\/i18n|ToastContainer)\.tsx?$/ }, (args) => ({ path: args.path, namespace: 'fixture' }));
        builder.onLoad({ filter: /.*/, namespace: 'fixture' }, () => ({
          contents: "export const t=(key,params)=>params?key+JSON.stringify(params):key;export const useI18n=()=>({t});export const showStatusMessage=()=>{};export const showSuccessToast=()=>{};export const showErrorToast=()=>{};export const showWarningToast=()=>{};export const showInfoToast=()=>{};export const getTextDirection=()=>'ltr';",
          loader: 'js',
        }));
      },
    }],
  });
  browser = await chromium.launch({ headless: true });
  page = await browser.newPage({ viewport: { width: 1200, height: 1000 } });
  page.setDefaultTimeout(5000);
  await page.route('http://fixture.invalid/**', (route) => route.fulfill({ contentType: 'text/html', body: '<div id="root"></div>' }));
  await page.goto('http://fixture.invalid/');
  await page.addScriptTag({ content: output.outputFiles[0].text });
}, 60000);

afterAll(async () => { if (browser) await browser.close(); });

// Preact runs effects after paint, and dialogs register their key handling there.
const settle = () => page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => setTimeout(resolve))));
const mount = async (name, arg) => {
  await page.evaluate(() => {
    window.calls = { closed: [], played: [], applied: [] };
    document.documentElement.dir = 'ltr';
    window.unmount();
  });
  await page.evaluate(([n, a]) => window.mount(n, a), [name, arg]);
  await settle();
};
const pressed = (locator) => locator.evaluate((el) => el.getAttribute('aria-checked'));
const isFocused = (locator) => locator.evaluate((el) => el === document.activeElement);
const closedCalls = () => page.evaluate(() => window.calls.closed);

describe('Escape on dialogs', () => {
  it('closes a simple dialog', async () => {
    await mount('clearLogs');
    await page.keyboard.press('Escape');
    expect(await closedCalls()).toEqual(['clearLogs']);
  });

  it('closes an idle dialog but not one with work in progress', async () => {
    await mount('bulkIdle');
    await page.keyboard.press('Escape');
    expect(await closedCalls()).toEqual(['bulk']);
    await mount('bulkBusy');
    await page.keyboard.press('Escape');
    expect(await closedCalls()).toEqual([]);
  });

  it('closes only the topmost layer, and defers to a nested ModalDialog', async () => {
    await mount('layers');
    await page.evaluate(() => window.openInner());
    await settle();
    await page.evaluate(() => window.openConfirm());
    await page.getByRole('dialog').waitFor();
    await settle();
    await page.keyboard.press('Escape');
    await page.getByRole('dialog').waitFor({ state: 'detached' });
    await settle();
    expect(await page.locator('[data-layer]').count()).toBe(2);
    await page.keyboard.press('Escape');
    await page.locator('[data-layer="inner"]').waitFor({ state: 'detached' });
    await settle();
    expect(await page.locator('[data-layer="outer"]').count()).toBe(1);
    await page.keyboard.press('Escape');
    await page.locator('[data-layer="outer"]').waitFor({ state: 'detached' });
  });
});

describe('recording schedule grid', () => {
  const cells = () => page.getByRole('checkbox');
  const tabbable = () => page.locator('[role=checkbox][tabindex="0"]');

  it('exposes one tab stop and named checkbox cells', async () => {
    await mount('grid');
    expect(await cells().count()).toBe(168);
    expect(await tabbable().count()).toBe(1);
    expect(await cells().first().getAttribute('aria-label')).toBe('streamsConfig.sunShort 12am');
  });

  it('moves with arrow keys and toggles with Space and Enter', async () => {
    await mount('grid');
    const start = tabbable();
    await start.focus();
    await page.keyboard.press('ArrowRight');
    await page.keyboard.press('ArrowDown');
    const monday1am = page.getByRole('checkbox', { name: 'streamsConfig.monShort 1am', exact: true });
    expect(await isFocused(monday1am)).toBe(true);
    expect(await tabbable().count()).toBe(1);
    expect(await pressed(monday1am)).toBe('false');
    await page.keyboard.press('Space');
    expect(await pressed(monday1am)).toBe('true');
    expect(await page.evaluate(() => window.schedule[25])).toBe(true);
    await page.keyboard.press('Enter');
    expect(await pressed(monday1am)).toBe('false');
    expect(await isFocused(monday1am)).toBe(true);
  });

  it('stops at the edges and follows visual direction in RTL', async () => {
    await mount('grid');
    const first = page.getByRole('checkbox', { name: 'streamsConfig.sunShort 12am', exact: true });
    await first.focus();
    await page.keyboard.press('ArrowLeft');
    await page.keyboard.press('ArrowUp');
    expect(await isFocused(first)).toBe(true);
    await page.evaluate(() => { document.documentElement.dir = 'rtl'; });
    await page.keyboard.press('ArrowLeft');
    expect(await isFocused(page.getByRole('checkbox', { name: 'streamsConfig.monShort 12am', exact: true }))).toBe(true);
  });
});

describe('Live navigator location tree', () => {
  it('uses the same row button for Enter, Space, label and disclosure activation', async () => {
    await mount('navigator');
    const row = page.getByRole('button', { name: /Orchard/ });
    const child = page.getByRole('button', { name: /Packing/ });
    await child.waitFor();
    await row.focus();
    await page.keyboard.press('Enter');
    await child.waitFor({ state: 'detached' });
    expect(await row.getAttribute('aria-expanded')).toBe('false');
    expect(await row.evaluate(el => el.classList.contains('is-selected'))).toBe(true);
    await page.keyboard.press('Space');
    await child.waitFor();
    expect(await row.getAttribute('aria-expanded')).toBe('true');

    // Clicking the decorative caret bubbles once to the row button.
    await row.locator('.live-location-name > span').first().click();
    await child.waitFor({ state: 'detached' });
    expect(await row.getAttribute('aria-expanded')).toBe('false');
    await row.getByText('Orchard', { exact: true }).click();
    await child.waitFor();
    expect(await row.getAttribute('aria-expanded')).toBe('true');

    await child.click();
    expect(await child.getAttribute('aria-expanded')).toBeNull();
    expect(await child.evaluate(el => el.classList.contains('is-selected'))).toBe(true);
    expect(await row.getAttribute('aria-expanded')).toBe('true');
    await row.dblclick();
    expect(await page.evaluate(() => window.calls.applied)).toEqual([{ cameraUuids: [], columns: 2, rows: 1 }]);
    expect(await row.getAttribute('aria-expanded')).toBe('true');
  });

  it('expands and collapses from the keyboard, mirrored in RTL', async () => {
    await mount('navigator');
    const row = page.locator('.live-location-row', { hasText: 'Orchard' });
    const child = page.locator('.live-location-row', { hasText: 'Packing' });
    await row.waitFor();
    await child.waitFor();
    await row.focus();
    expect(await row.getAttribute('aria-expanded')).toBe('true');
    await page.keyboard.press('ArrowLeft');
    await child.waitFor({ state: 'detached' });
    await settle();
    // Collapsing the only open location must stick rather than re-expanding.
    expect(await row.getAttribute('aria-expanded')).toBe('false');
    expect(await child.count()).toBe(0);
    await page.keyboard.press('ArrowRight');
    await child.waitFor();
    await settle();
    expect(await row.getAttribute('aria-expanded')).toBe('true');
    expect(await isFocused(row)).toBe(true);
    await page.evaluate(() => { document.documentElement.dir = 'rtl'; });
    await page.keyboard.press('ArrowRight');
    await child.waitFor({ state: 'detached' });
    await settle();
    expect(await child.count()).toBe(0);
  });
});

describe('recording thumbnails', () => {
  const playButtons = () => page.getByRole('button', { name: 'recordings.play', exact: true });

  it('offers a keyboard Play button when the actions row is hidden', async () => {
    await mount('recordings', { actions: true });
    expect(await playButtons().count()).toBe(1);
    await playButtons().focus();
    await page.keyboard.press('Enter');
    expect(await page.evaluate(() => window.calls.played)).toEqual([7]);
  });

  it('has exactly one Play button when the actions row is shown', async () => {
    await mount('recordings', {});
    expect(await playButtons().count()).toBe(1);
  });
});
