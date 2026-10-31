/** Native label contracts, including multiple real Preact component instances. */
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
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
jest.mock('qrcode', () => ({ __esModule: true, default: { toCanvas: jest.fn(async () => {}) } }));
const { RoleManagerModal } = require('../../js/components/preact/users/RoleManagerModal.jsx');
const { TotpSetupModal } = require('../../js/components/preact/users/TotpSetupModal.jsx');
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

function checkLabelContract(tree) {
  const labels = nodes(tree).filter((node) => node.type === 'label');
  expect(labels.length).toBeGreaterThan(0);
  const ids = nodes(tree).map((node) => node.props?.id).filter(Boolean);
  expect(new Set(ids).size).toBe(ids.length);
  for (const label of labels) {
    const matching = nodes(tree).filter((node) => node.props?.id === label.props.htmlFor);
    expect(matching).toHaveLength(1);
    const control = matching[0];
    expect(['input', 'textarea', 'select']).toContain(control.type);
    // The configured legacy label-has-for rule requires BOTH native mechanisms.
    expect(nodes(label.props.children)).toContain(control);
    const namingNode = nodes(tree).find((node) => node.props?.id === control.props['aria-labelledby']);
    expect(text(namingNode)).not.toBe('');
    expect(nodes(namingNode.props.children)).not.toContain(control);
  }
  return labels;
}

it('role labels already nest and explicitly reference their own named controls', async () => {
  fetchJSON.mockImplementation(async (url) => url.endsWith('/actions')
    ? { actions: [{ key: 'recordings:view', description: 'Read recordings', category: 'recordings', enforced: true }] }
    : { roles: [{ uuid: 'role', name: 'Operator', description: 'Fixture', actions: ['recordings:view'], builtin: false }] });
  const tree = await loaded(RoleManagerModal, { onClose: jest.fn(), getAuthHeaders: () => ({}) });
  expect(checkLabelContract(tree)).toHaveLength(3);
});

it('TOTP label nests and explicitly references only its verification control', async () => {
  const props = { user: { id: 7, username: 'fixture' }, onClose: jest.fn(), getAuthHeaders: () => ({}) };
  let tree = await loaded(TotpSetupModal, props);
  await nodes(tree).find((node) => node.props?.children === 'translated:users.setupTwoFactorAuthentication').props.onClick();
  tree = render(TotpSetupModal, props);
  expect(checkLabelContract(tree)).toHaveLength(1);
  const input = nodes(tree).find((node) => node.type === 'input');
  expect(labelText(tree, input)).toBe('translated:users.enterVerificationCode');
  expect(input.props.required).toBe(true);
  expect(input.props.maxLength).toBe('6');
});

const browserTest = process.env.T7_187_BROWSER === '1' ? it : it.skip;
browserTest('repeated real modals retain unique stable IDs and label clicks focus only their associated control', async () => {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const path = require('node:path');
  const result = await build({
    stdin: { resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx', contents: `
      import { h, render } from 'preact';
      import { RoleManagerModal } from './js/components/preact/users/RoleManagerModal.jsx';
      import { TotpSetupModal } from './js/components/preact/users/TotpSetupModal.jsx';
      const root = document.querySelector('#root');
      window.closeCalls = 0;
      window.fixtureJSON = async (url) => url.endsWith('/actions')
        ? { actions: [{ key: 'recordings:view', description: 'Read recordings', category: 'recordings', enforced: true }] }
        : { roles: [{ uuid: 'role', name: 'Operator', description: 'Fixture', actions: ['recordings:view'], builtin: false }] };
      window.fetch = async (url) => ({ ok: true, json: async () => url.endsWith('/status')
        ? { totp_enabled: false } : { secret: 'synthetic', otpauth_uri: 'otpauth://fixture' } });
      const getAuthHeaders = () => ({});
      let revision = 0;
      window.refresh = () => render(h('div', { 'data-revision': revision++ },
        ['roles-0', 'roles-1', 'totp-0', 'totp-1'].map((key, index) => h('section', { key, 'data-case': key },
          h(key.startsWith('roles') ? RoleManagerModal : TotpSetupModal, {
            user: { id: index + 7, username: key }, onClose: () => window.closeCalls++, getAuthHeaders
          })))), root);
      window.refresh();
    ` },
    bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact',
    plugins: [{ name: 'test-services', setup(builder) {
      builder.onResolve({ filter: /(?:i18n|query-client|ToastContainer)\.(?:js|jsx)$|^qrcode$/ },
        (args) => ({ path: args.path, namespace: 'test-service' }));
      builder.onLoad({ filter: /.*/, namespace: 'test-service' }, () => ({
        contents: `export const useI18n = () => ({ t: key => 'translated:' + key });
          export const fetchJSON = (url, options) => window.fixtureJSON(url, options);
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
    for (const key of ['totp-0', 'totp-1']) {
      await page.locator(`[data-case="${key}"]`).getByRole('button', {
        name: 'translated:users.setupTwoFactorAuthentication', exact: true,
      }).click();
    }
    await page.waitForFunction(() => document.querySelectorAll('label').length === 8);
    const originalIds = await page.locator('[id]').evaluateAll((elements) => elements.map((element) => element.id));
    expect(new Set(originalIds).size).toBe(originalIds.length);
    const dialogs = page.locator('[role="dialog"][aria-modal="true"]');
    expect(await dialogs.count()).toBe(2);
    expect(await dialogs.evaluateAll((elements) => elements.every((dialog) => {
      const heading = document.getElementById(dialog.getAttribute('aria-labelledby'));
      return heading?.tagName === 'H2' && dialog.contains(heading)
        && heading.textContent === 'translated:access.roles.title';
    }))).toBe(true);
    const associationCheck = () => Array.from(document.querySelectorAll('label')).map((label) => {
      const control = label.control;
      return { hasControl: Boolean(control), matchesFor: control?.id === label.htmlFor,
        sameInstance: control?.closest('[data-case]') === label.closest('[data-case]'),
        labels: control?.labels.length };
    });
    expect(await page.evaluate(associationCheck)).toEqual(Array.from({ length: 8 }, () => ({
      hasControl: true, matchesFor: true, sameInstance: true, labels: 1,
    })));
    // Label text clicks must target the associated instance, including checkbox
    // labels that must dispatch one native change rather than duplicate activation.
    for (const key of ['roles-0', 'roles-1', 'totp-0', 'totp-1']) {
      const instance = page.locator(`[data-case="${key}"]`);
      const names = key.startsWith('roles')
        ? ['translated:common.name', 'translated:common.description', 'recordings:view']
        : ['translated:users.enterVerificationCode'];
      for (const name of names) {
        const control = instance.getByRole(name === 'recordings:view' ? 'checkbox' : 'textbox', { name, exact: true });
        await control.evaluate((element) => {
          element.dataset.changes = '0';
          element.addEventListener('change', () => element.dataset.changes = String(Number(element.dataset.changes) + 1));
        });
        await instance.getByText(name, { exact: true }).click();
        expect(await control.evaluate((element) => document.activeElement === element)).toBe(true);
        if (name === 'recordings:view') {
          expect(await control.isChecked()).toBe(false);
          expect(await control.getAttribute('data-changes')).toBe('1');
        } else {
          await control.fill(key.startsWith('totp') ? '123456' : `edited ${key}`);
        }
      }
    }
    expect(await page.evaluate(() => window.closeCalls)).toBe(0);
    await page.evaluate(() => window.refresh());
    expect(await page.locator('[id]').evaluateAll((elements) => elements.map((element) => element.id))).toEqual(originalIds);
    expect(await page.evaluate(associationCheck)).toEqual(Array.from({ length: 8 }, () => ({
      hasControl: true, matchesFor: true, sameInstance: true, labels: 1,
    })));
    // A second-instance label must still focus its own field after rerender.
    const second = page.locator('[data-case="totp-1"]');
    await second.getByText('translated:users.enterVerificationCode', { exact: true }).click();
    expect(await second.getByRole('textbox').evaluate((element) => document.activeElement === element)).toBe(true);
  } finally { await browser.close(); }
});
