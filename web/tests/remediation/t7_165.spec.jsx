/** Dialog/backdrop click boundaries are not action controls. Node vnode checks
 * plus opt-in actual Preact/Chromium interactions; no new default dependency. */
jest.mock('preact/hooks', () => ({
  useState: (initial) => [typeof initial === 'function' ? initial() : initial, jest.fn()],
  useEffect: jest.fn(), useCallback: (callback) => callback,
  useMemo: (factory) => factory(), useRef: (current) => ({ current }),
  useId: () => 'fixture-id',
}));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn() }));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: (key) => `translated:${key}` }) }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
jest.mock('qrcode', () => ({ __esModule: true, default: { toCanvas: jest.fn() } }));
const { RoleManagerModal } = require('../../js/components/preact/users/RoleManagerModal.jsx');
const { TotpSetupModal } = require('../../js/components/preact/users/TotpSetupModal.jsx');

const components = [['roles', RoleManagerModal], ['totp', TotpSetupModal]];
function nodes(node) {
  if (Array.isArray(node)) return node.flatMap(nodes);
  if (!node || typeof node !== 'object') return [];
  return [node, ...nodes(node.props?.children)];
}
function fixture(Component) {
  const onClose = jest.fn();
  const tree = Component({ user: { id: 7, username: 'fixture' }, onClose, getAuthHeaders: () => ({}) });
  const inside = nodes(tree).find((node) => node !== tree && ['div', 'section'].includes(node.type));
  return { tree, inside, onClose };
}
it.each(components)('%s uses a presentation backdrop without synthetic key activation', (_name, Component) => {
  const { tree, inside } = fixture(Component);
  expect(tree.props.role).toBe('presentation');
  expect(tree.props.tabIndex).toBeUndefined();
  expect(tree.props.onKeyDown).toBeUndefined();
  expect(inside.props.onClick).toBeUndefined();
  expect(inside.props.onKeyDown).toBeUndefined();
});
it.each(components)('%s ignores bubbled content clicks without exposing them outside the modal', (_name, Component) => {
  const { tree, onClose } = fixture(Component);
  const event = { target: {}, currentTarget: {}, stopPropagation: jest.fn() };
  tree.props.onClick(event);
  expect(onClose).not.toHaveBeenCalled();
  expect(event.stopPropagation).toHaveBeenCalledTimes(1);
});
it.each(components)('%s preserves one direct backdrop dismissal', (_name, Component) => {
  const { tree, onClose } = fixture(Component);
  const backdrop = {};
  tree.props.onClick({ target: backdrop, currentTarget: backdrop, stopPropagation: jest.fn() });
  expect(onClose).toHaveBeenCalledTimes(1);
});
it.each(components)('%s keeps native Close activation separate from the surface', (_name, Component) => {
  const { tree, onClose } = fixture(Component);
  const close = nodes(tree).find((node) => node.type === 'button' && node.props.children === 'translated:common.close');
  expect(close).toBeDefined();
  expect(close.props.onKeyDown).toBeUndefined();
  close.props.onClick();
  expect(onClose).toHaveBeenCalledTimes(1);
});

const browserTest = process.env.T7_165_BROWSER === '1' ? it : it.skip;
browserTest('actual dialogs preserve nested controls, saving guards, focus and keyboard dismissal', async () => {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const fs = require('node:fs');
  const path = require('node:path');
  const webRoot = path.resolve(__dirname, '../..');
  const result = await build({
    stdin: { resolveDir: webRoot, loader: 'jsx', contents: `
      import { h, render } from 'preact';
      import { RoleManagerModal } from './js/components/preact/users/RoleManagerModal.jsx';
      import { TotpSetupModal } from './js/components/preact/users/TotpSetupModal.jsx';
      const root = document.querySelector('#root');
      window.calls = { close: 0, setup: 0, verify: 0, save: 0, success: 0 };
      window.keyEpoch = 0;
      const add = window.addEventListener.bind(window);
      window.addEventListener = (type, callback, options) => {
        if (type === 'keydown') window.keyEpoch++;
        return add(type, callback, options);
      };
      const role = { uuid: 'fixture-role', name: 'Operator', description: 'Fixture', actions: ['recordings:view'] };
      window.fixtureJSON = async (url, options = {}) => {
        if (options.method === 'PUT') {
          window.calls.save++;
          return new Promise(resolve => { window.finishSave = () => resolve({ role }); });
        }
        if (url.endsWith('/actions')) return { actions: [{ key: 'recordings:view', category: 'recordings', description: 'Read', enforced: true }] };
        return { roles: [role], policy_version: 1 };
      };
      window.fetch = async (url) => {
        let data = {};
        if (url.endsWith('/status')) data = { totp_enabled: false };
        if (url.endsWith('/setup')) { window.calls.setup++; data = { otpauth_uri: 'otpauth://totp/fixture?secret=TEST', secret: 'synthetic-test-secret' }; }
        if (url.endsWith('/verify')) window.calls.verify++;
        return { ok: true, json: async () => data };
      };
      window.mountModal = (kind) => {
        render(null, root);
        const onClose = () => { window.calls.close++; render(null, root); };
        render(h(kind === 'roles' ? RoleManagerModal : TotpSetupModal, {
          onClose, getAuthHeaders: () => ({}), user: { id: 7, username: 'fixture' },
          onSuccess: () => window.calls.success++
        }), root);
      };
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
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    await page.setContent('<button id="launcher">Launch</button><div id="root"></div>');
    // Use the actual built users-page stylesheet, including real focus styling.
    const html = fs.readFileSync(path.join(webRoot, 'dist/users.html'), 'utf8');
    const links = html.match(/<link[^>]*rel="stylesheet"[^>]*>/g) || [];
    expect(links.length).toBeGreaterThan(0);
    for (const link of links) {
      const href = link.match(/href="([^"]+)"/)[1];
      await page.addStyleTag({ content: fs.readFileSync(path.join(webRoot, 'dist', href), 'utf8') });
    }
    await page.addScriptTag({ content: result.outputFiles[0].text });
    await page.evaluate(() => window.mountModal('roles'));
    const roleDialog = page.getByRole('dialog', { name: 'translated:access.roles.title', exact: true });
    await roleDialog.getByRole('textbox', { name: 'translated:common.name' }).waitFor();
    await page.waitForFunction(() => window.keyEpoch > 0);
    const close = roleDialog.getByRole('button', { name: 'translated:common.close', exact: true });
    const unfocusedShadow = await close.evaluate((button) => getComputedStyle(button).boxShadow);
    await close.focus();
    expect(await close.evaluate((button, previousShadow) => {
      const style = getComputedStyle(button);
      return document.activeElement === button && button.matches(':focus-visible')
        && ((style.outlineStyle !== 'none' && parseFloat(style.outlineWidth) > 0)
          || (style.boxShadow !== 'none' && style.boxShadow !== previousShadow));
    }, unfocusedShadow)).toBe(true);
    const nameInput = roleDialog.getByRole('textbox', { name: 'translated:common.name' });
    await nameInput.fill('Auditor');
    await nameInput.press('Space');
    expect(await page.evaluate(() => window.calls.close)).toBe(0);
    await roleDialog.getByRole('button', { name: 'translated:access.roles.create', exact: true }).focus();
    await page.keyboard.press('Space');
    await roleDialog.getByRole('heading', { name: 'translated:access.roles.newRole', exact: true }).waitFor();
    expect(await page.evaluate(() => window.calls.close)).toBe(0);
    await roleDialog.getByRole('button', { name: /^Operator/ }).click();
    const epoch = await page.evaluate(() => window.keyEpoch);
    await roleDialog.getByRole('button', { name: 'translated:common.saveChanges', exact: true }).focus();
    await page.keyboard.press('Enter');
    await page.waitForFunction((before) => window.calls.save === 1 && window.keyEpoch > before, epoch);
    expect(await close.isDisabled()).toBe(true);
    await page.keyboard.press('Escape');
    await page.locator('#root > div').click({ position: { x: 1, y: 1 } });
    expect(await page.evaluate(() => window.calls.close)).toBe(0);
    await page.evaluate(() => window.finishSave());
    await page.waitForFunction(() => !document.querySelector('#role-manager-title').closest('section').querySelector('header button').disabled);
    await roleDialog.getByRole('button', { name: 'translated:common.delete', exact: true }).click();
    const cancel = page.getByRole('button', { name: 'translated:common.cancel', exact: true });
    await page.waitForFunction(() => document.activeElement?.textContent === 'translated:common.cancel');
    expect(await cancel.count()).toBe(1);
    await page.keyboard.press('Escape');
    await page.waitForFunction(() => document.querySelectorAll('[role="dialog"]').length === 1);
    expect(await page.evaluate(() => window.calls.close)).toBe(0);
    await close.focus();
    await page.keyboard.press('Space');
    expect(await page.evaluate(() => window.calls.close)).toBe(1);

    // Both native Close keys and pointer dismissal remain single-dispatch.
    for (const kind of ['roles', 'totp']) {
      for (const key of ['Enter', 'Space']) {
        await page.evaluate((value) => window.mountModal(value), kind);
        const nativeClose = page.getByRole('button', { name: 'translated:common.close', exact: true });
        await nativeClose.focus();
        const before = await page.evaluate(() => window.calls.close);
        await page.keyboard.press(key);
        expect(await page.evaluate(() => window.calls.close)).toBe(before + 1);
        expect(await page.locator('#root > div').count()).toBe(0);
      }
      await page.evaluate((value) => window.mountModal(value), kind);
      const before = await page.evaluate(() => window.calls.close);
      await page.locator('#root > div').click({ position: { x: 1, y: 1 } });
      expect(await page.evaluate(() => window.calls.close)).toBe(before + 1);
    }

    await page.evaluate(() => window.mountModal('totp'));
    const setup = page.getByRole('button', { name: 'translated:users.setupTwoFactorAuthentication', exact: true });
    await setup.focus();
    const dismissals = await page.evaluate(() => window.calls.close);
    await page.keyboard.press('Space');
    const code = page.getByPlaceholder('000000');
    await code.waitFor();
    expect(await code.evaluate((input) => document.activeElement === input)).toBe(true);
    expect(await page.evaluate(() => window.calls.setup)).toBe(1);
    expect(await page.evaluate(() => window.calls.close)).toBe(dismissals);
    await code.fill('123456');
    await code.dispatchEvent('change');
    await code.press('Enter');
    await page.waitForFunction(() => window.calls.success === 1);
    expect(await page.evaluate(() => window.calls.verify)).toBe(1);
    expect(await page.evaluate(() => window.calls.close)).toBe(dismissals);
    expect(await page.getByRole('button', { name: 'translated:common.done', exact: true }).count()).toBe(1);
  } finally { await browser.close(); }
});
