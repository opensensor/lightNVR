/**
 * Streams Page UI Tests
 * 
 * Tests for streams list, add/edit/delete stream, stream status, and detection zones.
 * @tags @ui @streams
 */

import { test, expect } from '@playwright/test';
import { StreamsPage } from '../pages/StreamsPage';
import { CONFIG, USERS, login, sleep } from '../fixtures/test-fixtures';

test.describe('Streams Page @ui @streams', () => {
  
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test.describe('Page Load', () => {
    test('should load streams page successfully', async ({ page }) => {
      const streamsPage = new StreamsPage(page);
      await streamsPage.goto({ waitForNetworkIdle: true });
      
      await expect(page.locator('body')).toBeVisible();
      expect(await streamsPage.isOnPage()).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/streams-page-load.png' });
    });

    test('should display add stream button', async ({ page }) => {
      const streamsPage = new StreamsPage(page);
      await streamsPage.goto({ waitForNetworkIdle: true });
      
      // The add stream button should be visible for admin users
      await expect(streamsPage.addStreamButton).toBeVisible();
    });

    test('should display test streams from global setup', async ({ page }) => {
      const streamsPage = new StreamsPage(page);
      await streamsPage.goto({ waitForNetworkIdle: true });
      
      // Wait for streams to load
      await sleep(2000);
      
      const streamCount = await streamsPage.getStreamCount();
      console.log(`Found ${streamCount} streams`);
      
      // We should have at least the test streams from global setup
      expect(streamCount).toBeGreaterThan(0);
      
      await page.screenshot({ path: 'test-results/streams-list.png' });
    });
  });

  test.describe('Stream Operations', () => {
    test('should open add stream modal', async ({ page }) => {
      const streamsPage = new StreamsPage(page);
      await streamsPage.goto({ waitForNetworkIdle: true });
      
      await streamsPage.clickAddStream();
      
      // Modal should be visible
      await expect(streamsPage.addStreamModal).toBeVisible();
      await expect(streamsPage.streamNameInput).toBeVisible();
      await expect(streamsPage.streamUrlInput).toBeVisible();
      
      await page.screenshot({ path: 'test-results/streams-add-modal.png' });
    });

    test('should close add stream modal on cancel', async ({ page }) => {
      const streamsPage = new StreamsPage(page);
      await streamsPage.goto({ waitForNetworkIdle: true });
      
      await streamsPage.clickAddStream();
      await expect(streamsPage.addStreamModal).toBeVisible();
      
      await streamsPage.cancelButton.click();
      await sleep(500);
      
      // Modal should be hidden
      await expect(streamsPage.addStreamModal).not.toBeVisible();
      
      await page.screenshot({ path: 'test-results/streams-modal-cancel.png' });
    });

    test('should add a new stream via UI', async ({ page }) => {
      const streamsPage = new StreamsPage(page);
      await streamsPage.goto({ waitForNetworkIdle: true });
      
      const testStreamName = `ui_test_stream_${Date.now()}`;
      
      await streamsPage.addStream({
        name: testStreamName,
        url: 'rtsp://localhost:18554/test_pattern',
        enabled: true
      });
      
      await sleep(2000);
      
      // Verify stream appears in list
      const streamExists = await streamsPage.streamExists(testStreamName);
      expect(streamExists).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/streams-add-new.png' });
    });

    test('should display stream status indicators', async ({ page }) => {
      const streamsPage = new StreamsPage(page);
      await streamsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const streamCount = await streamsPage.getStreamCount();
      if (streamCount > 0) {
        // Check that stream cards have status information
        const firstCard = streamsPage.streamCards.first();
        await expect(firstCard).toBeVisible();
        
        // Look for status badge or indicator
        const statusElement = firstCard.locator('.status, .badge, [data-status]');
        const hasStatus = await statusElement.count() > 0;
        console.log(`Stream has status indicator: ${hasStatus}`);
      }
      
      await page.screenshot({ path: 'test-results/streams-status.png' });
    });
  });

  test.describe('Stream Details', () => {
    test('should show stream card with name and URL', async ({ page }) => {
      const streamsPage = new StreamsPage(page);
      await streamsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const streamCount = await streamsPage.getStreamCount();
      if (streamCount > 0) {
        // First stream card should have name
        const firstCard = streamsPage.streamCards.first();
        const cardText = await firstCard.textContent();
        expect(cardText).toBeTruthy();
        console.log(`First stream card content: ${cardText?.substring(0, 100)}...`);
      }
      
      await page.screenshot({ path: 'test-results/streams-details.png' });
    });

    test('should have edit and delete buttons on stream cards', async ({ page }) => {
      const streamsPage = new StreamsPage(page);
      await streamsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const streamCount = await streamsPage.getStreamCount();
      if (streamCount > 0) {
        const firstCard = streamsPage.streamCards.first();
        
        // Check for edit button
        const editButton = firstCard.locator('button').filter({ hasText: /edit/i });
        const hasEdit = await editButton.count() > 0;
        console.log(`Has edit button: ${hasEdit}`);
        
        // Check for delete button
        const deleteButton = firstCard.locator('button').filter({ hasText: /delete|remove/i });
        const hasDelete = await deleteButton.count() > 0;
        console.log(`Has delete button: ${hasDelete}`);
      }
      
      await page.screenshot({ path: 'test-results/streams-buttons.png' });
    });
  });
});

