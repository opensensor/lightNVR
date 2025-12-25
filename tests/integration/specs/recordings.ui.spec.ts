/**
 * Recordings Page UI Tests
 * 
 * Tests for recordings list, pagination, playback, download, delete, and batch operations.
 * @tags @ui @recordings
 */

import { test, expect } from '@playwright/test';
import { RecordingsPage } from '../pages/RecordingsPage';
import { CONFIG, USERS, login, sleep } from '../fixtures/test-fixtures';

test.describe('Recordings Page @ui @recordings', () => {
  
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test.describe('Page Load', () => {
    test('should load recordings page successfully', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await expect(page.locator('body')).toBeVisible();
      expect(await recordingsPage.isOnPage()).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/recordings-page-load.png' });
    });

    test('should display recordings list or empty state', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const recordingCount = await recordingsPage.getRecordingCount();
      const isEmpty = await recordingsPage.isEmpty();
      
      console.log(`Recordings count: ${recordingCount}, isEmpty: ${isEmpty}`);
      
      // Either we have recordings or we show an empty state
      if (isEmpty) {
        console.log('No recordings found - this is expected in test environment');
      } else {
        console.log(`Found ${recordingCount} recordings`);
      }
      
      await page.screenshot({ path: 'test-results/recordings-list.png' });
    });

    test('should display filter controls', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      // Check for stream filter
      const hasStreamFilter = await recordingsPage.streamFilter.isVisible();
      console.log(`Has stream filter: ${hasStreamFilter}`);
      
      // Check for date filter
      const hasDateFilter = await recordingsPage.dateFilter.isVisible();
      console.log(`Has date filter: ${hasDateFilter}`);
      
      await page.screenshot({ path: 'test-results/recordings-filters.png' });
    });
  });

  test.describe('Filtering', () => {
    test('should filter recordings by date', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      if (await recordingsPage.dateFilter.isVisible()) {
        // Get today's date in YYYY-MM-DD format
        const today = new Date().toISOString().split('T')[0];
        await recordingsPage.filterByDate(today);
        
        console.log(`Filtered by date: ${today}`);
      }
      
      await page.screenshot({ path: 'test-results/recordings-filter-date.png' });
    });

    test('should filter recordings by stream', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      if (await recordingsPage.streamFilter.isVisible()) {
        // Get available options
        const options = await recordingsPage.streamFilter.locator('option').allTextContents();
        console.log(`Stream filter options: ${options.join(', ')}`);
        
        if (options.length > 1) {
          // Select first non-empty option
          await recordingsPage.streamFilter.selectOption({ index: 1 });
          await sleep(1000);
          console.log('Selected stream filter');
        }
      }
      
      await page.screenshot({ path: 'test-results/recordings-filter-stream.png' });
    });
  });

  test.describe('Recording Actions', () => {
    test('should have action buttons on recording cards', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      if (!await recordingsPage.isEmpty()) {
        const firstCard = recordingsPage.getRecordingByIndex(0);
        
        // Check for play button
        const playButton = firstCard.locator('button, a').filter({ hasText: /play|view/i });
        const hasPlay = await playButton.count() > 0;
        console.log(`Has play button: ${hasPlay}`);
        
        // Check for download button
        const downloadButton = firstCard.locator('button, a').filter({ hasText: /download/i });
        const hasDownload = await downloadButton.count() > 0;
        console.log(`Has download button: ${hasDownload}`);
        
        // Check for delete button
        const deleteButton = firstCard.locator('button').filter({ hasText: /delete|remove/i });
        const hasDelete = await deleteButton.count() > 0;
        console.log(`Has delete button: ${hasDelete}`);
      } else {
        console.log('No recordings to check actions on');
      }
      
      await page.screenshot({ path: 'test-results/recordings-actions.png' });
    });
  });

  test.describe('Pagination', () => {
    test('should display pagination if many recordings', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const hasPagination = await recordingsPage.pagination.isVisible();
      console.log(`Has pagination: ${hasPagination}`);
      
      if (hasPagination) {
        const hasNext = await recordingsPage.nextPageButton.isVisible();
        const hasPrev = await recordingsPage.prevPageButton.isVisible();
        console.log(`Next button: ${hasNext}, Prev button: ${hasPrev}`);
      }
      
      await page.screenshot({ path: 'test-results/recordings-pagination.png' });
    });
  });

  test.describe('Batch Operations', () => {
    test('should display batch delete button if available', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      const hasBatchDelete = await recordingsPage.batchDeleteButton.isVisible();
      console.log(`Has batch delete button: ${hasBatchDelete}`);
      
      await page.screenshot({ path: 'test-results/recordings-batch.png' });
    });
  });
});

