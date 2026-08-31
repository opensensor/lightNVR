export const LIVE_AVAILABILITY_OPTIONS = Object.freeze([
  'live', 'offline', 'never_connected', 'disabled', 'all',
]);

/**
 * Stable semantic identity for a camera scope. Set identity is not suitable
 * for hook dependencies because equivalent Sets can be reconstructed while
 * async Navigator data is loading.
 */
export function cameraUuidScopeKey(cameraUuids) {
  return [...new Set(cameraUuids || [])].sort().join(',');
}

export function locationDescendantUuids(locations, rootUuid) {
  if (!rootUuid) return new Set();
  const children = new Map();
  for (const location of locations || []) {
    const parent = location.parent_uuid || '';
    if (!children.has(parent)) children.set(parent, []);
    children.get(parent).push(location.uuid);
  }
  const result = new Set();
  const pending = [rootUuid];
  while (pending.length > 0) {
    const uuid = pending.pop();
    if (!uuid || result.has(uuid)) continue;
    result.add(uuid);
    pending.push(...(children.get(uuid) || []));
  }
  return result;
}

export function cameraUuidsForLocation(locations, streams, rootUuid) {
  if (!rootUuid) return new Set((streams || []).map((stream) => stream.camera_uuid));
  const locationUuids = locationDescendantUuids(locations, rootUuid);
  return new Set((streams || [])
    .filter((stream) => locationUuids.has(stream.location_uuid))
    .map((stream) => stream.camera_uuid));
}

export function filterLiveOperatorStreams(streams, cameraUuids, availability) {
  return (streams || []).filter((stream) =>
    (!cameraUuids || cameraUuids.has(stream.camera_uuid)) &&
    (availability === 'all' || stream.availability === availability));
}

/**
 * Build the Navigator inventory before the playback surface removes cameras
 * that cannot currently create a video tile. Disabled cameras remain useful
 * for availability and location inspection; soft-deleted cameras do not.
 */
export function filterLiveNavigatorInventory(streams, cameraUuids = null) {
  return (streams || []).filter((stream) =>
    !stream.is_deleted &&
    (!cameraUuids || cameraUuids.has(stream.camera_uuid)));
}

export function cameraUuidsForLiveLayout(layout) {
  return [...new Set((layout?.camera_slots || [])
    .map((slot) => slot?.camera_uuid)
    .filter(Boolean))];
}

export function buildLocationTree(locations, streams) {
  const byUuid = new Map();
  const roots = [];
  for (const location of locations || []) {
    byUuid.set(location.uuid, { ...location, children: [], directStreams: [], total: 0, live: 0 });
  }
  for (const node of byUuid.values()) {
    const parent = byUuid.get(node.parent_uuid);
    if (parent) parent.children.push(node);
    else roots.push(node);
  }
  for (const stream of streams || []) {
    byUuid.get(stream.location_uuid)?.directStreams.push(stream);
  }
  const aggregate = (node, depth = 0) => {
    node.depth = depth;
    node.children.sort((a, b) =>
      (a.sort_order - b.sort_order) || a.name.localeCompare(b.name));
    node.total = node.directStreams.length;
    node.live = node.directStreams.filter((stream) => stream.availability === 'live').length;
    for (const child of node.children) {
      aggregate(child, depth + 1);
      node.total += child.total;
      node.live += child.live;
    }
    return node;
  };
  return roots
    .sort((a, b) => (a.sort_order - b.sort_order) || a.name.localeCompare(b.name))
    .map((root) => aggregate(root));
}

export function flattenLocationTree(roots, expandedUuids = null) {
  const rows = [];
  const visit = (node) => {
    rows.push(node);
    if (!expandedUuids || expandedUuids.has(node.uuid)) node.children.forEach(visit);
  };
  (roots || []).forEach(visit);
  return rows;
}

export function buildLiveLayoutPayload({
  name,
  isShared = false,
  locationUuid = '',
  availability = 'live',
  columns,
  rows,
  streams = [],
}) {
  return {
    name: name.trim(),
    is_shared: Boolean(isShared),
    location_uuid: locationUuid || null,
    availability: LIVE_AVAILABILITY_OPTIONS.includes(availability) ? availability : 'live',
    columns,
    rows,
    camera_slots: streams.slice(0, columns * rows).map((stream) => ({
      camera_uuid: stream.camera_uuid,
    })),
  };
}

export function buildRecentEventsRequests(cameraUuids, nowSeconds = Math.floor(Date.now() / 1000)) {
  const uniqueUuids = [...new Set(cameraUuids || [])];
  const requests = [];
  for (let offset = 0; offset < uniqueUuids.length; offset += 64) {
    requests.push({
      selector: {
        version: 1,
        expression: { op: 'camera_uuid', values: uniqueUuids.slice(offset, offset + 64) },
      },
      start_time: nowSeconds - 3600,
      end_time: nowSeconds,
      filters: {},
      limit: 20,
      include_summary: false,
    });
  }
  return requests;
}

export function investigationHref(event, cameraUuids = []) {
  const params = new URLSearchParams();
  const scoped = cameraUuids.slice(0, 16);
  if (scoped.length > 0) params.set('cameras', scoped.join(','));
  if (event?.start_time) {
    params.set('start', String(Math.max(1, Math.floor(event.start_time - 60))));
    params.set('end', String(Math.floor(event.start_time + 300)));
  }
  return `/investigation.html?${params.toString()}`;
}

export function clusterFloorPlanCameras(cameras, zoom = 1) {
  const placements = cameras || [];
  if (placements.length < 24 || zoom >= 1.75) {
    return placements.map((camera) => ({ kind: 'camera', ...camera }));
  }
  const cellSize = Math.max(0.025, 0.085 / Math.max(1, zoom));
  const buckets = new Map();
  for (const camera of placements) {
    const key = `${Math.floor(camera.x / cellSize)}:${Math.floor(camera.y / cellSize)}`;
    if (!buckets.has(key)) buckets.set(key, []);
    buckets.get(key).push(camera);
  }
  return [...buckets.values()].map((bucket) => {
    if (bucket.length === 1) return { kind: 'camera', ...bucket[0] };
    return {
      kind: 'cluster',
      camera_uuids: bucket.map((camera) => camera.camera_uuid),
      count: bucket.length,
      x: bucket.reduce((total, camera) => total + camera.x, 0) / bucket.length,
      y: bucket.reduce((total, camera) => total + camera.y, 0) / bucket.length,
    };
  });
}

/**
 * Fit an uploaded background image into a valid plan canvas: the long side
 * targets 2000 px while aspect ratio is preserved and both sides respect the
 * backend CHECK bounds (width 400-4000, height 300-4000). Returns null for
 * aspect ratios that cannot satisfy both constraints.
 */
export function floorPlanCanvasFromImage(width, height) {
  if (!Number.isFinite(width) || !Number.isFinite(height) ||
    !(width > 0) || !(height > 0)) return null;
  const scale = 2000 / Math.max(width, height);
  const lower = Math.max(400 / width, 300 / height);
  const upper = Math.min(4000 / width, 4000 / height);
  if (!Number.isFinite(lower) || !Number.isFinite(upper) || lower > upper) return null;
  const fitted = Math.max(lower, Math.min(upper, scale));
  return {
    canvas_width: Math.round(Math.min(4000, Math.max(400, width * fitted))),
    canvas_height: Math.round(Math.min(4000, Math.max(300, height * fitted))),
  };
}

export function floorPlanPayload(plan, cameras) {
  return {
    name: plan.name,
    location_uuid: plan.location_uuid || null,
    parent_plan_uuid: plan.parent_plan_uuid || null,
    canvas_width: plan.canvas_width,
    canvas_height: plan.canvas_height,
    revision: plan.revision,
    cameras: (cameras || []).map((camera) => ({
      camera_uuid: camera.camera_uuid,
      x: Math.max(0, Math.min(1, camera.x)),
      y: Math.max(0, Math.min(1, camera.y)),
      rotation: Math.max(-180, Math.min(180, camera.rotation || 0)),
      fov: Math.max(1, Math.min(180, camera.fov || 65)),
    })),
  };
}
