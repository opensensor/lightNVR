/** T7.160: real Header vnodes/callbacks with the repository's Node hook shim.
 * Optional Chromium smoke exercises native keyboard semantics without adding a
 * browser dependency to the default Jest suite: T7_160_BROWSER=1 npm test ... */
const mockHooks = { values: [], cursor: 0, effects: [], writes: [] };
jest.mock('preact/hooks', () => ({
  useState: (initial) => {
    const index = mockHooks.cursor++;
    if (!mockHooks.values[index]) {
      mockHooks.values[index] = { value: typeof initial === 'function' ? initial() : initial };
    }
    return [mockHooks.values[index].value, (next) => {
      const value = typeof next === 'function' ? next(mockHooks.values[index].value) : next;
      mockHooks.values[index].value = value;
      mockHooks.writes.push(value);
    }];
  },
  useEffect: (effect) => {
    const index = mockHooks.cursor++;
    if (!mockHooks.values[index]) {
      mockHooks.values[index] = { value: true };
      mockHooks.effects.push(effect);
    }
  },
  useCallback: (callback) => callback,
}));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn() }));
jest.mock('../../js/utils/settings-utils.js', () => ({ getSettings: jest.fn() }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
jest.mock('../../js/components/preact/users/EditUserModal.jsx', () => ({ EditUserModal: () => null }));
jest.mock('../../js/utils/auth-utils.js', () => ({
  getAuthHeaders: jest.fn(() => ({})), isDemoMode: jest.fn(() => false), validateSession: jest.fn(),
}));
jest.mock('../../js/utils/navigation-utils.js', () => ({ forceNavigation: jest.fn() }));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: (key) => `translated:${key}` }) }));
jest.mock('../../js/components/preact/common/LanguageSelector.jsx', () => ({ __esModule: true, default: () => null }));

const { Header } = require('../../js/components/preact/Header.jsx');
const { EditUserModal } = require('../../js/components/preact/users/EditUserModal.jsx');
const { validateSession } = require('../../js/utils/auth-utils.js');
const { getSettings } = require('../../js/utils/settings-utils.js');
const { forceNavigation } = require('../../js/utils/navigation-utils.js');
const originalGlobals = Object.fromEntries(['window', 'document', 'localStorage'].map(
  (key) => [key, Object.getOwnPropertyDescriptor(globalThis, key)]));

function render() { mockHooks.cursor = 0; return Header({ version: 'test-version' }); }
function allNodes(node) {
  if (Array.isArray(node)) return node.flatMap(allNodes);
  if (!node || typeof node !== 'object') return [];
  return [node, ...allNodes(node.props?.children)];
}
function byId(tree, id) { return allNodes(tree).filter((node) => node.props?.id === id); }
function click(node) { node.props.onClick({ preventDefault: jest.fn() }); }
function openMobile(tree) {
  click(allNodes(tree).find((node) => node.props?.['aria-label'] === 'translated:nav.toggleMenu'));
  return render();
}
async function load(session = {}, settings = {}) {
  validateSession.mockResolvedValue({ valid: true, id: 7, username: 'alice', role: 'admin', role_id: 0, ...session });
  getSettings.mockResolvedValue({ web_auth_enabled: true, ...settings });
  render();
  mockHooks.effects.splice(0).forEach((effect) => effect());
  await Promise.resolve();
  await Promise.resolve();
  mockHooks.writes = [];
  return render();
}
beforeEach(() => {
  jest.useFakeTimers();
  jest.clearAllMocks();
  mockHooks.values = []; mockHooks.effects = []; mockHooks.writes = []; mockHooks.cursor = 0;
  const values = new Map([['auth', 'test-token']]);
  Object.defineProperty(globalThis, 'localStorage', { configurable: true, value: {
    getItem: (key) => values.get(key) ?? null,
    setItem: (key, value) => values.set(key, String(value)),
    removeItem: (key) => values.delete(key),
  } });
  Object.defineProperty(globalThis, 'window', { configurable: true, value: { location: { pathname: '/streams.html' } } });
  Object.defineProperty(globalThis, 'document', { configurable: true, value: { getElementById: () => ({ dataset: { activeNav: 'nav-streams' } }) } });
});
afterEach(() => {
  jest.clearAllTimers(); jest.useRealTimers();
  for (const [key, descriptor] of Object.entries(originalGlobals)) {
    if (descriptor) Object.defineProperty(globalThis, key, descriptor);
    else delete globalThis[key];
  }
});

it('uses a localized native action button and opens the profile once without navigation', async () => {
  const profile = byId(await load(), 'nav-editProfile')[0];
  expect(profile.type).toBe('button');
  expect(profile.props.type).toBe('button');
  expect(profile.props.href).toBeUndefined();
  expect(profile.props.title).toBe('translated:auth.editProfile');
  expect(profile.props.children).toBe('alice');
  expect(profile.props.className).toContain('px-3 py-2');
  click(profile);
  const modals = allNodes(render()).filter((node) => node.type === EditUserModal);
  expect(modals).toHaveLength(1);
  expect(modals[0].props.currentUser.id).toBe(7);
  expect(mockHooks.writes.filter((value) => value === true)).toHaveLength(1);
  expect(forceNavigation).not.toHaveBeenCalled();
  expect(window.location.pathname).toBe('/streams.html');
});

it.each([
  ['missing current user', { id: null }, {}],
  ['demo mode', { demo_mode: true }, {}],
  ['authentication disabled', {}, { web_auth_enabled: false }],
])('disables the profile action with %s and cannot activate it', async (_name, session, settings) => {
  const profile = byId(await load(session, settings), 'nav-editProfile')[0];
  expect(profile.type).toBe('button');
  expect(profile.props.disabled).toBe(true);
  expect(profile.props['aria-disabled']).toBe(true);
  expect(profile.props.href).toBeUndefined();
  click(profile); // Guard remains effective even when a callback is invoked directly.
  expect(mockHooks.writes).toEqual([]);
  expect(allNodes(render()).some((node) => node.type === EditUserModal)).toBe(false);
  expect(forceNavigation).not.toHaveBeenCalled();
});

it('opens the mobile profile once and closes the menu without navigating', async () => {
  const tree = openMobile(await load());
  const profiles = byId(tree, 'nav-editProfile');
  expect(profiles).toHaveLength(2);
  expect(profiles[1].props.className).toContain('block w-full px-4 py-3');
  expect(profiles[1].props.className).toContain('text-center');
  mockHooks.writes = [];
  click(profiles[1]);
  const after = render();
  expect(byId(after, 'nav-editProfile')).toHaveLength(1);
  expect(allNodes(after).filter((node) => node.type === EditUserModal)).toHaveLength(1);
  expect(mockHooks.writes.filter((value) => value === true)).toHaveLength(1);
  expect(forceNavigation).not.toHaveBeenCalled();
});

it('keeps navigation destinations and closes the mobile menu on navigation', async () => {
  const tree = openMobile(await load());
  const destinations = {
    'nav-live': 'index.html', 'nav-recordings': 'recordings.html',
    'nav-streams': 'streams.html', 'nav-settings': 'settings.html',
    'nav-users': 'users.html', 'nav-system': 'system.html', 'nav-logout': '/logout',
  };
  for (const [id, href] of Object.entries(destinations)) {
    for (const node of byId(tree, id)) { expect(node.type).toBe('a'); expect(node.props.href).toBe(href); }
  }
  const mobileLink = byId(tree, 'nav-recordings')[1];
  click(mobileLink);
  expect(forceNavigation).toHaveBeenCalledTimes(1);
  expect(forceNavigation).toHaveBeenCalledWith('recordings.html', expect.any(Object));
  expect(byId(render(), 'nav-recordings')).toHaveLength(1);
  expect(allNodes(render()).some((node) => node.type === EditUserModal)).toBe(false);
});

it('preserves root live links and the demo login destination', async () => {
  window.location.pathname = '/';
  localStorage.removeItem('auth');
  const tree = await load({ demo_mode: true });
  expect(byId(tree, 'nav-live')[0].props.href).toBe('./');
  expect(byId(tree, 'nav-logo')[0].props.href).toBe('./');
  expect(byId(tree, 'nav-login')[0].props.href).toBe('/login.html');
  expect(byId(tree, 'nav-login')[0].type).toBe('a');
});

// Run explicitly with the root workspace's installed Playwright. This is a
// browser semantics supplement: DOM attributes come from actual Header vnodes,
// and the browser click calls the actual Header handler through an exposed bridge.
const keyboardTest = process.env.T7_160_BROWSER === '1' ? it : it.skip;
keyboardTest('native Enter/Space activate once; disabled action skips keyboard activation', async () => {
  const profile = byId(await load(), 'nav-editProfile')[0];
  jest.clearAllTimers();
  jest.useRealTimers();
  const { chromium } = require('../../../node_modules/playwright');
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    expect(profile.type).toBe('button');
    let activations = 0;
    await page.exposeFunction('activateProfile', () => { activations++; click(profile); });
    await page.setContent(`<button id="before">Before</button><${profile.type} id="profile" type="${profile.props.type}">alice</${profile.type}><button id="after">After</button>`);
    await page.evaluate(() => {
      window.done = 0;
      document.querySelector('#profile').addEventListener('click', async () => {
        await window.activateProfile(); window.done++;
      });
    });
    const url = page.url();
    await page.locator('#profile').focus();
    await page.keyboard.press('Enter');
    await page.waitForFunction(() => window.done === 1);
    expect(activations).toBe(1);
    await page.keyboard.press('Space');
    await page.waitForFunction(() => window.done === 2);
    expect(activations).toBe(2);
    expect(page.url()).toBe(url);
    expect(forceNavigation).not.toHaveBeenCalled();
    await page.locator('#profile').evaluate((button) => { button.disabled = true; });
    await page.locator('#before').focus();
    await page.keyboard.press('Tab');
    expect(await page.locator('#after').evaluate((button) => document.activeElement === button)).toBe(true);
    await page.keyboard.press('Enter');
    await page.keyboard.press('Space');
    expect(activations).toBe(2);
    expect(page.url()).toBe(url);
  } finally { await browser.close(); }
});
