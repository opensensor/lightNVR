/**
 * Last known image for a live tile in its error state (#974).
 *
 * When a tile cannot play, the area behind its error overlay shows the most
 * recent image of the camera instead of plain black:
 *
 *   1. The tile's own last decoded frame, drawn once from the <video> element
 *      when playback fails (before the cell tears the element down). Only
 *      failure paths capture, and the JPEG is encoded only once the tile is
 *      actually in its error state, so healthy tiles never pay for it.
 *   2. Otherwise the thumbnail of the camera's newest completed recording
 *      (frame index 2, about 1 s before the end; index 0 when that fails).
 *
 * Everything here is framework-free so the rules can be unit tested; the
 * useLastFrame hook and the LastFrameFallback components wire it to the cells.
 *
 * Network cost is bounded per tile: one recordings-list request and at most
 * two thumbnail requests per outage. The result (including "nothing found") is
 * reused while the tile stays down, so auto-retry cycles never refetch it; it is
 * dropped when the stream plays again or after LAST_FRAME_RECORDING_TTL_MS.
 */
import { fetchJSON } from '../../../fetch-utils.ts';
import { Priority, queueThumbnailLoad } from '../../../request-queue.ts';
import type { Recording } from '../../../types/core.ts';
import type { RecordingsListResponse } from '../../../types/recordings.ts';
import type { LastFrameImage } from '../../../types/live.ts';

/** Newest recordings fetched per lookup; the first may still be in progress. */
export const LAST_FRAME_RECORDING_LIMIT = 5;
/** Thumbnail indexes tried in order: the recording's last frame, then its first. */
export const LAST_FRAME_THUMBNAIL_INDEXES = Object.freeze([2, 0] as const);
/** How long a recording lookup (hit or miss) is reused while the tile stays down. */
export const LAST_FRAME_RECORDING_TTL_MS = 5 * 60 * 1000;
/** Captured frames are scaled down to this width to bound memory per tile. */
export const LAST_FRAME_MAX_WIDTH = 1280;
/** Error overlay background while an image is shown behind it. */
export const LAST_FRAME_OVERLAY_BACKGROUND = 'rgba(0, 0, 0, 0.55)';

type RecordingEnd = Pick<Recording, 'end_time_unix' | 'end_time' | 'in_progress'> & Partial<Pick<Recording, 'id'>>;

export function recordingsListUrl(streamName: string): string {
  const params = new URLSearchParams({
    stream: streamName,
    sort: 'start_time',
    order: 'desc',
    limit: String(LAST_FRAME_RECORDING_LIMIT),
  });
  return `/api/recordings?${params.toString()}`;
}

export function recordingThumbnailUrl(id: number | string, index: number): string {
  return `/api/recordings/thumbnail/${encodeURIComponent(String(id))}/${index}`;
}

/** Epoch milliseconds a completed recording ended, or 0 when it has no end yet. */
export function recordingEndMilliseconds(recording: Partial<RecordingEnd> | null | undefined): number {
  if (!recording || recording.in_progress) return 0;
  const unix = Number(recording.end_time_unix);
  if (Number.isFinite(unix) && unix > 0) return unix * 1000;
  const parsed = typeof recording.end_time === 'string' ? Date.parse(recording.end_time) : NaN;
  return Number.isFinite(parsed) && parsed > 0 ? parsed : 0;
}

/** The recording with the latest non-zero end time, ignoring in-progress ones. */
export function pickNewestCompletedRecording<T extends Partial<RecordingEnd>>(recordings: readonly T[] | null | undefined): T | null {
  if (!Array.isArray(recordings)) return null;
  let newest: T | null = null;
  let newestEnd = 0;
  for (const recording of recordings) {
    const end = recordingEndMilliseconds(recording);
    if (end > newestEnd && recording?.id !== undefined && recording.id !== null) {
      newest = recording;
      newestEnd = end;
    }
  }
  return newest;
}

export interface RecordingFallbackDeps {
  fetchRecordings?: (streamName: string) => Promise<RecordingsListResponse | null | undefined>;
  /** Resolves when the thumbnail loaded; rejects on any HTTP or network failure. */
  loadThumbnail?: (url: string) => Promise<unknown>;
}

const defaultFetchRecordings = (streamName: string) =>
  fetchJSON<RecordingsListResponse>(recordingsListUrl(streamName), { retries: 0, timeout: 15000 });

// No retries: a 400 (thumbnails_per_recording <= 1) or 404 moves straight on.
const defaultLoadThumbnail = (url: string) => queueThumbnailLoad(url, Priority.LOW, 0);

/**
 * Thumbnail of the stream's newest completed recording: one list request, then
 * index 2 and, only if that fails, index 0. Resolves null instead of rejecting.
 */
export async function loadRecordingFallback(streamName: string, deps: RecordingFallbackDeps = {}): Promise<LastFrameImage | null> {
  const fetchRecordings = deps.fetchRecordings || defaultFetchRecordings;
  const loadThumbnail = deps.loadThumbnail || defaultLoadThumbnail;
  let recording;
  try {
    recording = pickNewestCompletedRecording((await fetchRecordings(streamName))?.recordings);
  } catch {
    return null;
  }
  const id = recording?.id;
  if (id === undefined || id === null) return null;
  const capturedAt = recordingEndMilliseconds(recording);
  for (const index of LAST_FRAME_THUMBNAIL_INDEXES) {
    const url = recordingThumbnailUrl(id, index);
    try {
      await loadThumbnail(url);
      return { url, capturedAt, source: 'recording' };
    } catch {
      // Try the next index.
    }
  }
  return null;
}

/** The subset of HTMLVideoElement a frame capture reads. */
export type CapturableVideo = Pick<HTMLVideoElement, 'videoWidth' | 'videoHeight' | 'readyState'> & CanvasImageSource;

/**
 * Draws the video's current frame onto a canvas, once. Returns null, without
 * throwing, when there is no decoded frame or the canvas cannot be drawn.
 */
export function captureVideoFrame(
  video: CapturableVideo | null | undefined,
  doc: Pick<Document, 'createElement'> | null | undefined = globalThis.document,
  maxWidth: number = LAST_FRAME_MAX_WIDTH
): HTMLCanvasElement | null {
  try {
    if (!video || !doc) return null;
    const width = video.videoWidth;
    const height = video.videoHeight;
    // HAVE_CURRENT_DATA: a frame is decoded and paintable.
    if (!width || !height || video.readyState < 2) return null;
    const scale = Math.min(1, maxWidth / width);
    const canvas = doc.createElement('canvas');
    canvas.width = Math.max(1, Math.round(width * scale));
    canvas.height = Math.max(1, Math.round(height * scale));
    const context = canvas.getContext('2d');
    if (!context) return null;
    context.drawImage(video, 0, 0, canvas.width, canvas.height);
    return canvas;
  } catch {
    return null;
  }
}

/** JPEG blob of a canvas, or null when it is tainted, empty or unsupported. */
export function canvasToBlob(canvas: Pick<HTMLCanvasElement, 'toBlob'> | null | undefined): Promise<Blob | null> {
  return new Promise((resolve) => {
    try {
      if (!canvas || typeof canvas.toBlob !== 'function') {
        resolve(null);
        return;
      }
      // A cross-origin (tainted) canvas throws a SecurityError here.
      canvas.toBlob((blob) => resolve(blob && blob.size > 0 ? blob : null), 'image/jpeg', 0.85);
    } catch {
      resolve(null);
    }
  });
}

function sameLocalDay(a: Date, b: Date): boolean {
  return a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate();
}

/**
 * Caption time in the user's locale and timezone: the time alone for today,
 * otherwise the date too (and the year when it differs from this year).
 */
export function formatLastImageTime(capturedAt: number, locale?: string | null, now: number = Date.now()): string {
  const date = new Date(capturedAt);
  if (!Number.isFinite(date.getTime())) return '';
  const today = new Date(now);
  const options: Intl.DateTimeFormatOptions = sameLocalDay(date, today)
    ? { hour: 'numeric', minute: '2-digit' }
    : {
      month: 'short',
      day: 'numeric',
      hour: 'numeric',
      minute: '2-digit',
      ...(date.getFullYear() === today.getFullYear() ? {} : { year: 'numeric' }),
    };
  try {
    return new Intl.DateTimeFormat(locale || undefined, options).format(date);
  } catch {
    return new Intl.DateTimeFormat(undefined, options).format(date);
  }
}

/** The error overlay keeps its own opaque background unless an image is shown. */
export function lastFrameOverlayBackground(image: LastFrameImage | null | undefined, defaultBackground: string): string {
  return image ? LAST_FRAME_OVERLAY_BACKGROUND : defaultBackground;
}

export interface LastFrameControllerDeps {
  loadRecordingFallback?: (streamName: string) => Promise<LastFrameImage | null>;
  captureFrame?: (video: CapturableVideo) => HTMLCanvasElement | null;
  toBlob?: (canvas: HTMLCanvasElement) => Promise<Blob | null>;
  createObjectURL?: (blob: Blob) => string;
  revokeObjectURL?: (url: string) => void;
  now?: () => number;
}

export interface LastFrameController {
  /**
   * Capture the current frame on a failure teardown, before the cell resets
   * the video. Only the canvas draw happens here; encoding waits for setError.
   */
  capture(video: CapturableVideo | null | undefined): boolean;
  setError(hasError: boolean, video?: CapturableVideo | null): void;
  setEnabled(enabled: boolean): void;
  setStream(streamName: string): void;
  /** The video is playing again: drop every image and re-arm capture. */
  recovered(): void;
  dispose(): void;
}

/**
 * State machine behind useLastFrame. `onChange` receives the image to show
 * (only while in error and enabled) or null.
 *
 * Capture is armed by playback: after a frame is captured, no further capture
 * happens until the video plays again, so a retry cycle never re-stamps an old
 * frozen frame with a new time.
 *
 * A captured canvas is held unencoded until the tile enters its error state;
 * if playback resumes first it is simply dropped, with no JPEG encode.
 */
export function createLastFrameController(
  onChange: (image: LastFrameImage | null) => void,
  deps: LastFrameControllerDeps = {},
  initialEnabled: boolean = true
): LastFrameController {
  const loadRecording = deps.loadRecordingFallback || ((name: string) => loadRecordingFallback(name));
  const captureFrame = deps.captureFrame || ((video: CapturableVideo) => captureVideoFrame(video));
  const toBlob = deps.toBlob || canvasToBlob;
  const createObjectURL = deps.createObjectURL || ((blob: Blob) => URL.createObjectURL(blob));
  const revokeObjectURL = deps.revokeObjectURL || ((url: string) => URL.revokeObjectURL(url));
  const now = deps.now || Date.now;

  let enabled = initialEnabled;
  let streamName = '';
  let inError = false;
  let armed = false;
  let disposed = false;
  let frame: LastFrameImage | null = null;
  let heldCanvas: { canvas: HTMLCanvasElement; capturedAt: number } | null = null;
  let framePending = false;
  let frameToken = 0;
  let recording: { image: LastFrameImage | null; fetchedAt: number } | null = null;
  let recordingInFlight = false;
  let recordingToken = 0;
  let shown: LastFrameImage | null = null;

  const publish = () => {
    if (disposed) return;
    const next = enabled && inError ? (frame || recording?.image || null) : null;
    if (next !== shown) {
      shown = next;
      onChange(next);
    }
  };

  const dropFrame = () => {
    frameToken++;
    framePending = false;
    heldCanvas = null;
    if (frame) {
      try { revokeObjectURL(frame.url); } catch { /* already gone */ }
      frame = null;
    }
  };

  const dropRecording = () => {
    recordingToken++;
    recordingInFlight = false;
    recording = null;
  };

  const requestRecording = () => {
    if (disposed || !enabled || !inError || !streamName) return;
    if (frame || framePending || heldCanvas || recordingInFlight) return;
    if (recording && now() - recording.fetchedAt < LAST_FRAME_RECORDING_TTL_MS) return;
    recordingInFlight = true;
    const token = ++recordingToken;
    Promise.resolve()
      .then(() => loadRecording(streamName))
      .catch(() => null)
      .then((image) => {
        if (disposed || token !== recordingToken) return;
        recordingInFlight = false;
        recording = { image: image || null, fetchedAt: now() };
        publish();
      });
  };

  /** Encode the held canvas; only called once the tile is in error. */
  const encodeHeld = () => {
    if (!heldCanvas) return;
    const { canvas, capturedAt } = heldCanvas;
    heldCanvas = null;
    const token = frameToken;
    framePending = true;
    Promise.resolve()
      .then(() => toBlob(canvas))
      .catch(() => null)
      .then((blob) => {
        if (disposed || token !== frameToken) return;
        framePending = false;
        let url: string | null = null;
        if (blob) {
          try { url = createObjectURL(blob); } catch { url = null; }
        }
        if (url) {
          frame = { url, capturedAt, source: 'frame' };
          publish();
        } else {
          requestRecording();
        }
      });
  };

  const tryCapture = (video: CapturableVideo | null | undefined): boolean => {
    if (disposed || !enabled || !armed || !video) return false;
    const canvas = captureFrame(video);
    if (!canvas) return false;
    armed = false;
    dropFrame();
    heldCanvas = { canvas, capturedAt: now() };
    if (inError) encodeHeld();
    return true;
  };

  return {
    capture(video) {
      // While in error the element already shows a stale or empty frame.
      return inError ? false : tryCapture(video);
    },
    setError(hasError, video) {
      if (hasError === inError) return;
      if (hasError) {
        inError = true;
        tryCapture(video);
        encodeHeld();
        requestRecording();
      } else {
        inError = false;
      }
      publish();
    },
    setEnabled(next) {
      if (next === enabled) return;
      enabled = next;
      if (!enabled) {
        dropFrame();
        dropRecording();
      } else {
        requestRecording();
      }
      publish();
    },
    setStream(next) {
      if (next === streamName) return;
      streamName = next;
      armed = false;
      dropFrame();
      dropRecording();
      if (inError) requestRecording();
      publish();
    },
    recovered() {
      armed = true;
      dropFrame();
      dropRecording();
      publish();
    },
    dispose() {
      dropFrame();
      dropRecording();
      disposed = true;
      shown = null;
    },
  };
}
