import { prepareRecordingPlayback } from '../js/utils/recording-playback.js';

describe('recording playback preparation', () => {
  const videoUrl = '/api/recordings/play/42?token=example';
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
    await expect(prepareRecordingPlayback(videoUrl)).resolves.toBe(videoUrl);
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
    expect(ready).toHaveBeenCalledWith(videoUrl);
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
});
