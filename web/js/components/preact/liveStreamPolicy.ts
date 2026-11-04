/**
 * Decide whether a fullscreen WebRTC main-stream upgrade should step back to
 * the already configured sub-stream.
 *
 * Fullscreen is the only automatic main-stream upgrade. A failed upgrade
 * should therefore fail fast without changing the source used by other
 * viewers. We also fall back when media arrives but sustained transport
 * quality is poor enough to make fullscreen stutter (#468).
 */
export function shouldFallbackFullscreenToSubStream({
  fullscreenUpgraded,
  effectiveUseSubStream,
  noVideoCheckCount = 0,
  connectionQuality = 'unknown',
}: {
  fullscreenUpgraded: boolean;
  effectiveUseSubStream: boolean;
  noVideoCheckCount?: number;
  connectionQuality?: string;
}): boolean {
  if (!fullscreenUpgraded || effectiveUseSubStream) return false;

  return noVideoCheckCount >= 1 ||
    connectionQuality === 'poor' ||
    connectionQuality === 'bad';
}

export type WebRTCAudioDirection = 'none' | 'recvonly' | 'sendrecv';

/**
 * Decide which audio track a WebRTC tile should request.
 *
 * The instance audio policy always wins. Two-way talk needs its sender from the
 * start of the connection. Otherwise audio is requested only once the viewer has
 * asked to listen, because the request alone can start a server-side transcoder.
 */
export function getWebRTCAudioDirection({
  audioDisabled,
  backchannelEnabled,
  listeningRequested,
}: {
  audioDisabled: boolean;
  backchannelEnabled: boolean;
  listeningRequested: boolean;
}): WebRTCAudioDirection {
  if (audioDisabled) return 'none';
  if (backchannelEnabled) return 'sendrecv';
  return listeningRequested ? 'recvonly' : 'none';
}
