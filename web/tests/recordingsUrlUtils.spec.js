import { urlUtils } from '../js/components/preact/recordings/urlUtils.js';

describe('recordings collection URL state', () => {
  const originalWindow = global.window;

  afterEach(() => {
    global.window = originalWindow;
  });

  test('loads a collection as a compact mutually exclusive stream filter', () => {
    global.window = {
      location: { search: '?collection=collection-a&stream=cam-a,cam-b' },
    };

    const state = urlUtils.getFiltersFromUrl();
    expect(state.filters.collectionUuid).toBe('collection-a');
    expect(state.filters.streamIds).toEqual([]);
  });

  test('labels a selected collection by name in active filters', () => {
    const filters = {
      ...urlUtils.createDefaultFilters(),
      collectionUuid: 'collection-a',
    };
    expect(urlUtils.getActiveFiltersDisplay(filters, [
      { uuid: 'collection-a', name: 'North Campus' },
    ])).toContainEqual({
      key: 'collectionUuid',
      value: 'collection-a',
      label: 'Collection: North Campus',
    });
  });
});
