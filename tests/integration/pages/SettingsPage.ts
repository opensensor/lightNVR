/**
 * Settings Page Object Model
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from './BasePage';
import { CONFIG, sleep } from '../fixtures/test-fixtures';

export class SettingsPage extends BasePage {
  protected path = '/settings.html';

  // Locators
  get pageTitle(): Locator {
    return this.page.locator('h1, .page-title').first();
  }

  get saveButton(): Locator {
    return this.page.locator('button').filter({ hasText: /save|apply/i }).first();
  }

  get cancelButton(): Locator {
    return this.page.locator('button').filter({ hasText: /cancel|reset/i }).first();
  }

  get successMessage(): Locator {
    return this.page.locator('.success, .alert-success, [role="alert"]').filter({ hasText: /success|saved/i }).first();
  }

  get errorMessage(): Locator {
    return this.page.locator('.error, .alert-error, [role="alert"]').filter({ hasText: /error|failed/i }).first();
  }

  // Storage settings
  get storagePathInput(): Locator {
    return this.page.locator('input[name="storage_path"], #storage-path, #storage_path').first();
  }

  get maxStorageSizeInput(): Locator {
    return this.page.locator('input[name="max_storage_size"], #max-storage-size, #max_storage_size').first();
  }

  get retentionDaysInput(): Locator {
    return this.page.locator('input[name="retention_days"], #retention-days, #retention_days').first();
  }

  // Recording settings
  get defaultRecordCheckbox(): Locator {
    return this.page.locator('input[name="default_record"], #default-record').first();
  }

  get segmentDurationInput(): Locator {
    return this.page.locator('input[name="segment_duration"], #segment-duration, #segment_duration').first();
  }

  // Detection settings
  get detectionEnabledCheckbox(): Locator {
    return this.page.locator('input[name="detection_enabled"], #detection-enabled').first();
  }

  get detectionIntervalInput(): Locator {
    return this.page.locator('input[name="detection_interval"], #detection-interval').first();
  }

  // Web settings
  get webPortInput(): Locator {
    return this.page.locator('input[name="web_port"], #web-port, #web_port').first();
  }

  get authEnabledCheckbox(): Locator {
    return this.page.locator('input[name="web_auth_enabled"], #auth-enabled').first();
  }

  /**
   * Save settings
   */
  async saveSettings(): Promise<void> {
    await this.saveButton.click();
    await sleep(1000);
  }

  /**
   * Cancel changes
   */
  async cancelChanges(): Promise<void> {
    await this.cancelButton.click();
    await sleep(500);
  }

  /**
   * Check if save was successful
   */
  async isSaveSuccessful(): Promise<boolean> {
    try {
      await this.successMessage.waitFor({ state: 'visible', timeout: 5000 });
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Get storage path value
   */
  async getStoragePath(): Promise<string> {
    return await this.storagePathInput.inputValue();
  }

  /**
   * Set storage path
   */
  async setStoragePath(path: string): Promise<void> {
    await this.storagePathInput.fill(path);
  }

  /**
   * Get retention days value
   */
  async getRetentionDays(): Promise<string> {
    return await this.retentionDaysInput.inputValue();
  }

  /**
   * Set retention days
   */
  async setRetentionDays(days: string): Promise<void> {
    await this.retentionDaysInput.fill(days);
  }

  /**
   * Get current settings from the form
   */
  async getCurrentSettings(): Promise<Record<string, string | boolean>> {
    const settings: Record<string, string | boolean> = {};
    
    if (await this.storagePathInput.isVisible()) {
      settings['storage_path'] = await this.storagePathInput.inputValue();
    }
    if (await this.retentionDaysInput.isVisible()) {
      settings['retention_days'] = await this.retentionDaysInput.inputValue();
    }
    if (await this.webPortInput.isVisible()) {
      settings['web_port'] = await this.webPortInput.inputValue();
    }
    
    return settings;
  }
}

