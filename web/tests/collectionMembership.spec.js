import { fetchCollectionCameraUuids } from '../js/components/preact/fleet/collectionMembership.js';

jest.mock('../js/query-client.js', () => ({
  fetchJSON: jest.fn(),
  useQuery: jest.fn(),
}));

describe('collection membership', () => {
  test('loads every authorized fleet page and removes duplicate UUIDs', async () => {
    const request = jest.fn()
      .mockResolvedValueOnce({
        cameras: [{ camera_uuid: 'camera-a' }, { camera_uuid: 'camera-b' }],
        total_pages: 2,
      })
      .mockResolvedValueOnce({
        cameras: [{ camera_uuid: 'camera-b' }, { camera_uuid: 'camera-c' }],
        total_pages: 2,
      });

    await expect(fetchCollectionCameraUuids('collection-a', request)).resolves.toEqual([
      'camera-a', 'camera-b', 'camera-c',
    ]);
    expect(request).toHaveBeenCalledTimes(2);
    expect(JSON.parse(request.mock.calls[0][1].body)).toMatchObject({
      collection_uuid: 'collection-a',
      page: 1,
      page_size: 200,
      facets: false,
    });
    expect(JSON.parse(request.mock.calls[1][1].body).page).toBe(2);
  });

  test('does not call the API without a selected collection', async () => {
    const request = jest.fn();
    await expect(fetchCollectionCameraUuids('', request)).resolves.toEqual([]);
    expect(request).not.toHaveBeenCalled();
  });

  test('surfaces a page failure', async () => {
    const request = jest.fn().mockRejectedValue(new Error('not authorized'));
    await expect(fetchCollectionCameraUuids('collection-a', request)).rejects.toThrow('not authorized');
  });
});
