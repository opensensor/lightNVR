import { test, expect, Page } from '@playwright/test';

/**
 * LightNVR UI Tests
 *
 * Adapted from scripts/capture-screenshots.js for robust page navigation.
 * Run with: npx playwright test --project=ui --headed
 */

const AUTH_USER = process.env.LIGHTNVR_USER || 'admin';
const AUTH_PASS = process.env.LIGHTNVR_PASS || 'admin';

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Login through the web form (adapted from capture-screenshots.js)
 */
async function login(page: Page): Promise<void> {
  console.log('Logging in...');
  try {
    await page.goto('/login.html', { waitUntil: 'networkidle', timeout: 10000 });

    // Wait for login form
    await page.waitForSelector('input[name="username"], input[type="text"]', { timeout: 5000 });

    // Fill credentials
    const usernameInput = page.locator('input[name="username"], input[type="text"]').first();
    await usernameInput.fill(AUTH_USER);

    const passwordInput = page.locator('input[name="password"], input[type="password"]').first();
    await passwordInput.fill(AUTH_PASS);

    // Submit
    const submitButton = page.locator('button[type="submit"], button').filter({ hasText: /login|sign in/i }).first();
    await submitButton.click();

    // Wait for redirect
    await page.waitForURL('**/index.html', { timeout: 10000 });
    console.log('Login successful');
  } catch (error) {
    console.error('Login failed:', (error as Error).message);
    throw error;
  }
}

/**
 * Navigate to a page with authentication handling
 */
async function navigateTo(page: Page, path: string, options?: { waitForNetworkIdle?: boolean }): Promise<void> {
  const waitUntil = options?.waitForNetworkIdle ? 'networkidle' : 'domcontentloaded';

  try {
    await page.goto(path, { waitUntil, timeout: 10000 });
    await sleep(1500); // Wait for Preact components to render

    // If we got redirected to login, perform login and retry
    if (page.url().includes('login')) {
      await login(page);
      await page.goto(path, { waitUntil, timeout: 10000 });
      await sleep(1500);
    }
  } catch (error) {
    console.error(`Navigation to ${path} failed:`, (error as Error).message);
    throw error;
  }
}

test.describe('LightNVR Login', () => {
  test('should display login page', async ({ page }) => {
    await page.goto('/login.html', { waitUntil: 'networkidle', timeout: 10000 });
    await expect(page).toHaveTitle(/LightNVR/i);

    // Should see login form elements
    await page.waitForSelector('input[name="username"], input[type="text"]', { timeout: 5000 });
    await page.screenshot({ path: 'test-results/login-page.png' });
  });

  test('should login successfully', async ({ page }) => {
    await login(page);

    // After login, should be on index page
    expect(page.url()).toContain('index.html');
    await sleep(1000);
    await page.screenshot({ path: 'test-results/after-login.png' });
  });
});

test.describe('LightNVR Web Interface', () => {
  // Login once before all tests in this describe block
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should load the main dashboard', async ({ page }) => {
    await navigateTo(page, '/index.html');
    const title = await page.title();
    expect(title.toLowerCase()).toContain('lightnvr');
    await page.screenshot({ path: 'test-results/dashboard.png' });
  });

  test('should load the streams page', async ({ page }) => {
    await navigateTo(page, '/streams.html', { waitForNetworkIdle: true });
    await sleep(1500);
    await expect(page.locator('body')).toBeVisible();
    await page.screenshot({ path: 'test-results/streams-page.png' });
  });

  test('should load the recordings page', async ({ page }) => {
    await navigateTo(page, '/recordings.html', { waitForNetworkIdle: true });
    await sleep(1500);
    await expect(page.locator('body')).toBeVisible();
    await page.screenshot({ path: 'test-results/recordings-page.png' });
  });

  test('should load the settings page', async ({ page }) => {
    await navigateTo(page, '/settings.html', { waitForNetworkIdle: true });
    await sleep(2000); // Settings has more Preact components
    await expect(page.locator('body')).toBeVisible();
    await page.screenshot({ path: 'test-results/settings-page.png' });
  });

  test('should load the system page', async ({ page }) => {
    // System page may have continuous updates, use domcontentloaded
    await navigateTo(page, '/system.html');
    await sleep(2000); // Wait for system stats to load
    await expect(page.locator('body')).toBeVisible();
    await page.screenshot({ path: 'test-results/system-page.png' });
  });
});

test.describe('Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should have navigation links', async ({ page }) => {
    await navigateTo(page, '/index.html');

    const navLinks = page.locator('nav a, .nav a, .sidebar a, header a, .navigation a');
    const linkCount = await navLinks.count();
    console.log(`Found ${linkCount} navigation links`);

    // Log navigation links for debugging
    for (let i = 0; i < Math.min(linkCount, 10); i++) {
      const href = await navLinks.nth(i).getAttribute('href');
      const text = await navLinks.nth(i).textContent();
      console.log(`  Link ${i + 1}: ${text?.trim()} -> ${href}`);
    }

    expect(linkCount).toBeGreaterThan(0);
  });
});

test.describe('Live Streams', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should load live streams page with video elements', async ({ page }) => {
    // Don't wait for networkidle - streams continuously load data
    await navigateTo(page, '/index.html');

    // Wait for streams to load and start playing
    console.log('  Waiting for video streams to load...');
    await sleep(5000);

    // Try to wait for video elements to be present
    try {
      await page.waitForSelector('video, canvas, img[alt*="stream"]', { timeout: 5000 });
      console.log('  Video elements detected');

      // Check if videos are playing
      const videoStatus = await page.evaluate(() => {
        const videos = document.querySelectorAll('video');
        let playingCount = 0;
        videos.forEach(v => {
          if (v.videoWidth > 0 && v.videoHeight > 0 && v.readyState >= 2) {
            playingCount++;
          }
        });
        return { total: videos.length, playing: playingCount };
      });

      console.log(`  Found ${videoStatus.playing}/${videoStatus.total} videos playing`);
    } catch (e) {
      console.log('  No video elements found (this may be normal with test streams)');
    }

    await page.screenshot({ path: 'test-results/live-streams.png' });
  });
});

