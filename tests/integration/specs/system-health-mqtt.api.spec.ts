import { test, expect, type APIRequestContext } from '@playwright/test';
import { CONFIG, USERS, getAuthHeader } from '../fixtures/test-fixtures';

const ADMIN_AUTH = getAuthHeader(USERS.admin);
const HEALTH_ALERT = 'io.lightnvr.system.health_alert.v1';
const HEALTH_RECOVERED = 'io.lightnvr.system.health_recovered.v1';

test.describe('System-health MQTT routing @api @system-health @mqtt', () => {
  test.describe.configure({ mode: 'serial' });
  let admin: APIRequestContext;
  let destinationId: string | null = null;
  let destinationRevision: number | null = null;
  let routeId: string | null = null;
  let routeRevision: number | null = null;

  test.beforeAll(async ({ playwright }) => {
    admin = await playwright.request.newContext({
      baseURL: CONFIG.LIGHTNVR_URL,
      extraHTTPHeaders: { Authorization: ADMIN_AUTH },
    });
  });

  test.afterAll(async () => {
    if (routeId && routeRevision) {
      await admin.delete(
        `/api/event-routes/${routeId}?revision=${routeRevision}`,
      ).catch(() => undefined);
    }
    if (destinationId && destinationRevision) {
      await admin.delete(
        `/api/event-destinations/${destinationId}?revision=${destinationRevision}`,
      ).catch(() => undefined);
    }
    await admin.dispose();
  });

  test('rejects wildcard and incomplete managed status topics', async () => {
    const prefix = `T24 invalid ${Date.now()}`;
    for (const [name, statusTopic] of [
      [`${prefix} wildcard`, 'lightnvr/status/+/{installation_uuid}'],
      [`${prefix} missing identity`, 'lightnvr/status/static'],
      [`${prefix} unknown placeholder`, 'lightnvr/status/{installation_uuid}/{serial}'],
    ]) {
      const response = await admin.post('/api/event-destinations', {
        data: {
          name,
          broker: {
            host: '127.0.0.1',
            port: 18884,
            topic_template: 'lightnvr/t24/{type}/{subject_id}',
            status_topic_template: statusTopic,
          },
          tls: { mode: 'disabled' },
        },
      });
      expect(response.status(), statusTopic).toBe(400);
    }
  });

  test('accepts a privacy-safe status topic and a system-scoped health route', async () => {
    const suffix = Date.now();
    const statusTopic =
      'lightnvr/t24/status/{installation_uuid}/{destination_uuid}';
    const destinationResponse = await admin.post('/api/event-destinations', {
      data: {
        name: `T24 health destination ${suffix}`,
        description: 'Disposable unreachable broker for delivery observability',
        enabled: true,
        broker: {
          host: '127.0.0.1',
          port: 18884,
          client_id: `lightnvr-t24-${suffix}`,
          topic_template: 'lightnvr/t24/{type}/{subject_id}',
          status_topic_template: statusTopic,
          keepalive_seconds: 5,
          qos: 1,
        },
        tls: { mode: 'disabled' },
      },
    });
    expect(destinationResponse.status()).toBe(201);
    const destination = await destinationResponse.json();
    destinationId = destination.uuid;
    destinationRevision = destination.revision;
    expect(destination.broker.status_topic_template).toBe(statusTopic);
    expect(destination.authentication).not.toHaveProperty('password');
    expect(destinationId).toMatch(/^[0-9a-f-]{36}$/);

    const previewBody = {
      name: `T24 health preview ${suffix}`,
      destination: `mqtt:${destinationId}`,
      event_types: [HEALTH_ALERT, HEALTH_RECOVERED],
      predicate: {
        version: 1,
        health: {
          condition_codes_any: ['memory.available_low', 'filesystem.read_only'],
          severities_any: ['warning', 'error', 'critical'],
        },
      },
    };
    const previewResponse = await admin.post('/api/event-routes/preview', {
      data: previewBody,
    });
    expect(previewResponse.status()).toBe(200);
    const preview = await previewResponse.json();
    expect(preview.would_publish).toBe(false);
    expect(preview.matched_camera_count).toBe(0);
    expect(preview.camera_sample).toEqual([]);
    expect(preview.event_types).toHaveLength(2);
    for (const type of preview.event_types) {
      expect(type.subject_kind).toBe('system');
    }
    const alert = preview.event_types.find(
      (type: Record<string, unknown>) => type.type === HEALTH_ALERT,
    );
    expect(alert).toBeDefined();
    expect(alert.dynamic_severity).toBe(true);
    expect(alert.minimum_severity).toBe('warning');
    expect(alert.maximum_severity).toBe('critical');
    const recovered = preview.event_types.find(
      (type: Record<string, unknown>) => type.type === HEALTH_RECOVERED,
    );
    expect(recovered).toBeDefined();
    expect(recovered.dynamic_severity).toBe(false);
    expect(recovered.severity).toBe('info');

    const routeResponse = await admin.post('/api/event-routes', {
      data: previewBody,
    });
    expect(routeResponse.status()).toBe(201);
    const route = await routeResponse.json();
    routeId = route.uuid;
    routeRevision = route.revision;
    expect(route.destination).toBe(`mqtt:${destinationId}`);
    expect(route.camera_scope.type).toBe('all');
    expect(route.predicate.health.condition_codes_any)
      .toEqual(['memory.available_low', 'filesystem.read_only']);
  });

  test('rejects camera selectors and unknown condition codes for system events', async () => {
    const cameraScoped = await admin.post('/api/event-routes/preview', {
      data: {
        name: `T24 invalid camera health ${Date.now()}`,
        event_types: [HEALTH_ALERT],
        camera_scope: {
          type: 'selector',
          selector: {
            version: 1,
            expression: {
              op: 'camera_uuid',
              values: ['22222222-2222-4222-8222-222222222222'],
            },
          },
        },
      },
    });
    expect(cameraScoped.status()).toBe(400);

    const unknownCondition = await admin.post('/api/event-routes/preview', {
      data: {
        name: `T24 invalid condition ${Date.now()}`,
        event_types: [HEALTH_ALERT],
        predicate: {
          version: 1,
          health: { condition_codes_any: ['memory.private_typo'] },
        },
      },
    });
    expect(unknownCondition.status()).toBe(400);
  });

  test('exposes bounded delivery state while the isolated broker is unavailable', async () => {
    expect(destinationId).not.toBeNull();
    const healthResponse = await admin.get('/api/system/health');
    expect(healthResponse.status()).toBe(200);
    const health = await healthResponse.json();
    const delivery = health.self_observability.event_delivery;

    expect(typeof delivery.degraded).toBe('boolean');
    expect(typeof delivery.circular_report_path).toBe('boolean');
    expect(typeof delivery.outbox_full).toBe('boolean');
    expect(typeof delivery.all_destinations_unavailable).toBe('boolean');
    expect(typeof delivery.bus.running).toBe('boolean');
    expect(typeof delivery.worker.running).toBe('boolean');
    expect(typeof delivery.outbox.available).toBe('boolean');
    expect(delivery.destinations.length).toBeLessThanOrEqual(65);

    const managed = delivery.destinations.find(
      (item: Record<string, unknown>) => item.destination === destinationId,
    );
    expect(managed).toBeDefined();
    expect(typeof managed.runtime_available).toBe('boolean');
    expect(typeof managed.connected).toBe('boolean');
    expect(['none', 'configuration', 'connection', 'publication'])
      .toContain(managed.last_failure);
    expect(managed.outbox.available === true || managed.outbox.available === false)
      .toBeTruthy();

    const metricsResponse = await admin.get('/api/metrics');
    expect(metricsResponse.status()).toBe(200);
    const metrics = await metricsResponse.text();
    expect(metrics).toContain('# HELP lightnvr_event_destination_connected');
    expect(metrics).toContain('# HELP lightnvr_event_delivery_degraded');
    expect(metrics).toContain('# HELP lightnvr_event_outbox_rows');
    expect(metrics).not.toContain('127.0.0.1');
    expect(metrics).not.toContain('lightnvr-t24-');
    expect(metrics).not.toContain('status/{installation_uuid}');
  });
});
