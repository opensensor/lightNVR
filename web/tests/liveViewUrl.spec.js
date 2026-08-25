import {
  buildLiveViewHref,
  resolveForcedLiveTransport,
} from '../js/utils/live-view-url.js';

describe('live view links', () => {
  test('preserves collection and layout filters between playback modes', () => {
    expect(buildLiveViewHref(
      '/index.html',
      '?collection=collection-a&tag=Outdoor&cols=4&rows=3&mode=mse'
    )).toBe('/index.html?collection=collection-a&tag=Outdoor&cols=4&rows=3');
  });

  test('sets MSE mode while preserving the current collection', () => {
    expect(buildLiveViewHref('/hls.html', '?collection=collection-a', 'mse'))
      .toBe('/hls.html?collection=collection-a&mode=mse');
  });

  test('distinguishes default precedence from explicit transport modes', () => {
    expect(resolveForcedLiveTransport('/index.html', '?collection=collection-a'))
      .toBeNull();
    expect(resolveForcedLiveTransport('/index.html', '?mode=webrtc'))
      .toBe('webrtc');
    expect(resolveForcedLiveTransport('/hls.html', ''))
      .toBe('hls');
    expect(resolveForcedLiveTransport('/hls.html', '?mode=mse'))
      .toBe('mse');
  });
});
