import {
  buildPlaybackTransportPlan,
  isPlaybackFallback,
  normalizePlaybackTransport,
} from '../js/utils/playback-transport.js';

describe('per-stream playback transport', () => {
  const all = { webrtc: true, mse: true, hls: true };

  test('normalizes unknown persisted values to auto', () => {
    expect(normalizePlaybackTransport('not-a-profile')).toBe('auto');
  });

  test('auto follows WebRTC, MSE, HLS order on the main live view', () => {
    expect(buildPlaybackTransportPlan('auto', all).available)
      .toEqual(['webrtc', 'mse', 'hls']);
  });

  test('honors independent fixed and ordered stream profiles', () => {
    expect(buildPlaybackTransportPlan('mse_only', all).available).toEqual(['mse']);
    expect(buildPlaybackTransportPlan('webrtc_then_mse', all).available)
      .toEqual(['webrtc', 'mse']);
  });

  test('reports a server-disabled only transport instead of degrading', () => {
    const plan = buildPlaybackTransportPlan('webrtc_only', { ...all, webrtc: false });
    expect(plan.available).toEqual([]);
    expect(plan.unavailable).toEqual(['webrtc']);
  });

  test('skips a disabled first choice and marks explicit fallback use', () => {
    const plan = buildPlaybackTransportPlan(
      'webrtc_then_mse',
      { ...all, webrtc: false }
    );
    expect(plan.available).toEqual(['mse']);
    expect(isPlaybackFallback(plan, 'mse', 0)).toBe(true);
  });

  test('supports the explicit HLS/MSE viewer defaults for auto streams', () => {
    expect(buildPlaybackTransportPlan('auto', all, 'mse').available)
      .toEqual(['mse', 'hls']);
    expect(buildPlaybackTransportPlan('auto', all, 'hls').available)
      .toEqual(['hls']);
  });
});
