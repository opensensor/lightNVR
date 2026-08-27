import { fetchAllStreamSummaries } from '../js/utils/stream-summaries.js';

describe('stream summary pagination', () => {
  afterEach(() => jest.restoreAllMocks());

  test('walks bounded pages and combines only summary rows', async () => {
    const fetchMock = jest.spyOn(global, 'fetch')
      .mockResolvedValueOnce({
        ok: true,
        status: 200,
        statusText: 'OK',
        json: async () => ({
          streams: [{ name: 'Camera 001' }],
          page: 1,
          total_pages: 2,
        }),
      })
      .mockResolvedValueOnce({
        ok: true,
        status: 200,
        statusText: 'OK',
        json: async () => ({
          streams: [{ name: 'Camera 101' }],
          page: 2,
          total_pages: 2,
        }),
      });

    await expect(fetchAllStreamSummaries({ surface: 'live', availability: 'live' })).resolves.toEqual([
      { name: 'Camera 001' },
      { name: 'Camera 101' },
    ]);
    expect(fetchMock).toHaveBeenCalledTimes(2);
    expect(fetchMock.mock.calls[0][0]).toContain('page=1');
    expect(fetchMock.mock.calls[0][0]).toContain('page_size=100');
    expect(fetchMock.mock.calls[0][0]).toContain('surface=live');
    expect(fetchMock.mock.calls[0][0]).toContain('availability=live');
    expect(fetchMock.mock.calls[1][0]).toContain('page=2');
  });
});
