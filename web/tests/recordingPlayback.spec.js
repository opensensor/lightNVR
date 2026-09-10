import { loadRecordingPlayback, prepareRecordingPlayback } from '../js/utils/recording-playback.js';

describe('recording playback preparation', () => {
  const videoUrl = '/api/recordings/play/42?token=example';
  const compatibleUrl = 'http://localhost/api/recordings/play/42?token=example&transcode=1';
  const response = (status, body = {}, retryAfter = '2') => ({
    status,
    headers: { get: () => retryAfter },
    json: async () => body,
  });

  beforeEach(() => {
    jest.useFakeTimers();
    jest.spyOn(globalThis, 'fetch');
  });
  afterEach(() => {
    jest.restoreAllMocks();
    jest.useRealTimers();
  });

  test('loads a ready recording after one preparation request, preserving query parameters', async () => {
    fetch.mockResolvedValue(response(200));
    await expect(prepareRecordingPlayback(videoUrl)).resolves.toBe(compatibleUrl);
    expect(fetch).toHaveBeenCalledTimes(1);
    const [url, options] = fetch.mock.calls[0];
    expect(new URL(url).searchParams.get('prepare')).toBe('1');
    expect(new URL(url).searchParams.get('token')).toBe('example');
    expect(options.cache).toBe('no-store');
  });

  test('polls a slow conversion until ready without returning a media URL early', async () => {
    fetch.mockResolvedValueOnce(response(202)).mockResolvedValueOnce(response(202))
      .mockResolvedValueOnce(response(200));
    const onWaiting = jest.fn();
    const ready = jest.fn();
    const promise = prepareRecordingPlayback(videoUrl, { onWaiting }).then(ready);
    await jest.advanceTimersByTimeAsync(0);
    expect(onWaiting).toHaveBeenCalledTimes(1);
    expect(ready).not.toHaveBeenCalled();
    await jest.advanceTimersByTimeAsync(2000);
    expect(ready).not.toHaveBeenCalled();
    await jest.advanceTimersByTimeAsync(2000);
    await promise;
    expect(fetch).toHaveBeenCalledTimes(3);
    expect(ready).toHaveBeenCalledWith(compatibleUrl);
  });

  test('closing while waiting cancels the timer and prevents another request', async () => {
    fetch.mockResolvedValue(response(202));
    const controller = new AbortController();
    const promise = prepareRecordingPlayback(videoUrl, { signal: controller.signal });
    const rejected = expect(promise).rejects.toMatchObject({ name: 'AbortError' });
    await jest.advanceTimersByTimeAsync(0);
    controller.abort();
    await rejected;
    await jest.advanceTimersByTimeAsync(10000);
    expect(fetch).toHaveBeenCalledTimes(1);
    expect(jest.getTimerCount()).toBe(0);
  });

  test('a late preparation response cannot load a recording after switching clips', async () => {
    const controller = new AbortController();
    fetch.mockImplementation(async () => {
      controller.abort();
      return response(200);
    });
    await expect(prepareRecordingPlayback(videoUrl, { signal: controller.signal }))
      .rejects.toMatchObject({ name: 'AbortError' });
  });

  test.each([401, 403, 404, 500])('reports HTTP %s without retrying indefinitely', async status => {
    fetch.mockResolvedValue(response(status, { error: 'Recording unavailable' }));
    await expect(prepareRecordingPlayback(videoUrl)).rejects.toThrow('Recording unavailable');
    expect(fetch).toHaveBeenCalledTimes(1);
    expect(jest.getTimerCount()).toBe(0);
  });

  test('keeps ordinary video URLs playable', async () => {
    await expect(prepareRecordingPlayback('/example.mp4')).resolves.toBe('/example.mp4');
    expect(fetch).not.toHaveBeenCalled();
  });

  function mediaElement() {
    const video = new EventTarget();
    video.load = jest.fn(() => { video.error = null; });
    video.pause = jest.fn();
    video.removeAttribute = jest.fn(() => { video.src = ''; });
    video.fail = code => {
      video.error = { code };
      video.dispatchEvent(new Event('error'));
    };
    return video;
  }

  test('loads native recordings immediately without a preparation request', () => {
    const video = mediaElement();
    const cleanup = loadRecordingPlayback(video, videoUrl);
    expect(video.src).toBe(videoUrl);
    expect(video.load).toHaveBeenCalledTimes(1);
    expect(fetch).not.toHaveBeenCalled();
    cleanup();
  });

  test.each([3, 4])('prepares one compatibility copy after media error %s', async code => {
    fetch.mockResolvedValueOnce(response(202)).mockResolvedValueOnce(response(200));
    const video = mediaElement();
    const onPreparing = jest.fn();
    const onError = jest.fn();
    const cleanup = loadRecordingPlayback(video, videoUrl, { onPreparing, onError });
    video.fail(code);
    expect(onPreparing).toHaveBeenCalledTimes(1);
    await jest.advanceTimersByTimeAsync(2000);
    expect(video.src).toBe(compatibleUrl);
    expect(fetch).toHaveBeenCalledTimes(2);
    video.fail(code);
    expect(onError).toHaveBeenCalledTimes(1);
    expect(fetch).toHaveBeenCalledTimes(2);
    cleanup();
  });

  test.each([1, 2])('does not convert after an aborted load or network error (%s)', code => {
    const video = mediaElement();
    const cleanup = loadRecordingPlayback(video, videoUrl);
    video.fail(code);
    expect(fetch).not.toHaveBeenCalled();
    cleanup();
  });

  test('closing a player cancels fallback polling and prevents stale media loading', async () => {
    fetch.mockResolvedValue(response(202));
    const video = mediaElement();
    const onError = jest.fn();
    const cleanup = loadRecordingPlayback(video, videoUrl, { onError });
    video.fail(4);
    await jest.advanceTimersByTimeAsync(0);
    cleanup();
    await jest.advanceTimersByTimeAsync(10000);
    expect(fetch).toHaveBeenCalledTimes(1);
    expect(video.src).toBe('');
    expect(onError).not.toHaveBeenCalled();
    expect(jest.getTimerCount()).toBe(0);
  });
});
