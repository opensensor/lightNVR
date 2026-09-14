import { test, expect } from '@playwright/test';

const enforcedKeys = [
  'live.view', 'recordings.replay', 'recordings.export', 'snapshot.create',
  'ptz.control', 'evidence.protect', 'recording.delete', 'camera.configure',
  'storage.configure', 'events.configure', 'users.manage', 'system.admin',
  'lpr.read', 'lpr.search', 'lpr.export', 'lpr.delete',
];
const unenforcedKeys = ['audio.listen', 'audio.talk', 'fleet.execute_job'];

test.describe('API token permission catalog @ui @users', () => {
  for (const width of [1280, 375]) {
    test(`offers server-enforced permissions and submits selections at ${width}px`, async ({ page }) => {
      await page.setViewportSize({ width, height: 800 });
      const admin = { id: 1, username: 'admin', role: 0, is_active: true };
      const actions = [...enforcedKeys, ...unenforcedKeys].map((key) => ({
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
          '/api/auth/verify': { ...admin, role: 'admin', role_id: 0, authenticated: true },
          '/api/auth/users': { users: [admin] },
          '/api/authorization/actions': { actions, count: actions.length },
          '/api/camera-collections': { collections: [] },
          '/api/locations': { locations: [] },
          '/api/camera-tags': { tags: [] },
        };
        if (path === '/api/authorization/users/1/tokens') {
          if (route.request().method() === 'POST') {
            submitted = route.request().postDataJSON();
            return route.fulfill({ json: {
              token: { ...submitted, uuid: 'token-test', prefix: 'test', created_at: 1000 },
              secret: 'fixture-token-secret',
            } });
          }
          return route.fulfill({ json: { tokens: [] } });
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

      await dialog.getByLabel('Integration name').fill('Read-only integration');
      await dialog.getByRole('checkbox', { name: /^live\.view/ }).check();
      await dialog.getByRole('checkbox', { name: /^lpr\.read/ }).check();
      await dialog.getByRole('button', { name: 'Create token', exact: true }).click();
      await expect(dialog.locator('input[readonly]')).toHaveValue('fixture-token-secret');
      expect(submitted.actions).toEqual(['live.view', 'lpr.read']);
      expect(submitted.scope).toEqual({ type: 'all' });
    });
  }
});
