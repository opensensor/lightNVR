export const MAX_INVESTIGATION_CAMERAS = 16;
export const MAX_ACTIVE_INVESTIGATION_PLAYERS = 4;

export function findSegmentAt(segments, timestamp) {
  if (!Array.isArray(segments) || !Number.isFinite(timestamp)) return null;
  let low = 0;
  let high = segments.length - 1;
  while (low <= high) {
    const middle = Math.floor((low + high) / 2);
    const segment = segments[middle];
    if (timestamp < segment.start_time) {
      high = middle - 1;
    } else if (timestamp > segment.end_time) {
      low = middle + 1;
    } else {
      return segment;
    }
  }
  return null;
}

export function nextAvailableTimestamp(tracks, activeCameraUuids, timestamp) {
  const active = new Set(activeCameraUuids || []);
  let next = null;
  for (const track of tracks || []) {
    if (!active.has(track.camera_uuid)) continue;
    for (const segment of track.segments || []) {
      if (segment.end_time < timestamp) continue;
      if (segment.start_time <= timestamp) return timestamp;
      if (next === null || segment.start_time < next) next = segment.start_time;
      break;
    }
  }
  return next;
}

export function advanceInvestigationCursor({
  cursor,
  elapsedSeconds,
  speed,
  endTime,
  mode,
  tracks,
  activeCameraUuids,
}) {
  const candidate = Math.min(endTime, cursor + elapsedSeconds * speed);
  if (mode !== 'skip-common-gaps') return candidate;
  if ((tracks || []).some((track) =>
    (activeCameraUuids || []).includes(track.camera_uuid) &&
    findSegmentAt(track.segments, candidate))) {
    return candidate;
  }
  return Math.min(
    endTime,
    nextAvailableTimestamp(tracks, activeCameraUuids, candidate) ?? endTime,
  );
}

export function segmentTrackPosition(segment, startTime, endTime) {
  const duration = Math.max(endTime - startTime, 1);
  const clippedStart = Math.max(segment.start_time, startTime);
  const clippedEnd = Math.min(segment.end_time, endTime);
  return {
    left: `${Math.max(0, ((clippedStart - startTime) / duration) * 100)}%`,
    width: `${Math.max(0.2, ((clippedEnd - clippedStart) / duration) * 100)}%`,
  };
}

export function adjacentInvestigationResultIndex(results, selectedResultId, direction) {
  if (!Array.isArray(results) || results.length === 0) return -1;
  const step = direction < 0 ? -1 : 1;
  const selectedIndex = results.findIndex((result) =>
    result.result_id === selectedResultId);
  if (selectedIndex < 0) return step < 0 ? results.length - 1 : 0;
  const nextIndex = selectedIndex + step;
  return nextIndex >= 0 && nextIndex < results.length ? nextIndex : -1;
}

export function formatDateTimeLocal(timestamp) {
  if (!Number.isFinite(timestamp)) return '';
  const date = new Date(timestamp * 1000);
  const offset = date.getTimezoneOffset() * 60 * 1000;
  return new Date(date.getTime() - offset).toISOString().slice(0, 16);
}

export function parseDateTimeLocal(value) {
  const timestamp = new Date(value).getTime();
  return Number.isFinite(timestamp) ? Math.floor(timestamp / 1000) : null;
}

export function formatCursorTime(timestamp) {
  if (!Number.isFinite(timestamp)) return '—';
  return new Intl.DateTimeFormat(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    timeZoneName: 'short',
  }).format(new Date(timestamp * 1000));
}
