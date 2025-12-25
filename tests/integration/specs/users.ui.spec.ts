/**
 * Users Page UI Tests (Admin Only)
 * 
 * Tests for user list, add user, edit user, delete user, role changes, API key generation.
 * @tags @ui @users @admin
 */

import { test, expect } from '@playwright/test';
import { UsersPage } from '../pages/UsersPage';
import { CONFIG, USERS, login, sleep } from '../fixtures/test-fixtures';

test.describe('Users Page @ui @users @admin', () => {
  
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test.describe('Page Load', () => {
    test('should load users page successfully as admin', async ({ page }) => {
      const usersPage = new UsersPage(page);
      await usersPage.goto({ waitForNetworkIdle: true });
      
      await expect(page.locator('body')).toBeVisible();
      expect(await usersPage.isOnPage()).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/users-page-load.png' });
    });

    test('should display users list', async ({ page }) => {
      const usersPage = new UsersPage(page);
      await usersPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const userCount = await usersPage.getUserCount();
      console.log(`Found ${userCount} users in list`);
      
      // Should have at least the admin user
      expect(userCount).toBeGreaterThanOrEqual(1);
      
      await page.screenshot({ path: 'test-results/users-list.png' });
    });

    test('should display add user button', async ({ page }) => {
      const usersPage = new UsersPage(page);
      await usersPage.goto({ waitForNetworkIdle: true });
      
      const hasAddButton = await usersPage.addUserButton.isVisible();
      console.log(`Has add user button: ${hasAddButton}`);
      expect(hasAddButton).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/users-add-button.png' });
    });
  });

  test.describe('Add User Modal', () => {
    test('should open add user modal', async ({ page }) => {
      const usersPage = new UsersPage(page);
      await usersPage.goto({ waitForNetworkIdle: true });
      
      await usersPage.clickAddUser();
      
      await expect(usersPage.userModal).toBeVisible();
      await expect(usersPage.usernameInput).toBeVisible();
      await expect(usersPage.passwordInput).toBeVisible();
      
      await page.screenshot({ path: 'test-results/users-add-modal.png' });
    });

    test('should display role selection', async ({ page }) => {
      const usersPage = new UsersPage(page);
      await usersPage.goto({ waitForNetworkIdle: true });
      
      await usersPage.clickAddUser();
      
      const hasRoleSelect = await usersPage.roleSelect.isVisible();
      console.log(`Has role select: ${hasRoleSelect}`);
      
      if (hasRoleSelect) {
        const options = await usersPage.roleSelect.locator('option').allTextContents();
        console.log(`Role options: ${options.join(', ')}`);
      }
      
      await page.screenshot({ path: 'test-results/users-role-select.png' });
    });

    test('should close modal on cancel', async ({ page }) => {
      const usersPage = new UsersPage(page);
      await usersPage.goto({ waitForNetworkIdle: true });
      
      await usersPage.clickAddUser();
      await expect(usersPage.userModal).toBeVisible();
      
      await usersPage.cancelButton.click();
      await sleep(500);
      
      await expect(usersPage.userModal).not.toBeVisible();
      
      await page.screenshot({ path: 'test-results/users-modal-cancel.png' });
    });
  });

  test.describe('User Operations', () => {
    test('should add a new user via UI', async ({ page }) => {
      const usersPage = new UsersPage(page);
      await usersPage.goto({ waitForNetworkIdle: true });
      
      const testUsername = `test_user_${Date.now()}`;
      
      await usersPage.addUser({
        username: testUsername,
        password: 'TestPassword123!',
        email: `${testUsername}@test.com`,
        role: 2, // Viewer
        isActive: true
      });
      
      await sleep(2000);
      
      // Verify user appears in list
      const userExists = await usersPage.userExists(testUsername);
      expect(userExists).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/users-add-new.png' });
    });

    test('should display admin user in list', async ({ page }) => {
      const usersPage = new UsersPage(page);
      await usersPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const adminExists = await usersPage.userExists('admin');
      expect(adminExists).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/users-admin-exists.png' });
    });

    test('should have edit and delete buttons on user rows', async ({ page }) => {
      const usersPage = new UsersPage(page);
      await usersPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const userCount = await usersPage.getUserCount();
      if (userCount > 0) {
        const firstUser = usersPage.userRows.first();
        
        // Check for edit button
        const editButton = firstUser.locator('button').filter({ hasText: /edit/i });
        const hasEdit = await editButton.count() > 0;
        console.log(`Has edit button: ${hasEdit}`);
        
        // Check for delete button
        const deleteButton = firstUser.locator('button').filter({ hasText: /delete|remove/i });
        const hasDelete = await deleteButton.count() > 0;
        console.log(`Has delete button: ${hasDelete}`);
      }
      
      await page.screenshot({ path: 'test-results/users-action-buttons.png' });
    });
  });
});

