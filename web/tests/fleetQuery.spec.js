import {
  DEFAULT_FLEET_STATE,
  buildFleetQueryRequest,
  buildFleetSavedViewPayload,
  buildFleetSelector,
  clampFleetPage,
  countFleetFilters,
  fleetStateFromOperationalQueue,
  fleetStateFromSavedView,
  readFleetUrlState,
  toggleFleetValue,
  writeFleetUrlState,
} from '../js/components/preact/fleet/fleetQuery.js';

describe('fleet query state', () => {
  test('builds an all selector for an unfiltered fleet', () => {
    expect(buildFleetSelector(DEFAULT_FLEET_STATE)).toEqual({
      version: 1,
      expression: { op: 'all' },
    });
  });

  test('composes fleet filters into a versioned selector', () => {
    const state = {
      ...DEFAULT_FLEET_STATE,
      locationUuid: 'location-1',
      tagUuids: ['tag-1', 'tag-2'],
      health: ['down', 'degraded'],
      enabled: 'true',
      recordingModes: ['continuous'],
    };

    expect(buildFleetSelector(state)).toEqual({
      version: 1,
      expression: {
        op: 'and',
        children: [
          { op: 'location_subtree', uuid: 'location-1' },
          { op: 'tag_any', uuids: ['tag-1', 'tag-2'] },
          { op: 'health', values: ['down', 'degraded'] },
          { op: 'enabled', value: true },
          { op: 'recording_mode', values: ['continuous'] },
        ],
      },
    });
  });

  test('round trips durable view state through the URL', () => {
    const state = {
      ...DEFAULT_FLEET_STATE,
      search: 'north door',
      health: ['down'],
      enabled: 'false',
      availability: 'offline',
      recordingModes: ['off', 'detection'],
      tagUuids: ['tag-a'],
      locationUuid: 'location-a',
      collectionUuid: 'collection-a',
      page: 3,
      pageSize: 100,
      sortBy: 'health',
      sortOrder: 'desc',
    };
    const url = writeFleetUrlState('http://localhost/fleet.html?unrelated=kept', state);

    expect(url.searchParams.get('unrelated')).toBe('kept');
    expect(readFleetUrlState(url.search)).toEqual(state);
  });

  test('normalizes invalid URL values to safe API defaults', () => {
    expect(readFleetUrlState('?page=-4&size=999&sort=nope&order=sideways&health=down,nope,down')).toEqual({
      ...DEFAULT_FLEET_STATE,
      health: ['down'],
    });
  });

  test('builds the paginated API request with debounced search', () => {
    expect(buildFleetQueryRequest({ ...DEFAULT_FLEET_STATE, page: 2, collectionUuid: 'collection-a' }, ' lobby ')).toMatchObject({
      search: 'lobby',
      collection_uuid: 'collection-a',
      page: 2,
      page_size: 50,
      sort_by: 'name',
      sort_order: 'asc',
      facets: true,
      availability: 'all',
    });
  });

  test('toggles values, counts filters, and clamps stale pages', () => {
    expect(toggleFleetValue(['up'], 'down')).toEqual(['up', 'down']);
    expect(toggleFleetValue(['up', 'down'], 'up')).toEqual(['down']);
    expect(countFleetFilters({
      ...DEFAULT_FLEET_STATE,
      health: ['down'],
      recordingModes: ['off'],
      tagUuids: ['tag-a'],
      enabled: 'true',
      availability: 'never_connected',
      locationUuid: 'location-a',
      collectionUuid: 'collection-a',
    })).toBe(7);
    expect(clampFleetPage(8, 3)).toBe(3);
    expect(clampFleetPage(8, 0)).toBe(1);
  });

  test('round trips supported filter state through a saved view payload', () => {
    const state = {
      ...DEFAULT_FLEET_STATE,
      search: ' north ',
      health: ['down'],
      enabled: 'true',
      tagUuids: ['tag-a'],
      pageSize: 100,
      sortBy: 'health',
      sortOrder: 'desc',
    };
    const payload = buildFleetSavedViewPayload(' Offline ', true, state);
    expect(payload.name).toBe('Offline');
    expect(payload.is_shared).toBe(true);
    expect(payload.search).toBe('north');
    expect(fleetStateFromSavedView({
      ...payload,
      collection_uuid: payload.collection_uuid,
    }, state)).toMatchObject({
      search: 'north',
      health: ['down'],
      enabled: 'true',
      tagUuids: ['tag-a'],
      page: 1,
      pageSize: 100,
      sortBy: 'health',
      sortOrder: 'desc',
    });
  });

  test('applies operational queues and rejects selectors the filter UI cannot represent', () => {
    const queueState = fleetStateFromOperationalQueue({
      selector: { version: 1, expression: { op: 'health', values: ['down'] } },
    }, { ...DEFAULT_FLEET_STATE, pageSize: 200, search: 'stale' });
    expect(queueState).toMatchObject({ health: ['down'], search: '', pageSize: 200 });
    expect(fleetStateFromSavedView({
      selector: { version: 1, expression: { op: 'or', children: [] } },
    })).toBeNull();
  });
});
