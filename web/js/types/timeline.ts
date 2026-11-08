/**
 * Timeline domain types, read from src/web/api_handlers_timeline.c,
 * src/web/api_handlers_detection.c, and the timelineState store in
 * components/preact/timeline/TimelinePage.tsx.
 */
import type { DetectionObservation, StreamSummary } from './core.ts';

/** Anything with a Unix-second span: segments and exact detection intervals. */
export interface TimelineRange {
  start_timestamp: number;
  end_timestamp: number;
}

/** Exact recording identity; large IDs stay decimal strings throughout the UI. */
export type TimelineRecordingId = number | string;

/** One entry of `segments` from /api/timeline/segments and /segments-by-ids. */
export interface TimelineSegment extends TimelineRange {
  id: TimelineRecordingId;
  stream: string;
  camera_uuid: string | null;
  /** Local time, "YYYY-MM-DD HH:MM:SS". */
  start_time: string;
  end_time: string;
  duration: number;
  /** Formatted for display, for example "12.3 MB". */
  size: string;
  capture_method: string;
  has_detection: boolean;
  in_progress: boolean;
  schedule_restricted: boolean | null;
  local_start_timestamp: number;
  local_end_timestamp: number;
  /**
   * Read as a fallback by the timeline, but emitted by neither handler, which
   * send `stream` instead.
   */
  stream_name?: string;
  /** Client-only: set on a polled-out segment while it fades from the timeline. */
  _removing?: boolean;
}

/** Exact external-motion range from `detection_intervals`. */
export type TimelineDetectionInterval = TimelineRange;

/** GET /api/timeline/segments */
export interface TimelineSegmentsResponse {
  segments: TimelineSegment[];
  detection_intervals?: TimelineDetectionInterval[];
  detection_interval_count?: number;
  stream: string;
  start_time: string;
  end_time: string;
  segment_count: number;
}

/** GET /api/timeline/segments-by-ids */
export interface TimelineSegmentsByIdsResponse {
  segments: TimelineSegment[];
  stream: string;
  multi_stream: boolean;
  start_time: string;
  end_time: string;
  segment_count: number;
}

/** One entry of `detections` from GET /api/detection/results/<stream>. */
export interface TimelineDetection {
  label: string;
  confidence: number;
  x: number;
  y: number;
  width: number;
  height: number;
  timestamp: number;
  end_timestamp: number;
  observation?: DetectionObservation;
}

export interface TimelineDetectionsResponse {
  detections?: TimelineDetection[];
}

/** GET /api/recordings/<id>/tags */
export interface TimelineRecordingTagsResponse {
  tags?: string[];
}

/** Timestamped drag position, used to compute a fling's launch velocity. */
export interface TimelineFlingSample {
  time: number;
  position: number;
}

export interface TimelineHourRange {
  startHour: number;
  endHour: number;
}

export interface TimelineDayBounds {
  startTimestamp: number;
  endTimestamp: number;
  durationHours: number;
}

/** Thumbnail shown while the cursor is dragged. */
export interface TimelineDragPreview {
  frameIndex: number;
  label: string;
  recordingId: TimelineRecordingId;
  url: string;
}

/** The data half of timelineState: what setState may update. */
export interface TimelineStateFields {
  streams: StreamSummary[];
  timelineSegments: TimelineSegment[];
  selectedStream: string | null;
  selectedDate: string | null;
  currentRecordingId: TimelineRecordingId | null;
  currentRecordingProtected: boolean;
  currentRecordingTags: string[];
  isPlaying: boolean;
  currentSegmentIndex: number;
  timelineStartHour: number;
  timelineEndHour: number;
  autoFitStartHour: number;
  autoFitEndHour: number;
  currentTime: number | null;
  prevCurrentTime: number | null;
  playbackSpeed: number;
  forceReload: boolean;
  userControllingCursor: boolean;
  preserveCursorPosition: boolean;
  cursorPositionLocked: boolean;
  /** Absent from the initial state; only TimelinePlayer reads or clears it. */
  directVideoControl?: boolean;
}

export type TimelineStateUpdate = Partial<TimelineStateFields>;

export type TimelineStateListener = (state: TimelineState) => void;

export interface TimelineState extends TimelineStateFields {
  timelineHourToTimestamp: (hour: number, selectedDate: string | null) => number;
  timestampToTimelineHour: (timestamp: number | null, selectedDate?: string | null) => number | null;
  listeners: Set<TimelineStateListener>;
  lastUpdateTime: number;
  pendingUpdates: TimelineStateUpdate;
  isNotifying: boolean;
  setState(newState: TimelineStateUpdate): void;
  subscribe(listener: TimelineStateListener): () => boolean;
  notifyListeners(): void;
  flushPendingUpdates(): void;
}
