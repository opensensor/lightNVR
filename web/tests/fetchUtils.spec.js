import { enhancedFetch } from '../js/fetch-utils.js';

const okResponse = { ok: true, status: 200, statusText: 'OK' };

function waitForAbort(_url, { signal }) {
  return new Promise((_resolve, reject) => {
    const abort = () => reject(Object.assign(new Error('aborted'), { name: 'AbortError' }));
    if (signal.aborted) abort();
    else signal.addEventListener('abort', abort, { once: true });
  });
}

describe('fetch retries', () => {
  beforeEach(() => {
    jest.useFakeTimers();
    for (const method of ['log', 'debug', 'warn', 'error']) {
      jest.spyOn(console, method).mockImplementation(() => {});
    }
  });
  afterEach(() => {
    jest.useRealTimers();
    jest.restoreAllMocks();
  });

  test('retries a timed-out camera list with a fresh signal', async () => {
    const fetchMock = jest.spyOn(global, 'fetch')
      .mockImplementationOnce(waitForAbort)
      .mockImplementationOnce(async (_url, { signal }) => {
        expect(signal.aborted).toBe(false);
        return okResponse;
      });
    const result = expect(enhancedFetch('/api/streams', {
      timeout: 100, retryDelay: 10, retries: 1,
    })).resolves.toBe(okResponse);
    await jest.runAllTimersAsync();
    await result;
    expect(fetchMock).toHaveBeenCalledTimes(2);
    expect(fetchMock.mock.calls[0][1].signal).not.toBe(fetchMock.mock.calls[1][1].signal);
    expect(jest.getTimerCount()).toBe(0);
  });

  test('retries network failures without reusing an aborted signal', async () => {
    const fetchMock = jest.spyOn(global, 'fetch')
      .mockRejectedValueOnce(new TypeError('network failure'))
      .mockImplementationOnce(async (_url, { signal }) => {
        expect(signal.aborted).toBe(false);
        return okResponse;
      });
    const result = expect(enhancedFetch('/api/streams', {
      timeout: 100, retryDelay: 10,
    })).resolves.toBe(okResponse);
    await jest.runAllTimersAsync();
    await result;
    expect(fetchMock).toHaveBeenCalledTimes(2);
    expect(jest.getTimerCount()).toBe(0);
  });

  test('bounds every attempt and reports the final timeout', async () => {
    const fetchMock = jest.spyOn(global, 'fetch').mockImplementation(waitForAbort);
    const result = expect(enhancedFetch('/api/streams', {
      timeout: 100, retryDelay: 10, retries: 1,
    })).rejects.toThrow('Request timed out');
    await jest.runAllTimersAsync();
    await result;
    expect(fetchMock).toHaveBeenCalledTimes(2);
    expect(jest.getTimerCount()).toBe(0);
  });

  test('does not retry externally cancelled requests', async () => {
    const controller = new AbortController();
    const fetchMock = jest.spyOn(global, 'fetch').mockImplementation(waitForAbort);
    const result = expect(enhancedFetch('/api/streams', {
      signal: controller.signal, timeout: 100,
    })).rejects.toThrow('Request was cancelled');
    controller.abort();
    await result;
    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(jest.getTimerCount()).toBe(0);
  });

  test('keeps external cancellation connected to response body readers', async () => {
    const controller = new AbortController();
    let requestOptions;
    jest.spyOn(global, 'fetch').mockImplementation(async (_url, options) => {
      requestOptions = options;
      return okResponse;
    });
    await enhancedFetch('/api/streams', { signal: controller.signal });
    controller.abort();
    expect(requestOptions.signal.aborted).toBe(true);
    expect(jest.getTimerCount()).toBe(0);
  });

  test('does not issue a retry after cancellation during backoff', async () => {
    const controller = new AbortController();
    const fetchMock = jest.spyOn(global, 'fetch')
      .mockRejectedValue(new TypeError('network failure'));
    const result = expect(enhancedFetch('/api/streams', {
      signal: controller.signal, retryDelay: 100,
    })).rejects.toThrow('Request was cancelled');
    await jest.advanceTimersByTimeAsync(1);
    controller.abort();
    await jest.runAllTimersAsync();
    await result;
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  test('does not repeat a mutation after its response times out', async () => {
    const fetchMock = jest.spyOn(global, 'fetch').mockImplementation(waitForAbort);
    const result = expect(enhancedFetch('/api/streams', {
      method: 'POST', timeout: 100, retries: 1,
    })).rejects.toThrow('Request timed out');
    await jest.runAllTimersAsync();
    await result;
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  test('does not retry access errors', async () => {
    const fetchMock = jest.spyOn(global, 'fetch').mockResolvedValue({
      ok: false, status: 403, statusText: 'Forbidden',
    });
    await expect(enhancedFetch('/api/streams')).rejects.toMatchObject({ status: 403 });
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });
});
