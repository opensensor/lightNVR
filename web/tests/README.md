# LightNVR Automated Testing

This directory contains automated tests for the LightNVR web interface.

## Test Structure

The tests are organized as follows:

```
tests/
├── e2e/                  # End-to-end tests with Selenium
│   ├── pages/            # Page objects
│   ├── specs/            # Test specifications
│   └── utils/            # Test utilities
├── unit/                 # Unit tests (if needed)
└── setup.js              # Test setup
```

## Page Objects

Page objects encapsulate the structure and behavior of web pages, providing a clean API for tests to interact with the UI. This makes tests more maintainable and readable.

Current page objects:
- `StreamsPage.js` - For interacting with the streams management page
- `LoginPage.js` - For handling authentication
- `NavigationMenu.js` - For navigating between different pages

## Test Utilities

The `utils` directory contains helper functions for common test operations:
- `createDriver()` - Creates a WebDriver instance for Chrome or Firefox
- `takeScreenshot()` - Captures screenshots during test execution
- `sleep()` - Waits for a specified amount of time

## Prerequisites

Before running the tests, make sure you have the following installed:

1. Node.js and npm
2. Chrome and/or Firefox browsers
3. ChromeDriver and/or GeckoDriver (for Selenium)
4. Application running with authentication (username: `admin`, password: `admin`)

## Running Tests Locally

For detailed instructions on running tests locally, please see [RUNNING_TESTS_LOCALLY.md](./RUNNING_TESTS_LOCALLY.md).

This guide includes:
- Complete prerequisites and setup instructions
- Step-by-step guide to running tests
- Troubleshooting common issues
- Tips for customizing test execution

## Headless Mode

By default, tests run with the browser visible. To run in headless mode (without a visible browser window), modify the test file to enable headless mode:

```javascript
// In the test file
driver = await createDriver('chrome', true); // Set second parameter to true for headless mode
```

## Screenshots

Screenshots are saved to the `screenshots` directory during test execution. This is useful for debugging test failures.

## Adding New Tests

To add a new test:

1. Create a new page object in `e2e/pages/` if testing a new page
2. Create a new test specification in `e2e/specs/`
3. Use existing page objects and utilities to interact with the UI

## Best Practices

1. Keep tests independent - each test should be able to run on its own
2. Use page objects to encapsulate page structure and behavior
3. Use descriptive test names that explain what is being tested
4. Take screenshots at key points to help with debugging
5. Add comments to explain complex test logic
6. Use explicit waits rather than fixed timeouts when possible
