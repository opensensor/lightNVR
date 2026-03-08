/**
 * Shared timeline helpers for choosing segments and clipping them to a day view.
 */

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

