export function buildLiveViewHref(path, currentSearch = '', mode = '') {
  const params = new URLSearchParams(currentSearch);
  if (mode) params.set('mode', mode);
  else params.delete('mode');
  const query = params.toString();
  return query ? `${path}?${query}` : path;
}

const FORCED_LIVE_TRANSPORTS = new Set(['webrtc', 'mse', 'hls']);

/**
 * Resolve a page-level playback override. The default live URL has no mode and
 * normally preserves each stream's configured transport chain. When the Auto
 * selector is disabled, that URL resolves to the first offered explicit
 * transport. Legacy hls.html URLs imply an explicit HLS selection.
 */
export function resolveForcedLiveTransport(
  pathname = '',
  currentSearch = '',
  { autoDisabled = false, offerings = {} } = {}
) {
  const mode = new URLSearchParams(currentSearch).get('mode');
  if (FORCED_LIVE_TRANSPORTS.has(mode)) return mode;
  if (pathname.endsWith('/hls.html')) return 'hls';
  if (autoDisabled) {
    return ['webrtc', 'mse', 'hls'].find(
      (transport) => offerings[transport] !== false
    ) || null;
  }
  return null;
}
