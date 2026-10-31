/** T7.176: preserve menu child semantics while giving its composite container
 * a programmatic focus target. Optional browser test mounts real Preact. */
const mockMenuRef = { current: null };
const mockEffects = [];
jest.mock('preact/hooks', () => ({
  useRef: () => mockMenuRef,
  useEffect: (effect) => mockEffects.push(effect),
  useState: jest.fn(),
}));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: (key) => `translated:${key}` }) }));
jest.mock('../../js/utils/date-utils.js', () => ({ formatFilenameTimestamp: jest.fn() }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
const { MobileTileContextMenu } = require('../../js/components/preact/MobileTileGestures.jsx');

function fixture(options = {}) {
  const gestures = {
    menuOpen: true, canReorder: true, closeMenu: jest.fn(), toggleAudio: jest.fn(),
    takeSnapshot: jest.fn(), openRecordings: jest.fn(), requestReorder: jest.fn(),
    ...options.gestures,
  };
  const tree = MobileTileContextMenu({ audioEnabled: false, ...options, gestures });
  const menu = tree?.props.children;
  const buttons = menu?.props.children.flat().filter(Boolean) ?? [];
  return { gestures, tree, menu, buttons };
}
beforeEach(() => { mockMenuRef.current = null; mockEffects.length = 0; });

it('exposes a programmatically focusable menu without adding a container Tab stop', () => {
  const { menu } = fixture();
  expect(menu.type).toBe('div');
  expect(menu.props.role).toBe('menu');
  expect(menu.props.tabIndex).toBe(-1);
  expect(menu.props['aria-label']).toBe('translated:live.tileActions');
});

it('keeps first-child initial focus, including when audio is unavailable', () => {
  const { buttons } = fixture({ audioAvailable: false });
  expect(buttons[0].props.children).toBe('translated:live.takeSnapshot');
  const focus = jest.fn();
  mockMenuRef.current = { querySelector: jest.fn(() => ({ focus })) };
  mockEffects.forEach((effect) => effect());
  expect(mockMenuRef.current.querySelector).toHaveBeenCalledWith('button');
  expect(focus).toHaveBeenCalledTimes(1);
});

it('does not render or move focus when the menu is closed', () => {
  const focus = jest.fn();
  mockMenuRef.current = { querySelector: jest.fn(() => ({ focus })) };
  const { tree } = fixture({ gestures: { menuOpen: false } });
  mockEffects.forEach((effect) => effect());
  expect(tree).toBeNull();
  expect(focus).not.toHaveBeenCalled();
});

it('retains native action buttons and invokes each callback once without backdrop activation', () => {
  const { buttons, gestures } = fixture();
  const handlers = ['toggleAudio', 'takeSnapshot', 'openRecordings', 'requestReorder'];
  expect(buttons).toHaveLength(4);
  buttons.forEach((button, index) => {
    expect(button.type).toBe('button');
    expect(button.props.type).toBe('button');
    expect(button.props.role).toBe('menuitem');
    expect(button.props.onKeyDown).toBeUndefined(); // Native Enter/Space; no duplicate synthetic activation.
    const event = { preventDefault: jest.fn(), stopPropagation: jest.fn() };
    button.props.onClick(event);
    expect(gestures[handlers[index]]).toHaveBeenCalledTimes(1);
    expect(event.preventDefault).toHaveBeenCalledTimes(1);
    expect(event.stopPropagation).toHaveBeenCalledTimes(1);
  });
  expect(gestures.closeMenu).not.toHaveBeenCalled();
});

it('handles Escape once without turning container Enter/Space into actions', () => {
  const { menu, gestures } = fixture();
  ['Enter', ' ', 'ArrowDown'].forEach((key) => menu.props.onKeyDown({ key }));
  expect(gestures.closeMenu).not.toHaveBeenCalled();
  menu.props.onKeyDown({ key: 'Escape' });
  expect(gestures.closeMenu).toHaveBeenCalledTimes(1);
  expect(gestures.toggleAudio).not.toHaveBeenCalled();
  expect(gestures.takeSnapshot).not.toHaveBeenCalled();
});

it('keeps click containment, backdrop dismissal and optional actions', () => {
  const { tree, menu, gestures, buttons } = fixture({ audioAvailable: false, canReorder: false });
  expect(buttons.map((button) => button.props.children)).toEqual([
    'translated:live.takeSnapshot', 'translated:live.openStreamRecordings',
  ]);
  const event = { preventDefault: jest.fn(), stopPropagation: jest.fn() };
  menu.props.onClick(event);
  expect(event.stopPropagation).toHaveBeenCalledTimes(1);
  expect(gestures.closeMenu).not.toHaveBeenCalled();
  tree.props.onClick(event);
  expect(gestures.closeMenu).toHaveBeenCalledTimes(1);
});

const browserTest = process.env.T7_176_BROWSER === '1' ? it : it.skip;
browserTest('real Preact menu focuses children, handles Escape and activates each native child once', async () => {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const path = require('node:path');
  const result = await build({
    stdin: {
      contents: `
        import { h, render } from 'preact';
        import { MobileTileContextMenu } from './js/components/preact/MobileTileGestures.jsx';
        const root = document.querySelector('#root');
        window.calls = { close: 0, audio: 0, snapshot: 0, recordings: 0, reorder: 0 };
        const action = (name) => () => { window.calls[name]++; render(null, root); };
        window.showMenu = (options = {}) => render(h(MobileTileContextMenu, {
          gestures: { menuOpen: true, canReorder: true, closeMenu: action('close'),
            toggleAudio: action('audio'), takeSnapshot: action('snapshot'),
            openRecordings: action('recordings'), requestReorder: action('reorder') },
          audioEnabled: false, ...options
        }), root);
      `,
      resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx',
    },
    bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact',
    plugins: [{ name: 'test-service-boundaries', setup(builder) {
      builder.onResolve({ filter: /(?:i18n|date-utils|ToastContainer)\.(?:js|jsx)$/ },
        (args) => ({ path: args.path, namespace: 'test-boundary' }));
      builder.onLoad({ filter: /.*/, namespace: 'test-boundary' }, () => ({
        contents: `export const useI18n = () => ({ t: key => 'translated:' + key });
          export const formatFilenameTimestamp = () => 'test';
          export const showStatusMessage = () => {};`, loader: 'js',
      }));
    } }],
  });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    await page.setContent('<button id="before">Before</button><div id="root"></div><button id="after">After</button>');
    await page.addScriptTag({ content: result.outputFiles[0].text });
    await page.evaluate(() => window.showMenu());
    await page.waitForFunction(() => document.activeElement?.textContent === 'translated:live.unmuteCameraAudio');
    const menu = page.getByRole('menu');
    expect(await menu.getAttribute('tabindex')).toBe('-1');
    await page.locator('#before').focus();
    await page.keyboard.press('Tab');
    expect(await page.getByRole('menuitem').first().evaluate((node) => document.activeElement === node)).toBe(true);
    await menu.focus();
    expect(await menu.evaluate((node) => document.activeElement === node)).toBe(true);
    await page.keyboard.press('Enter');
    await page.keyboard.press('Space');
    expect(await page.evaluate(() => Object.values(window.calls).reduce((sum, n) => sum + n, 0))).toBe(0);
    await page.keyboard.press('Escape');
    await page.waitForFunction(() => !document.querySelector('[role="menu"]'));
    expect(await page.evaluate(() => window.calls.close)).toBe(1);

    const actions = [
      ['audio', 'live.unmuteCameraAudio'], ['snapshot', 'live.takeSnapshot'],
      ['recordings', 'live.openStreamRecordings'], ['reorder', 'live.enterReorderMode'],
    ];
    for (const [name, label] of actions) {
      for (const key of ['Enter', 'Space']) {
        await page.evaluate(() => window.showMenu());
        await page.waitForFunction(() => document.activeElement?.textContent === 'translated:live.unmuteCameraAudio');
        const before = await page.evaluate((action) => window.calls[action], name);
        await page.getByRole('menuitem', { name: `translated:${label}`, exact: true }).focus();
        await page.keyboard.press(key);
        await page.waitForFunction(() => !document.querySelector('[role="menu"]'));
        expect(await page.evaluate((action) => window.calls[action], name)).toBe(before + 1);
        expect(await page.evaluate(() => window.calls.close)).toBe(1);
      }
    }
    await page.evaluate(() => window.showMenu({ audioAvailable: false, canReorder: false }));
    await page.waitForFunction(() => document.activeElement?.textContent === 'translated:live.takeSnapshot');
    expect(await page.getByRole('menuitem').count()).toBe(2);
    await page.keyboard.press('Escape');
    await page.waitForFunction(() => !document.querySelector('[role="menu"]'));
    expect(await page.evaluate(() => window.calls.close)).toBe(2);
  } finally { await browser.close(); }
});
