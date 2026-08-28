import {
  buildLiveLayoutPayload,
  buildLocationTree,
  buildRecentEventsRequests,
  cameraUuidsForLocation,
  cameraUuidScopeKey,
  filterLiveOperatorStreams,
  flattenLocationTree,
} from '../js/components/preact/live/liveOperator.js';

const locations = [
  { uuid: 'campus', parent_uuid: null, name: 'Campus', type: 'site', sort_order: 0 },
  { uuid: 'building', parent_uuid: 'campus', name: 'New Building', type: 'building', sort_order: 0 },
  { uuid: 'floor', parent_uuid: 'building', name: 'Floor 2', type: 'floor', sort_order: 0 },
];
const streams = [
  { camera_uuid: 'camera-a', name: 'Lobby', location_uuid: 'building', availability: 'live' },
  { camera_uuid: 'camera-b', name: 'Hall', location_uuid: 'floor', availability: 'offline' },
];

describe('Live operator navigator model', () => {
  test('uses camera membership rather than Set identity for effect dependencies', () => {
    expect(cameraUuidScopeKey(new Set(['camera-b', 'camera-a']))).toBe(
      cameraUuidScopeKey(new Set(['camera-a', 'camera-b'])));
    expect(cameraUuidScopeKey(new Set(['camera-a', 'camera-c']))).not.toBe(
      cameraUuidScopeKey(new Set(['camera-a', 'camera-b'])));
  });

  test('aggregates nested location counts and scopes descendant cameras', () => {
    const tree = buildLocationTree(locations, streams);
    expect(tree[0]).toMatchObject({ uuid: 'campus', total: 2, live: 1 });
    expect(flattenLocationTree(tree).map((row) => row.uuid)).toEqual([
      'campus', 'building', 'floor',
    ]);
    expect([...cameraUuidsForLocation(locations, streams, 'building')]).toEqual([
      'camera-a', 'camera-b',
    ]);
  });

  test('defaults layouts and stream filters to cameras live now', () => {
    const scoped = cameraUuidsForLocation(locations, streams, 'campus');
    expect(filterLiveOperatorStreams(streams, scoped, 'live')).toEqual([streams[0]]);
    expect(buildLiveLayoutPayload({
      name: ' Desk ', columns: 2, rows: 1, streams,
    })).toEqual({
      name: 'Desk',
      is_shared: false,
      location_uuid: null,
      availability: 'live',
      columns: 2,
      rows: 1,
      camera_slots: [
        { camera_uuid: 'camera-a' },
        { camera_uuid: 'camera-b' },
      ],
    });
  });

  test('chunks recent event requests at the server selector limit', () => {
    const requests = buildRecentEventsRequests(
      Array.from({ length: 65 }, (_, index) => `camera-${index}`), 10_000,
    );
    expect(requests).toHaveLength(2);
    expect(requests[0]).toMatchObject({
      selector: { version: 1, expression: { op: 'camera_uuid' } },
      start_time: 6_400,
      end_time: 10_000,
      limit: 20,
      include_summary: false,
    });
    expect(requests[0].selector.expression.values).toHaveLength(64);
    expect(requests[1].selector.expression.values).toHaveLength(1);
  });
});
