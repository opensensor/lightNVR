# LightNVR Automated Testing

This directory contains automated tests for the LightNVR web interface.

## Test Structure

The tests are organized as follows:

```
tests/
├── *.spec.js             # Unit tests -- run by `npm test`
├── e2e/                  # End-to-end tests with Selenium
│   ├── pages/            # Page objects
│   ├── specs/            # Test specifications
│   └── utils/            # Test utilities
└── setup.cjs             # Test setup (both suites)
```

The two suites run separately:

| Command | Config | Scope |
| --- | --- | --- |
| `npm test` | `jest.config.cjs` | Unit tests only; no browser or server needed |
| `npm run test:e2e` | `jest.e2e.config.cjs` | Selenium E2E; needs a browser and a running LightNVR server |

E2E is excluded from `npm test` on purpose — those specs drive a real browser
against a live server, so they cannot pass in a plain checkout.

## Page Objects

Page objects encapsulate the structure and behavior of web pages, providing a clean API for tests to interact with the UI. This makes tests more maintainable and readable.

Current page objects:
- `StreamsPage.js` - For interacting with the streams management page
- `LoginPage.js` - For handling authentication
- `NavigationMenu.js` - For navigating between different pages

## Test Utilities

The `utils` directory contains helper functions for common test operations:
- `createDriver()` - Creates a WebDriver instance for Chrome or Firefox
- `url()` / `baseUrl()` - Builds URLs against the server under test
- `takeScreenshot()` - Captures screenshots during test execution
- `sleep()` - Waits for a specified amount of time

## Prerequisites

Before running the tests, make sure you have the following installed:

1. Node.js 24.11+ (24.x) and npm
2. Chrome and/or Firefox browsers
3. Application running with authentication (username: `admin`, password: `admin`)

No WebDriver download is needed: Selenium Manager (bundled with
`selenium-webdriver`) fetches a driver matching your installed browser.

## Running Tests Locally

For detailed instructions on running tests locally, please see [RUNNING_TESTS_LOCALLY.md](./RUNNING_TESTS_LOCALLY.md).

This guide includes:
- Complete prerequisites and setup instructions
- Step-by-step guide to running tests
- Troubleshooting common issues
- Tips for customizing test execution

## Headless Mode

Tests run headless by default. To watch the browser, set `E2E_HEADLESS=false`:

```bash
E2E_HEADLESS=false npm run test:e2e
```

See [RUNNING_TESTS_LOCALLY.md](./RUNNING_TESTS_LOCALLY.md) for the full list of
environment variables (`E2E_BASE_URL`, `E2E_BROWSER`, `E2E_HEADLESS`).

## Screenshots

Screenshots are saved to `web/screenshots` during test execution (gitignored).
This is useful for debugging test failures.

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
