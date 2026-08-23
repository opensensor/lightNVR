import { buildLiveViewHref } from '../js/utils/live-view-url.js';

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
});
