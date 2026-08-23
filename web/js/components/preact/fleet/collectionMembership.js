import { useMemo } from 'preact/hooks';
import { fetchJSON, useQuery } from '../../../query-client.js';

const COLLECTION_PAGE_SIZE = 200;

export async function fetchCollectionCameraUuids(collectionUuid, request = fetchJSON, signal) {
  if (!collectionUuid) return [];

  const cameraUuids = [];
  let page = 1;
  let totalPages;

  do {
    const response = await request('/api/fleet/cameras/query', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        collection_uuid: collectionUuid,
        selector: { version: 1, expression: { op: 'all' } },
        page,
        page_size: COLLECTION_PAGE_SIZE,
        sort_by: 'camera_uuid',
        sort_order: 'asc',
        facets: false,
        explain: false,
      }),
      signal,
      timeout: 20000,
      retries: 1,
    });
    (response.cameras || []).forEach((camera) => {
      if (camera.camera_uuid) cameraUuids.push(camera.camera_uuid);
    });
    totalPages = Math.max(1, Number(response.total_pages) || 1);
    page += 1;
  } while (page <= totalPages);

  return [...new Set(cameraUuids)];
}

export function useCollectionMembership(collectionUuid) {
  const collectionsQuery = useQuery(['camera-collections'], '/api/camera-collections', {}, {
    staleTime: 30000,
  });
  const membershipQuery = useQuery({
    queryKey: ['camera-collection-membership', collectionUuid],
    queryFn: ({ signal }) => fetchCollectionCameraUuids(collectionUuid, fetchJSON, signal),
    enabled: Boolean(collectionUuid),
    staleTime: 30000,
  });
  const cameraUuids = useMemo(
    () => new Set(membershipQuery.data || []),
    [membershipQuery.data]
  );

  return {
    collections: collectionsQuery.data?.collections || [],
    cameraUuids,
    isLoading: Boolean(collectionUuid) && membershipQuery.isLoading,
    error: collectionsQuery.error || membershipQuery.error,
  };
}
