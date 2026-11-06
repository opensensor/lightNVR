import { listCameraCollections, queryFleetCameras } from '../../../api/fleet.ts';
import { useMemo } from 'preact/hooks';
import { useQuery } from '../../../query-client.ts';
import type {
  CameraCollection,
  CameraCollectionsResponse,
  FleetQueryResponse,
} from '../../../types/fleet.ts';

const COLLECTION_PAGE_SIZE = 200;

export async function fetchCollectionCameraUuids(collectionUuid: string | null | undefined, request: (body: unknown, options: import('../../../api/fleet.ts').FleetRequestOptions) => Promise<FleetQueryResponse> = queryFleetCameras, signal?: AbortSignal): Promise<string[]> {
  if (!collectionUuid) return [];

  const cameraUuids: string[] = [];
  let page = 1;
  let totalPages: number;

  do {
    const response = await request({
      collection_uuid: collectionUuid,
      selector: {version: 1, expression: {op: 'all'}},
      page, page_size: COLLECTION_PAGE_SIZE, sort_by: 'camera_uuid',
      sort_order: 'asc', facets: false, explain: false,
    }, {signal, timeout: 20000, retries: 1});
    (response.cameras || []).forEach((camera) => {
      if (camera.camera_uuid) cameraUuids.push(camera.camera_uuid);
    });
    totalPages = Math.max(1, Number(response.total_pages) || 1);
    page += 1;
  } while (page <= totalPages);

  return [...new Set(cameraUuids)];
}

export interface CollectionMembership {
  collections: CameraCollection[];
  cameraUuids: Set<string>;
  isLoading: boolean;
  error: Error | null;
}

export function useCollectionMembership(collectionUuid: string | null | undefined): CollectionMembership {
  const collectionsQuery = useQuery<CameraCollectionsResponse>({queryKey: ['camera-collections'], queryFn: ({signal}) => listCameraCollections({signal}), staleTime: 30000 });
  const membershipQuery = useQuery<string[]>({
    queryKey: ['camera-collection-membership', collectionUuid],
    queryFn: ({ signal }) => fetchCollectionCameraUuids(collectionUuid, undefined, signal),
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
