/** Presentation boundaries retain native controls and existing modal guards.
 * Browser supplements mount real Preact and use the built users stylesheet. */
const mockState = { values: {}, cursor: 0 };
jest.mock('preact/hooks', () => ({
  useState: (initial) => {
    const index = mockState.cursor++;
    return [Object.hasOwn(mockState.values, index) ? mockState.values[index]
      : typeof initial === 'function' ? initial() : initial, jest.fn()];
  },
  useId: () => 'fixture', useEffect: jest.fn(), useCallback: (callback) => callback,
  useMemo: (factory) => factory(), useRef: (current) => ({ current }),
}));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn() }));
jest.mock('../../js/fetch-utils.js', () => ({ enhancedFetch: jest.fn() }));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: (key) => `translated:${key}` }) }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
const { AccessPolicyModal } = require('../../js/components/preact/users/AccessPolicyModal.jsx');
const { AddUserModal } = require('../../js/components/preact/users/AddUserModal.jsx');
const { ApiKeyModal } = require('../../js/components/preact/users/ApiKeyModal.jsx');
const { AuditHistoryModal } = require('../../js/components/preact/users/AuditHistoryModal.jsx');
const { DeleteUserModal } = require('../../js/components/preact/users/DeleteUserModal.jsx');
const { EditUserModal } = require('../../js/components/preact/users/EditUserModal.jsx');
const components = [
  ['access', AccessPolicyModal], ['add', AddUserModal], ['api', ApiKeyModal],
  ['audit', AuditHistoryModal], ['delete', DeleteUserModal], ['edit', EditUserModal],
];
function nodes(node) {
  if (Array.isArray(node)) return node.flatMap(nodes);
  if (!node || typeof node !== 'object') return [];
  return [node, ...nodes(node.props?.children)];
}
function fixture(Component, overrides = {}) {
  mockState.cursor = 0; mockState.values = overrides;
  const onClose = jest.fn();
  const user = { id: 7, username: 'fixture' };
  const tree = Component({ user, currentUser: user, onClose, getAuthHeaders: () => ({}), formData: {}, users: [] });
  return { tree, onClose };
}
function eventFor(backdrop, inside = false) {
  return { target: inside ? {} : backdrop, currentTarget: backdrop, stopPropagation: jest.fn() };
}
it.each(components)('%s backdrop is presentation, not a synthetic keyboard action', (kind, Component) => {
  const { tree } = fixture(Component);
  expect(tree.props.role).toBe('presentation');
  expect(tree.props.tabIndex).toBeUndefined();
  expect(tree.props.onKeyDown).toBeUndefined();
  const content = nodes(tree).find((node) => node.props?.role === 'dialog') || tree.props.children;
  if (kind === 'add' || kind === 'edit') {
    expect(typeof content.props.onKeyDown).toBe('function');
  } else {
    expect(content.props.onClick).toBeUndefined();
    expect(content.props.onKeyDown).toBeUndefined();
  }
});
it.each(components)('%s bubbled content clicks cannot dismiss the backdrop', (_kind, Component) => {
  const { tree, onClose } = fixture(Component);
  tree.props.onClick(eventFor({}, true));
  expect(onClose).not.toHaveBeenCalled();
});
it.each(components)('%s direct pointer backdrop click dismisses exactly once', (_kind, Component) => {
  const { tree, onClose } = fixture(Component);
  const event = eventFor({});
  tree.props.onMouseDown?.(event);
  tree.props.onClick(event);
  expect(onClose).toHaveBeenCalledTimes(1);
});
it.each(components)('%s retains native Close or Cancel without a duplicate key handler', (_kind, Component) => {
  const { tree, onClose } = fixture(Component);
  const close = nodes(tree).find((node) => node.type === 'button'
    && ['translated:common.close', 'translated:common.cancel'].includes(node.props.children));
  expect(close).toBeDefined();
  expect(close.props.onKeyDown).toBeUndefined();
  close.props.onClick();
  expect(onClose).toHaveBeenCalledTimes(1);
});
it.each([['add', AddUserModal], ['edit', EditUserModal]])('%s preserves inside-to-outside pointer drag protection', (_kind, Component) => {
  const { tree, onClose } = fixture(Component);
  const backdrop = {};
  tree.props.onMouseDown(eventFor(backdrop, true));
  tree.props.onClick(eventFor(backdrop));
  expect(onClose).not.toHaveBeenCalled();
  tree.props.onMouseDown(eventFor(backdrop));
  tree.props.onClick(eventFor(backdrop));
  tree.props.onClick(eventFor(backdrop)); // pointer-down latch has been consumed
  expect(onClose).toHaveBeenCalledTimes(1);
});
// State slots are component-local, only used to isolate the existing backdrop
// guards here; browser cases below exercise real state transitions and requests.
it.each([
  ['access saving', AccessPolicyModal, { 2: true }],
  ['API creating', ApiKeyModal, { 8: true }],
  ['API generating', ApiKeyModal, { 11: true }],
  ['API revealed', ApiKeyModal, { 12: { kind: 'legacy', secret: 'synthetic' } }],
  ['audit saving', AuditHistoryModal, { 11: true }],
  ['audit exporting', AuditHistoryModal, { 12: true }],
])('%s preserves blocked backdrop dismissal', (_kind, Component, state) => {
  const { tree, onClose } = fixture(Component, state);
  tree.props.onClick(eventFor({}));
  expect(onClose).not.toHaveBeenCalled();
});

async function openBrowser() {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const fs = require('node:fs');
  const path = require('node:path');
  const webRoot = path.resolve(__dirname, '../..');
  const result = await build({
    stdin: { resolveDir: webRoot, loader: 'jsx', contents: `
      import { h, render } from 'preact';
      import { useState } from 'preact/hooks';
      import { AccessPolicyModal } from './js/components/preact/users/AccessPolicyModal.jsx';
      import { AddUserModal } from './js/components/preact/users/AddUserModal.jsx';
      import { ApiKeyModal } from './js/components/preact/users/ApiKeyModal.jsx';
      import { AuditHistoryModal } from './js/components/preact/users/AuditHistoryModal.jsx';
      import { DeleteUserModal } from './js/components/preact/users/DeleteUserModal.jsx';
      import { EditUserModal } from './js/components/preact/users/EditUserModal.jsx';
      const root = document.querySelector('#root');
      const components = { access: AccessPolicyModal, add: AddUserModal, api: ApiKeyModal,
        audit: AuditHistoryModal, delete: DeleteUserModal, edit: EditUserModal };
      const user = { id: 7, username: 'fixture' };
      const role = { uuid: 'role', name: 'Operator', actions: ['recordings:view'] };
      const policy = { policy_version: 1, mode: 'policy', grants: [{ uuid: 'grant', role_uuid: 'role', scope: { type: 'all' } }] };
      const settings = { retention_days: 365, allowed_decision_modes: [] };
      const getAuthHeaders = () => ({});
      window.keyEpoch = 0;
      const add = window.addEventListener.bind(window);
      window.addEventListener = (type, callback, options) => {
        if (type === 'keydown') window.keyEpoch++;
        return add(type, callback, options);
      };
      window.pending = (name, value) => {
        window.calls[name] = (window.calls[name] || 0) + 1;
        return new Promise(resolve => { window.finish[name] = () => resolve(value); });
      };
      window.fixtureJSON = async (url, options = {}) => {
        if (options.method === 'PUT' && url === '/api/audit/settings') return window.pending('save', { retention_days: 30, allowed_decision_modes: [] });
        if (options.method === 'PUT') return window.pending('save', { ...policy, policy_version: 2, grants: [] });
        if (options.method === 'POST' && url.endsWith('/api-key')) return window.pending('generate', { api_key: 'synthetic-fixture-key' });
        if (url.endsWith('/actions')) return { actions: [{ key: 'recordings:view', category: 'recordings', description: 'Read', enforced: true }] };
        if (url.endsWith('/roles')) return { roles: [role] };
        if (url.includes('/api/authorization/users/') && !url.endsWith('/tokens')) return policy;
        if (url === '/api/audit/settings') return settings;
        if (url.startsWith('/api/audit/events?')) return { page: 1, page_size: 50, count: 1, total: 1,
          events: [{ uuid: 'event', action: 'recordings:view', occurred_at: 1700000000, outcome: 'allowed' }] };
        return { tokens: [], collections: [], locations: [], tags: [] };
      };
      window.fixtureExport = () => window.pending('export', { blob: async () => new Blob(['synthetic fixture']) });
      function Host({ kind }) {
        const [formData, setFormData] = useState({ username: 'fixture', password: 'fixture-password',
          email: 'fixture@example.invalid', role: 1, is_active: true, password_change_locked: false, allowed_login_cidrs: '' });
        return h(components[kind], { user, currentUser: user, users: [user], formData, getAuthHeaders,
          onClose: () => { window.calls.close++; render(null, root); },
          handleInputChange: event => setFormData({ ...formData, [event.target.name]: event.target.type === 'checkbox' ? event.target.checked : event.target.value }),
          handleAddUser: () => window.pending('submit'), handleEditUser: () => window.pending('submit'),
          handleDeleteUser: () => window.pending('submit'), handleClearLoginLockout: () => { window.calls.clear++; }
        });
      }
      window.mountModal = kind => {
        render(null, root); window.calls = { close: 0, submit: 0, clear: 0, save: 0, generate: 0, export: 0 }; window.finish = {};
        document.querySelector('#launcher').focus();
        render(h(Host, { kind }), root);
      };
    ` },
    bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact',
    plugins: [{ name: 'test-services', setup(builder) {
      builder.onResolve({ filter: /(?:i18n|query-client|fetch-utils|ToastContainer)\.(?:js|jsx)$/ },
        (args) => ({ path: args.path, namespace: 'test-service' }));
      builder.onLoad({ filter: /.*/, namespace: 'test-service' }, () => ({
        contents: `export const useI18n = () => ({ t: key => 'translated:' + key });
          export const fetchJSON = (url, options) => window.fixtureJSON(url, options);
          export const enhancedFetch = () => window.fixtureExport();
          export const showStatusMessage = () => {};`, loader: 'js',
      }));
    } }],
  });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } });
    page.setDefaultTimeout(5000);
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.setContent('<button id="launcher">Launch</button><div id="root"></div>');
    const html = fs.readFileSync(path.join(webRoot, 'dist/users.html'), 'utf8');
    const links = html.match(/<link[^>]*rel="stylesheet"[^>]*>/g) || [];
    expect(links.length).toBeGreaterThan(0);
    for (const link of links) {
      const href = link.match(/href="([^"]+)"/)[1];
      await page.addStyleTag({ content: fs.readFileSync(path.join(webRoot, 'dist', href), 'utf8') });
    }
    await page.addScriptTag({ content: result.outputFiles[0].text });
    return { browser, page, errors };
  } catch (error) { await browser.close(); throw error; }
}
const browserTest = process.env.T7_164_BROWSER === '1' ? it : it.skip;
async function focusVisible(page, button) {
  const oldShadow = await button.evaluate(element => getComputedStyle(element).boxShadow);
  await page.keyboard.press('Tab');
  await button.focus();
  expect(await button.evaluate((element, previous) => {
    const style = getComputedStyle(element);
    return element.matches(':focus-visible') && document.activeElement === element
      && ((style.outlineStyle !== 'none' && parseFloat(style.outlineWidth) > 0)
        || (style.boxShadow !== 'none' && style.boxShadow !== previous));
  }, oldShadow)).toBe(true);
}
async function outerClick(page) { await page.locator('#root > div').click({ position: { x: 1, y: 1 } }); }
async function flushEffects(page) { await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(resolve, 0))))); }
async function closedCount(page, count) { expect(await page.evaluate(() => window.calls.close)).toBe(count); }

browserTest('native Add/Edit/Delete actions preserve keyboard, pointer, focus and pending behavior', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    for (const kind of ['add', 'edit', 'delete']) {
      await page.evaluate(value => window.mountModal(value), kind);
      const close = page.getByRole('button', { name: 'translated:common.cancel', exact: true });
      await close.waitFor();
      if (kind !== 'delete') {
        const first = page.getByRole('textbox', { name: 'translated:fields.username', exact: true });
        await page.waitForFunction(() => document.activeElement?.id === 'username');
        const submitName = kind === 'add' ? 'translated:users.addUser' : 'translated:users.updateUser';
        const submit = page.getByRole('button', { name: submitName, exact: true });
        await first.press('Shift+Tab');
        expect(await submit.evaluate(element => document.activeElement === element)).toBe(true);
        await submit.press('Tab');
        expect(await first.evaluate(element => document.activeElement === element)).toBe(true);
        // Keep the mouse-down origin within content while releasing over backdrop.
        const inside = await first.boundingBox();
        await page.mouse.move(inside.x + 5, inside.y + 5); await page.mouse.down();
        await page.mouse.move(1, 1); await page.mouse.up();
        await closedCount(page, 0);
        await first.press('Space'); await closedCount(page, 0);
      }
      await focusVisible(page, close);
      await close.press('Enter'); await closedCount(page, 1);
      if (kind === 'edit') expect(await page.locator('#launcher').evaluate(element => document.activeElement === element)).toBe(true);
      await page.evaluate(value => window.mountModal(value), kind);
      const cancel = page.getByRole('button', { name: 'translated:common.cancel', exact: true });
      await cancel.press('Space'); await closedCount(page, 1);
      await page.evaluate(value => window.mountModal(value), kind);
      await outerClick(page); await closedCount(page, 1);
      await page.evaluate(value => window.mountModal(value), kind);
      const action = page.locator(kind === 'delete' ? '#root .btn-danger' : '#root .btn-primary');
      expect(await action.textContent()).toContain(kind === 'add' ? 'translated:users.addUser'
        : kind === 'edit' ? 'translated:users.updateUser' : 'translated:users.deleteUser');
      await action.press('Enter');
      await page.waitForFunction(() => window.calls.submit === 1);
      expect(await action.isDisabled()).toBe(true);
      await page.keyboard.press('Space'); await page.keyboard.press('Enter');
      expect(await page.evaluate(() => window.calls.submit)).toBe(1);
      await closedCount(page, 0);
      await page.evaluate(() => window.finish.submit());
      await page.waitForFunction(() => !document.querySelector('#root .btn-primary, #root .btn-danger').disabled);
      await action.click();
      expect(await page.evaluate(() => window.calls.submit)).toBe(2);
      await page.evaluate(() => window.finish.submit());
      if (kind !== 'delete') { await action.press('Escape'); await closedCount(page, 1); }
    }
  } catch (error) { console.error(await page.evaluate(() => ({ calls: window.calls, keyEpoch: window.keyEpoch, text: document.body.innerText.slice(0, 1800) }))); throw error; } finally { await browser.close(); expect(errors).toEqual([]); }
});

browserTest('Access/API preserve nested confirmation, saving, reveal and native dismissal guards', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    for (const kind of ['access', 'api']) {
      await page.evaluate(value => window.mountModal(value), kind);
      const parent = page.locator('#root [role="dialog"]');
      await parent.waitFor();
      const close = parent.getByRole('button', { name: 'translated:common.close', exact: true });
      await focusVisible(page, close);
      await close.press('Enter'); await closedCount(page, 1);
      await page.evaluate(value => window.mountModal(value), kind);
      await parent.getByRole('button', { name: 'translated:common.close', exact: true }).press('Space');
      await closedCount(page, 1);
      await page.evaluate(value => window.mountModal(value), kind);
      await outerClick(page); await closedCount(page, 1);
      await page.evaluate(value => window.mountModal(value), kind);
      if (kind === 'access') {
        await parent.getByRole('button', { name: 'translated:common.remove', exact: true }).click();
        await parent.getByRole('button', { name: 'translated:common.saveChanges', exact: true }).press('Enter');
      } else {
        await parent.getByText('translated:tokens.legacy.title', { exact: true }).click();
        await parent.getByRole('button', { name: 'translated:tokens.legacy.generate', exact: true }).press('Space');
      }
      const child = page.locator('body > div[data-keyboard-nav-preserve] [role="dialog"]');
      await child.waitFor();
      await page.waitForFunction(() => document.activeElement?.textContent === 'translated:common.cancel');
      await child.getByRole('button', { name: 'translated:common.cancel', exact: true }).press('Escape');
      expect(await child.count()).toBe(0); await closedCount(page, 0);
      if (kind === 'access') await parent.getByRole('button', { name: 'translated:common.saveChanges', exact: true }).click();
      else await parent.getByRole('button', { name: 'translated:tokens.legacy.generate', exact: true }).click();
      await child.waitFor();
      const confirm = child.getByRole('button').last();
      await flushEffects(page);
      const epoch = await page.evaluate(() => window.keyEpoch);
      await confirm.press('Enter');
      await page.waitForFunction(before => window.keyEpoch > before && (window.calls.save === 1 || window.calls.generate === 1), epoch);
      await page.keyboard.press('Space'); await page.keyboard.press('Escape');
      await closedCount(page, 0);
      expect(await page.evaluate(() => window.calls.save + window.calls.generate)).toBe(1);
      // Target-only click sent to the parent backdrop even while the child owns focus.
      await page.locator('#root > div').dispatchEvent('click'); await closedCount(page, 0);
      await page.evaluate(value => window.finish[value](), kind === 'access' ? 'save' : 'generate');
      await child.waitFor({ state: 'detached' });
      await flushEffects(page);
      if (kind === 'api') {
        await parent.getByRole('textbox', { name: 'translated:tokens.secret.value', exact: true }).waitFor();
        await page.keyboard.press('Escape'); await outerClick(page); await closedCount(page, 0);
        const done = parent.getByRole('button', { name: 'translated:common.done', exact: true });
        expect(await done.isDisabled()).toBe(true);
        await parent.getByRole('checkbox').press('Space');
        await done.press('Enter'); await closedCount(page, 1);
        continue;
      }
      await parent.getByRole('button', { name: 'translated:common.close', exact: true }).press('Escape');
      await closedCount(page, 1);
    }
  } catch (error) { console.error(await page.evaluate(() => ({ calls: window.calls, keyEpoch: window.keyEpoch, text: document.body.innerText.slice(0, 1800) }))); throw error; } finally { await browser.close(); expect(errors).toEqual([]); }
});

browserTest('Audit retains saving/export guards, inner controls and native dismissal', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mountModal('audit'));
    const close = page.getByRole('button', { name: 'translated:common.close', exact: true });
    await focusVisible(page, close); await close.press('Enter'); await closedCount(page, 1);
    await page.evaluate(() => window.mountModal('audit'));
    await close.press('Space'); await closedCount(page, 1);
    await page.evaluate(() => window.mountModal('audit'));
    await outerClick(page); await closedCount(page, 1);
    await page.evaluate(() => window.mountModal('audit'));
    await page.waitForFunction(() => Array.from(document.querySelectorAll('#root button')).some(button => button.textContent === 'translated:audit.exportPage' && !button.disabled));
    await page.getByRole('spinbutton', { name: 'translated:audit.retentionDays', exact: true }).fill('30');
    await flushEffects(page);
    await page.getByRole('button', { name: 'translated:common.saveChanges', exact: true }).press('Enter');
    await page.waitForFunction(() => window.calls.save === 1);
    await flushEffects(page);
    expect(await close.isDisabled()).toBe(true);
    await page.keyboard.press('Escape'); await outerClick(page); await closedCount(page, 0);
    await page.evaluate(() => window.finish.save());
    await page.waitForFunction(() => !document.querySelector('#root header button').disabled);
    await flushEffects(page);
    await page.getByRole('button', { name: 'translated:audit.exportPage', exact: true }).press('Space');
    await page.waitForFunction(() => window.calls.export === 1);
    await flushEffects(page);
    expect(await close.isDisabled()).toBe(true);
    await page.keyboard.press('Escape'); await outerClick(page); await closedCount(page, 0);
    await page.evaluate(() => window.finish.export());
    await page.waitForFunction(() => !document.querySelector('#root header button').disabled);
    await flushEffects(page);
    await close.press('Escape'); await closedCount(page, 1);
  } catch (error) { console.error(await page.evaluate(() => ({ calls: window.calls, keyEpoch: window.keyEpoch, text: document.body.innerText.slice(0, 1800) }))); throw error; } finally { await browser.close(); expect(errors).toEqual([]); }
});

browserTest('Audit single-mount saving blocks Escape independently of remount history', async () => {
  const { browser, page, errors } = await openBrowser();
  try {
    await page.evaluate(() => window.mountModal('audit'));
    await page.waitForFunction(() => Array.from(document.querySelectorAll('#root button')).some(button => button.textContent === 'translated:audit.exportPage' && !button.disabled));
    await flushEffects(page);
    await page.getByRole('spinbutton', { name: 'translated:audit.retentionDays', exact: true }).fill('30');
    await page.getByRole('button', { name: 'translated:common.saveChanges', exact: true }).click();
    await page.waitForFunction(() => window.calls.save === 1);
    await flushEffects(page);
    expect(await page.locator('#root header button').isDisabled()).toBe(true);
    await page.keyboard.press('Escape');
    await closedCount(page, 0);
    await page.evaluate(() => window.finish.save());
  } catch (error) { console.error(await page.evaluate(() => ({ calls: window.calls, keyEpoch: window.keyEpoch, text: document.body.innerText.slice(0, 1800) }))); throw error; } finally { await browser.close(); expect(errors).toEqual([]); }
});
