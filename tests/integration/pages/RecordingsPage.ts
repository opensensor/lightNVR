/**
 * Recordings Page Object Model
 */

import { Locator } from '@playwright/test';
import { BasePage } from './BasePage';
import { sleep } from '../fixtures/test-fixtures';

export class RecordingsPage extends BasePage {
  protected path = '/recordings.html';

  // Locators
  get pageTitle(): Locator {
    return this.page.locator('h1, .page-title').first();
  }

  get recordingsList(): Locator {
    return this.page.locator('.recordings-list, .recording-grid, [data-testid="recordings-list"]');
  }

  get recordingCards(): Locator {
    return this.page.locator('.recording-card, .recording-item, [data-testid="recording-card"]');
  }

  get streamFilter(): Locator {
    return this.page.locator('select[name="stream"], #stream-filter, [data-testid="stream-filter"]').first();
  }

  get detectionLabelFilter(): Locator {
    return this.page.locator('#detection-label-filter').first();
  }

  get captureMethodFilter(): Locator {
    return this.page.locator('#capture-method-filter').first();
  }

  get tagFilter(): Locator {
    return this.page.locator('#tag-filter').first();
  }

  get applyFiltersButton(): Locator {
    return this.page.locator('#apply-filters-btn').first();
  }

  get activeFilters(): Locator {
    return this.page.locator('#active-filters .filter-tag');
  }

  get filtersSidebarToggleButton(): Locator {
    return this.page.locator('#filters-sidebar button[title="Show filters"], #filters-sidebar button[title="Hide filters"]').first();
  }

  get detectionObjectsSectionButton(): Locator {
    return this.page.locator('#filters-sidebar button').filter({ hasText: /Detection Objects/i }).first();
  }

  get captureMethodSectionButton(): Locator {
    return this.page.locator('#filters-sidebar button').filter({ hasText: /Capture Method/i }).first();
  }

  get recordingTagsSectionButton(): Locator {
    return this.page.locator('#filters-sidebar button').filter({ hasText: /Recording Tags/i }).first();
  }

  get dateFilter(): Locator {
    return this.page.locator('input[type="date"], #date-filter, [data-testid="date-filter"]').first();
  }

  get refreshButton(): Locator {
    return this.page.locator('button').filter({ hasText: /refresh/i }).first();
  }

  get pagination(): Locator {
    return this.page.locator('.pagination, [data-testid="pagination"]');
  }

  get nextPageButton(): Locator {
    return this.page.locator('button').filter({ hasText: /next|>/i }).first();
  }

  get prevPageButton(): Locator {
    return this.page.locator('button').filter({ hasText: /prev|</i }).first();
  }

  get batchDeleteButton(): Locator {
    return this.page.locator('button').filter({ hasText: /batch|delete selected/i }).first();
  }

  get selectAllCheckbox(): Locator {
    return this.page.locator('input[type="checkbox"]').filter({ hasText: /select all/i }).first();
  }

  get emptyStateMessage(): Locator {
    return this.page.locator('.empty-state, .no-recordings, [data-testid="empty-state"]').first();
  }

  /**
   * Get count of recordings displayed
   */
  async getRecordingCount(): Promise<number> {
    await sleep(500);
    return await this.recordingCards.count();
  }

  /**
   * Check if recordings list is empty
   */
  async isEmpty(): Promise<boolean> {
    const count = await this.getRecordingCount();
    return count === 0;
  }

  /**
   * Filter recordings by stream
   */
  async filterByStream(streamName: string): Promise<void> {
    await this.streamFilter.selectOption({ label: streamName });
    await sleep(1000); // Wait for filter to apply
  }

  async expandFiltersSidebar(): Promise<void> {
    if (await this.streamFilter.isVisible()) return;
    await this.filtersSidebarToggleButton.click();
    await this.streamFilter.waitFor({ state: 'visible' });
  }

  async expandDetectionObjectsSection(): Promise<void> {
    await this.expandFiltersSidebar();
    if (await this.detectionLabelFilter.isVisible()) return;
    await this.detectionObjectsSectionButton.click();
    await this.detectionLabelFilter.waitFor({ state: 'visible' });
  }

  async expandCaptureMethodSection(): Promise<void> {
    await this.expandFiltersSidebar();
    if (await this.captureMethodFilter.isVisible()) return;
    await this.captureMethodSectionButton.click();
    await this.captureMethodFilter.waitFor({ state: 'visible' });
  }

  async expandRecordingTagsSection(): Promise<void> {
    await this.expandFiltersSidebar();
    if (await this.tagFilter.isVisible()) return;
    await this.recordingTagsSectionButton.click();
    await this.tagFilter.waitFor({ state: 'visible' });
  }

  async addDetectionLabel(label: string): Promise<void> {
    await this.expandDetectionObjectsSection();
    await this.detectionLabelFilter.fill(label);
    await this.page.getByRole('button', { name: 'Add' }).click();
    await sleep(300);
  }

  async addCaptureMethod(method: string): Promise<void> {
    await this.expandCaptureMethodSection();
    await this.captureMethodFilter.selectOption({ value: method });
    await sleep(300);
  }

  async addTag(tag: string): Promise<void> {
    await this.expandRecordingTagsSection();
    await this.tagFilter.selectOption({ value: tag });
    await sleep(300);
  }

  async applyFilters(): Promise<void> {
    await this.applyFiltersButton.click();
    await sleep(300);
  }

  getActiveFilter(label: string): Locator {
    return this.page.locator('#active-filters .filter-tag').filter({ hasText: label }).first();
  }

  /**
   * Filter recordings by date
   */
  async filterByDate(date: string): Promise<void> {
    await this.dateFilter.fill(date);
    await sleep(1000);
  }

  /**
   * Get a recording card by index
   */
  getRecordingByIndex(index: number): Locator {
    return this.recordingCards.nth(index);
  }

  /**
   * Play a recording
   */
  async playRecording(index: number): Promise<void> {
    const card = this.getRecordingByIndex(index);
    const playButton = card.locator('button').filter({ hasText: /play|view/i });
    await playButton.click();
    await sleep(500);
  }

  /**
   * Download a recording
   */
  async downloadRecording(index: number): Promise<void> {
    const card = this.getRecordingByIndex(index);
    const downloadButton = card.locator('button, a').filter({ hasText: /download/i });
    await downloadButton.click();
    await sleep(500);
  }

  /**
   * Delete a recording
   */
  async deleteRecording(index: number): Promise<void> {
    const card = this.getRecordingByIndex(index);
    const deleteButton = card.locator('button').filter({ hasText: /delete|remove/i });
    await deleteButton.click();
    await sleep(500);
  }

  /**
   * Navigate to next page
   */
  async goToNextPage(): Promise<void> {
    if (await this.nextPageButton.isEnabled()) {
      await this.nextPageButton.click();
      await sleep(1000);
    }
  }

  /**
   * Navigate to previous page
   */
  async goToPrevPage(): Promise<void> {
    if (await this.prevPageButton.isEnabled()) {
      await this.prevPageButton.click();
      await sleep(1000);
    }
  }
}

