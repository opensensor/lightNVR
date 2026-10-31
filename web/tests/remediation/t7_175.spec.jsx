/** Explicit visible label relationships and named icon actions. Actual Preact
 * browser checks are opt-in; the default Node suite exercises real callbacks. */
const mockHooks = { values: [], cursor: 0, effects: [] };
jest.mock('preact/hooks', () => ({
  useState: (initial) => {
    const index = mockHooks.cursor++;
    if (!mockHooks.values[index]) mockHooks.values[index] = { value: typeof initial === 'function' ? initial() : initial };
    return [mockHooks.values[index].value, (next) => {
      mockHooks.values[index].value = typeof next === 'function' ? next(mockHooks.values[index].value) : next;
    }];
  },
  useId: () => `fixture-${mockHooks.cursor++}`,
  useRef: (current) => {
    const index = mockHooks.cursor++;
    if (!mockHooks.values[index]) mockHooks.values[index] = { current };
    return mockHooks.values[index];
  },
  useEffect: (effect) => {
    const index = mockHooks.cursor++;
    if (!mockHooks.values[index]) { mockHooks.values[index] = {}; mockHooks.effects.push(effect); }
  },
  useCallback: (callback) => callback, useMemo: (factory) => factory(),
}));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn() }));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: (key) => `translated:${key}` }) }));
jest.mock('../../js/utils/date-utils.js', () => ({ formatLocalDateTime: () => 'fixture-date' }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
jest.mock('qrcode', () => ({ __esModule: true, default: { toCanvas: jest.fn(async () => {}) } }));
const { RoleManagerModal } = require('../../js/components/preact/users/RoleManagerModal.jsx');
const { TotpSetupModal } = require('../../js/components/preact/users/TotpSetupModal.jsx');
const { UsersTable } = require('../../js/components/preact/users/UsersTable.jsx');
const { fetchJSON } = require('../../js/query-client.js');
const savedGlobals = Object.fromEntries(['window', 'fetch'].map((key) => [key, Object.getOwnPropertyDescriptor(globalThis, key)]));
function nodes(node) {
  if (Array.isArray(node)) return node.flatMap(nodes);
  if (!node || typeof node !== 'object') return [];
  return [node, ...nodes(node.props?.children)];
}
function text(node) {
  if (Array.isArray(node)) return node.map(text).join('');
  if (node == null || typeof node === 'boolean') return '';
  if (typeof node !== 'object') return String(node);
  return text(node.props?.children);
}
function render(Component, props) { mockHooks.cursor = 0; return Component(props); }
async function loaded(Component, props) {
  render(Component, props);
  mockHooks.effects.splice(0).forEach((effect) => effect());
  for (let i = 0; i < 5; i++) await Promise.resolve();
  return render(Component, props);
}
function labelText(tree, control) {
  const id = control.props['aria-labelledby'];
  expect(typeof id).toBe('string');
  const matches = nodes(tree).filter((node) => node.props?.id === id);
  expect(matches).toHaveLength(1);
  return text(matches[0]);
}
beforeEach(() => {
  mockHooks.values = []; mockHooks.effects = []; mockHooks.cursor = 0;
  jest.clearAllMocks();
  Object.defineProperty(globalThis, 'window', { configurable: true, value: { addEventListener: jest.fn(), removeEventListener: jest.fn() } });
  Object.defineProperty(globalThis, 'fetch', { configurable: true, value: jest.fn(async (url) => ({
    ok: true, json: async () => url.endsWith('/status') ? { totp_enabled: false } : { secret: 'synthetic', otpauth_uri: 'otpauth://fixture' },
  })) });
});
afterEach(() => {
  for (const [key, descriptor] of Object.entries(savedGlobals)) {
    if (descriptor) Object.defineProperty(globalThis, key, descriptor);
    else delete globalThis[key];
  }
});

it.each([false, true])('role fields have explicit visible names and preserve builtin=%s disabled state', async (builtin) => {
  fetchJSON.mockImplementation(async (url) => url.endsWith('/actions')
    ? { actions: [{ key: 'recordings:view', description: 'Read recordings', category: 'recordings', enforced: true }] }
    : { roles: [{ uuid: 'role', name: 'Operator', description: 'Fixture', actions: ['recordings:view'], builtin }] });
  const tree = await loaded(RoleManagerModal, { onClose: jest.fn(), getAuthHeaders: () => ({}) });
  const fields = nodes(tree).filter((node) => node.type === 'input' || node.type === 'textarea');
  expect(fields).toHaveLength(3);
  expect(fields.map((control) => labelText(tree, control))).toEqual([
    'translated:common.name', 'translated:common.description', 'recordings:view',
  ]);
  fields.forEach((control) => {
    expect(control.props.disabled).toBe(builtin);
    expect(nodes(tree).filter((node) => node.type === 'label' && node.props.htmlFor === control.props.id)).toHaveLength(1);
  });
  const checkbox = fields.find((control) => control.props.type === 'checkbox');
  const description = nodes(tree).find((node) => node.props?.id === checkbox.props['aria-describedby']);
  expect(text(description)).toContain('Read recordings');
});

it('TOTP QR image and verification input have meaningful visible names without changing verification', async () => {
  const props = { user: { id: 7, username: 'fixture' }, onClose: jest.fn(), onSuccess: jest.fn(), getAuthHeaders: () => ({}) };
  let tree = await loaded(TotpSetupModal, props);
  const setup = nodes(tree).find((node) => node.props?.children === 'translated:users.setupTwoFactorAuthentication');
  await setup.props.onClick();
  tree = render(TotpSetupModal, props);
  const canvas = nodes(tree).find((node) => node.type === 'canvas');
  const qrImage = nodes(tree).find((node) => node.props?.role === 'img');
  expect(qrImage.props['aria-label']).toBe('translated:users.scanQrCode');
  expect(qrImage.props.children).toBe(canvas);
  expect(canvas.props['aria-hidden']).toBe('true');
  const input = nodes(tree).find((node) => node.type === 'input');
  expect(labelText(tree, input)).toBe('translated:users.enterVerificationCode');
  expect(nodes(tree).filter((node) => node.type === 'label' && node.props.htmlFor === input.props.id)).toHaveLength(1);
  // Preact/compat normalizes text-field onChange to the native oninput prop.
  input.props.oninput({ target: { value: 'a12b34567' } });
  tree = render(TotpSetupModal, props);
  expect(nodes(tree).find((node) => node.type === 'input').props.value).toBe('123456');
  await nodes(tree).find((node) => node.type === 'form').props.onSubmit({ preventDefault: jest.fn() });
  expect(props.onSuccess).toHaveBeenCalledTimes(1);
  expect(fetch.mock.calls.filter(([url]) => url.endsWith('/verify'))).toHaveLength(1);
});

it('all five icon actions have localized names independent of title and keep original callbacks', () => {
  const user = { id: 7, username: 'fixture', role: 1 };
  const names = ['access.policy.manage', 'users.editUser', 'users.deleteUser', 'tokens.manage', 'users.manageMfa'];
  const handlers = ['onAccess', 'onEdit', 'onDelete', 'onApiKey', 'onMfa'];
  const props = { users: [user], ...Object.fromEntries(handlers.map((name) => [name, jest.fn()])) };
  const tree = render(UsersTable, props);
  const buttons = nodes(tree).filter((node) => node.type === 'button');
  expect(buttons).toHaveLength(5);
  buttons.forEach((button, index) => {
    expect(button.props['aria-label']).toBe(`translated:${names[index]}`);
    expect(button.props.title).toBe(`translated:${names[index]}`);
    const event = { preventDefault: jest.fn(), stopPropagation: jest.fn() };
    button.props.onClick(event);
    expect(props[handlers[index]]).toHaveBeenCalledTimes(1);
    expect(props[handlers[index]]).toHaveBeenCalledWith(user);
    expect(event.preventDefault).toHaveBeenCalledTimes(1);
    expect(event.stopPropagation).toHaveBeenCalledTimes(1);
  });
});

const browserTest = process.env.T7_175_BROWSER === '1' ? it : it.skip;
browserTest('actual rendered controls retain accessible names, label activation and native action dispatch', async () => {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const path = require('node:path');
  const result = await build({
    stdin: { resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx', contents: `
      import { h, render } from 'preact';
      import { RoleManagerModal } from './js/components/preact/users/RoleManagerModal.jsx';
      import { TotpSetupModal } from './js/components/preact/users/TotpSetupModal.jsx';
      import { UsersTable } from './js/components/preact/users/UsersTable.jsx';
      const root = document.querySelector('#root');
      window.calls = { access: 0, edit: 0, delete: 0, api: 0, mfa: 0, verify: 0 };
      window.fixtureJSON = async (url) => url.endsWith('/actions')
        ? { actions: [{ key: 'recordings:view', description: 'Read recordings', category: 'recordings', enforced: true }] }
        : { roles: [{ uuid: 'role', name: 'Operator', description: 'Fixture', actions: ['recordings:view'], builtin: window.builtin }] };
      window.fetch = async (url) => {
        if (url.endsWith('/verify')) window.calls.verify++;
        return { ok: true, json: async () => url.endsWith('/status')
          ? { totp_enabled: false } : { secret: 'synthetic', otpauth_uri: 'otpauth://fixture' } };
      };
      window.mountCase = (kind, builtin = false) => {
        render(null, root); window.builtin = builtin;
        const user = { id: 7, username: 'fixture', role: 1 };
        const common = { onClose: () => {}, getAuthHeaders: () => ({}), user };
        const record = (name) => (value) => { if (value.id !== 7) throw Error('wrong user'); window.calls[name]++; };
        render(h(kind === 'roles' ? RoleManagerModal : kind === 'totp' ? TotpSetupModal : UsersTable,
          kind === 'table' ? { users: [user], onAccess: record('access'), onEdit: record('edit'),
            onDelete: record('delete'), onApiKey: record('api'), onMfa: record('mfa') } : common), root);
      };
    ` },
    bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact',
    plugins: [{ name: 'test-services', setup(builder) {
      builder.onResolve({ filter: /(?:i18n|query-client|date-utils|ToastContainer)\.(?:js|jsx)$|^qrcode$/ },
        (args) => ({ path: args.path, namespace: 'test-service' }));
      builder.onLoad({ filter: /.*/, namespace: 'test-service' }, () => ({
        contents: `export const useI18n = () => ({ t: key => 'translated:' + key });
          export const fetchJSON = (url, options) => window.fixtureJSON(url, options);
          export const formatLocalDateTime = () => 'fixture-date';
          export const showStatusMessage = () => {};
          export default { toCanvas: async () => {} };`, loader: 'js',
      }));
    } }],
  });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    page.setDefaultTimeout(5000);
    await page.setContent('<div id="root"></div>');
    await page.addScriptTag({ content: result.outputFiles[0].text });
    for (const builtin of [false, true]) {
      await page.evaluate((value) => window.mountCase('roles', value), builtin);
      const name = page.getByRole('textbox', { name: 'translated:common.name', exact: true });
      await name.waitFor();
      const description = page.getByRole('textbox', { name: 'translated:common.description', exact: true });
      const action = page.getByRole('checkbox', { name: 'recordings:view', exact: true });
      expect(await action.count()).toBe(1);
      expect(await name.isDisabled()).toBe(builtin);
      expect(await description.isDisabled()).toBe(builtin);
      expect(await action.isDisabled()).toBe(builtin);
      if (!builtin) {
        await page.getByText('translated:common.name', { exact: true }).click();
        expect(await name.evaluate((input) => document.activeElement === input)).toBe(true);
        await page.getByText('recordings:view', { exact: true }).click();
        expect(await action.isChecked()).toBe(false);
        await action.focus(); await page.keyboard.press('Space');
        expect(await action.isChecked()).toBe(true);
      }
    }
    await page.evaluate(() => window.mountCase('totp'));
    await page.getByRole('button', { name: 'translated:users.setupTwoFactorAuthentication', exact: true }).click();
    const code = page.getByRole('textbox', { name: 'translated:users.enterVerificationCode', exact: true });
    await code.waitFor();
    expect(await page.getByRole('img', { name: 'translated:users.scanQrCode', exact: true }).count()).toBe(1);
    await page.getByText('translated:users.enterVerificationCode', { exact: true }).click();
    expect(await code.evaluate((input) => document.activeElement === input)).toBe(true);
    await code.fill('123456'); await code.dispatchEvent('change'); await code.press('Enter');
    await page.waitForFunction(() => window.calls.verify === 1);

    await page.evaluate(() => window.mountCase('table'));
    // Removing tooltips proves accessible names do not rely on title fallback.
    await page.locator('button[title]').evaluateAll((buttons) => buttons.forEach((button) => button.removeAttribute('title')));
    const actions = [['access', 'access.policy.manage'], ['edit', 'users.editUser'],
      ['delete', 'users.deleteUser'], ['api', 'tokens.manage'], ['mfa', 'users.manageMfa']];
    for (const [name, key] of actions) {
      const button = page.getByRole('button', { name: 'translated:' + key, exact: true });
      await button.focus(); await page.keyboard.press('Enter');
      expect(await page.evaluate((action) => window.calls[action], name)).toBe(1);
      await page.keyboard.press('Space');
      expect(await page.evaluate((action) => window.calls[action], name)).toBe(2);
    }
    const actionCell = page.getByRole('cell').last();
    expect(await actionCell.getByRole('button').count()).toBe(5);
  } finally { await browser.close(); }
});
