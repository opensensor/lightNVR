export const MAX_INVESTIGATION_CAMERAS = 16;
export const MAX_ACTIVE_INVESTIGATION_PLAYERS = 4;

const REGION_MATCHES = new Set(['center', 'intersects', 'minimum_intersection']);

function roundedRegionValue(value) {
  return Math.round(value * 10000) / 10000;
}

export function bookmarkFiltersFromState(filters = {}) {
  const saved = {};
  const fields = [
    ['eventType', 'event_type'],
    ['location', 'location'],
    ['label', 'label'],
    ['zone', 'zone'],
    ['source', 'source'],
    ['captureMethod', 'capture_method'],
    ['recordingTag', 'recording_tag'],
    ['protection', 'protection'],
  ];
  fields.forEach(([stateKey, savedKey]) => {
    if (filters[stateKey]) saved[savedKey] = filters[stateKey];
  });
  if (filters.minConfidence !== '' && filters.minConfidence !== null &&
      filters.minConfidence !== undefined) {
    const value = Number(filters.minConfidence);
    if (Number.isFinite(value) && value >= 0 && value <= 1) {
      saved.min_confidence = value;
    }
  }
  if (filters.region) {
    const region = filters.region;
    saved.region = {
      camera_uuid: region.cameraUuid,
      x: region.x,
      y: region.y,
      width: region.width,
      height: region.height,
      match: region.match,
      min_intersection: region.minIntersection,
    };
  }
  return saved;
}

export function bookmarkRepresentativeResult(result) {
  if (!result) return null;
  const saved = {};
  [
    'result_id', 'camera_uuid', 'start_time', 'end_time', 'event_type',
    'recording_id', 'detection_id', 'label',
  ].forEach((key) => {
    if (typeof result[key] === 'string' || Number.isFinite(result[key])) {
      saved[key] = result[key];
    }
  });
  return Object.keys(saved).length > 0 ? saved : null;
}

export function buildInvestigationBookmarkPayload({
  title,
  note,
  timeline,
  cursor,
  primaryCameraUuid,
  searchFilters,
  selectedResult,
}) {
  if (!timeline || !Array.isArray(timeline.tracks) ||
      timeline.tracks.length === 0 || !primaryCameraUuid) return null;
  return {
    title: String(title || '').trim(),
    note: String(note || '').trim(),
    camera_uuids: timeline.tracks.map((track) => track.camera_uuid),
    start_time: Math.floor(timeline.start_time),
    end_time: Math.floor(timeline.end_time),
    cursor_time: Math.floor(cursor),
    primary_camera_uuid: primaryCameraUuid,
    filters: bookmarkFiltersFromState(searchFilters),
    representative_result: bookmarkRepresentativeResult(selectedResult),
  };
}

export function investigationActionSelection(timeline, selectedResult, scope) {
  if (!timeline || !Array.isArray(timeline.tracks) ||
      !Number.isFinite(timeline.start_time) ||
      !Number.isFinite(timeline.end_time) ||
      timeline.end_time <= timeline.start_time) return null;
  if (scope === 'result' && selectedResult?.camera_uuid &&
      Number.isFinite(selectedResult.start_time)) {
    const startTime = Math.max(
      Math.floor(timeline.start_time), Math.floor(selectedResult.start_time),
    );
    const rawEnd = Number.isFinite(selectedResult.end_time)
      ? Math.ceil(selectedResult.end_time) : startTime + 1;
    const endTime = Math.min(
      Math.ceil(timeline.end_time), Math.max(startTime + 1, rawEnd),
    );
    if (endTime > startTime) {
      return {
        cameraUuids: [selectedResult.camera_uuid],
        startTime,
        endTime,
      };
    }
  }
  const cameraUuids = timeline.tracks
    .map((track) => track.camera_uuid)
    .filter(Boolean);
  return cameraUuids.length > 0 ? {
    cameraUuids,
    startTime: Math.floor(timeline.start_time),
    endTime: Math.ceil(timeline.end_time),
  } : null;
}

export function summarizeInvestigationActionPreview(preview) {
  const recordings = Array.isArray(preview?.recordings)
    ? preview.recordings : [];
  const unprotected = recordings.filter((recording) => !recording.protected);
  const protectable = unprotected.filter((recording) => recording.can_protect);
  const protectDenied = unprotected.filter((recording) => !recording.can_protect);
  const exportDenied = recordings.filter((recording) => !recording.can_export);
  return {
    recordingCount: recordings.length,
    // Only the recordings the caller may actually protect. Submitting the
    // out-of-scope ones would just have the server refuse them and report
    // failures the preview already warned about.
    protectableIds: protectable.map((recording) => recording.id),
    protectableCount: protectable.length,
    protectDeniedCount: protectDenied.length,
    exportDeniedCount: exportDenied.length,
    canProtect: protectable.length > 0,
    canExport: recordings.length > 0 && exportDenied.length === 0,
  };
}

export function investigationBookmarkUrl(bookmark, baseUrl) {
  const url = new URL(baseUrl || window.location.href);
  [
    'stream', 'cameras', 'start', 'end', 'cursor', 'primary',
    'event', 'location', 'label', 'zone', 'source', 'capture', 'tag',
    'protected', 'confidence_min', 'region_camera', 'region_rect',
    'region_match', 'region_min', 'drill_camera', 'drill_start', 'drill_end',
  ].forEach((name) => url.searchParams.delete(name));
  url.searchParams.set('cameras', (bookmark.camera_uuids || []).join(','));
  url.searchParams.set('start', String(bookmark.start_time));
  url.searchParams.set('end', String(bookmark.end_time));
  url.searchParams.set('cursor', String(bookmark.cursor_time));
  url.searchParams.set('primary', bookmark.primary_camera_uuid);
  const filters = bookmark.filters || {};
  const queryFields = {
    event: filters.event_type,
    location: filters.location,
    label: filters.label,
    zone: filters.zone,
    source: filters.source,
    capture: filters.capture_method,
    tag: filters.recording_tag,
    protected: filters.protection,
    confidence_min: filters.min_confidence,
  };
  Object.entries(queryFields).forEach(([key, value]) => {
    if (value !== undefined && value !== null && value !== '') {
      url.searchParams.set(key, String(value));
    }
  });
  if (filters.region) {
    url.searchParams.set('region_camera', filters.region.camera_uuid);
    url.searchParams.set('region_rect', [
      filters.region.x,
      filters.region.y,
      filters.region.width,
      filters.region.height,
    ].join(','));
    url.searchParams.set('region_match', filters.region.match || 'center');
    url.searchParams.set(
      'region_min', String(filters.region.min_intersection ?? 0.25),
    );
  }
  return url.toString();
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

export function narrowThumbnailWindow(samples, selectedIndex, startTime, endTime) {
  if (!Array.isArray(samples) || samples.length < 2 ||
      !Number.isInteger(selectedIndex) || selectedIndex < 0 ||
      selectedIndex >= samples.length || !Number.isFinite(startTime) ||
      !Number.isFinite(endTime) || endTime <= startTime) return null;
  const selected = Number(samples[selectedIndex]?.timestamp);
  if (!Number.isFinite(selected)) return null;
  const previous = Number(samples[selectedIndex - 1]?.timestamp);
  const next = Number(samples[selectedIndex + 1]?.timestamp);
  const narrowedStart = selectedIndex === 0 || !Number.isFinite(previous)
    ? startTime : Math.floor((previous + selected) / 2);
  const narrowedEnd = selectedIndex === samples.length - 1 || !Number.isFinite(next)
    ? endTime : Math.ceil((selected + next) / 2);
  if (narrowedStart < startTime || narrowedEnd > endTime ||
      narrowedEnd - narrowedStart < 1 ||
      (narrowedStart === startTime && narrowedEnd === endTime)) return null;
  return { startTime: narrowedStart, endTime: narrowedEnd };
}

export function thumbnailWindowForResult(result, timelineStart, timelineEnd) {
  if (!result || !Number.isFinite(timelineStart) ||
      !Number.isFinite(timelineEnd) || timelineEnd <= timelineStart) return null;
  const eventStart = Number(result.start_time);
  const eventEnd = Number(result.end_time ?? result.start_time);
  if (!Number.isFinite(eventStart) || !Number.isFinite(eventEnd)) return null;
  const padding = Math.max(30, Math.max(0, eventEnd - eventStart));
  const startTime = Math.max(timelineStart, Math.floor(eventStart - padding));
  const endTime = Math.min(timelineEnd, Math.ceil(eventEnd + padding));
  return endTime > startTime ? { startTime, endTime } : null;
}

export function formatDateTimeLocal(timestamp, includeSeconds = false) {
  if (!Number.isFinite(timestamp)) return '';
  const date = new Date(timestamp * 1000);
  const offset = date.getTimezoneOffset() * 60 * 1000;
  return new Date(date.getTime() - offset).toISOString()
    .slice(0, includeSeconds ? 19 : 16);
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
