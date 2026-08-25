export const PLAYBACK_TRANSPORT_CHAINS = Object.freeze({
  webrtc_only: ['webrtc'],
  mse_only: ['mse'],
  hls_only: ['hls'],
  webrtc_then_mse: ['webrtc', 'mse'],
  mse_then_hls: ['mse', 'hls'],
});

const AUTO_CHAINS = Object.freeze({
  webrtc: ['webrtc', 'mse', 'hls'],
  mse: ['mse', 'hls'],
  hls: ['hls'],
});

export function normalizePlaybackTransport(value) {
  return value === 'auto' || Object.hasOwn(PLAYBACK_TRANSPORT_CHAINS, value)
    ? value
    : 'auto';
}

export function buildPlaybackTransportPlan(
  preference,
  offerings = { webrtc: true, mse: true, hls: true },
  defaultTransport = 'webrtc'
) {
  const profile = normalizePlaybackTransport(preference);
  const requested = profile === 'auto'
    ? [...(AUTO_CHAINS[defaultTransport] || AUTO_CHAINS.webrtc)]
    : [...PLAYBACK_TRANSPORT_CHAINS[profile]];
  const available = requested.filter((transport) => offerings[transport] !== false);
  const unavailable = requested.filter((transport) => offerings[transport] === false);

  return { profile, requested, available, unavailable };
}

export function isPlaybackFallback(plan, activeTransport, runtimeFallbackIndex = 0) {
  if (!plan || !activeTransport) return false;
  if (runtimeFallbackIndex > 0) return true;
  return plan.profile !== 'auto' && plan.requested.indexOf(activeTransport) > 0;
}
