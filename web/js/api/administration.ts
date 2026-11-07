/** F3 wire boundary. Every accepted route selects its own canonical guard. */
import { enhancedFetch, type EnhancedFetchOptions } from '../fetch-utils.ts';
import type { components } from '../../generated/api/schema';
import * as guards from '../../generated/api/validators.js';
import { ContractValidationError } from './contractErrors.ts';
export type AdministrationOperation = 'listAdminUsers' | 'createAdminUser' | 'getAdminUser' | 'updateAdminUser' | 'deleteAdminUser' | 'createLegacyApiKey' | 'setUserPasswordLock' | 'clearUserLoginLockout' | 'listAuthorizationActions' | 'simulateAuthorization' | 'listAuthorizationRoles' | 'createAuthorizationRole' | 'updateAuthorizationRole' | 'deleteAuthorizationRole' | 'getUserAuthorizationPolicy' | 'setUserAuthorizationPolicy' | 'listAuditEvents' | 'exportAuditEvents' | 'getAuditSettings' | 'updateAuditSettings';
type RouteContract = { route: RegExp; method: string; operation: AdministrationOperation; schema: string; guard: (value: unknown) => boolean; request?: (value: unknown) => boolean; status: number };
const contracts: RouteContract[] = [
  {route: /^\/api\/auth\/users$/, method: 'GET', operation: 'listAdminUsers', schema: 'AdminUsers', guard: guards.validateAdminUsers, status: 200},
  {route: /^\/api\/auth\/users$/, method: 'POST', operation: 'createAdminUser', schema: 'AdminUser', guard: guards.validateAdminUser, request: guards.validateAdminUserCreate, status: 200},
  {route: /^\/api\/auth\/users\/([1-9][0-9]*)$/, method: 'GET', operation: 'getAdminUser', schema: 'AdminUser', guard: guards.validateAdminUser, status: 200},
  {route: /^\/api\/auth\/users\/([1-9][0-9]*)$/, method: 'PUT', operation: 'updateAdminUser', schema: 'AdminUser', guard: guards.validateAdminUser, request: guards.validateAdminUserUpdate, status: 200},
  {route: /^\/api\/auth\/users\/([1-9][0-9]*)$/, method: 'DELETE', operation: 'deleteAdminUser', schema: 'AdminUserDeleted', guard: guards.validateAdminUserDeleted, status: 200},
  {route: /^\/api\/auth\/users\/([1-9][0-9]*)\/api-key$/, method: 'POST', operation: 'createLegacyApiKey', schema: 'LegacyApiKeyCreated', guard: guards.validateLegacyApiKeyCreated, status: 200},
  {route: /^\/api\/auth\/users\/([1-9][0-9]*)\/password-lock$/, method: 'PUT', operation: 'setUserPasswordLock', schema: 'AdminUser', guard: guards.validateAdminUser, request: guards.validatePasswordLockRequest, status: 200},
  {route: /^\/api\/auth\/users\/([1-9][0-9]*)\/login-lockout\/clear$/, method: 'POST', operation: 'clearUserLoginLockout', schema: 'LoginLockoutCleared', guard: guards.validateLoginLockoutCleared, status: 200},
  {route: /^\/api\/authorization\/actions$/, method: 'GET', operation: 'listAuthorizationActions', schema: 'AdminActionCatalog', guard: guards.validateAdminActionCatalog, status: 200},
  {route: /^\/api\/authorization\/simulate$/, method: 'POST', operation: 'simulateAuthorization', schema: 'AdminSimulation', guard: guards.validateAdminSimulation, request: guards.validateAdminSimulationRequest, status: 200},
  {route: /^\/api\/authorization\/roles$/, method: 'GET', operation: 'listAuthorizationRoles', schema: 'AdminRoles', guard: guards.validateAdminRoles, status: 200},
  {route: /^\/api\/authorization\/roles$/, method: 'POST', operation: 'createAuthorizationRole', schema: 'AdminRoleWritten', guard: guards.validateAdminRoleWritten, request: guards.validateAdminRoleWrite, status: 201},
  {route: /^\/api\/authorization\/roles\/([0-9a-fA-F-]{36})$/, method: 'PUT', operation: 'updateAuthorizationRole', schema: 'AdminRoleWritten', guard: guards.validateAdminRoleWritten, request: guards.validateAdminRoleWrite, status: 200},
  {route: /^\/api\/authorization\/roles\/([0-9a-fA-F-]{36})$/, method: 'DELETE', operation: 'deleteAuthorizationRole', schema: 'AdminRoleDeleted', guard: guards.validateAdminRoleDeleted, request: guards.validateAdminVersionRequest, status: 200},
  {route: /^\/api\/authorization\/users\/([1-9][0-9]*)$/, method: 'GET', operation: 'getUserAuthorizationPolicy', schema: 'AdminUserPolicy', guard: guards.validateAdminUserPolicy, status: 200},
  {route: /^\/api\/authorization\/users\/([1-9][0-9]*)$/, method: 'PUT', operation: 'setUserAuthorizationPolicy', schema: 'AdminUserPolicy', guard: guards.validateAdminUserPolicy, request: guards.validateAdminPolicyWrite, status: 200},
  {route: /^\/api\/audit\/events$/, method: 'GET', operation: 'listAuditEvents', schema: 'AuditEventsPage', guard: guards.validateAuditEventsPage, status: 200},
  {route: /^\/api\/audit\/settings$/, method: 'GET', operation: 'getAuditSettings', schema: 'AuditSettings', guard: guards.validateAuditSettings, status: 200},
  {route: /^\/api\/audit\/settings$/, method: 'PUT', operation: 'updateAuditSettings', schema: 'AuditSettings', guard: guards.validateAuditSettings, request: guards.validateAuditSettingsWrite, status: 200},
 ];
export class AdministrationRequestError extends Error {
  readonly name = 'AdministrationRequestError';
  constructor(readonly operation: AdministrationOperation, readonly status: number | undefined, readonly mayHaveApplied: boolean) {
    super(`${operation} failed${status === undefined ? '' : ` (HTTP ${status})`}.`);
  }
}
async function fetchPrivate(url: string, operation: AdministrationOperation, options: EnhancedFetchOptions): Promise<Response> {
  const mutation = (options.method ?? 'GET') !== 'GET';
  try {
    return await enhancedFetch(url, { timeout: 15000, ...options, credentials: 'same-origin', redirect: 'error', cache: 'no-store', sensitive: true,
      retries: mutation ? 0 : (options.retries ?? 1) });
  } catch (error: unknown) {
    if (options.signal?.aborted || (error instanceof Error && error.name === 'AbortError')) throw new DOMException('Request was cancelled', 'AbortError');
    const status = typeof error === 'object' && error !== null && 'status' in error && typeof error.status === 'number' ? error.status : undefined;
    throw new AdministrationRequestError(operation, status, mutation && (status === undefined || status >= 500));
  }
}
/** The generic denotes the consumer's generated wire alias; never bypasses the route-specific validator. */
export async function administrationJSON<T = unknown>(url: string, options: EnhancedFetchOptions = {}): Promise<T> {
  const method = (options.method ?? 'GET').toUpperCase();
  const pathname = url.split('?')[0]!;
  const contract = contracts.find(candidate => candidate.method === method && candidate.route.test(pathname));
  if (!contract) throw new Error('Unsupported administration operation.');
  const id = pathname.match(/\/(?:auth|authorization)\/users\/([0-9]+)/)?.[1];
  if (id && !Number.isSafeInteger(Number(id))) throw new ContractValidationError(contract.operation, 'request', 'UserId');
  if (contract.request) {
    let body: unknown;
    try { body = typeof options.body === 'string' ? JSON.parse(options.body) : undefined; }
    catch { throw new ContractValidationError(contract.operation, 'request', contract.schema, undefined, 'json'); }
    if (!contract.request(body)) throw new ContractValidationError(contract.operation, 'request', contract.schema);
  }
  const response = await fetchPrivate(url, contract.operation, {...options, method});
  if (response.status !== contract.status) throw new ContractValidationError(contract.operation, 'response', contract.schema, response.status, 'status');
  if (!/^application\/json(?:\s*;|$)/i.test(response.headers.get('content-type') ?? '')) throw new ContractValidationError(contract.operation, 'response', contract.schema, response.status, 'json');
  let value: unknown;
  try { value = await response.json(); }
  catch { throw new ContractValidationError(contract.operation, 'response', contract.schema, response.status, 'json'); }
  if (!contract.guard(value)) throw new ContractValidationError(contract.operation, 'response', contract.schema, response.status);
  if (id && ((contract.schema === 'AdminUser' && (value as components['schemas']['AdminUser']).id !== Number(id)) ||
      (contract.schema === 'AdminUserPolicy' && (value as components['schemas']['AdminUserPolicy']).user_id !== Number(id)))) {
    throw new ContractValidationError(contract.operation, 'response', contract.schema, response.status);
  }
  return value as T;
}
export const auditCsvHeader = 'uuid,occurred_at,request_id,principal_user_id,principal_username,auth_method,api_token_uuid,action,target_type,target_uuid,outcome,remote_address,details_json\n';
export async function exportAuditEvents(query: string, options: EnhancedFetchOptions = {}): Promise<Blob> {
  const response = await fetchPrivate(`/api/audit/events/export?${query}`, 'exportAuditEvents', {...options, method:'GET', retries:0});
  if (response.status !== 200 || !/^text\/csv(?:\s*;|$)/i.test(response.headers.get('content-type') ?? '') ||
      response.headers.get('content-disposition') !== 'attachment; filename=lightnvr-audit.csv' ||
      !/^[0-9]+$/.test(response.headers.get('x-total-count') ?? '')) throw new ContractValidationError('exportAuditEvents', 'response', 'AuditCsv', response.status, 'status');
  const text = await response.text();
  if (!text.startsWith(auditCsvHeader)) throw new ContractValidationError('exportAuditEvents', 'response', 'AuditCsv', response.status);
  return new Blob([text], {type:'text/csv;charset=utf-8'});
}

/** CRUD and password locks are separate native operations. Never retry either write.
 * A lock failure after saving the user is a partial result, not a failed creation. */
export class AdministrationPartialSaveError extends Error {
  readonly mayHaveApplied = true;
  constructor() {super('User details were saved, but the password lock was not confirmed. Refresh this user before trying again.');}
}
export async function saveAdminUser(url: string, payload: {password_change_locked: boolean} & Record<string, unknown>, options: EnhancedFetchOptions): Promise<components['schemas']['AdminUser']> {
  const {password_change_locked: locked, ...body} = payload;
  const saved = await administrationJSON<components['schemas']['AdminUser']>(url, {...options, body:JSON.stringify(body)});
  if (saved.password_change_locked === locked) return saved;
  try {
    return await administrationJSON<components['schemas']['AdminUser']>(`/api/auth/users/${saved.id}/password-lock`, {...options, method:'PUT', body:JSON.stringify({locked})});
  } catch {
    throw new AdministrationPartialSaveError();
  }
}
