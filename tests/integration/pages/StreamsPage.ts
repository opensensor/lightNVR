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
    // Use specific ID from StreamsView.jsx
    return this.page.locator('#add-stream-btn, button:has-text("Add Stream")').first();
  }

  get streamsList(): Locator {
    return this.page.locator('#streams-table').first();
  }

  get streamCards(): Locator {
    // Streams are displayed in table rows - target tbody rows in #streams-table
    return this.page.locator('#streams-table tbody tr');
  }

  get refreshButton(): Locator {
    return this.page.locator('button').filter({ hasText: /refresh/i }).first();
  }

  // Modal locators - StreamConfigModal is a fixed overlay
  get addStreamModal(): Locator {
    // The modal is a fixed overlay containing the stream form
    // Look for the visible modal overlay that contains the stream-name field
    return this.page.locator('.fixed.inset-0, div[class*="fixed"][class*="inset-0"]').filter({ has: this.page.locator('#stream-name') }).first();
  }

  get streamNameInput(): Locator {
    return this.page.locator('#stream-name');
  }

  get streamUrlInput(): Locator {
    return this.page.locator('#stream-url');
  }

  get saveButton(): Locator {
    // The save button has btn-primary class and text "Add Stream" or "Update Stream"
    // It's in the modal footer alongside the Cancel button
    // Use the modal locator to ensure we're finding the right button (not the zone config button)
    return this.addStreamModal.locator('button.btn-primary').filter({ hasText: /add stream|update stream/i }).first();
  }

  get cancelButton(): Locator {
    // The cancel button is in the modal with text "Cancel" - scope to the modal
    return this.addStreamModal.locator('button').filter({ hasText: /^cancel$/i }).first();
  }

  /**
   * Get count of streams displayed
   */
  async getStreamCount(): Promise<number> {
    await sleep(1000); // Wait for data to load
    // Wait for the streams table to be visible first
    try {
      await this.streamsList.waitFor({ state: 'visible', timeout: 5000 });
    } catch (e) {
      // Table might not exist if no streams
      return 0;
    }
    await sleep(500); // Wait for list to render
    // Only count rows that have td cells (actual data rows)
    return await this.streamCards.filter({ has: this.page.locator('td') }).count();
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
    await sleep(500);
    const streamRow = this.getStreamByName(name);
    try {
      await streamRow.waitFor({ state: 'visible', timeout: 3000 });
      return true;
    } catch (e) {
      return false;
    }
  }

  /**
   * Click add stream button to open modal/form
   */
  async clickAddStream(): Promise<void> {
    // Wait for button to be ready
    await this.addStreamButton.waitFor({ state: 'visible', timeout: 5000 });
    await sleep(500);
    await this.addStreamButton.click();
    // Wait for the stream form to appear by waiting for name input
    await this.streamNameInput.waitFor({ state: 'visible', timeout: 5000 });
    await sleep(300);
  }

  /**
   * Add a new stream via the UI
   */
  async addStream(config: { name: string; url: string; enabled?: boolean }): Promise<void> {
    await this.clickAddStream();

    // Clear and fill the form fields
    await this.streamNameInput.clear();
    await this.streamNameInput.fill(config.name);
    await this.streamUrlInput.clear();
    await this.streamUrlInput.fill(config.url);

    if (config.enabled !== undefined) {
      const enabledCheckbox = this.page.locator('#stream-enabled, input[name="enabled"]').first();
      if (config.enabled) {
        await enabledCheckbox.check({ force: true });
      } else {
        await enabledCheckbox.uncheck({ force: true });
      }
    }

    // Click save and wait for it to complete
    await this.saveButton.click({ force: true });
    await sleep(2000); // Wait for save to complete and list to refresh
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

