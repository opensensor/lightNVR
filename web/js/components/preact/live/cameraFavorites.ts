import { fleetWriteJSON } from '../../../api/fleetWrites.ts';
/**
 * Per-user favorite cameras for the Live view.
 *
 * Backed by GET/PUT/DELETE /api/camera-favorites. The pure helpers are
 * exported for unit tests; useCameraFavorites wires them to the query cache.
 */
import { useCallback, useMemo } from 'preact/hooks';
import { useMutation, useQuery, useQueryClient } from '../../../query-client.ts';
import type { EnhancedFetchOptions } from '../../../fetch-utils.ts';
import type {
  CameraFavoriteChange,
  CameraFavoriteChangeResult,
  CameraFavoritesResponse,
  LiveStream,
} from '../../../types/live.ts';

type FavoriteRequest = (url: string, options: EnhancedFetchOptions) => Promise<CameraFavoriteChangeResult>;

export const CAMERA_FAVORITES_QUERY_KEY = ['camera-favorites'];

const EMPTY_FAVORITES: Readonly<CameraFavoritesResponse> = Object.freeze({ favorites: [], can_modify: false });

export function favoriteCameraUuids(data?: CameraFavoritesResponse | null): Set<string> {
  return new Set((data?.favorites || []).map((favorite) => favorite.camera_uuid));
}

export function filterFavoriteStreams(streams: LiveStream[] | null | undefined, favoriteUuids: Set<string>): LiveStream[] {
  return (streams || []).filter((stream) => favoriteUuids.has(stream.camera_uuid));
}

export function favoriteEmptyStateKey(favoritesFilterActive: boolean, favoriteUuids: Set<string>): string {
  return favoritesFilterActive && favoriteUuids.size === 0
    ? 'live.noFavorites'
    : 'live.navigator.noCameras';
}

/**
 * Next cached favorites document after a toggle resolves. `record` is the
 * server's row for an add, or null for a removal. Keeps oldest-first order.
 */
export function applyFavoriteChange(data: CameraFavoritesResponse | null | undefined, cameraUuid: string, record: CameraFavoriteChangeResult | null): CameraFavoritesResponse {
  const favorites = (data?.favorites || []).filter((favorite) => favorite.camera_uuid !== cameraUuid);
  if (record) favorites.push({ camera_uuid: cameraUuid, ...record });
  favorites.sort((a, b) =>
    (Number(a.created_at) - Number(b.created_at)) || String(a.camera_uuid).localeCompare(String(b.camera_uuid)));
  return { ...(data || EMPTY_FAVORITES), favorites };
}

export function requestFavoriteChange(cameraUuid: string, favorite: boolean, request: FavoriteRequest = fleetWriteJSON): Promise<CameraFavoriteChangeResult> {
  const url = `/api/camera-favorites/${encodeURIComponent(cameraUuid)}`;
  return request(url, { method: favorite ? 'PUT' : 'DELETE', retries: 0 });
}

export function useCameraFavorites() {
  const queryClient = useQueryClient();
  const query = useQuery<CameraFavoritesResponse>({queryKey: CAMERA_FAVORITES_QUERY_KEY, queryFn: ({signal}) => fleetWriteJSON<CameraFavoritesResponse>('/api/camera-favorites', {signal}),
    staleTime: 30000,
    retry: false,
  });
  const favoriteUuids = useMemo(() => favoriteCameraUuids(query.data), [query.data]);
  const mutation = useMutation<CameraFavoriteChangeResult, Error, CameraFavoriteChange>({
    mutationFn: ({ cameraUuid, favorite }) => requestFavoriteChange(cameraUuid, favorite),
    // Update after server confirmation so a failed write leaves the grid intact.
    onSuccess: (result, { cameraUuid, favorite }) => {
      queryClient.setQueryData<CameraFavoritesResponse>(CAMERA_FAVORITES_QUERY_KEY,
        (current) => applyFavoriteChange(current, cameraUuid, favorite ? result : null));
    },
    onSettled: () => queryClient.invalidateQueries({ queryKey: CAMERA_FAVORITES_QUERY_KEY }),
  });
  const toggleFavorite = useCallback(
    (cameraUuid: string) => mutation.mutateAsync({ cameraUuid, favorite: !favoriteUuids.has(cameraUuid) }),
    [mutation, favoriteUuids]
  );
  const pending = mutation.isLoading;

  return {
    favoriteUuids,
    canModify: query.data?.can_modify === true,
    isLoading: query.isLoading,
    error: query.error,
    toggleFavorite,
    pendingUuid: pending ? (mutation.variables?.cameraUuid || '') : '',
  };
}
