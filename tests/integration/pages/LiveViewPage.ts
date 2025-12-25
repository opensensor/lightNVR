/**
 * Live View (Index) Page Object Model
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from './BasePage';
import { CONFIG, sleep } from '../fixtures/test-fixtures';

export class LiveViewPage extends BasePage {
  protected path = '/index.html';

  // Locators
  get videoGrid(): Locator {
    return this.page.locator('.video-grid, .stream-grid, .live-view, [data-testid="video-grid"]');
  }

  get videoElements(): Locator {
    return this.page.locator('video');
  }

  get canvasElements(): Locator {
    return this.page.locator('canvas');
  }

  get streamContainers(): Locator {
    return this.page.locator('.stream-container, .video-container, [data-testid="stream-container"]');
  }

  get fullscreenButton(): Locator {
    return this.page.locator('button').filter({ hasText: /fullscreen/i }).first();
  }

  get layoutSelector(): Locator {
    return this.page.locator('select, .layout-selector, [data-testid="layout-selector"]').first();
  }

  get loadingIndicator(): Locator {
    return this.page.locator('.loading, .spinner, [data-testid="loading"]').first();
  }

  /**
   * Get count of video elements
   */
  async getVideoCount(): Promise<number> {
    return await this.videoElements.count();
  }

  /**
   * Get count of stream containers
   */
  async getStreamContainerCount(): Promise<number> {
    return await this.streamContainers.count();
  }

  /**
   * Wait for streams to load
   */
  async waitForStreamsToLoad(timeout: number = 10000): Promise<void> {
    const startTime = Date.now();
    while (Date.now() - startTime < timeout) {
      const videoCount = await this.videoElements.count();
      const canvasCount = await this.canvasElements.count();
      if (videoCount > 0 || canvasCount > 0) {
        return;
      }
      await sleep(500);
    }
  }

  /**
   * Check if any video is playing
   */
  async isAnyVideoPlaying(): Promise<boolean> {
    return await this.page.evaluate(() => {
      const videos = document.querySelectorAll('video');
      for (const video of videos) {
        if (video.videoWidth > 0 && video.videoHeight > 0 && video.readyState >= 2) {
          return true;
        }
      }
      return false;
    });
  }

  /**
   * Get video playback status for all videos
   */
  async getVideoStatus(): Promise<{ total: number; playing: number }> {
    return await this.page.evaluate(() => {
      const videos = document.querySelectorAll('video');
      let playingCount = 0;
      videos.forEach(v => {
        if (v.videoWidth > 0 && v.videoHeight > 0 && v.readyState >= 2) {
          playingCount++;
        }
      });
      return { total: videos.length, playing: playingCount };
    });
  }

  /**
   * Click on a stream container to select it
   */
  async selectStream(index: number): Promise<void> {
    const container = this.streamContainers.nth(index);
    await container.click();
    await sleep(500);
  }

  /**
   * Toggle fullscreen mode
   */
  async toggleFullscreen(): Promise<void> {
    if (await this.fullscreenButton.isVisible()) {
      await this.fullscreenButton.click();
      await sleep(500);
    }
  }

  /**
   * Check if page is showing WebRTC view
   */
  async isWebRTCView(): Promise<boolean> {
    return await this.page.locator('[data-testid="webrtc-view"], .webrtc-view').isVisible();
  }

  /**
   * Check if page is showing HLS view
   */
  async isHLSView(): Promise<boolean> {
    return await this.page.locator('[data-testid="hls-view"], .hls-view').isVisible();
  }
}

