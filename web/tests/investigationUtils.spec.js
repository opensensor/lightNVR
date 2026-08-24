import {
  adjacentInvestigationResultIndex,
  advanceInvestigationCursor,
  bookmarkFiltersFromState,
  buildInvestigationBookmarkPayload,
  findSegmentAt,
  formatDateTimeLocal,
  nextAvailableTimestamp,
  narrowThumbnailWindow,
  normalizedRegionRectangle,
  investigationBookmarkUrl,
  investigationActionSelection,
  parseDateTimeLocal,
  parseInvestigationRegion,
  segmentTrackPosition,
  summarizeInvestigationActionPreview,
  thumbnailWindowForResult,
  videoContentBox,
} from '../js/components/preact/investigation/investigationUtils.js';

const tracks = [
  {
    camera_uuid: 'a',
    segments: [
      { id: 1, start_time: 100, end_time: 120 },
      { id: 2, start_time: 200, end_time: 240 },
    ],
  },
  {
    camera_uuid: 'b',
    segments: [{ id: 3, start_time: 150, end_time: 180 }],
  },
];

test('findSegmentAt includes recording boundaries', () => {
  expect(findSegmentAt(tracks[0].segments, 100)?.id).toBe(1);
  expect(findSegmentAt(tracks[0].segments, 120)?.id).toBe(1);
  expect(findSegmentAt(tracks[0].segments, 121)).toBeNull();
});

test('nextAvailableTimestamp uses the earliest active camera boundary', () => {
  expect(nextAvailableTimestamp(tracks, ['a', 'b'], 121)).toBe(150);
  expect(nextAvailableTimestamp(tracks, ['a'], 121)).toBe(200);
  expect(nextAvailableTimestamp(tracks, ['a'], 250)).toBeNull();
});

test('skip common gaps advances only when every active camera lacks footage', () => {
  expect(advanceInvestigationCursor({
    cursor: 120,
    elapsedSeconds: 1,
    speed: 1,
    endTime: 300,
    mode: 'skip-common-gaps',
    tracks,
    activeCameraUuids: ['a', 'b'],
  })).toBe(150);
  expect(advanceInvestigationCursor({
    cursor: 155,
    elapsedSeconds: 1,
    speed: 2,
    endTime: 300,
    mode: 'skip-common-gaps',
    tracks,
    activeCameraUuids: ['a', 'b'],
  })).toBe(157);
});

test('segmentTrackPosition clips segments to the visible UTC window', () => {
  expect(segmentTrackPosition(
    { start_time: 90, end_time: 120 }, 100, 200,
  )).toEqual({ left: '0%', width: '20%' });
});

test('adjacent result navigation is stable by opaque result id', () => {
  const results = [
    { result_id: 'detection:9' },
    { result_id: 'detection:7' },
    { result_id: 'detection:4' },
  ];
  expect(adjacentInvestigationResultIndex(results, null, 1)).toBe(0);
  expect(adjacentInvestigationResultIndex(results, null, -1)).toBe(2);
  expect(adjacentInvestigationResultIndex(results, 'detection:7', 1)).toBe(2);
  expect(adjacentInvestigationResultIndex(results, 'detection:7', -1)).toBe(0);
  expect(adjacentInvestigationResultIndex(results, 'detection:4', 1)).toBe(-1);
  expect(adjacentInvestigationResultIndex([], null, 1)).toBe(-1);
});

test('thumbnail selection narrows to midpoints around the selected sample', () => {
  const samples = [100, 120, 140, 160, 180]
    .map((timestamp) => ({ timestamp }));
  expect(narrowThumbnailWindow(samples, 2, 100, 180)).toEqual({
    startTime: 130,
    endTime: 150,
  });
  expect(narrowThumbnailWindow(samples, 0, 100, 180)).toEqual({
    startTime: 100,
    endTime: 110,
  });
  expect(narrowThumbnailWindow(
    [{ timestamp: 100 }, { timestamp: 101 }], 0, 100, 101,
  )).toBeNull();
});

test('selected event thumbnail window is padded and clipped to the timeline', () => {
  expect(thumbnailWindowForResult(
    { start_time: 110, end_time: 115 }, 100, 200,
  )).toEqual({ startTime: 100, endTime: 145 });
  expect(thumbnailWindowForResult(
    { start_time: 150, end_time: 190 }, 100, 200,
  )).toEqual({ startTime: 110, endTime: 200 });
});

test('videoContentBox accounts for letterboxing and pillarboxing', () => {
  expect(videoContentBox(16, 9, 4, 3)).toEqual({
    left: 0.125, top: 0, width: 0.75, height: 1,
  });
  const letterboxed = videoContentBox(16, 9, 21, 9);
  expect(letterboxed.left).toBe(0);
  expect(letterboxed.width).toBe(1);
  expect(letterboxed.top).toBeCloseTo(0.119047619);
  expect(letterboxed.height).toBeCloseTo(0.761904762);
});

test('normalizedRegionRectangle supports reverse drag and clamps to the image', () => {
  expect(normalizedRegionRectangle(
    { x: 0.8, y: 0.7 }, { x: -0.1, y: 1.2 },
  )).toEqual({ x: 0, y: 0.7, width: 0.8, height: 0.3 });
});

test('parseInvestigationRegion accepts only bounded navigation-safe state', () => {
  const valid = new URLSearchParams({
    region_camera: '11111111-1111-1111-1111-111111111111',
    region_rect: '0.1,0.2,0.3,0.4',
    region_match: 'minimum_intersection',
    region_min: '0.2',
  });
  expect(parseInvestigationRegion(valid)).toEqual({
    cameraUuid: '11111111-1111-1111-1111-111111111111',
    x: 0.1,
    y: 0.2,
    width: 0.3,
    height: 0.4,
    match: 'minimum_intersection',
    minIntersection: 0.2,
  });
  valid.set('region_rect', '0.9,0.2,0.3,0.4');
  expect(parseInvestigationRegion(valid)).toBeNull();
});

test('bookmark payload keeps navigation state but excludes arbitrary result fields', () => {
  const payload = buildInvestigationBookmarkPayload({
    title: '  Shift handoff  ',
    note: ' private note ',
    timeline: {
      start_time: 100.8,
      end_time: 300.9,
      tracks: [{ camera_uuid: 'camera-a' }, { camera_uuid: 'camera-b' }],
    },
    cursor: 155.9,
    primaryCameraUuid: 'camera-b',
    searchFilters: {
      eventType: 'detection',
      label: 'person',
      minConfidence: '0.75',
      region: {
        cameraUuid: 'camera-b',
        x: 0.1,
        y: 0.2,
        width: 0.3,
        height: 0.4,
        match: 'intersects',
        minIntersection: 0.25,
      },
    },
    selectedResult: {
      result_id: 'detection:9',
      camera_uuid: 'camera-b',
      label: 'person',
      thumbnail_url: '/secret-ish/path',
    },
  });
  expect(payload).toEqual({
    title: 'Shift handoff',
    note: 'private note',
    camera_uuids: ['camera-a', 'camera-b'],
    start_time: 100,
    end_time: 300,
    cursor_time: 155,
    primary_camera_uuid: 'camera-b',
    filters: {
      event_type: 'detection',
      label: 'person',
      min_confidence: 0.75,
      region: {
        camera_uuid: 'camera-b',
        x: 0.1,
        y: 0.2,
        width: 0.3,
        height: 0.4,
        match: 'intersects',
        min_intersection: 0.25,
      },
    },
    representative_result: {
      result_id: 'detection:9',
      camera_uuid: 'camera-b',
      label: 'person',
    },
  });
});

test('investigation actions default to the exact loaded window and cameras', () => {
  expect(investigationActionSelection({
    start_time: 100.8,
    end_time: 300.2,
    tracks: [{ camera_uuid: 'a' }, { camera_uuid: 'b' }],
  }, null, 'investigation')).toEqual({
    cameraUuids: ['a', 'b'],
    startTime: 100,
    endTime: 301,
  });
});

test('selected-result actions use one camera and a non-empty bounded interval', () => {
  const timeline = {
    start_time: 100,
    end_time: 300,
    tracks: [{ camera_uuid: 'a' }, { camera_uuid: 'b' }],
  };
  expect(investigationActionSelection(timeline, {
    camera_uuid: 'b', start_time: 150, end_time: 150,
  }, 'result')).toEqual({
    cameraUuids: ['b'], startTime: 150, endTime: 151,
  });
});

test('action datetime values retain seconds for one-second result windows', () => {
  expect(formatDateTimeLocal(150, true)).toMatch(/:30$/);
  expect(parseDateTimeLocal(formatDateTimeLocal(150, true))).toBe(150);
});

test('action preview summary separates partial protection from all-or-nothing export', () => {
  expect(summarizeInvestigationActionPreview({ recordings: [
    { id: 1, protected: false, can_protect: true, can_export: true },
    { id: 2, protected: false, can_protect: false, can_export: false },
    { id: 3, protected: true, can_protect: true, can_export: true },
  ] })).toEqual({
    recordingCount: 3,
    unprotectedIds: [1, 2],
    protectableCount: 1,
    protectDeniedCount: 1,
    exportDeniedCount: 1,
    canProtect: true,
    canExport: false,
  });
});

test('bookmark filters ignore invalid confidence instead of persisting it', () => {
  expect(bookmarkFiltersFromState({
    source: 'local',
    minConfidence: 'not-a-number',
  })).toEqual({ source: 'local' });
});

test('bookmark URL contains only navigation-safe state and restores primary camera', () => {
  const url = new URL(investigationBookmarkUrl({
    camera_uuids: ['one', 'two'],
    start_time: 100,
    end_time: 200,
    cursor_time: 150,
    primary_camera_uuid: 'two',
    note: 'must never enter the URL',
    filters: {
      label: 'person',
      protection: 'protected',
      region: {
        camera_uuid: 'two',
        x: 0.1,
        y: 0.2,
        width: 0.3,
        height: 0.4,
        match: 'center',
        min_intersection: 0.25,
      },
    },
  }, 'https://nvr.test/investigation.html?drill_camera=old&event=motion'));
  expect(url.searchParams.get('cameras')).toBe('one,two');
  expect(url.searchParams.get('primary')).toBe('two');
  expect(url.searchParams.get('label')).toBe('person');
  expect(url.searchParams.get('protected')).toBe('protected');
  expect(url.searchParams.get('region_rect')).toBe('0.1,0.2,0.3,0.4');
  expect(url.searchParams.has('drill_camera')).toBe(false);
  expect(url.search).not.toContain('must+never');
});
