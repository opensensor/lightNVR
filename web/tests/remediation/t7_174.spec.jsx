/** Explicit control names mirror existing localized labels. Browser checks cover
 * computed accessibility, native activation and actual component callbacks. */
const mockHooks = { cursor: 0, values: {}, setters: [] };
const mockTimeline = { selectedDate: '2026-10-31', timelineStartHour: 6, timelineEndHour: 18, setState: jest.fn() };
jest.mock('preact/hooks', () => ({
  useState: initial => {
    const index = mockHooks.cursor++; const setter = jest.fn(); mockHooks.setters[index] = setter;
    return [Object.hasOwn(mockHooks.values, index) ? mockHooks.values[index] : typeof initial === 'function' ? initial() : initial, setter];
  }, useId: () => 'fixture', useEffect: jest.fn(), useCallback: callback => callback, useMemo: factory => factory(), useRef: current => ({ current }),
}));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: key => `translated:${key}` }) }));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn() }));
jest.mock('../../js/fetch-utils.js', () => ({ enhancedFetch: jest.fn() }));
jest.mock('../../js/components/preact/timeline/TimelinePage.jsx', () => ({ timelineState: mockTimeline }));
jest.mock('../../js/components/preact/UI.jsx', () => ({ ConfirmDialog: () => null }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
const { TimelineControls } = require('../../js/components/preact/timeline/TimelineControls.jsx');
const { TimelinePlayer } = require('../../js/components/preact/timeline/TimelinePlayer.jsx');
const { AddUserModal } = require('../../js/components/preact/users/AddUserModal.jsx');
const { EditUserModal } = require('../../js/components/preact/users/EditUserModal.jsx');
const { ApiKeyModal } = require('../../js/components/preact/users/ApiKeyModal.jsx');
const { AuditHistoryModal } = require('../../js/components/preact/users/AuditHistoryModal.jsx');
function nodes(node) {
  if (Array.isArray(node)) return node.flatMap(nodes);
  if (!node || typeof node !== 'object') return [];
  return [node, ...nodes(node.props?.children)];
}
function render(Component, props = {}, values = {}) {
  mockHooks.cursor = 0; mockHooks.values = values; mockHooks.setters = [];
  return Component(props);
}
function named(node, name) { expect(node).toBeDefined(); expect(node.props['aria-label']).toBe(name); }
beforeEach(() => jest.clearAllMocks());
it.each([['add', AddUserModal], ['edit', EditUserModal]])('%s reported form fields retain localized names and input callbacks', (kind, Component) => {
  const onInput = jest.fn();
  const tree = render(Component, { currentUser: { username: 'fixture' }, formData: {}, handleInputChange: onInput });
  const fields = nodes(tree).filter(node => ['input', 'textarea'].includes(node.type));
  const names = ['fields.username', kind === 'add' ? 'fields.password' : 'users.passwordKeepCurrent', 'fields.email', 'users.active', 'users.lockPasswordChanges', 'users.allowedLoginIpRanges'];
  expect(fields).toHaveLength(6);
  fields.forEach((field, index) => {
    named(field, 'translated:' + names[index]);
    const event = { target: { name: field.props.name, value: 'fixture', checked: true } };
    (field.props.onChange || field.props.oninput)(event);
    expect(onInput).toHaveBeenLastCalledWith(event);
  });
  expect(onInput).toHaveBeenCalledTimes(6);
});
it('API token controls expose visible naming text without changing action/confirmation callbacks', () => {
  const action = { key: 'recordings:view', description: 'Read recordings', category: 'recordings', enforced: true, destructive: true };
  const token = { uuid: 'old', description: 'Old token', expires_at: 1 };
  let tree = render(ApiKeyModal, { currentUser: { id: 7 }, getAuthHeaders: () => ({}) }, { 0: false, 2: [token], 3: [action] });
  const description = nodes(tree).find(node => node.type === 'input' && node.props.maxLength === '127');
  named(description, 'translated:tokens.name');
  const inactive = nodes(tree).find(node => node.type === 'input' && node.props.type === 'checkbox');
  named(inactive, 'translated:tokens.showInactive'); inactive.props.onChange({ currentTarget: { checked: true } });
  expect(mockHooks.setters[10]).toHaveBeenCalledWith(true);
  const picker = nodes(tree).find(node => typeof node.type === 'function' && node.type.name === 'ActionPicker');
  const actionTree = picker.type(picker.props);
  const checkbox = nodes(actionTree).find(node => node.type === 'input');
  named(checkbox, 'recordings:view translated:tokens.destructive Read recordings');
  checkbox.props.onChange(); expect(mockHooks.setters[7]).toHaveBeenCalledTimes(1);
  tree = render(ApiKeyModal, { currentUser: { id: 7 }, getAuthHeaders: () => ({}) }, { 12: { kind: 'legacy', secret: 'synthetic' } });
  const reveal = nodes(tree).find(node => typeof node.type === 'function' && node.type.name === 'SecretReveal');
  const revealTree = reveal.type(reveal.props);
  const saved = nodes(revealTree).find(node => node.type === 'input' && node.props.type === 'checkbox');
  named(saved, 'translated:tokens.secret.savedConfirm translated:tokens.secret.savedHelp');
  saved.props.onChange({ currentTarget: { checked: true } }); expect(mockHooks.setters[14]).toHaveBeenCalledWith(true);
});
it('all six reported audit inputs retain their visible localized names and updates', () => {
  const tree = render(AuditHistoryModal, { users: [], getAuthHeaders: () => ({}) });
  const fields = nodes(tree).filter(node => node.type === 'input');
  const names = ['audit.retentionDays', 'audit.action', 'audit.since', 'audit.until', 'audit.requestId', 'audit.targetUuid'];
  expect(fields).toHaveLength(6);
  fields.forEach((field, index) => {
    named(field, 'translated:' + names[index]);
    (field.props.onInput || field.props.oninput)({ currentTarget: { value: 'fixture' } });
  });
  expect(mockHooks.setters[9]).toHaveBeenCalledWith('fixture'); expect(mockHooks.setters[0]).toHaveBeenCalledTimes(5);
});
it('zoom controls have localized names independent of titles and keep range callbacks', () => {
  const tree = render(TimelineControls);
  for (const [id, key] of [['zoom-in-button', 'timeline.zoomIn'], ['zoom-out-button', 'timeline.zoomOut']]) {
    const button = nodes(tree).find(node => node.props?.id === id);
    named(button, 'translated:' + key); expect(button.props.onKeyDown).toBeUndefined(); button.props.onClick();
  }
  expect(mockTimeline.setState).toHaveBeenCalledTimes(2);
  for (const [change] of mockTimeline.setState.mock.calls) {
    expect(Number.isFinite(change.timelineStartHour)).toBe(true); expect(Number.isFinite(change.timelineEndHour)).toBe(true);
  }
});
it.each([false, true])('player names native playback and only exposes the enabled detection graphic (overlay=%s)', enabled => {
  const tree = render(TimelinePlayer, {}, { 3: enabled });
  named(nodes(tree).find(node => node.type === 'video'), 'translated:timeline.timelinePlayback');
  const toggle = nodes(tree).find(node => node.type === 'input' && node.props.id === 'timeline-detection-overlay');
  named(toggle, 'translated:recordings.detections'); toggle.props.onChange({ target: { checked: !enabled } });
  expect(mockHooks.setters[3]).toHaveBeenCalledWith(!enabled);
  const images = nodes(tree).filter(node => node.props?.role === 'img');
  expect(images).toHaveLength(enabled ? 1 : 0);
  if (enabled) {
    named(images[0], 'translated:recordings.detections');
    const canvas = images[0].props.children;
    expect(canvas.type).toBe('canvas'); expect(canvas.props['aria-hidden']).toBe('true'); expect(canvas.ref).toBeDefined();
    expect(images[0].props.className).toContain('pointer-events-none'); expect(images[0].props.style.zIndex).toBe(2);
  }
});
it('EPTZ keeps the detection toggle disabled and does not expose a nonexistent detection image', () => {
  const tree = render(TimelinePlayer, { streamConfig: { eptz_config: { version: 1, projection: 'equidistant', mount: 'ceiling' } } }, { 3: true });
  expect(nodes(tree).find(node => node.props?.id === 'timeline-detection-overlay').props.disabled).toBe(true);
  expect(nodes(tree).filter(node => node.props?.role === 'img')).toHaveLength(0);
});

async function openBrowser() {
  const { chromium } = require('../../../node_modules/playwright'); const { build } = require('esbuild');
  const path = require('node:path'); const fs = require('node:fs');
  const webRoot = path.resolve(__dirname, '../..');
  const result = await build({
    stdin: { resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx', contents: `
      import { h, render } from 'preact'; import { useState } from 'preact/hooks';
      import { TimelineControls } from './js/components/preact/timeline/TimelineControls.jsx';
      import { TimelinePlayer } from './js/components/preact/timeline/TimelinePlayer.jsx';
      import { AddUserModal } from './js/components/preact/users/AddUserModal.jsx';
      import { EditUserModal } from './js/components/preact/users/EditUserModal.jsx';
      import { ApiKeyModal } from './js/components/preact/users/ApiKeyModal.jsx';
      import { AuditHistoryModal } from './js/components/preact/users/AuditHistoryModal.jsx';
      const root = document.querySelector('#root'); const headers = () => ({});
      const action = { key: 'recordings:view', description: 'Read recordings', category: 'recordings', enforced: true };
      const oldToken = { uuid: 'old', description: 'Old fixture token', expires_at: 1, actions: ['recordings:view'], scope: { type: 'all' } };
      window.fixtureJSON = async (url, options = {}) => {
        window.calls.urls.push(url);
        if (options.method === 'POST' && url.endsWith('/tokens')) {
          window.calls.create++; window.calls.payload = JSON.parse(options.body);
          return { token: { ...oldToken, uuid: 'new', description: 'Fixture token', expires_at: 2000000000 }, secret: 'synthetic' };
        }
        if (url.endsWith('/actions')) return { actions: [action] };
        if (url.endsWith('/tokens')) return { tokens: [oldToken] };
        if (url === '/api/audit/settings') {
          if (options.method === 'PUT') { window.calls.save++; window.calls.retention = JSON.parse(options.body).retention_days; }
          return { retention_days: options.method === 'PUT' ? window.calls.retention : 365, allowed_decision_modes: [] };
        }
        if (url.startsWith('/api/audit/events?')) return { count: 0, total: 0, events: [] };
        return { collections: [], locations: [], tags: [] };
      };
      const subscribers = new Set();
      window.timelineFixture = { selectedDate: '2026-10-31', timelineStartHour: 6, timelineEndHour: 18,
        timelineSegments: [], currentSegmentIndex: -1, currentTime: null, isPlaying: false, playbackSpeed: 1,
        subscribe: listener => { subscribers.add(listener); return () => subscribers.delete(listener); },
        setState(change) { Object.assign(window.timelineFixture, change); window.calls.timeline.push(change);
          subscribers.forEach(listener => listener(window.timelineFixture)); } };
      function FormHost({ kind }) {
        const [formData, setFormData] = useState({ username: kind, password: '', email: '', is_active: false, password_change_locked: false, allowed_login_cidrs: '', role: 1 });
        return h(kind === 'add' ? AddUserModal : EditUserModal, { currentUser: { username: 'fixture' }, formData,
          handleInputChange: event => { window.calls.input[kind]++; setFormData({ ...formData, [event.target.name]: event.target.type === 'checkbox' ? event.target.checked : event.target.value }); },
          onClose: () => window.calls.close++, handleAddUser: () => {}, handleEditUser: () => {} });
      }
      window.mount = kind => {
        render(null, root); window.calls = { urls: [], create: 0, save: 0, close: 0, input: { add: 0, edit: 0 }, timeline: [] };
        if (kind === 'forms') return render(h('div', {}, ['add', 'edit'].map(value => h('section', { 'data-case': value }, h(FormHost, { kind: value })))), root);
        if (kind === 'timeline') return render(h('div', {}, h(TimelinePlayer, {}), h(TimelineControls, {})), root);
        render(h(kind === 'api' ? ApiKeyModal : AuditHistoryModal, { currentUser: { id: 7, username: 'fixture' }, users: [],
          getAuthHeaders: headers, onClose: () => { window.calls.close++; render(null, root); } }), root);
      };
    ` },
    bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact',
    plugins: [{ name: 'test-service', setup(builder) {
      builder.onResolve({ filter: /(?:i18n|query-client|fetch-utils|ToastContainer|TimelinePage)\.(?:js|jsx)$/ }, args => ({ path: args.path, namespace: 'service' }));
      builder.onLoad({ filter: /.*/, namespace: 'service' }, args => ({ loader: 'js', contents: args.path.includes('TimelinePage')
        ? 'export const timelineState = new Proxy({}, { get: (_target, key) => window.timelineFixture[key], set: (_target, key, value) => (window.timelineFixture[key] = value, true) });'
        : `export const t = key => 'translated:' + key; export const useI18n = () => ({ t });
          export const fetchJSON = (url, options) => window.fixtureJSON(url, options);
          export const enhancedFetch = async () => ({ blob: async () => new Blob([]) });
          export const useMutation = () => ({}); export const useQuery = () => ({}); export const useQueryClient = () => ({});
          export const showStatusMessage = () => {};` }));
    } }],
  });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } }); page.setDefaultTimeout(5000);
    const errors = []; page.on('pageerror', error => errors.push(error.message));
    await page.route('http://fixture.invalid/**', route => route.fulfill({ contentType: 'text/html', body: '<div id="root"></div>' }));
    await page.goto('http://fixture.invalid/');
    await page.route('http://fixture.invalid/media-fixture.mp4', route => route.fulfill({ contentType: 'video/mp4',
      body: fs.readFileSync(path.resolve(webRoot, '../tests/integration/fixtures/recording.mp4')) }));
    const styles = new Set();
    for (const file of ['users.html', 'timeline.html']) {
      const html = fs.readFileSync(path.join(webRoot, 'dist', file), 'utf8');
      for (const tag of html.match(/<link[^>]*rel="stylesheet"[^>]*>/g) || []) styles.add(tag.match(/href="([^"]+)"/)[1]);
    }
    expect(styles.size).toBeGreaterThan(0);
    for (const href of styles) await page.addStyleTag({ content: fs.readFileSync(path.join(webRoot, 'dist', href), 'utf8') });
    await page.addScriptTag({ content: result.outputFiles[0].text });
    return { browser, page, errors };
  } catch (error) { await browser.close(); throw error; }
}
const browserTest = process.env.T7_174_BROWSER === '1' ? it : it.skip;
browserTest('reported add/edit fields are named per instance and edits never cross modal boundaries', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('forms'));
    for (const kind of ['add', 'edit']) {
      const scope = page.locator(`[data-case="${kind}"]`);
      const textFields = ['fields.username', kind === 'add' ? 'fields.password' : 'users.passwordKeepCurrent', 'fields.email', 'users.allowedLoginIpRanges'];
      for (const name of textFields) {
        const field = scope.getByLabel('translated:' + name, { exact: true });
        expect(await field.count()).toBe(1); await field.fill(name === 'fields.email' ? `${kind}@example.invalid` : `changed-${kind}`);
      }
      for (const name of ['users.active', 'users.lockPasswordChanges']) {
        const field = scope.getByRole('checkbox', { name: 'translated:' + name, exact: true });
        await field.press('Space'); expect(await field.isChecked()).toBe(true);
      }
    }
    expect(await page.locator('[data-case="add"]').getByRole('textbox', { name: 'translated:fields.username', exact: true }).inputValue()).toBe('changed-add');
    expect(await page.locator('[data-case="edit"]').getByRole('textbox', { name: 'translated:fields.username', exact: true }).inputValue()).toBe('changed-edit');
    expect(await page.evaluate(() => window.calls.input)).toEqual({ add: 6, edit: 6 });
    expect(await page.evaluate(() => window.calls.close)).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('API controls preserve visible accessible names, inactive selection and single token confirmation', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('api'));
    await page.locator('button[title]').evaluateAll(elements => elements.forEach(element => element.removeAttribute('title')));
    const inactive = page.getByRole('checkbox', { name: 'translated:tokens.showInactive', exact: true });
    await inactive.waitFor(); expect(await page.getByText('Old fixture token', { exact: true }).count()).toBe(0);
    await inactive.press('Space'); expect(await page.getByText('Old fixture token', { exact: true }).count()).toBe(1);
    await page.getByRole('textbox', { name: 'translated:tokens.name', exact: true }).fill('Fixture token');
    const action = page.getByRole('checkbox', { name: 'recordings:view Read recordings', exact: true });
    await action.press('Space'); expect(await action.isChecked()).toBe(true);
    await page.getByRole('button', { name: 'translated:tokens.create', exact: true }).press('Enter');
    const saved = page.getByRole('checkbox', { name: 'translated:tokens.secret.savedConfirm translated:tokens.secret.savedHelp', exact: true });
    await saved.waitFor(); expect(await page.evaluate(() => window.calls.create)).toBe(1);
    expect(await page.evaluate(() => window.calls.payload.actions)).toEqual(['recordings:view']);
    expect(await page.evaluate(() => window.calls.payload.description)).toBe('Fixture token');
    const done = page.getByRole('button', { name: 'translated:common.done', exact: true });
    expect(await done.isDisabled()).toBe(true); await saved.press('Space'); expect(await done.isEnabled()).toBe(true);
    await done.press('Enter'); expect(await page.evaluate(() => window.calls.close)).toBe(1);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('six audit input names preserve retention and filter updates', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('audit'));
    await page.getByRole('heading', { name: 'translated:audit.empty', exact: true }).waitFor();
    const retention = page.getByRole('spinbutton', { name: 'translated:audit.retentionDays', exact: true });
    await retention.fill('30'); await page.getByRole('button', { name: 'translated:common.saveChanges', exact: true }).click();
    await page.waitForFunction(() => window.calls.save === 1); expect(await page.evaluate(() => window.calls.retention)).toBe(30);
    const entries = { 'audit.action': 'recordings:view', 'audit.since': '2026-10-30T08:00', 'audit.until': '2026-10-30T09:00', 'audit.requestId': 'request-fixture', 'audit.targetUuid': 'target-fixture' };
    for (const [name, value] of Object.entries(entries)) {
      const field = page.getByLabel('translated:' + name, { exact: true }); expect(await field.count()).toBe(1); await field.fill(value);
    }
    await page.getByRole('button', { name: 'translated:audit.applyFilters', exact: true }).press('Enter');
    await page.waitForFunction(() => window.calls.urls.some(url => url.includes('request-fixture')));
    const query = await page.evaluate(() => window.calls.urls.filter(url => url.includes('request-fixture')).at(-1));
    expect(query).toContain('action=recordings%3Aview'); expect(query).toContain('target-fixture');
    expect(await page.evaluate(() => window.calls.close)).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
browserTest('zoom names survive title removal and media detection names match only the rendered layer', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mount('timeline'));
    await page.locator('button[title]').evaluateAll(elements => elements.forEach(element => element.removeAttribute('title')));
    const zoomIn = page.getByRole('button', { name: 'translated:timeline.zoomIn', exact: true });
    const zoomOut = page.getByRole('button', { name: 'translated:timeline.zoomOut', exact: true });
    await zoomIn.waitFor();
    const before = await page.evaluate(() => window.calls.timeline.filter(change => Object.hasOwn(change, 'timelineStartHour')).length);
    await zoomIn.press('Enter'); await zoomOut.press('Space');
    expect(await page.evaluate(() => window.calls.timeline.filter(change => Object.hasOwn(change, 'timelineStartHour')).length)).toBe(before + 2);
    await page.evaluate(() => window.timelineFixture.setState({ timelineStartHour: 0, timelineEndHour: 24 }));
    await page.waitForFunction(() => document.getElementById('zoom-out-button')?.disabled === true);
    expect(await zoomOut.isDisabled()).toBe(true);
    const video = page.locator('video');
    // An empty video reports Chromium's media-error name instead of its label.
    // Load the existing tiny repository fixture to inspect the playable state.
    await video.evaluate(element => new Promise((resolve, reject) => {
      element.addEventListener('loadeddata', resolve, { once: true });
      element.addEventListener('error', () => reject(Error('media fixture could not load')), { once: true });
      element.src = '/media-fixture.mp4'; element.load();
    }));
    // Chromium exposes native Video in its AX tree; Playwright's ARIA-role
    // serializer omits that native role, so inspect the actual browser node.
    const accessibility = await page.context().newCDPSession(page);
    const { root } = await accessibility.send('DOM.getDocument');
    const { nodeId } = await accessibility.send('DOM.querySelector', { nodeId: root.nodeId, selector: 'video' });
    const { nodes: mediaNodes } = await accessibility.send('Accessibility.getPartialAXTree', { nodeId, fetchRelatives: false });
    expect(mediaNodes[0].ignored).toBe(false);
    expect(mediaNodes[0].name?.value).toBe('translated:timeline.timelinePlayback');
    await accessibility.detach();
    expect(await video.evaluate(element => element.controls)).toBe(true);
    const toggle = page.getByRole('checkbox', { name: 'translated:recordings.detections', exact: true });
    const image = page.getByRole('img', { name: 'translated:recordings.detections', exact: true });
    expect(await image.count()).toBe(0); await toggle.press('Space'); await image.waitFor();
    expect(await image.evaluate(element => getComputedStyle(element).pointerEvents)).toBe('none');
    expect(await image.locator('canvas').getAttribute('aria-hidden')).toBe('true');
    const geometry = await image.evaluate(element => {
      const wrapper = element.getBoundingClientRect(); const canvas = element.querySelector('canvas').getBoundingClientRect();
      const player = document.querySelector('[data-testid="timeline-video-container"]').getBoundingClientRect();
      return { width: canvas.width, height: canvas.height, wrapperWidth: wrapper.width, wrapperHeight: wrapper.height, playerWidth: player.width, playerHeight: player.height };
    });
    expect(geometry.width).toBe(geometry.wrapperWidth); expect(geometry.height).toBe(geometry.wrapperHeight);
    expect(geometry.width).toBe(geometry.playerWidth); expect(geometry.height).toBe(geometry.playerHeight);
    await toggle.press('Space'); expect(await image.count()).toBe(0);
  } finally { await browser.close(); expect(errors).toEqual([]); }
});
