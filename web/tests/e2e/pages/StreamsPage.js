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
  get deleteModal() { return '.fixed.inset-0.bg-black'; }
  get softDisableButton() { return 'button:contains("Disable Stream")'; }
  get hardDeleteButton() { return 'button:contains("Delete Stream")'; }
  get confirmDeleteButton() { return 'button:contains("Yes, Delete Permanently")'; }
  get cancelDeleteButton() { return 'button:contains("Cancel")'; }

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
   * @param {boolean} isAfterDelete - Whether this check is after a delete operation
   */
  async streamExists(streamName, isAfterDelete = false) {
    // If this is after a delete operation, try multiple times with a delay
    if (isAfterDelete) {
      console.log(`Checking if stream "${streamName}" exists after deletion (with retries)`);

      // Try up to 3 times with a delay between attempts
      for (let attempt = 1; attempt <= 3; attempt++) {
        console.log(`Attempt ${attempt} to check if stream exists after deletion`);

        // Force a refresh on each attempt
        if (attempt > 1) {
          console.log('Refreshing page for next attempt...');
          await this.driver.navigate().refresh();
          await this.waitForPageLoad();
          await this.driver.sleep(1000); // Wait for the table to update
        }

        // Check if the stream exists
        const exists = await this._checkStreamExists(streamName);

        // If the stream doesn't exist, we're done
        if (!exists) {
          console.log(`Stream "${streamName}" no longer exists (confirmed on attempt ${attempt})`);
          return false;
        }

        // If this isn't the last attempt, wait before trying again
        if (attempt < 3) {
          console.log(`Stream still exists, waiting before attempt ${attempt + 1}...`);
          await this.driver.sleep(2000);
        }
      }

      // If we get here, the stream still exists after all attempts
      console.log(`Stream "${streamName}" still exists after ${3} attempts`);
      return true;
    } else {
      // Normal check (not after delete)
      return await this._checkStreamExists(streamName);
    }
  }

  /**
   * Internal method to check if a stream exists
   * @private
   */
  async _checkStreamExists(streamName) {
    // Get the current list of streams directly from the table
    const { By } = require('selenium-webdriver');
    const rows = await this.driver.findElements(By.css(this.streamRows));
    console.log(`Found ${rows.length} stream rows when checking if "${streamName}" exists`);

    // Check each row for the stream name
    let exists = false;
    for (let i = 0; i < rows.length; i++) {
      try {
        const nameCell = await rows[i].findElement(By.css('td:first-child'));
        const name = await nameCell.getText();
        console.log(`Row ${i}: Stream name = "${name}"`);

        // Check if this is the stream we're looking for
        if (name.trim() === streamName.trim()) {
          console.log(`Found exact match for stream "${streamName}" at row ${i}`);
          exists = true;
          break;
        }
      } catch (error) {
        console.log(`Error getting name from row ${i}: ${error.message}`);
      }
    }

    console.log(`Stream "${streamName}" exists: ${exists}`);
    return exists;
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
    console.log(`Looking for stream to delete: ${streamName}`);

    try {
      // Get all stream rows
      const rows = await this.driver.findElements(By.css(this.streamRows));
      console.log(`Found ${rows.length} stream rows`);

      let found = false;

      // Loop through rows to find the stream
      for (let i = 0; i < rows.length; i++) {
        const nameCell = await rows[i].findElement(By.css('td:first-child'));
        const name = await nameCell.getText();
        console.log(`Row ${i}: Stream name = "${name}"`);

        if (name.includes(streamName)) {
          console.log(`Found stream to delete at row ${i}`);
          found = true;

          // Take a screenshot before clicking delete
          const fs = require('fs');
          const screenshot = await this.driver.takeScreenshot();
          const dir = 'screenshots';
          if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir, { recursive: true });
          }
          fs.writeFileSync(`${dir}/before-delete-click.png`, screenshot, 'base64');
          console.log('Screenshot before delete click saved');

          // Find and click the delete button
          const deleteButton = await rows[i].findElement(By.css('button[title="Delete"]'));
          console.log('Found delete button');
          await deleteButton.click();
          console.log('Clicked delete button');

          break;
        }
      }

      if (!found) {
        console.error(`Stream "${streamName}" not found in the table`);

        // Take a screenshot of the current state
        const fs = require('fs');
        const screenshot = await this.driver.takeScreenshot();
        const dir = 'screenshots';
        if (!fs.existsSync(dir)) {
          fs.mkdirSync(dir, { recursive: true });
        }
        fs.writeFileSync(`${dir}/stream-not-found.png`, screenshot, 'base64');
        console.log('Screenshot of stream not found saved');

        throw new Error(`Stream "${streamName}" not found in the table`);
      }
    } catch (error) {
      console.error('Error in deleteStream:', error);

      // Take a screenshot of the error state
      const fs = require('fs');
      const screenshot = await this.driver.takeScreenshot();
      const dir = 'screenshots';
      if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
      }
      fs.writeFileSync(`${dir}/delete-stream-error.png`, screenshot, 'base64');
      console.log('Screenshot of error state saved');

      throw error;
    }
  }

  /**
   * Confirm stream deletion with hard delete (permanent)
   */
  async confirmDelete() {
    const { By, until } = require('selenium-webdriver');
    console.log('Waiting for delete modal...');

    try {
      // Wait for the modal to appear
      await this.driver.wait(until.elementLocated(By.css(this.deleteModal)), 5000);
      console.log('Delete modal found');

      // Take a screenshot of the delete modal
      const fs = require('fs');
      const screenshot1 = await this.driver.takeScreenshot();
      const dir = 'screenshots';
      if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
      }
      fs.writeFileSync(`${dir}/delete-modal-1.png`, screenshot1, 'base64');
      console.log('Screenshot of delete modal saved');

      // Find and click the Delete Stream (permanent) button
      try {
        // First, try using XPath to find the button by its text content
        const hardDeleteButton = await this.driver.findElement(By.xpath("//button[contains(text(), 'Delete Stream')]"));
        console.log('Found Delete Stream button');
        await hardDeleteButton.click();
        console.log('Clicked Delete Stream button');
      } catch (error) {
        console.log('Delete Stream button not found with XPath, trying alternative approach...');
        // Try finding all buttons and check their text
        const buttons = await this.driver.findElements(By.css('button'));
        let found = false;

        for (const button of buttons) {
          try {
            const text = await button.getText();
            console.log(`Found button with text: "${text}"`);

            if (text.includes('Delete Stream')) {
              console.log('Found Delete Stream button by text content');
              await button.click();
              console.log('Clicked Delete Stream button');
              found = true;
              break;
            }
          } catch (err) {
            console.log('Error getting button text:', err.message);
          }
        }

        if (!found) {
          // If still not found, try clicking any button with 'delete' in its text
          for (const button of buttons) {
            try {
              const text = await button.getText();
              if (text.toLowerCase().includes('delete')) {
                console.log(`Found button with text containing 'delete': "${text}"`);
                await button.click();
                console.log('Clicked button containing "delete"');
                found = true;
                break;
              }
            } catch (err) {
              console.log('Error getting button text:', err.message);
            }
          }
        }

        if (!found) {
          throw new Error('Could not find Delete Stream button');
        }
      }

      // Wait a moment for the confirmation dialog to appear
      await this.driver.sleep(1000);

      // Take a screenshot after clicking Delete Stream
      const screenshot2 = await this.driver.takeScreenshot();
      fs.writeFileSync(`${dir}/after-delete-stream-click.png`, screenshot2, 'base64');
      console.log('Screenshot after Delete Stream click saved');

      // Now find and click the "Yes, Delete Permanently" button
      try {
        // First try XPath to find the button by its text content
        const confirmButton = await this.driver.findElement(By.xpath("//button[contains(text(), 'Yes, Delete Permanently')]"));
        console.log('Found Yes, Delete Permanently button');
        await confirmButton.click();
        console.log('Clicked Yes, Delete Permanently button');
      } catch (error) {
        console.log('Yes, Delete Permanently button not found with XPath, trying alternative approach...');
        // Try finding all buttons and check their text
        const buttons = await this.driver.findElements(By.css('button'));
        let found = false;

        for (const button of buttons) {
          try {
            const text = await button.getText();
            console.log(`Found button with text: "${text}"`);

            if (text.includes('Yes, Delete Permanently') ||
                text.includes('Delete Permanently') ||
                (text.includes('Yes') && text.includes('Delete'))) {
              console.log('Found confirmation button by text content');
              await button.click();
              console.log('Clicked confirmation button');
              found = true;
              break;
            }
          } catch (err) {
            console.log('Error getting button text:', err.message);
          }
        }

        if (!found) {
          // If still not found, try clicking any red button (likely the delete button)
          const redButtons = await this.driver.findElements(By.css('button.bg-red-600'));
          if (redButtons.length > 0) {
            console.log('Found red button (likely delete button)');
            await redButtons[0].click();
            console.log('Clicked red button');
            found = true;
          }
        }

        if (!found) {
          throw new Error('Could not find confirmation button');
        }
      }

      // Refresh the page to ensure we see the updated list
      console.log('Refreshing page to see updated stream list');
      await this.driver.navigate().refresh();

      // Wait for the page to load after refresh
      await this.waitForPageLoad();

      // Wait for the fetch to complete and update the table
      console.log('Waiting for table to update after refresh...');
      await this.driver.sleep(2000); // Longer wait for the table to update
      console.log('Page refreshed and table updated, stream should be deleted now');

    } catch (error) {
      console.error('Error in confirmDelete:', error.message);

      // Take a screenshot of the error state
      const fs = require('fs');
      const screenshot = await this.driver.takeScreenshot();
      const dir = 'screenshots';
      if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
      }
      fs.writeFileSync(`${dir}/delete-modal-error.png`, screenshot, 'base64');
      console.log('Screenshot of error state saved');

      throw error;
    }
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
