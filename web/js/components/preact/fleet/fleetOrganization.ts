import { getCameraTagAssignments } from '../../../api/fleet.ts';
import type {
  BoundedResult,
  BulkOrganizationChanges,
  BulkOrganizationFailure,
  BulkOrganizationResult,
  JsonFetcher,
  LocationLike,
  LocationRow,
} from '../../../types/fleet.ts';

const MAX_CAMERA_TAGS = 64;

function compareLocations(left: LocationLike, right: LocationLike): number {
  const order = (left.sort_order || 0) - (right.sort_order || 0);
  return order || (left.name || '').localeCompare(right.name || '');
}

export function buildLocationRows<T extends LocationLike>(locations: T[] = []): LocationRow<T>[] {
  const childrenByParent = new Map<string | null, T[]>();
  const byUuid = new Map(locations.map((location) => [location.uuid, location]));
  for (const location of locations) {
    const parentUuid = location.parent_uuid && byUuid.has(location.parent_uuid)
      ? location.parent_uuid
      : null;
    const siblings = childrenByParent.get(parentUuid) || [];
    siblings.push(location);
    childrenByParent.set(parentUuid, siblings);
  }
  for (const children of childrenByParent.values()) children.sort(compareLocations);

  const rows: LocationRow<T>[] = [];
  const visited = new Set<string>();
  const visit = (location: T, depth: number, parentPath: string) => {
    if (visited.has(location.uuid)) return;
    visited.add(location.uuid);
    const path = parentPath ? `${parentPath} / ${location.name}` : location.name;
    rows.push({ ...location, depth, path });
    for (const child of childrenByParent.get(location.uuid) || []) {
      visit(child, depth + 1, path);
    }
  };

  for (const root of childrenByParent.get(null) || []) visit(root, 0, '');
  for (const location of [...locations].sort(compareLocations)) {
    if (!visited.has(location.uuid)) visit(location, 0, '');
  }
  return rows;
}

export function getLocationDescendantUuids(locations: LocationLike[], locationUuid: string): Set<string> {
  const childrenByParent = new Map<string | null | undefined, string[]>();
  for (const location of locations) {
    const children = childrenByParent.get(location.parent_uuid) || [];
    children.push(location.uuid);
    childrenByParent.set(location.parent_uuid, children);
  }
  const descendants = new Set([locationUuid]);
  const queue = [locationUuid];
  while (queue.length > 0) {
    const current = queue.shift();
    for (const childUuid of childrenByParent.get(current) || []) {
      if (!descendants.has(childUuid)) {
        descendants.add(childUuid);
        queue.push(childUuid);
      }
    }
  }
  return descendants;
}

export function locationParentOptions<T extends LocationLike>(locations: T[], currentUuid = ''): LocationRow<T>[] {
  const excluded = currentUuid ? getLocationDescendantUuids(locations, currentUuid) : new Set<string>();
  return buildLocationRows(locations).filter((location) => !excluded.has(location.uuid));
}

export function resolveTagAssignments(existingUuids: string[], operation: string, selectedUuids?: string[]): string[] {
  const existing = [...new Set(existingUuids)];
  const selected = [...new Set(selectedUuids)];
  if (operation === 'replace') return selected;
  if (operation === 'add') return [...new Set([...existing, ...selected])];
  if (operation === 'remove') {
    const removals = new Set(selected);
    return existing.filter((uuid) => !removals.has(uuid));
  }
  return existing;
}

export async function runBounded<T, R>(items: T[], worker: (item: T, index: number) => Promise<R>, concurrency = 5): Promise<BoundedResult<R>[]> {
  const results: BoundedResult<R>[] = new Array(items.length);
  let nextIndex = 0;
  const runners = Array.from({ length: Math.min(concurrency, items.length) }, async () => {
    while (nextIndex < items.length) {
      const index = nextIndex++;
      try {
        results[index] = { status: 'fulfilled', value: await worker(items[index]!, index) };
      } catch (reason) {
        results[index] = { status: 'rejected', reason };
      }
    }
  });
  await Promise.all(runners);
  return results;
}

export async function applyBulkOrganization<T extends { camera_uuid: string }>(
  cameras: T[],
  changes: BulkOrganizationChanges,
  fetcher: JsonFetcher,
  onProgress: (completed: number, total: number) => void = () => {},
  readTags: (uuid: string) => Promise<{tags: {uuid: string}[]}> = uuid => getCameraTagAssignments(uuid, {timeout: 15000, retries: 1}),
): Promise<BulkOrganizationResult<T>> {
  let completed = 0;
  const results = await runBounded(cameras, async (camera) => {
    try {
      if (changes.locationUuid) {
        await fetcher(`/api/cameras/${encodeURIComponent(camera.camera_uuid)}/location`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ location_uuid: changes.locationUuid }),
          timeout: 15000,
          retries: 1,
        });
      }

      if (changes.tagOperation && changes.tagOperation !== 'none') {
        const current = await readTags(camera.camera_uuid);
        const nextUuids = resolveTagAssignments(
          (current.tags || []).map((tag) => tag.uuid),
          changes.tagOperation,
          changes.tagUuids
        );
        if (nextUuids.length > MAX_CAMERA_TAGS) {
          throw new Error(`Camera would exceed the ${MAX_CAMERA_TAGS}-tag limit`);
        }
        await fetcher(`/api/cameras/${encodeURIComponent(camera.camera_uuid)}/tags`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ tag_uuids: nextUuids }),
          timeout: 15000,
          retries: 1,
        });
      }
      return camera;
    } finally {
      completed += 1;
      onProgress(completed, cameras.length);
    }
  }, 5);

  const succeeded: T[] = [];
  const failed: BulkOrganizationFailure<T>[] = [];
  results.forEach((result, index) => {
    const camera = cameras[index]!;
    if (result.status === 'fulfilled') {
      succeeded.push(camera);
    } else {
      failed.push({ camera, error: (result.reason as Error | null | undefined)?.message || String(result.reason) });
    }
  });
  return { succeeded, failed };
}

export { MAX_CAMERA_TAGS };
