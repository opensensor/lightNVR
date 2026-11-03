/**
 * Shapes shared by the live transport cells (WebRTC, HLS, MSE), their selector
 * (PlaybackTransportCell), and the fullscreen and picture-in-picture helpers.
 */
import type { StreamSummary, TransportOfferings } from './core.ts';

/**
 * The live-surface stream a cell renders. Two fields are read by the cells but
 * emitted by no handler in src/web/, so they stay here rather than in core.ts:
 * - `state`: fallback for `status` in WebRTCVideoCell and PlaybackTransportCell.
 * - `ptz_type`: read by MSEVideoCell. The handlers emit `ptz_enabled` instead.
 */
export interface LiveStream extends StreamSummary {
  state?: string;
  ptz_type?: string;
}

/** Called with the stream name, the triggering event, and the cell element. */
export type FullscreenToggleHandler = (
  streamName: string,
  event: Event,
  cellElement: HTMLElement | null
) => unknown;

/** Props accepted by every transport cell, so PlaybackTransportCell can pick one. */
export interface LiveTransportCellProps {
  stream: LiveStream;
  streamId: string;
  useSubStream?: boolean;
  /** Read by WebRTCVideoCell only. PlaybackTransportCell passes undefined to the others. */
  fullscreenUpgraded?: boolean;
  onToggleFullscreen: FullscreenToggleHandler;
  showLabels?: boolean;
  showControls?: boolean;
  globalShowDetections?: boolean;
  alwaysFullscreenOnTap?: boolean;
  onRequestReorder?: (() => unknown) | undefined;
  mobileGesturesDisabled?: boolean;
  audioDisabled?: boolean;
  onTransportFailure?: ((failure: string) => unknown) | undefined;
}

export interface PlaybackTransportCellProps extends Omit<LiveTransportCellProps, 'onTransportFailure'> {
  offerings?: TransportOfferings;
  defaultTransport?: string;
  forcedTransport?: string | null;
}

/** HLS source: go2rtc's dynamic HLS, lightNVR's FFmpeg HLS, or neither. */
export type HlsMode = 'go2rtc' | 'native' | 'failed';

/** Fields read from RTCPeerConnection.getStats() reports, whose values lib.dom types as any. */
export interface WebRTCStatsEntry {
  type: string;
  kind?: string;
  state?: string;
  packetsLost?: number;
  packetsReceived?: number;
  jitter?: number;
  currentRoundTripTime?: number;
}

/** GET /api/ice-servers (handle_get_ice_servers). */
export interface IceServersResponse {
  ice_servers?: RTCIceServer[];
}

/** JSON text frame from go2rtc's /api/ws. */
export interface Go2rtcWsMessage {
  type: string;
  value: string;
}

/** An error that a WebSocket handler has already surfaced to the user. */
export interface HandledGateError extends Error {
  handled?: boolean;
}

/** Settle callbacks of the pending stream-connection-gate promise in MSEVideoCell. */
export interface GateSettle {
  resolve: () => void;
  reject: (reason: Error) => void;
}

/** `document` with the WebKit-prefixed fullscreen API. */
export interface VendorFullscreenDocument extends Document {
  webkitFullscreenElement?: Element | null;
  webkitExitFullscreen?: () => Promise<void> | void;
}

/** An element with the WebKit-prefixed fullscreen API. */
export interface VendorFullscreenElement extends Element {
  webkitRequestFullscreen?: () => Promise<void> | void;
}

/** A video element with Safari's presentation-mode picture-in-picture API. */
export interface PresentationModeVideo extends HTMLVideoElement {
  webkitSupportsPresentationMode?: (mode: string) => boolean;
  webkitSetPresentationMode?: (mode: string) => void;
  webkitPresentationMode?: string;
}
