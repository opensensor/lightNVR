/**
 * Timeline Page Object Model
 */

import { Locator } from '@playwright/test';
import { BasePage } from './BasePage';
import { CONFIG, sleep } from '../fixtures/test-fixtures';

export class TimelinePage extends BasePage {
  protected path = '/timeline.html';

  // Locators
  get pageTitle(): Locator {
    return this.page.locator('h1, .page-title').first();
  }

  get timelineContainer(): Locator {
    return this.page.locator('.timeline-container, .timeline, [data-testid="timeline"]');
  }

  get timelineSegments(): Locator {
    return this.page.locator('.timeline-segment, .segment, [data-testid="segment"]');
  }

  get datePicker(): Locator {
    return this.page.locator('[data-testid="date-picker"]').first();
  }

  get dateDisplay(): Locator {
    return this.page.locator('[data-testid="date-display"]').first();
  }

  get prevDayButton(): Locator {
    return this.page.locator('button').filter({ hasText: /prev|</i }).first();
  }

  get nextDayButton(): Locator {
    return this.page.locator('button').filter({ hasText: /next|>/i }).first();
  }

  get todayButton(): Locator {
    return this.page.locator('button').filter({ hasText: /today/i }).first();
  }

  get streamFilter(): Locator {
    return this.page.locator('select[name="stream"], #stream-filter, [data-testid="stream-filter"]').first();
  }

  get playhead(): Locator {
    return this.page.locator('.playhead, [data-testid="playhead"]').first();
  }

  get timeDisplay(): Locator {
    return this.page.locator('.time-display, [data-testid="time-display"]').first();
  }

  get videoPlayer(): Locator {
    return this.page.locator('video, .video-player, [data-testid="video-player"]').first();
  }

  get loadingIndicator(): Locator {
    return this.page.locator('.loading, .spinner, [data-testid="loading"]').first();
  }

  /**
   * Get count of segments on the timeline
   */
  async getSegmentCount(): Promise<number> {
    return await this.timelineSegments.count();
  }

  /**
   * Check if timeline is empty (no segments)
   */
  async isEmpty(): Promise<boolean> {
    return (await this.getSegmentCount()) === 0;
  }

  /**
   * Navigate to a specific date using URL navigation
   * (CalendarPicker is a custom component, not a native date input)
   */
  async goToDate(date: string): Promise<void> {
    const currentUrl = new URL(this.page.url());
    currentUrl.searchParams.set('date', date);
    await this.page.goto(currentUrl.toString(), {
      waitUntil: 'networkidle',
      timeout: CONFIG.DEFAULT_TIMEOUT
    });
    await this.waitForDateSyncWithUrl();
  }

  /**
   * Go to previous day
   */
  async goToPreviousDay(): Promise<void> {
    await this.prevDayButton.click();
    await this.waitForDateSyncWithUrl();
  }

  /**
   * Go to next day
   */
  async goToNextDay(): Promise<void> {
    await this.nextDayButton.click();
    await this.waitForDateSyncWithUrl();
  }

  /**
   * Go to today
   */
  async goToToday(): Promise<void> {
    if (await this.todayButton.isVisible()) {
      await this.todayButton.click();
      await this.waitForDateSyncWithUrl();
    }
  }

  /**
   * Filter by stream
   */
  async filterByStream(streamName: string): Promise<void> {
    await this.streamFilter.selectOption({ label: streamName });
    await this.waitForStreamSelection(streamName);
  }

  /**
   * Click on a segment to play
   */
  async clickSegment(index: number): Promise<void> {
    await this.timelineSegments.nth(index).click();
    await this.videoPlayer.waitFor({ state: 'visible', timeout: CONFIG.DEFAULT_TIMEOUT });
  }

  /**
   * Check if video player is visible
   */
  async isVideoPlayerVisible(): Promise<boolean> {
    return await this.videoPlayer.isVisible();
  }

  /**
   * Get current time display
   */
  async getCurrentTime(): Promise<string | null> {
    if (await this.timeDisplay.isVisible()) {
      return await this.timeDisplay.textContent();
    }
    return null;
  }

  /**
   * Get the selected date from the date picker's data-value attribute
   */
  async getSelectedDate(): Promise<string | null> {
    if (await this.dateDisplay.isVisible()) {
      return await this.dateDisplay.getAttribute('data-value');
    }
    return null;
  }

  /**
   * Get the date from the URL parameter
   */
  getDateFromUrl(): string | null {
    const url = new URL(this.page.url());
    return url.searchParams.get('date');
  }

  /**
   * Get the currently selected stream from the stream filter
   */
  async getSelectedStream(): Promise<string | null> {
    if (await this.streamFilter.isVisible()) {
      return await this.streamFilter.inputValue();
    }
    return null;
  }

  /**
   * Get today's date in YYYY-MM-DD format
   */
  static getTodayDate(): string {
    const today = new Date();
    const year = today.getFullYear();
    const month = String(today.getMonth() + 1).padStart(2, '0');
    const day = String(today.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
  }

  /**
   * Wait until the selected date in the UI matches the date in the URL.
   * This replaces fixed sleeps after date navigation with a condition-based wait.
   */
  private async waitForDateSyncWithUrl(timeout: number = CONFIG.DEFAULT_TIMEOUT): Promise<void> {
    const start = Date.now();
    // Poll periodically until the selected date matches the date in the URL or timeout elapses.
    while (Date.now() - start < timeout) {
      const [selectedDate, urlDate] = await Promise.all([
        this.getSelectedDate(),
        Promise.resolve(this.getDateFromUrl())
      ]);
      if (selectedDate && urlDate && selectedDate === urlDate) {
        return;
      }
      await sleep(100);
    }
  }

  /**
   * Wait until the stream filter shows the expected stream name.
   * This replaces fixed sleeps after changing the stream filter.
   */
  private async waitForStreamSelection(streamName: string, timeout: number = CONFIG.DEFAULT_TIMEOUT): Promise<void> {
    const start = Date.now();
    while (Date.now() - start < timeout) {
      const selectedStream = await this.getSelectedStream();
      if (selectedStream === streamName) {
        return;
      }
      await sleep(100);
    }
  }
}

