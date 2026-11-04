/**
 * Shapes for the investigation workspace and license plate views, read from
 * src/web/api_handlers_investigations.c, api_handlers_investigation_bookmarks.c,
 * and api_handlers_lpr.c. Filter and region state are the views' own.
 */
import type { t as translate, TranslationParams } from '../i18n.ts';

export type Translate = typeof translate;

// --- Timeline ---------------------------------------------------------------

/**
 * POST /api/investigations/timeline segment and segment-at's `segment`. Aggregated
 * tracks return coverage-only intervals, which omit `capture_method`.
 */
export interface InvestigationSegment {
  id: number;
  start_time: number;
  end_time: number;
  duration?: number;
  capture_method?: string;
  has_detection?: boolean;
  size_bytes?: number;
  protected?: boolean;
  media_available?: boolean;
  source_segment_count?: number;
  coverage_only?: boolean;
}

export interface InvestigationTrackCoverage {
  identity_resolved?: boolean;
  requested_start?: number;
  requested_end?: number;
}

export interface InvestigationTimelineTrack {
  camera_uuid: string;
  name: string;
  stream_name?: string;
  segment_count: number;
  returned_interval_count?: number;
  aggregated: boolean;
  aggregation_bucket_seconds?: number;
  truncated: boolean;
  coverage?: InvestigationTrackCoverage;
  segments: InvestigationSegment[];
}

export interface InvestigationTimeline {
  start_time: number;
  end_time: number;
  camera_count?: number;
  max_active_decoders: number;
  tracks: InvestigationTimelineTrack[];
}

export interface InvestigationSegmentAtResponse {
  segment?: InvestigationSegment | null;
}

// --- Search -----------------------------------------------------------------

export interface InvestigationDetection {
  id?: number;
  label: string;
  confidence: number;
  source: string;
  zone_uuid: string | null;
  track_id?: number | null;
  bounding_box?: {
    x: number;
    y: number;
    width: number;
    height: number;
    normalized?: boolean;
  } | null;
}

export interface InvestigationResult {
  result_id: string;
  event_type: string;
  camera_uuid: string;
  start_time: number;
  end_time: number;
  media_available?: boolean;
  known_gap: boolean;
  recording_id: number | null;
  camera: {
    name: string;
    location_uuid?: string | null;
    location_name?: string | null;
    location_path?: string | null;
  };
  detection: InvestigationDetection;
  thumbnail: { status: string; url?: string };
  recording: { capture_method: string | null; protected: boolean | null };
}

export interface InvestigationFacet {
  value: string;
  label?: string;
  count: number;
}

export interface InvestigationFacets {
  cameras?: InvestigationFacet[];
  labels?: InvestigationFacet[];
  zones?: InvestigationFacet[];
  sources?: InvestigationFacet[];
  event_types?: InvestigationFacet[];
  locations?: InvestigationFacet[];
  capture_methods?: InvestigationFacet[];
  recording_tags?: InvestigationFacet[];
  protection?: InvestigationFacet[];
}

export interface InvestigationHistogramBucket {
  start_time: number;
  end_time: number;
  event_time?: number;
  count: number;
}

export interface InvestigationHistogramData {
  bucket_seconds?: number;
  buckets: InvestigationHistogramBucket[];
}

export interface InvestigationSearchCoverage {
  requested_start?: number;
  requested_end?: number;
  searched_start?: number;
  searched_end?: number;
  unresolved_legacy_rows: number;
  /** The region fields appear only when the request had a region. */
  spatial_metadata: {
    requested: boolean;
    rows_with_boxes?: number;
    rows_without_boxes?: number;
  };
  incomplete_reasons?: string[];
  complete?: boolean;
}

export interface InvestigationSearchPage {
  limit?: number;
  returned?: number;
  total: number;
  has_more: boolean;
  next_cursor: string | null;
}

export interface InvestigationSearchResponse {
  camera_count?: number;
  selector_applied?: boolean;
  summary_complete?: boolean;
  results: InvestigationResult[];
  page: InvestigationSearchPage;
  facets: InvestigationFacets;
  histogram: InvestigationHistogramData;
  coverage: InvestigationSearchCoverage;
}

export interface InvestigationSearchRequestFilters {
  event_types?: string[];
  locations?: string[];
  labels?: string[];
  zones?: string[];
  sources?: string[];
  capture_methods?: string[];
  recording_tags?: string[];
  protected?: boolean;
  min_confidence?: number;
}

export interface InvestigationSearchContext {
  cameraUuids: string[];
  startTime: number;
  endTime: number;
}

// --- Region and filter state ------------------------------------------------

export type InvestigationRegionMatch = 'center' | 'intersects' | 'minimum_intersection';

export interface RegionPoint {
  x: number;
  y: number;
}

export interface RegionRectangle extends RegionPoint {
  width: number;
  height: number;
}

export type RegionCoordinateName = keyof RegionRectangle;

export interface InvestigationRegion extends RegionRectangle {
  cameraUuid: string;
  match: InvestigationRegionMatch;
  minIntersection: number;
}

/** The view's filter state. Every text field holds its control's raw value. */
export interface InvestigationSearchFilters {
  eventType: string;
  location: string;
  label: string;
  zone: string;
  source: string;
  captureMethod: string;
  recordingTag: string;
  protection: string;
  minConfidence: string;
  region: InvestigationRegion | null;
}

/** What bookmarkFiltersFromState tolerates: a partial state and a numeric confidence. */
export type BookmarkFilterInput = Partial<Omit<InvestigationSearchFilters, 'minConfidence'>> & {
  minConfidence?: string | number | null;
};

// --- Bookmarks --------------------------------------------------------------

export interface BookmarkRegion extends RegionRectangle {
  camera_uuid: string;
  match?: InvestigationRegionMatch;
  min_intersection?: number;
}

export interface BookmarkFilters {
  event_type?: string;
  location?: string;
  label?: string;
  zone?: string;
  source?: string;
  capture_method?: string;
  recording_tag?: string;
  protection?: string;
  min_confidence?: number;
  region?: BookmarkRegion;
}

/**
 * The keys bookmarkRepresentativeResult copies when they hold a string or a finite
 * number. `detection_id` and `label` are not on a search result (see the task log).
 */
export interface RepresentativeResultSource {
  result_id?: unknown;
  camera_uuid?: unknown;
  start_time?: unknown;
  end_time?: unknown;
  event_type?: unknown;
  recording_id?: unknown;
  detection_id?: unknown;
  label?: unknown;
}

export type BookmarkRepresentativeResult = Partial<
  Record<keyof RepresentativeResultSource, string | number>
>;

export interface InvestigationBookmarkPayload {
  title: string;
  note: string;
  camera_uuids: string[];
  start_time: number;
  end_time: number;
  cursor_time: number;
  primary_camera_uuid: string;
  filters: BookmarkFilters;
  representative_result: BookmarkRepresentativeResult | null;
}

/** GET /api/investigation-bookmarks item (bookmark_to_json). */
export interface InvestigationBookmark {
  uuid: string;
  title: string;
  note: string;
  start_time: number;
  end_time: number;
  cursor_time: number;
  primary_camera_uuid: string;
  camera_uuids: string[];
  filters?: BookmarkFilters;
  representative_result?: BookmarkRepresentativeResult | null;
  revision: number;
  created_at?: number;
  updated_at?: number;
  holds_recordings?: boolean;
}

export interface InvestigationBookmarksResponse {
  bookmarks?: InvestigationBookmark[];
  count?: number;
}

// --- Actions ----------------------------------------------------------------

export interface InvestigationActionSelection {
  cameraUuids: string[];
  startTime: number;
  endTime: number;
}

export interface InvestigationPreviewRecording {
  id: number;
  camera_uuid?: string;
  camera_name: string;
  stream_name?: string;
  start_time: number;
  end_time: number;
  size_bytes: number;
  protected: boolean;
  can_protect: boolean;
  can_export: boolean;
}

/** POST /api/investigations/recordings/preview. */
export interface InvestigationPreview {
  start_time: number;
  end_time: number;
  camera_count?: number;
  recording_count: number;
  total_bytes: number;
  protected_count: number;
  unprotected_count?: number;
  recordings: InvestigationPreviewRecording[];
}

export interface BatchProtectResponse {
  success_count?: number;
  fail_count?: number;
  protected?: boolean;
  message?: string;
}

export interface BatchDownloadJob {
  token: string;
  total?: number;
}

export interface BatchDownloadStatus {
  status: 'pending' | 'running' | 'complete' | 'error';
  current?: number;
  total?: number;
  error?: string;
}

// --- Thumbnails -------------------------------------------------------------

export interface ThumbnailWindow {
  startTime: number;
  endTime: number;
}

export interface ThumbnailDrilldown extends ThumbnailWindow {
  cameraUuid: string;
  history: ThumbnailWindow[];
}

export interface InvestigationThumbnailSample {
  timestamp: number;
  media_status: string;
  thumbnail?: { status: string; url?: string };
  recording_id?: number;
  recording_start_time?: number;
  recording_end_time?: number;
  offset_ms?: number;
}

/** POST /api/investigations/thumbnail-samples. */
export interface InvestigationThumbnailData {
  start_time?: number;
  end_time?: number;
  sample_count?: number;
  camera?: { camera_uuid?: string; name: string };
  samples: InvestigationThumbnailSample[];
  coverage?: {
    segments_truncated?: boolean;
    available_samples?: number;
    gap_samples?: number;
  };
}

// --- License plates ---------------------------------------------------------

/**
 * In-memory plate filters. `match` is a string, not a union, because
 * validateLprFilters exists to reject other values.
 */
export interface LprFilters {
  cameraUuid: string;
  start: string;
  end: string;
  plate: string;
  match: string;
  limit: string;
}

export type LprField = 'camera' | 'start' | 'end' | 'limit' | 'match' | 'plate';

export interface LprValidationError {
  field: LprField;
  key: string;
  params?: TranslationParams;
}

export interface LprSearchBody {
  camera_uuid: string;
  start_at: number | null;
  end_at: number | null;
  limit: number;
  match?: 'exact' | 'partial';
  plate?: string;
}

/** read_to_json in api_handlers_lpr.c. */
export interface LprAttributes {
  country: string | null;
  region: string | null;
  plate_type: string | null;
  direction: string | null;
  lane: string | null;
  vehicle_type: string | null;
  vehicle_color: string | null;
  object_id: string | null;
  correlation_id: string | null;
  vendor_topic?: string;
  bounding_box?: { left: number; top: number; right: number; bottom: number } | null;
}

export interface LprRead {
  id: string;
  type?: string;
  camera_uuid: string;
  stream_name: string;
  /** Epoch milliseconds. */
  observed_at: number;
  received_at?: number;
  source?: string;
  plate: string;
  /** Normalized to 0..1. */
  confidence: number | null;
  attributes?: LprAttributes;
  recording_id?: number | null;
}

export type LprFailure =
  | {
    outcome: 'invalid' | 'unauthorized' | 'forbidden' | 'not_found' | 'unavailable';
    message: string;
  }
  | { outcome: 'error'; status: number; message: string };

export type LprResponseOutcome = { outcome: 'ok' } | LprFailure;
export type LprSearchOutcome = { outcome: 'ok'; reads: LprRead[] } | LprFailure;
export type LprExportOutcome = { outcome: 'ok'; blob: Blob } | LprFailure;

export interface LprRequestOptions {
  fetchImpl?: typeof fetch;
  signal?: AbortSignal;
}

export type LprAction = 'search' | 'export' | 'delete';

/** Actions the server refused, by camera UUID. */
export type LprDeniedActions = Partial<Record<string, Partial<Record<LprAction, boolean>>>>;
