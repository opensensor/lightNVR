/**
 * Types for the Live page: the grid, operator navigator, building plan, layout
 * sketch, and PTZ and tile controls. Response shapes come from the C handlers in
 * src/web/, named on each type; see js/types/core.ts for the shared stream types.
 */
import type { TranslationParams } from '../i18n.ts';
import type { StreamSummary } from './core.ts';

/** The shape of i18n's `t` that the plan components take as a prop. */
export type Translate = (key: string, params?: TranslationParams) => string;

// ---------------------------------------------------------------------------
// Streams on the Live page
// ---------------------------------------------------------------------------

/**
 * Item of the Live page's stream list. handle_get_stream_summaries emits neither
 * `is_deleted` nor `sub_stream_url`, so both are undefined at runtime; they are
 * typed because LiveView and filterLiveNavigatorInventory read them.
 */
export interface LiveStream extends StreamSummary {
  is_deleted?: boolean;
  sub_stream_url?: string;
}

export type LiveOperatorSurface = 'grid' | 'plan';

/** The Navigator's scope, applied by LiveView before camera ordering. */
export interface LiveOperatorFilter {
  selectedLocation: string;
  availability: string;
  cameraUuids: Set<string>;
}

/** A layout the Navigator or the plan asks LiveView to show. */
export interface LiveOperatorLayout {
  cameraUuids?: string[];
  columns?: number;
  rows?: number;
}

// ---------------------------------------------------------------------------
// Tile controls
// ---------------------------------------------------------------------------

export type LiveDisplayMode = 'fit' | 'fill';
export type RecordingMode = 'constant' | 'detection' | 'schedule';
export type LiveTileHealth = 'healthy' | 'degraded' | 'offline';

/**
 * The stream fields tile status reads. Every stream form the cells hold fits
 * this. No stream handler emits `state`; healthStateForStream falls back to it.
 */
export interface LiveTileStream {
  record?: boolean;
  detection_based_recording?: boolean;
  record_on_schedule?: boolean;
  status?: string;
  state?: string;
}

export interface LiveTilePlayback {
  error?: unknown;
  isPlaying?: boolean;
  isLoading?: boolean;
}

/** GET and POST /api/streams/:name/recording (api_handlers_recording_control.c). */
export interface ManualRecordingStatus {
  stream?: string;
  state?: string;
  recording_active?: boolean;
  capture_method?: string | null;
  recording_id?: number | null;
  manual_control_allowed?: boolean;
  manual_control_reason?: string | null;
}

/** GET and PUT /api/streams/:name/daynight (api_handlers_imaging.c). */
export interface DayNightResponse {
  success?: boolean;
  mode?: string;
  error?: string;
}

// ---------------------------------------------------------------------------
// PTZ: api_handlers_ptz.c
// ---------------------------------------------------------------------------

export type PtzDirection = 'up' | 'down' | 'left' | 'right' | 'zoom-in' | 'zoom-out';

/** `status` is set for a failed HTTP response, `stage` by savePtzPreset. */
export interface PtzError extends Error {
  status?: number;
  stage?: string;
}

/** Every PTZ action body; a failure may report `success: false` with HTTP 200. */
export interface PtzActionResult {
  success?: boolean;
  message?: string;
  token?: string;
}

export interface PtzPreset {
  token: string;
  name: string;
}

export interface PtzPresetsResponse extends PtzActionResult {
  presets?: PtzPreset[];
  count?: number;
}

export interface PtzRange {
  min: number;
  max: number;
}

export interface PtzCapabilities extends PtzActionResult {
  ptz_enabled?: boolean;
  onvif_queried?: boolean;
  ptz_service_url?: string;
  has_pan_tilt?: boolean;
  has_zoom?: boolean;
  has_continuous_move?: boolean;
  has_absolute_move?: boolean;
  has_relative_move?: boolean;
  has_home_position?: boolean;
  has_presets?: boolean;
  max_presets?: number;
  pan_range?: PtzRange;
  tilt_range?: PtzRange;
  zoom_range?: PtzRange;
}

// ---------------------------------------------------------------------------
// Favorites: api_handlers_camera_favorites.c
// ---------------------------------------------------------------------------

export interface CameraFavorite {
  camera_uuid: string;
  created_at?: number;
}

export interface CameraFavoritesResponse {
  favorites?: CameraFavorite[];
  can_modify?: boolean;
}

/** PUT returns the favorite; DELETE returns `{ success, removed }`. */
export type CameraFavoriteChangeResult = Partial<CameraFavorite>;

export interface CameraFavoriteChange {
  cameraUuid: string;
  favorite: boolean;
}

// ---------------------------------------------------------------------------
// Locations: api_handlers_locations.c
// ---------------------------------------------------------------------------

export interface LiveLocation {
  uuid: string;
  parent_uuid?: string | null;
  name: string;
  type?: string;
  sort_order: number;
  is_system?: boolean;
  metadata?: unknown;
  child_count?: number;
  camera_count?: number;
  created_at?: number;
  updated_at?: number;
}

export interface LiveLocationsResponse {
  locations?: LiveLocation[];
  count?: number;
  unassigned_uuid?: string;
}

/** A location plus what buildLocationTree computes for it. */
export interface LocationTreeNode extends LiveLocation {
  children: LocationTreeNode[];
  directStreams: LiveStream[];
  total: number;
  live: number;
  depth: number;
}

// ---------------------------------------------------------------------------
// Saved views: api_handlers_live_layouts.c
// ---------------------------------------------------------------------------

export interface LiveLayoutSlot {
  camera_uuid: string;
}

export interface LiveLayout {
  uuid: string;
  name: string;
  is_shared: boolean;
  owned?: boolean;
  location_uuid?: string | null;
  availability: string;
  columns: number;
  rows: number;
  camera_slots?: LiveLayoutSlot[];
  partially_authorized?: boolean;
  revision: number;
  created_at?: number;
  updated_at?: number;
}

export interface LiveLayoutsResponse {
  layouts?: LiveLayout[];
  count?: number;
  can_share?: boolean;
  can_modify?: boolean;
}

/** Body of POST /api/live/layouts, and of PUT with `revision` added. */
export interface LiveLayoutPayload {
  name: string;
  is_shared: boolean;
  location_uuid: string | null;
  availability: string;
  columns: number;
  rows: number;
  camera_slots: LiveLayoutSlot[];
}

// ---------------------------------------------------------------------------
// Recent events: POST /api/investigations/search (api_handlers_investigations.c)
// ---------------------------------------------------------------------------

/** Only the fields the Navigator's event list reads. */
export interface LiveEvent {
  result_id?: string | number;
  event_type?: string;
  start_time: number;
  camera?: { name?: string };
  detection?: { label?: string };
  thumbnail?: { status?: string; url?: string };
}

export interface LiveEventsResponse {
  results?: LiveEvent[];
}

// ---------------------------------------------------------------------------
// Layout sketch: sketch_shape() in api_handlers_operator_floor_plans.c
// ---------------------------------------------------------------------------

export type SketchTone = 'slate' | 'blue' | 'green' | 'amber' | 'red' | 'violet';
export type SketchLabelSize = 'sm' | 'md' | 'lg';
export type SketchTool = 'select' | 'room' | 'wall' | 'area' | 'label';
export type RectCorner = 'nw' | 'ne' | 'sw' | 'se';
export type RectEdge = 'n' | 's' | 'e' | 'w';
export type ShapeOrderDirection = 'front' | 'back' | 'forward' | 'backward';

/** A plan-fraction point (0..1), the form shapes store. */
export type SketchPoint = [number, number];

/** A plan-fraction point (0..1), the form pointer handlers work in. */
export interface PlanPoint {
  x: number;
  y: number;
}

export interface SketchCanvas {
  width?: number;
  height?: number;
}

export interface PlanRect extends PlanPoint {
  w: number;
  h: number;
}

export interface SketchBounds {
  x0: number;
  y0: number;
  x1: number;
  y1: number;
}

// `points?: undefined` and `label?: undefined` let the helpers read those fields
// on any shape, and make them discriminants when they are checked.
export interface SketchRectShape extends PlanRect {
  id: string;
  type: 'rect';
  tone: SketchTone;
  filled: boolean;
  label?: string;
  points?: undefined;
}

export interface SketchWallShape {
  id: string;
  type: 'wall';
  tone: SketchTone;
  points: SketchPoint[];
  label?: undefined;
}

export interface SketchAreaShape {
  id: string;
  type: 'area';
  tone: SketchTone;
  points: SketchPoint[];
  label?: string;
}

export interface SketchLabelShape extends PlanPoint {
  id: string;
  type: 'label';
  tone: SketchTone;
  text: string;
  size: SketchLabelSize;
  points?: undefined;
  label?: undefined;
}

export type SketchShape = SketchRectShape | SketchWallShape | SketchAreaShape | SketchLabelShape;

/** Input of normalizeShape: stored or drafted data, validated before use. */
export interface RawSketchShape {
  id?: unknown;
  type?: unknown;
  tone?: unknown;
  x?: unknown;
  y?: unknown;
  w?: unknown;
  h?: unknown;
  filled?: unknown;
  label?: unknown;
  points?: unknown;
  text?: unknown;
  size?: unknown;
}

/** Undo history. The coalesce fields are set together, or not at all. */
export interface SketchHistory<T> {
  past: T[];
  present: T;
  future: T[];
  coalesceKey?: string;
  coalesceAt?: number;
}

/** What a pointer gesture on a shape grabbed. */
export type ShapeHandle =
  | { kind: 'move' }
  | { kind: 'edge'; edge: RectEdge }
  | { kind: 'corner'; corner: RectCorner }
  | { kind: 'insert'; index: number }
  | { kind: 'vertex'; index: number };

/** An in-progress room, or an in-progress wall or area. */
export type SketchDrawing =
  | {
    tool: 'room';
    start: PlanPoint;
    current: PlanPoint;
    points?: undefined;
    cursor?: undefined;
  }
  | {
    tool: 'wall' | 'area';
    points: SketchPoint[];
    cursor?: PlanPoint;
    start?: undefined;
    current?: undefined;
  };

// ---------------------------------------------------------------------------
// Building plans: api_handlers_operator_floor_plans.c
// ---------------------------------------------------------------------------

export interface FloorPlanCamera extends PlanPoint {
  camera_uuid: string;
  rotation?: number;
  fov?: number;
}

export interface FloorPlanSketch {
  version?: number;
  shapes?: RawSketchShape[];
}

export interface FloorPlanCanvasSize {
  canvas_width: number;
  canvas_height: number;
}

export interface FloorPlan extends FloorPlanCanvasSize {
  uuid: string;
  name: string;
  location_uuid?: string | null;
  parent_plan_uuid?: string | null;
  background_mime?: string | null;
  sketch?: FloorPlanSketch | null;
  cameras?: FloorPlanCamera[];
  partially_authorized?: boolean;
  revision: number;
  can_modify?: boolean;
  created_at?: number;
  updated_at?: number;
}

export interface FloorPlansResponse {
  plans?: FloorPlan[];
  can_modify?: boolean;
}

export type PlanCameraMarker = FloorPlanCamera & { kind: 'camera' };

export interface PlanClusterMarker extends PlanPoint {
  kind: 'cluster';
  camera_uuids: string[];
  count: number;
}

export type PlanMarker = PlanCameraMarker | PlanClusterMarker;

export interface PlanCoverageInput {
  fov?: number | null;
  rotation?: number | null;
}

/** Plan draft edited as one undo history, covering shapes and cameras. */
export interface PlanDraft {
  shapes: SketchShape[];
  cameras: FloorPlanCamera[];
}

/** One item of PlanContextMenu; see the component's comment. */
export interface PlanMenuAction {
  id: string;
  label: string;
  shortcut?: string;
  disabled?: boolean;
  danger?: boolean;
  onSelect: () => unknown;
  separator?: undefined;
  swatches?: undefined;
}

export interface PlanMenuSeparator {
  separator: true;
  id?: undefined;
  label?: undefined;
  swatches?: undefined;
}

export interface PlanMenuSwatch {
  id: string;
  className: string;
  label: string;
  active: boolean;
  onSelect: () => unknown;
}

export interface PlanMenuSwatches {
  id: string;
  label: string;
  swatches: PlanMenuSwatch[];
  separator?: undefined;
}

export type PlanMenuEntry = PlanMenuAction | PlanMenuSeparator | PlanMenuSwatches;

export interface PlanMenuState {
  clientX: number;
  clientY: number;
  items: PlanMenuEntry[];
}
