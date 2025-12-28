/**
 * Timeline Page Object Model
 */

import { Page, Locator, expect } from '@playwright/test';
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
    return this.page.locator('input[type="date"], .date-picker, [data-testid="date-picker"]').first();
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
    await sleep(500);
    return await this.timelineSegments.count();
  }

  /**
   * Check if timeline is empty (no segments)
   */
  async isEmpty(): Promise<boolean> {
    return (await this.getSegmentCount()) === 0;
  }

  /**
   * Navigate to a specific date
   */
  async goToDate(date: string): Promise<void> {
    await this.datePicker.fill(date);
    await sleep(1000);
  }

  /**
   * Go to previous day
   */
  async goToPreviousDay(): Promise<void> {
    await this.prevDayButton.click();
    await sleep(1000);
  }

  /**
   * Go to next day
   */
  async goToNextDay(): Promise<void> {
    await this.nextDayButton.click();
    await sleep(1000);
  }

  /**
   * Go to today
   */
  async goToToday(): Promise<void> {
    if (await this.todayButton.isVisible()) {
      await this.todayButton.click();
      await sleep(1000);
    }
  }

  /**
   * Filter by stream
   */
  async filterByStream(streamName: string): Promise<void> {
    await this.streamFilter.selectOption({ label: streamName });
    await sleep(1000);
  }

  /**
   * Click on a segment to play
   */
  async clickSegment(index: number): Promise<void> {
    await this.timelineSegments.nth(index).click();
    await sleep(500);
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
   * Get the selected date from the date picker
   */
  async getSelectedDate(): Promise<string | null> {
    if (await this.datePicker.isVisible()) {
      return await this.datePicker.inputValue();
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
}

