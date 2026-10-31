import {
  getLiveTransportTabs,
  LIVE_VIEW_VARIANTS,
  resolveForcedLiveTransport,
  resolveLiveViewVariant,
} from '../js/utils/live-view-url.js';

// Before #654 index.html rendered WebRTCView unless WebRTC was disabled, and
// LiveView otherwise (hls.html always rendered LiveView). WebRTCView always
// showed its WebRTC tab; LiveView hid it when WebRTC was disabled. Both hid the
// Auto/HLS/MSE tabs the same way.
function legacyTabs(variant, flags, go2rtcAvailable) {
  return [
    !flags.autoDisabled && 'auto',
    (variant === 'webrtc' || !flags.webrtcDisabled) && 'webrtc',
    !flags.hlsDisabled && 'hls',
    go2rtcAvailable && !flags.mseDisabled && 'mse',
  ].filter(Boolean);
}

function legacyIndexVariant(flags) {
  return flags.webrtcDisabled ? 'hls' : 'webrtc';
}

const FLAG_NAMES = ['webrtcDisabled', 'hlsDisabled', 'mseDisabled', 'autoDisabled'];
const FLAG_COMBINATIONS = Array.from({ length: 1 << FLAG_NAMES.length }, (_, bits) =>
  Object.fromEntries(FLAG_NAMES.map((name, i) => [name, !!(bits & (1 << i))])));

function renderTabs(pathname, search, flags, go2rtcAvailable) {
  const forcedTransport = resolveForcedLiveTransport(pathname, search, {
    autoDisabled: flags.autoDisabled,
    offerings: {
      webrtc: !flags.webrtcDisabled,
      mse: !flags.mseDisabled,
      hls: !flags.hlsDisabled,
    },
  });
  const tabs = getLiveTransportTabs({
    forcedTransport,
    currentSearch: search,
    go2rtcAvailable,
    ...flags,
  });
  return { forcedTransport, tabs };
}

describe('merged live view transport tabs', () => {
  const cases = FLAG_COMBINATIONS.flatMap((flags) =>
    [true, false].map((go2rtcAvailable) => [flags, go2rtcAvailable]));

  test.each(cases)('index.html with %o (go2rtc %s) matches the legacy view', (flags, go2rtcAvailable) => {
    const variant = resolveLiveViewVariant(flags);
    expect(variant).toBe(legacyIndexVariant(flags));

    const { forcedTransport, tabs } = renderTabs('/index.html', '', flags, go2rtcAvailable);
    expect(tabs.map((tab) => tab.id)).toEqual(legacyTabs(variant, flags, go2rtcAvailable));

    // At most one tab is highlighted, and it is the active transport.
    const active = tabs.filter((tab) => tab.active).map((tab) => tab.id);
    expect(active.length).toBeLessThanOrEqual(1);
    if (active.length) expect(active[0]).toBe(forcedTransport ?? 'auto');
  });

  test.each(cases)('hls.html with %o (go2rtc %s) matches the legacy view', (flags, go2rtcAvailable) => {
    const { forcedTransport, tabs } = renderTabs('/hls.html', '', flags, go2rtcAvailable);
    expect(forcedTransport).toBe('hls');
    expect(tabs.map((tab) => tab.id)).toEqual(legacyTabs('hls', flags, go2rtcAvailable));
  });

  test('default transport follows the first enabled method when Auto is off', () => {
    const flags = { webrtcDisabled: true, hlsDisabled: false, mseDisabled: false, autoDisabled: true };
    const { forcedTransport, tabs } = renderTabs('/index.html', '', flags, true);
    expect(forcedTransport).toBe('mse');
    expect(tabs.find((tab) => tab.active).id).toBe('mse');
  });

  test('tab links keep the current filters and set the mode', () => {
    const { tabs } = renderTabs('/index.html', '?collection=c1&mode=mse', {}, true);
    expect(Object.fromEntries(tabs.map((tab) => [tab.id, tab.href]))).toEqual({
      auto: '/index.html?collection=c1',
      webrtc: '/index.html?collection=c1&mode=webrtc',
      hls: '/hls.html?collection=c1',
      mse: '/hls.html?collection=c1&mode=mse',
    });
    expect(tabs.find((tab) => tab.active).id).toBe('mse');
  });

  test('each variant keeps its own storage keys', () => {
    expect(LIVE_VIEW_VARIANTS.webrtc.storagePrefix).toBe('webrtc');
    expect(LIVE_VIEW_VARIANTS.hls.storagePrefix).toBe('hls');
  });
});
