/**
 * Base Page Object Model for LightNVR pages
 * 
 * Provides common functionality for all page objects.
 */

import { Page, Locator, expect } from '@playwright/test';
import { CONFIG, sleep } from '../fixtures/test-fixtures';

export abstract class BasePage {
  protected page: Page;
  protected abstract path: string;

  constructor(page: Page) {
    this.page = page;
  }

  /**
   * Navigate to this page
   */
  async goto(options?: { waitForNetworkIdle?: boolean }): Promise<void> {
    const waitUntil = options?.waitForNetworkIdle ? 'networkidle' : 'domcontentloaded';
    await this.page.goto(this.path, { waitUntil, timeout: CONFIG.DEFAULT_TIMEOUT });
    await sleep(CONFIG.COMPONENT_RENDER_DELAY);
  }

  /**
   * Get the page URL
   */
  getUrl(): string {
    return this.page.url();
  }

  /**
   * Check if we're on this page
   */
  async isOnPage(): Promise<boolean> {
    return this.page.url().includes(this.path);
  }

  /**
   * Wait for page to load
   */
  async waitForLoad(): Promise<void> {
    await this.page.waitForLoadState('domcontentloaded');
    await sleep(CONFIG.COMPONENT_RENDER_DELAY);
  }

  /**
   * Get the navigation menu
   */
  get navigation(): NavigationComponent {
    return new NavigationComponent(this.page);
  }

  /**
   * Get header component
   */
  get header(): Locator {
    return this.page.locator('header, .header, nav');
  }

  /**
   * Get footer component
   */
  get footer(): Locator {
    return this.page.locator('footer, .footer');
  }

  /**
   * Get main content area
   */
  get mainContent(): Locator {
    return this.page.locator('#main-content, main, .main-content');
  }

  /**
   * Take a screenshot with a meaningful name
   */
  async screenshot(name: string): Promise<void> {
    await this.page.screenshot({ path: `test-results/${name}.png` });
  }
}

/**
 * Navigation Component
 */
export class NavigationComponent {
  private page: Page;

  constructor(page: Page) {
    this.page = page;
  }

  // Navigation links
  get homeLink(): Locator {
    return this.page.locator('a[href="index.html"], a[href="/index.html"]').first();
  }

  get streamsLink(): Locator {
    return this.page.locator('a[href="streams.html"], a[href="/streams.html"]').first();
  }

  get recordingsLink(): Locator {
    return this.page.locator('a[href="recordings.html"], a[href="/recordings.html"]').first();
  }

  get timelineLink(): Locator {
    return this.page.locator('a[href="timeline.html"], a[href="/timeline.html"]').first();
  }

  get settingsLink(): Locator {
    return this.page.locator('a[href="settings.html"], a[href="/settings.html"]').first();
  }

  get systemLink(): Locator {
    return this.page.locator('a[href="system.html"], a[href="/system.html"]').first();
  }

  get usersLink(): Locator {
    return this.page.locator('a[href="users.html"], a[href="/users.html"]').first();
  }

  get logoutLink(): Locator {
    return this.page.locator('a.logout-link, a[href="/logout"], a[href="logout"]').first();
  }

  /**
   * Navigate to a page via the navigation menu
   */
  async navigateTo(link: Locator): Promise<void> {
    await link.click();
    await this.page.waitForLoadState('domcontentloaded');
    await sleep(CONFIG.COMPONENT_RENDER_DELAY);
  }

  /**
   * Get all visible navigation links
   */
  async getVisibleLinks(): Promise<string[]> {
    const links = this.page.locator('nav a, .nav a, .sidebar a, header a, .navigation a');
    const count = await links.count();
    const hrefs: string[] = [];
    for (let i = 0; i < count; i++) {
      const href = await links.nth(i).getAttribute('href');
      if (href) hrefs.push(href);
    }
    return hrefs;
  }
}

