/** Favorites filtering and server-confirmed updates in the Live grid. */
import { test, expect, Page } from '@playwright/test';

const streams = [
  { name: 'Lobby', camera_uuid: 'camera-a', tags: 'Indoor' },
  { name: 'Yard', camera_uuid: 'camera-b', tags: 'Outdoor' },
].map(stream => ({
  ...stream, enabled: true, streaming_enabled: true, status: 'Running',
  availability: 'live', playback_transport: 'mse_only',
}));

async function mockLiveApi(page: Page, initialFavorites: string[] = []) {
  let favorites = initialFavorites.map(camera_uuid => ({ camera_uuid, created_at: 1 }));
  const writes: { method: string; cameraUuid: string }[] = [];
  let failWrites = false;
  await page.route('**/api/**', async route => {
    const path = new URL(route.request().url()).pathname;
    if (path.startsWith('/api/camera-favorites/')) {
      const cameraUuid = path.split('/').pop()!;
      const method = route.request().method();
      writes.push({ method, cameraUuid });
      if (failWrites) {
        await route.fulfill({ status: 403, json: { error: 'Favorite write denied' } });
        return;
      }
      favorites = favorites.filter(favorite => favorite.camera_uuid !== cameraUuid);
      const record = { camera_uuid: cameraUuid, created_at: 2 };
      if (method === 'PUT') favorites.push(record);
      await route.fulfill({ json: method === 'PUT' ? record : { success: true, removed: true } });
      return;
    }
    const responses: Record<string, unknown> = {
      '/api/client-config': {
        go2rtc_enabled: true, go2rtc_available: true, web_auth_enabled: false,
      },
      '/api/auth/verify': {
        authenticated: true, username: 'admin', role: 'admin', role_id: 0,
        auth_enabled: false,
      },
      '/api/setup/status': { complete: true },
      '/api/streams': { streams, total_pages: 1 },
      '/api/camera-favorites': { favorites, can_modify: true },
      '/api/camera-collections': { collections: [{ uuid: 'yard-only', name: 'Yard only' }] },
      '/api/fleet/cameras/query': { cameras: [streams[1]], total_pages: 1 },
    };
    await route.fulfill({ json: responses[path] || {} });
  });
  await page.routeWebSocket('**/go2rtc/api/ws*', () => {});
  return {
    writes,
    rejectWrites: () => { failWrites = true; },
    allowWrites: () => { failWrites = false; },
  };
}

for (const livePage of ['index.html', 'hls.html']) {
  test.describe(livePage, () => {
    for (const filter of ['tag=Outdoor', 'collection=yard-only']) {
      test(`favorites combined with ${filter} show a no-match message @ui @liveview`, async ({ page }) => {
        await mockLiveApi(page, ['camera-a']);
        await page.goto(`/${livePage}?mode=mse&favorites=1&${filter}`);
        await expect(page.getByRole('button', { name: 'Favorites', exact: true })).toHaveAttribute('aria-pressed', 'true');
        await expect(page.locator('#video-grid')).toContainText('No cameras match this scope.');
        await expect(page.locator('#video-grid')).not.toContainText('No favorite cameras yet.');
      });
    }

    test('an empty favorite set explains how to add cameras @ui @liveview', async ({ page }) => {
      await mockLiveApi(page);
      await page.goto(`/${livePage}?mode=mse&favorites=1`);
      await expect(page.locator('#video-grid')).toContainText('No favorite cameras yet.');
    });

    test('stars add and remove favorites while failed writes preserve the filtered grid @ui @liveview', async ({ page }) => {
      const api = await mockLiveApi(page);
      await page.goto(`/${livePage}?mode=mse`);
      const lobbyStar = page.getByRole('button', { name: 'Add Lobby to favorites', exact: true });
      await expect(lobbyStar).toHaveAttribute('aria-pressed', 'false');
      await lobbyStar.click();
      const removeStar = page.getByRole('button', { name: 'Remove Lobby from favorites', exact: true });
      await expect(removeStar).toHaveAttribute('aria-pressed', 'true');
      expect(api.writes).toEqual([{ method: 'PUT', cameraUuid: 'camera-a' }]);

      await page.getByRole('button', { name: 'Favorites', exact: true }).click();
      await expect(page.locator('.video-cell[data-stream-name="Lobby"]').first()).toBeVisible();
      await expect(page.locator('.video-cell[data-stream-name="Yard"]')).toHaveCount(0);
      await expect(page).toHaveURL(/favorites=1/);
      await page.reload();
      await expect(page.getByRole('button', { name: 'Favorites', exact: true })).toHaveAttribute('aria-pressed', 'true');
      await expect(removeStar).toHaveAttribute('aria-pressed', 'true');

      api.rejectWrites();
      await removeStar.click();
      await expect(page.getByText('Could not update favorites:', { exact: false })).toBeVisible();
      await expect(removeStar).toHaveAttribute('aria-pressed', 'true');
      await expect(page.locator('.video-cell[data-stream-name="Lobby"]').first()).toBeVisible();
      expect(api.writes.at(-1)).toEqual({ method: 'DELETE', cameraUuid: 'camera-a' });

      api.allowWrites();
      await removeStar.click();
      await expect(page.locator('#video-grid')).toContainText('No favorite cameras yet.');
      await expect(page.locator('.video-cell[data-stream-name="Lobby"]')).toHaveCount(0);
    });
  });
}
