import {
  DEFAULT_LIVE_SLIDESHOW_INTERVAL,
  LIVE_SLIDESHOW_INTERVAL_KEY,
  LIVE_SLIDESHOW_KEY,
  nextSlideshowTarget,
  normalizeSlideshowInterval,
  readLiveSlideshow,
  slideshowSlideCount,
  writeLiveSlideshow,
} from '../js/components/preact/useLiveSlideshow.js';

function memoryStorage(initial = {}) {
  const values = new Map(Object.entries(initial));
  return {
    getItem: (key) => values.get(key) ?? null,
    setItem: (key, value) => values.set(key, value),
  };
}

const names = ['front', 'back', 'garage', 'side', 'drive'];

describe('live slideshow preference', () => {
  test('is off by default with a 10 second interval', () => {
    expect(readLiveSlideshow(memoryStorage(), '')).toEqual({ enabled: false, interval: 10 });
    expect(DEFAULT_LIVE_SLIDESHOW_INTERVAL).toBe(10);
  });

  test('round-trips through storage', () => {
    const storage = memoryStorage();
    writeLiveSlideshow({ enabled: true, interval: 30 }, storage);
    expect(storage.getItem(LIVE_SLIDESHOW_KEY)).toBe('true');
    expect(storage.getItem(LIVE_SLIDESHOW_INTERVAL_KEY)).toBe('30');
    expect(readLiveSlideshow(storage, '')).toEqual({ enabled: true, interval: 30 });
  });

  test('a ?slideshow= URL param overrides storage so kiosks can bookmark it', () => {
    const storage = memoryStorage({ [LIVE_SLIDESHOW_KEY]: 'false', [LIVE_SLIDESHOW_INTERVAL_KEY]: '5' });
    expect(readLiveSlideshow(storage, '?slideshow=15')).toEqual({ enabled: true, interval: 15 });
    expect(readLiveSlideshow(memoryStorage({ [LIVE_SLIDESHOW_KEY]: 'true' }), '?slideshow=0').enabled).toBe(false);
  });

  test('rejects intervals outside the offered list', () => {
    expect(normalizeSlideshowInterval('60')).toBe(60);
    expect(normalizeSlideshowInterval('1')).toBe(DEFAULT_LIVE_SLIDESHOW_INTERVAL);
    expect(normalizeSlideshowInterval('abc')).toBe(DEFAULT_LIVE_SLIDESHOW_INTERVAL);
    expect(readLiveSlideshow(memoryStorage(), '?slideshow=7')).toEqual({ enabled: true, interval: 10 });
  });

  test('survives storage that throws', () => {
    const broken = { getItem: () => { throw new Error('denied'); }, setItem: () => { throw new Error('denied'); } };
    expect(readLiveSlideshow(broken, '')).toEqual({ enabled: false, interval: 10 });
    expect(() => writeLiveSlideshow({ enabled: true, interval: 5 }, broken)).not.toThrow();
  });
});

describe('live slideshow advancing', () => {
  test('counts cameras in 1x1 and pages in a grid', () => {
    expect(slideshowSlideCount({ isSingleStream: true, streamCount: 5, maxStreams: 1 })).toBe(5);
    expect(slideshowSlideCount({ isSingleStream: false, streamCount: 5, maxStreams: 4 })).toBe(2);
    expect(slideshowSlideCount({ isSingleStream: false, streamCount: 0, maxStreams: 4 })).toBe(0);
  });

  test('1x1 cycles to the next camera and wraps', () => {
    const base = { isSingleStream: true, streamNames: names, currentPage: 0, maxStreams: 1 };
    expect(nextSlideshowTarget({ ...base, selectedStream: 'front' })).toEqual({ selectedStream: 'back' });
    expect(nextSlideshowTarget({ ...base, selectedStream: 'drive' })).toEqual({ selectedStream: 'front' });
    // A selection that no longer exists restarts from the first camera.
    expect(nextSlideshowTarget({ ...base, selectedStream: 'gone' })).toEqual({ selectedStream: 'front' });
  });

  test('a grid cycles pages and wraps', () => {
    const base = { isSingleStream: false, streamNames: names, selectedStream: '', maxStreams: 4 };
    expect(nextSlideshowTarget({ ...base, currentPage: 0 })).toEqual({ page: 1 });
    expect(nextSlideshowTarget({ ...base, currentPage: 1 })).toEqual({ page: 0 });
  });

  test('does nothing when everything already fits on screen', () => {
    expect(nextSlideshowTarget({
      isSingleStream: false, streamNames: names, selectedStream: '', currentPage: 0, maxStreams: 9,
    })).toBeNull();
    expect(nextSlideshowTarget({
      isSingleStream: true, streamNames: ['front'], selectedStream: 'front', currentPage: 0, maxStreams: 1,
    })).toBeNull();
  });
});
