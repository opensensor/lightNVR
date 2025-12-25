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
    return this.page.locator('h1, .page-title').first();
  }

  get addUserButton(): Locator {
    // The main page "Add User" button (not in modal)
    return this.page.locator('button.btn-primary').filter({ hasText: /add user/i }).first();
  }

  get usersList(): Locator {
    return this.page.locator('table').first();
  }

  get userRows(): Locator {
    // User rows are in tbody, skip the header row
    return this.page.locator('tbody tr');
  }

  // Add/Edit User Modal - uses fixed positioning with bg-black overlay
  get userModal(): Locator {
    return this.page.locator('.fixed.inset-0').filter({ hasText: /add new user|edit user/i }).first();
  }

  get usernameInput(): Locator {
    // Input in the modal form
    return this.userModal.locator('#username');
  }

  get emailInput(): Locator {
    return this.userModal.locator('#email');
  }

  get passwordInput(): Locator {
    return this.userModal.locator('#password');
  }

  get roleSelect(): Locator {
    return this.userModal.locator('#role');
  }

  get isActiveCheckbox(): Locator {
    return this.userModal.locator('input[name="is_active"]');
  }

  get saveButton(): Locator {
    // The submit button inside the modal (type="submit")
    return this.userModal.locator('button[type="submit"]');
  }

  get cancelButton(): Locator {
    return this.userModal.locator('button').filter({ hasText: /cancel/i }).first();
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
    await sleep(500);
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
    return await this.getUserByUsername(username).isVisible();
  }

  /**
   * Click add user button and wait for modal to open
   */
  async clickAddUser(): Promise<void> {
    await this.addUserButton.click();
    // Wait for the modal to appear by waiting for the username input
    await this.userModal.waitFor({ state: 'visible', timeout: 5000 });
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

    // Wait for modal to close and list to refresh
    await this.userModal.waitFor({ state: 'hidden', timeout: 10000 });
    await sleep(2000); // Additional wait for API response and list refresh
  }

  /**
   * Edit user
   */
  async editUser(username: string): Promise<void> {
    const userRow = this.getUserByUsername(username);
    const editButton = userRow.locator('button').filter({ hasText: /edit/i });
    await editButton.click();
    await sleep(500);
  }

  /**
   * Delete user
   */
  async deleteUser(username: string): Promise<void> {
    const userRow = this.getUserByUsername(username);
    const deleteButton = userRow.locator('button').filter({ hasText: /delete|remove/i });
    await deleteButton.click();
    await sleep(500);
  }

  /**
   * Generate API key for user
   */
  async generateApiKey(username: string): Promise<void> {
    const userRow = this.getUserByUsername(username);
    const apiKeyButton = userRow.locator('button').filter({ hasText: /api key|generate/i });
    await apiKeyButton.click();
    await sleep(500);
  }
}

