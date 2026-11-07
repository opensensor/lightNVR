import { test, expect } from '@playwright/test';
import type { components } from '../../../web/generated/api/schema';
import { USERS, login } from '../fixtures/test-fixtures';
import { collectionFixture, collectionsFixture, contractFixture } from '../fixtures/fleet-contracts';
import { validateFleetLocationsResponse, validateFleetTagsResponse, validateAdminUsers, validateApiTokenCreated, validateApiTokenList, validateAdminActionCatalog } from '../../../web/generated/api/validators.js';

const tokenSecret = `lnvr_${'a'.repeat(64)}`;

const northCollection = '00000000-0000-4000-8000-000000000010';

type Action = components['schemas']['AuthorizationAction'];

const enforcedKeys: Action[] = [
  'live.view', 'recordings.replay', 'recordings.export', 'snapshot.create',
  'ptz.control', 'evidence.protect', 'recording.delete', 'camera.configure',
  'storage.configure', 'events.configure', 'users.manage', 'system.admin',
  'lpr.read', 'lpr.search', 'lpr.export', 'lpr.delete',
];
const unenforcedKeys: Action[] = ['audio.listen', 'audio.talk', 'fleet.execute_job'];

test.describe('API token permission catalog @ui @users', () => {
  test.beforeEach(async ({ page }) => {
    // Unmocked requests such as /api/client-config need a real session.
    await login(page, USERS.admin);
  });

  for (const width of [1280, 375]) {
    test(`offers server-enforced permissions and submits selections at ${width}px`, async ({ page }) => {
      await page.setViewportSize({ width, height: 800 });
      const admin = { id: 1, username: 'admin', email: '', role: 0 as const, role_name: 'admin' as const,
        created_at: 1000, updated_at: 1000, last_login: 1000, is_active: true,
        password_change_locked: false, must_change_password: false, totp_enabled: false,
        authorization_mode: 'legacy', allowed_login_cidrs: null };
      const actions = [...enforcedKeys, ...unenforcedKeys].map((key, index) => ({
        mask_bit: 2 ** index,
        key,
        category: key.startsWith('lpr.') ? 'License plates' : 'Camera and system access',
        description: `Permission to ${key}`,
        enforced: enforcedKeys.includes(key),
        destructive: key.endsWith('.delete'),
        camera_scoped: !['storage.configure', 'events.configure', 'users.manage', 'system.admin'].includes(key),
      }));
      let submitted: any;
      await page.route('**/api/**', async (route) => {
        const path = new URL(route.request().url()).pathname;
        const responses: Record<string, unknown> = {
          '/api/auth/users': contractFixture(validateAdminUsers, { users: [admin] }),
          '/api/authorization/actions': contractFixture(validateAdminActionCatalog, { actions, count: actions.length }),
          '/api/camera-collections': collectionsFixture([collectionFixture(northCollection, 'North cameras', 2)]),
          '/api/locations': contractFixture(validateFleetLocationsResponse, { locations: [], count: 0 }),
          '/api/camera-tags': contractFixture(validateFleetTagsResponse, { tags: [], count: 0 }),
        };
        if (path === '/api/authorization/users/1/tokens') {
          if (route.request().method() === 'POST') {
            submitted = route.request().postDataJSON();
            return route.fulfill({ json: contractFixture(validateApiTokenCreated, {
              token: { ...submitted, uuid: '00000000-0000-4000-8000-000000000011',
                prefix: 'lnvr_aaaaaaaa', created_at: 1000, expires_at: submitted.expires_at ?? 0,
                revoked_at: null, last_used_at: null,
                scope: { ...submitted.scope, selector: null, collection_uuid: null } },
              secret: tokenSecret,
            }) });
          }
          return route.fulfill({ json: contractFixture(validateApiTokenList, { user_id: 1, count: 0, tokens: [] }) });
        }
        if (path in responses) return route.fulfill({ json: responses[path] });
        return route.continue();
      });

      await page.goto('/users.html', { waitUntil: 'domcontentloaded' });
      await page.getByTitle('Manage API access').click();
      const dialog = page.getByRole('dialog', { name: 'API Access — admin' });
      await expect(dialog).toBeVisible();
      await expect(dialog.locator('fieldset input[type="checkbox"]')).toHaveCount(16);
      for (const key of enforcedKeys) {
        const checkbox = dialog.getByRole('checkbox', { name: new RegExp(`^${key.replace('.', '\\.')}`) });
        await checkbox.scrollIntoViewIfNeeded();
        await expect(checkbox).toBeInViewport();
      }
      for (const key of unenforcedKeys) {
        await expect(dialog.getByText(key, { exact: true })).toHaveCount(0);
      }
      await expect(dialog).not.toContainText('Current endpoint coverage:');

      await dialog.getByLabel('Integration name').fill('Fleet integration');
      await dialog.getByRole('checkbox', { name: /^live\.view/ }).check();
      await dialog.getByRole('checkbox', { name: /^lpr\.read/ }).check();
      const createButton = dialog.getByRole('button', { name: 'Create token', exact: true });
      const scope = dialog.getByRole('combobox', { name: /^Camera scope/ });
      const scopeError = dialog.getByText('Non-camera permissions require Entire camera fleet scope. Choose that scope or remove those permissions.');
      for (const scopeType of ['collection', 'selector']) {
        await scope.selectOption(scopeType);
        if (scopeType === 'collection') {
          await dialog.getByRole('combobox', { name: /^Shared camera collection/ }).selectOption(northCollection);
        }
        await expect(createButton).toBeEnabled();
        for (const key of ['storage.configure', 'events.configure', 'users.manage', 'system.admin']) {
          const checkbox = dialog.getByRole('checkbox', { name: new RegExp(`^${key.replace('.', '\\.')}`) });
          await checkbox.check();
          await expect(scopeError).toBeVisible();
          await expect(createButton).toBeDisabled();
          await expect(scope).toHaveValue(scopeType);
          expect(submitted).toBeUndefined();
          await checkbox.uncheck();
          await expect(scopeError).toHaveCount(0);
          await expect(createButton).toBeEnabled();
        }
      }
      await dialog.getByRole('checkbox', { name: /^system\.admin/ }).check();
      await expect(createButton).toBeDisabled();
      await scope.selectOption('all');
      await expect(scopeError).toHaveCount(0);
      await expect(createButton).toBeEnabled();
      await createButton.click();
      await expect(dialog.locator('input[readonly]')).toHaveValue(tokenSecret);
      expect(submitted.actions).toEqual(['live.view', 'lpr.read', 'system.admin']);
      expect(submitted.scope).toEqual({ type: 'all' });
    });
  }
});
