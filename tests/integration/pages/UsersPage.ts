/**
 * Users Page Object Model
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from './BasePage';
import { CONFIG, sleep } from '../fixtures/test-fixtures';

export class UsersPage extends BasePage {
  protected path = '/users.html';

  // Locators
  get pageTitle(): Locator {
    // UsersView has h2 with "User Management"
    return this.page.locator('h2').filter({ hasText: /user management/i }).first();
  }

  get addUserButton(): Locator {
    // The main page "Add User" button is a btn-primary in the header section
    // NOT inside the modal (modal has .fixed.inset-0 wrapper)
    // Target button that's NOT inside a modal overlay
    return this.page.locator('div.mb-4 button.btn-primary').filter({ hasText: /add user/i }).first();
  }

  get usersList(): Locator {
    return this.page.locator('table').first();
  }

  get userRows(): Locator {
    // User rows are in tbody - target rows that contain user data (have td cells)
    // Exclude header rows and empty states
    return this.page.locator('table tbody tr').filter({ has: this.page.locator('td') });
  }

  // Add/Edit User Modal - uses fixed positioning with bg-black overlay
  get userModal(): Locator {
    // The modal overlay contains a form with #username input
    // Look for the visible modal overlay that contains the username field
    return this.page.locator('.fixed.inset-0, div[class*="fixed"][class*="inset-0"]').filter({ has: this.page.locator('#username') }).first();
  }

  get usernameInput(): Locator {
    // Input in the modal form - use page-level locator since modal might not be found yet
    return this.page.locator('#username');
  }

  get emailInput(): Locator {
    return this.page.locator('#email');
  }

  get passwordInput(): Locator {
    return this.page.locator('#password');
  }

  get roleSelect(): Locator {
    return this.page.locator('#role');
  }

  get isActiveCheckbox(): Locator {
    return this.page.locator('input[name="is_active"]');
  }

  get saveButton(): Locator {
    // The submit button inside the modal - "Add User" button with type="submit" and btn-primary class
    // Scope to the modal to avoid matching the page-level "Add User" button
    return this.userModal.locator('button[type="submit"].btn-primary').first();
  }

  get cancelButton(): Locator {
    // The cancel button is in the modal, has type="button" and text "Cancel"
    // Scope to the modal to avoid matching other cancel buttons
    return this.userModal.locator('button[type="button"]').filter({ hasText: /^cancel$/i }).first();
  }

  // Confirmation dialog
  get confirmDialog(): Locator {
    return this.page.locator('.confirm-dialog, [role="alertdialog"]');
  }

  get confirmButton(): Locator {
    return this.page.locator('button').filter({ hasText: /confirm|yes|delete/i }).first();
  }

  /**
   * Get count of users
   */
  async getUserCount(): Promise<number> {
    await sleep(1000);
    // Wait for table to be visible first
    try {
      await this.usersList.waitFor({ state: 'visible', timeout: 5000 });
    } catch (e) {
      // Table might not exist if no users
      return 0;
    }
    return await this.userRows.count();
  }

  /**
   * Get user row by username
   */
  getUserByUsername(username: string): Locator {
    return this.userRows.filter({ hasText: username }).first();
  }

  /**
   * Check if user exists
   */
  async userExists(username: string): Promise<boolean> {
    await sleep(500);
    const userRow = this.getUserByUsername(username);
    try {
      await userRow.waitFor({ state: 'visible', timeout: 3000 });
      return true;
    } catch (e) {
      return false;
    }
  }

  /**
   * Click add user button and wait for modal to open
   */
  async clickAddUser(): Promise<void> {
    // Wait for the add user button to be ready
    await this.addUserButton.waitFor({ state: 'visible', timeout: 5000 });
    await sleep(500);
    await this.addUserButton.click();
    // Wait for the username input to appear (indicates modal is open)
    await this.usernameInput.waitFor({ state: 'visible', timeout: 5000 });
    await sleep(300);
  }

  /**
   * Add a new user via UI
   */
  async addUser(config: {
    username: string;
    password: string;
    email?: string;
    role?: number;
    isActive?: boolean
  }): Promise<void> {
    await this.clickAddUser();

    // Fill the form fields
    await this.usernameInput.fill(config.username);
    await this.passwordInput.fill(config.password);

    if (config.email) {
      await this.emailInput.fill(config.email);
    }

    if (config.role !== undefined) {
      await this.roleSelect.selectOption({ value: String(config.role) });
    }

    if (config.isActive !== undefined) {
      const checkbox = this.isActiveCheckbox;
      if (config.isActive) {
        await checkbox.check({ force: true });
      } else {
        await checkbox.uncheck({ force: true });
      }
    }

    // Click save - using force because modal backdrop may intercept events
    await this.saveButton.click({ force: true });

    // Wait for username input to disappear (modal closed)
    await this.usernameInput.waitFor({ state: 'hidden', timeout: 10000 });
    await sleep(2000); // Additional wait for API response and list refresh
  }

  /**
   * Edit user - button has title="Edit User" with SVG icon
   */
  async editUser(username: string): Promise<void> {
    const userRow = this.getUserByUsername(username);
    const editButton = userRow.locator('button[title="Edit User"]');
    await editButton.click();
    await sleep(500);
  }

  /**
   * Delete user - button has title="Delete User" with SVG icon
   */
  async deleteUser(username: string): Promise<void> {
    const userRow = this.getUserByUsername(username);
    const deleteButton = userRow.locator('button[title="Delete User"]');
    await deleteButton.click();
    await sleep(500);
  }

  /**
   * Generate API key for user - button has title="Manage API Key"
   */
  async generateApiKey(username: string): Promise<void> {
    const userRow = this.getUserByUsername(username);
    const apiKeyButton = userRow.locator('button[title="Manage API Key"]');
    await apiKeyButton.click();
    await sleep(500);
  }
}

