/** Personal credential lifecycle. Wire bodies are private and always validated. */
import { enhancedFetch } from '../fetch-utils.ts';
import type { EnhancedFetchOptions } from '../fetch-utils.ts';
import type { components } from '../../generated/api/schema';
import { validateAuthSessionsResponse, validateTrustedDevicesResponse, validateCredentialRevoked } from '../../generated/api/validators.js';
import type { ContractValidator } from '../../generated/api/validators.js';
import { ContractValidationError } from './contractErrors.ts';
export type IdentityOperation = 'listAuthSessions' | 'listTrustedDevices' | 'revokeAuthSession' | 'revokeTrustedDevice';
export type IdentityRequestOptions = Omit<EnhancedFetchOptions, 'method' | 'body' | 'sensitive'>;
export type AuthSessionsResponse = components['schemas']['AuthSessionsResponse'];
export type TrustedDevicesResponse = components['schemas']['TrustedDevicesResponse'];
export type CredentialRevoked = components['schemas']['CredentialRevoked'];

export class IdentityRequestError extends Error {
  readonly name = 'IdentityRequestError';
  constructor(readonly operation: IdentityOperation, readonly status: number | undefined,
    readonly mayHaveApplied: boolean) {
    super(`${operation} failed${status === undefined ? '' : ` (HTTP ${status})`}.`);
  }
}

async function request<T>(url: string, operation: IdentityOperation, schema: string,
  guard: ContractValidator<T>, options: IdentityRequestOptions, mutation = false): Promise<T> {
  let response: Response;
  try {
    response = await enhancedFetch(url, { timeout: 10000, ...options, method: mutation ? 'DELETE' : 'GET',
      credentials: 'same-origin', redirect: 'error', cache: 'no-store', sensitive: true, retries: mutation ? 0 : (options.retries ?? 1) });
  } catch (error: unknown) {
    if (options.signal?.aborted || (error instanceof Error && error.name === 'AbortError')) throw new DOMException('Request was cancelled', 'AbortError');
    const status = typeof error === 'object' && error !== null && 'status' in error && typeof error.status === 'number' ? error.status : undefined;
    throw new IdentityRequestError(operation, status, mutation && (status === undefined || status >= 500));
  }
  if (response.status !== 200) throw new ContractValidationError(operation, 'response', schema, response.status, 'status');
  if (!/^application\/json(?:\s*;|$)/i.test(response.headers.get('content-type') ?? ''))
    throw new ContractValidationError(operation, 'response', schema, response.status, 'json');
  let value: unknown;
  try { value = await response.json(); }
  catch (error: unknown) {
    if (options.signal?.aborted || (error instanceof Error && error.name === 'AbortError')) throw new DOMException('Request was cancelled', 'AbortError');
    throw new ContractValidationError(operation, 'response', schema, response.status, 'json');
  }
  if (!guard(value)) throw new ContractValidationError(operation, 'response', schema, response.status);
  return value;
}
function idPath(id: number, operation: IdentityOperation): string {
  if (!Number.isSafeInteger(id) || id <= 0) throw new ContractValidationError(operation, 'request', 'CredentialId');
  return String(id);
}
export function listAuthSessions(options: IdentityRequestOptions = {}): Promise<AuthSessionsResponse> {
  return request('/api/auth/sessions', 'listAuthSessions', 'AuthSessionsResponse', validateAuthSessionsResponse, options);
}
export function listTrustedDevices(options: IdentityRequestOptions = {}): Promise<TrustedDevicesResponse> {
  return request('/api/auth/trusted-devices', 'listTrustedDevices', 'TrustedDevicesResponse', validateTrustedDevicesResponse, options);
}
export async function revokeAuthSession(id: number, options: IdentityRequestOptions = {}): Promise<CredentialRevoked> {
  return request(`/api/auth/sessions/${idPath(id, 'revokeAuthSession')}`, 'revokeAuthSession', 'CredentialRevoked', validateCredentialRevoked, options, true);
}
export async function revokeTrustedDevice(id: number, options: IdentityRequestOptions = {}): Promise<CredentialRevoked> {
  return request(`/api/auth/trusted-devices/${idPath(id, 'revokeTrustedDevice')}`, 'revokeTrustedDevice', 'CredentialRevoked', validateCredentialRevoked, options, true);
}
