/**
 * Shared Test Fixtures for LightNVR Integration Tests
 * 
 * Provides reusable fixtures for authentication, page setup, and test utilities.
 */

import { expect, Page } from '@playwright/test';

// Configuration
// Use longer timeouts in CI environments
const isCI = !!process.env.CI;
export const CONFIG = {
  LIGHTNVR_URL: process.env.LIGHTNVR_URL || 'http://localhost:18080',
  GO2RTC_URL: process.env.GO2RTC_URL || 'http://localhost:11984',
  GO2RTC_RTSP_PORT: process.env.GO2RTC_RTSP_PORT || '18554',
  DEFAULT_TIMEOUT: isCI ? 30000 : 10000,  // 30s in CI, 10s locally
  NAVIGATION_TIMEOUT: isCI ? 30000 : 15000,  // Timeout for page navigations
  COMPONENT_RENDER_DELAY: isCI ? 2500 : 1500,  // More time for components in CI
};

// User credentials for different roles
export const USERS = {
  admin: { username: 'admin', password: 'admin', role: 0 },
  // Note: These users need to be created during test setup
  user: { username: 'test_user', password: 'TestUser123!', role: 1 },
  viewer: { username: 'test_viewer', password: 'TestViewer123!', role: 2 },
  api: { username: 'test_api', password: 'TestApi123!', role: 3 },
};

export type UserRole = keyof typeof USERS;

/**
 * Sleep utility
 */
export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Get Basic Auth header for a user
 */
export function getAuthHeader(user: { username: string; password: string }): string {
  return 'Basic ' + Buffer.from(`${user.username}:${user.password}`).toString('base64');
}

/**
 * Dismiss the first-run setup wizard if it is visible.
 * Calls POST /api/setup/status (no auth required) to mark setup complete,
 * then reloads the page so the wizard doesn't block further interactions.
 */
export async function dismissSetupWizard(page: Page): Promise<void> {
  try {
    const wizard = page.locator('[data-testid="setup-wizard"]');
    if (await wizard.isVisible({ timeout: 1500 }).catch(() => false)) {
      console.log('Setup wizard detected – dismissing via API...');
      await page.request.post('/api/setup/status', {
        data: { complete: true },
        headers: { 'Content-Type': 'application/json' },
      });
      await page.reload({ waitUntil: 'domcontentloaded' });
      await sleep(CONFIG.COMPONENT_RENDER_DELAY);
      console.log('Setup wizard dismissed');
    }
  } catch (e) {
    // Non-fatal – log and continue
    console.warn('dismissSetupWizard: error ignored –', (e as Error).message);
  }
}

/**
 * Login to the web interface
 */
export async function login(page: Page, user: { username: string; password: string } = USERS.admin): Promise<void> {
  console.log(`Logging in as ${user.username}...`);
  try {
    // Use domcontentloaded instead of networkidle for faster initial load
    await page.goto('/login.html', { waitUntil: 'domcontentloaded', timeout: CONFIG.NAVIGATION_TIMEOUT });

    // Wait for login form to be visible and interactive
    const usernameInput = page.locator('input[name="username"], input#username, input[type="text"]').first();
    await usernameInput.waitFor({ state: 'visible', timeout: CONFIG.DEFAULT_TIMEOUT });

    // Fill credentials
    await usernameInput.fill(user.username);

    const passwordInput = page.locator('input[name="password"], input#password, input[type="password"]').first();
    await passwordInput.waitFor({ state: 'visible', timeout: 5000 });
    await passwordInput.fill(user.password);

    // Submit
    const submitButton = page.locator('button[type="submit"], button').filter({ hasText: /login|sign in/i }).first();
    await submitButton.waitFor({ state: 'visible', timeout: 5000 });

    // Use Promise.all to start waiting for navigation BEFORE clicking
    // This is critical because window.location.href redirects can happen very quickly
    // and we need the listener attached before the redirect starts
    await Promise.all([
      page.waitForURL('**/index.html', { timeout: CONFIG.NAVIGATION_TIMEOUT, waitUntil: 'commit' }),
      submitButton.click(),
    ]);

    // Wait a bit for the page to stabilize
    await sleep(CONFIG.COMPONENT_RENDER_DELAY);

    console.log(`Login successful as ${user.username}`);
  } catch (error) {
    // Take a screenshot on failure for debugging
    try {
      await page.screenshot({ path: `test-results/login-failure-${user.username}-${Date.now()}.png` });
      console.error(`Current URL: ${page.url()}`);
    } catch (e) {
      // Ignore screenshot errors
    }
    console.error(`Login failed for ${user.username}:`, (error as Error).message);
    throw error;
  }
}

/**
 * Navigate to a page with authentication handling
 */
export async function navigateTo(
  page: Page, 
  path: string, 
  options?: { waitForNetworkIdle?: boolean; user?: { username: string; password: string } }
): Promise<void> {
  const waitUntil = options?.waitForNetworkIdle ? 'networkidle' : 'domcontentloaded';
  const user = options?.user || USERS.admin;

  try {
    await page.goto(path, { waitUntil, timeout: CONFIG.DEFAULT_TIMEOUT });
    await sleep(CONFIG.COMPONENT_RENDER_DELAY);

    // If redirected to login, perform login and retry
    if (page.url().includes('login')) {
      await login(page, user);
      await page.goto(path, { waitUntil, timeout: CONFIG.DEFAULT_TIMEOUT });
      await sleep(CONFIG.COMPONENT_RENDER_DELAY);
    }
  } catch (error) {
    console.error(`Navigation to ${path} failed:`, (error as Error).message);
    throw error;
  }
}

/**
 * Logout from the web interface
 */
export async function logout(page: Page): Promise<void> {
  try {
    // Desktop logout link is in the header user-menu (visible on md+)
    const desktopLogoutLink = page.locator('.user-menu a.logout-link, .user-menu a[href="/logout"]').first();

    // Mobile logout link appears after clicking hamburger menu
    const mobileMenuButton = page.locator('button[aria-label="Toggle menu"]').first();
    const mobileLogoutLink = page.locator('li a.logout-link, a.logout-link').first();

    // Check if desktop logout is visible
    if (await desktopLogoutLink.isVisible({ timeout: 2000 }).catch(() => false)) {
      // Use Promise.all to start waiting before click to avoid race condition
      await Promise.all([
        page.waitForURL('**/login.html**', { timeout: CONFIG.DEFAULT_TIMEOUT }),
        desktopLogoutLink.click(),
      ]);
      console.log('Logout successful (desktop)');
    } else if (await mobileMenuButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      // Open mobile menu and click logout
      await mobileMenuButton.click();
      await sleep(500);
      // Use Promise.all to start waiting before click to avoid race condition
      await Promise.all([
        page.waitForURL('**/login.html**', { timeout: CONFIG.DEFAULT_TIMEOUT }),
        mobileLogoutLink.click(),
      ]);
      console.log('Logout successful (mobile)');
    } else {
      // Fallback: navigate directly to logout endpoint
      await page.goto('/logout', { timeout: CONFIG.DEFAULT_TIMEOUT });
      await page.waitForURL('**/login.html**', { timeout: CONFIG.DEFAULT_TIMEOUT });
      console.log('Logout successful (direct navigation)');
    }
  } catch (error) {
    console.error('Logout failed:', (error as Error).message);
    throw error;
  }
}

// Page paths
export const PAGES = {
  login: '/login.html',
  index: '/index.html',
  streams: '/streams.html',
  recordings: '/recordings.html',
  timeline: '/timeline.html',
  settings: '/settings.html',
  system: '/system.html',
  users: '/users.html',
  hls: '/hls.html',
};

// API endpoints
export const API = {
  auth: {
    login: '/api/auth/login',
    logout: '/api/auth/logout',
    verify: '/api/auth/verify',
    users: '/api/auth/users',
  },
  streams: '/api/streams',
  recordings: '/api/recordings',
  settings: '/api/settings',
  system: '/api/system',
};

// Export test utilities
export { expect };

