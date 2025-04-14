/**
 * Login page E2E tests
 */
const { By, until } = require('selenium-webdriver');
const { createDriver, takeScreenshot } = require('../utils/test-utils');
const LoginPage = require('../pages/LoginPage');

describe('Login Page', () => {
  let driver;
  let loginPage;
  
  beforeAll(async () => {
    // Create the WebDriver
    driver = await createDriver('chrome', false);
    
    // Create the page object
    loginPage = new LoginPage(driver);
  });
  
  afterAll(async () => {
    // Quit the driver
    if (driver) {
      await driver.quit();
    }
  });
  
  beforeEach(async () => {
    // Navigate to the login page before each test
    await loginPage.navigate();
    await loginPage.waitForPageLoad();
  });
  
  test('should load the login page correctly', async () => {
    // Check that the login form elements are present
    const usernameInput = await driver.findElement(By.css(loginPage.usernameInput));
    const passwordInput = await driver.findElement(By.css(loginPage.passwordInput));
    const loginButton = await driver.findElement(By.css(loginPage.loginButton));
    
    expect(await usernameInput.isDisplayed()).toBe(true);
    expect(await passwordInput.isDisplayed()).toBe(true);
    expect(await loginButton.isDisplayed()).toBe(true);
    
    // Take a screenshot of the login page
    await takeScreenshot(driver, 'screenshots/login-page.png');
  });
  
  test('should show an error message with invalid credentials', async () => {
    // Attempt to login with invalid credentials
    await loginPage.login('invalid_user', 'invalid_password');
    
    // Wait for the error message to appear
    await driver.wait(until.elementLocated(By.css(loginPage.errorMessage)), 5000);
    
    // Check the error message
    const errorMessage = await loginPage.getErrorMessage();
    expect(errorMessage).not.toBeNull();
    expect(errorMessage.toLowerCase()).toContain('invalid');
    
    // Take a screenshot of the error
    await takeScreenshot(driver, 'screenshots/login-error.png');
  });
  
  test('should login successfully with valid credentials', async () => {
    // Login with valid credentials
    await loginPage.login('admin', 'admin');
    
    // Wait for redirection after successful login
    await driver.wait(async () => {
      const isLoggedIn = await loginPage.isLoggedIn();
      return isLoggedIn;
    }, 5000, 'Timed out waiting for successful login redirection');
    
    // Check that we're logged in
    const isLoggedIn = await loginPage.isLoggedIn();
    expect(isLoggedIn).toBe(true);
    
    // Take a screenshot after successful login
    await takeScreenshot(driver, 'screenshots/after-login.png');
  });
});
