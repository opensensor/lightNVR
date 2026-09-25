import {
  DEFAULT_LIVE_DISPLAY_MODE,
  LIVE_DISPLAY_MODES,
  LIVE_DISPLAY_MODE_KEY,
  displayedVideoBox,
  nextLiveDisplayMode,
  normalizeLiveDisplayMode,
  objectFitForLiveDisplayMode,
  readLiveDisplayMode,
  writeLiveDisplayMode,
} from '../js/components/preact/useLiveDisplayMode.js';

function memoryStorage() {
  const values = new Map();
  return {
    getItem: (key) => values.get(key) ?? null,
    setItem: (key, value) => values.set(key, value),
  };
}

describe('live display mode preference (#619)', () => {
  test('defaults to fit so existing installs keep the letterboxed grid', () => {
    expect(DEFAULT_LIVE_DISPLAY_MODE).toBe('fit');
    expect(LIVE_DISPLAY_MODES).toEqual(['fit', 'fill']);
    expect(readLiveDisplayMode(memoryStorage())).toBe('fit');
  });

  test('normalizes stored values and rejects garbage', () => {
    expect(normalizeLiveDisplayMode('fill')).toBe('fill');
    expect(normalizeLiveDisplayMode(' Fill ')).toBe('fill');
    expect(normalizeLiveDisplayMode('fit')).toBe('fit');
    expect(normalizeLiveDisplayMode('stretch')).toBe('fit');
    expect(normalizeLiveDisplayMode(null)).toBe('fit');
    expect(normalizeLiveDisplayMode(42)).toBe('fit');
  });

  test('persists the choice under the live preference key', () => {
    const storage = memoryStorage();
    writeLiveDisplayMode('fill', storage);
    expect(storage.getItem(LIVE_DISPLAY_MODE_KEY)).toBe('fill');
    expect(readLiveDisplayMode(storage)).toBe('fill');
    writeLiveDisplayMode('bogus', storage);
    expect(readLiveDisplayMode(storage)).toBe('fit');
  });

  test('tolerates storage that throws', () => {
    const broken = {
      getItem: () => { throw new Error('disabled'); },
      setItem: () => { throw new Error('disabled'); },
    };
    expect(readLiveDisplayMode(broken)).toBe('fit');
    expect(() => writeLiveDisplayMode('fill', broken)).not.toThrow();
  });

  test('toggles between the two modes and maps to object-fit', () => {
    expect(nextLiveDisplayMode('fit')).toBe('fill');
    expect(nextLiveDisplayMode('fill')).toBe('fit');
    expect(nextLiveDisplayMode(undefined)).toBe('fill');
    expect(objectFitForLiveDisplayMode('fit')).toBe('contain');
    expect(objectFitForLiveDisplayMode('fill')).toBe('cover');
    expect(objectFitForLiveDisplayMode('nonsense')).toBe('contain');
  });
});

describe('displayedVideoBox', () => {
  test('letterboxes a wide frame in a squarer box under contain', () => {
    // 16:9 frame inside a 4:3 box: full width, bars top and bottom.
    expect(displayedVideoBox(400, 300, 1920, 1080, 'contain')).toEqual({
      drawWidth: 400,
      drawHeight: 225,
      offsetX: 0,
      offsetY: 37.5,
    });
  });

  test('pillarboxes a tall frame in a wide box under contain', () => {
    // 4:3 frame inside a 16:9 box: full height, bars left and right.
    expect(displayedVideoBox(320, 180, 640, 480, 'contain')).toEqual({
      drawWidth: 240,
      drawHeight: 180,
      offsetX: 40,
      offsetY: 0,
    });
  });

  test('overflows the box under cover so the tile is filled', () => {
    // 16:9 frame covering a 4:3 box: full height, width overflows both sides.
    expect(displayedVideoBox(400, 300, 1920, 1080, 'cover')).toEqual({
      drawWidth: 400 * (4 / 3),
      drawHeight: 300,
      offsetX: (400 - 400 * (4 / 3)) / 2,
      offsetY: 0,
    });
    // 4:3 frame covering a 16:9 box: full width, height overflows top/bottom.
    expect(displayedVideoBox(320, 180, 640, 480, 'cover')).toEqual({
      drawWidth: 320,
      drawHeight: 240,
      offsetX: 0,
      offsetY: -30,
    });
  });

  test('fills the box exactly when aspects match in either mode', () => {
    for (const objectFit of ['contain', 'cover']) {
      expect(displayedVideoBox(320, 180, 1280, 720, objectFit)).toEqual({
        drawWidth: 320,
        drawHeight: 180,
        offsetX: 0,
        offsetY: 0,
      });
    }
  });

  test('falls back to the whole box when dimensions are unknown', () => {
    expect(displayedVideoBox(320, 180, 0, 0, 'cover')).toEqual({
      drawWidth: 320,
      drawHeight: 180,
      offsetX: 0,
      offsetY: 0,
    });
    expect(displayedVideoBox(0, 0, 1280, 720)).toEqual({
      drawWidth: 0,
      drawHeight: 0,
      offsetX: 0,
      offsetY: 0,
    });
  });
});
