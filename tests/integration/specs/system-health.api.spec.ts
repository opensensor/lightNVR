import { test, expect, type APIRequestContext } from '@playwright/test';
import { CONFIG, USERS, getAuthHeader } from '../fixtures/test-fixtures';

const ADMIN_AUTH = getAuthHeader(USERS.admin);
const CAPABILITIES = new Set([
  'available', 'unsupported', 'permission_denied', 'stale', 'error',
]);
const SCOPES = new Set(['process', 'container', 'host', 'filesystem', 'device']);
const FORBIDDEN_KEYS = new Set([
  'raw_path', 'path', 'serial', 'serial_number', 'wwn', 'ip', 'ip_address',
  'command', 'command_output', 'credential', 'password',
]);
const LOGICAL_ID = /^[A-Za-z0-9_.:-]{1,63}$/;

function expectFiniteNonNegative(value: unknown): void {
  expect(typeof value).toBe('number');
  expect(Number.isFinite(value as number)).toBeTruthy();
  expect(value as number).toBeGreaterThanOrEqual(0);
}

function inspectPrivacy(value: unknown, path = 'root'): void {
  if (Array.isArray(value)) {
    value.forEach((item, index) => inspectPrivacy(item, `${path}[${index}]`));
    return;
  }
  if (!value || typeof value !== 'object') return;
  for (const [key, child] of Object.entries(value as Record<string, unknown>)) {
    expect(FORBIDDEN_KEYS.has(key.toLowerCase()), `${path}.${key}`).toBeFalsy();
    if (typeof child === 'string') {
      expect(child, `${path}.${key}`).not.toContain('/dev/');
      expect(child, `${path}.${key}`).not.toContain('/proc/');
      expect(child, `${path}.${key}`).not.toContain('/sys/');
    }
    inspectPrivacy(child, `${path}.${key}`);
  }
}

async function fetchHealth(request: APIRequestContext): Promise<Record<string, any>> {
  const response = await request.get('/api/system/health');
  expect(response.status()).toBe(200);
  return response.json();
}

test.describe('Operational system-health API @api @system-health', () => {
  let admin: APIRequestContext;
  let anonymous: APIRequestContext;

  test.beforeAll(async ({ playwright }) => {
    admin = await playwright.request.newContext({
      baseURL: CONFIG.LIGHTNVR_URL,
      extraHTTPHeaders: { Authorization: ADMIN_AUTH },
    });
    anonymous = await playwright.request.newContext({
      baseURL: CONFIG.LIGHTNVR_URL,
    });
  });

  test.afterAll(async () => {
    await Promise.all([admin.dispose(), anonymous.dispose()]);
  });

  test('requires administrator authentication for current and historical health', async () => {
    for (const endpoint of [
      '/api/system/health',
      '/api/system/health/incidents?limit=1',
      '/api/metrics',
    ]) {
      const missing = await anonymous.get(endpoint);
      expect(missing.status(), endpoint).toBe(401);
      const invalid = await anonymous.get(endpoint, {
        headers: {
          Authorization: `Basic ${Buffer.from('admin:not-the-password').toString('base64')}`,
        },
      });
      expect(invalid.status(), endpoint).toBe(401);
    }
  });

  test('returns a bounded capability-aware schema without private hardware data', async () => {
    const data = await fetchHealth(admin);

    expect(data.schema_version).toBe(1);
    expect(['unknown', 'healthy', 'warning', 'error', 'critical']).toContain(data.overall_state);
    expect(SCOPES.has(data.visibility.effective_scope)).toBeTruthy();
    expect(typeof data.visibility.host_hardware_visible).toBe('boolean');
    expect(['unknown', 'container_and_visible_mounts', 'host_and_visible_mounts'])
      .toContain(data.visibility.coverage_boundary);

    expect(typeof data.snapshot.available).toBe('boolean');
    expect(['unknown', 'fresh', 'stale']).toContain(data.snapshot.freshness);
    for (const field of ['sequence', 'completed_at_ms', 'age_ms']) {
      expect(data.snapshot[field] === null ||
        (typeof data.snapshot[field] === 'number' && data.snapshot[field] >= 0), field)
        .toBeTruthy();
    }

    expect(Array.isArray(data.observations)).toBeTruthy();
    expect(data.observations.length).toBeLessThanOrEqual(256);
    for (const observation of data.observations) {
      expect(observation.metric).toMatch(LOGICAL_ID);
      expect(observation.resource).toMatch(LOGICAL_ID);
      expect(SCOPES.has(observation.scope)).toBeTruthy();
      expect(CAPABILITIES.has(observation.capability)).toBeTruthy();
      expect(['unknown', 'fresh', 'stale']).toContain(observation.freshness);
      if (observation.capability === 'available') {
        expect(typeof observation.value).toBe('number');
        expect(Number.isFinite(observation.value)).toBeTruthy();
      } else {
        expect(observation.value).toBeNull();
      }
    }

    const capabilityTotal = Object.values(data.coverage.capabilities)
      .reduce((sum: number, count: any) => sum + count, 0);
    const scopeTotal = Object.values(data.coverage.scopes)
      .reduce((sum: number, count: any) => sum + count, 0);
    expect(capabilityTotal).toBe(data.observations.length);
    expect(scopeTotal).toBe(data.observations.length);
    expect(typeof data.coverage.complete).toBe('boolean');
    for (const field of [
      'incomplete_count', 'unavailable_observations', 'observations_dropped',
      'collection_errors', 'collection_timeouts', 'abandoned_helpers',
      'coverage_overflows',
    ]) expectFiniteNonNegative(data.coverage[field]);

    expect(Array.isArray(data.thresholds)).toBeTruthy();
    expect(data.thresholds.length).toBeLessThanOrEqual(64);
    expect(Array.isArray(data.active_incidents)).toBeTruthy();
    expect(data.active_incidents.length).toBeLessThanOrEqual(64);
    expect(Array.isArray(data.recent_samples)).toBeTruthy();
    expect(data.recent_samples.length).toBeLessThanOrEqual(120);
    for (let index = 1; index < data.recent_samples.length; index += 1) {
      expect(data.recent_samples[index].sequence)
        .toBeGreaterThan(data.recent_samples[index - 1].sequence);
      expect(data.recent_samples[index].completed_at_ms)
        .toBeGreaterThanOrEqual(data.recent_samples[index - 1].completed_at_ms);
    }

    expect(data.self_observability.collectors.length).toBeLessThanOrEqual(33);
    expect(data.self_observability.event_delivery.destinations.length).toBeLessThanOrEqual(65);
    inspectPrivacy(data);
  });

  test('validates bounded incident pagination and follows an available cursor', async () => {
    const firstResponse = await admin.get(
      '/api/system/health/incidents?limit=1&include_closed=true',
    );
    expect(firstResponse.status()).toBe(200);
    const first = await firstResponse.json();
    expect(first.schema_version).toBe(1);
    expect(first.count).toBe(first.incidents.length);
    expect(first.incidents.length).toBeLessThanOrEqual(1);
    expect(first.next_cursor === null || typeof first.next_cursor === 'string').toBeTruthy();
    inspectPrivacy(first);

    if (typeof first.next_cursor === 'string') {
      const secondResponse = await admin.get(
        `/api/system/health/incidents?limit=1&cursor=${encodeURIComponent(first.next_cursor)}`,
      );
      expect(secondResponse.status()).toBe(200);
      const second = await secondResponse.json();
      expect(second.incidents.length).toBeLessThanOrEqual(1);
      if (first.incidents[0] && second.incidents[0]) {
        expect(second.incidents[0].incident_id).not.toBe(first.incidents[0].incident_id);
      }
    }

    for (const query of [
      'limit=0', 'limit=101', 'limit=1.5', 'include_closed=maybe',
      'cursor=not-a-versioned-cursor',
    ]) {
      const response = await admin.get(`/api/system/health/incidents?${query}`);
      expect(response.status(), query).toBe(400);
    }
  });

  test('keeps liveness independent from operational state', async () => {
    const response = await admin.get('/api/health');
    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(typeof data.healthy).toBe('boolean');
    expect(data).not.toHaveProperty('overall_state');
    expect(data).not.toHaveProperty('active_incidents');
    expect(data).not.toHaveProperty('self_observability');
  });

  test('serves concurrent health and Prometheus reads without triggering collection', async () => {
    const before = await fetchHealth(admin);
    const requests = Array.from({ length: 24 }, (_, index) =>
      admin.get(index % 2 === 0 ? '/api/system/health' : '/api/metrics'));
    const responses = await Promise.all(requests);
    for (const response of responses) expect(response.status()).toBe(200);

    const metricResponses = responses.filter((_, index) => index % 2 === 1);
    for (const response of metricResponses) {
      const text = await response.text();
      expect(text).toContain('# HELP lightnvr_health_snapshot_sequence');
      expect(text).toMatch(/lightnvr_health_incidents\{severity="warning"\} \d+/);
      expect(text).not.toContain('/dev/');
      expect(text).not.toContain('/proc/');
      expect(text).not.toContain('/sys/');
      const healthSeries = text.split('\n').filter((line) =>
        line.startsWith('lightnvr_health_') || line.startsWith('lightnvr_system_'));
      expect(healthSeries.length).toBeLessThan(2048);
      for (const line of healthSeries) {
        expect(line).not.toMatch(/\{[^}]*\b(path|serial|wwn|ip|event_id|incident_id|run_id)=/);
      }
    }

    const after = await fetchHealth(admin);
    if (before.snapshot.sequence !== null && after.snapshot.sequence !== null) {
      expect(after.snapshot.sequence).toBeGreaterThanOrEqual(before.snapshot.sequence);
      // At most one scheduled completion per tier can race this short scrape burst;
      // request count must never translate into collection count.
      expect(after.snapshot.sequence - before.snapshot.sequence).toBeLessThanOrEqual(4);
    }
  });
});
