import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import {
  ContractValidationError, StreamRequestError, getStreamFull,
  listStreams, listStreamSummaries, updateStream,
} from '../../js/api/streams.ts';
import { enhancedFetch, fetchJSON } from '../../js/fetch-utils.ts';
import {
  validateApiError, validateStreamCompatibilityList, validateStreamFullResponse,
  validateStreamPrivacyUpdated, validateStreamSummaryPage,
} from '../../generated/api/validators.js';

const baseline: { cases: Record<string, { status: number; body: unknown }> } =
  JSON.parse(readFileSync(resolve(__dirname, '../../../tests/contracts/fixtures/baseline-payloads.json'), 'utf8'));
const empty = { streams: [], total: 0, page: 1, page_size: 50, total_pages: 0 };
const secret = 'rtsp://sensitive-user:sensitive-password@private-host.invalid/path';
function fixture(name: string): unknown {
  const item = baseline.cases[name];
  if (!item) throw new Error(`Missing test fixture: ${name}`);
  return structuredClone(item.body);
}
function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
function object(value: unknown): Record<string, unknown> {
  if (!isObject(value)) throw new Error('Test fixture is not an object');
  return value;
}
function firstRow(value: unknown): Record<string, unknown> {
  const rows = object(value).streams;
  if (!Array.isArray(rows)) throw new Error('Test fixture has no rows');
  return object(rows[0]);
}
function respond(value: unknown, status = 200): jest.SpyInstance {
  return jest.spyOn(global, 'fetch').mockResolvedValue(new Response(JSON.stringify(value), {
    status, headers: { 'Content-Type': 'application/json' },
  }));
}
function expectNoLogs(): void {
  for (const method of ['log', 'debug', 'warn', 'error'] as const) expect(console[method]).not.toHaveBeenCalled();
}

beforeEach(() => {
  for (const method of ['log', 'debug', 'warn', 'error'] as const) jest.spyOn(console, method).mockImplementation(() => {});
});
afterEach(() => jest.restoreAllMocks());

test('generated guards accept all twelve independently captured baseline payloads', () => {
  expect(Object.keys(baseline.cases)).toHaveLength(12);
  for (const [name, item] of Object.entries(baseline.cases)) {
    const guard = item.status >= 400 ? validateApiError
      : name.includes('summary') ? validateStreamSummaryPage
      : name.includes('full') ? validateStreamFullResponse
      : name === 'admin-update' ? validateStreamPrivacyUpdated : validateStreamCompatibilityList;
    expect(guard(item.body)).toBe(true);
  }
});

test.each(['admin-summary', 'viewer-summary'])('summary adapter accepts %s and preserves additive fields', async name => {
  const value = fixture(name);
  firstRow(value).future_field = { retained: true };
  const fetchMock = respond(value);
  expect(await listStreamSummaries({ surface: 'admin', include_admin_url: 'true', page: 2, page_size: 10, search: 'front & rear' })).toEqual(value);
  const url = new URL(String(fetchMock.mock.calls[0]?.[0]), 'https://fixture.invalid');
  expect(url.searchParams.get('summary')).toBe('true');
  expect(url.searchParams.get('search')).toBe('front & rear');
  expect(url.searchParams.get('page')).toBe('2');
  expectNoLogs();
});

test('empty summary and compatibility lists remain valid', async () => {
  respond(empty);
  expect(await listStreamSummaries()).toEqual(empty);
  respond([]);
  expect(await listStreams()).toEqual([]);
});

test('live and LPR summaries use their requested surface guard', async () => {
  const live = fixture('viewer-summary');
  const row = firstRow(live);
  delete row.admin_url;
  Object.assign(row, {
    streaming_enabled: false, playback_transport: 'auto', eptz_config: '', detection_model: '',
    privacy_mode: false, can_control_privacy: false, has_sub_stream: false,
    ptz_enabled: false, backchannel_enabled: false, go2rtc_hls_available: false,
  });
  respond(live);
  expect(await listStreamSummaries({ surface: 'live' })).toEqual(live);
  delete row.privacy_mode;
  respond(live);
  await expect(listStreamSummaries({ surface: 'live' })).rejects.toMatchObject({ schema: 'LiveStreamSummaryPage', status: 200 });

  const lpr = { ...empty, streams: [{ name: 'Identity', camera_uuid: 'synthetic' }], total: 1, total_pages: 1 };
  respond(lpr);
  expect(await listStreamSummaries({ surface: 'lpr' })).toEqual(lpr);
  respond(lpr);
  await expect(listStreamSummaries({ surface: 'admin' })).rejects.toBeInstanceOf(ContractValidationError);
});

test.each(['admin-full', 'viewer-full'])('full adapter validates %s without losing wrapper or redaction', async name => {
  const value = fixture(name);
  object(object(value).stream).future_field = 'retained';
  const fetchMock = respond(value);
  expect(await getStreamFull('Front & rear')).toEqual(value);
  expect(fetchMock).toHaveBeenCalledWith('/api/streams/Front%20%26%20rear/full', expect.objectContaining({ method: 'GET' }));
});

test('compatibility adapter accepts captured array, rejects an envelope', async () => {
  const value = fixture('admin-list');
  respond(value);
  expect(await listStreams()).toEqual(value);
  respond(empty);
  await expect(listStreams()).rejects.toMatchObject({ schema: 'StreamCompatibilityList' });
});

test('required types and forbidden redacted secrets are rejected without logging response data', async () => {
  const summary = fixture('admin-summary');
  firstRow(summary).enabled = 'false';
  firstRow(summary).url = secret;
  respond(summary);
  await expect(listStreamSummaries()).rejects.toMatchObject({
    name: 'ContractValidationError', message: 'Invalid listStreamSummaries response: StreamSummaryPage (shape).', status: 200,
  });
  const full = fixture('viewer-full');
  object(object(full).stream).onvif_password = secret;
  respond(full);
  await expect(getStreamFull('Sensitive')).rejects.toMatchObject({ schema: 'StreamFullResponse', issue: 'shape' });
  respond({ stream: { name: 'partial' } });
  await expect(getStreamFull('Sensitive')).rejects.toBeInstanceOf(ContractValidationError);
  expectNoLogs();
});

test('invalid JSON and unexpected success status are sanitized', async () => {
  jest.spyOn(global, 'fetch').mockResolvedValue(new Response(secret, { status: 200 }));
  await expect(getStreamFull(secret)).rejects.toMatchObject({ issue: 'json', status: 200,
    message: 'Invalid getStreamFull response: StreamFullResponse (json).' });
  respond(empty, 201);
  await expect(listStreamSummaries()).rejects.toMatchObject({ issue: 'status', status: 201 });
  expectNoLogs();
});

test('ordinary update validates and preserves DTO fields, omissions, zeros and null clear operations', async () => {
  const dto = { enabled: false, retention_days: 0, detection_threshold: 0, tags: null, onvif_password: secret, future_field: 7 };
  const before = structuredClone(dto);
  const ack = { success: true, message: 'Accepted', future_field: 'retained' };
  const fetchMock = respond(ack, 202);
  expect(await updateStream('Front & rear', dto, { headers: { Authorization: 'Bearer synthetic-private' } })).toEqual({ status: 202, data: ack });
  expect(dto).toEqual(before);
  const options = fetchMock.mock.calls[0]?.[1];
  expect(options).toMatchObject({ method: 'PUT', body: JSON.stringify(dto) });
  expect(options).not.toHaveProperty('sensitive');
  expect(options).not.toHaveProperty('retries');
  expect(new Headers(options?.headers).get('Content-Type')).toBe('application/json');
  expect(new Headers(options?.headers).get('Authorization')).toBe('Bearer synthetic-private');
  expectNoLogs();
});

test('runtime-invalid update fails before transport without exposing the body', async () => {
  const fetchMock = jest.spyOn(global, 'fetch');
  // Invalid combination is expressible in TS; runtime schema supplies this invariant.
  await expect(updateStream('A', { enabled: false, enable_disabled: true, onvif_password: secret })).rejects.toMatchObject({
    name: 'ContractValidationError', phase: 'request', schema: 'StreamUpdateRequest',
  });
  expect(fetchMock).not.toHaveBeenCalled();
  expectNoLogs();
});

test('non-JSON additive request data fails before transport with a sanitized error', async () => {
  const body: { enabled: boolean; future_field?: unknown } = { enabled: true };
  body.future_field = body;
  const fetchMock = jest.spyOn(global, 'fetch');
  await expect(updateStream('A', body)).rejects.toMatchObject({
    phase: 'request', issue: 'json', mayHaveApplied: false,
    message: 'Invalid updateStream request: StreamUpdateRequest (json).',
  });
  expect(fetchMock).not.toHaveBeenCalled();
  expectNoLogs();
});

test('privacy and ordinary acknowledgements cannot be mistaken for each other', async () => {
  const privacy = { success: true, privacy_mode: true, stream_registered: false };
  respond(privacy);
  expect(await updateStream('A', { set_privacy_mode: true })).toEqual({ status: 200, data: privacy });
  respond(privacy);
  await expect(updateStream('A', { enabled: true })).rejects.toMatchObject({ issue: 'status', status: 200 });
  respond({ success: true, message: 'Accepted' }, 202);
  await expect(updateStream('A', { set_privacy_mode: false })).rejects.toMatchObject({ issue: 'status', status: 202 });
  respond({ success: true }, 202);
  await expect(updateStream('A', { enabled: true })).rejects.toMatchObject({ issue: 'shape', status: 202 });
});

test('privacy 502 preserves status, communicates possible saved state, and does not retry by default', async () => {
  const fetchMock = respond({ error: secret }, 502);
  await expect(updateStream('A', { set_privacy_mode: true })).rejects.toMatchObject({
    name: 'StreamRequestError', status: 502, mayHaveApplied: true,
    message: 'Privacy mode may have been saved; refresh stream state before retrying.',
  });
  expect(fetchMock).toHaveBeenCalledTimes(1);
  expectNoLogs();
});

test('explicit retry override survives while HTTP and network error details are sanitized', async () => {
  const fetchMock = respond({ error: secret }, 500);
  await expect(updateStream('A', { url: secret }, { retries: 1, retryDelay: 0 })).rejects.toMatchObject({ status: 500, mayHaveApplied: true });
  expect(fetchMock).toHaveBeenCalledTimes(2);
  fetchMock.mockRejectedValue(new Error(secret));
  await expect(getStreamFull(secret, { retries: 0 })).rejects.toMatchObject({
    name: 'StreamRequestError', kind: 'network', message: 'getStreamFull failed.',
  });
  expectNoLogs();
});

test.each([400, 401, 403, 404])('HTTP %i status survives without response text', async status => {
  respond({ error: secret }, status);
  await expect(getStreamFull(secret, { skipAuthRedirect: true })).rejects.toMatchObject({
    status, message: `getStreamFull failed (HTTP ${status}).`,
  });
  expectNoLogs();
});

test('pre-aborted signal prevents transport and stays recognizable as cancellation', async () => {
  const controller = new AbortController(); controller.abort();
  const fetchMock = jest.spyOn(global, 'fetch');
  await expect(getStreamFull('A', { signal: controller.signal })).rejects.toMatchObject({ name: 'AbortError' });
  expect(fetchMock).not.toHaveBeenCalled();
});

test('in-flight abort preserves the native AbortError and never retries', async () => {
  const controller = new AbortController();
  const aborted = new DOMException('The operation was aborted.', 'AbortError');
  const fetchMock = jest.spyOn(global, 'fetch').mockImplementation((_url, options) => new Promise((_resolve, reject) => {
    options?.signal?.addEventListener('abort', () => reject(aborted), { once: true });
  }));
  const result = getStreamFull('A', { signal: controller.signal });
  controller.abort();
  await expect(result).rejects.toBe(aborted);
  expect(fetchMock).toHaveBeenCalledTimes(1);
  expectNoLogs();
});

test('body-reader cancellation is not misreported as a malformed contract', async () => {
  const response = new Response('{}');
  const aborted = new DOMException('The operation was aborted.', 'AbortError');
  jest.spyOn(response, 'json').mockRejectedValue(aborted);
  jest.spyOn(global, 'fetch').mockResolvedValue(response);
  await expect(getStreamFull('A')).rejects.toBe(aborted);
});

test('mutation timeout retains timeout information and never retries a possibly applied write', async () => {
  jest.useFakeTimers();
  try {
    const fetchMock = jest.spyOn(global, 'fetch').mockImplementation((_url, options) => new Promise((_resolve, reject) => {
      options?.signal?.addEventListener('abort', () => reject(new DOMException('Aborted', 'AbortError')), { once: true });
    }));
    const result = expect(updateStream('A', { enabled: true }, { timeout: 100, retries: 2 })).rejects.toMatchObject({
      kind: 'timeout', mayHaveApplied: true, message: 'updateStream failed: request timed out.',
    });
    await jest.runAllTimersAsync();
    await result;
    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(jest.getTimerCount()).toBe(0);
    expectNoLogs();
  } finally {
    jest.useRealTimers();
  }
});

test('sensitive option is consumed by transport, suppresses JSON errors and leaves default logging intact', async () => {
  const fetchMock = respond({}, 200);
  await fetchJSON('/api/synthetic', { sensitive: true, headers: { Authorization: secret } });
  expect(fetchMock.mock.calls[0]?.[1]).not.toHaveProperty('sensitive');
  expectNoLogs();
  respond({ error: secret }, 500);
  await expect(fetchJSON('/api/synthetic', { sensitive: true, retries: 0 })).rejects.toHaveProperty('status', 500);
  expectNoLogs();
  respond({});
  await enhancedFetch('/api/synthetic');
  expect(console.log).toHaveBeenCalled();
});

test('generated validators contain standalone runtime helpers, no browser schema compiler imports', () => {
  const generated = readFileSync(resolve(__dirname, '../../generated/api/validators.js'), 'utf8');
  // Standalone helpers retain a .code string mentioning require(); it is not
  // an import or the schema compiler. The producer bundles those helpers.
  expect(/^\s*import\s/m.test(generated)).toBe(false);
  expect(/node_modules\/ajv\/dist\/compile\/|new\s+Ajv/.test(generated)).toBe(false);
  expect(new StreamRequestError('updateStream', 400, 'http').mayHaveApplied).toBe(false);
});

test.each([307,308] as const)('credential-bearing update refuses HTTP %s without forwarding payload',async status=>{
  const {withRedirectFixture}=await import('./redirectHarness');
  await withRedirectFixture(status,async fixture=>{
    const spy=jest.spyOn(global,'fetch').mockImplementation(fixture.fetch);
    await expect(updateStream('Owned',{isOnvif:true,onvif_username:'owned-user',onvif_password:'owned-password'},
      {redirect:'follow',credentials:'include'})).rejects.toBeInstanceOf(Error);
    expect(fixture.sourceBodies).toHaveLength(1);
    expect(fixture.forwardedBodies).toEqual([]);
    expect(spy.mock.calls[0]?.[1]).toMatchObject({redirect:'error',credentials:'same-origin'});
  });
});
