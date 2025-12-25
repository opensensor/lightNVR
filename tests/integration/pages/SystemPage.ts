/**
 * System Page Object Model
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from './BasePage';
import { CONFIG, sleep } from '../fixtures/test-fixtures';

export class SystemPage extends BasePage {
  protected path = '/system.html';

  // Locators
  get pageTitle(): Locator {
    return this.page.locator('h1, .page-title').first();
  }

  get versionInfo(): Locator {
    return this.page.locator('[data-testid="version"], .version, :text-matches("version", "i")').first();
  }

  get uptimeInfo(): Locator {
    return this.page.locator('[data-testid="uptime"], .uptime, :text-matches("uptime", "i")').first();
  }

  get cpuUsage(): Locator {
    return this.page.locator('[data-testid="cpu-usage"], .cpu-usage, :text-matches("cpu", "i")').first();
  }

  get memoryUsage(): Locator {
    return this.page.locator('[data-testid="memory-usage"], .memory-usage, :text-matches("memory", "i")').first();
  }

  get storageUsage(): Locator {
    return this.page.locator('[data-testid="storage-usage"], .storage-usage, :text-matches("storage", "i")').first();
  }

  get streamsInfo(): Locator {
    return this.page.locator('[data-testid="streams-info"], .streams-info, :text-matches("streams", "i")').first();
  }

  get recordingsInfo(): Locator {
    return this.page.locator('[data-testid="recordings-info"], .recordings-info, :text-matches("recordings", "i")').first();
  }

  // Action buttons
  get restartButton(): Locator {
    return this.page.locator('button').filter({ hasText: /restart/i }).first();
  }

  get shutdownButton(): Locator {
    return this.page.locator('button').filter({ hasText: /shutdown|stop/i }).first();
  }

  get refreshButton(): Locator {
    return this.page.locator('button').filter({ hasText: /refresh/i }).first();
  }

  // Logs section
  get logsSection(): Locator {
    return this.page.locator('.logs-section, [data-testid="logs"]');
  }

  get logsContent(): Locator {
    return this.page.locator('.logs-content, pre, code, [data-testid="logs-content"]');
  }

  get clearLogsButton(): Locator {
    return this.page.locator('button').filter({ hasText: /clear logs/i }).first();
  }

  get downloadLogsButton(): Locator {
    return this.page.locator('button, a').filter({ hasText: /download logs/i }).first();
  }

  // Confirmation dialogs
  get confirmDialog(): Locator {
    return this.page.locator('.modal, [role="dialog"], .confirm-dialog');
  }

  get confirmButton(): Locator {
    return this.page.locator('button').filter({ hasText: /confirm|yes|ok/i }).first();
  }

  get cancelButton(): Locator {
    return this.page.locator('button').filter({ hasText: /cancel|no/i }).first();
  }

  /**
   * Get system version
   */
  async getVersion(): Promise<string | null> {
    if (await this.versionInfo.isVisible()) {
      return await this.versionInfo.textContent();
    }
    return null;
  }

  /**
   * Refresh system info
   */
  async refresh(): Promise<void> {
    if (await this.refreshButton.isVisible()) {
      await this.refreshButton.click();
      await sleep(1000);
    }
  }

  /**
   * Click restart (but don't confirm - test should handle confirmation)
   */
  async clickRestart(): Promise<void> {
    await this.restartButton.click();
    await sleep(500);
  }

  /**
   * Check if confirmation dialog is visible
   */
  async isConfirmDialogVisible(): Promise<boolean> {
    return await this.confirmDialog.isVisible();
  }

  /**
   * Confirm action in dialog
   */
  async confirmAction(): Promise<void> {
    await this.confirmButton.click();
    await sleep(500);
  }

  /**
   * Cancel action in dialog
   */
  async cancelAction(): Promise<void> {
    await this.cancelButton.click();
    await sleep(500);
  }

  /**
   * Check if logs section is visible
   */
  async isLogsSectionVisible(): Promise<boolean> {
    return await this.logsSection.isVisible();
  }

  /**
   * Get logs content
   */
  async getLogsContent(): Promise<string | null> {
    if (await this.logsContent.isVisible()) {
      return await this.logsContent.textContent();
    }
    return null;
  }
}

