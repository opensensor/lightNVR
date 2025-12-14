#!/usr/bin/env node

/**
 * LightNVR Automated Demo Video Capture
 * 
 * This script uses Playwright to automatically capture demo videos and GIFs
 * of key LightNVR features.
 * 
 * Usage:
 *   npm install --save-dev playwright
 *   node scripts/capture-demos.js [options]
 * 
 * Options:
 *   --url <url>          LightNVR URL (default: http://localhost:8080)
 *   --username <user>    Username (default: admin)
 *   --password <pass>    Password (default: admin)
 *   --output <dir>       Output directory (default: docs/videos)
 *   --demo <name>        Specific demo to capture (zone-creation, theme-switch, detection)
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
  outputDir: 'docs/videos',
  demo: 'all',
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
    case '--demo':
      config.demo = args[++i];
      break;
  }
}

// Ensure output directory exists
if (!fs.existsSync(config.outputDir)) {
  fs.mkdirSync(config.outputDir, { recursive: true });
}

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function login(page, username, password) {
  console.log('Logging in...');
  await page.goto(`${config.url}/login.html`);
  await page.fill('input[name="username"]', username);
  await page.fill('input[name="password"]', password);
  await page.click('button[type="submit"]');
  await page.waitForURL('**/index.html', { timeout: 10000 });
  console.log('Login successful');
}

async function recordVideo(page, name, recordingFn) {
  console.log(`\n=== Recording: ${name} ===`);
  
  // Start recording
  const videoPath = path.join(config.outputDir, `${name}.webm`);
  
  // Playwright records video automatically if configured in context
  // We'll use the page's video() method
  await recordingFn(page);
  
  console.log(`✓ Recorded: ${videoPath}`);
}

async function demoZoneCreation(page) {
  console.log('\n=== Demo: Zone Creation ===');
  
  // Navigate to streams
  await page.goto(`${config.url}/streams.html`);
  await sleep(1500);
  
  // Click configure on first stream
  const configButton = await page.locator('button').filter({ hasText: /configure/i }).first();
  if (!configButton) {
    console.log('No configure button found, skipping zone demo');
    return;
  }
  
  await configButton.click();
  await sleep(1000);
  
  // Click Detection Zones tab
  const zonesTab = await page.locator('button, a').filter({ hasText: /detection zones/i }).first();
  if (zonesTab) {
    await zonesTab.click();
    await sleep(1000);
    
    // Click Add Zone
    const addZoneButton = await page.locator('button').filter({ hasText: /add zone/i }).first();
    if (addZoneButton) {
      await addZoneButton.click();
      await sleep(500);
      
      // Simulate drawing a polygon (if canvas is available)
      const canvas = await page.locator('canvas').first();
      if (canvas) {
        const box = await canvas.boundingBox();
        if (box) {
          // Draw a simple rectangle as polygon
          await page.mouse.click(box.x + 100, box.y + 100);
          await sleep(300);
          await page.mouse.click(box.x + 400, box.y + 100);
          await sleep(300);
          await page.mouse.click(box.x + 400, box.y + 300);
          await sleep(300);
          await page.mouse.click(box.x + 100, box.y + 300);
          await sleep(300);
          await page.mouse.click(box.x + 100, box.y + 100); // Close polygon
          await sleep(500);
        }
      }
      
      // Fill in zone name
      const nameInput = await page.locator('input[placeholder*="zone" i], input[name*="name" i]').first();
      if (nameInput) {
        await nameInput.fill('Driveway');
        await sleep(500);
      }
      
      // Set class filter
      const classInput = await page.locator('input[placeholder*="class" i], input[name*="class" i]').first();
      if (classInput) {
        await classInput.fill('person,car');
        await sleep(500);
      }
      
      // Save zone
      const saveButton = await page.locator('button').filter({ hasText: /save/i }).first();
      if (saveButton) {
        await saveButton.click();
        await sleep(1000);
      }
    }
  }
  
  // Close modal
  const closeButton = await page.locator('button').filter({ hasText: /close|cancel/i }).first();
  if (closeButton) {
    await closeButton.click();
    await sleep(500);
  }
}

async function demoThemeSwitching(page) {
  console.log('\n=== Demo: Theme Switching ===');
  
  // Navigate to settings
  await page.goto(`${config.url}/settings.html`);
  await sleep(1500);
  
  // Scroll to theme section
  const themeSection = await page.locator('h3').filter({ hasText: /color theme/i }).first();
  if (themeSection) {
    await themeSection.scrollIntoViewIfNeeded();
    await sleep(500);
  }
  
  // Click through different themes
  const themes = ['blue', 'emerald', 'purple', 'rose'];
  for (const theme of themes) {
    const themeButton = await page.locator('button').filter({ hasText: new RegExp(theme, 'i') }).first();
    if (themeButton) {
      await themeButton.click();
      await sleep(1000);
    }
  }
  
  // Adjust intensity slider
  const intensitySlider = await page.locator('input[type="range"]').first();
  if (intensitySlider) {
    await intensitySlider.fill('30');
    await sleep(1000);
    await intensitySlider.fill('70');
    await sleep(1000);
    await intensitySlider.fill('50');
    await sleep(500);
  }
  
  // Toggle dark mode
  const darkModeToggle = await page.locator('button').filter({ hasText: /dark mode/i }).first();
  if (darkModeToggle) {
    await darkModeToggle.click();
    await sleep(1500);
    await darkModeToggle.click();
    await sleep(1000);
  }
}

async function demoLiveDetection(page) {
  console.log('\n=== Demo: Live Detection ===');
  
  // Navigate to live view
  await page.goto(`${config.url}/index.html`);
  await sleep(3000); // Wait for streams to load
  
  // Let it run for a bit to show detection overlays
  await sleep(5000);
  
  // Toggle fullscreen on a stream if available
  const videoCell = await page.locator('.video-cell').first();
  if (videoCell) {
    await videoCell.hover();
    await sleep(500);
    
    const fullscreenButton = await page.locator('button[title*="fullscreen" i]').first();
    if (fullscreenButton) {
      await fullscreenButton.click();
      await sleep(2000);
      
      // Exit fullscreen
      await page.keyboard.press('Escape');
      await sleep(1000);
    }
  }
}

async function demoStreamConfiguration(page) {
  console.log('\n=== Demo: Stream Configuration ===');
  
  // Navigate to streams
  await page.goto(`${config.url}/streams.html`);
  await sleep(1500);
  
  // Click Add Stream
  const addButton = await page.locator('button').filter({ hasText: /add stream/i }).first();
  if (addButton) {
    await addButton.click();
    await sleep(1000);
    
    // Fill in stream details
    const nameInput = await page.locator('input[name="name"], input[placeholder*="name" i]').first();
    if (nameInput) {
      await nameInput.fill('Demo Camera');
      await sleep(500);
    }
    
    const urlInput = await page.locator('input[name="url"], input[placeholder*="url" i]').first();
    if (urlInput) {
      await urlInput.fill('rtsp://demo:demo@192.168.1.100:554/stream');
      await sleep(500);
    }
    
    // Test connection
    const testButton = await page.locator('button').filter({ hasText: /test/i }).first();
    if (testButton) {
      await testButton.click();
      await sleep(2000);
    }
    
    // Cancel (don't actually save)
    const cancelButton = await page.locator('button').filter({ hasText: /cancel/i }).first();
    if (cancelButton) {
      await cancelButton.click();
      await sleep(500);
    }
  }
}

async function main() {
  console.log('LightNVR Demo Video Capture Tool');
  console.log('=================================\n');
  console.log('Configuration:');
  console.log(`  URL: ${config.url}`);
  console.log(`  Username: ${config.username}`);
  console.log(`  Output: ${config.outputDir}`);
  console.log(`  Demo: ${config.demo}\n`);
  
  // Allow overriding browser executable (e.g. system Chrome with H264 support)
  const browserExecutablePath = process.env.PLAYWRIGHT_BROWSER_PATH || undefined;
  if (browserExecutablePath) {
    console.log(`Using custom Playwright browser executable: ${browserExecutablePath}`);
  } else {
    console.log('Using default Playwright Chromium executable');
  }

  const browser = await chromium.launch({
    headless: false, // Show browser for video recording
    executablePath: browserExecutablePath,
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--disable-web-security', // Allow cross-origin requests to go2rtc
      '--disable-features=IsolateOrigins,site-per-process', // Further CORS relaxation
      '--use-fake-ui-for-media-stream', // Auto-accept media permissions
      '--use-fake-device-for-media-stream', // Use fake camera/mic
    ]
  });
  
  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 },
    deviceScaleFactor: 1,
    recordVideo: {
      dir: config.outputDir,
      size: { width: 1920, height: 1080 }
    }
  });
  
  const page = await context.newPage();
  
  try {
    // Login
    await login(page, config.username, config.password);
    
    // Run demos
    if (config.demo === 'all' || config.demo === 'zone-creation') {
      await demoZoneCreation(page);
    }
    
    if (config.demo === 'all' || config.demo === 'theme-switch') {
      await demoThemeSwitching(page);
    }
    
    if (config.demo === 'all' || config.demo === 'detection') {
      await demoLiveDetection(page);
    }
    
    if (config.demo === 'all' || config.demo === 'stream-config') {
      await demoStreamConfiguration(page);
    }
    
    console.log('\n✓ All demos recorded successfully!');
    console.log('\nNote: Videos are saved in WebM format.');
    console.log('Convert to MP4 with: ffmpeg -i input.webm -c:v libx264 output.mp4');
    console.log('Convert to GIF with: ffmpeg -i input.webm -vf "fps=15,scale=1280:-1" output.gif');
    
  } catch (error) {
    console.error('\n✗ Error recording demos:', error);
    process.exit(1);
  } finally {
    await context.close();
    await browser.close();
  }
}

main();

