/**
 * Page Object for the Streams page
 */
class StreamsPage {
  constructor(driver) {
    this.driver = driver;
  }

  // Selectors
  get pageTitle() { return '.page-header h2'; }
  get addStreamButton() { return '#add-stream-btn'; }
  get discoverOnvifButton() { return '#discover-onvif-btn'; }
  get streamsTable() { return '#streams-table'; }
  get streamRows() { return '#streams-table tbody tr'; }
  get streamNameCells() { return '#streams-table tbody tr td:first-child'; }
  get editButtons() { return 'button[title="Edit"]'; }
  get deleteButtons() { return 'button[title="Delete"]'; }
  
  // Modal selectors
  get streamModal() { return '#stream-modal'; }
  get streamNameInput() { return '#stream-name'; }
  get streamUrlInput() { return '#stream-url'; }
  get streamEnabledCheckbox() { return '#stream-enabled'; }
  get streamRecordCheckbox() { return '#stream-record'; }
  get saveButton() { return '#stream-save-btn'; }
  get cancelButton() { return '#stream-cancel-btn'; }
  get testConnectionButton() { return '#stream-test-btn'; }
  
  // Delete modal selectors
  get deleteModal() { return '.stream-delete-modal'; }
  get confirmDeleteButton() { return '.stream-delete-modal button.delete-permanent'; }
  get cancelDeleteButton() { return '.stream-delete-modal button.cancel'; }
  
  // Status message
  get statusMessage() { return '#status-message'; }

  /**
   * Navigate to the streams page
   */
  async navigate() {
    await this.driver.get('http://localhost:8080/streams.html');
  }

  /**
   * Wait for the page to load
   */
  async waitForPageLoad() {
    const { By, until } = require('selenium-webdriver');
    await this.driver.wait(until.elementLocated(By.css(this.pageTitle)), 10000);
  }

  /**
   * Get the page title text
   */
  async getPageTitleText() {
    const { By } = require('selenium-webdriver');
    const titleElement = await this.driver.findElement(By.css(this.pageTitle));
    return await titleElement.getText();
  }

  /**
   * Click the Add Stream button
   */
  async clickAddStream() {
    const { By } = require('selenium-webdriver');
    const button = await this.driver.findElement(By.css(this.addStreamButton));
    await button.click();
  }

  /**
   * Wait for the stream modal to be visible
   */
  async waitForStreamModal() {
    const { By, until } = require('selenium-webdriver');
    await this.driver.wait(until.elementLocated(By.css(this.streamModal)), 5000);
  }

  /**
   * Fill in the stream form
   * @param {Object} streamData - Data for the stream
   */
  async fillStreamForm(streamData) {
    const { By } = require('selenium-webdriver');
    
    // Fill in the name
    if (streamData.name) {
      const nameInput = await this.driver.findElement(By.css(this.streamNameInput));
      await nameInput.clear();
      await nameInput.sendKeys(streamData.name);
    }
    
    // Fill in the URL
    if (streamData.url) {
      const urlInput = await this.driver.findElement(By.css(this.streamUrlInput));
      await urlInput.clear();
      await urlInput.sendKeys(streamData.url);
    }
    
    // Set enabled checkbox
    if (streamData.enabled !== undefined) {
      const enabledCheckbox = await this.driver.findElement(By.css(this.streamEnabledCheckbox));
      const isChecked = await enabledCheckbox.isSelected();
      
      if ((streamData.enabled && !isChecked) || (!streamData.enabled && isChecked)) {
        await enabledCheckbox.click();
      }
    }
    
    // Set record checkbox
    if (streamData.record !== undefined) {
      const recordCheckbox = await this.driver.findElement(By.css(this.streamRecordCheckbox));
      const isChecked = await recordCheckbox.isSelected();
      
      if ((streamData.record && !isChecked) || (!streamData.record && isChecked)) {
        await recordCheckbox.click();
      }
    }
  }

  /**
   * Save the stream form
   */
  async saveStream() {
    const { By } = require('selenium-webdriver');
    const saveButton = await this.driver.findElement(By.css(this.saveButton));
    await saveButton.click();
  }

  /**
   * Cancel the stream form
   */
  async cancelStream() {
    const { By } = require('selenium-webdriver');
    const cancelButton = await this.driver.findElement(By.css(this.cancelButton));
    await cancelButton.click();
  }

  /**
   * Get all stream names from the table
   */
  async getStreamNames() {
    const { By } = require('selenium-webdriver');
    const cells = await this.driver.findElements(By.css(this.streamNameCells));
    const names = [];
    
    for (const cell of cells) {
      const text = await cell.getText();
      // Extract just the name (remove the status indicator)
      const name = text.trim();
      names.push(name);
    }
    
    return names;
  }

  /**
   * Check if a stream with the given name exists
   * @param {string} streamName - Name of the stream to check
   */
  async streamExists(streamName) {
    const names = await this.getStreamNames();
    return names.some(name => name.includes(streamName));
  }

  /**
   * Edit a stream by name
   * @param {string} streamName - Name of the stream to edit
   */
  async editStream(streamName) {
    const { By } = require('selenium-webdriver');
    const rows = await this.driver.findElements(By.css(this.streamRows));
    
    for (let i = 0; i < rows.length; i++) {
      const nameCell = await rows[i].findElement(By.css('td:first-child'));
      const name = await nameCell.getText();
      
      if (name.includes(streamName)) {
        const editButton = await rows[i].findElement(By.css('button[title="Edit"]'));
        await editButton.click();
        break;
      }
    }
  }

  /**
   * Delete a stream by name
   * @param {string} streamName - Name of the stream to delete
   */
  async deleteStream(streamName) {
    const { By } = require('selenium-webdriver');
    const rows = await this.driver.findElements(By.css(this.streamRows));
    
    for (let i = 0; i < rows.length; i++) {
      const nameCell = await rows[i].findElement(By.css('td:first-child'));
      const name = await nameCell.getText();
      
      if (name.includes(streamName)) {
        const deleteButton = await rows[i].findElement(By.css('button[title="Delete"]'));
        await deleteButton.click();
        break;
      }
    }
  }

  /**
   * Confirm stream deletion
   */
  async confirmDelete() {
    const { By, until } = require('selenium-webdriver');
    await this.driver.wait(until.elementLocated(By.css(this.deleteModal)), 5000);
    const confirmButton = await this.driver.findElement(By.css(this.confirmDeleteButton));
    await confirmButton.click();
  }

  /**
   * Get the status message text
   */
  async getStatusMessage() {
    const { By, until } = require('selenium-webdriver');
    try {
      await this.driver.wait(until.elementLocated(By.css(this.statusMessage)), 5000);
      const messageElement = await this.driver.findElement(By.css(this.statusMessage));
      return await messageElement.getText();
    } catch (error) {
      return null; // Status message might not be visible
    }
  }
}

module.exports = StreamsPage;
