import {
  ALWAYS_FULLSCREEN_ON_TAP_KEY,
  readAlwaysFullscreenOnTap,
  shouldToggleFullscreenFromTap,
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

  test('single tap toggles fullscreen for its own tile and ignores controls and zoom', () => {
    const plainTarget = { closest: () => null };
    const controlTarget = { closest: () => ({}) };
    const cell = {};
    expect(shouldToggleFullscreenFromTap({ target: plainTarget }, true, false, cell, null)).toBe(true);
    expect(shouldToggleFullscreenFromTap({ target: plainTarget }, false, false, cell, null)).toBe(false);
    expect(shouldToggleFullscreenFromTap({ target: controlTarget }, true, false, cell, null)).toBe(false);
    expect(shouldToggleFullscreenFromTap({ target: plainTarget }, true, true, cell, null)).toBe(false);
    // Once this tile is fullscreen, the next single tap exits.
    expect(shouldToggleFullscreenFromTap({ target: plainTarget }, true, false, cell, cell)).toBe(true);
    expect(shouldToggleFullscreenFromTap({ target: controlTarget }, true, false, cell, cell)).toBe(false);
    // Something else owning fullscreen is never exited by a tile tap.
    expect(shouldToggleFullscreenFromTap({ target: plainTarget }, true, false, cell, {})).toBe(false);
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
