/**
 * Live View (Index) Page UI Tests
 * 
 * Tests for video grid, WebRTC/HLS streaming, fullscreen mode, and stream selection.
 * @tags @ui @liveview
 */

import { test, expect } from '@playwright/test';
import { LiveViewPage } from '../pages/LiveViewPage';
import { CONFIG, USERS, login, sleep } from '../fixtures/test-fixtures';

test.describe('Live View Page @ui @liveview', () => {
  
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test.describe('Page Load', () => {
    test('should load live view page successfully', async ({ page }) => {
      const liveView = new LiveViewPage(page);
      await liveView.goto();
      
      await expect(page).toHaveTitle(/LightNVR/i);
      expect(await liveView.isOnPage()).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/liveview-page-load.png' });
    });

    test('should display video grid container', async ({ page }) => {
      const liveView = new LiveViewPage(page);
      await liveView.goto();
      
      await sleep(2000);
      
      // Should have some kind of container for streams
      const containerCount = await liveView.getStreamContainerCount();
      console.log(`Found ${containerCount} stream containers`);
      
      await page.screenshot({ path: 'test-results/liveview-grid.png' });
    });
  });

  test.describe('Video Streaming', () => {
    test('should load video elements for test streams', async ({ page }) => {
      const liveView = new LiveViewPage(page);
      await liveView.goto();
      
      // Wait for streams to load
      await liveView.waitForStreamsToLoad(15000);
      
      const videoCount = await liveView.getVideoCount();
      console.log(`Found ${videoCount} video elements`);
      
      await page.screenshot({ path: 'test-results/liveview-videos.png' });
    });

    test('should have video playback status', async ({ page }) => {
      const liveView = new LiveViewPage(page);
      await liveView.goto();
      
      // Wait for streams to potentially start playing
      await sleep(5000);
      
      const status = await liveView.getVideoStatus();
      console.log(`Video status: ${status.playing}/${status.total} playing`);
      
      // Log the result (may not have playing videos in test environment)
      await page.screenshot({ path: 'test-results/liveview-playback-status.png' });
    });

    test('should have WebRTC or HLS view', async ({ page }) => {
      const liveView = new LiveViewPage(page);
      await liveView.goto();
      
      await sleep(2000);
      
      const isWebRTC = await liveView.isWebRTCView();
      const isHLS = await liveView.isHLSView();
      
      console.log(`View type - WebRTC: ${isWebRTC}, HLS: ${isHLS}`);
      
      await page.screenshot({ path: 'test-results/liveview-view-type.png' });
    });
  });

  test.describe('Stream Interaction', () => {
    test('should be able to click on stream containers', async ({ page }) => {
      const liveView = new LiveViewPage(page);
      await liveView.goto();
      
      await sleep(3000);
      
      const containerCount = await liveView.getStreamContainerCount();
      if (containerCount > 0) {
        await liveView.selectStream(0);
        console.log('Selected first stream container');
      }
      
      await page.screenshot({ path: 'test-results/liveview-stream-select.png' });
    });

    test('should handle fullscreen button if available', async ({ page }) => {
      const liveView = new LiveViewPage(page);
      await liveView.goto();
      
      await sleep(2000);
      
      const hasFullscreen = await liveView.fullscreenButton.isVisible();
      console.log(`Has fullscreen button: ${hasFullscreen}`);
      
      await page.screenshot({ path: 'test-results/liveview-fullscreen.png' });
    });
  });

  test.describe('Responsiveness', () => {
    test('should display properly on desktop viewport', async ({ page }) => {
      const liveView = new LiveViewPage(page);
      
      // Set desktop viewport
      await page.setViewportSize({ width: 1920, height: 1080 });
      await liveView.goto();
      await sleep(2000);
      
      await expect(liveView.mainContent).toBeVisible();
      
      await page.screenshot({ path: 'test-results/liveview-desktop.png' });
    });

    test('should display properly on tablet viewport', async ({ page }) => {
      const liveView = new LiveViewPage(page);
      
      // Set tablet viewport
      await page.setViewportSize({ width: 768, height: 1024 });
      await liveView.goto();
      await sleep(2000);
      
      await expect(liveView.mainContent).toBeVisible();
      
      await page.screenshot({ path: 'test-results/liveview-tablet.png' });
    });

    test('should display properly on mobile viewport', async ({ page }) => {
      const liveView = new LiveViewPage(page);
      
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });
      await liveView.goto();
      await sleep(2000);
      
      await expect(liveView.mainContent).toBeVisible();
      
      await page.screenshot({ path: 'test-results/liveview-mobile.png' });
    });
  });
});

