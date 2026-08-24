import {
  adjacentInvestigationResultIndex,
  advanceInvestigationCursor,
  findSegmentAt,
  nextAvailableTimestamp,
  narrowThumbnailWindow,
  normalizedRegionRectangle,
  parseInvestigationRegion,
  segmentTrackPosition,
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
