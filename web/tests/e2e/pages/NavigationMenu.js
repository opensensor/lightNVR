/**
 * Page Object for the Navigation Menu
 */
class NavigationMenu {
  constructor(driver) {
    this.driver = driver;
  }

  // Selectors
  get homeLink() { return 'a[href="index.html"]'; }
  get streamsLink() { return 'a[href="streams.html"]'; }
  get recordingsLink() { return 'a[href="recordings.html"]'; }
  get timelineLink() { return 'a[href="timeline.html"]'; }
  get settingsLink() { return 'a[href="settings.html"]'; }
  get usersLink() { return 'a[href="users.html"]'; }
  get systemLink() { return 'a[href="system.html"]'; }
  get logoutLink() { return 'a.logout-link'; }

  /**
   * Navigate to the home page
   */
  async navigateToHome() {
    const { By } = require('selenium-webdriver');
    const link = await this.driver.findElement(By.css(this.homeLink));
    await link.click();
  }

  /**
   * Navigate to the streams page
   */
  async navigateToStreams() {
    const { By } = require('selenium-webdriver');
    const link = await this.driver.findElement(By.css(this.streamsLink));
    await link.click();
  }

  /**
   * Navigate to the recordings page
   */
  async navigateToRecordings() {
    const { By } = require('selenium-webdriver');
    const link = await this.driver.findElement(By.css(this.recordingsLink));
    await link.click();
  }

  /**
   * Navigate to the timeline page (via recordings page)
   */
  async navigateToTimeline() {
    const { By, until } = require('selenium-webdriver');
    
    // First navigate to the recordings page
    await this.navigateToRecordings();
    
    // Wait for the recordings page to load
    await this.driver.wait(until.elementLocated(By.css('h2.text-xl')), 10000);
    
    // Then click on the Timeline View link
    const timelineLink = await this.driver.findElement(By.css('a[href="timeline.html"]'));
    await timelineLink.click();
  }

  /**
   * Navigate to the settings page
   */
  async navigateToSettings() {
    const { By } = require('selenium-webdriver');
    const link = await this.driver.findElement(By.css(this.settingsLink));
    await link.click();
  }

  /**
   * Navigate to the users page
   */
  async navigateToUsers() {
    const { By } = require('selenium-webdriver');
    const link = await this.driver.findElement(By.css(this.usersLink));
    await link.click();
  }

  /**
   * Navigate to the system page
   */
  async navigateToSystem() {
    const { By } = require('selenium-webdriver');
    const link = await this.driver.findElement(By.css(this.systemLink));
    await link.click();
  }

  /**
   * Logout
   */
  async logout() {
    const { By } = require('selenium-webdriver');
    try {
      const link = await this.driver.findElement(By.css(this.logoutLink));
      await link.click();
      return true;
    } catch (error) {
      console.log('Logout link not found');
      return false;
    }
  }

  /**
   * Wait for navigation to complete
   * @param {string} pageIdentifier - CSS selector to identify the page has loaded
   * @param {number} timeout - Timeout in milliseconds
   */
  async waitForNavigation(pageIdentifier, timeout = 10000) {
    const { By, until } = require('selenium-webdriver');
    await this.driver.wait(until.elementLocated(By.css(pageIdentifier)), timeout);
  }

  /**
   * Get the current page URL
   * @returns {string} The current URL
   */
  async getCurrentUrl() {
    return await this.driver.getCurrentUrl();
  }

  /**
   * Check if a specific navigation link exists
   * @param {string} linkSelector - CSS selector for the link
   * @returns {boolean} True if the link exists, false otherwise
   */
  async hasLink(linkSelector) {
    const { By } = require('selenium-webdriver');
    try {
      await this.driver.findElement(By.css(linkSelector));
      return true;
    } catch (error) {
      return false;
    }
  }
}

module.exports = NavigationMenu;
