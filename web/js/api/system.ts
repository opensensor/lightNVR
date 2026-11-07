/** Validated system and workspace operations. Never retry administrative writes. */
import {enhancedFetch} from '../fetch-utils.ts';
import type {EnhancedFetchOptions} from '../fetch-utils.ts';
import type {ContractValidator} from '../../generated/api/validators.js';
import {ContractValidationError} from './contractErrors.ts';
import {validateStreamHealthHistory,validateGo2rtcConfigurationInput,validateGo2rtcConfigurationValidation,validateGo2rtcEffectiveConfiguration,validateGo2rtcOverrideDiagnostics,validateHostHealth,validateHostHealthIncidents,validateSetupStatus,validateSetupUpdate,validateSetupUpdated,validateStreamHealth,validateSystemActionAccepted,validateSystemBackupCreated,validateSystemInformation,validateSystemLogs,validateSystemSettings,validateSystemSettingsSaved,validateSystemSettingsUpdate,validateSystemStatus,validateWorkspacesResponse,validateWorkspacesUpdate} from '../../generated/api/validators.js';
export type SystemOperation = 'getSystemSettings' | 'getSystemInformation' | 'getSystemStatus' | 'getHostHealth' | 'getHostHealthIncidents' | 'getSystemLogs' | 'getGo2rtcEffectiveConfiguration' | 'getGo2rtcOverrideDiagnostics' | 'getWorkspaces' | 'getSetupStatus' | 'getStreamHealth' | 'updateSystemSettings' | 'validateGo2rtcConfiguration' | 'updateWorkspaces' | 'updateSetupStatus' | 'clearSystemLogs' | 'createSystemBackup' | 'restartSystem' | 'shutdownSystem';
export type SystemRequestOptions = Omit<EnhancedFetchOptions, 'method' | 'body' | 'sensitive'>;
export class SystemRequestError extends Error {
  readonly name = 'SystemRequestError';
  constructor(readonly operation: SystemOperation, readonly status?: number, readonly mayHaveApplied = false) {
    super(`${operation} failed${status === undefined ? '' : ` (HTTP ${status})`}.${mayHaveApplied ? ' Refresh state before retrying.' : ''}`);
  }
}
function bodyJson(body: unknown, guard: ContractValidator<unknown>, operation: SystemOperation, schema: string): string {
  if (!guard(body)) throw new ContractValidationError(operation, 'request', schema);
  return JSON.stringify(body);
}
function invalidResponse(operation: SystemOperation, schema: string, status: number, issue: 'shape' | 'json' | 'status' = 'shape') {
  const error = new ContractValidationError(operation, 'response', schema, status, issue);
  if (error.mayHaveApplied) error.message += ' The change may have applied; refresh state before retrying.';
  return error;
}
async function request<T>(url: string, operation: SystemOperation, schema: string, guard: ContractValidator<T>,
  options: SystemRequestOptions, method = 'GET', body?: string): Promise<T> {
  const mutation = method !== 'GET' && operation !== 'validateGo2rtcConfiguration';
  let response: Response;
  try {
    const headers = new Headers(options.headers);
    if (body !== undefined) headers.set('Content-Type', 'application/json');
    response = await enhancedFetch(url, {timeout: 20000, ...options, method, body,
      retries: method === 'GET' ? (options.retries ?? 1) : 0, cache: 'no-store', credentials: 'same-origin', redirect: 'error', sensitive: true, headers});
  } catch (error: unknown) {
    if (options.signal?.aborted || (error instanceof Error && error.name === 'AbortError')) throw new DOMException('Request cancelled', 'AbortError');
    const status = typeof error === 'object' && error !== null && 'status' in error && typeof error.status === 'number' ? error.status : undefined;
    throw new SystemRequestError(operation, status, mutation && (status === undefined || status >= 500));
  }
  if (response.status !== 200) throw invalidResponse(operation, schema, response.status, 'status');
  if (!/^application\/json(?:\s*;|$)/i.test(response.headers.get('content-type') ?? ''))
    throw invalidResponse(operation, schema, response.status, 'json');
  let value: unknown;
  try {value = await response.json();}
  catch {
    if (options.signal?.aborted) throw new DOMException('Request cancelled', 'AbortError');
    throw invalidResponse(operation, schema, response.status, 'json');
  }
  if (options.signal?.aborted) throw new DOMException('Request cancelled', 'AbortError');
  if (!guard(value)) throw invalidResponse(operation, schema, response.status);
  return value;
}
export const getSystemSettings = (options: SystemRequestOptions = {}) => request('/api/settings','getSystemSettings','SystemSettings',validateSystemSettings,options);
export const getSystemInformation = (options: SystemRequestOptions = {}) => request('/api/system/info','getSystemInformation','SystemInformation',validateSystemInformation,options);
export const getSystemStatus = (options: SystemRequestOptions = {}) => request('/api/system/status','getSystemStatus','SystemStatus',validateSystemStatus,options);
export const getHostHealth = (options: SystemRequestOptions = {}) => request('/api/system/health','getHostHealth','HostHealth',validateHostHealth,options);
export const getHostHealthIncidents = (query: {limit?:number;include_closed?:boolean;cursor?:string} = {}, options: SystemRequestOptions = {}) => {
  const params = new URLSearchParams();
  for (const [key,value] of Object.entries(query)) if (value !== undefined) params.set(key,String(value));
  return request(`/api/system/health/incidents?${params}`,'getHostHealthIncidents','HostHealthIncidents',validateHostHealthIncidents,options);
};
export const getSystemLogs = (query: {level?:string;count?:number;last_ts?:string} = {}, options: SystemRequestOptions = {}) => {
  const params = new URLSearchParams();
  for (const [key,value] of Object.entries(query)) if (value !== undefined) params.set(key,String(value));
  return request(`/api/system/logs?${params}`,'getSystemLogs','SystemLogs',validateSystemLogs,options);
};
export const getGo2rtcEffectiveConfiguration = (options: SystemRequestOptions = {}) => request('/api/system/go2rtc/effective-config','getGo2rtcEffectiveConfiguration','Go2rtcEffectiveConfiguration',validateGo2rtcEffectiveConfiguration,options);
export const getGo2rtcOverrideDiagnostics = (options: SystemRequestOptions = {}) => request('/api/system/go2rtc/override-status','getGo2rtcOverrideDiagnostics','Go2rtcOverrideDiagnostics',validateGo2rtcOverrideDiagnostics,options);
export const getWorkspaces = (options: SystemRequestOptions = {}) => request('/api/ui/workspaces','getWorkspaces','WorkspacesResponse',validateWorkspacesResponse,options);
export const getSetupStatus = (options: SystemRequestOptions = {}) => request('/api/setup/status','getSetupStatus','SetupStatus',validateSetupStatus,options);
export const getStreamHealth = (options: SystemRequestOptions = {}) => request('/api/health','getStreamHealth','StreamHealth',validateStreamHealth,options);
export const updateSystemSettings = (body: unknown, options: SystemRequestOptions = {}) => request('/api/settings','updateSystemSettings','SystemSettingsSaved',validateSystemSettingsSaved,options,'POST',bodyJson(body,validateSystemSettingsUpdate,'updateSystemSettings','SystemSettingsUpdate'));
export const validateGo2rtcConfiguration = (body: unknown, options: SystemRequestOptions = {}) => request('/api/settings/go2rtc/validate','validateGo2rtcConfiguration','Go2rtcConfigurationValidation',validateGo2rtcConfigurationValidation,options,'POST',bodyJson(body,validateGo2rtcConfigurationInput,'validateGo2rtcConfiguration','Go2rtcConfigurationInput'));
export const updateWorkspaces = (body: unknown, options: SystemRequestOptions = {}) => request('/api/ui/workspaces','updateWorkspaces','WorkspacesResponse',validateWorkspacesResponse,options,'PUT',bodyJson(body,validateWorkspacesUpdate,'updateWorkspaces','WorkspacesUpdate'));
export const updateSetupStatus = (body: unknown, options: SystemRequestOptions = {}) => request('/api/setup/status','updateSetupStatus','SetupUpdated',validateSetupUpdated,options,'POST',bodyJson(body,validateSetupUpdate,'updateSetupStatus','SetupUpdate'));
export const clearSystemLogs = (options: SystemRequestOptions = {}) => request('/api/system/logs/clear','clearSystemLogs','SystemActionAccepted',validateSystemActionAccepted,options,'POST');
export const createSystemBackup = (options: SystemRequestOptions = {}) => request('/api/system/backup','createSystemBackup','SystemBackupCreated',validateSystemBackupCreated,options,'POST');
export const restartSystem = (options: SystemRequestOptions = {}) => request('/api/system/restart','restartSystem','SystemActionAccepted',validateSystemActionAccepted,options,'POST');
export const shutdownSystem = (options: SystemRequestOptions = {}) => request('/api/system/shutdown','shutdownSystem','SystemActionAccepted',validateSystemActionAccepted,options,'POST');

export const getStreamHealthHistory = (options: SystemRequestOptions = {}) => request('/api/health?sparklines=true&history_only=true','getStreamHealth','StreamHealthHistory',validateStreamHealthHistory,options);
