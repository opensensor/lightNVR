/**
 * Shared timeline helpers for choosing segments and clipping them to a day view.
 */

import dayjs from 'dayjs';

const pad = (value) => String(value).padStart(2, '0');

export const MIN_TIMELINE_VIEW_HOURS = 0.5;
export const MAX_TIMELINE_VIEW_HOURS = 24;

function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

function getLocalDayStart(selectedDate) {
  if (!selectedDate || typeof selectedDate !== 'string' || !selectedDate.includes('-')) {
    return null;
  }

  const localDayStart = dayjs(selectedDate).startOf('day');
  return localDayStart.isValid() ? localDayStart : null;
}

export function getTimelineDayLengthHours(selectedDate) {
  const bounds = getLocalDayBounds(selectedDate);
  return bounds?.durationHours ?? MAX_TIMELINE_VIEW_HOURS;
}

export function normalizeTimelineRange(startHour, endHour, maxHours = MAX_TIMELINE_VIEW_HOURS) {
  const cappedMaxHours = Number.isFinite(maxHours) && maxHours > 0
    ? maxHours
    : MAX_TIMELINE_VIEW_HOURS;
  const safeStart = Number.isFinite(startHour) ? startHour : 0;
  const safeEnd = Number.isFinite(endHour) ? endHour : cappedMaxHours;
  const requestedRange = safeEnd - safeStart;
  const range = clamp(requestedRange, MIN_TIMELINE_VIEW_HOURS, cappedMaxHours);
  const maxStart = cappedMaxHours - range;
  const start = clamp(safeStart, 0, maxStart);

  return {
    startHour: start,
    endHour: start + range
  };
}

export function panTimelineRange(startHour, endHour, deltaHours, maxHours = MAX_TIMELINE_VIEW_HOURS) {
  const normalized = normalizeTimelineRange(startHour, endHour, maxHours);
  if (!Number.isFinite(deltaHours) || deltaHours === 0) {
    return normalized;
  }

  const range = normalized.endHour - normalized.startHour;
  const cappedMaxHours = Number.isFinite(maxHours) && maxHours > 0
    ? maxHours
    : MAX_TIMELINE_VIEW_HOURS;
  const maxStart = cappedMaxHours - range;
  const nextStart = clamp(normalized.startHour + deltaHours, 0, maxStart);

  return {
    startHour: nextStart,
    endHour: nextStart + range
  };
}

export function zoomTimelineRange(startHour, endHour, zoomFactor, anchorHour = null, maxHours = MAX_TIMELINE_VIEW_HOURS) {
  const normalized = normalizeTimelineRange(startHour, endHour, maxHours);
  if (!Number.isFinite(zoomFactor) || zoomFactor <= 0 || zoomFactor === 1) {
    return normalized;
  }

  const cappedMaxHours = Number.isFinite(maxHours) && maxHours > 0
    ? maxHours
    : MAX_TIMELINE_VIEW_HOURS;
  const currentRange = normalized.endHour - normalized.startHour;
  const nextRange = clamp(currentRange * zoomFactor, MIN_TIMELINE_VIEW_HOURS, cappedMaxHours);
  const resolvedAnchorHour = clamp(
    Number.isFinite(anchorHour) ? anchorHour : ((normalized.startHour + normalized.endHour) / 2),
    normalized.startHour,
    normalized.endHour
  );
  const anchorRatio = currentRange > 0
    ? (resolvedAnchorHour - normalized.startHour) / currentRange
    : 0.5;
  const nextStart = resolvedAnchorHour - (anchorRatio * nextRange);

  return normalizeTimelineRange(nextStart, nextStart + nextRange, cappedMaxHours);
}

function formatLocalDate(date) {
  return dayjs(date).format('YYYY-MM-DD');
}

export function formatTimestampAsLocalDate(timestamp) {
  if (timestamp === null || timestamp === undefined || !Number.isFinite(timestamp)) {
    return '';
  }

  return dayjs.unix(timestamp).format('YYYY-MM-DD');
}

export function getLocalDayBounds(selectedDate) {
  const dayStart = getLocalDayStart(selectedDate);
  if (!dayStart) {
    return null;
  }

  const nextDayStart = dayStart.add(1, 'day');

  return {
    startTimestamp: dayStart.unix(),
    endTimestamp: nextDayStart.unix(),
    durationHours: nextDayStart.diff(dayStart, 'hour', true)
  };
}

export function formatTimestampAsClock(timestamp) {
  if (timestamp === null || timestamp === undefined) {
    return '';
  }

  return dayjs.unix(timestamp).format('HH:mm:ss');
}

export function formatPlaybackTimeLabel(timestamp, streamName = '') {
  const formattedTime = formatTimestampAsClock(timestamp);
  const trimmedStreamName = typeof streamName === 'string' ? streamName.trim() : '';

  if (!formattedTime) {
    return trimmedStreamName;
  }

  return trimmedStreamName ? `${trimmedStreamName} - ${formattedTime}` : formattedTime;
}

export function segmentIntersectsDay(segment, selectedDate) {
  return getClippedSegmentHourRange(segment, selectedDate) !== null;
}

export function getAvailableDatesForSegments(segments) {
  if (!Array.isArray(segments) || segments.length === 0) {
    return [];
  }

  const dates = new Set();

  segments.forEach(segment => {
    if (!segment || !Number.isFinite(segment.start_timestamp) || !Number.isFinite(segment.end_timestamp)) {
      return;
    }

    const effectiveEnd = Math.max(segment.start_timestamp, segment.end_timestamp - 1);
    let cursor = dayjs.unix(segment.start_timestamp).startOf('day');
    const endDate = dayjs.unix(effectiveEnd).startOf('day');

    while (cursor.isBefore(endDate) || cursor.isSame(endDate, 'day')) {
      dates.add(formatLocalDate(cursor));
      cursor = cursor.add(1, 'day');
    }
  });

  return Array.from(dates).sort();
}

export function countSegmentsForDate(segments, selectedDate) {
  if (!Array.isArray(segments) || !selectedDate) {
    return 0;
  }

  return segments.reduce((count, segment) => (
    count + (segmentIntersectsDay(segment, selectedDate) ? 1 : 0)
  ), 0);
}

export function timelineOffsetToTimestamp(offsetHours, selectedDate) {
  const bounds = getLocalDayBounds(selectedDate);
  if (!bounds) {
    throw new Error(`timelineOffsetToTimestamp: invalid selectedDate "${selectedDate}"`);
  }

  const numericOffset = Number(offsetHours);
  if (!Number.isFinite(numericOffset)) {
    throw new Error(`timelineOffsetToTimestamp: invalid offset value "${offsetHours}"`);
  }

  const normalizedOffset = clamp(numericOffset, 0, bounds.durationHours);
  return Math.round(bounds.startTimestamp + normalizedOffset * 3600);
}

export function timestampToTimelineOffset(timestamp, selectedDate = null) {
  if (timestamp === null || timestamp === undefined || !Number.isFinite(timestamp)) {
    return null;
  }

  const effectiveDate = selectedDate || formatTimestampAsLocalDate(timestamp);
  const bounds = getLocalDayBounds(effectiveDate);
  if (!bounds) {
    return null;
  }

  return (timestamp - bounds.startTimestamp) / 3600;
}

export function localClockTimeToTimestamp(timeString, selectedDate) {
  if (!selectedDate || typeof timeString !== 'string') {
    return null;
  }

  const timeMatch = timeString.match(/^(\d{2}):(\d{2}):(\d{2})$/);
  if (!timeMatch) {
    return null;
  }

  const timestamp = dayjs(`${selectedDate}T${timeString}`);
  return timestamp.isValid() ? timestamp.unix() : null;
}

export function formatTimelineOffsetLabel(offsetHours, selectedDate) {
  const bounds = getLocalDayBounds(selectedDate);
  if (!bounds || !Number.isFinite(offsetHours)) {
    return '';
  }

  const displayTimestamp = bounds.startTimestamp + (offsetHours * 3600);
  return dayjs.unix(displayTimestamp).format('H:mm');
}

export function findFirstVisibleSegmentIndex(segments, selectedDate) {
  if (!Array.isArray(segments) || segments.length === 0) {
    return -1;
  }

  for (let i = 0; i < segments.length; i++) {
    if (segmentIntersectsDay(segments[i], selectedDate)) {
      return i;
    }
  }

  return -1;
}

export function segmentContainsTimestamp(segment, timestamp) {
  if (!segment || timestamp === null || timestamp === undefined) return false;
  return timestamp >= segment.start_timestamp && timestamp <= segment.end_timestamp;
}

export function findContainingSegmentIndex(segments, timestamp) {
  if (!Array.isArray(segments) || segments.length === 0) return -1;

  let bestIndex = -1;
  let bestStart = -Infinity;
  let bestEnd = Infinity;

  for (let i = 0; i < segments.length; i++) {
    const segment = segments[i];
    if (!segmentContainsTimestamp(segment, timestamp)) continue;

    if (
      segment.start_timestamp > bestStart ||
      (segment.start_timestamp === bestStart && segment.end_timestamp < bestEnd)
    ) {
      bestIndex = i;
      bestStart = segment.start_timestamp;
      bestEnd = segment.end_timestamp;
    }
  }

  return bestIndex;
}

export function findNearestSegmentIndex(segments, timestamp) {
  if (!Array.isArray(segments) || segments.length === 0) return -1;

  let bestIndex = -1;
  let bestDistance = Infinity;
  let bestStart = Infinity;

  for (let i = 0; i < segments.length; i++) {
    const segment = segments[i];
    const distance = timestamp < segment.start_timestamp
      ? segment.start_timestamp - timestamp
      : timestamp > segment.end_timestamp
        ? timestamp - segment.end_timestamp
        : 0;

    if (
      distance < bestDistance ||
      (distance === bestDistance && segment.start_timestamp < bestStart)
    ) {
      bestIndex = i;
      bestDistance = distance;
      bestStart = segment.start_timestamp;
    }
  }

  return bestIndex;
}

export function getClippedSegmentHourRange(segment, selectedDate) {
  const bounds = getLocalDayBounds(selectedDate);
  if (!segment || !bounds) return null;

  const visibleStart = Math.max(segment.start_timestamp, bounds.startTimestamp);
  const visibleEnd = Math.min(segment.end_timestamp, bounds.endTimestamp);

  if (visibleEnd <= visibleStart) {
    return null;
  }

  return {
    startHour: (visibleStart - bounds.startTimestamp) / 3600,
    endHour: (visibleEnd - bounds.startTimestamp) / 3600
  };
}

