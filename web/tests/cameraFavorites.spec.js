import {
  applyFavoriteChange,
  favoriteCameraUuids,
  filterFavoriteStreams,
  requestFavoriteChange,
} from '../js/components/preact/live/cameraFavorites.js';

jest.mock('../js/query-client.js', () => ({
  fetchJSON: jest.fn(),
  useQuery: jest.fn(),
  useMutation: jest.fn(),
  useQueryClient: jest.fn(),
}));

describe('camera favorites', () => {
  const streams = [
    { name: 'Lobby', camera_uuid: 'cam-a' },
    { name: 'Yard', camera_uuid: 'cam-b' },
    { name: 'Gate', camera_uuid: 'cam-c' },
  ];

  test('derives the favorite set from the API document and filters streams by it', () => {
    const favorites = favoriteCameraUuids({
      favorites: [{ camera_uuid: 'cam-c', created_at: 5 }, { camera_uuid: 'cam-a', created_at: 9 }],
      can_modify: true,
    });
    expect([...favorites]).toEqual(['cam-c', 'cam-a']);
    expect(filterFavoriteStreams(streams, favorites).map((stream) => stream.name)).toEqual(['Lobby', 'Gate']);
    expect(favoriteCameraUuids(undefined).size).toBe(0);
    expect(filterFavoriteStreams(undefined, favorites)).toEqual([]);
  });

  test('applies an add and a removal to the cached list in oldest-first order', () => {
    const initial = { favorites: [{ camera_uuid: 'cam-b', created_at: 20 }], can_modify: true };

    const added = applyFavoriteChange(initial, 'cam-a', { camera_uuid: 'cam-a', created_at: 10 });
    expect(added.favorites.map((favorite) => favorite.camera_uuid)).toEqual(['cam-a', 'cam-b']);
    expect(added.can_modify).toBe(true);
    expect(initial.favorites).toHaveLength(1);

    const repeated = applyFavoriteChange(added, 'cam-a', { camera_uuid: 'cam-a', created_at: 10 });
    expect(repeated.favorites).toHaveLength(2);

    const removed = applyFavoriteChange(repeated, 'cam-b', null);
    expect(removed.favorites.map((favorite) => favorite.camera_uuid)).toEqual(['cam-a']);

    expect(applyFavoriteChange(undefined, 'cam-z', { created_at: 1 }).favorites)
      .toEqual([{ camera_uuid: 'cam-z', created_at: 1 }]);
  });

  test('sends PUT to add and DELETE to remove with the camera UUID encoded', async () => {
    const request = jest.fn().mockResolvedValue({ ok: true });

    await requestFavoriteChange('cam/1', true, request);
    expect(request).toHaveBeenCalledWith('/api/camera-favorites/cam%2F1',
      expect.objectContaining({ method: 'PUT' }));

    await requestFavoriteChange('cam-2', false, request);
    expect(request).toHaveBeenLastCalledWith('/api/camera-favorites/cam-2',
      expect.objectContaining({ method: 'DELETE' }));
  });

  test('surfaces a request failure', async () => {
    const request = jest.fn().mockRejectedValue(new Error('forbidden'));
    await expect(requestFavoriteChange('cam-1', true, request)).rejects.toThrow('forbidden');
  });
});
