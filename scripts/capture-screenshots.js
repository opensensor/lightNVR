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
    await page.goto(`${config.url}/settings.html`, { waitUntil: 'domcontentloaded', timeout: 10000 });
    await sleep(3000); // Wait for Preact components to render

    // Try to set dark mode (optional, may not exist)
    try {
      // Wait for the Appearance section to load
      await page.waitForSelector('h3:has-text("Appearance")', { timeout: 5000 });
      console.log('  Found Appearance section');

      // Find the dark mode card (has border and rounded corners)
      const darkModeCard = await page.locator('div.bg-card.rounded-lg.border').filter({ hasText: /Dark Mode/i }).first();
      await darkModeCard.waitFor({ timeout: 5000 });
      console.log('  Found Dark Mode card');

      // Get the toggle button within the dark mode card (it's a toggle switch)
      // The button has specific styling for the toggle switch
      const toggleButton = await darkModeCard.locator('button.relative.inline-flex').first();
      await toggleButton.waitFor({ timeout: 3000 });

      // Check current state by looking at the description text
      const descText = await darkModeCard.locator('p.text-sm').first().textContent();
      const isDark = descText.includes('Switch to light mode');
      console.log(`  Current mode: ${isDark ? 'dark' : 'light'}, target: ${darkMode ? 'dark' : 'light'}`);

      if (darkMode !== isDark) {
        console.log('  Toggling dark mode...');
        await toggleButton.click();
        await sleep(1000);
      } else {
        console.log('  Dark mode already set correctly');
      }
    } catch (e) {
      console.log(`  Dark mode toggle not found: ${e.message}`);
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
      console.log(`  Looking for theme: ${themeName}`);

      const themeButton = await page.locator('button').filter({ hasText: themeName }).first();
      await themeButton.waitFor({ timeout: 5000 });
      console.log('  Found theme button, clicking...');
      await themeButton.click();
      await sleep(1000);
      console.log(`  ✓ Theme "${themeName}" applied`);
    } catch (e) {
      console.log(`  Theme button for "${theme}" not found: ${e.message}`);
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
    // Don't wait for networkidle - streams continuously load data
    await page.goto(`${config.url}/index.html`, { waitUntil: 'domcontentloaded', timeout: 10000 });

    // Wait for streams to load and start playing
    console.log('  Waiting for video streams to load...');
    await sleep(5000); // Give more time for initial page load

    // Try to wait for video elements to be present
    try {
      await page.waitForSelector('video, canvas, img[alt*="stream"]', { timeout: 5000 });
      console.log('  Video elements detected');

      // Wait for actual video frames to be rendered (check if video is actually playing)
      console.log('  Waiting for WebRTC streams to connect and display frames...');

      // Try to ensure videos are playing (in case autoplay is blocked)
      await page.evaluate(() => {
        const videos = document.querySelectorAll('video');
        videos.forEach(v => {
          if (v.paused) {
            v.play().catch(e => console.log('Play failed:', e));
          }
        });
      });

      // WebRTC can take 20-60 seconds to establish connection
      // Check periodically for video to start playing
      console.log('  Waiting for WebRTC streams to connect (checking every 5s)...');
      let maxAttempts = 12; // 12 attempts * 5 seconds = 60 seconds max
      let attempt = 0;
      let videosPlaying = { playing: 0, total: 0 };

      while (attempt < maxAttempts) {
        await sleep(5000); // Wait 5 seconds between checks
        attempt++;

        videosPlaying = await page.evaluate(() => {
          const videos = document.querySelectorAll('video');
          let playingCount = 0;
          videos.forEach(v => {
            if (v.videoWidth > 0 && v.videoHeight > 0 && v.readyState >= 2) {
              playingCount++;
            }
          });
          return { total: videos.length, playing: playingCount };
        });

        console.log(`  Attempt ${attempt}/${maxAttempts}: ${videosPlaying.playing}/${videosPlaying.total} videos playing`);

        // If at least one video is playing, wait a bit more for others and break
        if (videosPlaying.playing > 0) {
          console.log('  At least one video playing, waiting for others...');
          await sleep(10000); // Wait 10 more seconds for other streams
          break;
        }
      }

      if (videosPlaying.playing === 0) {
        console.log('  ⚠ Warning: No videos playing after 60 seconds, capturing anyway');
        console.log('  (WebRTC may have failed - check server logs)');
      } else {
        console.log(`  ✓ ${videosPlaying.playing} video(s) playing successfully`);
        await sleep(3000); // Final wait for stability
      }
    } catch (e) {
      console.log(`  Error checking video status: ${e.message}`);
      await sleep(10000); // Fallback wait
    }

    await captureScreenshot(page, 'live-streams', { delay: 1000 });

    // Also capture HLS view if available
    try {
      console.log('  Checking for HLS view...');
      await page.goto(`${config.url}/hls.html`, { waitUntil: 'domcontentloaded', timeout: 10000 });
      await sleep(3000);

      // Wait for HLS video to load
      await page.waitForSelector('video', { timeout: 5000 });
      console.log('  HLS video elements detected');

      // Wait for HLS video to actually have content
      console.log('  Waiting for HLS streams to buffer and play...');
      await sleep(12000); // 12 seconds for HLS to buffer

      // Check if videos are playing
      const hlsVideosPlaying = await page.evaluate(() => {
        const videos = document.querySelectorAll('video');
        let playingCount = 0;
        videos.forEach(v => {
          if (v.videoWidth > 0 && v.videoHeight > 0 && !v.paused) {
            playingCount++;
          }
        });
        return { total: videos.length, playing: playingCount };
      });

      console.log(`  Found ${hlsVideosPlaying.playing}/${hlsVideosPlaying.total} HLS videos playing`);

      if (hlsVideosPlaying.playing > 0) {
        console.log('  HLS videos playing, waiting for stability...');
        await sleep(3000);
      } else {
        console.log('  HLS videos not yet playing, waiting more...');
        await sleep(5000);
      }

      await captureScreenshot(page, 'live-streams-hls', { delay: 1000 });
    } catch (e) {
      console.log(`  HLS view error: ${e.message}`);
    }
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
    await sleep(2000);

    // Scroll down past the theme customizer to show actual settings
    try {
      // Find a settings section below appearance (like Storage or Detection)
      const settingsSection = await page.locator('h3').filter({ hasText: /(Storage|Detection|Recording|System)/i }).first();
      await settingsSection.waitFor({ timeout: 3000 });
      await settingsSection.scrollIntoViewIfNeeded();
      await sleep(500);
      console.log('  Scrolled to settings section');
    } catch (e) {
      console.log('  Could not find settings section to scroll to, capturing current view');
    }

    await captureScreenshot(page, 'settings-management', { delay: 1000 });
  } catch (error) {
    console.log(`  ✗ Failed to capture settings management: ${error.message}`);
  }
}

async function captureSystemInfo(page) {
  console.log('\n=== Capturing System Information ===');
  try {
    // System page may have continuous updates, use domcontentloaded
    await page.goto(`${config.url}/system.html`, { waitUntil: 'domcontentloaded', timeout: 10000 });
    await sleep(2000); // Wait for system stats to load

    await captureScreenshot(page, 'system-info', { delay: 1500 });
  } catch (error) {
    console.log(`  ✗ Failed to capture system info: ${error.message}`);
  }
}

async function captureStreamConfiguration(page) {
  console.log('\n=== Capturing Stream Configuration ===');
  try {
    await page.goto(`${config.url}/streams.html`, { waitUntil: 'networkidle', timeout: 10000 });
    await sleep(2000);

    // Try to open stream configuration modal
    try {
      // Look for edit button (has title="Edit" attribute)
      const configButton = await page.locator('button[title="Edit"]').first();
      await configButton.waitFor({ timeout: 5000 });
      console.log('  Opening stream configuration...');
      await configButton.click();
      await sleep(2000);

      // Capture the modal with Basic Settings expanded (default)
      try {
        console.log('  Capturing Basic Settings section...');
        await captureScreenshot(page, 'stream-config-basic', { delay: 1000 });
      } catch (e) {
        console.log('  Could not capture basic settings');
      }

      // Try to expand and capture Recording Settings section
      try {
        const recordingSection = await page.locator('button').filter({ hasText: /^Recording Settings$/i }).first();
        await recordingSection.waitFor({ timeout: 3000 });
        console.log('  Opening Recording Settings section...');
        await recordingSection.click();
        await sleep(1000);

        await captureScreenshot(page, 'stream-config-recording', { delay: 1000 });
      } catch (e) {
        console.log('  Recording Settings section not found');
      }

      // Try to expand and capture AI Detection Recording section
      let detectionWasEnabled = false;
      let needToReopenModal = false;
      try {
        const detectionSection = await page.locator('button').filter({ hasText: /AI Detection Recording/i }).first();
        await detectionSection.waitFor({ timeout: 3000 });
        console.log('  Opening AI Detection Recording section...');
        await detectionSection.click();
        await sleep(1000);

        // Check if detection is enabled, if not enable it to show the zones section
        try {
          const detectionCheckbox = await page.locator('input#stream-detection-enabled').first();
          const isChecked = await detectionCheckbox.isChecked();
          detectionWasEnabled = isChecked;

          if (!isChecked) {
            console.log('  Detection not enabled, enabling it to showcase zones feature...');
            await detectionCheckbox.click();
            await sleep(1500);
            needToReopenModal = true; // Need to save and reopen to see zones section
          } else {
            console.log('  Detection already enabled');
          }
        } catch (e) {
          console.log('  Could not check/enable detection checkbox');
        }

        // Scroll down within the section to show all detection settings
        try {
          const detectionThresholdLabel = await page.locator('label').filter({ hasText: /Detection Threshold|Confidence/i }).first();
          await detectionThresholdLabel.scrollIntoViewIfNeeded();
          await sleep(500);
          console.log('  Scrolled to show detection threshold settings');
        } catch (e) {
          console.log('  Could not scroll to detection settings');
        }

        await captureScreenshot(page, 'stream-config-detection', { delay: 1000 });
      } catch (e) {
        console.log('  AI Detection Recording section not found');
      }

      // If we just enabled detection, save and reopen the modal to see the Detection Zones section
      if (needToReopenModal) {
        console.log('  Saving changes and reopening modal to show Detection Zones section...');
        try {
          // Scroll to the bottom to make sure the Update button is visible
          const modal = await page.locator('div.fixed.inset-0').filter({ has: page.locator('h3').filter({ hasText: /Edit Stream|Add Stream/i }) }).first();

          // Scroll modal content to bottom
          await page.evaluate(() => {
            const modalContent = document.querySelector('div.overflow-y-auto');
            if (modalContent) {
              modalContent.scrollTop = modalContent.scrollHeight;
            }
          });
          await sleep(500);

          const saveButton = await modal.locator('button').filter({ hasText: /Update Stream|Add Stream/i }).last(); // last() to get the one in footer, not header
          await saveButton.waitFor({ timeout: 3000 });
          await saveButton.scrollIntoViewIfNeeded();
          await sleep(500);
          await saveButton.click();
          await sleep(4000); // Wait for save to complete and modal to close

          // Reopen the modal
          const configButton = await page.locator('button[title="Edit"]').first();
          await configButton.waitFor({ timeout: 5000 });
          await configButton.click();
          await sleep(2500);
          console.log('  Modal reopened, looking for Detection Zones section...');
        } catch (e) {
          console.log(`  Could not save and reopen modal: ${e.message}`);
        }
      }

      // Scroll down in the modal to find Detection Zones section
      console.log('  Scrolling down in modal to find Detection Zones section...');
      await page.evaluate(() => {
        const modalContent = document.querySelector('div.overflow-y-auto');
        if (modalContent) {
          modalContent.scrollTop = modalContent.scrollHeight;
        }
      });
      await sleep(1000);

      // Debug: Take a screenshot of the modal scrolled down to see what sections are available
      await captureScreenshot(page, 'stream-config-modal-bottom', { delay: 500 });
      console.log('  Captured modal bottom view for debugging');

      // Try to expand and capture Detection Zones section (only if detection enabled)
      try {
        // Match any button that contains "Detection Zones" text. The button also
        // includes a small "Optional" badge, so avoid strict ^...$ matching.
        const zonesSection = await page.locator('button').filter({ hasText: /Detection Zones/i }).first();
        await zonesSection.waitFor({ timeout: 5000 });
        console.log('  Opening Detection Zones section...');
        await zonesSection.scrollIntoViewIfNeeded();
        await sleep(500);
        await zonesSection.click();
        await sleep(1500);

        await captureScreenshot(page, 'stream-config-zones', { delay: 1000 });

        // Try to open zone editor to showcase the feature
        try {
          console.log('  Attempting to open zone editor...');
          const configureZonesButton = await page
            .locator('button')
            .filter({ hasText: /Configure Zones|Edit Zones/i })
            .first();
          await configureZonesButton.waitFor({ timeout: 3000 });
          await configureZonesButton.click();
          await sleep(2500);

          // Wait for zone editor to load
          try {
            // Scope to the Detection Zone Editor dialog to avoid picking up
            // any other canvases that might exist on the page.
            const zoneEditorDialog = page
              .locator('div')
              .filter({ hasText: /Detection Zone Editor/i })
              .first();

            // Wait for the editor container to appear
            await zoneEditorDialog.waitFor({ timeout: 5000 });
            console.log('  Zone editor opened, waiting for it to fully load...');

            const canvas = zoneEditorDialog.locator('canvas').first();

            // Wait for the canvas to have a non-zero bounding box so
            // clicks actually hit the drawing area. The snapshot load
            // path controls when the canvas is shown.
            let box = null;
            for (let i = 0; i < 20; i++) {
              box = await canvas.boundingBox();
              if (box && box.width > 0 && box.height > 0) break;
              await sleep(500);
            }

            if (!box) {
              console.log('  Zone editor canvas never became ready for drawing');
            } else {
              console.log('  Drawing a sample detection zone for documentation...');

              // Define points relative to the canvas so the clicks are
              // guaranteed to hit the drawing surface even if there are
              // other overlays on the page.
              const points = [
                { x: box.width * 0.3, y: box.height * 0.3 },
                { x: box.width * 0.7, y: box.height * 0.3 },
                { x: box.width * 0.7, y: box.height * 0.7 },
                { x: box.width * 0.3, y: box.height * 0.7 },
              ];

              for (const point of points) {
                await canvas.click({ position: { x: point.x, y: point.y } });
                await sleep(250);
              }

              // Complete the polygon so it becomes a saved zone in the
              // editor UI.
              try {
                const completeButton = zoneEditorDialog
                  .locator('button')
                  .filter({ hasText: /Complete Zone/i })
                  .first();
                await completeButton.waitFor({ timeout: 3000 });
                await completeButton.click();
                await sleep(1000);
              } catch (e) {
                console.log(`  Could not click Complete Zone button: ${e.message}`);
              }
            }

            // Give the UI a moment to render the new zone, then capture
            // the screenshot of the editor with the zone visible.
            await sleep(1000);
            await captureScreenshot(page, 'zone-editor', { delay: 1000 });

            // If there are existing zones, capture that
            // Otherwise log that we're showing the interface for
            // creating zones.
            try {
              const zonesList = await zoneEditorDialog
                .locator('div, p')
                .filter({ hasText: /zone\(s\)|No zones/i })
                .first();
              const zonesText = await zonesList.textContent();

              if (zonesText.includes('No zones')) {
                console.log('  No zones configured, showing empty zone editor');
              } else {
                console.log('  Zones are configured, showing zone editor with zones');
              }
            } catch (e) {
              console.log('  Could not determine zone status');
            }

            // Save and close zone editor so the configuration modal
            // reflects the new zone count. Use force to avoid issues
            // with overlay hit testing in headless mode.
            try {
              const saveZonesButton = zoneEditorDialog
                .locator('button')
                .filter({ hasText: /Save Zones/i })
                .first();
              await saveZonesButton.waitFor({ timeout: 3000 });
              await saveZonesButton.click({ force: true });
              await sleep(1000);
            } catch (e) {
              console.log(`  Failed to save/close zone editor: ${e.message}`);
            }
          } catch (e) {
            console.log(`  Zone editor canvas not found: ${e.message}`);
          }
        } catch (e) {
          console.log(`  Could not open zone editor: ${e.message}`);
        }
      } catch (e) {
        console.log(`  Detection Zones section not found: ${e.message}`);
      }

      // Close modal
      try {
        const closeButton = await page.locator('button').filter({ hasText: /close|cancel|×/i }).first();
        await closeButton.waitFor({ timeout: 2000 });
        await closeButton.click();
        await sleep(500);
      } catch (e) {
        // Try pressing Escape
        await page.keyboard.press('Escape');
        await sleep(500);
      }
    } catch (e) {
      console.log('  Configure button not found, skipping stream configuration');
    }
  } catch (error) {
    console.log(`  ✗ Failed to capture stream configuration: ${error.message}`);
  }
}

async function captureThemeSelector(page) {
  console.log('\n=== Capturing Theme Selector ===');
  try {
    await page.goto(`${config.url}/settings.html`, { waitUntil: 'networkidle', timeout: 10000 });
    await sleep(2000); // Wait for Preact to render

    // Scroll to appearance section
    try {
      const appearanceSection = await page.locator('h3').filter({ hasText: /Appearance/i }).first();
      await appearanceSection.waitFor({ timeout: 5000 });
      await appearanceSection.scrollIntoViewIfNeeded();
      await sleep(500);
    } catch (e) {
      console.log('  Appearance section not found, capturing full settings page');
    }

    // Capture default theme selector
    await captureScreenshot(page, 'theme-selector', { delay: 1000 });

    // Capture with different themes selected to show variety
    const themes = ['blue', 'emerald', 'purple', 'rose'];
    const themeNames = {
      'blue': 'Ocean Blue',
      'emerald': 'Forest Green',
      'purple': 'Royal Purple',
      'rose': 'Sunset Rose'
    };

    for (const theme of themes) {
      try {
        console.log(`  Capturing theme: ${theme}`);
        const themeName = themeNames[theme];
        const themeButton = await page.locator('button').filter({ hasText: themeName }).first();
        await themeButton.waitFor({ timeout: 3000 });
        await themeButton.click();
        await sleep(1500); // Wait for theme to apply

        await captureScreenshot(page, `theme-${theme}`, { delay: 500 });
      } catch (e) {
        console.log(`  Could not capture theme ${theme}: ${e.message}`);
      }
    }

    // Capture dark mode variant
    try {
      console.log('  Capturing dark mode theme selector');
      const darkModeCard = await page.locator('div.bg-card.rounded-lg.border').filter({ hasText: /Dark Mode/i }).first();
      await darkModeCard.waitFor({ timeout: 3000 });
      const toggleButton = await darkModeCard.locator('button.relative.inline-flex').first();
      await toggleButton.waitFor({ timeout: 3000 });
      await toggleButton.click();
      await sleep(1500);

      await captureScreenshot(page, 'theme-selector-dark', { delay: 1000 });
    } catch (e) {
      console.log(`  Could not capture dark mode: ${e.message}`);
    }
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
  
  // Allow overriding browser executable (e.g. system Chrome with H264 support)
  const browserExecutablePath = process.env.PLAYWRIGHT_BROWSER_PATH || undefined;
  if (browserExecutablePath) {
    console.log(`Using custom Playwright browser executable: ${browserExecutablePath}`);
  } else {
    console.log('Using default Playwright Chromium executable');
  }

  const browser = await chromium.launch({
    headless: false, // Use headed mode for better WebRTC support
    executablePath: browserExecutablePath,
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--disable-web-security', // Allow cross-origin requests to go2rtc
      '--disable-features=IsolateOrigins,site-per-process', // Further CORS relaxation
      '--use-fake-ui-for-media-stream', // Auto-accept media permissions
      '--use-fake-device-for-media-stream', // Use fake camera/mic
      '--enable-features=NetworkService,NetworkServiceInProcess'
    ]
  });

  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 },
    deviceScaleFactor: 1,
    permissions: ['camera', 'microphone'], // Grant media permissions
  });
  
  const page = await context.newPage();

  // Listen to console messages for debugging (filtered)
  page.on('console', msg => {
    const text = msg.text();
    // Only log WebRTC/video related messages
    if (text.toLowerCase().includes('webrtc') ||
        text.toLowerCase().includes('ice connection') ||
        text.toLowerCase().includes('video') && (text.includes('play') || text.includes('load') || text.includes('metadata'))) {
      console.log(`  [Browser] ${text}`);
    }
  });

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
      await captureStreamConfiguration(page);
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

