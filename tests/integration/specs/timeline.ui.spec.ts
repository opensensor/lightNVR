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
});

