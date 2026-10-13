import { startDetectionPolling } from '../js/utils/detection-polling.js';

describe('detection polling under slow responses', () => {
  let stop;
  let originalFetch;
  let onResults;
  let onError;

  beforeEach(() => {
    jest.useFakeTimers();
    originalFetch = global.fetch;
    onResults = jest.fn();
    onError = jest.fn();
  });

  afterEach(() => {
    stop?.();
    jest.useRealTimers();
    global.fetch = originalFetch;
  });

  function start(options = {}) {
    stop = startDetectionPolling({
      streamName: 'front door', canPoll: () => true,
      onResults, onError, ...options,
    });
  }

  test('waits for the complete response before starting another poll', async () => {
    let completeBody;
    global.fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: () => new Promise(resolve => { completeBody = resolve; }),
    });
    start();
    await jest.advanceTimersByTimeAsync(4000);
    expect(global.fetch).toHaveBeenCalledTimes(1);
    expect(global.fetch.mock.calls[0][0]).toContain('front%20door');
    completeBody({ detections: [] });
    await jest.advanceTimersByTimeAsync(0);
    expect(onResults).toHaveBeenCalledWith({ detections: [] });
    await jest.advanceTimersByTimeAsync(999);
    expect(global.fetch).toHaveBeenCalledTimes(1);
    await jest.advanceTimersByTimeAsync(1);
    expect(global.fetch).toHaveBeenCalledTimes(2);
  });

  test('times out stalled requests and resumes without overlapping them', async () => {
    let active = 0;
    let maximumActive = 0;
    global.fetch = jest.fn((_url, { signal }) => new Promise((_resolve, reject) => {
      active++;
      maximumActive = Math.max(maximumActive, active);
      signal.addEventListener('abort', () => {
        active--;
        reject(new Error('aborted'));
      }, { once: true });
    }));
    start();
    await jest.advanceTimersByTimeAsync(5999);
    expect(global.fetch).toHaveBeenCalledTimes(1);
    expect(onError).not.toHaveBeenCalled();
    await jest.advanceTimersByTimeAsync(1);
    expect(onError).toHaveBeenCalledTimes(1);
    expect(active).toBe(0);
    await jest.advanceTimersByTimeAsync(1000);
    expect(global.fetch).toHaveBeenCalledTimes(2);
    expect(maximumActive).toBe(1);
  });

  test('closing the overlay aborts the request and ignores late results', async () => {
    let finish;
    global.fetch = jest.fn().mockImplementation(() => new Promise(resolve => { finish = resolve; }));
    start();
    await jest.advanceTimersByTimeAsync(1000);
    const signal = global.fetch.mock.calls[0][1].signal;
    stop();
    expect(signal.aborted).toBe(true);
    finish({ ok: true, json: async () => ({ detections: [{ label: 'person' }] }) });
    await jest.advanceTimersByTimeAsync(10000);
    expect(global.fetch).toHaveBeenCalledTimes(1);
    expect(onResults).not.toHaveBeenCalled();
    expect(onError).not.toHaveBeenCalled();
  });

  test('skips requests before video loads and backs off after repeated errors', async () => {
    let videoReady = false;
    global.fetch = jest.fn().mockResolvedValue({ ok: false, status: 503 });
    start({ canPoll: () => videoReady });
    await jest.advanceTimersByTimeAsync(10000);
    expect(global.fetch).not.toHaveBeenCalled();
    videoReady = true;
    await jest.advanceTimersByTimeAsync(4000);
    expect(global.fetch).toHaveBeenCalledTimes(4);
    expect(onError).toHaveBeenCalledTimes(4);
    await jest.advanceTimersByTimeAsync(1999);
    expect(global.fetch).toHaveBeenCalledTimes(4);
    await jest.advanceTimersByTimeAsync(1);
    expect(global.fetch).toHaveBeenCalledTimes(5);
  });
});
