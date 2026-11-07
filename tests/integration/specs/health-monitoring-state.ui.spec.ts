import { test, expect } from '@playwright/test';

import { contractFixture } from '../fixtures/fleet-contracts';
import { validateHostHealth, validateHostHealthIncidents, validateSystemInformation } from '../../../web/generated/api/validators.js';

for (const state of ['disabled', 'restart_required', 'waiting'] as const) {
  test(`health coverage explains ${state} without claiming container observations @ui @system`, async ({ page }) => {
    const health = contractFixture(validateHostHealth, {
      schema_version: 1,
      overall_state: 'unknown',
      monitoring: {
        configured_enabled: state !== 'disabled',
        initialized: state !== 'waiting', enabled: false, worker_threads: 0,
        restart_required: state === 'restart_required',
      },
      snapshot: { available: false, sequence: null, completed_at_ms: null, age_ms: null, freshness: 'unknown' },
      visibility: { effective_scope: 'unknown', host_hardware_visible: false, coverage_boundary: 'unknown' },
      coverage: { complete: false, incomplete_count: 0, unavailable_observations: 0, observations_dropped: 0, collection_errors: 0, collection_timeouts: 0, abandoned_helpers: 0, coverage_overflows: 0 }, observations: [], active_incidents: [], recent_samples: [], thresholds: [],
      self_observability: { collectors: [], coverage_overflows: 0, incident_persistence: {}, event_delivery: {} },
    });
    await page.route('**/api/**', async route => {
      const path = new URL(route.request().url()).pathname;
      const responses: Record<string, unknown> = {
        '/api/client-config': { web_auth_enabled: false },
        '/api/auth/verify': {
          authenticated: true, username: 'admin', role: 'admin', role_id: 0,
          auth_enabled: false,
        },
        '/api/system/info': contractFixture(validateSystemInformation, {
          version: 'test', git_commit: 'fixture', uptime: 100, threads: 1, webThreadPoolSize: 4, global_retention_days: 30, streamStorage: [], cpu: { model: 'test', cores: 2, usage: 0 },
          memory: { total: 1024, used: 0, free: 1024 },
          systemMemory: { total: 1024, used: 0, free: 1024 },
          go2rtcMemory: { total: 1024, used: 0, free: 1024 },
          detectorMemory: { total: 1024, used: 0, free: 1024 },
          disk: { total: 1024, used: 0, free: 1024 },
          systemDisk: { total: 1024, used: 0, free: 1024 },
          network: { interfaces: [] }, streams: { active: 0, total: 0 },
          recordings: { count: 0, size: 0 }, versions: { items: [] },
        }),
        '/api/system/health': health,
        '/api/system/health/incidents': contractFixture(validateHostHealthIncidents, {
          schema_version: 1, count: 1,
          incidents: [{
            incident_id: 'historic-memory', condition: 'memory.available_low',
            subject: 'host', scope: 'host', state: 'closed', severity: 'critical',
            first_observed_at_ms: Date.now() - 172800000,
            last_observed_at_ms: Date.now() - 86400000, closed_at_ms: Date.now() - 86400000, revision: 1, reconciliation: 'reconciled',
            observation: { metric: 'host.memory.available_ratio', resource: 'host', scope: 'host', freshness: 'fresh', capability: 'available', value: .232, unit: 'ratio' },
          }], next_cursor: null,
        }),
      };
      await route.fulfill({ json: responses[path] || {} });
    });
    await page.goto('/system.html');
    await page.getByTestId('health-tab').click();
    const banner = page.getByTestId('health-coverage-banner');
    const expected = state === 'disabled' ? 'Current health facts are not being collected'
      : state === 'restart_required' ? 'Restart LightNVR to apply the change'
      : 'No completed health snapshot is available yet';
    await expect(banner).toContainText(expected);
    await expect(banner).not.toContainText('This installation can see container');
    await expect(page.getByTestId('health-history-item')).toContainText('Last recorded value');
    await expect(page.getByTestId('health-history-item')).toContainText('23.2%');
  });
}
