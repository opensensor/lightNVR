/**
 * Recordings page shapes: list response extensions, filter and pagination state,
 * batch delete and download payloads. Read from api_handlers_recordings_list.c,
 * api_handlers_recordings_backend_agnostic.c, api_handlers_recording_tags.c,
 * api_handlers_recordings_batch_download.c, and api_handlers_camera_collections.c.
 * Shared shapes stay in core.ts.
 */

import type { TranslationParams } from '../i18n.ts';
import type { BatchDeleteCounts, Recording, RecordingsPagination } from './core.ts';

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

/**
 * Item of the list as the page holds it. No handler emits `has_detections`
 * (they emit `has_detection`); the client sets it locally in loadRecordings()
 * and RecordingsView.
 */
export interface RecordingListItem extends Recording {
  has_detections?: boolean;
}

/** GET /api/recordings/:id. handle_get_recording() does not emit `tags`. */
export type RecordingDetail = Omit<Recording, 'tags'>;

/**
 * GET /api/recordings. The handler always emits `pagination` and never a top-level
 * `total`; RecordingsView keeps a fallback for responses without `pagination`, so
 * both stay optional here.
 */
export interface RecordingsListResponse {
  recordings: RecordingListItem[];
  pagination?: RecordingsPagination;
  total?: number;
}

/** GET /api/recordings/tags and GET /api/recordings/:id/tags. */
export interface RecordingTagsListResponse {
  tags?: string[];
}

/** GET /api/recordings/detection-labels. */
export interface DetectionLabelsResponse {
  labels?: string[];
}

/** PUT /api/recordings/:id/tags. */
export interface SetRecordingTagsResponse {
  id: number;
  tags: string[];
}

/** POST /api/recordings/batch-tags. */
export interface BatchTagsResponse {
  recordings_count: number;
  tags_added: number;
  tags_removed: number;
}

/** The fields of a /api/camera-collections item this page reads. */
export interface RecordingsCollectionOption {
  uuid: string;
  name: string;
  effective_count: number;
}

/** GET /api/camera-collections. */
export interface RecordingsCollectionsResponse {
  collections?: RecordingsCollectionOption[];
}

// ---------------------------------------------------------------------------
// Batch delete
// ---------------------------------------------------------------------------

/** buildFilterObject() in recordingsAPI.tsx; the `filter` of POST /api/recordings/batch-delete. */
export interface BatchDeleteFilter {
  start?: string;
  end?: string;
  collection_uuid?: string;
  stream?: string;
  detection?: number;
  detection_label?: string;
  tag?: string;
  capture_method?: string;
  protected?: number;
}

/** Body of POST /api/recordings/batch-delete; `totalCount` is only read by the modal. */
export interface BatchDeleteParams {
  ids?: Array<number | string>;
  filter?: BatchDeleteFilter;
  totalCount?: number;
}

/**
 * POST /api/recordings/batch-delete. The handler always emits `job_id` and `status`;
 * the client also accepts counts directly, so the counts stay in the type.
 */
export interface BatchDeleteStartResponse extends BatchDeleteCounts {
  job_id?: string;
  status?: string;
}

/** GET /api/recordings/batch-delete/progress/:token. */
export interface BatchDeleteProgressResponse extends BatchDeleteCounts {
  job_id?: string;
  status?: string;
  total?: number;
  current?: number;
  status_message?: string;
  error_message?: string;
  complete?: boolean;
}

// ---------------------------------------------------------------------------
// Batch download: api_handlers_recordings_batch_download.c
// ---------------------------------------------------------------------------

/** POST /api/recordings/batch-download. */
export interface BatchDownloadStartResponse {
  token: string;
  total: number;
}

/** GET /api/recordings/batch-download/status/:token. */
export interface BatchDownloadStatusResponse {
  status: string;
  current: number;
  total?: number;
  error?: string;
}

// ---------------------------------------------------------------------------
// Page state
// ---------------------------------------------------------------------------

export type RecordingTypeFilter = 'all' | 'detection' | 'no_detection';
export type ProtectedStatusFilter = 'all' | 'yes' | 'no';

export interface RecordingFilters {
  /** A preset name or 'custom'; read from the URL unvalidated, so any string. */
  dateRange: string;
  startDate: string;
  startTime: string;
  endDate: string;
  endTime: string;
  streamIds: string[];
  collectionUuid: string;
  recordingType: RecordingTypeFilter;
  detectionLabels: string[];
  tags: string[];
  captureMethods: string[];
  protectedStatus: ProtectedStatusFilter;
}

/** The array-valued filters, edited by the sidebar's add and remove helpers. */
export type MultiValueFilterKey = 'streamIds' | 'detectionLabels' | 'tags' | 'captureMethods';

export interface RecordingsPaginationState {
  currentPage: number;
  pageSize: number;
  showAll: boolean;
  totalItems: number;
  totalPages: number;
  startItem: number;
  endItem: number;
}

/** What urlUtils.getFiltersFromUrl() resolves the query string to. */
export interface RecordingsUrlState {
  filters: RecordingFilters;
  page: number;
  limit: number;
  showAll: boolean;
  sort: string;
  order: string;
}

export type ActiveFilterKey = keyof Pick<
  RecordingFilters,
  | 'dateRange'
  | 'streamIds'
  | 'collectionUuid'
  | 'recordingType'
  | 'detectionLabels'
  | 'tags'
  | 'captureMethods'
  | 'protectedStatus'
>;

export interface ActiveFilter {
  key: ActiveFilterKey;
  value?: string;
  label: string;
}

/** recording id to selected flag. Selected ids are the keys, as strings. */
export type RecordingSelection = Record<string, boolean>;

/** Accepted by both TagsOverlay (id and tags) and BulkTagsOverlay (no arguments). */
export type RecordingTagsChanged = (recordingId?: number, newTags?: string[]) => unknown;

/** The form of t() that recordings helpers and props take. */
export type TranslateFn = (key: string, params?: TranslationParams) => string;
