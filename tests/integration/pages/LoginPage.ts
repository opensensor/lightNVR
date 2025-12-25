/**
 * Login Page Object Model
 */

import { Page, Locator, expect } from '@playwright/test';
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
   * Login with credentials
   */
  async login(
    username: string = USERS.admin.username,
    password: string = USERS.admin.password
  ): Promise<void> {
    await this.goto();
    await this.usernameInput.fill(username);
    await this.passwordInput.fill(password);
    await this.submitButton.click();
  }

  /**
   * Login and expect success (redirect to index)
   */
  async loginExpectSuccess(
    username: string = USERS.admin.username,
    password: string = USERS.admin.password
  ): Promise<void> {
    await this.login(username, password);
    await this.page.waitForURL('**/index.html', { timeout: CONFIG.DEFAULT_TIMEOUT });
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

