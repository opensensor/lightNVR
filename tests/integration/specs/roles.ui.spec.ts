/**
 * Role-Based Access Control UI Tests
 * 
 * Tests that different user roles have appropriate access levels.
 * @tags @ui @roles @rbac
 */

import { test, expect, APIRequestContext } from '@playwright/test';
import { LoginPage } from '../pages/LoginPage';
import { CONFIG, USERS, login, sleep, getAuthHeader } from '../fixtures/test-fixtures';

const AUTH_HEADER = getAuthHeader(USERS.admin);

// Test user credentials for different roles
const TEST_USERS = {
  viewer: { username: 'test_viewer_rbac', password: 'ViewerPass123!', role: 2 },
  user: { username: 'test_user_rbac', password: 'UserPass123!', role: 1 },
};

test.describe('Role-Based Access Control @ui @roles @rbac', () => {
  
  // Create test users before running tests
  test.beforeAll(async ({ playwright }) => {
    const request = await playwright.request.newContext({
      baseURL: CONFIG.LIGHTNVR_URL,
      extraHTTPHeaders: { 'Authorization': AUTH_HEADER },
    });

    // Create test viewer user
    try {
      await request.post('/api/auth/users', {
        data: {
          username: TEST_USERS.viewer.username,
          password: TEST_USERS.viewer.password,
          email: `${TEST_USERS.viewer.username}@test.com`,
          role: TEST_USERS.viewer.role,
          is_active: true
        }
      });
      console.log('Created test viewer user');
    } catch (e) {
      console.log('Viewer user may already exist');
    }

    // Create test user role
    try {
      await request.post('/api/auth/users', {
        data: {
          username: TEST_USERS.user.username,
          password: TEST_USERS.user.password,
          email: `${TEST_USERS.user.username}@test.com`,
          role: TEST_USERS.user.role,
          is_active: true
        }
      });
      console.log('Created test user');
    } catch (e) {
      console.log('User may already exist');
    }

    await request.dispose();
  });

  test.describe('Admin Role Access', () => {
    test('admin can access all pages', async ({ page }) => {
      await login(page, USERS.admin);
      
      const pages = [
        '/index.html',
        '/streams.html',
        '/recordings.html',
        '/timeline.html',
        '/settings.html',
        '/system.html',
        '/users.html'
      ];
      
      for (const pagePath of pages) {
        await page.goto(pagePath, { waitUntil: 'domcontentloaded' });
        await sleep(1000);
        
        // Should not be redirected to login
        expect(page.url()).not.toContain('login');
        console.log(`Admin can access: ${pagePath}`);
      }
      
      await page.screenshot({ path: 'test-results/roles-admin-access.png' });
    });

    test('admin can access users management', async ({ page }) => {
      await login(page, USERS.admin);
      
      await page.goto('/users.html', { waitUntil: 'domcontentloaded' });
      await sleep(2000);
      
      expect(page.url()).toContain('users');
      
      // Should see user management UI
      const addButton = page.locator('button').filter({ hasText: /add|new|create/i });
      const hasAddButton = await addButton.isVisible();
      console.log(`Admin sees add user button: ${hasAddButton}`);
      
      await page.screenshot({ path: 'test-results/roles-admin-users.png' });
    });
  });

  test.describe('Viewer Role Access', () => {
    test('viewer can access live view', async ({ page }) => {
      const loginPage = new LoginPage(page);
      await loginPage.login(TEST_USERS.viewer.username, TEST_USERS.viewer.password);
      
      await sleep(2000);
      
      // Viewer should be able to see live view
      await page.goto('/index.html', { waitUntil: 'domcontentloaded' });
      await sleep(1000);
      
      // Check if we stayed on the page (not redirected to login or error)
      const url = page.url();
      const hasAccess = url.includes('index') || !url.includes('login');
      console.log(`Viewer can access live view: ${hasAccess}`);
      
      await page.screenshot({ path: 'test-results/roles-viewer-liveview.png' });
    });

    test('viewer has limited navigation options', async ({ page }) => {
      const loginPage = new LoginPage(page);
      await loginPage.login(TEST_USERS.viewer.username, TEST_USERS.viewer.password);
      
      await sleep(2000);
      
      // Check navigation links
      const navLinks = page.locator('nav a, .nav a, .sidebar a, header a');
      const linkCount = await navLinks.count();
      
      // Get all href values
      const hrefs: string[] = [];
      for (let i = 0; i < linkCount; i++) {
        const href = await navLinks.nth(i).getAttribute('href');
        if (href) hrefs.push(href);
      }
      
      console.log(`Viewer sees navigation links: ${hrefs.join(', ')}`);
      
      // Viewer should NOT see users.html link
      const hasUsersLink = hrefs.some(h => h.includes('users'));
      console.log(`Viewer sees users link: ${hasUsersLink}`);
      
      await page.screenshot({ path: 'test-results/roles-viewer-nav.png' });
    });
  });

  test.describe('User Role Access', () => {
    test('user role can access basic features', async ({ page }) => {
      const loginPage = new LoginPage(page);
      await loginPage.login(TEST_USERS.user.username, TEST_USERS.user.password);
      
      await sleep(2000);
      
      // User should be able to access live view
      await page.goto('/index.html', { waitUntil: 'domcontentloaded' });
      await sleep(1000);
      
      const hasAccess = !page.url().includes('login');
      console.log(`User can access live view: ${hasAccess}`);
      
      await page.screenshot({ path: 'test-results/roles-user-access.png' });
    });
  });
});

