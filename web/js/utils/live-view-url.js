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

/**
 * Per-variant settings for the Live page. The WebRTC-first page (index.html
 * with WebRTC available) and the HLS page (hls.html, or index.html when WebRTC
 * is off) used to be two components. They persist their state under separate
 * keys and differ in a few toolbar details, so those differences live here.
 */
export const LIVE_VIEW_VARIANTS = {
  webrtc: {
    storagePrefix: 'webrtc',
    queryKey: 'webrtc-summary',
    tagFilterStyle: 'chips',
    streamLabelKey: 'live.stream',
    fullscreenTitleKey: 'live.toggleFullscreen',
  },
  hls: {
    storagePrefix: 'hls',
    queryKey: 'live-summary',
    tagFilterStyle: 'select',
    streamLabelKey: 'nav.streams',
    fullscreenTitleKey: 'timeline.fullscreen',
  },
};

/** Variant that index.html renders: WebRTC-first unless WebRTC is disabled. */
export function resolveLiveViewVariant({ webrtcDisabled = false } = {}) {
  return webrtcDisabled ? 'hls' : 'webrtc';
}

/**
 * Transport tabs shown in the Live page header, in display order. Each tab
 * only renders if its method is enabled in settings (#397). MSE additionally
 * requires go2rtc to be reachable.
 */
export function getLiveTransportTabs({
  forcedTransport = null,
  currentSearch = '',
  autoDisabled = false,
  webrtcDisabled = false,
  hlsDisabled = false,
  mseDisabled = false,
  go2rtcAvailable = false,
} = {}) {
  return [
    !autoDisabled && {
      id: 'auto', labelKey: 'live.autoShort', active: forcedTransport === null,
      href: buildLiveViewHref('/index.html', currentSearch),
    },
    !webrtcDisabled && {
      id: 'webrtc', labelKey: 'live.webrtcShort', active: forcedTransport === 'webrtc',
      href: buildLiveViewHref('/index.html', currentSearch, 'webrtc'),
    },
    !hlsDisabled && {
      id: 'hls', labelKey: 'live.hlsShort', active: forcedTransport === 'hls',
      href: buildLiveViewHref('/hls.html', currentSearch),
    },
    go2rtcAvailable && !mseDisabled && {
      id: 'mse', labelKey: 'live.mseShort', active: forcedTransport === 'mse',
      href: buildLiveViewHref('/hls.html', currentSearch, 'mse'),
    },
  ].filter(Boolean);
}
