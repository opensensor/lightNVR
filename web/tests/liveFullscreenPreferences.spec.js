import {
  ALWAYS_FULLSCREEN_ON_TAP_KEY,
  readAlwaysFullscreenOnTap,
  shouldEnterFullscreenFromTap,
  shouldToggleFullscreenFromDoubleClick,
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

  test('single-tap entry ignores controls, zoomed tiles, and existing fullscreen', () => {
    const plainTarget = { closest: () => null };
    const controlTarget = { closest: () => ({}) };
    expect(shouldEnterFullscreenFromTap({ target: plainTarget }, true, false, {})).toBe(true);
    expect(shouldEnterFullscreenFromTap({ target: controlTarget }, true, false, {})).toBe(false);
    expect(shouldEnterFullscreenFromTap({ target: plainTarget }, true, true, {})).toBe(false);
    expect(shouldEnterFullscreenFromTap({ target: plainTarget }, true, false, { fullscreenElement: {} })).toBe(false);
  });

  test('double-click remains a toggle while native fullscreen is active', () => {
    const plainTarget = { closest: () => null };
    const controlTarget = { closest: () => ({}) };
    const previousDocument = global.document;
    global.document = { fullscreenElement: {} };

    try {
      expect(shouldToggleFullscreenFromDoubleClick(
        { target: plainTarget },
        false,
        false
      )).toBe(true);
      expect(shouldToggleFullscreenFromDoubleClick(
        { target: controlTarget },
        false,
        false
      )).toBe(false);
      expect(shouldToggleFullscreenFromDoubleClick(
        { target: plainTarget },
        true,
        false
      )).toBe(false);
      expect(shouldToggleFullscreenFromDoubleClick(
        { target: plainTarget },
        false,
        true
      )).toBe(false);
    } finally {
      if (previousDocument === undefined) delete global.document;
      else global.document = previousDocument;
    }
  });
});
