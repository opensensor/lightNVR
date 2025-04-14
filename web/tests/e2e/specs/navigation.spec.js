/**
 * Navigation E2E tests
 */
const { By, until } = require('selenium-webdriver');
const { createDriver, takeScreenshot, login } = require('../utils/test-utils');
const NavigationMenu = require('../pages/NavigationMenu');

describe('Navigation', () => {
  let driver;
  let navMenu;
  
  beforeAll(async () => {
    // Create the WebDriver
    driver = await createDriver('chrome', false);
    
    // Create the navigation menu page object
    navMenu = new NavigationMenu(driver);
    
    // Login before running tests
    const loginSuccess = await login(driver);
    if (!loginSuccess) {
      throw new Error('Login failed. Tests cannot proceed.');
    }
  });
  
  afterAll(async () => {
    // Quit the driver
    if (driver) {
      await driver.quit();
    }
  });
  
  beforeEach(async () => {
    // Navigate to the index page before each test
    await driver.get('http://localhost:8080/index.html');
    
    // Wait for the page to load
    await driver.wait(until.elementLocated(By.id('main-content')), 10000);
  });
  
  test('should navigate to streams page from index page', async () => {
    // Take a screenshot before clicking
    await takeScreenshot(driver, 'screenshots/before-navigation-to-streams.png');
    
    // Navigate to streams page using the navigation menu
    await navMenu.navigateToStreams();
    
    // Wait for the streams page to load
    await navMenu.waitForNavigation('#streams-page');
    
    // Take a screenshot after navigation
    await takeScreenshot(driver, 'screenshots/after-navigation-to-streams.png');
    
    // Verify we're on the streams page
    const currentUrl = await navMenu.getCurrentUrl();
    expect(currentUrl).toContain('streams.html');
    
    // Verify the page title is correct
    const pageTitle = await driver.findElement(By.css('.page-header h2'));
    const titleText = await pageTitle.getText();
    expect(titleText).toBe('Streams');
  });
  
  test('should navigate to recordings page from index page', async () => {
    // Navigate to recordings page using the navigation menu
    await navMenu.navigateToRecordings();
    
    // Wait for the recordings page to load
    await navMenu.waitForNavigation('h2');
    
    // Take a screenshot after navigation
    await takeScreenshot(driver, 'screenshots/after-navigation-to-recordings.png');
    
    // Verify we're on the recordings page
    const currentUrl = await navMenu.getCurrentUrl();
    expect(currentUrl).toContain('recordings.html');
  });
  
  test('should navigate to timeline page from recordings page', async () => {
    // Navigate to timeline page via recordings page using the navigation menu
    await navMenu.navigateToTimeline();
    
    // Wait for the timeline page to load
    await navMenu.waitForNavigation('.timeline-page');
    
    // Take a screenshot after navigation
    await takeScreenshot(driver, 'screenshots/after-navigation-to-timeline.png');
    
    // Verify we're on the timeline page
    const currentUrl = await navMenu.getCurrentUrl();
    expect(currentUrl).toContain('timeline.html');
    
    // Verify the page title is correct
    // Use a more specific selector to find the Timeline Playback heading
    const pageTitle = await driver.findElement(By.css('.timeline-page h1'));
    const titleText = await pageTitle.getText();
    expect(titleText).toBe('Timeline Playback');
  });
  
  test('should navigate to settings page from index page', async () => {
    // Navigate to settings page using the navigation menu
    await navMenu.navigateToSettings();
    
    // Wait for the settings page to load
    await navMenu.waitForNavigation('h2');
    
    // Take a screenshot after navigation
    await takeScreenshot(driver, 'screenshots/after-navigation-to-settings.png');
    
    // Verify we're on the settings page
    const currentUrl = await navMenu.getCurrentUrl();
    expect(currentUrl).toContain('settings.html');
  });
  
  test('should navigate back to index page from streams page', async () => {
    // First navigate to streams page
    await navMenu.navigateToStreams();
    
    // Wait for the streams page to load
    await navMenu.waitForNavigation('#streams-page');
    
    // Navigate back to home page
    await navMenu.navigateToHome();
    
    // Wait for the index page to load
    await navMenu.waitForNavigation('#main-content');
    
    // Take a screenshot after navigation
    await takeScreenshot(driver, 'screenshots/after-navigation-back-to-index.png');
    
    // Verify we're back on the index page
    const currentUrl = await navMenu.getCurrentUrl();
    expect(currentUrl).toContain('index.html');
  });
});
