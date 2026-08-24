import {
  adjacentInvestigationResultIndex,
  advanceInvestigationCursor,
  findSegmentAt,
  nextAvailableTimestamp,
  segmentTrackPosition,
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
