# Running Selenium Tests Locally

This guide provides detailed instructions for running the Selenium-based automated tests locally on your development machine.

## Prerequisites

Before running the tests, ensure you have the following installed:

1. **Node.js and npm**: Required to run JavaScript-based tests
   - Download from [nodejs.org](https://nodejs.org/)
   - Verify installation with `node -v` and `npm -v`

2. **Chrome or Firefox browser**: The tests are configured to run in Chrome by default
   - Download Chrome from [google.com/chrome](https://www.google.com/chrome/)
   - Download Firefox from [mozilla.org/firefox](https://www.mozilla.org/firefox/)

3. **WebDriver executables**: Required for Selenium to control the browser
   - **ChromeDriver**: For testing with Chrome
     - Download from [chromedriver.chromium.org](https://chromedriver.chromium.org/downloads)
     - Make sure to download a version compatible with your Chrome browser
   - **GeckoDriver**: For testing with Firefox
     - Download from [github.com/mozilla/geckodriver/releases](https://github.com/mozilla/geckodriver/releases)

4. **Add WebDriver to your PATH**: Ensure the WebDriver executables are in your system PATH
   - **Windows**: Add the directory containing the WebDriver to your PATH environment variable
   - **macOS/Linux**: Move the WebDriver to `/usr/local/bin` or add its location to your PATH

5. **Application running with authentication**: The tests assume:
   - The application is running at http://localhost:8080
   - Authentication is enabled
   - Default credentials are username: `admin` and password: `admin`

## Installation

1. **Install dependencies**:
   ```bash
   cd /path/to/nvr_soft/web
   npm install
   ```

   This will install all the required dependencies, including:
   - Jest (testing framework)
   - Selenium WebDriver
   - ChromeDriver/GeckoDriver

2. **Verify WebDriver installation**:
   ```bash
   # For Chrome
   chromedriver --version
   
   # For Firefox
   geckodriver --version
   ```

## Running the Tests

### Option 1: Using the Convenience Script

We've provided a convenience script that handles dependency installation and test execution:

```bash
cd /path/to/nvr_soft/web/tests
chmod +x install-and-run.sh  # Make sure the script is executable
./install-and-run.sh
```

### Option 2: Running Tests Manually

1. **Start your application server**:
   ```bash
   # Navigate to the web directory
   cd /path/to/nvr_soft/web
   
   # Start the development server
   npm start
   ```

2. **In a separate terminal, run the tests**:
   ```bash
   # Navigate to the web directory
   cd /path/to/nvr_soft/web
   
   # Run all E2E tests
   npm run test:e2e
   
   # Or run a specific test file
   npx jest tests/e2e/specs/streams.spec.js
   ```

### Running Tests in Headless Mode

By default, tests run with the browser visible. To run in headless mode (without a visible browser window):

1. **Edit the test file** you want to run in headless mode:
   ```javascript
   // Change this line in the test file
   driver = await createDriver('chrome', false);
   
   // To this
   driver = await createDriver('chrome', true);
   ```

2. **Or temporarily modify the test-utils.js file** to make all tests run in headless mode:
   ```javascript
   // In web/tests/e2e/utils/test-utils.js
   // Change the default parameter
   async function createDriver(browserName = 'chrome', headless = true) {
     // ...
   }
   ```

## Viewing Test Results

1. **Console output**: Test results will be displayed in the terminal

2. **Screenshots**: During test execution, screenshots are saved to the `web/tests/screenshots` directory
   - These are useful for debugging test failures
   - Each test captures screenshots at key points in the test flow

3. **Debugging failures**: If a test fails, check:
   - The error message in the console
   - The screenshots taken during the test
   - The state of your application (database, logs, etc.)

## Common Issues and Solutions

### ES Modules vs CommonJS

**Error**: `ReferenceError: module is not defined in ES module scope`

**Solution**:
- This project uses ES modules (`"type": "module"` in package.json) but Jest and Babel configurations use CommonJS
- We've addressed this by renaming configuration files to use the `.cjs` extension:
  - `jest.config.cjs` instead of `jest.config.js`
  - `babel.config.cjs` instead of `babel.config.js`
  - `setup.cjs` instead of `setup.js`
- If you create new configuration files that use CommonJS syntax, use the `.cjs` extension
- Test files can continue to use the `.js` extension as they're processed by Babel

### WebDriver Not Found

**Error**: `Error: The ChromeDriver could not be found on the current PATH`

**Solution**:
- Ensure the WebDriver executable is in your PATH
- Verify the WebDriver version matches your browser version
- Try specifying the WebDriver path explicitly in your test:
  ```javascript
  const chrome = require('selenium-webdriver/chrome');
  const options = new chrome.Options();
  options.setChromeBinaryPath('/path/to/chromedriver');
  ```

### Browser Version Mismatch

**Error**: `This version of ChromeDriver only supports Chrome version XX`

**Solution**:
- Update your ChromeDriver to match your Chrome browser version:
  ```bash
  # Check your Chrome version
  google-chrome --version
  # Example output: Google Chrome 134.0.6998.117
  
  # Update the chromedriver version in package.json
  # Change this line:
  # "chromedriver": "^120.0.0",
  # To match your Chrome version:
  # "chromedriver": "^134.0.0",
  
  # Then reinstall dependencies
  npm install
  ```
- Or update your Chrome browser to match the ChromeDriver version
- Alternatively, you can use a WebDriver manager that automatically downloads the correct driver:
  ```bash
  npm install --save-dev webdriver-manager
  npx webdriver-manager update
  ```

### Connection Refused

**Error**: `Error: connect ECONNREFUSED 127.0.0.1:8080` or `WebDriverError: unknown error: net::ERR_CONNECTION_REFUSED`

**Solution**:
- Ensure your application server is running before starting the tests
  ```bash
  # In one terminal, start the application
  cd /path/to/nvr_soft/web
  npm start
  
  # In another terminal, run the tests
  cd /path/to/nvr_soft/web
  npm run test:e2e
  ```
- Verify it's running on the expected port (8080)
- Check for any firewall issues
- If you're running the application in a Docker container, make sure port 8080 is properly exposed

### Element Not Found

**Error**: `NoSuchElementError: no such element: Unable to locate element`

**Solution**:
- Check if the selector is correct
- Increase the wait time for the element to appear
- Verify the page structure hasn't changed
- Check the screenshots to see the actual state of the page
- For navigation tests, be aware of the application's navigation patterns:
  - Some pages are accessed through submenu items rather than main navigation
  - For example, the Timeline page is accessed via the Recordings page, not directly from the main menu

### Navigation Patterns

**Note**: The application has some specific navigation patterns that the tests need to follow:

- **Timeline Page**: Accessed through the Recordings page, not directly from the main navigation menu
  ```javascript
  // Example of navigating to the Timeline page
  async navigateToTimeline() {
    // First navigate to the recordings page
    await this.navigateToRecordings();
    
    // Wait for the recordings page to load
    await this.driver.wait(until.elementLocated(By.css('h2.text-xl')), 10000);
    
    // Then click on the Timeline View link
    const timelineLink = await this.driver.findElement(By.css('a[href="timeline.html"]'));
    await timelineLink.click();
  }
  ```

## Customizing Tests

### Testing Different Browsers

To run tests in Firefox instead of Chrome:

```javascript
// In your test file
driver = await createDriver('firefox', false);
```

### Adjusting Timeouts

If tests are failing due to timeout issues:

```javascript
// In web/tests/setup.js
jest.setTimeout(60000); // Increase to 60 seconds

// Or in individual test files
test('my slow test', async () => {
  // ...
}, 60000); // Timeout for this specific test
```

### Running Tests in Parallel

By default, tests run sequentially. To run them in parallel:

```bash
# Add this to package.json scripts
"test:e2e:parallel": "jest tests/e2e --maxWorkers=4"

# Then run
npm run test:e2e:parallel
```

Note: Running Selenium tests in parallel may require additional setup to avoid conflicts.

## Continuous Integration

These tests can also be run in a CI environment. The key requirements are:
- Node.js installation
- Browser installation (in headless mode)
- WebDriver installation
- Running the application server before tests

For detailed CI setup instructions, please refer to the CI configuration documentation.
