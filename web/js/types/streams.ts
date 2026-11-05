/**
 * Types for the Streams page and its modals. Pilot wire shapes use generated
 * contracts; editable state and non-pilot create/test/ONVIF shapes stay separate.
 */
import type { EptzFormFields } from '../utils/eptz-config.ts';
import type { components } from '../../generated/api/schema';

/**
 * A numeric form field: a number until the user edits it, then the input's
 * string value. handleSubmit in StreamsView parses both.
 */
export type FormNumber = number | string;

// ---------------------------------------------------------------------------
// Detection zones: api_handlers_zones.c
// ---------------------------------------------------------------------------

/** A polygon vertex, as a [0,1] fraction of the image. */
export interface ZonePoint {
  x: number;
  y: number;
}

/**
 * Item of GET and POST /api/streams/:name/zones. The handler always emits the
 * optional fields, but zones drawn in ZoneEditor start without the last three.
 */
export interface DetectionZone {
  id: string;
  name: string;
  polygon: ZonePoint[];
  enabled: boolean;
  color?: string;
  stream_name?: string;
  filter_classes?: string;
  min_confidence?: number;
}

export interface ZonesResponse {
  zones?: DetectionZone[];
}

// ---------------------------------------------------------------------------
// Stream API: api_handlers_streams_get.c, api_handlers_streams_test.c
// ---------------------------------------------------------------------------

/** Validated pilot wire shapes; editable form state is defined separately. */
export type StreamListResponse = components['schemas']['StreamSummaryPage'];
export type StreamListItem = StreamListResponse['streams'][number];
export type StreamFullResponse = components['schemas']['StreamFullResponse'];
export type StreamFull = StreamFullResponse['stream'];

/** Explicit form-to-wire mapping shared by manual creation and ordinary PUT. */
export type StreamSavePayload = components['schemas']['StreamConfigurationUpdate'] & {
  name: string;
  is_deleted?: boolean;
};

/** Existing ONVIF POST shape, outside the generated pilot. */
export interface OnvifStreamCreatePayload {
  name: string;
  url: string;
  enabled: boolean;
  streaming_enabled: boolean;
  protocol: string;
  priority: string;
  segment_duration: number;
  record: boolean;
  record_audio: boolean;
  audio_voice_enhancement: boolean;
  backchannel_enabled: boolean;
  isOnvif: boolean;
  onvif_username: string;
  onvif_password: string;
  onvif_profile: string;
  onvif_port: number;
}

/** Body of POST /api/streams/test. */
export interface StreamTestRequest {
  url: string;
  protocol: number;
  onvif_username?: string;
  onvif_password?: string;
}

/** POST /api/streams/test. `message` is only present on failure. */
export interface StreamTestResponse {
  success: boolean;
  message?: string;
  info?: {
    width: number;
    height: number;
    fps: number;
    codec: string;
  };
}

/** fetchJSON rejects with HTTPError, which has `status`, or a plain Error, which does not. */
export type StreamRequestError = Error & { status?: number };

// ---------------------------------------------------------------------------
// Detection models: api_handlers_detection_models.c
// ---------------------------------------------------------------------------

export interface DetectionModel {
  id: string;
  name: string;
  path?: string;
  type?: string;
  supported?: boolean;
  /** Only present when supported is false. */
  unsupported_reason?: string;
  description?: string;
  size?: number;
}

export interface DetectionModelsResponse {
  models?: DetectionModel[];
}

// ---------------------------------------------------------------------------
// ONVIF: api_handlers_onvif_backend_agnostic.c
// ---------------------------------------------------------------------------

/** Item of GET /api/onvif/devices and POST /api/onvif/discovery/discover. */
export interface OnvifDevice {
  endpoint: string;
  device_service: string;
  ip_address: string;
  manufacturer: string;
  model: string;
  online: boolean;
  inventory_uuid: string;
  last_seen_at: number;
  claim_state: string;
  duplicate_suspected: boolean;
  /** No handler emits this; addOnvifDeviceAsStream reads it, then falls back. */
  name?: string;
}

export interface OnvifDevicesResponse {
  devices?: OnvifDevice[];
}

/** Item of GET /api/onvif/device/profiles. */
export interface OnvifProfile {
  token: string;
  name: string;
  stream_uri: string;
  width: number;
  height: number;
  encoding: string;
  fps: number;
}

export interface OnvifProfilesResponse {
  profiles?: OnvifProfile[];
}

export interface OnvifCredentials {
  username: string;
  password: string;
}

/** Body of POST /api/onvif/device/test. */
export interface OnvifTestRequest extends OnvifCredentials {
  url: string;
  insecure: boolean;
}

/** POST /api/onvif/device/test. Failures reject; the UI still reads `success`. */
export interface OnvifTestResponse {
  success?: boolean;
  message?: string;
}

// ---------------------------------------------------------------------------
// Configuration form
// ---------------------------------------------------------------------------

/** The fields the stream form owns. Edited numeric inputs hold strings. */
export interface StreamFormFields {
  name: string;
  url: string;
  adminUrl: string;
  enabled: boolean;
  streamingEnabled: boolean;
  playbackTransport: components['schemas']['PlaybackTransport'];
  width: number;
  height: number;
  fps: number;
  codec: string;
  protocol: string;
  priority: string;
  segment: FormNumber;
  record: boolean;
  recordAudio: boolean;
  audioVoiceEnhancement: boolean;
  backchannelEnabled: boolean;
  isOnvif: boolean;
  onvifUsername: string;
  onvifPassword: string;
  onvifProfile: string;
  onvifPort: FormNumber;
  detectionEnabled: boolean;
  detectionModel: string;
  detectionThreshold: FormNumber;
  detectionInterval: FormNumber;
  preBuffer: FormNumber;
  postBuffer: FormNumber;
  /** Absent in the add form. */
  detectionZones?: DetectionZone[];
  detectionObjectFilter: string;
  detectionObjectFilterList: string;
  ptzEnabled: boolean;
  ptzMaxX: FormNumber;
  ptzMaxY: FormNumber;
  ptzMaxZ: FormNumber;
  ptzHasHome: boolean;
  retentionDays: FormNumber;
  detectionRetentionDays: FormNumber;
  maxStorageMb: FormNumber;
  // Only set by the edit form or by the user's input:
  tierCriticalMultiplier?: FormNumber;
  tierImportantMultiplier?: FormNumber;
  tierEphemeralMultiplier?: FormNumber;
  storagePriority?: FormNumber;
  recordOnSchedule: boolean;
  recordingSchedule: boolean[];
  detectionRecordOnSchedule: boolean;
  detectionRecordingSchedule: boolean[];
  /** Comma-separated tag string. */
  tags: string;
  motionTriggerSource: string;
  go2rtcSourceOverride: string;
  publishUrl: string;
  subStreamUrl: string;
  detectionUrl: string;
}

/** ePTZ form fields, where edited numeric inputs also hold strings. */
export type StreamEptzFormFields =
  Pick<EptzFormFields, 'eptzEnabled'> &
  { [K in Exclude<keyof EptzFormFields, 'eptzEnabled'>]: FormNumber };

/** UI-owned fields only; full wire responses are mapped explicitly. */
export type StreamFormState = StreamFormFields & StreamEptzFormFields & {
  canControlPrivacy?: boolean;
};

/** The part of an input's target that handleInputChange reads. */
export interface StreamFieldTarget {
  name: string;
  value: unknown;
  type?: string;
  checked?: boolean;
}

/** StreamConfigModal also sends handleInputChange synthetic events of this shape. */
export interface StreamFieldChange {
  target: StreamFieldTarget;
}

export type StreamInputHandler = (event: Event | StreamFieldChange) => void;

/** StreamConfigModal's accordion sections and whether each is open. */
export interface ExpandedSections {
  basic: boolean;
  recording: boolean;
  detection: boolean;
  zones: boolean;
  motion: boolean;
  eptz: boolean;
  ptz: boolean;
  advanced: boolean;
  go2rtcOverride: boolean;
  restream: boolean;
}

export type StreamBulkAction = 'enable' | 'disable' | 'delete';
