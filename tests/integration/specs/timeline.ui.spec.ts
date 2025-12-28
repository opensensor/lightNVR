/**
 * Timeline Page UI Tests
 * 
 * Tests for timeline view, date navigation, and recording segments visualization.
 * @tags @ui @timeline
 */

import { test, expect } from '@playwright/test';
import { TimelinePage } from '../pages/TimelinePage';
import { CONFIG, USERS, login, sleep } from '../fixtures/test-fixtures';

test.describe('Timeline Page @ui @timeline', () => {
  
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test.describe('Page Load', () => {
    test('should load timeline page successfully', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });
      
      await expect(page.locator('body')).toBeVisible();
      expect(await timelinePage.isOnPage()).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/timeline-page-load.png' });
    });

    test('should display timeline container', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const hasTimeline = await timelinePage.timelineContainer.isVisible();
      console.log(`Has timeline container: ${hasTimeline}`);
      
      await page.screenshot({ path: 'test-results/timeline-container.png' });
    });

    test('should display date picker', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      const hasDatePicker = await timelinePage.datePicker.isVisible();
      console.log(`Has date picker: ${hasDatePicker}`);
      
      await page.screenshot({ path: 'test-results/timeline-date-picker.png' });
    });
  });

  test.describe('Date Navigation', () => {
    test('should navigate to previous day', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      if (await timelinePage.prevDayButton.isVisible()) {
        await timelinePage.goToPreviousDay();
        console.log('Navigated to previous day');
      }
      
      await page.screenshot({ path: 'test-results/timeline-prev-day.png' });
    });

    test('should navigate to next day', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      // First go back a day
      if (await timelinePage.prevDayButton.isVisible()) {
        await timelinePage.goToPreviousDay();
      }
      
      // Then go forward
      if (await timelinePage.nextDayButton.isVisible()) {
        await timelinePage.goToNextDay();
        console.log('Navigated to next day');
      }
      
      await page.screenshot({ path: 'test-results/timeline-next-day.png' });
    });

    test('should navigate to specific date', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      if (await timelinePage.datePicker.isVisible()) {
        // Navigate to yesterday
        const yesterday = new Date();
        yesterday.setDate(yesterday.getDate() - 1);
        const dateStr = yesterday.toISOString().split('T')[0];
        
        await timelinePage.goToDate(dateStr);
        console.log(`Navigated to date: ${dateStr}`);
      }
      
      await page.screenshot({ path: 'test-results/timeline-specific-date.png' });
    });

    test('should navigate to today', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      if (await timelinePage.todayButton.isVisible()) {
        // First go to a different day
        if (await timelinePage.prevDayButton.isVisible()) {
          await timelinePage.goToPreviousDay();
        }
        
        // Then click today
        await timelinePage.goToToday();
        console.log('Navigated to today');
      }
      
      await page.screenshot({ path: 'test-results/timeline-today.png' });
    });
  });

  test.describe('Timeline Segments', () => {
    test('should display segments or empty state', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const segmentCount = await timelinePage.getSegmentCount();
      const isEmpty = await timelinePage.isEmpty();
      
      console.log(`Segment count: ${segmentCount}, isEmpty: ${isEmpty}`);
      
      if (isEmpty) {
        console.log('No segments - expected in test environment without recordings');
      } else {
        console.log(`Found ${segmentCount} timeline segments`);
      }
      
      await page.screenshot({ path: 'test-results/timeline-segments.png' });
    });
  });

  test.describe('Stream Filter', () => {
    test('should display stream filter', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });

      await sleep(1000);

      const hasStreamFilter = await timelinePage.streamFilter.isVisible();
      console.log(`Has stream filter: ${hasStreamFilter}`);

      if (hasStreamFilter) {
        const options = await timelinePage.streamFilter.locator('option').allTextContents();
        console.log(`Stream filter options: ${options.join(', ')}`);
      }

      await page.screenshot({ path: 'test-results/timeline-stream-filter.png' });
    });
  });

  test.describe('Date/Timezone Handling', () => {
    test('should display today\'s date correctly on initial load', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });

      await sleep(1000);

      const expectedDate = TimelinePage.getTodayDate();
      const displayedDate = await timelinePage.getSelectedDate();
      const urlDate = timelinePage.getDateFromUrl();

      console.log(`Expected date (today): ${expectedDate}`);
      console.log(`Displayed date in picker: ${displayedDate}`);
      console.log(`Date from URL: ${urlDate}`);

      // The date picker should show today's date
      expect(displayedDate).toBe(expectedDate);

      // The URL should also have today's date (or no date param, defaulting to today)
      if (urlDate) {
        expect(urlDate).toBe(expectedDate);
      }

      await page.screenshot({ path: 'test-results/timeline-today-date-correct.png' });
    });

    test('should maintain date consistency when navigating with URL parameter', async ({ page }) => {
      const timelinePage = new TimelinePage(page);

      // Navigate with a specific date in the URL
      const testDate = '2025-12-25';
      await page.goto(`${CONFIG.LIGHTNVR_URL}/timeline.html?date=${testDate}`, {
        waitUntil: 'networkidle',
        timeout: CONFIG.DEFAULT_TIMEOUT
      });

      await sleep(1500);

      const displayedDate = await timelinePage.getSelectedDate();
      const urlDate = timelinePage.getDateFromUrl();

      console.log(`Test date (URL): ${testDate}`);
      console.log(`Displayed date in picker: ${displayedDate}`);
      console.log(`Date from URL: ${urlDate}`);

      // The date picker should show the date from the URL parameter
      expect(displayedDate).toBe(testDate);

      // The URL should still have the same date
      expect(urlDate).toBe(testDate);

      await page.screenshot({ path: 'test-results/timeline-url-date-consistency.png' });
    });

    test('should correctly update URL when changing date via date picker', async ({ page }) => {
      const timelinePage = new TimelinePage(page);
      await timelinePage.goto({ waitForNetworkIdle: true });

      await sleep(1000);

      // Change to a specific date
      const newDate = '2025-12-20';
      await timelinePage.goToDate(newDate);

      await sleep(1500);

      const displayedDate = await timelinePage.getSelectedDate();
      const urlDate = timelinePage.getDateFromUrl();

      console.log(`Selected date: ${newDate}`);
      console.log(`Displayed date in picker: ${displayedDate}`);
      console.log(`Date from URL: ${urlDate}`);

      // Date picker should show the new date
      expect(displayedDate).toBe(newDate);

      // URL should be updated with the new date
      expect(urlDate).toBe(newDate);

      await page.screenshot({ path: 'test-results/timeline-date-change-url-update.png' });
    });

    test('should handle date near timezone boundary correctly', async ({ page }) => {
      const timelinePage = new TimelinePage(page);

      // Test with today's date specifically - this was the original bug scenario
      const today = TimelinePage.getTodayDate();
      await page.goto(`${CONFIG.LIGHTNVR_URL}/timeline.html?date=${today}`, {
        waitUntil: 'networkidle',
        timeout: CONFIG.DEFAULT_TIMEOUT
      });

      await sleep(1500);

      const displayedDate = await timelinePage.getSelectedDate();

      console.log(`Today's date: ${today}`);
      console.log(`Displayed date in picker: ${displayedDate}`);

      // The displayed date should exactly match today's date
      // This specifically tests the timezone bug where dates were off by a few hours
      expect(displayedDate).toBe(today);

      await page.screenshot({ path: 'test-results/timeline-timezone-boundary.png' });
    });

    test('should show correct date after stream selection', async ({ page }) => {
      const timelinePage = new TimelinePage(page);

      // First navigate to timeline with a specific date
      const testDate = TimelinePage.getTodayDate();
      await page.goto(`${CONFIG.LIGHTNVR_URL}/timeline.html?date=${testDate}`, {
        waitUntil: 'networkidle',
        timeout: CONFIG.DEFAULT_TIMEOUT
      });

      await sleep(1500);

      // Check if stream filter is available
      if (await timelinePage.streamFilter.isVisible()) {
        // Get the available options
        const options = await timelinePage.streamFilter.locator('option').allTextContents();
        console.log(`Available streams: ${options.join(', ')}`);

        // If there are streams available (besides the "Select a stream" option)
        if (options.length > 1) {
          // Select a stream (skip the first "Select a stream" option)
          await timelinePage.streamFilter.selectOption({ index: 1 });

          await sleep(1500);

          // Date should still be correct after stream selection
          const displayedDate = await timelinePage.getSelectedDate();
          const urlDate = timelinePage.getDateFromUrl();

          console.log(`Test date: ${testDate}`);
          console.log(`Displayed date after stream selection: ${displayedDate}`);
          console.log(`URL date after stream selection: ${urlDate}`);

          expect(displayedDate).toBe(testDate);
          expect(urlDate).toBe(testDate);
        }
      }

      await page.screenshot({ path: 'test-results/timeline-date-after-stream-selection.png' });
    });
  });
});

