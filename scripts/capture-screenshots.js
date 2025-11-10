#!/usr/bin/env node

/**
 * LightNVR Automated Screenshot Capture
 * 
 * This script uses Playwright to automatically capture screenshots of the LightNVR
 * web interface for documentation purposes.
 * 
 * Usage:
 *   npm install --save-dev playwright
 *   node scripts/capture-screenshots.js [options]
 * 
 * Options:
 *   --url <url>          LightNVR URL (default: http://localhost:8080)
 *   --username <user>    Username (default: admin)
 *   --password <pass>    Password (default: admin)
 *   --output <dir>       Output directory (default: docs/images)
 *   --theme <theme>      Theme to use (default: blue)
 *   --dark               Use dark mode
 *   --all-themes         Capture screenshots for all themes
 */

const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

// Parse command line arguments
const args = process.argv.slice(2);
const config = {
  url: 'http://localhost:8080',
  username: 'admin',
  password: 'admin',
  outputDir: 'docs/images',
  theme: 'blue',
  darkMode: false,
  allThemes: false,
};

for (let i = 0; i < args.length; i++) {
  switch (args[i]) {
    case '--url':
      config.url = args[++i];
      break;
    case '--username':
      config.username = args[++i];
      break;
    case '--password':
      config.password = args[++i];
      break;
    case '--output':
      config.outputDir = args[++i];
      break;
    case '--theme':
      config.theme = args[++i];
      break;
    case '--dark':
      config.darkMode = true;
      break;
    case '--all-themes':
      config.allThemes = true;
      break;
  }
}

// Ensure output directory exists
if (!fs.existsSync(config.outputDir)) {
  fs.mkdirSync(config.outputDir, { recursive: true });
}

const THEMES = ['default', 'blue', 'emerald', 'purple', 'rose', 'amber', 'slate'];

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function login(page, username, password) {
  console.log('Logging in...');
  try {
    await page.goto(`${config.url}/login.html`, { waitUntil: 'networkidle', timeout: 10000 });

    // Wait for login form
    await page.waitForSelector('input[name="username"], input[type="text"]', { timeout: 5000 });

    // Fill credentials
    const usernameInput = await page.locator('input[name="username"], input[type="text"]').first();
    await usernameInput.fill(username);

    const passwordInput = await page.locator('input[name="password"], input[type="password"]').first();
    await passwordInput.fill(password);

    // Submit
    const submitButton = await page.locator('button[type="submit"], button').filter({ hasText: /login|sign in/i }).first();
    await submitButton.click();

    // Wait for redirect
    await page.waitForURL('**/index.html', { timeout: 10000 });
    console.log('Login successful');
  } catch (error) {
    console.error('Login failed:', error.message);
    throw error;
  }
}

async function setTheme(page, theme, darkMode = false) {
  console.log(`Setting theme: ${theme} (${darkMode ? 'dark' : 'light'})`);

  try {
    // Navigate to settings
    await page.goto(`${config.url}/settings.html`, { waitUntil: 'networkidle', timeout: 10000 });
    await sleep(2000); // Wait for Preact components to render

    // Try to set dark mode (optional, may not exist)
    try {
      // Find the dark mode toggle button (it's a toggle switch, not text button)
      const darkModeSection = await page.locator('div').filter({ hasText: /Dark Mode/i }).first();
      await darkModeSection.waitFor({ timeout: 5000 });

      // Get the toggle button within the dark mode section
      const toggleButton = await darkModeSection.locator('button').first();

      // Check current state by looking at the description text
      const descText = await darkModeSection.locator('p').textContent();
      const isDark = descText.includes('Switch to light mode');

      if (darkMode !== isDark) {
        await toggleButton.click();
        await sleep(1000);
      }
    } catch (e) {
      console.log('  Dark mode toggle not found, skipping');
    }

    // Try to set theme (optional, may not exist)
    try {
      // Theme buttons have the theme name as text
      const themeNames = {
        'default': 'Default',
        'blue': 'Ocean Blue',
        'emerald': 'Forest Green',
        'purple': 'Royal Purple',
        'rose': 'Sunset Rose',
        'amber': 'Golden Amber',
        'slate': 'Cool Slate'
      };

      const themeName = themeNames[theme] || theme;
      const themeButton = await page.locator('button').filter({ hasText: themeName }).first();
      await themeButton.waitFor({ timeout: 5000 });
      await themeButton.click();
      await sleep(1000);
    } catch (e) {
      console.log(`  Theme button for "${theme}" not found, skipping`);
    }

    console.log('Theme settings applied');
  } catch (error) {
    console.log(`  Warning: Could not set theme: ${error.message}`);
  }
}

async function captureScreenshot(page, name, options = {}) {
  const filename = path.join(config.outputDir, `${name}.png`);
  console.log(`Capturing: ${filename}`);
  
  // Wait for any animations to complete
  await sleep(options.delay || 1000);
  
  if (options.selector) {
    const element = await page.locator(options.selector).first();
    await element.screenshot({ path: filename });
  } else {
    await page.screenshot({ 
      path: filename,
      fullPage: options.fullPage || false 
    });
  }
  
  console.log(`✓ Saved: ${filename}`);
}

async function captureLiveStreams(page) {
  console.log('\n=== Capturing Live Streams ===');
  try {
    await page.goto(`${config.url}/index.html`, { waitUntil: 'networkidle', timeout: 10000 });
    await sleep(2000); // Wait for streams to load

    await captureScreenshot(page, 'live-streams', { delay: 2000 });
  } catch (error) {
    console.log(`  ✗ Failed to capture live streams: ${error.message}`);
  }
}

async function captureStreamManagement(page) {
  console.log('\n=== Capturing Stream Management ===');
  try {
    await page.goto(`${config.url}/streams.html`, { waitUntil: 'networkidle', timeout: 10000 });
    await sleep(1500);

    await captureScreenshot(page, 'stream-management', { delay: 1500 });
  } catch (error) {
    console.log(`  ✗ Failed to capture stream management: ${error.message}`);
  }
}

async function captureRecordingManagement(page) {
  console.log('\n=== Capturing Recording Management ===');
  try {
    await page.goto(`${config.url}/recordings.html`, { waitUntil: 'networkidle', timeout: 10000 });
    await sleep(1500);

    await captureScreenshot(page, 'recording-management', { delay: 1500 });
  } catch (error) {
    console.log(`  ✗ Failed to capture recording management: ${error.message}`);
  }
}

async function captureSettingsManagement(page) {
  console.log('\n=== Capturing Settings Management ===');
  try {
    await page.goto(`${config.url}/settings.html`, { waitUntil: 'networkidle', timeout: 10000 });
    await sleep(1500);

    await captureScreenshot(page, 'settings-management', { delay: 1500 });
  } catch (error) {
    console.log(`  ✗ Failed to capture settings management: ${error.message}`);
  }
}

async function captureSystemInfo(page) {
  console.log('\n=== Capturing System Information ===');
  try {
    await page.goto(`${config.url}/system.html`, { waitUntil: 'networkidle', timeout: 10000 });
    await sleep(1500);

    await captureScreenshot(page, 'system-info', { delay: 1500 });
  } catch (error) {
    console.log(`  ✗ Failed to capture system info: ${error.message}`);
  }
}

async function captureZoneEditor(page) {
  console.log('\n=== Capturing Zone Editor ===');
  try {
    await page.goto(`${config.url}/streams.html`, { waitUntil: 'networkidle', timeout: 10000 });
    await sleep(1500);

    // Try to open stream configuration modal
    try {
      const configButton = await page.locator('button').filter({ hasText: /configure/i }).first();
      await configButton.waitFor({ timeout: 5000 });
      await configButton.click();
      await sleep(1000);

      // Click on Detection Zones tab if it exists
      try {
        const zonesTab = await page.locator('button, a').filter({ hasText: /detection zones/i }).first();
        await zonesTab.waitFor({ timeout: 3000 });
        await zonesTab.click();
        await sleep(1000);

        await captureScreenshot(page, 'zone-editor', { delay: 1500 });
      } catch (e) {
        console.log('  Detection Zones tab not found, skipping');
      }

      // Close modal
      try {
        const closeButton = await page.locator('button').filter({ hasText: /close|cancel/i }).first();
        await closeButton.waitFor({ timeout: 2000 });
        await closeButton.click();
        await sleep(500);
      } catch (e) {
        // Modal might auto-close or not exist
      }
    } catch (e) {
      console.log('  Configure button not found, skipping zone editor');
    }
  } catch (error) {
    console.log(`  ✗ Failed to capture zone editor: ${error.message}`);
  }
}

async function captureThemeSelector(page) {
  console.log('\n=== Capturing Theme Selector ===');
  try {
    await page.goto(`${config.url}/settings.html`, { waitUntil: 'networkidle', timeout: 10000 });
    await sleep(1500);

    // Scroll to theme section
    try {
      const themeSection = await page.locator('h3, h2').filter({ hasText: /theme|appearance/i }).first();
      await themeSection.waitFor({ timeout: 3000 });
      await themeSection.scrollIntoViewIfNeeded();
      await sleep(500);
    } catch (e) {
      console.log('  Theme section not found, capturing full settings page');
    }

    await captureScreenshot(page, 'theme-selector', { delay: 1000 });
  } catch (error) {
    console.log(`  ✗ Failed to capture theme selector: ${error.message}`);
  }
}

async function captureAllThemes(page) {
  console.log('\n=== Capturing All Themes ===');
  
  for (const theme of THEMES) {
    // Light mode
    await setTheme(page, theme, false);
    await page.goto(`${config.url}/index.html`);
    await sleep(2000);
    await captureScreenshot(page, `theme-${theme}-light`, { delay: 1000 });
    
    // Dark mode
    await setTheme(page, theme, true);
    await page.goto(`${config.url}/index.html`);
    await sleep(2000);
    await captureScreenshot(page, `theme-${theme}-dark`, { delay: 1000 });
  }
}

async function main() {
  console.log('LightNVR Screenshot Capture Tool');
  console.log('================================\n');
  console.log('Configuration:');
  console.log(`  URL: ${config.url}`);
  console.log(`  Username: ${config.username}`);
  console.log(`  Output: ${config.outputDir}`);
  console.log(`  Theme: ${config.theme}`);
  console.log(`  Dark Mode: ${config.darkMode}`);
  console.log(`  All Themes: ${config.allThemes}\n`);
  
  const browser = await chromium.launch({ 
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });
  
  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 },
    deviceScaleFactor: 1,
  });
  
  const page = await context.newPage();
  
  try {
    // Login
    await login(page, config.username, config.password);
    
    // Set theme
    if (!config.allThemes) {
      await setTheme(page, config.theme, config.darkMode);
    }
    
    // Capture screenshots
    if (config.allThemes) {
      await captureAllThemes(page);
    } else {
      await captureLiveStreams(page);
      await captureStreamManagement(page);
      await captureRecordingManagement(page);
      await captureSettingsManagement(page);
      await captureSystemInfo(page);
      await captureZoneEditor(page);
      await captureThemeSelector(page);
    }
    
    console.log('\n✓ All screenshots captured successfully!');
    
  } catch (error) {
    console.error('\n✗ Error capturing screenshots:', error);
    process.exit(1);
  } finally {
    await browser.close();
  }
}

main();

