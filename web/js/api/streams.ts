import { enhancedFetch } from '../fetch-utils.ts';
import type { EnhancedFetchOptions } from '../fetch-utils.ts';
import type { components, operations } from '../../generated/api/schema';
import {
  validateLiveStreamSummaryPage, validateLprStreamSummaryPage,
  validateStreamCompatibilityList, validateStreamFullResponse,
  validateStreamPrivacyUpdated, validateStreamSummaryPage,
  validateStreamUpdateAccepted, validateStreamUpdateRequest,
} from '../../generated/api/validators.js';
import type { ContractValidator } from '../../generated/api/validators.js';
import { ContractValidationError, StreamRequestError } from './contractErrors.ts';
import type { StreamOperation } from './contractErrors.ts';

export { ContractValidationError, StreamRequestError } from './contractErrors.ts';
export type StreamSummaryPage = components['schemas']['StreamSummaryPage'];
export type LiveStreamSummaryPage = components['schemas']['LiveStreamSummaryPage'];
export type LprStreamSummaryPage = components['schemas']['LprStreamSummaryPage'];
export type StreamCompatibilityList = components['schemas']['StreamCompatibilityList'];
export type StreamFullResponse = components['schemas']['StreamFullResponse'];
export type StreamUpdateRequest = components['schemas']['StreamUpdateRequest'];
export type StreamUpdateAccepted = components['schemas']['StreamUpdateAccepted'];
export type StreamPrivacyUpdated = components['schemas']['StreamPrivacyUpdated'];
export type StreamUpdateResult =
  | { status: 202; data: StreamUpdateAccepted }
  | { status: 200; data: StreamPrivacyUpdated };
export type StreamSummaryQuery = Omit<NonNullable<operations['listStreams']['parameters']['query']>, 'summary'>;
export type StreamRequestOptions = Omit<EnhancedFetchOptions, 'method' | 'body' | 'sensitive'>;

function validate<T>(value: unknown, guard: ContractValidator<T>, operation: StreamOperation,
  schema: string, phase: 'request' | 'response', status?: number): T {
  if (!guard(value)) throw new ContractValidationError(operation, phase, schema, status);
  return value;
}

async function request(url: string, operation: StreamOperation, options: EnhancedFetchOptions,
  privacy = false): Promise<Response> {
  try {
    return await enhancedFetch(url, { ...options, redirect: 'error', credentials: 'same-origin', sensitive: true });
  } catch (error: unknown) {
    // Keep native body/transport cancellation recognizable to query consumers.
    if (typeof error === 'object' && error !== null && 'name' in error && error.name === 'AbortError') throw error;
    if (options.signal?.aborted) throw new DOMException('Request was cancelled', 'AbortError');
    const status = typeof error === 'object' && error !== null && 'status' in error &&
      typeof error.status === 'number' ? error.status : undefined;
    const kind = status !== undefined ? 'http'
      : error instanceof Error && error.message === 'Request timed out' ? 'timeout' : 'network';
    throw new StreamRequestError(operation, status, kind,
      operation === 'updateStream' && (status === undefined || status >= 500), privacy);
  }
}

async function json(response: Response, operation: StreamOperation, schema: string): Promise<unknown> {
  try {
    const value: unknown = await response.json();
    return value;
  } catch (error: unknown) {
    if (typeof error === 'object' && error !== null && 'name' in error && error.name === 'AbortError') throw error;
    throw new ContractValidationError(operation, 'response', schema, response.status, 'json');
  }
}

async function read<T>(url: string, operation: StreamOperation, schema: string,
  guard: ContractValidator<T>, options: StreamRequestOptions): Promise<T> {
  const response = await request(url, operation, { ...options, method: 'GET' });
  if (response.status !== 200) throw new ContractValidationError(operation, 'response', schema, response.status, 'status');
  return validate(await json(response, operation, schema), guard, operation, schema, 'response', response.status);
}

/** Compatibility array, including the authorized empty array. */
export function listStreams(options: StreamRequestOptions = {}): Promise<StreamCompatibilityList> {
  return read('/api/streams', 'listStreams', 'StreamCompatibilityList', validateStreamCompatibilityList, options);
}

export function listStreamSummaries(query: StreamSummaryQuery & { surface: 'live' }, options?: StreamRequestOptions): Promise<LiveStreamSummaryPage>;
export function listStreamSummaries(query: StreamSummaryQuery & { surface: 'lpr' }, options?: StreamRequestOptions): Promise<LprStreamSummaryPage>;
export function listStreamSummaries(query?: StreamSummaryQuery & { surface?: 'admin' }, options?: StreamRequestOptions): Promise<StreamSummaryPage>;
export function listStreamSummaries(query: StreamSummaryQuery, options?: StreamRequestOptions): Promise<StreamSummaryPage | LiveStreamSummaryPage | LprStreamSummaryPage>;
export function listStreamSummaries(query: StreamSummaryQuery = {}, options: StreamRequestOptions = {}): Promise<StreamSummaryPage | LiveStreamSummaryPage | LprStreamSummaryPage> {
  const params = new URLSearchParams({ summary: 'true' });
  for (const [key, value] of Object.entries(query)) {
    if (value !== undefined) params.set(key, String(value));
  }
  // Pin the mode even if untyped callers pass an extra summary parameter.
  params.set('summary', 'true');
  const url = `/api/streams?${params.toString()}`;
  if (query.surface === 'live') return read(url, 'listStreamSummaries', 'LiveStreamSummaryPage', validateLiveStreamSummaryPage, options);
  if (query.surface === 'lpr') return read(url, 'listStreamSummaries', 'LprStreamSummaryPage', validateLprStreamSummaryPage, options);
  return read(url, 'listStreamSummaries', 'StreamSummaryPage', validateStreamSummaryPage, options);
}

/** Full read keeps its wire wrapper and permission-redacted fields. */
export function getStreamFull(name: string, options: StreamRequestOptions = {}): Promise<StreamFullResponse> {
  return read(`/api/streams/${encodeURIComponent(name)}/full`, 'getStreamFull', 'StreamFullResponse', validateStreamFullResponse, options);
}

/** Ordinary updates are queued (202); privacy changes complete synchronously (200).
 * A failed privacy request can already have saved the flag. Neither errors nor
 * acknowledgements prove rollback or durable application; refresh current state.
 */
export async function updateStream(name: string, body: StreamUpdateRequest,
  options: StreamRequestOptions = {}): Promise<StreamUpdateResult> {
  validate(body, validateStreamUpdateRequest, 'updateStream', 'StreamUpdateRequest', 'request');
  const privacy = typeof body.set_privacy_mode === 'boolean';
  const schema = privacy ? 'StreamPrivacyUpdated' : 'StreamUpdateAccepted';
  const headers = new Headers(options.headers);
  headers.set('Content-Type', 'application/json');
  let serialized: string;
  try {
    serialized = JSON.stringify(body);
  } catch {
    throw new ContractValidationError('updateStream', 'request', 'StreamUpdateRequest', undefined, 'json');
  }
  const response = await request(`/api/streams/${encodeURIComponent(name)}`, 'updateStream', {
    retries: 0, ...options, method: 'PUT', headers, body: serialized,
  }, privacy);
  if (response.status !== (privacy ? 200 : 202)) {
    throw new ContractValidationError('updateStream', 'response', schema, response.status, 'status');
  }
  const value = await json(response, 'updateStream', schema);
  if (privacy) return { status: 200, data: validate(value, validateStreamPrivacyUpdated, 'updateStream', schema, 'response', response.status) };
  return { status: 202, data: validate(value, validateStreamUpdateAccepted, 'updateStream', schema, 'response', response.status) };
}
