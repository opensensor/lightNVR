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
}) {
  if (!fullscreenUpgraded || effectiveUseSubStream) return false;

  return noVideoCheckCount >= 1 ||
    connectionQuality === 'poor' ||
    connectionQuality === 'bad';
}
