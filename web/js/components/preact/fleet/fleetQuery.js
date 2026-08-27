const HEALTH_VALUES = ['unknown', 'up', 'degraded', 'down', 'disabled'];
const RECORDING_VALUES = ['off', 'continuous', 'detection'];
const AVAILABILITY_VALUES = ['all', 'live', 'offline', 'never_connected', 'disabled'];
const SORT_FIELDS = ['name', 'camera_uuid', 'location', 'health', 'enabled', 'recording_mode', 'address'];
const PAGE_SIZES = [25, 50, 100, 200];

export const DEFAULT_FLEET_STATE = Object.freeze({
  search: '',
  health: [],
  enabled: 'all',
  availability: 'all',
  recordingModes: [],
  tagUuids: [],
  locationUuid: '',
  collectionUuid: '',
  page: 1,
  pageSize: 50,
  sortBy: 'name',
  sortOrder: 'asc',
});

function uniqueAllowed(values, allowedValues = null) {
  const seen = new Set();
  return values.filter((value) => {
    if (!value || seen.has(value) || (allowedValues && !allowedValues.includes(value))) {
      return false;
    }
    seen.add(value);
    return true;
  });
}

function parseList(value, allowedValues = null) {
  return uniqueAllowed((value || '').split(',').map((item) => item.trim()), allowedValues);
}

function parsePositiveInteger(value, fallback) {
  const parsed = Number.parseInt(value, 10);
  return Number.isInteger(parsed) && parsed > 0 ? parsed : fallback;
}

export function readFleetUrlState(search = '') {
  const params = new URLSearchParams(search);
  const pageSize = parsePositiveInteger(params.get('size'), DEFAULT_FLEET_STATE.pageSize);
  const enabled = params.get('enabled');
  const availability = params.get('availability');
  const sortBy = params.get('sort');
  const sortOrder = params.get('order');

  return {
    search: (params.get('q') || '').slice(0, 255),
    health: parseList(params.get('health'), HEALTH_VALUES),
    enabled: enabled === 'true' || enabled === 'false' ? enabled : 'all',
    availability: AVAILABILITY_VALUES.includes(availability) ? availability : 'all',
    recordingModes: parseList(params.get('recording'), RECORDING_VALUES),
    tagUuids: parseList(params.get('tags')),
    locationUuid: params.get('location') || '',
    collectionUuid: params.get('collection') || '',
    page: parsePositiveInteger(params.get('page'), DEFAULT_FLEET_STATE.page),
    pageSize: PAGE_SIZES.includes(pageSize) ? pageSize : DEFAULT_FLEET_STATE.pageSize,
    sortBy: SORT_FIELDS.includes(sortBy) ? sortBy : DEFAULT_FLEET_STATE.sortBy,
    sortOrder: sortOrder === 'desc' ? 'desc' : DEFAULT_FLEET_STATE.sortOrder,
  };
}

export function writeFleetUrlState(url, state) {
  const nextUrl = new URL(url);
  const setOrDelete = (key, value, defaultValue = '') => {
    if (value !== defaultValue && value !== '' && value !== null && value !== undefined) {
      nextUrl.searchParams.set(key, String(value));
    } else {
      nextUrl.searchParams.delete(key);
    }
  };

  setOrDelete('q', state.search.trim());
  setOrDelete('health', state.health.join(','));
  setOrDelete('enabled', state.enabled, 'all');
  setOrDelete('availability', state.availability, 'all');
  setOrDelete('recording', state.recordingModes.join(','));
  setOrDelete('tags', state.tagUuids.join(','));
  setOrDelete('location', state.locationUuid);
  setOrDelete('collection', state.collectionUuid);
  setOrDelete('page', state.page, 1);
  setOrDelete('size', state.pageSize, DEFAULT_FLEET_STATE.pageSize);
  setOrDelete('sort', state.sortBy, DEFAULT_FLEET_STATE.sortBy);
  setOrDelete('order', state.sortOrder, DEFAULT_FLEET_STATE.sortOrder);
  return nextUrl;
}

export function buildFleetSelector(state) {
  const children = [];

  if (state.locationUuid) {
    children.push({ op: 'location_subtree', uuid: state.locationUuid });
  }
  if (state.tagUuids.length > 0) {
    children.push({ op: 'tag_any', uuids: uniqueAllowed(state.tagUuids) });
  }
  if (state.health.length > 0) {
    children.push({ op: 'health', values: uniqueAllowed(state.health, HEALTH_VALUES) });
  }
  if (state.enabled === 'true' || state.enabled === 'false') {
    children.push({ op: 'enabled', value: state.enabled === 'true' });
  }
  if (state.recordingModes.length > 0) {
    children.push({ op: 'recording_mode', values: uniqueAllowed(state.recordingModes, RECORDING_VALUES) });
  }

  let expression = { op: 'all' };
  if (children.length === 1) {
    [expression] = children;
  } else if (children.length > 1) {
    expression = { op: 'and', children };
  }

  return { version: 1, expression };
}

export function buildFleetQueryRequest(state, search = state.search) {
  const request = {
    selector: buildFleetSelector(state),
    search: search.trim(),
    page: state.page,
    page_size: state.pageSize,
    sort_by: state.sortBy,
    sort_order: state.sortOrder,
    facets: true,
    explain: false,
    availability: AVAILABILITY_VALUES.includes(state.availability)
      ? state.availability : 'all',
  };
  if (state.collectionUuid) request.collection_uuid = state.collectionUuid;
  return request;
}

function selectorRules(selector) {
  if (selector?.version !== 1 || !selector.expression) return null;
  if (selector.expression.op === 'all') return [];
  return selector.expression.op === 'and' && Array.isArray(selector.expression.children)
    ? selector.expression.children
    : [selector.expression];
}

export function fleetStateFromSavedView(view, current = DEFAULT_FLEET_STATE) {
  const rules = selectorRules(view?.selector);
  if (!rules) return null;
  const next = {
    ...DEFAULT_FLEET_STATE,
    pageSize: current.pageSize,
    search: typeof view.search === 'string' ? view.search : '',
    collectionUuid: typeof view.collection_uuid === 'string' ? view.collection_uuid : '',
    sortBy: SORT_FIELDS.includes(view.sort_by) ? view.sort_by : DEFAULT_FLEET_STATE.sortBy,
    sortOrder: view.sort_order === 'desc' ? 'desc' : 'asc',
  };
  for (const rule of rules) {
    if (rule.op === 'location_subtree' && typeof rule.uuid === 'string') {
      next.locationUuid = rule.uuid;
    } else if (rule.op === 'tag_any' && Array.isArray(rule.uuids)) {
      next.tagUuids = uniqueAllowed(rule.uuids);
    } else if (rule.op === 'health' && Array.isArray(rule.values)) {
      next.health = uniqueAllowed(rule.values, HEALTH_VALUES);
    } else if (rule.op === 'enabled' && typeof rule.value === 'boolean') {
      next.enabled = String(rule.value);
    } else if (rule.op === 'recording_mode' && Array.isArray(rule.values)) {
      next.recordingModes = uniqueAllowed(rule.values, RECORDING_VALUES);
    } else {
      return null;
    }
  }
  return next;
}

export function buildFleetSavedViewPayload(name, isShared, state) {
  return {
    name: name.trim(),
    is_shared: Boolean(isShared),
    selector: buildFleetSelector(state),
    search: state.search.trim(),
    collection_uuid: state.collectionUuid || '',
    columns: ['camera', 'health', 'location', 'tags', 'recording', 'actions'],
    sort_by: state.sortBy,
    sort_order: state.sortOrder,
  };
}

export function fleetStateFromOperationalQueue(queue, current = DEFAULT_FLEET_STATE) {
  return fleetStateFromSavedView({
    selector: queue?.selector,
    search: '',
    collection_uuid: '',
    sort_by: 'health',
    sort_order: 'desc',
  }, current);
}

export function toggleFleetValue(values, value) {
  return values.includes(value)
    ? values.filter((item) => item !== value)
    : [...values, value];
}

export function countFleetFilters(state) {
  return state.health.length + state.recordingModes.length + state.tagUuids.length +
    (state.enabled === 'all' ? 0 : 1) + (state.availability === 'all' ? 0 : 1) +
    (state.locationUuid ? 1 : 0) +
    (state.collectionUuid ? 1 : 0);
}

export function facetCount(facets, group, value) {
  return facets?.[group]?.find((item) => String(item.value) === String(value))?.count || 0;
}

export function clampFleetPage(page, totalPages) {
  if (totalPages < 1) return 1;
  return Math.min(Math.max(1, page), totalPages);
}

export { AVAILABILITY_VALUES, HEALTH_VALUES, RECORDING_VALUES, SORT_FIELDS, PAGE_SIZES };
