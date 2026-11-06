import type { components } from '../../generated/api/schema';
/**
 * Shapes for the fleet explorer, camera collections, saved views, and event
 * routing, read from src/web/api_handlers_fleet.c, _fleet_views.c,
 * _camera_collections.c, _locations.c, _camera_tags.c, _event_destinations.c,
 * and _event_routes.c. Fields the handlers do not always emit are optional.
 */
import type { EnhancedFetchOptions } from '../fetch-utils.ts';
import type { t as translate } from '../i18n.ts';
import type { FormNumber } from './streams.ts';

export type Translate = typeof translate;

/** What fetchJSON rejects with: an HTTPError carries status, other failures do not. */
export type RequestError = Error & { status?: number };

/** fetchJSON's signature, for helpers that take the fetcher as an argument. */
export type JsonFetcher = <T = unknown>(url: string, options?: EnhancedFetchOptions) => Promise<T>;

// ---------------------------------------------------------------------------
// Selectors
// ---------------------------------------------------------------------------

/** An alias, not an interface, so it stays assignable to `{ op: string; [key: string]: unknown }`. */
export type FleetSelectorExpression = {
  op: string;
  uuid?: string;
  uuids?: string[];
  values?: string[];
  value?: boolean;
  children?: FleetSelectorExpression[];
};

export interface FleetSelector {
  version: number;
  expression: FleetSelectorExpression;
}

/**
 * A selector that has not been validated: stored data, or FleetView's fallback
 * queues, which carry `health` and no expression.
 */
export interface FleetSelectorInput {
  version?: number;
  expression?: FleetSelectorExpression | null;
  health?: string[];
}

/** What the selector builder takes to start from: stored data it parses and may reject. */
export interface FleetSelectorSeed {
  version?: number;
  expression?: unknown;
}

/** One row of the visual collection builder. `type` is an expression op. */
export interface CollectionRule {
  type: string;
  uuid?: string;
  value?: boolean;
  values?: string[];
}

export interface CollectionSelectorParseResult {
  supported: boolean;
  match: string;
  rules: CollectionRule[];
}

export interface CollectionSelectorValidation {
  selector: FleetSelector | null;
  error: string;
}

export type CollectionSelectorChange = (selector: FleetSelector | null, error: string) => unknown;

// ---------------------------------------------------------------------------
// Fleet explorer
// ---------------------------------------------------------------------------

export type FleetSortOrder = 'asc' | 'desc';

/** Filter, sort, and page state, mirrored into the URL. */
export interface FleetState {
  search: string;
  health: string[];
  /** 'all', 'true', or 'false'. */
  enabled: string;
  availability: string;
  recordingModes: string[];
  tagUuids: string[];
  locationUuid: string;
  collectionUuid: string;
  page: number;
  pageSize: number;
  sortBy: string;
  sortOrder: FleetSortOrder;
}

export type FleetStateChange = (changes: Partial<FleetState>, resetPage?: boolean) => void;

export interface FleetQueryRequest {
  selector: FleetSelector;
  search: string;
  page: number;
  page_size: number;
  sort_by: string;
  sort_order: FleetSortOrder;
  facets: boolean;
  explain: boolean;
  availability: string;
  collection_uuid?: string;
}

export interface FleetCameraTag {
  uuid: string;
  label: string;
}

export interface FleetCameraLocation {
  uuid: string;
  name: string;
  path: string;
}

export type FleetCamera = components['schemas']['FleetCamera'];

export interface FleetFacetValue {
  value: string | boolean;
  count: number;
}

export type FleetFacetEntity = components['schemas']['FleetFacetEntity'];

export type FleetFacets = components['schemas']['FleetFacets'];

export type FleetValueFacetGroup = 'health' | 'enabled' | 'recording_mode' | 'availability';

export type FleetOperationalQueue = components['schemas']['FleetOperationalQueue'];

export type FleetQueryResponse = components['schemas']['FleetQueryResponse'];

// ---------------------------------------------------------------------------
// Saved views
// ---------------------------------------------------------------------------

export type FleetSavedView = components['schemas']['FleetSavedView'];

export type FleetSavedViewsResponse = components['schemas']['FleetSavedViewsResponse'];

/** The part of a saved view that fleetStateFromSavedView reads. */
export interface FleetSavedViewInput {
  selector?: FleetSelectorInput | null;
  search?: string;
  collection_uuid?: string | null;
  sort_by?: string;
  sort_order?: string;
}

export interface FleetSavedViewPayload {
  name: string;
  is_shared: boolean;
  selector: FleetSelector;
  search: string;
  collection_uuid: string;
  columns: string[];
  sort_by: string;
  sort_order: FleetSortOrder;
}

// ---------------------------------------------------------------------------
// Locations and tags
// ---------------------------------------------------------------------------

/** All that the location helpers read, so other domains can pass their own rows. */
export interface LocationLike {
  uuid: string;
  name: string;
  parent_uuid?: string | null;
  sort_order?: number;
}

export type FleetLocation = components['schemas']['FleetLocation'];

export type LocationRow<T extends LocationLike = FleetLocation> = T & { depth: number; path: string };

export type FleetLocationsResponse = components['schemas']['FleetLocationsResponse'];

/** All that the selector builder reads from a tag. */
export interface TagLike {
  uuid: string;
  label: string;
}

export type FleetTag = components['schemas']['FleetTag'];

export type FleetTagsResponse = components['schemas']['FleetTagsResponse'];

export interface BulkOrganizationChanges {
  locationUuid?: string;
  tagOperation?: string;
  tagUuids?: string[];
}

export interface BulkOrganizationFailure<T> {
  camera: T;
  error: string;
}

export interface BulkOrganizationResult<T> {
  succeeded: T[];
  failed: BulkOrganizationFailure<T>[];
}

export type BoundedResult<R> =
  | { status: 'fulfilled'; value: R }
  | { status: 'rejected'; reason: unknown };

// ---------------------------------------------------------------------------
// Collections
// ---------------------------------------------------------------------------

export type CameraCollection = components['schemas']['CameraCollection'];

export type CameraCollectionsResponse = components['schemas']['CameraCollectionsResponse'];

export type CameraCollectionMembersResponse = components['schemas']['CameraCollectionMembersResponse'];

export type CameraCollectionPreview = components['schemas']['CameraCollectionPreview'];

export interface CameraCollectionPayload {
  name: string;
  description: string;
  type: string;
  shared: boolean;
  selector?: FleetSelector | null;
}

export interface CollectionEditorSpec {
  collection: CameraCollection | null;
  members: string[];
}

// ---------------------------------------------------------------------------
// Event destinations
// ---------------------------------------------------------------------------

export interface EventDestinationBroker {
  host: string;
  port: number;
  client_id: string;
  topic_template: string;
  status_topic_template: string;
  keepalive_seconds: number;
  qos: number;
}

export interface EventDestination {
  uuid: string;
  key: string;
  name: string;
  description: string;
  enabled: boolean;
  type: string;
  managed: boolean;
  broker: EventDestinationBroker;
  authentication: { username: string; password_configured: boolean };
  tls: { mode: string; ca_file: string; cert_file: string; key_file: string };
  revision: number;
  created_at?: number;
  updated_at?: number;
}

/** The settings-managed MQTT destination, which has no broker or uuid here. */
export interface DefaultEventDestination {
  key: string;
  name: string;
  configuration_source?: string;
  managed: boolean;
}

export interface EventDestinationsResponse {
  default_destination?: DefaultEventDestination;
  count?: number;
  destinations?: EventDestination[];
}

/** One entry of the route editor's destination select. */
export interface EventDestinationOption {
  key: string;
  name: string;
  managed: boolean;
  enabled?: boolean;
}

export interface EventDestinationDraft {
  name: string;
  description: string;
  enabled: boolean;
  host: string;
  port: FormNumber;
  clientId: string;
  topicTemplate: string;
  statusTopicTemplate: string;
  keepaliveSeconds: FormNumber;
  qos: FormNumber;
  username: string;
  password: string;
  clearPassword: boolean;
  passwordConfigured: boolean;
  tlsMode: string;
  caFile: string;
  certFile: string;
  keyFile: string;
  revision: number;
  uuid: string;
}

export interface EventDestinationPayload {
  name: string;
  description: string;
  enabled: boolean;
  type: string;
  broker: EventDestinationBroker;
  authentication: { username: string; password?: string | null };
  tls: { mode: string; ca_file: string; cert_file: string; key_file: string };
  revision?: number;
}

// ---------------------------------------------------------------------------
// Event routes
// ---------------------------------------------------------------------------

export interface EventScheduleWindow {
  days: number[];
  start: string;
  end: string;
}

export interface EventRouteSuppression {
  debounce_seconds?: number;
  cooldown_seconds?: number;
  grouping_window_seconds?: number;
  max_events_per_minute?: number;
}

export interface EventRouteDetectionPredicate {
  labels_any?: string[];
  zone_ids_any?: string[];
  min_confidence?: number;
}

export interface EventRouteHealthPredicate {
  condition_codes_any?: string[];
  severities_any?: string[];
}

export interface EventRoutePredicate {
  version?: number;
  detection?: EventRouteDetectionPredicate;
  health?: EventRouteHealthPredicate;
}

export interface EventRouteCameraScope {
  type: string;
  selector?: FleetSelector | null;
}

export interface EventRouteSchedule {
  version?: number;
  timezone?: string;
  windows?: EventScheduleWindow[];
}

export interface EventRoute {
  uuid: string;
  name: string;
  description?: string;
  enabled: boolean;
  destination: string;
  event_types?: string[];
  camera_scope?: EventRouteCameraScope;
  predicate?: EventRoutePredicate;
  schedule?: EventRouteSchedule;
  suppression?: EventRouteSuppression;
  revision: number;
  created_at?: number;
  updated_at?: number;
}

export interface EventRoutesResponse {
  routes?: EventRoute[];
  count?: number;
}

export interface EventCatalogEntry {
  type: string;
  family: string;
  description: string;
  severity: string;
  dynamic_severity?: boolean;
  minimum_severity: string;
  maximum_severity: string;
  sensitivity: string;
  media_policy?: string;
  expected_rate: string;
  subject_kind: string;
  default_expiry_seconds?: number;
}

export interface EventCatalogResponse {
  schema_version?: number;
  event_types?: EventCatalogEntry[];
}

export interface EventCatalogGroup {
  family: string;
  types: EventCatalogEntry[];
}

/** Entries of `health_effective_policy.conditions`. The helpers accept `code` or `condition`. */
export interface HealthPolicyCondition {
  code?: string;
  condition?: string;
}

export type HealthCondition = HealthPolicyCondition & { code: string };

/** The part of GET /api/settings that the route editor reads. */
export interface HealthPolicySettings {
  health_effective_policy?: { conditions?: HealthPolicyCondition[] };
}

export interface EventRouteDraft {
  name: string;
  description: string;
  enabled: boolean;
  destination: string;
  eventTypes: string[];
  scopeType: string;
  selector: FleetSelector | null;
  selectorError: string;
  detectionFilterEnabled: boolean;
  labels: string;
  zones: string;
  minConfidence: FormNumber;
  healthFilterEnabled: boolean;
  healthConditionCodes: string[];
  healthSeverities: string[];
  scheduleEnabled: boolean;
  timezone: string;
  windows: EventScheduleWindow[];
  debounceSeconds: FormNumber;
  cooldownSeconds: FormNumber;
  groupingWindowSeconds: FormNumber;
  maxEventsPerMinute: FormNumber;
  revision: number;
  uuid: string;
}

export interface EventRoutePayload {
  name: string;
  description: string;
  enabled: boolean;
  destination: string;
  event_types: string[];
  camera_scope: EventRouteCameraScope;
  predicate: EventRoutePredicate;
  schedule: Required<EventRouteSchedule>;
  suppression: Required<EventRouteSuppression>;
  revision?: number;
}

/** Registries validateRouteDraft checks a draft against. */
export interface EventRouteRegistries {
  catalog?: EventCatalogEntry[];
  healthConditions?: HealthPolicyCondition[];
  healthSeverities?: string[];
}

export interface EventRoutePreview {
  matched_camera_count?: number;
  sample_limit?: number;
  camera_sample?: { camera_uuid: string; name: string; location_path?: string }[];
  event_types?: Pick<EventCatalogEntry, 'type' | 'subject_kind'>[];
  would_publish?: boolean;
}
