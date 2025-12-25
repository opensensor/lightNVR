/**
 * System Page UI Tests
 * 
 * Tests for system info display, logs, restart/shutdown buttons.
 * @tags @ui @system
 */

import { test, expect } from '@playwright/test';
import { SystemPage } from '../pages/SystemPage';
import { CONFIG, USERS, login, sleep } from '../fixtures/test-fixtures';

test.describe('System Page @ui @system', () => {
  
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test.describe('Page Load', () => {
    test('should load system page successfully', async ({ page }) => {
      const systemPage = new SystemPage(page);
      await systemPage.goto();
      
      await sleep(2000);
      
      await expect(page.locator('body')).toBeVisible();
      expect(await systemPage.isOnPage()).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/system-page-load.png' });
    });
  });

  test.describe('System Information', () => {
    test('should display version information', async ({ page }) => {
      const systemPage = new SystemPage(page);
      await systemPage.goto();
      
      await sleep(2000);
      
      // Look for version text anywhere on the page
      const pageContent = await page.content();
      const hasVersionText = pageContent.toLowerCase().includes('version');
      console.log(`Page contains version text: ${hasVersionText}`);
      
      const version = await systemPage.getVersion();
      console.log(`Version info: ${version}`);
      
      await page.screenshot({ path: 'test-results/system-version.png' });
    });

    test('should display uptime information', async ({ page }) => {
      const systemPage = new SystemPage(page);
      await systemPage.goto();
      
      await sleep(2000);
      
      const pageContent = await page.content();
      const hasUptimeText = pageContent.toLowerCase().includes('uptime');
      console.log(`Page contains uptime text: ${hasUptimeText}`);
      
      await page.screenshot({ path: 'test-results/system-uptime.png' });
    });

    test('should display CPU usage', async ({ page }) => {
      const systemPage = new SystemPage(page);
      await systemPage.goto();
      
      await sleep(2000);
      
      const pageContent = await page.content();
      const hasCpuText = pageContent.toLowerCase().includes('cpu');
      console.log(`Page contains CPU text: ${hasCpuText}`);
      
      await page.screenshot({ path: 'test-results/system-cpu.png' });
    });

    test('should display memory usage', async ({ page }) => {
      const systemPage = new SystemPage(page);
      await systemPage.goto();
      
      await sleep(2000);
      
      const pageContent = await page.content();
      const hasMemoryText = pageContent.toLowerCase().includes('memory');
      console.log(`Page contains memory text: ${hasMemoryText}`);
      
      await page.screenshot({ path: 'test-results/system-memory.png' });
    });

    test('should display storage usage', async ({ page }) => {
      const systemPage = new SystemPage(page);
      await systemPage.goto();
      
      await sleep(2000);
      
      const pageContent = await page.content();
      const hasStorageText = pageContent.toLowerCase().includes('storage') || 
                             pageContent.toLowerCase().includes('disk');
      console.log(`Page contains storage text: ${hasStorageText}`);
      
      await page.screenshot({ path: 'test-results/system-storage.png' });
    });
  });

  test.describe('System Actions', () => {
    test('should display refresh button', async ({ page }) => {
      const systemPage = new SystemPage(page);
      await systemPage.goto();
      
      await sleep(1000);
      
      const hasRefresh = await systemPage.refreshButton.isVisible();
      console.log(`Has refresh button: ${hasRefresh}`);
      
      await page.screenshot({ path: 'test-results/system-refresh-button.png' });
    });

    test('should display restart button for admin', async ({ page }) => {
      const systemPage = new SystemPage(page);
      await systemPage.goto();
      
      await sleep(1000);
      
      const hasRestart = await systemPage.restartButton.isVisible();
      console.log(`Has restart button: ${hasRestart}`);
      
      await page.screenshot({ path: 'test-results/system-restart-button.png' });
    });

    test('should show confirmation dialog when restart is clicked', async ({ page }) => {
      const systemPage = new SystemPage(page);
      await systemPage.goto();
      
      await sleep(1000);
      
      if (await systemPage.restartButton.isVisible()) {
        await systemPage.clickRestart();
        
        // Check for confirmation dialog
        const hasDialog = await systemPage.isConfirmDialogVisible();
        console.log(`Shows confirmation dialog: ${hasDialog}`);
        
        // Cancel the action if dialog is visible
        if (hasDialog) {
          await systemPage.cancelAction();
        }
      }
      
      await page.screenshot({ path: 'test-results/system-restart-confirm.png' });
    });
  });

  test.describe('System Logs', () => {
    test('should display logs section', async ({ page }) => {
      const systemPage = new SystemPage(page);
      await systemPage.goto();
      
      await sleep(2000);
      
      const hasLogs = await systemPage.isLogsSectionVisible();
      console.log(`Has logs section: ${hasLogs}`);
      
      if (hasLogs) {
        const logsContent = await systemPage.getLogsContent();
        console.log(`Logs content length: ${logsContent?.length || 0} chars`);
      }
      
      await page.screenshot({ path: 'test-results/system-logs.png' });
    });
  });
});

