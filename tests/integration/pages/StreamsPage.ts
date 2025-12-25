/**
 * Streams Page Object Model
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from './BasePage';
import { CONFIG, sleep } from '../fixtures/test-fixtures';

export class StreamsPage extends BasePage {
  protected path = '/streams.html';

  // Locators
  get pageTitle(): Locator {
    return this.page.locator('h1, .page-title').first();
  }

  get addStreamButton(): Locator {
    return this.page.locator('button').filter({ hasText: /add|new|create/i }).first();
  }

  get streamsList(): Locator {
    return this.page.locator('#streams-table, .streams-container, table');
  }

  get streamCards(): Locator {
    // Streams are displayed in table rows, not cards - target tbody rows
    return this.page.locator('#streams-table tbody tr, .streams-container tbody tr');
  }

  get refreshButton(): Locator {
    return this.page.locator('button').filter({ hasText: /refresh/i }).first();
  }

  // Modal locators - match the actual modal implementation which uses fixed positioning
  get addStreamModal(): Locator {
    return this.page.locator('.fixed.inset-0').filter({ hasText: /add stream|edit stream/i }).first();
  }

  get streamNameInput(): Locator {
    return this.page.locator('#stream-name, input[name="name"]').first();
  }

  get streamUrlInput(): Locator {
    return this.page.locator('#stream-url, input[name="url"]').first();
  }

  get saveButton(): Locator {
    return this.page.locator('button').filter({ hasText: /save|submit|create/i }).first();
  }

  get cancelButton(): Locator {
    // The close button in the modal header has an X icon, look for cancel text or close button
    return this.page.locator('button').filter({ hasText: /cancel/i }).first();
  }

  /**
   * Get count of streams displayed
   */
  async getStreamCount(): Promise<number> {
    await sleep(500); // Wait for list to render
    return await this.streamCards.count();
  }

  /**
   * Get a stream card by name
   */
  getStreamByName(name: string): Locator {
    return this.streamCards.filter({ hasText: name }).first();
  }

  /**
   * Check if a stream exists by name
   */
  async streamExists(name: string): Promise<boolean> {
    return await this.getStreamByName(name).isVisible();
  }

  /**
   * Click add stream button to open modal
   */
  async clickAddStream(): Promise<void> {
    await this.addStreamButton.click();
    await sleep(500); // Wait for modal animation
  }

  /**
   * Add a new stream via the UI
   */
  async addStream(config: { name: string; url: string; enabled?: boolean }): Promise<void> {
    await this.clickAddStream();
    await this.streamNameInput.fill(config.name);
    await this.streamUrlInput.fill(config.url);

    if (config.enabled !== undefined) {
      const enabledCheckbox = this.page.locator('input[name="enabled"], #enabled').first();
      if (config.enabled) {
        await enabledCheckbox.check();
      } else {
        await enabledCheckbox.uncheck();
      }
    }

    // Use force: true because the modal backdrop intercepts pointer events
    await this.saveButton.click({ force: true });
    await sleep(1000); // Wait for save
  }

  /**
   * Click edit button on a stream card
   */
  async editStream(name: string): Promise<void> {
    const streamCard = this.getStreamByName(name);
    const editButton = streamCard.locator('button').filter({ hasText: /edit/i });
    await editButton.click();
    await sleep(500);
  }

  /**
   * Click delete button on a stream card
   */
  async deleteStream(name: string): Promise<void> {
    const streamCard = this.getStreamByName(name);
    const deleteButton = streamCard.locator('button').filter({ hasText: /delete|remove/i });
    await deleteButton.click();
    await sleep(500);
  }

  /**
   * Get stream status
   */
  async getStreamStatus(name: string): Promise<string | null> {
    const streamCard = this.getStreamByName(name);
    const statusBadge = streamCard.locator('.status, .badge, [data-status]').first();
    return await statusBadge.textContent();
  }
}

