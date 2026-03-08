import {
  countSegmentsForDate,
  findFirstVisibleSegmentIndex,
  formatTimestampAsLocalDate,
  findContainingSegmentIndex,
  findNearestSegmentIndex,
  formatTimestampAsClock,
  getAvailableDatesForSegments,
  getClippedSegmentHourRange,
  getLocalDayBounds,
  segmentIntersectsDay
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

  test('formats timestamps as local YYYY-MM-DD keys', () => {
    expect(formatTimestampAsLocalDate(new Date(2026, 2, 8, 12, 10, 0).getTime() / 1000)).toBe('2026-03-08');
  });

  test('returns all local dates touched by selected recordings, including overnight spans', () => {
    const mar8 = getLocalDayBounds('2026-03-08');
    const mar9 = getLocalDayBounds('2026-03-09');

    const dates = getAvailableDatesForSegments([
      {
        id: 1,
        start_timestamp: mar8.startTimestamp + 3600,
        end_timestamp: mar8.startTimestamp + 7200
      },
      {
        id: 2,
        start_timestamp: mar8.endTimestamp - 300,
        end_timestamp: mar9.startTimestamp + 300
      }
    ]);

    expect(dates).toEqual(['2026-03-08', '2026-03-09']);
  });

  test('does not add the next day when a recording ends exactly at midnight', () => {
    const mar8 = getLocalDayBounds('2026-03-08');

    expect(getAvailableDatesForSegments([
      {
        id: 1,
        start_timestamp: mar8.endTimestamp - 600,
        end_timestamp: mar8.endTimestamp
      }
    ])).toEqual(['2026-03-08']);
  });

  test('counts and locates segments visible on a selected day', () => {
    const mar8 = getLocalDayBounds('2026-03-08');
    const mar9 = getLocalDayBounds('2026-03-09');
    const segments = [
      {
        id: 1,
        start_timestamp: mar8.startTimestamp + 120,
        end_timestamp: mar8.startTimestamp + 600
      },
      {
        id: 2,
        start_timestamp: mar8.endTimestamp - 300,
        end_timestamp: mar9.startTimestamp + 120
      },
      {
        id: 3,
        start_timestamp: mar9.startTimestamp + 600,
        end_timestamp: mar9.startTimestamp + 1200
      }
    ];

    expect(segmentIntersectsDay(segments[1], '2026-03-08')).toBe(true);
    expect(segmentIntersectsDay(segments[1], '2026-03-09')).toBe(true);
    expect(countSegmentsForDate(segments, '2026-03-08')).toBe(2);
    expect(countSegmentsForDate(segments, '2026-03-09')).toBe(2);
    expect(findFirstVisibleSegmentIndex(segments, '2026-03-09')).toBe(1);
  });
});

