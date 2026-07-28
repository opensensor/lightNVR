/**
 * Test utilities for E2E tests
 */
const { Builder, By, until } = require('selenium-webdriver');
const chrome = require('selenium-webdriver/chrome');
const firefox = require('selenium-webdriver/firefox');

/**
 * Base URL of the LightNVR server under test.
 * @returns {string} The base URL, without a trailing slash
 */
function baseUrl() {
  return (process.env.E2E_BASE_URL || 'http://localhost:8080').replace(/\/+$/, '');
}

/**
 * Build an absolute URL for a path on the server under test
 * @param {string} path - Path such as '/login.html'
 * @returns {string} The absolute URL
 */
function url(path) {
  return `${baseUrl()}${path.startsWith('/') ? path : `/${path}`}`;
}

/**
 * Create a WebDriver instance for the specified browser
 *
 * The driver binary is resolved by Selenium Manager, which downloads a build
 * matching the locally installed browser. Do not pin a chromedriver/geckodriver
 * npm package: it lands on PATH and overrides this, breaking whenever the
 * browser and the pin drift apart.
 *
 * @param {string} [browserName] - The browser to use ('chrome' or 'firefox');
 *   defaults to $E2E_BROWSER, else 'chrome'
 * @param {boolean} [headless] - Whether to run headless; defaults to true
 *   unless $E2E_HEADLESS is 'false'
 * @returns {WebDriver} The WebDriver instance
 */
async function createDriver(browserName, headless) {
  const browser = (browserName || process.env.E2E_BROWSER || 'chrome').toLowerCase();
  const runHeadless = headless === undefined
    ? process.env.E2E_HEADLESS !== 'false'
    : headless;

  let driver;

  if (browser === 'chrome') {
    const options = new chrome.Options();

    if (runHeadless) {
      // `options.headless()` was removed in selenium-webdriver 4.x.
      options.addArguments('--headless=new', '--no-sandbox', '--disable-dev-shm-usage');
    }

    driver = await new Builder()
      .forBrowser('chrome')
      .setChromeOptions(options)
      .build();
  } else if (browser === 'firefox') {
    const options = new firefox.Options();

    if (runHeadless) {
      options.addArguments('-headless');
    }

    driver = await new Builder()
      .forBrowser('firefox')
      .setFirefoxOptions(options)
      .build();
  } else {
    throw new Error(`Unsupported browser: ${browser}`);
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
    await driver.get(url('/login.html'));
    
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
  baseUrl,
  url,
  createDriver,
  takeScreenshot,
  sleep,
  login
};
