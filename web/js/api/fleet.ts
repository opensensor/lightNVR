/** Validated fleet reads. Query/preview POSTs are side-effect-free; no mutation endpoints here. */
import {enhancedFetch} from '../fetch-utils.ts';
import type {EnhancedFetchOptions} from '../fetch-utils.ts';
import type {ContractValidator} from '../../generated/api/validators.js';
import {
  validateFleetQueryRequest,validateFleetPreviewRequest,validateFleetQueryResponse,validateFleetPreviewResponse,
  validateFleetSavedView,validateFleetSavedViewsResponse,validateFleetLocation,validateFleetLocationsResponse,
  validateFleetTag,validateFleetTagsResponse,validateCameraTagAssignmentsResponse,validateCameraCollection,
  validateCameraCollectionsResponse,validateCameraCollectionMembersResponse,validateCameraCollectionPreview,
} from '../../generated/api/validators.js';
import {ContractValidationError} from './contractErrors.ts';
export type FleetOperation = 'queryFleetCameras' | 'previewFleetSelector' | 'listFleetViews' | 'getFleetView' |
  'listCameraCollections' | 'getCameraCollection' | 'getCameraCollectionMembers' | 'previewCameraCollection' |
  'listFleetLocations' | 'getFleetLocation' | 'listFleetTags' | 'getFleetTag' | 'getCameraTagAssignments';
export type FleetRequestOptions = Omit<EnhancedFetchOptions, 'method' | 'body' | 'sensitive'>;
export class FleetRequestError extends Error {
  readonly name = 'FleetRequestError';
  constructor(readonly operation: FleetOperation, readonly status?: number) {
    super(`${operation} failed${status === undefined ? '' : ` (HTTP ${status})`}.`);
  }
}
function uuidPath(uuid: string, operation: FleetOperation): string {
  if (!/^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/.test(uuid))
    throw new ContractValidationError(operation, 'request', 'CameraUuid');
  return uuid;
}
function bodyJson(value: unknown, guard: ContractValidator<unknown>, operation: FleetOperation, schema: string): string {
  if (!guard(value)) throw new ContractValidationError(operation, 'request', schema);
  return JSON.stringify(value);
}
async function read<T>(url: string, operation: FleetOperation, schema: string, guard: ContractValidator<T>,
  options: FleetRequestOptions, method = 'GET', body?: string): Promise<T> {
  let response: Response;
  try {
    const headers = new Headers(options.headers);
    if (body !== undefined) headers.set('Content-Type', 'application/json');
    response = await enhancedFetch(url, {timeout: 20000, retries: 1, ...options, method, body,
      cache: 'no-store', sensitive: true, headers, credentials: 'same-origin', redirect: 'error'});
  } catch (error: unknown) {
    if (options.signal?.aborted || (error instanceof Error && error.name === 'AbortError')) throw new DOMException('Request cancelled', 'AbortError');
    const status = typeof error === 'object' && error !== null && 'status' in error && typeof error.status === 'number' ? error.status : undefined;
    throw new FleetRequestError(operation, status);
  }
  if (response.status !== 200) throw new ContractValidationError(operation, 'response', schema, response.status, 'status');
  let value: unknown;
  try {value = await response.json();}
  catch {
    if (options.signal?.aborted) throw new DOMException('Request cancelled', 'AbortError');
    throw new ContractValidationError(operation, 'response', schema, response.status, 'json');
  }
  if (options.signal?.aborted) throw new DOMException('Request cancelled', 'AbortError');
  if (!guard(value)) throw new ContractValidationError(operation, 'response', schema, response.status);
  return value;
}
export const queryFleetCameras = (body: unknown, options: FleetRequestOptions = {}) =>
  read('/api/fleet/cameras/query','queryFleetCameras','FleetQueryResponse',validateFleetQueryResponse,options,'POST',bodyJson(body,validateFleetQueryRequest,'queryFleetCameras','FleetQueryRequest'));
export const previewFleetSelector = (body: unknown, options: FleetRequestOptions = {}) =>
  read('/api/fleet/selectors/preview','previewFleetSelector','FleetPreviewResponse',validateFleetPreviewResponse,options,'POST',bodyJson(body,validateFleetPreviewRequest,'previewFleetSelector','FleetPreviewRequest'));
export const listFleetViews = (options: FleetRequestOptions = {}) => read('/api/fleet/views','listFleetViews','FleetSavedViewsResponse',validateFleetSavedViewsResponse,options);
export const getFleetView = (uuid: string, options: FleetRequestOptions = {}) => read(`/api/fleet/views/${uuidPath(uuid,'getFleetView')}`,'getFleetView','FleetSavedView',validateFleetSavedView,options);
export const listCameraCollections = (options: FleetRequestOptions = {}) => read('/api/camera-collections','listCameraCollections','CameraCollectionsResponse',validateCameraCollectionsResponse,options);
export const getCameraCollection = (uuid: string, options: FleetRequestOptions = {}) => read(`/api/camera-collections/${uuidPath(uuid,'getCameraCollection')}`,'getCameraCollection','CameraCollection',validateCameraCollection,options);
export const getCameraCollectionMembers = (uuid: string, options: FleetRequestOptions = {}) => read(`/api/camera-collections/${uuidPath(uuid,'getCameraCollectionMembers')}/members`,'getCameraCollectionMembers','CameraCollectionMembersResponse',validateCameraCollectionMembersResponse,options);
export const previewCameraCollection = (uuid: string, options: FleetRequestOptions = {}) => read(`/api/camera-collections/${uuidPath(uuid,'previewCameraCollection')}/preview`,'previewCameraCollection','CameraCollectionPreview',validateCameraCollectionPreview,options,'POST');
export const listFleetLocations = (options: FleetRequestOptions = {}) => read('/api/locations','listFleetLocations','FleetLocationsResponse',validateFleetLocationsResponse,options);
export const getFleetLocation = (uuid: string, options: FleetRequestOptions = {}) => read(`/api/locations/${uuidPath(uuid,'getFleetLocation')}`,'getFleetLocation','FleetLocation',validateFleetLocation,options);
export const listFleetTags = (options: FleetRequestOptions = {}) => read('/api/camera-tags','listFleetTags','FleetTagsResponse',validateFleetTagsResponse,options);
export const getFleetTag = (uuid: string, options: FleetRequestOptions = {}) => read(`/api/camera-tags/${uuidPath(uuid,'getFleetTag')}`,'getFleetTag','FleetTag',validateFleetTag,options);
export const getCameraTagAssignments = (uuid: string, options: FleetRequestOptions = {}) => read(`/api/cameras/${uuidPath(uuid,'getCameraTagAssignments')}/tags`,'getCameraTagAssignments','CameraTagAssignmentsResponse',validateCameraTagAssignmentsResponse,options);
