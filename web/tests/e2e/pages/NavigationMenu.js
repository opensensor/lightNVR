/**
 * Page Object for the Navigation Menu
 */
const { By, until } = require('selenium-webdriver');

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
    return await this.navigateWithRetry(this.homeLink, '#main-content');
  }

  /**
   * Navigate to the streams page with retry
   */
  async navigateToStreams() {
    return await this.navigateWithRetry(this.streamsLink, '#streams-page');
  }

  /**
   * Navigate to the recordings page
   */
  async navigateToRecordings() {
    return await this.navigateWithRetry(this.recordingsLink, 'h2.text-xl');
  }

  /**
   * Navigate to the timeline page (via recordings page)
   */
  async navigateToTimeline() {
    // First navigate to the recordings page
    await this.navigateToRecordings();
    
    // Then navigate to timeline with retry
    return await this.navigateWithRetry('a[href="timeline.html"]', '.timeline-page');
  }

  /**
   * Navigate to the settings page
   */
  async navigateToSettings() {
    return await this.navigateWithRetry(this.settingsLink, 'h2');
  }

  /**
   * Navigate to the users page
   */
  async navigateToUsers() {
    return await this.navigateWithRetry(this.usersLink, 'h2');
  }

  /**
   * Navigate to the system page
   */
  async navigateToSystem() {
    return await this.navigateWithRetry(this.systemLink, 'h2');
  }

  /**
   * Logout
   */
  async logout() {
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
    try {
      await this.driver.findElement(By.css(linkSelector));
      return true;
    } catch (error) {
      return false;
    }
  }
  
  /**
   * Navigate to a page with retry mechanism
   * @param {string} linkSelector - CSS selector for the navigation link
   * @param {string} targetPageSelector - CSS selector to identify the target page has loaded
   * @param {number} maxRetries - Maximum number of retry attempts
   * @returns {boolean} - True if navigation was successful
   */
  async navigateWithRetry(linkSelector, targetPageSelector, maxRetries = 3) {
    console.log(`Attempting to navigate to ${linkSelector} with retry mechanism`);

    for (let attempt = 1; attempt <= maxRetries; attempt++) {
      console.log(`Navigation attempt ${attempt} of ${maxRetries}`);
      
      try {
        // Find and click the link
        const link = await this.driver.findElement(By.css(linkSelector));
        await link.click();
        console.log(`Clicked on ${linkSelector}`);
        
        // Wait for the page to load with a short timeout
        try {
          await this.driver.wait(until.elementLocated(By.css(targetPageSelector)), 3000);
          console.log(`Successfully navigated to page with selector ${targetPageSelector}`);
          return true;
        } catch (timeoutError) {
          console.log(`Timeout waiting for ${targetPageSelector} on attempt ${attempt}`);
          
          // If this is the last attempt, throw the error
          if (attempt === maxRetries) {
            throw timeoutError;
          }
          
          // Otherwise, wait a moment before retrying
          console.log('Waiting before retry...');
          await this.driver.sleep(1000);
        }
      } catch (error) {
        console.error(`Error during navigation attempt ${attempt}:`, error.message);
        
        // If this is the last attempt, throw the error
        if (attempt === maxRetries) {
          throw error;
        }
        
        // Otherwise, wait a moment before retrying
        console.log('Waiting before retry...');
        await this.driver.sleep(1000);
      }
    }
    
    return false;
  }
}

module.exports = NavigationMenu;
