/**
 * License plate (LPR) search helpers.
 *
 * Plate text is sensitive: these helpers never persist filters or results and
 * never place a plate in a URL. Callers keep filter state in memory only.
 */

import type {
  LprAction,
  LprDeniedActions,
  LprExportOutcome,
  LprFilters,
  LprRead,
  LprRequestOptions,
  LprResponseOutcome,
  LprSearchBody,
  LprSearchOutcome,
  LprValidationError,
} from '../../../types/investigation.ts';
import type { StreamSummary } from '../../../types/core.ts';

export const LPR_DEFAULT_LIMIT = 50;
export const LPR_SEARCH_LIMIT_MAX = 100;
export const LPR_EXPORT_LIMIT = 1000;
export const LPR_MAX_RANGE_MS = 366 * 24 * 60 * 60 * 1000;
// Mirrors LPR_QUERY_MAX (64 bytes including the terminator) on the server.
export const LPR_PLATE_QUERY_MAX_LENGTH = 63;
// Exact matches need two letters or digits; "contains" matches need three.
export const LPR_PLATE_QUERY_MIN_CHARACTERS = 2;
export const LPR_PARTIAL_QUERY_MIN_CHARACTERS = 3;
export const LPR_DEFAULT_WINDOW_MS = 24 * 60 * 60 * 1000;

const pad = (value: number) => String(value).padStart(2, '0');

/** Format epoch milliseconds for a datetime-local input in the browser zone. */
export function toDateTimeLocalValue(milliseconds: number) {
  if (!Number.isFinite(milliseconds)) return '';
  const date = new Date(milliseconds);
  if (!Number.isFinite(date.getTime())) return '';
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`
    + `T${pad(date.getHours())}:${pad(date.getMinutes())}`;
}

/** Parse a datetime-local value as local time; returns epoch ms or null. */
export function parseDateTimeLocalValue(value: unknown) {
  if (typeof value !== 'string' || !value) return null;
  const match = /^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2})(?::(\d{2}))?$/.exec(value);
  if (!match) return null;
  const [, year, month, day, hour, minute, second = '0'] = match;
  const yearNum = Number(year);
  const monthNum = Number(month);
  const dayNum = Number(day);
  const hourNum = Number(hour);
  const minuteNum = Number(minute);
  const secondNum = Number(second);
  const date = new Date(
    yearNum, monthNum - 1, dayNum,
    hourNum, minuteNum, secondNum,
  );
  if (
    date.getFullYear() !== yearNum
    || date.getMonth() !== monthNum - 1
    || date.getDate() !== dayNum
    || date.getHours() !== hourNum
    || date.getMinutes() !== minuteNum
    || date.getSeconds() !== secondNum
  ) {
    return null;
  }
  return Number.isFinite(date.getTime()) ? date.getTime() : null;
}

export function createLprFilters(now = Date.now()): LprFilters {
  return {
    cameraUuid: '',
    start: toDateTimeLocalValue(now - LPR_DEFAULT_WINDOW_MS),
    end: toDateTimeLocalValue(now),
    plate: '',
    match: 'partial',
    limit: String(LPR_DEFAULT_LIMIT),
  };
}

function canonicalPlateLength(plate: string) {
  return plate.replace(/[^A-Za-z0-9]/g, '').length;
}

/**
 * Validate filters before they reach the server, which answers every
 * malformed query with the same generic 400. Returns translation descriptors.
 */
export function validateLprFilters(filters: Partial<LprFilters> | null | undefined) {
  const errors: LprValidationError[] = [];
  if (!filters?.cameraUuid) {
    errors.push({ field: 'camera', key: 'lpr.validation.cameraRequired' });
  }

  const start = parseDateTimeLocalValue(filters?.start);
  const end = parseDateTimeLocalValue(filters?.end);
  if (start === null) errors.push({ field: 'start', key: 'lpr.validation.startInvalid' });
  if (end === null) errors.push({ field: 'end', key: 'lpr.validation.endInvalid' });
  if (start !== null && end !== null) {
    if (end < start) {
      errors.push({ field: 'end', key: 'lpr.validation.rangeOrder' });
    } else if (end - start > LPR_MAX_RANGE_MS) {
      errors.push({ field: 'end', key: 'lpr.validation.rangeTooLong', params: { days: 366 } });
    }
  }

  const limitText = String(filters?.limit ?? '').trim();
  const limit = Number(limitText);
  if (!/^\d+$/.test(limitText) || limit < 1 || limit > LPR_SEARCH_LIMIT_MAX) {
    errors.push({
      field: 'limit',
      key: 'lpr.validation.limit',
      params: { max: LPR_SEARCH_LIMIT_MAX },
    });
  }

  const plate = String(filters?.plate ?? '').trim();
  if (plate) {
    if (!['exact', 'partial'].includes(filters?.match as string)) {
      errors.push({ field: 'match', key: 'lpr.validation.match' });
    }
    // eslint-disable-next-line no-control-regex
    if (/[^\x00-\x7f]/.test(plate)) {
      errors.push({ field: 'plate', key: 'lpr.validation.plateAscii' });
    } else if (plate.length > LPR_PLATE_QUERY_MAX_LENGTH) {
      errors.push({
        field: 'plate',
        key: 'lpr.validation.plateTooLong',
        params: { max: LPR_PLATE_QUERY_MAX_LENGTH },
      });
    } else {
      const minimum = filters?.match === 'exact'
        ? LPR_PLATE_QUERY_MIN_CHARACTERS
        : LPR_PARTIAL_QUERY_MIN_CHARACTERS;
      if (canonicalPlateLength(plate) < minimum) {
        errors.push({
          field: 'plate',
          key: 'lpr.validation.plateTooShort',
          params: { min: minimum },
        });
      }
    }
  }
  return errors;
}

/** Build the POST body for /api/lpr/search or /api/lpr/export. */
export function buildLprSearchBody(
  filters: LprFilters,
  { exporting = false }: { exporting?: boolean } = {},
) {
  const body: LprSearchBody = {
    camera_uuid: filters.cameraUuid,
    start_at: parseDateTimeLocalValue(filters.start),
    end_at: parseDateTimeLocalValue(filters.end),
    limit: exporting ? LPR_EXPORT_LIMIT : Number(String(filters.limit).trim()),
  };
  const plate = String(filters.plate ?? '').trim();
  if (plate) {
    body.match = filters.match === 'exact' ? 'exact' : 'partial';
    body.plate = plate;
  }
  return body;
}

async function readErrorMessage(response: Response) {
  try {
    const data: { error?: unknown } | null = await response.json();
    return typeof data?.error === 'string' ? data.error : '';
  } catch {
    return '';
  }
}

/**
 * Map an LPR API response to an outcome the view can render without toasts:
 * ok | forbidden | invalid | not_found | unavailable | unauthorized | error.
 */
export async function classifyLprResponse(response: Response): Promise<LprResponseOutcome> {
  if (response.ok) return { outcome: 'ok' };
  const message = await readErrorMessage(response);
  switch (response.status) {
    case 400: return { outcome: 'invalid', message };
    case 401: return { outcome: 'unauthorized', message };
    case 403: return { outcome: 'forbidden', message };
    case 404: return { outcome: 'not_found', message };
    case 503: return { outcome: 'unavailable', message };
    default: return { outcome: 'error', status: response.status, message };
  }
}

function postJson(
  fetchImpl: typeof fetch,
  url: string,
  body: LprSearchBody,
  signal: AbortSignal | undefined,
) {
  return fetchImpl(url, {
    method: 'POST',
    credentials: 'same-origin',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
    signal,
  });
}

export async function searchLprReads(
  body: LprSearchBody,
  { fetchImpl = globalThis.fetch, signal }: LprRequestOptions = {},
): Promise<LprSearchOutcome> {
  const response = await postJson(fetchImpl, '/api/lpr/search', body, signal);
  const result = await classifyLprResponse(response);
  if (result.outcome !== 'ok') return result;
  const data: { reads?: LprRead[] } | null = await response.json();
  return { outcome: 'ok', reads: Array.isArray(data?.reads) ? data.reads : [] };
}

export async function exportLprReads(
  body: LprSearchBody,
  { fetchImpl = globalThis.fetch, signal }: LprRequestOptions = {},
): Promise<LprExportOutcome> {
  const response = await postJson(fetchImpl, '/api/lpr/export', body, signal);
  const result = await classifyLprResponse(response);
  if (result.outcome !== 'ok') return result;
  return { outcome: 'ok', blob: await response.blob() };
}

export async function deleteLprRead(
  readId: string,
  { fetchImpl = globalThis.fetch }: Pick<LprRequestOptions, 'fetchImpl'> = {},
) {
  const response = await fetchImpl(`/api/lpr/reads/${encodeURIComponent(readId)}`, {
    method: 'DELETE',
    credentials: 'same-origin',
  });
  return classifyLprResponse(response);
}

/**
 * Per-camera action availability. The API offers no capability probe, so an
 * action stays visible until the server refuses it with 403 for that camera.
 */
export function lprActionsFor(
  deniedActions: LprDeniedActions | null | undefined,
  cameraUuid: string,
) {
  const denied: Partial<Record<LprAction, boolean>> = deniedActions?.[cameraUuid] || {};
  return {
    canSearch: !denied.search,
    canExport: !denied.export,
    canDelete: !denied.delete,
  };
}

export function denyLprAction(
  deniedActions: LprDeniedActions | null | undefined,
  cameraUuid: string,
  action: LprAction,
): LprDeniedActions {
  return {
    ...deniedActions,
    [cameraUuid]: { ...(deniedActions?.[cameraUuid] || {}), [action]: true },
  };
}

/**
 * Timeline deep link for a read. Recording IDs and camera UUIDs preserve
 * capture identity across camera renames. Names are only a display hint or a
 * compatibility fallback for reads without either identity.
 * The plate itself is never part of the link.
 */
export function lprTimelineHref(
  read: LprRead | null | undefined,
  streams: Array<Pick<StreamSummary, 'camera_uuid' | 'name'>> = [],
) {
  const value: unknown = read?.observed_at;
  if ((typeof value !== 'number' && typeof value !== 'string')
      || (typeof value === 'string' && value.trim() === '')) return null;
  const observedAt = Number(value);
  if (!Number.isFinite(observedAt)) return null;
  const date = new Date(observedAt);
  if (!Number.isFinite(date.getTime())) return null;
  const current = Array.isArray(streams)
    ? streams.find((stream) => stream?.camera_uuid && stream.camera_uuid === read!.camera_uuid)
    : null;
  const streamName = current?.name || read?.stream_name;
  const recordingId = Number(read?.recording_id);
  const hasRecordingId = Number.isSafeInteger(recordingId) && recordingId > 0;
  if (!hasRecordingId && !read?.camera_uuid && !streamName) return null;
  const params = new URLSearchParams();
  if (hasRecordingId) {
    params.set('ids', String(recordingId));
  } else {
    if (read?.camera_uuid) params.set('camera_uuid', read.camera_uuid);
    if (streamName) params.set('stream', streamName);
  }
  params.set('date', `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`);
  params.set('time', `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`);
  return `timeline.html?${params.toString()}`;
}

export function formatLprConfidence(confidence: number | null | undefined) {
  // The server stores confidence normalized to 0..1.
  if (!Number.isFinite(confidence)) return '';
  return `${Math.round(confidence! * 100)}%`;
}
