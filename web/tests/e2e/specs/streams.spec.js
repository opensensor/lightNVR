/**
 * Streams page E2E tests
 */
const { By, until } = require('selenium-webdriver');
const { createDriver, takeScreenshot, login } = require('../utils/test-utils');
const StreamsPage = require('../pages/StreamsPage');

describe('Streams Page', () => {
  let driver;
  let streamsPage;
  
  beforeAll(async () => {
    // Create the WebDriver
    driver = await createDriver('chrome', false);
    
    // Create the page object
    streamsPage = new StreamsPage(driver);
    
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
    // Navigate to the streams page before each test
    await streamsPage.navigate();
    await streamsPage.waitForPageLoad();
  });
  
  test('should load the streams page correctly', async () => {
    // Check the page title
    const title = await streamsPage.getPageTitleText();
    expect(title).toBe('Streams');
    
    // Check that the Add Stream button exists
    const addButton = await driver.findElement(By.css(streamsPage.addStreamButton));
    expect(await addButton.isDisplayed()).toBe(true);
    
    // Take a screenshot of the streams page
    await takeScreenshot(driver, 'screenshots/streams-page.png');
  });
  
  test('should open the add stream modal when clicking Add Stream button', async () => {
    // Click the Add Stream button
    await streamsPage.clickAddStream();
    
    // Wait for the modal to appear
    await streamsPage.waitForStreamModal();
    
    // Check that the modal is visible
    const modal = await driver.findElement(By.css(streamsPage.streamModal));
    expect(await modal.isDisplayed()).toBe(true);
    
    // Check that the form elements are present
    const nameInput = await driver.findElement(By.css(streamsPage.streamNameInput));
    const urlInput = await driver.findElement(By.css(streamsPage.streamUrlInput));
    const saveButton = await driver.findElement(By.css(streamsPage.saveButton));
    
    expect(await nameInput.isDisplayed()).toBe(true);
    expect(await urlInput.isDisplayed()).toBe(true);
    expect(await saveButton.isDisplayed()).toBe(true);
    
    // Take a screenshot of the add stream modal
    await takeScreenshot(driver, 'screenshots/add-stream-modal.png');
    
    // Cancel the modal
    await streamsPage.cancelStream();
  });
  
  // Store the test stream name globally so it can be used by the delete test
  let testStreamName;
  
  test('should be able to add a new stream', async () => {
    // Generate a unique stream name
    testStreamName = `Test Stream ${Date.now()}`;
    
    // Click the Add Stream button
    await streamsPage.clickAddStream();
    
    // Wait for the modal to appear
    await streamsPage.waitForStreamModal();
    
    // Fill in the form
    await streamsPage.fillStreamForm({
      name: testStreamName,
      url: 'rtsp://example.com/test',
      enabled: true,
      record: true
    });
    
    // Take a screenshot of the filled form
    await takeScreenshot(driver, 'screenshots/add-stream-filled-form.png');
    
    // Save the stream
    await streamsPage.saveStream();
    
    // Wait for the table to update (this might need adjustment based on your app's behavior)
    await driver.sleep(2000);
    
    // Take a screenshot after adding the stream
    await takeScreenshot(driver, 'screenshots/after-add-stream.png');
    
    // Check if the stream was added
    const exists = await streamsPage.streamExists(testStreamName);
    expect(exists).toBe(true);
  });
  
  // This test depends on the previous test adding a stream
  test('should be able to edit a stream', async () => {
    // Get the list of streams
    const streamNames = await streamsPage.getStreamNames();
    
    // Skip the test if there are no streams
    if (streamNames.length === 0) {
      console.log('No streams to edit, skipping test');
      return;
    }
    
    // Edit the first stream
    const streamToEdit = streamNames[0];
    await streamsPage.editStream(streamToEdit);
    
    // Wait for the modal to appear
    await streamsPage.waitForStreamModal();
    
    // Check that the name field is populated and disabled
    const nameInput = await driver.findElement(By.css(streamsPage.streamNameInput));
    expect(await nameInput.getAttribute('value')).toBe(streamToEdit);
    expect(await nameInput.getAttribute('disabled')).toBe('true');
    
    // Take a screenshot of the edit stream modal
    await takeScreenshot(driver, 'screenshots/edit-stream-modal.png');
    
    // Cancel the edit
    await streamsPage.cancelStream();
  });
  
  // This test deletes the test stream created in the previous test
  test('should be able to delete a stream', async () => {
    // Skip the test if testStreamName is not defined
    if (!testStreamName) {
      console.log('No test stream to delete, skipping test');
      return;
    }
    
    console.log(`Deleting test stream: ${testStreamName}`);
    
    // Delete the test stream
    await streamsPage.deleteStream(testStreamName);
    
    // Confirm deletion
    await streamsPage.confirmDelete();
    
    // Take a screenshot after deleting the stream
    await takeScreenshot(driver, 'screenshots/after-delete-stream.png');
    
    // Check that the stream was deleted (with retries)
    const exists = await streamsPage.streamExists(testStreamName, true);
    expect(exists).toBe(false);
  });
});
