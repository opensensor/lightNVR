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
  resolvePlaybackStreamName,
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

// JavaScript Date months are 0-indexed, so month index 2 represents March.
const MARCH = 2;
const SPRING_FORWARD_DATE = '2020-03-08';
const SPRING_FORWARD_NEXT_DATE = '2020-03-09';
const FALL_BACK_DATE = '2020-11-01';
const padClockPart = (value) => String(value).padStart(2, '0');
const baseTestDateTimestamp = (hours, minutes, seconds = 0) =>
  // Use UTC to avoid environment-dependent DST/local time interpretation in tests.
  Date.UTC(2020, MARCH, 8, hours, minutes, seconds) / 1000;
const baseLocalTestDateTimestamp = (hours, minutes, seconds = 0) =>
  dayjs(`${SPRING_FORWARD_DATE}T${padClockPart(hours)}:${padClockPart(minutes)}:${padClockPart(seconds)}`).unix();

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
    // Prerequisite verification: ensure that dayjs.tz is available and correctly
    // configured for the DST-sensitive timeline tests below, using the US
    // "spring forward" transition on 2020-03-08 in America/New_York.
    //
    // Before the transition: 2020-03-08T06:30:00Z = 01:30 local time, UTC-5 (EST).
    const preDstInstant = '2020-03-08T06:30:00Z';
    const preDstNyTime = dayjs.utc(preDstInstant).tz('America/New_York');

    expect(preDstNyTime.isValid()).toBe(true);
    expect(preDstNyTime.format('YYYY-MM-DD')).toBe('2020-03-08');
    expect(preDstNyTime.format('HH:mm')).toBe('01:30');
    expect(preDstNyTime.utcOffset()).toBe(-300); // minutes, UTC-5

    // After the transition: 2020-03-08T07:30:00Z = 03:30 local time, UTC-4 (EDT).
    const postDstInstant = '2020-03-08T07:30:00Z';
    const postDstNyTime = dayjs.utc(postDstInstant).tz('America/New_York');

    expect(postDstNyTime.isValid()).toBe(true);
    expect(postDstNyTime.format('YYYY-MM-DD')).toBe('2020-03-08');
    expect(postDstNyTime.format('HH:mm')).toBe('03:30');
    expect(postDstNyTime.utcOffset()).toBe(-240); // minutes, UTC-4
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
    const selectedDate = SPRING_FORWARD_DATE;
    const { startTimestamp } = getLocalDayBounds(selectedDate);
    const secondsBeforeDayStart = 120; // 2 minutes before day start
    const secondsAfterDayStart = 300; // 5 minutes into the day
    const range = getClippedSegmentHourRange(
      {
        start_timestamp: startTimestamp - secondsBeforeDayStart,
        end_timestamp: startTimestamp + secondsAfterDayStart
      },
      selectedDate
    );

    expect(range).toEqual({
      startHour: 0,
      endHour: secondsAfterDayStart / 3600
    });
  });

  test('clips a segment that continues into the next local day', () => {
    const selectedDate = SPRING_FORWARD_DATE;
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
    const selectedDate = SPRING_FORWARD_DATE;
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
    const exactTwelveTen = baseLocalTestDateTimestamp(12, 10, 0);
    const exactTwelveTenPlusNearlyASecond = exactTwelveTen + 0.999;
    const exactMidnightTwo = baseLocalTestDateTimestamp(0, 2, 0);
    const exactMidnightTwoPlusSmallFraction = exactMidnightTwo + 0.001;

    // Exact minute boundaries should format correctly.
    expect(formatTimestampAsClock(exactTwelveTen)).toBe('12:10:00');
    expect(formatTimestampAsClock(exactMidnightTwo)).toBe('00:02:00');

    // Fractional seconds just below the next whole second/minute should not cause rounding drift.
    expect(formatTimestampAsClock(exactTwelveTenPlusNearlyASecond)).toBe('12:10:00');
    expect(formatTimestampAsClock(exactMidnightTwoPlusSmallFraction)).toBe('00:02:00');
  });

  test('formats playback labels with a stream name prefix when available', () => {
    const timestamp = baseLocalTestDateTimestamp(9, 15, 0);

    expect(formatPlaybackTimeLabel(timestamp, 'front_door')).toBe('front_door - 09:15:00');
    expect(formatPlaybackTimeLabel(timestamp, '')).toBe('09:15:00');
  });

  test('returns only the stream name when playback time is unavailable', () => {
    expect(formatPlaybackTimeLabel(null, 'garage')).toBe('garage');
  });

  test('resolves playback stream names from the current segment when available', () => {
    const segments = [
      { id: 1, stream: 'front_door', start_timestamp: baseTestDateTimestamp(9, 0, 0), end_timestamp: baseTestDateTimestamp(9, 5, 0) },
      { id: 2, stream: 'garage', start_timestamp: baseTestDateTimestamp(9, 10, 0), end_timestamp: baseTestDateTimestamp(9, 15, 0) }
    ];

    expect(resolvePlaybackStreamName(segments, 1, baseTestDateTimestamp(9, 0, 30))).toBe('garage');
  });

  test('falls back to the segment containing the current timestamp when the active index is transiently invalid', () => {
    const segments = [
      { id: 1, stream: 'front_door', start_timestamp: baseTestDateTimestamp(9, 0, 0), end_timestamp: baseTestDateTimestamp(9, 5, 0) },
      { id: 2, stream: 'garage', start_timestamp: baseTestDateTimestamp(9, 10, 0), end_timestamp: baseTestDateTimestamp(9, 15, 0) }
    ];
    const timestampInContainingSegment = baseTestDateTimestamp(9, 10, 30);

    expect(resolvePlaybackStreamName(segments, -1, timestampInContainingSegment)).toBe('garage');
    expect(resolvePlaybackStreamName(segments, segments.length, timestampInContainingSegment)).toBe('garage');
    expect(resolvePlaybackStreamName(segments, segments.length + 1, timestampInContainingSegment)).toBe('garage');
  });

  test('falls back to the nearest segment stream when the current timestamp is in a gap', () => {
    const segments = [
      { id: 1, stream: 'front_door', start_timestamp: baseTestDateTimestamp(9, 0, 0), end_timestamp: baseTestDateTimestamp(9, 5, 0) },
      { id: 2, stream: 'garage', start_timestamp: baseTestDateTimestamp(9, 10, 0), end_timestamp: baseTestDateTimestamp(9, 15, 0) }
    ];
    const timestampInGap = baseTestDateTimestamp(9, 8, 0);
    const distanceToFrontDoor = timestampInGap - segments[0].end_timestamp;
    const distanceToGarage = segments[1].start_timestamp - timestampInGap;

    expect(distanceToGarage).toBeLessThan(distanceToFrontDoor);
    expect(resolvePlaybackStreamName(segments, -1, timestampInGap)).toBe('garage');
  });

  test('uses the actual local day length on a DST spring-forward day', () => {
    process.env.TZ = 'America/New_York';

    try {
      // 2020-03-08 is a known DST spring-forward date in America/New_York.
      const selectedDate = SPRING_FORWARD_DATE;
      const bounds = getLocalDayBounds(selectedDate);

      expect(bounds.endTimestamp - bounds.startTimestamp).toBe(23 * 3600);
      expect(bounds.durationHours).toBe(23);
      expect(getTimelineDayLengthHours(selectedDate)).toBe(23);

      // Verify that dayjs interprets the boundary timestamps using the configured timezone
      // by checking the local wall-clock times at the start and end of the day.
      const startLocal = dayjs.unix(bounds.startTimestamp);
      const endLocal = dayjs.unix(bounds.endTimestamp);
      const expectedStartInTimezone = dayjs.tz(`${selectedDate}T00:00:00`, 'America/New_York');

      // Explicitly verify the TZ-dependent local boundary matches New York midnight.
      expect(startLocal.unix()).toBe(expectedStartInTimezone.unix());
      expect(startLocal.utcOffset()).toBe(expectedStartInTimezone.utcOffset());

      // The selected local day should start at midnight on the selected date in America/New_York.
      expect(startLocal.year()).toBe(dayjs(selectedDate).year());
      expect(startLocal.month()).toBe(MARCH);
      expect(startLocal.date()).toBe(dayjs(selectedDate).date());
      expect(startLocal.hour()).toBe(0);
      expect(startLocal.minute()).toBe(0);
      expect(startLocal.second()).toBe(0);

      // The end timestamp should represent the end of that shortened 23-hour local day.
      expect(endLocal.unix() - startLocal.unix()).toBe(23 * 3600);
    } finally {
      process.env.TZ = originalTz;
    }
  });

  test('maps DST spring-forward timestamps to elapsed timeline offsets', () => {
    process.env.TZ = 'America/New_York';

    try {
      const selectedDate = SPRING_FORWARD_DATE;
      const timestamp = dayjs.tz(`${selectedDate}T03:10:00`, 'America/New_York').unix();

      expect(timestampToTimelineOffset(timestamp, selectedDate)).toBeCloseTo(2 + (10 / 60), 6);
      expect(timelineOffsetToTimestamp(2 + (10 / 60), selectedDate)).toBe(timestamp);
      expect(formatTimelineOffsetLabel(2, selectedDate)).toBe('3:00');

      // Confirm that dayjs sees this instant as the expected local wall-clock time.
      const local = dayjs.unix(timestamp);
      expect(local.year()).toBe(dayjs(selectedDate).year());
      expect(local.month()).toBe(MARCH);
      expect(local.date()).toBe(dayjs(selectedDate).date());
      expect(local.hour()).toBe(3);
      expect(local.minute()).toBe(10);
    } finally {
      process.env.TZ = originalTz;
    }
  });

  test('parses wall-clock query times on DST days without treating them as elapsed offsets', () => {
    process.env.TZ = 'America/New_York';

    try {
      const selectedDate = SPRING_FORWARD_DATE;
      const timestamp = localClockTimeToTimestamp('03:10:00', selectedDate);

      expect(timestamp).toBe(dayjs.tz(`${selectedDate}T03:10:00`, 'America/New_York').unix());
      expect(timestampToTimelineOffset(timestamp, selectedDate)).toBeCloseTo(2 + (10 / 60), 6);

      // Again, assert that dayjs interprets this timestamp as 03:10:00 local time on the selected day.
      const parsedLocal = dayjs.unix(timestamp);
      expect(parsedLocal.year()).toBe(dayjs(selectedDate).year());
      expect(parsedLocal.month()).toBe(MARCH);
      expect(parsedLocal.date()).toBe(dayjs(selectedDate).date());
      expect(parsedLocal.hour()).toBe(3);
      expect(parsedLocal.minute()).toBe(10);
    } finally {
      process.env.TZ = originalTz;
    }
  });

  test('uses the actual local day length on a DST fall-back day', () => {
    process.env.TZ = 'America/New_York';

    try {
      const bounds = getLocalDayBounds(FALL_BACK_DATE);

      expect(bounds.endTimestamp - bounds.startTimestamp).toBe(25 * 3600);
      expect(bounds.durationHours).toBe(25);
      expect(getTimelineDayLengthHours(FALL_BACK_DATE)).toBe(25);
    } finally {
      process.env.TZ = originalTz;
    }
  });

  test('shows the repeated local hour twice on a DST fall-back day', () => {
    process.env.TZ = 'America/New_York';

    try {
      const selectedDate = FALL_BACK_DATE;
      const firstRepeatedHourTimestamp = dayjs.tz(`${selectedDate}T01:10:00`, 'America/New_York').unix();
      const secondRepeatedHourTimestamp = dayjs
        .tz(`${selectedDate}T01:10:00`, 'America/New_York')
        .add(1, 'hour')
        .unix();

      expect(timestampToTimelineOffset(firstRepeatedHourTimestamp, selectedDate)).toBeCloseTo(1 + (10 / 60), 6);
      expect(timestampToTimelineOffset(secondRepeatedHourTimestamp, selectedDate)).toBeCloseTo(2 + (10 / 60), 6);
      expect(formatTimelineOffsetLabel(1, selectedDate)).toBe('1:00');
      expect(formatTimelineOffsetLabel(2, selectedDate)).toBe('1:00');
      expect(formatTimelineOffsetLabel(3, selectedDate)).toBe('2:00');

      // Also verify the reverse conversion from wall-clock time back to timeline offset
      const parsedFirstRepeatedHourTimestamp = localClockTimeToTimestamp('01:10:00', selectedDate);
      expect(parsedFirstRepeatedHourTimestamp).toBe(firstRepeatedHourTimestamp);
      expect(timestampToTimelineOffset(parsedFirstRepeatedHourTimestamp, selectedDate)).toBeCloseTo(1 + (10 / 60), 6);
    } finally {
      process.env.TZ = originalTz;
    }
  });

  test('handles clock boundary cases at midnight, end of day, and subsecond precision', () => {
    // Midnight at the start of the day
    expect(formatTimestampAsClock(baseLocalTestDateTimestamp(0, 0, 0))).toBe('00:00:00');

    // Last second of the day
    expect(formatTimestampAsClock(baseLocalTestDateTimestamp(23, 59, 59))).toBe('23:59:59');

    // Subsecond precision: ensure fractional seconds do not cause rounding drift
    const nearlyNextSecond = baseLocalTestDateTimestamp(12, 10, 30) + 0.999;
    expect(formatTimestampAsClock(nearlyNextSecond)).toBe('12:10:30');

    // Subsecond precision near zero: ensure small positive fractions do not change the displayed second
    const justAfterSecond = baseLocalTestDateTimestamp(12, 10, 30) + 0.001;
    expect(formatTimestampAsClock(justAfterSecond)).toBe('12:10:30');
  });

  test('formats timestamps as local YYYY-MM-DD keys', () => {
    expect(formatTimestampAsLocalDate(baseTestDateTimestamp(12, 10, 0))).toBe(SPRING_FORWARD_DATE);
  });

  test('returns all local dates touched by selected recordings, including overnight spans', () => {
    const mar8 = getLocalDayBounds(SPRING_FORWARD_DATE);
    const mar9 = getLocalDayBounds(SPRING_FORWARD_NEXT_DATE);

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

    expect(dates).toEqual([SPRING_FORWARD_DATE, SPRING_FORWARD_NEXT_DATE]);
  });

  test('does not add the next day when a recording ends exactly at midnight', () => {
    const mar8 = getLocalDayBounds(SPRING_FORWARD_DATE);

    expect(getAvailableDatesForSegments([
      {
        id: 1,
        start_timestamp: mar8.endTimestamp - 600,
        end_timestamp: mar8.endTimestamp
      }
    ])).toEqual([SPRING_FORWARD_DATE]);
  });

  test('does not add the previous day when a recording starts exactly at midnight', () => {
    const mar8 = getLocalDayBounds(SPRING_FORWARD_DATE);

    expect(getAvailableDatesForSegments([
      {
        id: 1,
        start_timestamp: mar8.startTimestamp,
        end_timestamp: mar8.startTimestamp + 600
      }
    ])).toEqual([SPRING_FORWARD_DATE]);
  });

  test('counts and locates segments visible on a selected day', () => {
    const mar8 = getLocalDayBounds(SPRING_FORWARD_DATE);
    const mar9 = getLocalDayBounds(SPRING_FORWARD_NEXT_DATE);
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

    expect(segmentIntersectsDay(segments[1], SPRING_FORWARD_DATE)).toBe(true);
    expect(segmentIntersectsDay(segments[1], SPRING_FORWARD_NEXT_DATE)).toBe(true);
    expect(countSegmentsForDate(segments, SPRING_FORWARD_DATE)).toBe(2);
    expect(countSegmentsForDate(segments, SPRING_FORWARD_NEXT_DATE)).toBe(2);
    expect(findFirstVisibleSegmentIndex(segments, SPRING_FORWARD_NEXT_DATE)).toBe(1);
  });

  test('returns 0 and -1 when segments are only on adjacent days', () => {
    const mar8 = getLocalDayBounds(SPRING_FORWARD_DATE);
    const mar10 = getLocalDayBounds('2020-03-10');

    const segments = [
      {
        // segment fully within previous day
        id: 1,
        start_timestamp: mar8.startTimestamp + 600,
        end_timestamp: mar8.startTimestamp + 1200
      },
      {
        // segment fully within next day
        id: 2,
        start_timestamp: mar10.startTimestamp + 600,
        end_timestamp: mar10.startTimestamp + 1200
      }
    ];

    expect(countSegmentsForDate(segments, SPRING_FORWARD_NEXT_DATE)).toBe(0);
    expect(findFirstVisibleSegmentIndex(segments, SPRING_FORWARD_NEXT_DATE)).toBe(-1);
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

