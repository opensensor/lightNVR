/**
 * Streams Page Object Model
 */

import { Locator } from '@playwright/test';
import { BasePage } from './BasePage';
import { sleep } from '../fixtures/test-fixtures';

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

  // Modal locators - StreamConfigModal is a fixed overlay with unique ID
  get addStreamModal(): Locator {
    // The modal has a unique ID 'stream-config-modal'
    return this.page.locator('#stream-config-modal');
  }

  get streamNameInput(): Locator {
    // Scope to the stream config modal to avoid conflicts with other modals
    return this.addStreamModal.locator('#stream-name');
  }

  get streamUrlInput(): Locator {
    // Scope to the stream config modal to avoid conflicts with other modals
    return this.addStreamModal.locator('#stream-url');
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
    // Wait for the streams list to be loaded and potentially refreshed
    await sleep(1000);
    const streamRow = this.getStreamByName(name);
    try {
      // Increase timeout to allow for API refetch and re-render
      await streamRow.waitFor({ state: 'visible', timeout: 10000 });
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

    // Wait for name input to be updated
    await this.page.waitForFunction(
      (name) => {
        const input = document.querySelector('#stream-name') as HTMLInputElement;
        return input && input.value === name;
      },
      config.name,
      { timeout: 5000 }
    );

    await this.streamUrlInput.clear();
    await this.streamUrlInput.fill(config.url);

    // Wait for URL input to be updated
    await this.page.waitForFunction(
      (url) => {
        const input = document.querySelector('#stream-url') as HTMLInputElement;
        return input && input.value === url;
      },
      config.url,
      { timeout: 5000 }
    );

    if (config.enabled !== undefined) {
      // Scope to the modal to avoid conflicts with other modals
      const enabledCheckbox = this.addStreamModal.locator('#stream-enabled, input[name="enabled"]').first();
      if (config.enabled) {
        await enabledCheckbox.check({ force: true });
      } else {
        await enabledCheckbox.uncheck({ force: true });
      }
    }

    // Wait for the save button to be enabled and visible
    await this.saveButton.waitFor({ state: 'visible', timeout: 5000 });

    // Wait for React state to update - this is critical for the mutation to work
    await sleep(500);

    // Set up listeners for both request and response
    const requestPromise = this.page.waitForRequest(
      request => {
        const url = request.url();
        const method = request.method();
        const isPostStreams = method === 'POST' && (url.endsWith('/api/streams') || url.match(/\/api\/streams(\?|$)/));
        if (isPostStreams) {
          console.log(`Detected POST request to: ${url}`);
        }
        return isPostStreams;
      },
      { timeout: 30000 }
    );

    const responsePromise = this.page.waitForResponse(
      response => {
        const url = response.url();
        const method = response.request().method();
        const isPostStreams = method === 'POST' && (url.endsWith('/api/streams') || url.match(/\/api\/streams(\?|$)/));
        if (isPostStreams) {
          console.log(`Detected POST response from: ${url} with status: ${response.status()}`);
        }
        return isPostStreams;
      },
      { timeout: 30000 }
    );

    // Ensure button is in viewport and clickable
    await this.saveButton.scrollIntoViewIfNeeded();

    // Check if button is enabled
    const isEnabled = await this.saveButton.isEnabled();
    console.log(`Save button enabled: ${isEnabled}`);
    if (!isEnabled) {
      await this.page.screenshot({ path: `test-results/stream-button-disabled-${config.name}.png` });
      throw new Error('Save button is disabled');
    }

    // Click save
    console.log(`Clicking save button for stream: ${config.name}`);
    await this.saveButton.click();

    // Wait for request to be sent first
    try {
      await requestPromise;
      console.log('POST request was sent successfully');
    } catch (e) {
      console.error('POST request was never sent:', e.message);
      await this.page.screenshot({ path: `test-results/stream-no-request-${config.name}.png` });
      throw new Error(`POST request was never sent: ${e.message}`);
    }

    // Wait for the API response
    try {
      console.log(`Waiting for POST /api/streams response for stream: ${config.name}`);
      const response = await responsePromise;
      console.log(`Received response with status: ${response.status()}`);
      if (!response.ok()) {
        const body = await response.text().catch(() => 'Unable to read response body');
        throw new Error(`Stream creation failed with status ${response.status()}: ${body}`);
      }
    } catch (e) {
      // Capture screenshot on failure
      console.error(`Failed to create stream ${config.name}:`, e);
      await this.page.screenshot({ path: `test-results/stream-add-failed-${config.name}.png` });

      // Also capture console logs
      const logs = await this.page.evaluate(() => {
        return (window as any).__testLogs || 'No logs captured';
      }).catch(() => 'Unable to capture logs');
      console.log('Browser console logs:', logs);

      throw new Error(`Failed to create stream: ${e.message}`);
    }

    // Wait for the modal to close - this indicates the save was successful
    // The modal closing is triggered by the onSuccess callback in the mutation
    // Note: The modal is unmounted (detached) from DOM, not just hidden
    try {
      await this.addStreamModal.waitFor({ state: 'detached', timeout: 5000 });
    } catch (e) {
      // Modal might have already closed, check if it's still visible
      const modalVisible = await this.addStreamModal.isVisible().catch(() => false);
      if (modalVisible) {
        await this.page.screenshot({ path: `test-results/stream-modal-stuck-${config.name}.png` });
        throw new Error(`Modal did not close after successful save - UI may be stuck`);
      }
      // If modal is not visible, it closed successfully
    }

    // Wait for the streams list to be refreshed (React Query refetch)
    await sleep(1000);
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

