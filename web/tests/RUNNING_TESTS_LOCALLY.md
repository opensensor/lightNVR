# Running Selenium Tests Locally

This guide provides detailed instructions for running the Selenium-based automated tests locally on your development machine.

## Prerequisites

Before running the tests, ensure you have the following installed:

1. **Node.js 24.11+ (24.x) and npm**: Required to run JavaScript-based tests
   - Download from [nodejs.org](https://nodejs.org/)
   - Verify installation with `node -v` and `npm -v`

2. **Chrome or Firefox browser**: The tests are configured to run in Chrome by default
   - Download Chrome from [google.com/chrome](https://www.google.com/chrome/)
   - Download Firefox from [mozilla.org/firefox](https://www.mozilla.org/firefox/)

3. **WebDriver executables**: Nothing to install. Selenium Manager (bundled with
   `selenium-webdriver`) downloads a driver matching your installed browser on
   first run and caches it under `~/.cache/selenium`.

   Do **not** add a `chromedriver` or `geckodriver` npm package. Those install a
   pinned binary into `node_modules/.bin`, which npm puts on PATH ahead of
   Selenium Manager — so the driver stops tracking your browser and the suite
   breaks the next time Chrome auto-updates.

4. **Application running with authentication**: The tests assume:
   - The application is running at http://localhost:8080
     (override with `E2E_BASE_URL`)
   - Authentication is enabled
   - Default credentials are username: `admin` and password: `admin`

   A plain `npm start` (Vite dev server) only serves the static frontend; the
   login and streams specs need a real LightNVR backend to authenticate against.

## Installation

1. **Install dependencies**:
   ```bash
   cd /path/to/lightNVR/web
   npm install
   ```

   This will install all the required dependencies, including:
   - Jest (testing framework)
   - Selenium WebDriver (which bundles Selenium Manager)

## Running the Tests

### Option 1: Using the Convenience Script

We've provided a convenience script that handles dependency installation and test execution:

```bash
cd /path/to/lightNVR/web/tests
chmod +x install-and-run.sh  # Make sure the script is executable
./install-and-run.sh
```

### Option 2: Running Tests Manually

1. **Start your application server**:
   ```bash
   # Navigate to the web directory
   cd /path/to/lightNVR/web
   
   # Start the development server
   npm start
   ```

2. **In a separate terminal, run the tests**:
   ```bash
   # Navigate to the web directory
   cd /path/to/lightNVR/web
   
   # Run all E2E tests
   npm run test:e2e
   
   # Or run a specific test file
   npx jest --config=jest.e2e.config.cjs tests/e2e/specs/streams.spec.js
   ```

Note that `npm test` runs the **unit** suites only. The E2E specs need a browser
and a running server, so they live in a separate config (`jest.e2e.config.cjs`)
and only run via `npm run test:e2e`.

### Configuration

The E2E suite is driven by environment variables — no source edits required:

| Variable | Default | Purpose |
| --- | --- | --- |
| `E2E_BASE_URL` | `http://localhost:8080` | Base URL of the server under test |
| `E2E_BROWSER` | `chrome` | `chrome` or `firefox` |
| `E2E_HEADLESS` | headless | Set to `false` to watch the browser |

```bash
# Watch the browser while testing against a box on the LAN
E2E_HEADLESS=false E2E_BASE_URL=http://nvr.local:8080 npm run test:e2e
```

## Viewing Test Results

1. **Console output**: Test results will be displayed in the terminal

2. **Screenshots**: During test execution, screenshots are saved to the `web/screenshots` directory (gitignored)
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
- Selenium Manager resolves the driver automatically, but it needs network
  access on first run to download it. Check your proxy settings, then retry.
- The cache lives at `~/.cache/selenium`; deleting it forces a fresh download.

### Browser Version Mismatch

**Error**: `This version of ChromeDriver only supports Chrome version XX`

This should no longer happen — Selenium Manager picks a driver that matches the
browser you actually have installed. Seeing this error means a stale driver
binary is shadowing it on PATH. Find it with:

```bash
which -a chromedriver
ls node_modules/.bin/chromedriver   # should not exist
```

**Solution**:
- If it is in `node_modules/.bin`, someone re-added the `chromedriver` (or
  `geckodriver`) npm package. Remove it from `package.json` and reinstall —
  pinning a driver version means re-pinning it every time Chrome updates.
- If it is a system-wide install, remove it or drop it from PATH.

### Connection Refused

**Error**: `Error: connect ECONNREFUSED 127.0.0.1:8080` or `WebDriverError: unknown error: net::ERR_CONNECTION_REFUSED`

**Solution**:
- Ensure your application server is running before starting the tests
  ```bash
  # In one terminal, start the application
  cd /path/to/lightNVR/web
  npm start
  
  # In another terminal, run the tests
  cd /path/to/lightNVR/web
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

```bash
E2E_BROWSER=firefox npm run test:e2e
```

### Adjusting Timeouts

The E2E suite already allows 60s per test (`testTimeout` in
`jest.e2e.config.cjs`). To change it globally, edit that value; to raise it for
a single test:

```javascript
test('my slow test', async () => {
  // ...
}, 120000); // Timeout for this specific test
```

`web/tests/setup.cjs` also calls `jest.setTimeout()` and applies to both suites.

### Running Tests in Parallel

By default, tests run sequentially. To run them in parallel:

```bash
# Add this to package.json scripts
"test:e2e:parallel": "jest --config=jest.e2e.config.cjs --maxWorkers=4"

# Then run
npm run test:e2e:parallel
```

Note: Running Selenium tests in parallel may require additional setup to avoid conflicts.

## Continuous Integration

These tests can also be run in a CI environment. The key requirements are:
- Node.js installation
- Browser installation (Chrome or Firefox); the suite is headless by default
- Running the application server before tests, with `E2E_BASE_URL` pointed at it

No WebDriver installation step is needed — Selenium Manager fetches a matching
driver at runtime. Cache `~/.cache/selenium` between runs to avoid re-downloading.

For detailed CI setup instructions, please refer to the CI configuration documentation.
