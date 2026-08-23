const HEALTH_VALUES = ['unknown', 'up', 'degraded', 'down', 'disabled'];
const RECORDING_VALUES = ['off', 'continuous', 'detection'];
const SORT_FIELDS = ['name', 'camera_uuid', 'location', 'health', 'enabled', 'recording_mode', 'address'];
const PAGE_SIZES = [25, 50, 100, 200];

export const DEFAULT_FLEET_STATE = Object.freeze({
  search: '',
  health: [],
  enabled: 'all',
  recordingModes: [],
  tagUuids: [],
  locationUuid: '',
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
  const sortBy = params.get('sort');
  const sortOrder = params.get('order');

  return {
    search: (params.get('q') || '').slice(0, 255),
    health: parseList(params.get('health'), HEALTH_VALUES),
    enabled: enabled === 'true' || enabled === 'false' ? enabled : 'all',
    recordingModes: parseList(params.get('recording'), RECORDING_VALUES),
    tagUuids: parseList(params.get('tags')),
    locationUuid: params.get('location') || '',
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
  setOrDelete('recording', state.recordingModes.join(','));
  setOrDelete('tags', state.tagUuids.join(','));
  setOrDelete('location', state.locationUuid);
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
  return {
    selector: buildFleetSelector(state),
    search: search.trim(),
    page: state.page,
    page_size: state.pageSize,
    sort_by: state.sortBy,
    sort_order: state.sortOrder,
    facets: true,
    explain: false,
  };
}

export function toggleFleetValue(values, value) {
  return values.includes(value)
    ? values.filter((item) => item !== value)
    : [...values, value];
}

export function countFleetFilters(state) {
  return state.health.length + state.recordingModes.length + state.tagUuids.length +
    (state.enabled === 'all' ? 0 : 1) + (state.locationUuid ? 1 : 0);
}

export function facetCount(facets, group, value) {
  return facets?.[group]?.find((item) => String(item.value) === String(value))?.count || 0;
}

export function clampFleetPage(page, totalPages) {
  if (totalPages < 1) return 1;
  return Math.min(Math.max(1, page), totalPages);
}

export { HEALTH_VALUES, RECORDING_VALUES, SORT_FIELDS, PAGE_SIZES };
