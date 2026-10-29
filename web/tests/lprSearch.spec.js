import {
  LPR_EXPORT_LIMIT,
  LPR_MAX_RANGE_MS,
  buildLprSearchBody,
  createLprFilters,
  deleteLprRead,
  denyLprAction,
  exportLprReads,
  formatLprConfidence,
  lprActionsFor,
  lprTimelineHref,
  parseDateTimeLocalValue,
  searchLprReads,
  toDateTimeLocalValue,
  validateLprFilters,
} from '../js/components/preact/lpr/lprSearch.js';
import { buildTimelineSegmentsUrl } from '../js/components/preact/timeline/timelineUtils.js';

const CAMERA = 'ad1d3d70-36ee-4f0c-9225-3b88373f42cb';

const jsonResponse = (status, body) => new Response(JSON.stringify(body), {
  status,
  headers: { 'Content-Type': 'application/json' },
});

const filters = (overrides = {}) => ({
  cameraUuid: CAMERA,
  start: '2026-10-28T15:00',
  end: '2026-10-28T16:00',
  plate: '',
  match: 'partial',
  limit: '50',
  ...overrides,
});

const keys = (errors) => errors.map((error) => error.key);

describe('LPR filter state', () => {
  test('defaults to the last 24 hours, partial matching and 50 results', () => {
    const now = new Date(2026, 9, 28, 15, 31).getTime();
    expect(createLprFilters(now)).toEqual({
      cameraUuid: '',
      start: '2026-10-27T15:31',
      end: '2026-10-28T15:31',
      plate: '',
      match: 'partial',
      limit: '50',
    });
  });

  test('datetime-local values round-trip in the browser zone', () => {
    const milliseconds = new Date(2026, 9, 28, 15, 31).getTime();
    expect(parseDateTimeLocalValue(toDateTimeLocalValue(milliseconds))).toBe(milliseconds);
    expect(parseDateTimeLocalValue('not a date')).toBeNull();
    expect(parseDateTimeLocalValue('')).toBeNull();
  });
});

describe('LPR request body', () => {
  test('omits match and plate when no plate is entered', () => {
    expect(buildLprSearchBody(filters())).toEqual({
      camera_uuid: CAMERA,
      start_at: new Date(2026, 9, 28, 15, 0).getTime(),
      end_at: new Date(2026, 9, 28, 16, 0).getTime(),
      limit: 50,
    });
  });

  test('sends a trimmed plate with the selected match mode', () => {
    const body = buildLprSearchBody(filters({ plate: '  abc-12 ', match: 'exact', limit: ' 25 ' }));
    expect(body).toMatchObject({ match: 'exact', plate: 'abc-12', limit: 25 });
  });

  test('exports use the server export ceiling instead of the page size', () => {
    const body = buildLprSearchBody(filters({ plate: 'AB1', limit: '10' }), { exporting: true });
    expect(body).toMatchObject({ limit: LPR_EXPORT_LIMIT, match: 'partial', plate: 'AB1' });
  });
});

describe('LPR filter validation', () => {
  test('accepts a valid partial plate search', () => {
    expect(validateLprFilters(filters({ plate: 'AB 1' }))).toEqual([]);
  });

  test('requires a camera and valid times', () => {
    expect(keys(validateLprFilters(filters({ cameraUuid: '', start: '', end: 'x' })))).toEqual([
      'lpr.validation.cameraRequired',
      'lpr.validation.startInvalid',
      'lpr.validation.endInvalid',
    ]);
  });

  test('rejects reversed and over-long ranges like the server', () => {
    expect(keys(validateLprFilters(filters({ start: '2026-10-28T16:00', end: '2026-10-28T15:00' }))))
      .toEqual(['lpr.validation.rangeOrder']);
    const start = new Date(2025, 0, 1).getTime();
    const tooLong = toDateTimeLocalValue(start + LPR_MAX_RANGE_MS + 60 * 1000);
    expect(validateLprFilters(filters({ start: toDateTimeLocalValue(start), end: tooLong })))
      .toEqual([{ field: 'end', key: 'lpr.validation.rangeTooLong', params: { days: 366 } }]);
  });

  test.each(['0', '101', '2.5', '', 'ten'])('rejects limit %p', (limit) => {
    expect(keys(validateLprFilters(filters({ limit })))).toEqual(['lpr.validation.limit']);
  });

  test('applies the server plate canonicalization rules', () => {
    expect(keys(validateLprFilters(filters({ plate: 'A-', match: 'exact' })))).toEqual(['lpr.validation.plateTooShort']);
    expect(validateLprFilters(filters({ plate: 'A-1', match: 'exact' }))).toEqual([]);
    expect(validateLprFilters(filters({ plate: 'A-1', match: 'partial' })))
      .toEqual([{ field: 'plate', key: 'lpr.validation.plateTooShort', params: { min: 3 } }]);
    expect(validateLprFilters(filters({ plate: 'A-1B', match: 'partial' }))).toEqual([]);
    expect(keys(validateLprFilters(filters({ plate: 'ÄB12' })))).toEqual(['lpr.validation.plateAscii']);
    expect(keys(validateLprFilters(filters({ plate: 'A'.repeat(64) })))).toEqual(['lpr.validation.plateTooLong']);
    expect(keys(validateLprFilters(filters({ plate: 'AB12', match: 'fuzzy' })))).toEqual(['lpr.validation.match']);
  });
});

describe('LPR API calls', () => {
  test('search posts the body as JSON and returns reads', async () => {
    const read = { id: 'r1', camera_uuid: CAMERA, plate: 'ABC123', observed_at: 1 };
    const fetchImpl = jest.fn().mockResolvedValue(jsonResponse(200, { reads: [read], count: 1 }));
    const body = buildLprSearchBody(filters({ plate: 'abc' }));

    await expect(searchLprReads(body, { fetchImpl })).resolves.toEqual({ outcome: 'ok', reads: [read] });
    expect(fetchImpl).toHaveBeenCalledWith('/api/lpr/search', expect.objectContaining({
      method: 'POST',
      credentials: 'same-origin',
    }));
    const [url, options] = fetchImpl.mock.calls[0];
    expect(url).not.toContain('abc');
    expect(JSON.parse(options.body)).toEqual(body);
  });

  test.each([
    [400, 'invalid'],
    [401, 'unauthorized'],
    [403, 'forbidden'],
    [404, 'not_found'],
    [503, 'unavailable'],
    [500, 'error'],
  ])('search maps HTTP %i to %s', async (status, outcome) => {
    const fetchImpl = jest.fn().mockResolvedValue(jsonResponse(status, { error: 'nope' }));
    await expect(searchLprReads({}, { fetchImpl })).resolves.toMatchObject({ outcome, message: 'nope' });
  });

  test('error messages are optional and tolerate non-JSON bodies', async () => {
    const plain = jest.fn().mockResolvedValue(new Response('Bad Gateway', { status: 502 }));
    await expect(searchLprReads({}, { fetchImpl: plain }))
      .resolves.toEqual({ outcome: 'error', status: 502, message: '' });
    const numeric = jest.fn().mockResolvedValue(jsonResponse(400, { error: 42 }));
    await expect(searchLprReads({}, { fetchImpl: numeric })).resolves.toEqual({ outcome: 'invalid', message: '' });
  });

  test('a success body without a reads array is treated as no reads', async () => {
    const fetchImpl = jest.fn().mockResolvedValue(jsonResponse(200, { count: 3 }));
    await expect(searchLprReads({}, { fetchImpl })).resolves.toEqual({ outcome: 'ok', reads: [] });
  });

  test('search passes the abort signal through', async () => {
    const controller = new AbortController();
    const fetchImpl = jest.fn().mockResolvedValue(jsonResponse(200, { reads: [] }));
    await searchLprReads({}, { fetchImpl, signal: controller.signal });
    expect(fetchImpl.mock.calls[0][1].signal).toBe(controller.signal);
  });

  test('calls use the global fetch by default', async () => {
    const original = global.fetch;
    global.fetch = jest.fn()
      .mockResolvedValueOnce(jsonResponse(200, { reads: [] }))
      .mockResolvedValueOnce(jsonResponse(503, {}))
      .mockResolvedValueOnce(jsonResponse(404, {}));
    try {
      await expect(searchLprReads({})).resolves.toEqual({ outcome: 'ok', reads: [] });
      await expect(exportLprReads({})).resolves.toMatchObject({ outcome: 'unavailable' });
      await expect(deleteLprRead('r1')).resolves.toMatchObject({ outcome: 'not_found' });
      expect(global.fetch.mock.calls.map(([url]) => url))
        .toEqual(['/api/lpr/search', '/api/lpr/export', '/api/lpr/reads/r1']);
    } finally {
      global.fetch = original;
    }
  });

  test('export returns the downloaded document as a blob', async () => {
    const fetchImpl = jest.fn().mockResolvedValue(new Response('{"reads":[]}', { status: 200 }));
    const result = await exportLprReads({ camera_uuid: CAMERA }, { fetchImpl });
    expect(result.outcome).toBe('ok');
    await expect(result.blob.text()).resolves.toBe('{"reads":[]}');
    expect(fetchImpl.mock.calls[0][0]).toBe('/api/lpr/export');
  });

  test('delete targets the read by id', async () => {
    const fetchImpl = jest.fn().mockResolvedValue(jsonResponse(200, { deleted: true }));
    await expect(deleteLprRead('r 1', { fetchImpl })).resolves.toEqual({ outcome: 'ok' });
    expect(fetchImpl).toHaveBeenCalledWith('/api/lpr/reads/r%201', expect.objectContaining({ method: 'DELETE' }));
  });
});

describe('permission-driven actions', () => {
  test('actions are offered until the server refuses them for that camera', async () => {
    let denied = {};
    expect(lprActionsFor(denied, CAMERA)).toEqual({ canSearch: true, canExport: true, canDelete: true });

    const fetchImpl = jest.fn().mockResolvedValue(jsonResponse(403, { error: 'Forbidden' }));
    const exportResult = await exportLprReads({ camera_uuid: CAMERA }, { fetchImpl });
    expect(exportResult.outcome).toBe('forbidden');
    denied = denyLprAction(denied, CAMERA, 'export');
    expect(lprActionsFor(denied, CAMERA)).toEqual({ canSearch: true, canExport: false, canDelete: true });

    const deleteResult = await deleteLprRead('r1', { fetchImpl });
    expect(deleteResult.outcome).toBe('forbidden');
    denied = denyLprAction(denied, CAMERA, 'delete');
    expect(lprActionsFor(denied, CAMERA)).toEqual({ canSearch: true, canExport: false, canDelete: false });

    // Denial is scoped to the camera that refused it.
    expect(lprActionsFor(denied, 'other-camera')).toEqual({ canSearch: true, canExport: true, canDelete: true });
  });
});

describe('recording deep link', () => {
  const observedAt = new Date(2026, 9, 28, 15, 31, 7).getTime();

  test('links unassociated reads by camera UUID despite a renamed stream', () => {
    const read = { camera_uuid: CAMERA, stream_name: 'Old name', observed_at: observedAt, plate: 'SECRET1' };
    const href = lprTimelineHref(read, [{ camera_uuid: CAMERA, name: 'Driveway' }]);
    expect(href).toBe(`timeline.html?camera_uuid=${CAMERA}&stream=Driveway&date=2026-10-28&time=15%3A31%3A07`);
    expect(href).not.toContain('SECRET1');

    const params = new URL(href, 'http://localhost').searchParams;
    const apiUrl = buildTimelineSegmentsUrl({
      cameraUuid: params.get('camera_uuid'),
      streamName: params.get('stream'),
      startTime: '2026-10-28T00:00:00Z',
      endTime: '2026-10-29T00:00:00Z',
    });
    expect(new URL(apiUrl, 'http://localhost').searchParams.get('camera_uuid')).toBe(CAMERA);
    expect(new URL(apiUrl, 'http://localhost').searchParams.has('stream')).toBe(false);
  });

  test.each([42, '42'])('prefers recording ID %p over renamed camera names', (recordingId) => {
    const href = lprTimelineHref({
      recording_id: recordingId,
      camera_uuid: CAMERA,
      stream_name: 'Old name',
      observed_at: observedAt,
      plate: 'SECRET1',
    }, [{ camera_uuid: CAMERA, name: 'Renamed camera' }]);
    expect(href).toBe('timeline.html?ids=42&date=2026-10-28&time=15%3A31%3A07');
    expect(href).not.toContain('SECRET1');
  });

  test.each([null, 0, -1, 'invalid', Number.MAX_SAFE_INTEGER + 1])(
    'falls back to the camera UUID for unusable recording ID %p', (recordingId) => {
      const href = lprTimelineHref({
        recording_id: recordingId, camera_uuid: CAMERA, observed_at: observedAt,
      });
      expect(new URL(href, 'http://localhost').searchParams.get('camera_uuid')).toBe(CAMERA);
      expect(href).not.toContain('ids=');
    },
  );

  test('supports an associated recording with no camera or stream name', () => {
    expect(lprTimelineHref({ recording_id: 42, observed_at: observedAt }))
      .toBe('timeline.html?ids=42&date=2026-10-28&time=15%3A31%3A07');
  });

  test('falls back to the recorded stream name and rejects reads without a time', () => {
    expect(lprTimelineHref({ camera_uuid: CAMERA, stream_name: 'Driveway', observed_at: observedAt }, []))
      .toContain('stream=Driveway');
    expect(lprTimelineHref({ stream_name: 'Driveway' }, [])).toBeNull();
  });

  test('supports UUID-only reads and retains the legacy name fallback', () => {
    expect(lprTimelineHref({ camera_uuid: CAMERA, stream_name: 'Driveway', observed_at: observedAt }, null))
      .toContain('stream=Driveway');
    expect(lprTimelineHref({ camera_uuid: CAMERA, observed_at: observedAt }, [{ name: 'No identity' }]))
      .toBe(`timeline.html?camera_uuid=${CAMERA}&date=2026-10-28&time=15%3A31%3A07`);
    expect(lprTimelineHref({ observed_at: observedAt })).toBeNull();
    expect(lprTimelineHref({ stream_name: 'Legacy camera', observed_at: observedAt }))
      .toBe('timeline.html?stream=Legacy+camera&date=2026-10-28&time=15%3A31%3A07');
    expect(lprTimelineHref({ camera_uuid: CAMERA, observed_at: String(observedAt), stream_name: 'Driveway' }))
      .toContain('time=15%3A31%3A07');
  });
});

test('confidence renders as a whole percentage', () => {
  expect(formatLprConfidence(0.876)).toBe('88%');
  expect(formatLprConfidence(null)).toBe('');
  expect(formatLprConfidence(0)).toBe('0%');
  expect(formatLprConfidence(1)).toBe('100%');
});

describe('defensive input handling', () => {
  test('invalid times format as empty and do not parse', () => {
    expect(toDateTimeLocalValue(NaN)).toBe('');
    expect(toDateTimeLocalValue(undefined)).toBe('');
    expect(parseDateTimeLocalValue('2026-10-28 15:00')).toBeNull();
    expect(parseDateTimeLocalValue(1793200000000)).toBeNull();
    expect(parseDateTimeLocalValue('2026-10-28T15:00:30')).toBe(new Date(2026, 9, 28, 15, 0, 30).getTime());
  });

  test('filters default to now', () => {
    jest.spyOn(Date, 'now').mockReturnValue(new Date(2026, 9, 28, 16, 0).getTime());
    try {
      expect(createLprFilters()).toMatchObject({ start: '2026-10-27T16:00', end: '2026-10-28T16:00' });
    } finally {
      jest.restoreAllMocks();
    }
  });

  test('missing filters report every required field instead of throwing', () => {
    expect(validateLprFilters(undefined).map((error) => error.field)).toEqual(['camera', 'start', 'end', 'limit']);
  });

  test('a missing plate is treated as empty', () => {
    const body = buildLprSearchBody({ ...filters(), plate: undefined });
    expect(body).not.toHaveProperty('plate');
    expect(validateLprFilters({ ...filters(), plate: undefined })).toEqual([]);
  });
});
