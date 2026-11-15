/**
 * The error-overlay pieces of the last known image (#974): the backdrop and
 * caption components, and the useLastFrame hook that the MSE, HLS and WebRTC
 * cells share. There is no DOM renderer in this project, so the components are
 * called directly and the returned vnodes inspected, and preact/hooks is
 * replaced with a small deps-aware shim to drive the hook.
 */

const mockHarness = { slots: [], cursor: 0, pendingEffects: [], dirty: false, cleanups: [] };

function mockDepsChanged(prev, next) {
  if (!prev || !next) return true;
  return prev.length !== next.length || prev.some((value, i) => !Object.is(value, next[i]));
}

jest.mock('preact/hooks', () => ({
  useState: (initial) => {
    const slot = mockHarness.cursor++;
    if (!mockHarness.slots[slot]) {
      const cell = { value: typeof initial === 'function' ? initial() : initial };
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
  useCallback: (fn, deps) => {
    const slot = mockHarness.cursor++;
    const cell = mockHarness.slots[slot];
    if (!cell || mockDepsChanged(cell.deps, deps)) mockHarness.slots[slot] = { value: fn, deps };
    return mockHarness.slots[slot].value;
  },
  useEffect: (fn, deps) => {
    const slot = mockHarness.cursor++;
    const cell = mockHarness.slots[slot] || (mockHarness.slots[slot] = { deps: undefined, cleanup: null });
    if (!mockHarness.cleanups.includes(cell)) mockHarness.cleanups.push(cell);
    if (mockDepsChanged(cell.deps, deps)) {
      cell.deps = deps;
      mockHarness.pendingEffects.push(() => {
        if (cell.cleanup) cell.cleanup();
        cell.cleanup = fn() || null;
      });
    }
  },
}));

jest.mock('../js/i18n.ts', () => ({
  useI18n: () => ({
    t: (key, params) => (params ? `${key}:${JSON.stringify(params)}` : key),
    locale: 'en-US',
  }),
}));

const { LastFrameBackdrop, LastFrameCaption } = require('../js/components/preact/live/LastFrameFallback.tsx');
const { lastFrameOverlayBackground, formatLastImageTime } = require('../js/components/preact/live/lastFrame.ts');
const { useLastFrame } = require('../js/components/preact/live/useLastFrame.ts');

const flush = async () => {
  for (let i = 0; i < 10; i++) await new Promise((resolve) => setImmediate(resolve));
};

describe('error overlay with and without a last image', () => {
  const image = { url: '/api/recordings/thumbnail/7/2', capturedAt: Date.now(), source: 'recording' };

  test('renders the image behind the overlay content and the caption', () => {
    const backdrop = LastFrameBackdrop({ image });
    expect(backdrop.type).toBe('img');
    expect(backdrop.props.src).toBe(image.url);
    expect(backdrop.props.alt).toBe('');
    expect(backdrop.props['aria-hidden']).toBe('true');
    expect(backdrop.props.style.zIndex).toBe(-1);
    expect(backdrop.props.style.pointerEvents).toBe('none');
    expect(backdrop.props['data-last-frame-source']).toBe('recording');

    const caption = LastFrameCaption({ image });
    expect(caption.type).toBe('p');
    expect(caption.props.children).toBe(
      `live.lastImageCaption:${JSON.stringify({ time: formatLastImageTime(image.capturedAt, 'en-US') })}`
    );
    expect(lastFrameOverlayBackground(image, 'rgba(0, 0, 0, 0.8)')).toBe('rgba(0, 0, 0, 0.55)');
  });

  test('a failed image hides itself instead of showing a broken icon', () => {
    const backdrop = LastFrameBackdrop({ image });
    const style = {};
    backdrop.props.onError({ currentTarget: { style } });
    expect(style.display).toBe('none');
  });

  test('renders exactly as before when there is no image', () => {
    expect(LastFrameBackdrop({ image: null })).toBeNull();
    expect(LastFrameCaption({ image: null })).toBeNull();
    expect(LastFrameBackdrop({ image: undefined })).toBeNull();
    expect(lastFrameOverlayBackground(null, 'rgba(0, 0, 0, 0.8)')).toBe('rgba(0, 0, 0, 0.8)');
    expect(lastFrameOverlayBackground(null, 'rgba(0, 0, 0, 0.7)')).toBe('rgba(0, 0, 0, 0.7)');
  });
});

describe('useLastFrame', () => {
  function fakeVideo() {
    const listeners = new Map();
    return {
      videoWidth: 640,
      videoHeight: 360,
      readyState: 4,
      addEventListener: jest.fn((type, fn) => listeners.set(type, fn)),
      removeEventListener: jest.fn((type) => listeners.delete(type)),
      emit: (type) => listeners.get(type)?.(),
      listeners,
    };
  }

  function mount(initialProps) {
    mockHarness.slots = [];
    mockHarness.pendingEffects = [];
    mockHarness.cleanups = [];
    const video = fakeVideo();
    const deps = {
      loadRecordingFallback: jest.fn().mockResolvedValue({ url: '/api/recordings/thumbnail/7/2', capturedAt: 5, source: 'recording' }),
      captureFrame: jest.fn((v) => (v && v.videoWidth ? {} : null)),
      toBlob: jest.fn().mockResolvedValue({ size: 1 }),
      createObjectURL: jest.fn(() => 'blob:frame'),
      revokeObjectURL: jest.fn(),
      now: () => 1000,
    };
    const state = { props: { streamName: 'garage', hasError: false, enabled: true, ...initialProps }, result: null };
    const videoRef = { current: video };
    const render = () => {
      for (let pass = 0; pass < 10; pass++) {
        mockHarness.cursor = 0;
        mockHarness.dirty = false;
        state.result = useLastFrame({ ...state.props, videoRef, deps });
        const effects = mockHarness.pendingEffects.splice(0);
        effects.forEach((run) => run());
        if (!mockHarness.dirty) return;
      }
    };
    render();
    return {
      video,
      deps,
      get image() { return state.result.image; },
      capture: () => state.result.capture(),
      update: async (props) => {
        Object.assign(state.props, props);
        render();
        await flush();
        render();
      },
      settle: async () => { await flush(); render(); },
      unmount: () => mockHarness.cleanups.forEach((cell) => cell.cleanup && cell.cleanup()),
    };
  }

  test('shows the frame captured when the error appears and removes it when the video plays', async () => {
    const tile = mount();
    expect(tile.video.addEventListener).toHaveBeenCalledWith('playing', expect.any(Function));
    tile.video.emit('playing');
    await tile.update({ hasError: true });
    expect(tile.image).toEqual({ url: 'blob:frame', capturedAt: 1000, source: 'frame' });
    expect(tile.deps.loadRecordingFallback).not.toHaveBeenCalled();

    await tile.update({ hasError: false });
    expect(tile.image).toBeNull();
    tile.video.emit('playing');
    expect(tile.deps.revokeObjectURL).toHaveBeenCalledWith('blob:frame');
  });

  test('never played: one recording lookup, reused while the error persists', async () => {
    const tile = mount();
    await tile.update({ hasError: true });
    expect(tile.image.source).toBe('recording');
    await tile.update({ hasError: true });
    await tile.update({ hasError: false });
    await tile.update({ hasError: true });
    expect(tile.deps.loadRecordingFallback).toHaveBeenCalledTimes(1);
  });

  test('disabled: no capture, no lookup, no image', async () => {
    const tile = mount({ enabled: false });
    tile.video.emit('playing');
    expect(tile.capture()).toBe(false);
    await tile.update({ hasError: true });
    expect(tile.image).toBeNull();
    expect(tile.deps.captureFrame).not.toHaveBeenCalled();
    expect(tile.deps.loadRecordingFallback).not.toHaveBeenCalled();
  });

  test('unmount removes the playing listener and does not capture afterwards', async () => {
    const tile = mount();
    tile.video.emit('playing');
    tile.unmount();
    expect(tile.video.removeEventListener).toHaveBeenCalledWith('playing', expect.any(Function));
    expect(tile.capture()).toBe(false);
    await flush();
    expect(tile.deps.createObjectURL).not.toHaveBeenCalled();
  });
});
