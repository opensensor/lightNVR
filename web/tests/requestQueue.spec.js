import {
  Priority,
  invalidateThumbnailLoad,
  parseRetryAfterMilliseconds,
  queueThumbnailLoad,
  shouldRetryThumbnailRequest,
} from '../js/request-queue.js';

describe('thumbnail request admission', () => {
  afterEach(() => {
    jest.restoreAllMocks();
    delete global.Image;
  });

  test('deduplicates pending and successful loads without speculative Image requests', async () => {
    global.Image = class {
      constructor() {
        throw new Error('queueThumbnailLoad must not create a speculative Image');
      }
    };

    let finishRequest;
    const fetchMock = jest.spyOn(global, 'fetch').mockImplementation(() => new Promise((resolve) => {
      finishRequest = () => resolve({
        ok: true,
        status: 200,
        headers: { get: () => null },
        blob: async () => new Blob(['jpeg']),
      });
    }));
    const url = '/api/recordings/thumbnail/request-queue-test/0';

    const first = queueThumbnailLoad(url, Priority.HIGH, 0);
    const duplicate = queueThumbnailLoad(url, Priority.HIGH, 0);
    await Promise.resolve();
    expect(fetchMock).toHaveBeenCalledTimes(1);

    finishRequest();
    await Promise.all([first, duplicate]);
    await queueThumbnailLoad(url, Priority.HIGH, 0);
    expect(fetchMock).toHaveBeenCalledTimes(1);

    invalidateThumbnailLoad(url);
    fetchMock.mockResolvedValue({
      ok: true,
      status: 200,
      headers: { get: () => null },
      blob: async () => new Blob(['jpeg']),
    });
    await queueThumbnailLoad(url, Priority.HIGH, 0);
    expect(fetchMock).toHaveBeenCalledTimes(2);
  });

  test('retries capacity responses but not permanent generation failures', () => {
    expect(shouldRetryThumbnailRequest({ status: 503 })).toBe(true);
    expect(shouldRetryThumbnailRequest({ status: 429 })).toBe(true);
    expect(shouldRetryThumbnailRequest({ status: 500 })).toBe(false);
  });

  test('parses Retry-After seconds and dates', () => {
    expect(parseRetryAfterMilliseconds('2', 1000)).toBe(2000);
    expect(parseRetryAfterMilliseconds('Thu, 01 Jan 1970 00:00:05 GMT', 1000)).toBe(4000);
    expect(parseRetryAfterMilliseconds('invalid', 1000)).toBeNull();
  });
});
