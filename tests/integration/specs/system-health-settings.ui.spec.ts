import { test, expect, type Page } from '@playwright/test';
import { USERS, login } from '../fixtures/test-fixtures';

const memoryCondition = {
  code: 'host.memory.pressure',
  enabled: true,
  profile: 'balanced',
  unit: 'ratio',
  warning: 0.8,
  critical: 0.95,
  recovery: 0.7,
  warning_for_seconds: 60,
  critical_for_seconds: 30,
  recovery_for_seconds: 120,
};

const healthSettings = {
  health_enabled: true,
  health_profile: 'conservative',
  health_fast_interval_seconds: 10,
  health_normal_interval_seconds: 60,
  health_slow_interval_seconds: 300,
  health_device_interval_seconds: 900,
  health_write_probe_enabled: true,
  health_hardware_provider: 'auto',
  health_presence_interval_seconds: 60,
  health_incident_retention_days: 90,
  health_condition_overrides: { version: 1, conditions: [] },
  health_effective_policy: {
    enabled: true,
    profile: 'conservative',
    generation: 7,
    conditions: [memoryCondition],
  },
};

async function mockSettingsAuxiliaryApis(page: Page) {
  await page.route('**/api/auth/sessions', (route) => route.fulfill({
    json: { sessions: [] },
  }));
  await page.route('**/api/auth/trusted-devices', (route) => route.fulfill({
    json: { devices: [] },
  }));
}

test.describe('System health settings @ui @settings', () => {
  test('renders bounded administrator controls from the server registry', async ({ page }) => {
    await login(page, USERS.admin);
    await mockSettingsAuxiliaryApis(page);
    await page.route('**/api/settings', (route) => route.fulfill({ json: healthSettings }));
    await page.goto('/settings.html#health', { waitUntil: 'domcontentloaded' });

    await expect(page.getByTestId('health-settings-tab')).toBeVisible();
    await expect(page.getByLabel('Policy profile')).toHaveValue('conservative');
    await expect(page.getByLabel('Hardware provider')).toHaveValue('auto');
    await expect(page.getByText('host.memory.pressure', { exact: true })).toBeVisible();
    await expect(page.getByTestId('health-reload-guidance')).toContainText('Saving is atomic');
    await expect(page.getByTestId('health-settings-tab').locator('input[type="text"]')).toHaveCount(0);
  });

  test('keeps dirty edits after an atomic server validation error', async ({ page }) => {
    await login(page, USERS.admin);
    await mockSettingsAuxiliaryApis(page);
    let posted: Record<string, unknown> | null = null;
    await page.route('**/api/settings', async (route) => {
      if (route.request().method() === 'POST') {
        posted = route.request().postDataJSON();
        return route.fulfill({ status: 400, json: { error: 'Policy generation changed; reload and retry.' } });
      }
      return route.fulfill({ json: healthSettings });
    });
    await page.goto('/settings.html#health', { waitUntil: 'domcontentloaded' });
    await page.getByLabel('Presence interval').fill('75');
    const save = page.locator('#save-settings-btn');
    await expect(save).toBeEnabled();
    await save.click();

    await expect(page.getByTestId('health-settings-tab').getByRole('alert').first())
      .toContainText('Policy generation changed');
    await expect(save).toBeEnabled();
    expect(posted).toMatchObject({
      health_hardware_provider: 'auto',
      health_presence_interval_seconds: 75,
      health_incident_retention_days: 90,
    });
    expect(posted).not.toHaveProperty('hardware_provider');
  });

  test('does not expose administrator health controls to viewers', async ({ page }) => {
    await login(page, USERS.viewer);
    await mockSettingsAuxiliaryApis(page);
    await page.route('**/api/settings', (route) => route.fulfill({ json: healthSettings }));
    await page.goto('/settings.html#health', { waitUntil: 'domcontentloaded' });

    await expect(page.getByText('System settings are only available to administrators.')).toBeVisible();
    await expect(page.getByTestId('health-settings-tab')).toHaveCount(0);
    await expect(page.locator('#save-settings-btn')).toHaveCount(0);
  });
});
