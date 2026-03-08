import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';
import timezone from 'dayjs/plugin/timezone';

import {
  countSegmentsForDate,
  findFirstVisibleSegmentIndex,
  formatTimestampAsLocalDate,
  findContainingSegmentIndex,
  findNearestSegmentIndex,
  formatPlaybackTimeLabel,
  formatTimestampAsClock,
  formatTimelineOffsetLabel,
  getAvailableDatesForSegments,
  getClippedSegmentHourRange,
  getLocalDayBounds,
  getTimelineDayLengthHours,
  localClockTimeToTimestamp,
  normalizeTimelineRange,
  panTimelineRange,
  timelineOffsetToTimestamp,
  timestampToTimelineOffset,
  zoomTimelineRange,
  segmentIntersectsDay
} from '../js/components/preact/timeline/timelineUtils.js';

dayjs.extend(utc);
dayjs.extend(timezone);

describe('timelineUtils', () => {
  const originalTz = process.env.TZ;

  afterEach(() => {
    if (originalTz === undefined) {
      delete process.env.TZ;
    } else {
      process.env.TZ = originalTz;
    }
  });

  test('dayjs timezone plugin is configured for DST-sensitive calculations', () => {
    // This instant is the same moment in time regardless of DST;
    // we verify that dayjs.tz is available and returns a valid date.
    const instant = '2020-03-08T07:30:00Z';
    const nyTime = dayjs.tz(instant, 'America/New_York');

    // 2020-03-08 is the US DST start date; at 02:00 local time clocks
    // jump to 03:00. The converted time should be shortly after 03:00.
    expect(nyTime.isValid()).toBe(true);
    expect(nyTime.format('YYYY-MM-DD')).toBe('2020-03-08');
  });

  test('prefers the segment with later start time when recordings overlap', () => {
    const segments = [
      { id: 1, start_timestamp: 100, end_timestamp: 200 },
      { id: 2, start_timestamp: 190, end_timestamp: 300 }
    ];

    expect(findContainingSegmentIndex(segments, 190)).toBe(1);
    expect(findContainingSegmentIndex(segments, 195)).toBe(1);
    expect(findContainingSegmentIndex(segments, 200)).toBe(1);
  });

  test('returns the later segment when timestamp matches boundary between segments', () => {
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
    expect(findNearestSegmentIndex(segments, 175)).toBe(0);
  });

  test('returns segment with earlier start time when multiple segments are equidistant from timestamp', () => {
    const segments = [
      { id: 1, start_timestamp: 100, end_timestamp: 150 },
      { id: 2, start_timestamp: 190, end_timestamp: 240 }
    ];

    const timestamp = 170;
    const distanceToFirst =
      Math.min(
        Math.abs(timestamp - segments[0].start_timestamp),
        Math.abs(timestamp - segments[0].end_timestamp)
      );
    const distanceToSecond =
      Math.min(
        Math.abs(timestamp - segments[1].start_timestamp),
        Math.abs(timestamp - segments[1].end_timestamp)
      );

    expect(distanceToFirst).toBe(distanceToSecond);
    expect(findNearestSegmentIndex(segments, timestamp)).toBe(0);
  });

  test('clips a boundary-spanning segment to the selected local day', () => {
    const selectedDate = '2026-03-08';
    const { startTimestamp } = getLocalDayBounds(selectedDate);
    const SECONDS_BEFORE_DAY_START = 120; // 2 minutes before day start
    const SECONDS_AFTER_DAY_START = 300; // 5 minutes into the day
    const range = getClippedSegmentHourRange(
      {
        start_timestamp: startTimestamp - SECONDS_BEFORE_DAY_START,
        end_timestamp: startTimestamp + SECONDS_AFTER_DAY_START
      },
      selectedDate
    );

    expect(range).toEqual({
      startHour: 0,
      endHour: SECONDS_AFTER_DAY_START / 3600
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
    const exactTwelveTen = new Date(2026, 2, 8, 12, 10, 0).getTime() / 1000;
    const exactTwelveTenPlusNearlyASecond = exactTwelveTen + 0.999;
    const exactMidnightTwo = new Date(2026, 2, 8, 0, 2, 0).getTime() / 1000;
    const exactMidnightTwoPlusSmallFraction = exactMidnightTwo + 0.001;

    // Exact minute boundaries should format correctly.
    expect(formatTimestampAsClock(exactTwelveTen)).toBe('12:10:00');
    expect(formatTimestampAsClock(exactMidnightTwo)).toBe('00:02:00');

    // Fractional seconds just below the next whole second/minute should not cause rounding drift.
    expect(formatTimestampAsClock(exactTwelveTenPlusNearlyASecond)).toBe('12:10:00');
    expect(formatTimestampAsClock(exactMidnightTwoPlusSmallFraction)).toBe('00:02:00');
  });

  test('formats playback labels with a stream name prefix when available', () => {
    const timestamp = new Date(2026, 2, 8, 9, 15, 0).getTime() / 1000;

    expect(formatPlaybackTimeLabel(timestamp, 'front_door')).toBe('front_door - 09:15:00');
    expect(formatPlaybackTimeLabel(timestamp, '')).toBe('09:15:00');
  });

  test('returns only the stream name when playback time is unavailable', () => {
    expect(formatPlaybackTimeLabel(null, 'garage')).toBe('garage');
  });

  test('uses the actual local day length on a DST spring-forward day', () => {
    process.env.TZ = 'America/New_York';

    const bounds = getLocalDayBounds('2026-03-08');

    expect(bounds.endTimestamp - bounds.startTimestamp).toBe(23 * 3600);
    expect(bounds.durationHours).toBe(23);
    expect(getTimelineDayLengthHours('2026-03-08')).toBe(23);

    // Verify that dayjs interprets the boundary timestamps using the configured timezone
    // by checking the local wall-clock times at the start and end of the day.
    const startLocal = dayjs.unix(bounds.startTimestamp);
    const endLocal = dayjs.unix(bounds.endTimestamp);

    // The selected local day should start at midnight on 2026-03-08 in America/New_York.
    expect(startLocal.year()).toBe(2026);
    expect(startLocal.month()).toBe(2); // March (0-based month index)
    expect(startLocal.date()).toBe(8);
    expect(startLocal.hour()).toBe(0);
    expect(startLocal.minute()).toBe(0);
    expect(startLocal.second()).toBe(0);

    // The end timestamp should represent the end of that shortened 23-hour local day.
    expect(endLocal.unix() - startLocal.unix()).toBe(23 * 3600);
  });

  test('maps DST spring-forward timestamps to elapsed timeline offsets', () => {
    process.env.TZ = 'America/New_York';

    const selectedDate = '2026-03-08';
    const timestamp = dayjs(`${selectedDate}T03:10:00`).unix();

    expect(timestampToTimelineOffset(timestamp, selectedDate)).toBeCloseTo(2 + (10 / 60), 6);
    expect(timelineOffsetToTimestamp(2 + (10 / 60), selectedDate)).toBe(timestamp);
    expect(formatTimelineOffsetLabel(2, selectedDate)).toBe('3:00');

    // Confirm that dayjs sees this instant as the expected local wall-clock time.
    const local = dayjs.unix(timestamp);
    expect(local.year()).toBe(2026);
    expect(local.month()).toBe(2); // March
    expect(local.date()).toBe(8);
    expect(local.hour()).toBe(3);
    expect(local.minute()).toBe(10);
  });

  test('parses wall-clock query times on DST days without treating them as elapsed offsets', () => {
    process.env.TZ = 'America/New_York';

    const selectedDate = '2026-03-08';
    const timestamp = localClockTimeToTimestamp('03:10:00', selectedDate);

    expect(timestamp).toBe(dayjs(`${selectedDate}T03:10:00`).unix());
    expect(timestampToTimelineOffset(timestamp, selectedDate)).toBeCloseTo(2 + (10 / 60), 6);

    // Again, assert that dayjs interprets this timestamp as 03:10:00 local time on the selected day.
    const parsedLocal = dayjs.unix(timestamp);
    expect(parsedLocal.year()).toBe(2026);
    expect(parsedLocal.month()).toBe(2); // March
    expect(parsedLocal.date()).toBe(8);
    expect(parsedLocal.hour()).toBe(3);
    expect(parsedLocal.minute()).toBe(10);
  });

  test('uses the actual local day length on a DST fall-back day', () => {
    process.env.TZ = 'America/New_York';

    const bounds = getLocalDayBounds('2026-11-01');

    expect(bounds.endTimestamp - bounds.startTimestamp).toBe(25 * 3600);
    expect(bounds.durationHours).toBe(25);
    expect(getTimelineDayLengthHours('2026-11-01')).toBe(25);
  });

  test('shows the repeated local hour twice on a DST fall-back day', () => {
    process.env.TZ = 'America/New_York';

    const selectedDate = '2026-11-01';
    const firstRepeatedHourTimestamp = Math.floor(Date.parse('2026-11-01T05:10:00Z') / 1000);
    const secondRepeatedHourTimestamp = Math.floor(Date.parse('2026-11-01T06:10:00Z') / 1000);

    expect(timestampToTimelineOffset(firstRepeatedHourTimestamp, selectedDate)).toBeCloseTo(1 + (10 / 60), 6);
    expect(timestampToTimelineOffset(secondRepeatedHourTimestamp, selectedDate)).toBeCloseTo(2 + (10 / 60), 6);
    expect(formatTimelineOffsetLabel(1, selectedDate)).toBe('1:00');
    expect(formatTimelineOffsetLabel(2, selectedDate)).toBe('1:00');
    expect(formatTimelineOffsetLabel(3, selectedDate)).toBe('2:00');
  });

  test('handles clock boundary cases at midnight, end of day, and subsecond precision', () => {
    // Midnight at the start of the day
    expect(formatTimestampAsClock(new Date(2026, 2, 8, 0, 0, 0).getTime() / 1000)).toBe('00:00:00');

    // Last second of the day
    expect(formatTimestampAsClock(new Date(2026, 2, 8, 23, 59, 59).getTime() / 1000)).toBe('23:59:59');

    // Subsecond precision: ensure fractional seconds do not cause rounding drift
    const nearlyNextSecond = new Date(2026, 2, 8, 12, 10, 30).getTime() / 1000 + 0.999;
    expect(formatTimestampAsClock(nearlyNextSecond)).toBe('12:10:30');

    // Subsecond precision near zero: ensure small positive fractions do not change the displayed second
    const justAfterSecond = new Date(2026, 2, 8, 12, 10, 30).getTime() / 1000 + 0.001;
    expect(formatTimestampAsClock(justAfterSecond)).toBe('12:10:30');
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

  test('does not add the previous day when a recording starts exactly at midnight', () => {
    const mar8 = getLocalDayBounds('2026-03-08');

    expect(getAvailableDatesForSegments([
      {
        id: 1,
        start_timestamp: mar8.startTimestamp,
        end_timestamp: mar8.startTimestamp + 600
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

  test('normalizes timeline ranges into the 0-24 hour window', () => {
    expect(normalizeTimelineRange(-2, 30)).toEqual({
      startHour: 0,
      endHour: 24
    });

    // normalizeTimelineRange enforces a minimum visible range (e.g. 0.5 hours)
    // and clamps the resulting window to the 0–24 hour bounds.
    expect(normalizeTimelineRange(23.9, 24.1)).toEqual({
      startHour: 23.5,
      endHour: 24
    });

    // When the requested range is even smaller than the minimum, it is expanded
    // to the minimum range while still clamping at the end of the day.
    expect(normalizeTimelineRange(23.95, 24.0)).toEqual({
      startHour: 23.5,
      endHour: 24
    });
  });

  test('pans the visible timeline range while clamping at the day bounds', () => {
    expect(panTimelineRange(6, 10, 2)).toEqual({
      startHour: 8,
      endHour: 12
    });

    expect(panTimelineRange(20, 24, 3)).toEqual({
      startHour: 20,
      endHour: 24
    });
  });

  test('zooms around an anchor hour while preserving that anchor when possible', () => {
    expect(zoomTimelineRange(4, 12, 0.5, 10)).toEqual({
      startHour: 7,
      endHour: 11
    });

    expect(zoomTimelineRange(4, 12, 2, 10)).toEqual({
      startHour: 0,
      endHour: 16
    });

    // Zooming near the start of the day: anchor at 0 should remain at 0,
    // and the visible range should be clamped to the [0, 24] window.
    expect(zoomTimelineRange(0, 4, 0.5, 0)).toEqual({
      startHour: 0,
      endHour: 2
    });

    // Zooming near the end of the day: anchor at 24 should remain at 24,
    // and the visible range should be clamped to the [0, 24] window.
    expect(zoomTimelineRange(20, 24, 0.5, 24)).toEqual({
      startHour: 22,
      endHour: 24
    });

    // Extreme zoom-out around a mid-range anchor should expand to the full day
    // and be clamped to the [0, 24] bounds.
    expect(zoomTimelineRange(8, 16, 10, 12)).toEqual({
      startHour: 0,
      endHour: 24
    });
  });
});

