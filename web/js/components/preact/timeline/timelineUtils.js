/**
 * Shared timeline helpers for choosing segments and clipping them to a day view.
 */

const pad = (value) => String(value).padStart(2, '0');

function formatLocalDate(date) {
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;
}

export function formatTimestampAsLocalDate(timestamp) {
  if (timestamp === null || timestamp === undefined || !Number.isFinite(timestamp)) {
    return '';
  }

  return formatLocalDate(new Date(timestamp * 1000));
}

export function getLocalDayBounds(selectedDate) {
  if (!selectedDate || typeof selectedDate !== 'string' || !selectedDate.includes('-')) {
    return null;
  }

  const [year, month, day] = selectedDate.split('-').map(Number);
  if (![year, month, day].every(Number.isFinite)) {
    return null;
  }

  const dayStart = new Date(year, month - 1, day, 0, 0, 0, 0);
  const nextDayStart = new Date(year, month - 1, day + 1, 0, 0, 0, 0);

  return {
    startTimestamp: Math.floor(dayStart.getTime() / 1000),
    endTimestamp: Math.floor(nextDayStart.getTime() / 1000)
  };
}

export function formatTimestampAsClock(timestamp) {
  if (timestamp === null || timestamp === undefined) {
    return '';
  }

  const date = new Date(timestamp * 1000);
  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  const seconds = date.getSeconds().toString().padStart(2, '0');

  return `${hours}:${minutes}:${seconds}`;
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
    const cursor = new Date(segment.start_timestamp * 1000);
    cursor.setHours(0, 0, 0, 0);

    const endDate = new Date(effectiveEnd * 1000);
    endDate.setHours(0, 0, 0, 0);

    while (cursor <= endDate) {
      dates.add(formatLocalDate(cursor));
      cursor.setDate(cursor.getDate() + 1);
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

