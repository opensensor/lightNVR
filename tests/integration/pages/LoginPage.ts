/**
 * Login Page Object Model
 */

import { Locator, expect } from '@playwright/test';
import { BasePage } from './BasePage';
import { CONFIG, USERS, sleep } from '../fixtures/test-fixtures';

export class LoginPage extends BasePage {
  protected path = '/login.html';

  // Locators
  get usernameInput(): Locator {
    return this.page.locator('input[name="username"], input#username, input[type="text"]').first();
  }

  get passwordInput(): Locator {
    return this.page.locator('input[name="password"], input#password, input[type="password"]').first();
  }

  get submitButton(): Locator {
    return this.page.locator('button[type="submit"], button').filter({ hasText: /login|sign in/i }).first();
  }

  get errorMessage(): Locator {
    return this.page.locator('.error, .alert-error, .error-message, [role="alert"]').first();
  }

  get loginForm(): Locator {
    return this.page.locator('form#login-form, form').first();
  }

  /**
   * Login with credentials (fills form and clicks submit, does not wait for navigation)
   */
  async fillAndSubmit(
    username: string = USERS.admin.username,
    password: string = USERS.admin.password
  ): Promise<void> {
    await this.goto();
    await this.usernameInput.fill(username);
    await this.passwordInput.fill(password);
    await this.submitButton.click();
  }

  /**
   * Login with credentials - deprecated, use loginExpectSuccess for navigation
   */
  async login(
    username: string = USERS.admin.username,
    password: string = USERS.admin.password
  ): Promise<void> {
    await this.fillAndSubmit(username, password);
  }

  /**
   * Login and expect success (redirect to index)
   * Uses Promise.all to avoid race condition with window.location.href redirect
   */
  async loginExpectSuccess(
    username: string = USERS.admin.username,
    password: string = USERS.admin.password
  ): Promise<void> {
    await this.goto();
    await this.usernameInput.fill(username);
    await this.passwordInput.fill(password);

    // Use Promise.all to start waiting for navigation BEFORE clicking
    // This is critical because window.location.href redirects can happen very quickly
    await Promise.all([
      this.page.waitForURL('**/index.html', { timeout: CONFIG.DEFAULT_TIMEOUT }),
      this.submitButton.click(),
    ]);
  }

  /**
   * Login and expect failure (stay on login page with error)
   */
  async loginExpectFailure(
    username: string,
    password: string
  ): Promise<void> {
    await this.login(username, password);
    // Should stay on login page
    await sleep(2000);
    expect(this.page.url()).toContain('login');
  }

  /**
   * Check if login form is visible
   */
  async isLoginFormVisible(): Promise<boolean> {
    return await this.loginForm.isVisible();
  }

  /**
   * Get error message text
   */
  async getErrorMessage(): Promise<string | null> {
    if (await this.errorMessage.isVisible()) {
      return await this.errorMessage.textContent();
    }
    return null;
  }
}

