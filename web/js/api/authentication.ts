/** Authentication wire boundary. No retries, raw diagnostics, or credential logging. */
import type { components } from '../../generated/api/schema';
import {
  validateLoginRequest, validateLoginResponse, validateLoginMfaChallenge, validateLoginSuccess,
  validateLoginConfig, validateLogoutResponse, validateTotpLoginRequest, validateTotpLoginResponse,
  validateTotpSetupResponse, validateTotpStatusResponse, validateTotpCodeRequest,
  validateTotpChangedResponse, validatePasswordChangeRequest, validatePasswordChangedResponse,
} from '../../generated/api/validators.js';
import type { ContractValidator } from '../../generated/api/validators.js';
import { ContractValidationError } from './contractErrors.ts';
export type AuthenticationOperation = 'login' | 'getLoginConfig' | 'logout' | 'completeTotpLogin' |
  'setupTotp' | 'getTotpStatus' | 'verifyTotp' | 'disableTotp' | 'changePassword';
type Schema = components['schemas'];
export type LoginRequest = Schema['LoginRequest'];
export interface AuthenticationOptions { signal?: AbortSignal; headers?: HeadersInit }
export class AuthenticationRequestError extends Error {
  readonly name = 'AuthenticationRequestError';
  constructor(readonly operation: AuthenticationOperation, readonly status?: number,
    readonly mayHaveApplied = false) {
    super(`${operation} failed${status === undefined ? '' : ` (HTTP ${status})`}.`);
  }
}
function requestBody<T>(value: unknown, guard: ContractValidator<T>, operation: AuthenticationOperation, schema: string): string {
  if (!guard(value)) throw new ContractValidationError(operation, 'request', schema);
  return JSON.stringify(value);
}
function userPath(id: number, operation: AuthenticationOperation): string {
  if (!Number.isSafeInteger(id) || id <= 0) throw new ContractValidationError(operation, 'request', 'UserId');
  return `/api/auth/users/${id}`;
}
async function request<T>(url: string, operation: AuthenticationOperation, schema: string,
  guard: ContractValidator<T>, options: AuthenticationOptions, method = 'GET', body?: string): Promise<T> {
  const controller = new AbortController();
  const abort = () => controller.abort();
  if (options.signal?.aborted) throw new DOMException('Request cancelled', 'AbortError');
  options.signal?.addEventListener('abort', abort, { once: true });
  const timeout = setTimeout(abort, 10000);
  try {
    const headers = new Headers(options.headers);
    headers.set('Accept', 'application/json');
    if (body !== undefined) headers.set('Content-Type', 'application/json');
    const response = await fetch(url, { method, body, headers, signal: controller.signal, credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
    if (!response.ok) throw new AuthenticationRequestError(operation, response.status, method !== 'GET' && response.status >= 500);
    if (response.status !== 200) throw new ContractValidationError(operation, 'response', schema, response.status, 'status');
    let value: unknown;
    try { value = await response.json(); }
    catch { throw new ContractValidationError(operation, 'response', schema, response.status, 'json'); }
    if (controller.signal.aborted) throw new AuthenticationRequestError(operation, undefined, method !== 'GET');
    if (!guard(value)) throw new ContractValidationError(operation, 'response', schema, response.status);
    return value;
  } catch (error: unknown) {
    if (options.signal?.aborted) throw new DOMException('Request cancelled', 'AbortError');
    if (error instanceof AuthenticationRequestError || error instanceof ContractValidationError) throw error;
    throw new AuthenticationRequestError(operation, undefined, method !== 'GET');
  } finally {
    clearTimeout(timeout);
    options.signal?.removeEventListener('abort', abort);
  }
}
export const getLoginConfig = (options: AuthenticationOptions = {}) =>
  request('/api/auth/login/config', 'getLoginConfig', 'LoginConfig', validateLoginConfig, options);
export const logout = (options: AuthenticationOptions = {}) =>
  request('/api/auth/logout', 'logout', 'LogoutResponse', validateLogoutResponse, options, 'POST');
export async function login(body: LoginRequest, options: AuthenticationOptions = {}): Promise<
  { kind: 'challenge'; token: string } | { kind: 'authenticated'; mustChangePassword: boolean }> {
  const data = await request('/api/auth/login', 'login', 'LoginResponse', validateLoginResponse, options,
    'POST', requestBody(body, validateLoginRequest, 'login', 'LoginRequest'));
  if (validateLoginMfaChallenge(data)) return { kind: 'challenge', token: data.totp_token };
  if (validateLoginSuccess(data)) return { kind: 'authenticated', mustChangePassword: data.must_change_password };
  throw new ContractValidationError('login', 'response', 'LoginResponse');
}
export const completeTotpLogin = (body: Schema['TotpLoginRequest'], options: AuthenticationOptions = {}) =>
  request('/api/auth/login/totp', 'completeTotpLogin', 'TotpLoginResponse', validateTotpLoginResponse, options,
    'POST', requestBody(body, validateTotpLoginRequest, 'completeTotpLogin', 'TotpLoginRequest'));
export const getTotpStatus = (id: number, options: AuthenticationOptions = {}) =>
  request(`${userPath(id, 'getTotpStatus')}/totp/status`, 'getTotpStatus', 'TotpStatusResponse', validateTotpStatusResponse, options);
export async function setupTotp(id: number, options: AuthenticationOptions = {}) {
  const data = await request(`${userPath(id, 'setupTotp')}/totp/setup`, 'setupTotp', 'TotpSetupResponse', validateTotpSetupResponse, options, 'POST');
  const uri = new URL(data.otpauth_uri);
  if (uri.protocol !== 'otpauth:' || uri.hostname !== 'totp' || uri.searchParams.getAll('secret').length !== 1 || uri.searchParams.get('secret') !== data.secret)
    throw new ContractValidationError('setupTotp', 'response', 'TotpSetupResponse');
  return data;
}
export const verifyTotp = (id: number, code: string, options: AuthenticationOptions = {}) =>
  request(`${userPath(id, 'verifyTotp')}/totp/verify`, 'verifyTotp', 'TotpChangedResponse', validateTotpChangedResponse, options,
    'POST', requestBody({code}, validateTotpCodeRequest, 'verifyTotp', 'TotpCodeRequest'));
export const disableTotp = (id: number, options: AuthenticationOptions = {}) =>
  request(`${userPath(id, 'disableTotp')}/totp/disable`, 'disableTotp', 'TotpChangedResponse', validateTotpChangedResponse, options, 'POST');
export const changePassword = (id: number, body: Schema['PasswordChangeRequest'], options: AuthenticationOptions = {}) =>
  request(`${userPath(id, 'changePassword')}/password`, 'changePassword', 'PasswordChangedResponse', validatePasswordChangedResponse, options,
    'PUT', requestBody(body, validatePasswordChangeRequest, 'changePassword', 'PasswordChangeRequest'));
