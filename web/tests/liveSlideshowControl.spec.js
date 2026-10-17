/**
 * LiveSlideshowControl render tests. The component is bundled with esbuild
 * (Jest's Babel config has no JSX transform) with i18n left external and stubbed, then called
 * directly so the returned preact vnode tree can be inspected.
 */
const path = require('path');
const { buildSync } = require('esbuild');

function loadControl() {
  const output = buildSync({
    absWorkingDir: path.resolve(__dirname, '..'),
    entryPoints: ['js/components/preact/LiveSlideshowControl.jsx'],
    bundle: true,
    format: 'cjs',
    platform: 'node',
    jsx: 'automatic',
    jsxImportSource: 'preact',
    write: false,
    logLevel: 'silent',
    external: ['*/i18n.js'],
  }).outputFiles[0].text;
  const loaded = { exports: {} };
  const i18nStub = { useI18n: () => ({ t: (key, params) => (params ? `${key}:${JSON.stringify(params)}` : key) }) };
  const requireWithStub = (id) => (id.endsWith('i18n.js') ? i18nStub : require(id));
  Function('module', 'exports', 'require', output)(loaded, loaded.exports, requireWithStub);
  return loaded.exports;
}

const { LiveSlideshowControl } = loadControl();

function findAll(node, type, found = []) {
  if (!node || typeof node !== 'object') return found;
  if (Array.isArray(node)) {
    node.forEach((child) => findAll(child, type, found));
    return found;
  }
  if (node.type === type) found.push(node);
  findAll(node.props?.children, type, found);
  return found;
}

function render(props) {
  const handlers = { onToggle: jest.fn(), onIntervalChange: jest.fn() };
  const tree = LiveSlideshowControl({ enabled: false, interval: 10, isSingleStream: false, ...handlers, ...props });
  return { tree, handlers, button: findAll(tree, 'button')[0], select: findAll(tree, 'select')[0] };
}

describe('LiveSlideshowControl', () => {
  test('off: offers to start cycling pages and hides the interval picker', () => {
    const { button, select, handlers } = render({ enabled: false });
    expect(button.props['aria-pressed']).toBe(false);
    expect(button.props['aria-label']).toBe('live.slideshowStartPages');
    expect(select).toBeUndefined();

    button.props.onClick();
    expect(handlers.onToggle).toHaveBeenCalledWith(true);
  });

  test('off in a 1x1 layout: describes cycling cameras', () => {
    const { button } = render({ enabled: false, isSingleStream: true });
    expect(button.props['aria-label']).toBe('live.slideshowStartCameras');
  });

  test('on: shows stop and the interval picker with every offered interval', () => {
    const { button, select, handlers } = render({ enabled: true, interval: 30 });
    expect(button.props['aria-pressed']).toBe(true);
    expect(button.props['aria-label']).toBe('live.slideshowStop');

    button.props.onClick();
    expect(handlers.onToggle).toHaveBeenCalledWith(false);

    expect(select.props.value).toBe(30);
    const options = findAll(select, 'option').map((option) => option.props.value);
    expect(options).toEqual([5, 10, 15, 30, 60]);

    select.props.onChange({ currentTarget: { value: '60' } });
    expect(handlers.onIntervalChange).toHaveBeenCalledWith('60');
  });
});
