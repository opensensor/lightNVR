export const MAX_INVESTIGATION_CAMERAS = 16;
export const MAX_ACTIVE_INVESTIGATION_PLAYERS = 4;

const REGION_MATCHES = new Set(['center', 'intersects', 'minimum_intersection']);

function roundedRegionValue(value) {
  return Math.round(value * 10000) / 10000;
}

export function videoContentBox(containerWidth, containerHeight, videoWidth, videoHeight) {
  if (![containerWidth, containerHeight, videoWidth, videoHeight]
    .every((value) => Number.isFinite(value) && value > 0)) {
    return { left: 0, top: 0, width: 1, height: 1 };
  }
  const containerAspect = containerWidth / containerHeight;
  const videoAspect = videoWidth / videoHeight;
  if (videoAspect >= containerAspect) {
    const height = containerAspect / videoAspect;
    return { left: 0, top: (1 - height) / 2, width: 1, height };
  }
  const width = videoAspect / containerAspect;
  return { left: (1 - width) / 2, top: 0, width, height: 1 };
}

export function normalizedRegionRectangle(anchor, point) {
  if (!anchor || !point ||
      ![anchor.x, anchor.y, point.x, point.y].every(Number.isFinite)) return null;
  const x = Math.max(0, Math.min(1, Math.min(anchor.x, point.x)));
  const y = Math.max(0, Math.min(1, Math.min(anchor.y, point.y)));
  const maxX = Math.max(0, Math.min(1, Math.max(anchor.x, point.x)));
  const maxY = Math.max(0, Math.min(1, Math.max(anchor.y, point.y)));
  return {
    x: roundedRegionValue(x),
    y: roundedRegionValue(y),
    width: roundedRegionValue(maxX - x),
    height: roundedRegionValue(maxY - y),
  };
}

export function parseInvestigationRegion(params) {
  if (!params?.get) return null;
  const cameraUuid = params.get('region_camera') || '';
  const values = (params.get('region_rect') || '').split(',').map(Number);
  const match = params.get('region_match') || 'center';
  const minIntersection = Number(params.get('region_min') || 0.25);
  if (cameraUuid.length !== 36 || values.length !== 4 ||
      !values.every(Number.isFinite) || !REGION_MATCHES.has(match) ||
      !Number.isFinite(minIntersection) || minIntersection <= 0 ||
      minIntersection > 1) return null;
  const [x, y, width, height] = values;
  if (x < 0 || y < 0 || width <= 0 || height <= 0 ||
      x + width > 1 || y + height > 1) return null;
  return { cameraUuid, x, y, width, height, match, minIntersection };
}

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
