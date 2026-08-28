import {
  buildLiveLayoutPayload,
  buildLocationTree,
  buildRecentEventsRequests,
  cameraUuidsForLiveLayout,
  cameraUuidsForLocation,
  cameraUuidScopeKey,
  filterLiveNavigatorInventory,
  filterLiveOperatorStreams,
  flattenLocationTree,
  clusterFloorPlanCameras,
  floorPlanPayload,
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

  test('keeps disabled cameras in Navigator inventory but removes deleted cameras', () => {
    const inventory = filterLiveNavigatorInventory([
      ...streams,
      { camera_uuid: 'camera-disabled', availability: 'disabled' },
      { camera_uuid: 'camera-deleted', availability: 'offline', is_deleted: true },
    ]);
    expect(inventory.map((stream) => stream.camera_uuid)).toEqual([
      'camera-a', 'camera-b', 'camera-disabled',
    ]);
    expect(filterLiveNavigatorInventory(inventory, new Set(['camera-b'])))
      .toEqual([streams[1]]);
  });

  test('uses the saved layout camera slots as an exact, stable scope', () => {
    expect(cameraUuidsForLiveLayout({ camera_slots: [
      { camera_uuid: 'camera-b' },
      { camera_uuid: 'camera-a' },
      { camera_uuid: 'camera-b' },
      {},
    ] })).toEqual(['camera-b', 'camera-a']);
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

  test('clusters dense floor-plan markers only at overview zoom', () => {
    const cameras = Array.from({ length: 30 }, (_, index) => ({
      camera_uuid: `camera-${index}`,
      x: 0.1 + (index % 3) * 0.005,
      y: 0.2 + (index % 2) * 0.005,
    }));
    const overview = clusterFloorPlanCameras(cameras, 1);
    expect(overview.some((marker) => marker.kind === 'cluster')).toBe(true);
    expect(overview.reduce((count, marker) =>
      count + (marker.kind === 'cluster' ? marker.count : 1), 0)).toBe(30);
    expect(clusterFloorPlanCameras(cameras, 2)).toHaveLength(30);
  });

  test('normalizes floor-plan updates without losing camera identity', () => {
    expect(floorPlanPayload({
      name: 'Building', canvas_width: 1200, canvas_height: 800, revision: 3,
    }, [{ camera_uuid: 'camera-a', x: -1, y: 2, rotation: 220, fov: 0 }]))
      .toMatchObject({
        name: 'Building', revision: 3,
        cameras: [{ camera_uuid: 'camera-a', x: 0, y: 1, rotation: 180, fov: 65 }],
      });
  });
});
