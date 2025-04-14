/**
 * Test utilities for E2E tests
 */
const { Builder, By, until } = require('selenium-webdriver');
const chrome = require('selenium-webdriver/chrome');
const firefox = require('selenium-webdriver/firefox');

/**
 * Create a WebDriver instance for the specified browser
 * @param {string} browserName - The browser to use ('chrome' or 'firefox')
 * @param {boolean} headless - Whether to run in headless mode
 * @returns {WebDriver} The WebDriver instance
 */
async function createDriver(browserName = 'chrome', headless = false) {
  let driver;
  
  if (browserName.toLowerCase() === 'chrome') {
    const options = new chrome.Options();
    
    if (headless) {
      options.headless();
    }
    
    driver = await new Builder()
      .forBrowser('chrome')
      .setChromeOptions(options)
      .build();
  } else if (browserName.toLowerCase() === 'firefox') {
    const options = new firefox.Options();
    
    if (headless) {
      options.headless();
    }
    
    driver = await new Builder()
      .forBrowser('firefox')
      .setFirefoxOptions(options)
      .build();
  } else {
    throw new Error(`Unsupported browser: ${browserName}`);
  }
  
  // Set implicit wait time
  await driver.manage().setTimeouts({ implicit: 5000 });
  
  return driver;
}

/**
 * Take a screenshot and save it to the specified path
 * @param {WebDriver} driver - The WebDriver instance
 * @param {string} path - The path to save the screenshot to
 */
async function takeScreenshot(driver, path) {
  const fs = require('fs');
  const screenshot = await driver.takeScreenshot();
  
  // Create the directory if it doesn't exist
  const dir = path.substring(0, path.lastIndexOf('/'));
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
  
  // Save the screenshot
  fs.writeFileSync(path, screenshot, 'base64');
  console.log(`Screenshot saved to ${path}`);
}

/**
 * Wait for a specified amount of time
 * @param {number} ms - The number of milliseconds to wait
 */
function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Login to the application
 * @param {WebDriver} driver - The WebDriver instance
 * @param {string} username - The username to use (default: 'admin')
 * @param {string} password - The password to use (default: 'admin')
 * @returns {Promise<boolean>} True if login was successful, false otherwise
 */
async function login(driver, username = 'admin', password = 'admin') {
  try {
    // Navigate to the login page
    await driver.get('http://localhost:8080/login.html');
    
    // Wait for the login form to load
    await driver.wait(until.elementLocated(By.css('#username')), 10000);
    
    // Enter username
    const usernameInput = await driver.findElement(By.css('#username'));
    await usernameInput.clear();
    await usernameInput.sendKeys(username);
    
    // Enter password
    const passwordInput = await driver.findElement(By.css('#password'));
    await passwordInput.clear();
    await passwordInput.sendKeys(password);
    
    // Click login button
    const loginButton = await driver.findElement(By.css('form#login-form button[type="submit"]'));
    await loginButton.click();
    
    // Wait for redirection after successful login
    await driver.wait(async () => {
      const currentUrl = await driver.getCurrentUrl();
      return !currentUrl.includes('login.html');
    }, 5000, 'Timed out waiting for successful login redirection');
    
    return true;
  } catch (error) {
    console.error('Login failed:', error);
    return false;
  }
}

module.exports = {
  createDriver,
  takeScreenshot,
  sleep,
  login
};
