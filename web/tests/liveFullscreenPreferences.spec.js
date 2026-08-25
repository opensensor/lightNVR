import {
  ALWAYS_FULLSCREEN_ON_TAP_KEY,
  readAlwaysFullscreenOnTap,
  shouldEnterFullscreenFromTap,
  writeAlwaysFullscreenOnTap,
} from '../js/components/preact/useAlwaysFullscreenOnTap.js';

function memoryStorage() {
  const values = new Map();
  return {
    getItem: (key) => values.get(key) ?? null,
    setItem: (key, value) => values.set(key, value),
  };
}

describe('always-fullscreen preference', () => {
  test('persists an explicit boolean and defaults off', () => {
    const storage = memoryStorage();
    expect(readAlwaysFullscreenOnTap(storage)).toBe(false);
    writeAlwaysFullscreenOnTap(true, storage);
    expect(storage.getItem(ALWAYS_FULLSCREEN_ON_TAP_KEY)).toBe('true');
    expect(readAlwaysFullscreenOnTap(storage)).toBe(true);
  });

  test('ignores controls, zoomed tiles, and an existing fullscreen element', () => {
    const plainTarget = { closest: () => null };
    const controlTarget = { closest: () => ({}) };
    expect(shouldEnterFullscreenFromTap({ target: plainTarget }, true, false, {})).toBe(true);
    expect(shouldEnterFullscreenFromTap({ target: controlTarget }, true, false, {})).toBe(false);
    expect(shouldEnterFullscreenFromTap({ target: plainTarget }, true, true, {})).toBe(false);
    expect(shouldEnterFullscreenFromTap({ target: plainTarget }, true, false, { fullscreenElement: {} })).toBe(false);
  });
});
