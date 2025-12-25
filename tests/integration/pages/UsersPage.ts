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
    return this.page.locator('button').filter({ hasText: /add|new|create/i }).first();
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
    return this.page.locator('#username, input[name="username"]').first();
  }

  get emailInput(): Locator {
    return this.page.locator('#email, input[name="email"]').first();
  }

  get passwordInput(): Locator {
    return this.page.locator('#password, input[name="password"]').first();
  }

  get roleSelect(): Locator {
    return this.page.locator('#role, select[name="role"]').first();
  }

  get isActiveCheckbox(): Locator {
    return this.page.locator('input[name="is_active"]').first();
  }

  get saveButton(): Locator {
    return this.page.locator('button').filter({ hasText: /add user|save|submit/i }).first();
  }

  get cancelButton(): Locator {
    return this.page.locator('button').filter({ hasText: /cancel/i }).first();
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
   * Click add user button
   */
  async clickAddUser(): Promise<void> {
    await this.addUserButton.click();
    await sleep(500);
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
        await checkbox.check();
      } else {
        await checkbox.uncheck();
      }
    }
    
    await this.saveButton.click();
    await sleep(1000);
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

