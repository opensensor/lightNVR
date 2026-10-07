/**
 * Per-user favorite cameras for the Live view.
 *
 * Backed by GET/PUT/DELETE /api/camera-favorites. The pure helpers are
 * exported for unit tests; useCameraFavorites wires them to the query cache.
 */
import { useCallback, useMemo } from 'preact/hooks';
import { fetchJSON, useMutation, useQuery, useQueryClient } from '../../../query-client.js';

export const CAMERA_FAVORITES_QUERY_KEY = ['camera-favorites'];

const EMPTY_FAVORITES = Object.freeze({ favorites: [], can_modify: false });

export function favoriteCameraUuids(data) {
  return new Set((data?.favorites || []).map((favorite) => favorite.camera_uuid));
}

export function filterFavoriteStreams(streams, favoriteUuids) {
  return (streams || []).filter((stream) => favoriteUuids.has(stream.camera_uuid));
}

/**
 * Next cached favorites document after a toggle resolves. `record` is the
 * server's row for an add, or null for a removal. Keeps oldest-first order.
 */
export function applyFavoriteChange(data, cameraUuid, record) {
  const favorites = (data?.favorites || []).filter((favorite) => favorite.camera_uuid !== cameraUuid);
  if (record) favorites.push({ camera_uuid: cameraUuid, ...record });
  favorites.sort((a, b) =>
    (Number(a.created_at) - Number(b.created_at)) || String(a.camera_uuid).localeCompare(String(b.camera_uuid)));
  return { ...(data || EMPTY_FAVORITES), favorites };
}

export function requestFavoriteChange(cameraUuid, favorite, request = fetchJSON) {
  const url = `/api/camera-favorites/${encodeURIComponent(cameraUuid)}`;
  return request(url, { method: favorite ? 'PUT' : 'DELETE', retries: 0 });
}

export function useCameraFavorites() {
  const queryClient = useQueryClient();
  const query = useQuery(CAMERA_FAVORITES_QUERY_KEY, '/api/camera-favorites', {}, {
    staleTime: 30000,
    retry: false,
  });
  const favoriteUuids = useMemo(() => favoriteCameraUuids(query.data), [query.data]);
  const mutation = useMutation({
    mutationFn: ({ cameraUuid, favorite }) => requestFavoriteChange(cameraUuid, favorite),
    onSuccess: (result, { cameraUuid, favorite }) => {
      queryClient.setQueryData(CAMERA_FAVORITES_QUERY_KEY,
        (current) => applyFavoriteChange(current, cameraUuid, favorite ? result : null));
    },
    onSettled: () => queryClient.invalidateQueries({ queryKey: CAMERA_FAVORITES_QUERY_KEY }),
  });
  const toggleFavorite = useCallback(
    (cameraUuid) => mutation.mutateAsync({ cameraUuid, favorite: !favoriteUuids.has(cameraUuid) }),
    [mutation, favoriteUuids]
  );
  const pending = mutation.isPending ?? mutation.isLoading;

  return {
    favoriteUuids,
    canModify: query.data?.can_modify === true,
    isLoading: query.isLoading,
    error: query.error,
    toggleFavorite,
    pendingUuid: pending ? (mutation.variables?.cameraUuid || '') : '',
  };
}
