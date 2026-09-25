import {
  FILTER_OPTIONS_ERROR,
  FILTER_OPTIONS_IDLE,
  FILTER_OPTIONS_LOADED,
  FILTER_OPTIONS_LOADING,
  areFilterOptionsInteractive,
  createCachedFilterOptionsLoader,
  shouldLoadFilterOptions
} from '../js/components/preact/recordings/lazyFilterOptions.js';

describe('lazy recordings filter options', () => {
  test('do not load while their accordion section is collapsed', () => {
    expect(shouldLoadFilterOptions(false, FILTER_OPTIONS_IDLE)).toBe(false);
    expect(shouldLoadFilterOptions(true, FILTER_OPTIONS_IDLE)).toBe(true);
    expect(shouldLoadFilterOptions(true, FILTER_OPTIONS_LOADING)).toBe(false);
    expect(shouldLoadFilterOptions(true, FILTER_OPTIONS_LOADED)).toBe(false);
    expect(shouldLoadFilterOptions(true, FILTER_OPTIONS_ERROR)).toBe(false);
  });

  test('keep the native select disabled until its options are present', () => {
    expect(areFilterOptionsInteractive(FILTER_OPTIONS_IDLE)).toBe(false);
    expect(areFilterOptionsInteractive(FILTER_OPTIONS_LOADING)).toBe(false);
    expect(areFilterOptionsInteractive(FILTER_OPTIONS_ERROR)).toBe(false);
    expect(areFilterOptionsInteractive(FILTER_OPTIONS_LOADED)).toBe(true);
  });
});

describe('cached filter option loader', () => {
  test('reuses a fresh result instead of refetching on every sidebar mount', async () => {
    const loader = jest.fn().mockResolvedValue(['car', 'person']);
    let clock = 1000;
    const load = createCachedFilterOptionsLoader(loader, { staleTime: 500, now: () => clock });

    await expect(load()).resolves.toEqual(['car', 'person']);
    clock += 100;
    await expect(load()).resolves.toEqual(['car', 'person']);
    expect(loader).toHaveBeenCalledTimes(1);

    clock += 500;
    await expect(load()).resolves.toEqual(['car', 'person']);
    expect(loader).toHaveBeenCalledTimes(2);
  });

  test('shares one in-flight request between concurrent callers', async () => {
    let resolve;
    const loader = jest.fn(() => new Promise((r) => { resolve = r; }));
    const load = createCachedFilterOptionsLoader(loader);

    const first = load();
    const second = load();
    expect(loader).toHaveBeenCalledTimes(1);

    resolve(['dog']);
    await expect(first).resolves.toEqual(['dog']);
    await expect(second).resolves.toEqual(['dog']);
  });

  test('does not cache failures, so a retry fetches again', async () => {
    const error = new Error('timeout');
    const loader = jest.fn().mockRejectedValueOnce(error).mockResolvedValueOnce(['bird']);
    const load = createCachedFilterOptionsLoader(loader);

    await expect(load()).rejects.toBe(error);
    await expect(load()).resolves.toEqual(['bird']);
    expect(loader).toHaveBeenCalledTimes(2);
  });

  test('invalidate forces the next call to refetch', async () => {
    const loader = jest.fn().mockResolvedValue(['cat']);
    const load = createCachedFilterOptionsLoader(loader);

    await load();
    load.invalidate();
    await load();
    expect(loader).toHaveBeenCalledTimes(2);
  });
});
