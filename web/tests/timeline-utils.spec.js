import {
  findContainingSegmentIndex,
  findNearestSegmentIndex,
  formatTimestampAsClock,
  getClippedSegmentHourRange,
  getLocalDayBounds
} from '../js/components/preact/timeline/timelineUtils.js';

describe('timelineUtils', () => {
  test('prefers the later segment when recordings overlap at the requested time', () => {
    const segments = [
      { id: 1, start_timestamp: 100, end_timestamp: 200 },
      { id: 2, start_timestamp: 190, end_timestamp: 300 }
    ];

    expect(findContainingSegmentIndex(segments, 190)).toBe(1);
    expect(findContainingSegmentIndex(segments, 200)).toBe(1);
  });

  test('prefers the next segment when recordings touch exactly at a boundary', () => {
    const segments = [
      { id: 1, start_timestamp: 100, end_timestamp: 200 },
      { id: 2, start_timestamp: 200, end_timestamp: 260 }
    ];

    expect(findContainingSegmentIndex(segments, 200)).toBe(1);
  });

  test('returns -1 when no segment contains the timestamp', () => {
    const segments = [
      { id: 1, start_timestamp: 100, end_timestamp: 150 },
      { id: 2, start_timestamp: 200, end_timestamp: 250 }
    ];

    expect(findContainingSegmentIndex(segments, 175)).toBe(-1);
  });

  test('finds the nearest segment when the timestamp falls in a gap', () => {
    const segments = [
      { id: 1, start_timestamp: 100, end_timestamp: 150 },
      { id: 2, start_timestamp: 200, end_timestamp: 250 }
    ];

    expect(findNearestSegmentIndex(segments, 170)).toBe(0);
    expect(findNearestSegmentIndex(segments, 195)).toBe(1);
  });

  test('breaks equal nearest-distance ties toward the earlier segment', () => {
    const segments = [
      { id: 1, start_timestamp: 100, end_timestamp: 150 },
      { id: 2, start_timestamp: 190, end_timestamp: 240 }
    ];

    expect(findNearestSegmentIndex(segments, 170)).toBe(0);
  });

  test('clips a boundary-spanning segment to the selected local day', () => {
    const selectedDate = '2026-03-08';
    const { startTimestamp } = getLocalDayBounds(selectedDate);
    const range = getClippedSegmentHourRange(
      {
        start_timestamp: startTimestamp - 120,
        end_timestamp: startTimestamp + 300
      },
      selectedDate
    );

    expect(range).toEqual({
      startHour: 0,
      endHour: 300 / 3600
    });
  });

  test('clips a segment that continues into the next local day', () => {
    const selectedDate = '2026-03-08';
    const { startTimestamp, endTimestamp } = getLocalDayBounds(selectedDate);
    const range = getClippedSegmentHourRange(
      {
        start_timestamp: endTimestamp - 600,
        end_timestamp: endTimestamp + 180
      },
      selectedDate
    );

    expect(range).toEqual({
      startHour: (endTimestamp - 600 - startTimestamp) / 3600,
      endHour: (endTimestamp - startTimestamp) / 3600
    });
  });

  test('returns null when a segment is fully outside the selected day', () => {
    const selectedDate = '2026-03-08';
    const { startTimestamp, endTimestamp } = getLocalDayBounds(selectedDate);

    expect(getClippedSegmentHourRange({
      start_timestamp: startTimestamp - 600,
      end_timestamp: startTimestamp - 1
    }, selectedDate)).toBeNull();

    expect(getClippedSegmentHourRange({
      start_timestamp: endTimestamp,
      end_timestamp: endTimestamp + 600
    }, selectedDate)).toBeNull();
  });

  test('formats exact minute boundaries without floating point drift', () => {
    expect(formatTimestampAsClock(new Date(2026, 2, 8, 12, 10, 0).getTime() / 1000)).toBe('12:10:00');
    expect(formatTimestampAsClock(new Date(2026, 2, 8, 0, 2, 0).getTime() / 1000)).toBe('00:02:00');
  });
});

