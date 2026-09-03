export const ALL_CAMERAS_SELECTOR = {
  version: 1,
  expression: { op: 'all' },
};

export const DEFAULT_TOPIC_TEMPLATE = 'lightnvr/v1/events/{type}/{subject_id}';
export const HEALTH_EVENT_FAMILY = 'system_health';
export const HEALTH_SEVERITY_ORDER = ['warning', 'error', 'critical'];

export const EMPTY_DESTINATION_DRAFT = {
  name: '',
  description: '',
  enabled: true,
  host: '',
  port: 8883,
  clientId: '',
  topicTemplate: DEFAULT_TOPIC_TEMPLATE,
  statusTopicTemplate: '',
  keepaliveSeconds: 60,
  qos: 1,
  username: '',
  password: '',
  clearPassword: false,
  passwordConfigured: false,
  tlsMode: 'system',
  caFile: '',
  certFile: '',
  keyFile: '',
  revision: 0,
  uuid: '',
};

export const EMPTY_ROUTE_DRAFT = {
  name: '',
  description: '',
  enabled: true,
  destination: 'mqtt:default',
  eventTypes: [],
  scopeType: 'all',
  selector: ALL_CAMERAS_SELECTOR,
  selectorError: '',
  detectionFilterEnabled: false,
  labels: '',
  zones: '',
  minConfidence: '',
  healthFilterEnabled: false,
  healthConditionCodes: [],
  healthSeverities: [],
  scheduleEnabled: false,
  timezone: 'UTC',
  windows: [],
  debounceSeconds: 0,
  cooldownSeconds: 0,
  groupingWindowSeconds: 0,
  maxEventsPerMinute: 0,
  revision: 0,
  uuid: '',
};

export const DEFAULT_SCHEDULE_WINDOW = {
  days: [1, 2, 3, 4, 5],
  start: '18:00',
  end: '06:00',
};

export const EVENT_SECTIONS = new Set(['routes', 'destinations', 'catalog']);

export function createDestinationDraft(randomUuid) {
  const uuid = randomUuid || (globalThis.crypto?.randomUUID?.() ?? 'events-client');
  const suffix = String(uuid).replace(/[^a-zA-Z0-9]/g, '').slice(0, 12).toLowerCase();
  return { ...EMPTY_DESTINATION_DRAFT, clientId: `lightnvr-${suffix || 'events-client'}` };
}

function uniqueStrings(values) {
  return [...new Set((values || []).map((value) => String(value).trim()).filter(Boolean))];
}

export function splitList(value) {
  return uniqueStrings(String(value || '').split(/[\n,]/));
}

function integer(value, fallback = 0) {
  const number = Number(value);
  return Number.isInteger(number) ? number : fallback;
}

export function destinationToDraft(destination) {
  if (!destination) return { ...EMPTY_DESTINATION_DRAFT };
  return {
    ...EMPTY_DESTINATION_DRAFT,
    name: destination.name || '',
    description: destination.description || '',
    enabled: destination.enabled !== false,
    host: destination.broker?.host || '',
    port: destination.broker?.port ?? 8883,
    clientId: destination.broker?.client_id || '',
    topicTemplate: destination.broker?.topic_template || DEFAULT_TOPIC_TEMPLATE,
    statusTopicTemplate: destination.broker?.status_topic_template || '',
    keepaliveSeconds: destination.broker?.keepalive_seconds ?? 60,
    qos: destination.broker?.qos ?? 1,
    username: destination.authentication?.username || '',
    passwordConfigured: destination.authentication?.password_configured === true,
    tlsMode: destination.tls?.mode || 'system',
    caFile: destination.tls?.ca_file || '',
    certFile: destination.tls?.cert_file || '',
    keyFile: destination.tls?.key_file || '',
    revision: destination.revision || 0,
    uuid: destination.uuid || '',
  };
}

export function buildDestinationPayload(draft, creating = false) {
  const authentication = { username: String(draft.username || '').trim() };
  if (draft.clearPassword) authentication.password = null;
  else if (draft.password) authentication.password = draft.password;

  // The API applies omitted TLS fields as partial updates. Always send the
  // complete TLS shape so changing modes also clears certificates that are no
  // longer valid for the selected mode.
  const tls = { mode: draft.tlsMode, ca_file: '', cert_file: '', key_file: '' };
  if (draft.tlsMode === 'custom_ca' || draft.tlsMode === 'mutual') {
    tls.ca_file = String(draft.caFile || '').trim();
  }
  if (draft.tlsMode === 'mutual') {
    tls.cert_file = String(draft.certFile || '').trim();
    tls.key_file = String(draft.keyFile || '').trim();
  }

  const payload = {
    name: String(draft.name || '').trim(),
    description: String(draft.description || '').trim(),
    enabled: draft.enabled === true,
    type: 'mqtt',
    broker: {
      host: String(draft.host || '').trim(),
      port: integer(draft.port, 8883),
      client_id: String(draft.clientId || '').trim(),
      topic_template: String(draft.topicTemplate || '').trim(),
      status_topic_template: String(draft.statusTopicTemplate || '').trim(),
      keepalive_seconds: integer(draft.keepaliveSeconds, 60),
      qos: integer(draft.qos, 1),
    },
    authentication,
    tls,
  };
  if (!creating) payload.revision = integer(draft.revision);
  return payload;
}

export function validateDestinationDraft(draft) {
  if (!String(draft.name || '').trim() || String(draft.name).length >= 128 ||
      String(draft.description || '').length >= 512) return 'name';
  const host = String(draft.host || '').trim();
  if (!host || host.length >= 256 || host.includes('://') || /[\s/\\?#]/.test(host)) return 'host';
  if (!String(draft.clientId || '').trim() || String(draft.clientId).length >= 128) return 'client_id';
  const topic = String(draft.topicTemplate || '').trim();
  const expandedTopic = topic
    .split('{type}').join('x'.repeat(95))
    .split('{subject_id}').join('x'.repeat(95));
  if (topic.length >= 512 || !topic.includes('{type}') || !topic.includes('{subject_id}') ||
      /[+#{]/.test(expandedTopic) || expandedTopic.includes('}') ||
      topic.startsWith('/') || topic.endsWith('/') || expandedTopic.length >= 512) return 'topic';
  const statusTopic = String(draft.statusTopicTemplate || '').trim();
  const expandedStatusTopic = statusTopic
    .split('{installation_uuid}').join('x'.repeat(36))
    .split('{destination_uuid}').join('x'.repeat(36));
  if (statusTopic && (statusTopic.length >= 512 ||
      !statusTopic.includes('{installation_uuid}') ||
      /[+#{]/.test(expandedStatusTopic) || expandedStatusTopic.includes('}') ||
      statusTopic.startsWith('/') || statusTopic.endsWith('/') ||
      expandedStatusTopic.length >= 512)) return 'status_topic';
  const port = Number(draft.port);
  if (!Number.isInteger(port) || port < 1 || port > 65535) return 'port';
  const keepalive = Number(draft.keepaliveSeconds);
  if (!Number.isInteger(keepalive) || keepalive < 5 || keepalive > 3600) return 'keepalive';
  if (![0, 1, 2].includes(Number(draft.qos))) return 'qos';
  if (String(draft.username || '').length >= 128 || String(draft.password || '').length >= 256) return 'username';
  if ((draft.password || draft.passwordConfigured) && !String(draft.username || '').trim() && !draft.clearPassword) return 'username';
  const caFile = String(draft.caFile || '').trim();
  const certFile = String(draft.certFile || '').trim();
  const keyFile = String(draft.keyFile || '').trim();
  if ((draft.tlsMode === 'custom_ca' || draft.tlsMode === 'mutual') &&
      (!caFile.startsWith('/') || caFile.length >= 512)) return 'ca_file';
  if (draft.tlsMode === 'mutual' &&
      (!certFile.startsWith('/') || !keyFile.startsWith('/') ||
       certFile.length >= 512 || keyFile.length >= 512)) return 'mutual_files';
  return '';
}

export function routeToDraft(route) {
  if (!route) return { ...EMPTY_ROUTE_DRAFT, selector: ALL_CAMERAS_SELECTOR, windows: [] };
  const detection = route.predicate?.detection;
  const health = route.predicate?.health;
  const windows = Array.isArray(route.schedule?.windows)
    ? route.schedule.windows.map((window) => ({ ...window, days: [...(window.days || [])] }))
    : [];
  return {
    ...EMPTY_ROUTE_DRAFT,
    name: route.name || '',
    description: route.description || '',
    enabled: route.enabled !== false,
    destination: route.destination || 'mqtt:default',
    eventTypes: [...(route.event_types || [])],
    scopeType: route.camera_scope?.type || 'all',
    selector: route.camera_scope?.selector || ALL_CAMERAS_SELECTOR,
    detectionFilterEnabled: Boolean(detection),
    labels: (detection?.labels_any || []).join(', '),
    zones: (detection?.zone_ids_any || []).join(', '),
    minConfidence: detection?.min_confidence ?? '',
    healthFilterEnabled: Boolean(health),
    healthConditionCodes: [...(health?.condition_codes_any || [])],
    healthSeverities: [...(health?.severities_any || [])],
    scheduleEnabled: windows.length > 0,
    timezone: route.schedule?.timezone || 'UTC',
    windows,
    debounceSeconds: route.suppression?.debounce_seconds ?? 0,
    cooldownSeconds: route.suppression?.cooldown_seconds ?? 0,
    groupingWindowSeconds: route.suppression?.grouping_window_seconds ?? 0,
    maxEventsPerMinute: route.suppression?.max_events_per_minute ?? 0,
    revision: route.revision || 0,
    uuid: route.uuid || '',
  };
}

export function buildRoutePayload(draft, creating = false) {
  const predicate = { version: 1 };
  if (draft.detectionFilterEnabled) {
    const detection = {};
    const labels = splitList(draft.labels);
    const zones = splitList(draft.zones);
    if (labels.length) detection.labels_any = labels;
    if (zones.length) detection.zone_ids_any = zones;
    if (draft.minConfidence !== '' && draft.minConfidence !== null) {
      detection.min_confidence = Number(draft.minConfidence);
    }
    predicate.detection = detection;
  }
  if (draft.healthFilterEnabled) {
    const health = {};
    const conditions = uniqueStrings(draft.healthConditionCodes);
    const severities = uniqueStrings(draft.healthSeverities);
    if (conditions.length) health.condition_codes_any = conditions;
    if (severities.length) health.severities_any = severities;
    predicate.health = health;
  }
  const payload = {
    name: String(draft.name || '').trim(),
    description: String(draft.description || '').trim(),
    enabled: draft.enabled === true,
    destination: draft.destination,
    event_types: uniqueStrings(draft.eventTypes),
    camera_scope: draft.scopeType === 'selector'
      ? { type: 'selector', selector: draft.selector }
      : { type: 'all' },
    predicate,
    schedule: {
      version: 1,
      timezone: String(draft.timezone || 'UTC').trim(),
      windows: draft.scheduleEnabled
        ? (draft.windows || []).map((window) => ({
            days: [...(window.days || [])].map(Number).sort((a, b) => a - b),
            start: window.start,
            end: window.end,
          }))
        : [],
    },
    suppression: {
      debounce_seconds: integer(draft.debounceSeconds),
      cooldown_seconds: integer(draft.cooldownSeconds),
      grouping_window_seconds: integer(draft.groupingWindowSeconds),
      max_events_per_minute: integer(draft.maxEventsPerMinute),
    },
  };
  if (!creating) payload.revision = integer(draft.revision);
  return payload;
}

const CLOCK_PATTERN = /^([01]\d|2[0-3]):[0-5]\d$/;

export function validateRouteDraft(draft, registries = {}) {
  if (!String(draft.name || '').trim()) return 'name';
  if (!draft.destination) return 'destination';
  if (!Array.isArray(draft.eventTypes) || draft.eventTypes.length === 0) return 'event_types';
  if (draft.scopeType === 'selector' && (!draft.selector || draft.selectorError)) return 'selector';
  const catalog = Array.isArray(registries.catalog) ? registries.catalog : [];
  const selectedDefinitions = draft.eventTypes
    .map((type) => catalog.find((entry) => entry.type === type))
    .filter(Boolean);
  if (draft.scopeType === 'selector' && selectedDefinitions.some(
    (entry) => entry.subject_kind !== 'camera')) return 'system_scope';
  if (draft.detectionFilterEnabled) {
    if (!draft.eventTypes.includes('io.lightnvr.detection.object.v1')) return 'detection_type';
    const confidence = draft.minConfidence === '' ? null : Number(draft.minConfidence);
    if (!splitList(draft.labels).length && !splitList(draft.zones).length && confidence === null) return 'detection_filter';
    if (confidence !== null && (!Number.isFinite(confidence) || confidence < 0 || confidence > 1)) return 'confidence';
  }
  if (draft.healthFilterEnabled) {
    if (draft.detectionFilterEnabled) return 'predicate_mix';
    if (selectedDefinitions.length !== draft.eventTypes.length ||
        selectedDefinitions.some((entry) => entry.family !== HEALTH_EVENT_FAMILY))
      return 'health_type';
    const conditions = uniqueStrings(draft.healthConditionCodes);
    const severities = uniqueStrings(draft.healthSeverities);
    if (!conditions.length && !severities.length) return 'health_filter';
    const conditionRegistry = new Set(
      (registries.healthConditions || []).map((entry) => entry.code || entry.condition));
    if (conditions.some((code) => !conditionRegistry.has(code)))
      return 'health_condition';
    const severityRegistry = new Set(registries.healthSeverities || []);
    if (severities.some((severity) => !severityRegistry.has(severity)))
      return 'health_severity';
  }
  if (!String(draft.timezone || '').trim()) return 'timezone';
  if (draft.scheduleEnabled) {
    if (!draft.windows?.length) return 'schedule_window';
    const invalidWindow = draft.windows.some((window) =>
      !window.days?.length || !CLOCK_PATTERN.test(window.start || '') ||
      !CLOCK_PATTERN.test(window.end || '') || window.start === window.end
    );
    if (invalidWindow) return 'schedule_window';
  }
  const limits = [
    ['debounceSeconds', 0, 86400],
    ['cooldownSeconds', 0, 604800],
    ['groupingWindowSeconds', 0, 3600],
    ['maxEventsPerMinute', 0, 60000],
  ];
  if (limits.some(([field, minimum, maximum]) => {
    const value = Number(draft[field]);
    return !Number.isInteger(value) || value < minimum || value > maximum;
  })) return 'suppression';
  return '';
}

export function healthConditionsFromSettings(settings) {
  const conditions = settings?.health_effective_policy?.conditions;
  if (!Array.isArray(conditions)) return [];
  const seen = new Set();
  return conditions.filter((condition) => {
    const code = condition?.code || condition?.condition;
    if (!code || seen.has(code)) return false;
    seen.add(code);
    return true;
  }).map((condition) => ({ ...condition, code: condition.code || condition.condition }));
}

export function healthSeveritiesFromCatalog(catalog, selectedTypes = []) {
  const selected = new Set(selectedTypes);
  const healthTypes = (catalog || []).filter((entry) =>
    entry.family === HEALTH_EVENT_FAMILY &&
    (selected.size === 0 || selected.has(entry.type)));
  const allowed = new Set();
  healthTypes.forEach((entry) => {
    const minimum = HEALTH_SEVERITY_ORDER.indexOf(entry.minimum_severity);
    const maximum = HEALTH_SEVERITY_ORDER.indexOf(entry.maximum_severity);
    if (minimum < 0 || maximum < minimum) return;
    HEALTH_SEVERITY_ORDER.slice(minimum, maximum + 1)
      .forEach((severity) => allowed.add(severity));
  });
  return HEALTH_SEVERITY_ORDER.filter((severity) => allowed.has(severity));
}

export function groupCatalog(eventTypes) {
  const groups = new Map();
  (eventTypes || []).forEach((eventType) => {
    const family = eventType.family || 'other';
    if (!groups.has(family)) groups.set(family, []);
    groups.get(family).push(eventType);
  });
  return [...groups.entries()]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([family, types]) => ({ family, types: types.sort((left, right) => left.type.localeCompare(right.type)) }));
}

export function shortEventType(type) {
  return String(type || '').replace(/^io\.lightnvr\./, '').replace(/\.v\d+$/, '');
}

export function readEventSection(search) {
  const section = new URLSearchParams(search || '').get('eventView');
  return EVENT_SECTIONS.has(section) ? section : 'routes';
}

export function writeEventSection(href, section) {
  const url = new URL(href, 'http://localhost');
  if (section === 'routes') url.searchParams.delete('eventView');
  else url.searchParams.set('eventView', EVENT_SECTIONS.has(section) ? section : 'routes');
  return url;
}
