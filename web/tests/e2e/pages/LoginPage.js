/**
 * Page Object for the Login page
 */
class LoginPage {
  constructor(driver) {
    this.driver = driver;
  }

  // Selectors
  get usernameInput() { return '#username'; }
  get passwordInput() { return '#password'; }
  get loginButton() { return 'form#login-form button[type="submit"]'; }
  get errorMessage() { return 'div[class*="bg-red-100"]'; }
  get rememberMeCheckbox() { return '#remember-me'; }

  /**
   * Navigate to the login page
   */
  async navigate() {
    await this.driver.get('http://localhost:8080/login.html');
  }

  /**
   * Wait for the page to load
   */
  async waitForPageLoad() {
    const { By, until } = require('selenium-webdriver');
    await this.driver.wait(until.elementLocated(By.css(this.usernameInput)), 10000);
  }

  /**
   * Login with the specified credentials
   * @param {string} username - The username to use
   * @param {string} password - The password to use
   * @param {boolean} rememberMe - Whether to check the "Remember Me" checkbox
   */
  async login(username, password, rememberMe = false) {
    const { By } = require('selenium-webdriver');
    
    // Enter username
    const usernameElement = await this.driver.findElement(By.css(this.usernameInput));
    await usernameElement.clear();
    await usernameElement.sendKeys(username);
    
    // Enter password
    const passwordElement = await this.driver.findElement(By.css(this.passwordInput));
    await passwordElement.clear();
    await passwordElement.sendKeys(password);
    
    // Set "Remember Me" checkbox
    if (rememberMe) {
      const rememberMeElement = await this.driver.findElement(By.css(this.rememberMeCheckbox));
      const isChecked = await rememberMeElement.isSelected();
      
      if (!isChecked) {
        await rememberMeElement.click();
      }
    }
    
    // Click login button
    const loginButton = await this.driver.findElement(By.css(this.loginButton));
    await loginButton.click();
  }

  /**
   * Get the error message text (if any)
   * @returns {string|null} The error message text, or null if no error message is displayed
   */
  async getErrorMessage() {
    const { By } = require('selenium-webdriver');
    
    try {
      const errorElement = await this.driver.findElement(By.css(this.errorMessage));
      return await errorElement.getText();
    } catch (error) {
      return null; // No error message displayed
    }
  }

  /**
   * Check if the user is logged in
   * @returns {boolean} True if the user is logged in, false otherwise
   */
  async isLoggedIn() {
    // After login, we should be redirected to another page
    // We can check the URL to see if we're still on the login page
    const currentUrl = await this.driver.getCurrentUrl();
    return !currentUrl.includes('login.html');
  }
}

module.exports = LoginPage;
