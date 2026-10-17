/**
 * Behaviour tests for the useLiveSlideshow hook: the timer, pausing, and the
 * localStorage / URL sync. There is no DOM renderer in this project (see
 * jest.config.cjs), so preact/hooks is replaced with a small deps-aware shim
 * and the hook is re-run as a "component" whenever its state changes.
 */

const mockHarness = {
  slots: [],
  cursor: 0,
  pendingEffects: [],
  dirty: false,
};

function mockDepsChanged(prev, next) {
  if (!prev || !next) return true;
  return prev.length !== next.length || prev.some((value, i) => !Object.is(value, next[i]));
}

jest.mock('preact/hooks', () => ({
  useState: (initial) => {
    const slot = mockHarness.cursor++;
    if (!mockHarness.slots[slot]) {
      const value = typeof initial === 'function' ? initial() : initial;
      const cell = { value };
      cell.set = (next) => {
        const resolved = typeof next === 'function' ? next(cell.value) : next;
        if (!Object.is(resolved, cell.value)) {
          cell.value = resolved;
          mockHarness.dirty = true;
        }
      };
      mockHarness.slots[slot] = cell;
    }
    return [mockHarness.slots[slot].value, mockHarness.slots[slot].set];
  },
  useRef: (initial) => {
    const slot = mockHarness.cursor++;
    if (!mockHarness.slots[slot]) mockHarness.slots[slot] = { current: initial };
    return mockHarness.slots[slot];
  },
  useMemo: (fn, deps) => {
    const slot = mockHarness.cursor++;
    const cell = mockHarness.slots[slot];
    if (!cell || mockDepsChanged(cell.deps, deps)) mockHarness.slots[slot] = { value: fn(), deps };
    return mockHarness.slots[slot].value;
  },
  useCallback: (fn, deps) => {
    const slot = mockHarness.cursor++;
    const cell = mockHarness.slots[slot];
    if (!cell || mockDepsChanged(cell.deps, deps)) mockHarness.slots[slot] = { value: fn, deps };
    return mockHarness.slots[slot].value;
  },
  useEffect: (fn, deps) => {
    const slot = mockHarness.cursor++;
    const cell = mockHarness.slots[slot] || (mockHarness.slots[slot] = { deps: undefined, cleanup: null });
    if (mockDepsChanged(cell.deps, deps)) {
      cell.deps = deps;
      mockHarness.pendingEffects.push(() => {
        if (cell.cleanup) cell.cleanup();
        cell.cleanup = fn() || null;
      });
    }
  },
}));

const {
  LIVE_SLIDESHOW_INTERVAL_KEY,
  LIVE_SLIDESHOW_KEY,
  useLiveSlideshow,
} = require('../js/components/preact/useLiveSlideshow.js');

function memoryStorage(initial = {}) {
  const values = new Map(Object.entries(initial));
  return {
    getItem: (key) => values.get(key) ?? null,
    setItem: (key, value) => values.set(key, String(value)),
    removeItem: (key) => values.delete(key),
  };
}

function installBrowser({ search = '', storage = memoryStorage() } = {}) {
  const listeners = new Map();
  const doc = {
    hidden: false,
    addEventListener: (type, fn) => listeners.set(type, fn),
    removeEventListener: (type) => listeners.delete(type),
  };
  const location = new URL(`http://nvr.local/${search}`);
  const win = {
    location,
    history: {
      state: null,
      replaceState: jest.fn((_state, _title, url) => {
        win.location = new URL(String(url));
      }),
    },
  };
  global.document = doc;
  global.window = win;
  global.localStorage = storage;
  Object.defineProperty(global, 'location', { configurable: true, get: () => win.location });
  return {
    storage,
    url: () => win.location,
    setHidden(hidden) {
      doc.hidden = hidden;
      listeners.get('visibilitychange')?.();
    },
  };
}

function uninstallBrowser() {
  delete global.document;
  delete global.window;
  delete global.localStorage;
  delete global.location;
}

const stream = (name) => ({ name });

/**
 * Mount the hook with a mutable "parent" whose selectedStream / currentPage
 * are real state the hook can drive, like LiveView's.
 */
function mount(initialProps) {
  mockHarness.slots = [];
  mockHarness.pendingEffects = [];
  const parent = {
    props: {
      paused: false,
      isSingleStream: false,
      orderedStreams: ['a', 'b', 'c', 'd', 'e'].map(stream),
      selectedStream: '',
      currentPage: 0,
      maxStreams: 4,
      ...initialProps,
    },
    result: null,
  };
  parent.props.setSelectedStream = jest.fn((name) => { parent.props.selectedStream = name; mockHarness.dirty = true; });
  parent.props.setCurrentPage = jest.fn((page) => { parent.props.currentPage = page; mockHarness.dirty = true; });

  const render = () => {
    for (let pass = 0; pass < 20; pass += 1) {
      mockHarness.dirty = false;
      mockHarness.cursor = 0;
      parent.result = useLiveSlideshow(parent.props);
      const effects = mockHarness.pendingEffects.splice(0);
      effects.forEach((run) => run());
      if (!mockHarness.dirty) return;
    }
    throw new Error('render loop did not settle');
  };
  render();

  return {
    get result() { return parent.result; },
    get props() { return parent.props; },
    rerender(next = {}) { Object.assign(parent.props, next); render(); },
    act(fn) { fn(); render(); },
    tick(ms) { jest.advanceTimersByTime(ms); render(); },
  };
}

describe('useLiveSlideshow', () => {
  let browser;

  beforeEach(() => {
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
    uninstallBrowser();
  });

  test('stays idle until enabled', () => {
    browser = installBrowser();
    const view = mount();
    expect(view.result.enabled).toBe(false);
    expect(view.result.canCycle).toBe(true);
    expect(view.result.running).toBe(false);
    view.tick(60_000);
    expect(view.props.setCurrentPage).not.toHaveBeenCalled();
  });

  test('cycles grid pages on the interval and wraps to the first page', () => {
    browser = installBrowser();
    const view = mount();
    view.act(() => view.result.setEnabled(true));
    expect(view.result.running).toBe(true);

    view.tick(9_999);
    expect(view.props.currentPage).toBe(0);
    view.tick(1);
    expect(view.props.currentPage).toBe(1);
    view.tick(10_000);
    expect(view.props.currentPage).toBe(0);
    expect(view.props.setSelectedStream).not.toHaveBeenCalled();
  });

  test('cycles cameras one at a time in a 1x1 layout', () => {
    browser = installBrowser();
    const view = mount({ isSingleStream: true, maxStreams: 1, selectedStream: 'd' });
    view.act(() => view.result.setEnabled(true));

    view.tick(10_000);
    expect(view.props.selectedStream).toBe('e');
    view.tick(10_000);
    expect(view.props.selectedStream).toBe('a');
    expect(view.props.setCurrentPage).not.toHaveBeenCalled();
  });

  test('uses the chosen interval', () => {
    browser = installBrowser();
    const view = mount();
    view.act(() => { view.result.setEnabled(true); view.result.setInterval('30'); });
    expect(view.result.interval).toBe(30);
    view.tick(29_000);
    expect(view.props.currentPage).toBe(0);
    view.tick(1_000);
    expect(view.props.currentPage).toBe(1);
  });

  test('a manual page change restarts the countdown', () => {
    browser = installBrowser();
    const view = mount({ orderedStreams: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'].map(stream) });
    view.act(() => view.result.setEnabled(true));

    view.tick(8_000);
    view.rerender({ currentPage: 2 }); // operator clicked through to page 3
    view.tick(8_000);
    expect(view.props.currentPage).toBe(2);
    view.tick(2_000);
    expect(view.props.currentPage).toBe(0);
  });

  test('a stream refetch with the same cameras does not restart the countdown', () => {
    browser = installBrowser();
    const view = mount();
    view.act(() => view.result.setEnabled(true));

    view.tick(6_000);
    view.rerender({ orderedStreams: ['a', 'b', 'c', 'd', 'e'].map(stream) }); // new array, same names
    view.tick(4_000);
    expect(view.props.currentPage).toBe(1);
  });

  test('pauses while the caller says so and resumes with a full interval', () => {
    browser = installBrowser();
    const view = mount();
    view.act(() => view.result.setEnabled(true));

    view.tick(5_000);
    view.rerender({ paused: true });
    expect(view.result.running).toBe(false);
    view.tick(60_000);
    expect(view.props.currentPage).toBe(0);

    view.rerender({ paused: false });
    view.tick(9_000);
    expect(view.props.currentPage).toBe(0);
    view.tick(1_000);
    expect(view.props.currentPage).toBe(1);
  });

  test('pauses while the browser tab is hidden', () => {
    browser = installBrowser();
    const view = mount();
    view.act(() => view.result.setEnabled(true));

    view.act(() => browser.setHidden(true));
    expect(view.result.running).toBe(false);
    view.tick(60_000);
    expect(view.props.currentPage).toBe(0);

    view.act(() => browser.setHidden(false));
    view.tick(10_000);
    expect(view.props.currentPage).toBe(1);
  });

  test('does not run when everything fits on one screen', () => {
    browser = installBrowser({ search: '?slideshow=5' });
    const view = mount({ maxStreams: 9 });
    expect(view.result.enabled).toBe(true);
    expect(view.result.canCycle).toBe(false);
    expect(view.result.running).toBe(false);
    view.tick(60_000);
    expect(view.props.setCurrentPage).not.toHaveBeenCalled();
  });

  test('stops cycling once the camera list shrinks to a single page', () => {
    browser = installBrowser();
    const view = mount();
    view.act(() => view.result.setEnabled(true));
    view.rerender({ orderedStreams: ['a', 'b'].map(stream) });
    expect(view.result.canCycle).toBe(false);
    view.tick(60_000);
    expect(view.props.setCurrentPage).not.toHaveBeenCalled();
  });

  test('disabling clears the pending advance', () => {
    browser = installBrowser();
    const view = mount();
    view.act(() => view.result.setEnabled(true));
    view.tick(9_000);
    view.act(() => view.result.setEnabled(false));
    view.tick(60_000);
    expect(view.props.currentPage).toBe(0);
  });

  test('persists the preference and mirrors it to ?slideshow=', () => {
    browser = installBrowser({ search: '?cols=1&rows=1' });
    const view = mount();
    expect(browser.url().searchParams.has('slideshow')).toBe(false);

    view.act(() => { view.result.setEnabled(true); view.result.setInterval(15); });
    expect(browser.storage.getItem(LIVE_SLIDESHOW_KEY)).toBe('true');
    expect(browser.storage.getItem(LIVE_SLIDESHOW_INTERVAL_KEY)).toBe('15');
    expect(browser.url().searchParams.get('slideshow')).toBe('15');
    // Other live view params are left alone.
    expect(browser.url().searchParams.get('cols')).toBe('1');

    view.act(() => view.result.setEnabled(false));
    expect(browser.storage.getItem(LIVE_SLIDESHOW_KEY)).toBe('false');
    expect(browser.url().searchParams.has('slideshow')).toBe(false);
  });

  test('starts from a bookmarked ?slideshow= URL', () => {
    browser = installBrowser({ search: '?slideshow=5' });
    const view = mount();
    expect(view.result).toMatchObject({ enabled: true, interval: 5, running: true });
    view.tick(5_000);
    expect(view.props.currentPage).toBe(1);
  });
});
