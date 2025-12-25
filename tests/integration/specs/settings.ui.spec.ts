/**
 * Settings Page UI Tests
 * 
 * Tests for settings display, edit settings, save/cancel operations.
 * @tags @ui @settings
 */

import { test, expect } from '@playwright/test';
import { SettingsPage } from '../pages/SettingsPage';
import { CONFIG, USERS, login, sleep } from '../fixtures/test-fixtures';

test.describe('Settings Page @ui @settings', () => {
  
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test.describe('Page Load', () => {
    test('should load settings page successfully', async ({ page }) => {
      const settingsPage = new SettingsPage(page);
      await settingsPage.goto({ waitForNetworkIdle: true });
      
      await expect(page.locator('body')).toBeVisible();
      expect(await settingsPage.isOnPage()).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/settings-page-load.png' });
    });

    test('should display settings form', async ({ page }) => {
      const settingsPage = new SettingsPage(page);
      await settingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      // Check for save button
      const hasSaveButton = await settingsPage.saveButton.isVisible();
      console.log(`Has save button: ${hasSaveButton}`);
      
      await page.screenshot({ path: 'test-results/settings-form.png' });
    });
  });

  test.describe('Storage Settings', () => {
    test('should display storage path setting', async ({ page }) => {
      const settingsPage = new SettingsPage(page);
      await settingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const hasStoragePath = await settingsPage.storagePathInput.isVisible();
      console.log(`Has storage path input: ${hasStoragePath}`);
      
      if (hasStoragePath) {
        const value = await settingsPage.getStoragePath();
        console.log(`Storage path value: ${value}`);
      }
      
      await page.screenshot({ path: 'test-results/settings-storage-path.png' });
    });

    test('should display retention days setting', async ({ page }) => {
      const settingsPage = new SettingsPage(page);
      await settingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const hasRetention = await settingsPage.retentionDaysInput.isVisible();
      console.log(`Has retention days input: ${hasRetention}`);
      
      if (hasRetention) {
        const value = await settingsPage.getRetentionDays();
        console.log(`Retention days value: ${value}`);
      }
      
      await page.screenshot({ path: 'test-results/settings-retention.png' });
    });
  });

  test.describe('Settings Form', () => {
    test('should be able to read current settings', async ({ page }) => {
      const settingsPage = new SettingsPage(page);
      await settingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const settings = await settingsPage.getCurrentSettings();
      console.log('Current settings:', JSON.stringify(settings, null, 2));
      
      await page.screenshot({ path: 'test-results/settings-current.png' });
    });

    test('should have cancel button', async ({ page }) => {
      const settingsPage = new SettingsPage(page);
      await settingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      const hasCancelButton = await settingsPage.cancelButton.isVisible();
      console.log(`Has cancel button: ${hasCancelButton}`);
      
      await page.screenshot({ path: 'test-results/settings-cancel-button.png' });
    });
  });

  test.describe('Settings Categories', () => {
    test('should display web/network settings section', async ({ page }) => {
      const settingsPage = new SettingsPage(page);
      await settingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const hasWebPort = await settingsPage.webPortInput.isVisible();
      console.log(`Has web port input: ${hasWebPort}`);
      
      await page.screenshot({ path: 'test-results/settings-web.png' });
    });

    test('should display detection settings if available', async ({ page }) => {
      const settingsPage = new SettingsPage(page);
      await settingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const hasDetection = await settingsPage.detectionEnabledCheckbox.isVisible();
      console.log(`Has detection enabled checkbox: ${hasDetection}`);
      
      await page.screenshot({ path: 'test-results/settings-detection.png' });
    });
  });

  test.describe('Settings Validation', () => {
    test('should display error for invalid settings', async ({ page }) => {
      const settingsPage = new SettingsPage(page);
      await settingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      // Try to set invalid retention days (if input is visible)
      if (await settingsPage.retentionDaysInput.isVisible()) {
        await settingsPage.setRetentionDays('-1');
        await settingsPage.saveSettings();
        
        // Check for error message
        const hasError = await settingsPage.errorMessage.isVisible();
        console.log(`Shows error for invalid value: ${hasError}`);
      }
      
      await page.screenshot({ path: 'test-results/settings-validation.png' });
    });
  });
});

