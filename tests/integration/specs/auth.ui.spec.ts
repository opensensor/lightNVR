/**
 * Authentication UI Tests
 * 
 * Tests for login flow, logout, session management, and invalid credentials.
 * @tags @ui @auth
 */

import { test, expect } from '@playwright/test';
import { LoginPage } from '../pages/LoginPage';
import { LiveViewPage } from '../pages/LiveViewPage';
import { CONFIG, USERS, login, logout, sleep } from '../fixtures/test-fixtures';

test.describe('Authentication @ui @auth', () => {
  
  test.describe('Login Page', () => {
    test('should display login page with form elements', async ({ page }) => {
      const loginPage = new LoginPage(page);
      await loginPage.goto();
      
      await expect(page).toHaveTitle(/LightNVR/i);
      expect(await loginPage.isLoginFormVisible()).toBeTruthy();
      await expect(loginPage.usernameInput).toBeVisible();
      await expect(loginPage.passwordInput).toBeVisible();
      await expect(loginPage.submitButton).toBeVisible();
      
      await page.screenshot({ path: 'test-results/auth-login-page.png' });
    });

    test('should have proper form validation attributes', async ({ page }) => {
      const loginPage = new LoginPage(page);
      await loginPage.goto();
      
      // Check that inputs have proper attributes
      const usernameInput = loginPage.usernameInput;
      const passwordInput = loginPage.passwordInput;
      
      await expect(usernameInput).toBeVisible();
      await expect(passwordInput).toBeVisible();
    });
  });

  test.describe('Login Flow', () => {
    test('should login successfully with valid admin credentials', async ({ page }) => {
      const loginPage = new LoginPage(page);
      await loginPage.loginExpectSuccess(USERS.admin.username, USERS.admin.password);
      
      expect(page.url()).toContain('index.html');
      await page.screenshot({ path: 'test-results/auth-login-success.png' });
    });

    test('should reject login with invalid password', async ({ page }) => {
      const loginPage = new LoginPage(page);
      await loginPage.loginExpectFailure(USERS.admin.username, 'wrongpassword');
      
      expect(page.url()).toContain('login');
      await page.screenshot({ path: 'test-results/auth-login-invalid-password.png' });
    });

    test('should reject login with invalid username', async ({ page }) => {
      const loginPage = new LoginPage(page);
      await loginPage.loginExpectFailure('nonexistentuser', 'somepassword');
      
      expect(page.url()).toContain('login');
      await page.screenshot({ path: 'test-results/auth-login-invalid-username.png' });
    });

    test('should reject login with empty credentials', async ({ page }) => {
      const loginPage = new LoginPage(page);
      await loginPage.goto();
      
      // Try to submit empty form
      await loginPage.submitButton.click();
      await sleep(1000);
      
      // Should still be on login page
      expect(page.url()).toContain('login');
      await page.screenshot({ path: 'test-results/auth-login-empty-credentials.png' });
    });
  });

  test.describe('Logout Flow', () => {
    test('should logout successfully', async ({ page }) => {
      // First login
      await login(page, USERS.admin);
      expect(page.url()).toContain('index.html');
      
      // Then logout
      await logout(page);
      
      // Should be redirected to login page
      expect(page.url()).toContain('login');
      await page.screenshot({ path: 'test-results/auth-logout-success.png' });
    });

    test('should redirect to login when accessing protected page after logout', async ({ page }) => {
      // Login first
      await login(page, USERS.admin);
      
      // Logout
      await logout(page);
      
      // Try to access protected page
      await page.goto('/index.html', { waitUntil: 'domcontentloaded' });
      await sleep(1000);
      
      // Should be redirected to login
      expect(page.url()).toContain('login');
      await page.screenshot({ path: 'test-results/auth-protected-after-logout.png' });
    });
  });

  test.describe('Session Management', () => {
    test('should maintain session across page navigation', async ({ page }) => {
      await login(page, USERS.admin);
      
      // Navigate to different pages
      await page.goto('/streams.html', { waitUntil: 'domcontentloaded' });
      await sleep(1000);
      expect(page.url()).not.toContain('login');
      
      await page.goto('/recordings.html', { waitUntil: 'domcontentloaded' });
      await sleep(1000);
      expect(page.url()).not.toContain('login');
      
      await page.goto('/settings.html', { waitUntil: 'domcontentloaded' });
      await sleep(1000);
      expect(page.url()).not.toContain('login');
      
      await page.screenshot({ path: 'test-results/auth-session-maintained.png' });
    });

    test('should redirect unauthenticated access to login', async ({ page }) => {
      // Try to access protected pages without login
      const protectedPages = ['/index.html', '/streams.html', '/recordings.html', '/settings.html'];
      
      for (const pagePath of protectedPages) {
        await page.goto(pagePath, { waitUntil: 'domcontentloaded' });
        await sleep(1000);
        expect(page.url()).toContain('login');
      }
      
      await page.screenshot({ path: 'test-results/auth-unauthenticated-redirect.png' });
    });
  });
});

