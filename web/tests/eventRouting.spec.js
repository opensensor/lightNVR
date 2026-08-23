import {
  ALL_CAMERAS_SELECTOR,
  EMPTY_ROUTE_DRAFT,
  buildDestinationPayload,
  buildRoutePayload,
  createDestinationDraft,
  destinationToDraft,
  groupCatalog,
  readEventSection,
  routeToDraft,
  splitList,
  validateDestinationDraft,
  validateRouteDraft,
  writeEventSection,
} from '../js/components/preact/events/eventRouting.js';

describe('event routing workspace helpers', () => {
  test('creates a stable safe MQTT client ID', () => {
    expect(createDestinationDraft('A0B1-C2D3-E4F5-6789').clientId).toBe('lightnvr-a0b1c2d3e4f5');
  });

  test('builds destination payloads without leaking preserved passwords', () => {
    const draft = destinationToDraft({
      uuid: 'destination-1',
      name: 'Operations',
      enabled: true,
      broker: { host: 'mqtt.example.test', port: 8883, client_id: 'lightnvr-operations', topic_template: 'events/{type}/{subject_id}', keepalive_seconds: 60, qos: 1 },
      authentication: { username: 'publisher', password_configured: true },
      tls: { mode: 'system', ca_file: '', cert_file: '', key_file: '' },
      revision: 4,
    });
    const payload = buildDestinationPayload(draft, false);
    expect(payload.revision).toBe(4);
    expect(payload.authentication).toEqual({ username: 'publisher' });
    expect(payload.tls).toEqual({ mode: 'system', ca_file: '', cert_file: '', key_file: '' });

    expect(buildDestinationPayload({ ...draft, clearPassword: true }, false).authentication.password).toBeNull();
    expect(buildDestinationPayload({ ...draft, password: 'replacement' }, false).authentication.password).toBe('replacement');
  });

  test('requires explicit certificate paths for mutual TLS', () => {
    const draft = {
      ...createDestinationDraft('01234567-89ab-cdef-0123-456789abcdef'),
      name: 'Cloud bridge',
      host: 'mqtt.example.test',
      tlsMode: 'mutual',
      username: 'publisher',
    };
    expect(validateDestinationDraft(draft)).toBe('ca_file');
    expect(validateDestinationDraft({ ...draft, caFile: '/ca.pem' })).toBe('mutual_files');
    const complete = { ...draft, caFile: '/ca.pem', certFile: '/client.pem', keyFile: '/client.key' };
    expect(validateDestinationDraft(complete)).toBe('');
    expect(buildDestinationPayload(complete, true).tls).toEqual({ mode: 'mutual', ca_file: '/ca.pem', cert_file: '/client.pem', key_file: '/client.key' });
    expect(buildDestinationPayload({ ...complete, tlsMode: 'system' }, false).tls)
      .toEqual({ mode: 'system', ca_file: '', cert_file: '', key_file: '' });
  });

  test('rejects URL hosts, MQTT wildcards, unknown tokens, and overflowing topics', () => {
    const draft = { ...createDestinationDraft('01234567-89ab-cdef-0123-456789abcdef'), name: 'Bridge', host: 'mqtt.example.test' };
    expect(validateDestinationDraft({ ...draft, host: 'mqtts://mqtt.example.test' })).toBe('host');
    expect(validateDestinationDraft({ ...draft, topicTemplate: 'events/+/{type}/{subject_id}' })).toBe('topic');
    expect(validateDestinationDraft({ ...draft, topicTemplate: 'events/{unknown}/{type}/{subject_id}' })).toBe('topic');
    expect(validateDestinationDraft({ ...draft, topicTemplate: `${'a'.repeat(400)}/{type}/{subject_id}` })).toBe('topic');
  });

  test('round trips a route draft and emits the complete versioned contract', () => {
    const route = {
      uuid: 'route-1',
      name: 'North entrance people',
      description: 'External notification input',
      enabled: true,
      destination: 'mqtt:destination-1',
      event_types: ['io.lightnvr.detection.object.v1'],
      camera_scope: { type: 'selector', selector: { version: 1, expression: { op: 'tag_any', uuids: ['tag-1'] } } },
      predicate: { version: 1, detection: { labels_any: ['person'], zone_ids_any: ['entry'], min_confidence: 0.8 } },
      schedule: { version: 1, timezone: 'America/New_York', windows: [{ days: [1, 2, 3, 4, 5], start: '18:00', end: '06:00' }] },
      suppression: { debounce_seconds: 2, cooldown_seconds: 30, grouping_window_seconds: 10, max_events_per_minute: 20 },
      revision: 7,
    };
    const draft = routeToDraft(route);
    expect(validateRouteDraft(draft)).toBe('');
    expect(buildRoutePayload(draft, false)).toEqual({
      name: route.name,
      description: route.description,
      enabled: true,
      destination: route.destination,
      event_types: route.event_types,
      camera_scope: route.camera_scope,
      predicate: route.predicate,
      schedule: route.schedule,
      suppression: route.suppression,
      revision: 7,
    });
    expect(buildRoutePayload(draft, true)).not.toHaveProperty('revision');
  });

  test('validates incomplete route drafts before API calls', () => {
    expect(validateRouteDraft({ ...EMPTY_ROUTE_DRAFT, eventTypes: [] })).toBe('name');
    const base = { ...EMPTY_ROUTE_DRAFT, name: 'Route', eventTypes: ['io.lightnvr.camera.offline.v1'] };
    expect(validateRouteDraft(base)).toBe('');
    expect(validateRouteDraft({ ...base, scopeType: 'selector', selector: null })).toBe('selector');
    expect(validateRouteDraft({ ...base, detectionFilterEnabled: true, labels: 'person' })).toBe('detection_type');
    expect(validateRouteDraft({ ...base, scheduleEnabled: true, windows: [] })).toBe('schedule_window');
    expect(buildRoutePayload(base, true).camera_scope).toEqual({ type: 'all' });
    expect(ALL_CAMERAS_SELECTOR.expression.op).toBe('all');
  });

  test('normalizes lists, catalog groups, and subsection URL state', () => {
    expect(splitList(' person, vehicle\nperson , ')).toEqual(['person', 'vehicle']);
    expect(groupCatalog([
      { type: 'io.lightnvr.storage.pressure.v1', family: 'storage' },
      { type: 'io.lightnvr.camera.offline.v1', family: 'camera' },
    ]).map((group) => group.family)).toEqual(['camera', 'storage']);
    expect(readEventSection('?view=events&eventView=destinations')).toBe('destinations');
    expect(readEventSection('?eventView=invalid')).toBe('routes');
    const url = writeEventSection('http://localhost/streams.html?view=events&kept=yes', 'catalog');
    expect(url.searchParams.get('view')).toBe('events');
    expect(url.searchParams.get('eventView')).toBe('catalog');
    expect(url.searchParams.get('kept')).toBe('yes');
  });
});
