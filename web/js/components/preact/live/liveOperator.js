import { eptzLayoutSlot } from '../../../utils/eptz-view.js';

export const LIVE_AVAILABILITY_OPTIONS = Object.freeze([
  'live', 'offline', 'never_connected', 'disabled', 'all',
]);

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
    camera_slots: streams.slice(0, columns * rows).map(eptzLayoutSlot),
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
