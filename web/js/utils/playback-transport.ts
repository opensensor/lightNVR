import type {
  LiveTransport,
  PlaybackTransportPreference,
  PlaybackTransportProfile,
  TransportOfferings,
} from '../types/core.ts';

export const PLAYBACK_TRANSPORT_CHAINS: Readonly<Record<PlaybackTransportProfile, readonly LiveTransport[]>> = Object.freeze({
  webrtc_only: ['webrtc'],
  mse_only: ['mse'],
  hls_only: ['hls'],
  webrtc_then_mse: ['webrtc', 'mse'],
  mse_then_hls: ['mse', 'hls'],
});

const AUTO_CHAINS: Readonly<Record<LiveTransport, readonly LiveTransport[]>> = Object.freeze({
  webrtc: ['webrtc', 'mse', 'hls'],
  mse: ['mse', 'hls'],
  hls: ['hls'],
});

export interface PlaybackTransportPlan {
  profile: PlaybackTransportPreference;
  forcedTransport: LiveTransport | null;
  requested: LiveTransport[];
  available: LiveTransport[];
  unavailable: LiveTransport[];
}

/** A failure reported by a transport cell: a message, or an object carrying one. */
export type PlaybackFailure = string | { kind?: string; message?: string } | null | undefined;

type PlaybackFailureDetail = Exclude<PlaybackFailure, string>;

export function normalizePlaybackTransport(value: unknown): PlaybackTransportPreference {
  return value === 'auto' || Object.hasOwn(PLAYBACK_TRANSPORT_CHAINS, value as string)
    ? value as PlaybackTransportPreference
    : 'auto';
}

export function buildPlaybackTransportPlan(
  preference: unknown,
  offerings: TransportOfferings = { webrtc: true, mse: true, hls: true },
  defaultTransport: string = 'webrtc',
  forcedTransport: string | null = null
): PlaybackTransportPlan {
  const profile = normalizePlaybackTransport(preference);
  const viewerOverride = Object.hasOwn(AUTO_CHAINS, forcedTransport as string)
    ? forcedTransport as LiveTransport
    : null;
  const requested = viewerOverride
    ? [viewerOverride]
    : profile === 'auto'
      ? [...(AUTO_CHAINS[defaultTransport as LiveTransport] || AUTO_CHAINS.webrtc)]
      : [...PLAYBACK_TRANSPORT_CHAINS[profile]];
  const available = requested.filter((transport) => offerings[transport] !== false);
  const unavailable = requested.filter((transport) => offerings[transport] === false);

  return { profile, forcedTransport: viewerOverride, requested, available, unavailable };
}

export function isPlaybackFallback(
  plan: PlaybackTransportPlan | null | undefined,
  activeTransport: LiveTransport | null | undefined,
  runtimeFallbackIndex: number = 0,
): boolean {
  if (!plan || !activeTransport) return false;
  if (runtimeFallbackIndex > 0) return true;
  return plan.profile !== 'auto' && plan.requested.indexOf(activeTransport) > 0;
}

const SOURCE_UNAVAILABLE_PATTERN = /(?:dial tcp|no route to host|host is unreachable|network is unreachable|connection refused|connect:|i\/o timeout)/i;
const TERMINAL_STREAM_STATUSES = new Set(['error', 'failed', 'stopped', 'offline']);

/**
 * Runtime transport fallback is useful for browser/codec/transport failures,
 * but cannot recover a camera source that is itself offline. In that case the
 * active player should retain its normal backoff retry instead of probing each
 * renderer in the transport chain.
 */
export function shouldFallbackPlaybackTransport(
  failure: PlaybackFailure,
  { streamStatus, sourceUnavailableMessage }: { streamStatus?: string; sourceUnavailableMessage?: string } = {}
): boolean {
  const status = String(streamStatus || '').trim().toLowerCase();
  if (TERMINAL_STREAM_STATUSES.has(status)) return false;

  if ((failure as PlaybackFailureDetail)?.kind === 'source-unavailable') return false;

  const message = String((failure as PlaybackFailureDetail)?.message || failure || '').trim();
  if (!message) return false;
  if (sourceUnavailableMessage && message === sourceUnavailableMessage) return false;
  return !SOURCE_UNAVAILABLE_PATTERN.test(message);
}
