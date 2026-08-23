import {
  applyBulkOrganization,
  buildLocationRows,
  locationParentOptions,
  resolveTagAssignments,
  runBounded,
} from '../js/components/preact/fleet/fleetOrganization.js';

describe('fleet organization helpers', () => {
  const locations = [
    { uuid: 'floor', parent_uuid: 'building', name: 'Floor 2', sort_order: 20 },
    { uuid: 'site', parent_uuid: null, name: 'SJC', sort_order: 10 },
    { uuid: 'building', parent_uuid: 'site', name: 'Building A', sort_order: 10 },
    { uuid: 'lobby', parent_uuid: 'building', name: 'Lobby', sort_order: 10 },
  ];

  test('turns flat locations into stable, path-aware hierarchy rows', () => {
    expect(buildLocationRows(locations).map(({ uuid, depth, path }) => ({ uuid, depth, path }))).toEqual([
      { uuid: 'site', depth: 0, path: 'SJC' },
      { uuid: 'building', depth: 1, path: 'SJC / Building A' },
      { uuid: 'lobby', depth: 2, path: 'SJC / Building A / Lobby' },
      { uuid: 'floor', depth: 2, path: 'SJC / Building A / Floor 2' },
    ]);
  });

  test('excludes a location and descendants from its parent choices', () => {
    expect(locationParentOptions(locations, 'building').map((item) => item.uuid)).toEqual(['site']);
  });

  test('resolves non-destructive tag operations', () => {
    expect(resolveTagAssignments(['a', 'b'], 'add', ['b', 'c'])).toEqual(['a', 'b', 'c']);
    expect(resolveTagAssignments(['a', 'b'], 'remove', ['b', 'c'])).toEqual(['a']);
    expect(resolveTagAssignments(['a', 'b'], 'replace', ['c'])).toEqual(['c']);
    expect(resolveTagAssignments(['a'], 'none', ['b'])).toEqual(['a']);
  });

  test('runs workers with bounded concurrency and preserves result order', async () => {
    let active = 0;
    let peak = 0;
    const results = await runBounded([1, 2, 3, 4, 5], async (value) => {
      active += 1;
      peak = Math.max(peak, active);
      await Promise.resolve();
      active -= 1;
      return value * 2;
    }, 2);

    expect(peak).toBeLessThanOrEqual(2);
    expect(results.map((result) => result.value)).toEqual([2, 4, 6, 8, 10]);
  });

  test('bulk organization reads tags before additive updates and reports partial failures', async () => {
    const requests = [];
    const fetcher = jest.fn(async (url, options = {}) => {
      requests.push({ url, options });
      if (url.includes('camera-b/location')) throw new Error('location unavailable');
      if (url.endsWith('/tags') && !options.method) return { tags: [{ uuid: 'existing' }] };
      return { success: true };
    });
    const cameras = [
      { camera_uuid: 'camera-a', name: 'A' },
      { camera_uuid: 'camera-b', name: 'B' },
    ];

    const result = await applyBulkOrganization(cameras, {
      locationUuid: 'location-a',
      tagOperation: 'add',
      tagUuids: ['new'],
    }, fetcher);

    expect(result.succeeded.map((camera) => camera.name)).toEqual(['A']);
    expect(result.failed).toHaveLength(1);
    const tagPut = requests.find((request) => request.url.includes('camera-a/tags') && request.options.method === 'PUT');
    expect(JSON.parse(tagPut.options.body)).toEqual({ tag_uuids: ['existing', 'new'] });
  });
});
