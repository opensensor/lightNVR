import { test, expect, type Page } from '@playwright/test';
import { USERS, login } from '../fixtures/test-fixtures';

const systemInfo = {
  version: 'test', uptime: 120,
  cpu: { model: 'test', cores: 2, usage: 0, usageCapability: 'available' },
  memory: { total: 1024, used: null, free: null, capability: 'stale' },
  go2rtcMemory: { total: 1024, used: null, free: null },
  detectorMemory: { total: 1024, used: null, free: null },
  systemMemory: { total: null, used: null, free: null, capability: 'stale' },
  disk: { total: null, used: 0, free: null, capability: 'permission_denied' },
  systemDisk: { total: null, used: null, free: null, capability: 'permission_denied' },
  network: { interfaces: [{ name: 'primary', address: 'Unavailable', mac: 'Unavailable', up: null, capability: 'permission_denied' }] },
  streams: { active: 0, total: 0 }, recordings: { count: 0, size: 0 }, versions: { items: [] },
};

const containerHealth = {
  schema_version: 1,
  overall_state: 'critical',
  visibility: { effective_scope: 'container', host_hardware_visible: false, coverage_boundary: 'container_and_visible_mounts' },
  snapshot: { available: true, sequence: 42, completed_at_ms: Date.now(), age_ms: 1000, freshness: 'fresh' },
  coverage: { complete: false, incomplete_count: 3, observations_dropped: 0, collection_errors: 0, collection_timeouts: 0, abandoned_helpers: 0, coverage_overflows: 0, capabilities: { available: 1, unsupported: 1, permission_denied: 1, stale: 0, error: 0 }, scopes: { process: 0, container: 1, host: 0, filesystem: 0, device: 1 } },
  observations: [
    { metric: 'container.cpu.usage_ratio', resource: 'container', scope: 'container', capability: 'available', freshness: 'fresh', unit: 'ratio', value: 0 },
    { metric: 'storage.device.life_used_ratio', resource: 'device-test', scope: 'device', capability: 'permission_denied', freshness: 'unknown', unit: 'none', value: null },
  ],
  thresholds: [],
  active_incidents: [{ incident_id: '11111111-1111-4111-8111-111111111111', condition: 'filesystem.read_only', subject: 'recording', scope: 'filesystem', state: 'open', severity: 'critical', first_observed_at_ms: Date.now() - 60000, last_observed_at_ms: Date.now(), persistence_pending: false, observation: { metric: 'filesystem.read_only', resource: 'recording', scope: 'filesystem', capability: 'available', freshness: 'fresh', unit: 'boolean', value: 1 }, thresholds: { warning_threshold: 1, critical_threshold: 1, recovery_threshold: 0, warning_for_seconds: 0, unit: 'boolean' } }],
  evaluator: { tracked_conditions: 1, active_incidents: 1, pending_persistence: 0 },
  recent_samples: [{ sequence: 41, completed_at_ms: Date.now() - 60000, observation_count: 2, observations_dropped: 0 }, { sequence: 42, completed_at_ms: Date.now(), observation_count: 2, observations_dropped: 0 }],
};

async function openSystemHealth(page: Page) {
  await page.route('**/api/system/info', (route) => route.fulfill({ json: systemInfo }));
  await page.goto('/system.html', { waitUntil: 'domcontentloaded' });
  await page.getByTestId('health-tab').click();
}

test.describe('System Health operator view @ui @system', () => {
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test('shows container coverage and unavailable hardware explicitly', async ({ page }) => {
    await page.route('**/api/system/health/incidents?**', (route) => route.fulfill({ json: { schema_version: 1, count: 0, incidents: [], next_cursor: null } }));
    await page.route('**/api/system/health', (route) => route.fulfill({ json: containerHealth }));
    await openSystemHealth(page);

    await expect(page.getByTestId('system-health-view')).toBeVisible();
    await expect(page.getByTestId('health-overall-state')).toHaveText('critical');
    await expect(page.getByTestId('health-coverage-banner')).toContainText('host hardware is unavailable');
    await expect(page.getByTestId('health-coverage-banner')).toContainText('Missing host values are unknown, not healthy');
    await expect(page.getByText('permission_denied', { exact: true })).toBeVisible();
    await expect(page.getByText('Unknown', { exact: true })).toBeVisible();
    await expect(page.getByText('0.0%', { exact: true })).toBeVisible();
    await expect(page.getByTestId('active-health-incident')).toContainText('critical · open');
    await expect(page.getByTestId('active-health-incident')).toContainText('Suggested response');
  });

  test('paginates bounded incident history', async ({ page }) => {
    await page.route('**/api/system/health', (route) => route.fulfill({ json: { ...containerHealth, overall_state: 'healthy', active_incidents: [] } }));
    await page.route('**/api/system/health/incidents?**', (route) => {
      const cursor = new URL(route.request().url()).searchParams.get('cursor');
      const incident = { incident_id: cursor ? '33333333-3333-4333-8333-333333333333' : '22222222-2222-4222-8222-222222222222', condition: cursor ? 'network.link_down' : 'memory.available_low', subject: 'host', scope: 'host', state: 'closed', severity: 'warning', first_observed_at_ms: Date.now() - 10000, last_observed_at_ms: Date.now(), closed_at_ms: Date.now(), reconciliation: 'reconciled', observation: { metric: 'host.memory.available_ratio', resource: 'host', scope: 'host', capability: 'available', unit: 'ratio', value: 0.1 } };
      return route.fulfill({ json: { schema_version: 1, count: 1, incidents: [incident], next_cursor: cursor ? null : 'v1:123:11111111-1111-4111-8111-111111111111' } });
    });
    await openSystemHealth(page);

    await expect(page.getByTestId('health-history-item')).toContainText('memory available low');
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByTestId('health-history-item')).toContainText('network link down');
    await page.getByRole('button', { name: 'Previous' }).click();
    await expect(page.getByTestId('health-history-item')).toContainText('memory available low');
  });

  test('keeps a usable health error state at mobile width', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 720 });
    await page.route('**/api/system/health', (route) => route.fulfill({ status: 503, json: { error: 'unavailable' } }));
    await openSystemHealth(page);
    await expect(page.getByTestId('system-health-error')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Retry' })).toBeVisible();
  });
});
