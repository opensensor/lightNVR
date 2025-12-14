#!/usr/bin/env node

/**
 * Capture detection-focused demo screenshots:
 * 1. Zone editor with a zone drawn around the driveway/vehicles
 * 2. WebRTC live view with detection overlays (bounding boxes)
 * 3. Recording playback
 */

const { chromium } = require('playwright');
const path = require('path');

const config = {
  url: process.argv.find((a, i) => process.argv[i-1] === '--url') || 'http://192.168.50.248:8080',
  username: process.argv.find((a, i) => process.argv[i-1] === '--username') || 'admin',
  password: process.argv.find((a, i) => process.argv[i-1] === '--password') || 'admin',
  outputDir: 'docs/images',
  streamName: 'frontdoor-vdb2', // Stream with vehicles
};

const sleep = ms => new Promise(r => setTimeout(r, ms));

async function login(page) {
  console.log('Logging in...');
  await page.goto(`${config.url}/login.html`, { waitUntil: 'networkidle', timeout: 15000 });
  await page.locator('input[name="username"], input[type="text"]').first().fill(config.username);
  await page.locator('input[name="password"], input[type="password"]').first().fill(config.password);
  await page.locator('button[type="submit"]').click();
  await page.waitForURL('**/index.html', { timeout: 10000 });
  console.log('✓ Logged in');
}

async function captureZoneEditorWithVehicleZone(page) {
  console.log('\n=== 1. Zone Editor with Driveway Zone ===');

  await page.goto(`${config.url}/streams.html`, { waitUntil: 'networkidle', timeout: 15000 });
  await sleep(2000);

  // Find the specific stream row by looking for a table row containing the stream name
  // then find the Edit button within that row
  const streamRows = await page.locator('tr').all();
  let editClicked = false;

  for (const row of streamRows) {
    const text = await row.textContent();
    if (text && text.includes(config.streamName)) {
      const editBtn = row.locator('button[title="Edit"]').first();
      await editBtn.click();
      editClicked = true;
      break;
    }
  }

  if (!editClicked) {
    // Fallback: try clicking the second Edit button (first stream is backdoor, second is frontdoor)
    console.log('  Fallback: clicking second Edit button...');
    await page.locator('button[title="Edit"]').nth(1).click();
  }

  await sleep(2000);
  
  // Open Detection Zones section
  const zonesSection = page.locator('button').filter({ hasText: /Detection Zones/i }).first();
  await zonesSection.scrollIntoViewIfNeeded();
  await zonesSection.click();
  await sleep(1500);
  
  // Open Zone Editor
  const configureZonesBtn = page.locator('button').filter({ hasText: /Configure Zones|Edit Zones/i }).first();
  await configureZonesBtn.click();
  await sleep(3000);

  // Wait for zone editor dialog
  const zoneEditorDialog = page.locator('div').filter({ hasText: /Detection Zone Editor/i }).first();
  await zoneEditorDialog.waitFor({ timeout: 10000 });

  // The snapshot is fetched directly from go2rtc at port 1984 (not proxied)
  // Wait for the snapshot image to load - go2rtc may need time to grab a frame
  console.log('  Waiting for go2rtc snapshot to load (direct request to port 1984)...');

  // Wait for the "Snapshot loaded successfully" console message or timeout
  let snapshotLoaded = false;
  for (let i = 0; i < 30; i++) { // 15 seconds max wait
    const loadingIndicator = await zoneEditorDialog.locator('text=Loading stream snapshot').count();
    if (loadingIndicator === 0) {
      snapshotLoaded = true;
      break;
    }
    await sleep(500);
  }

  if (snapshotLoaded) {
    console.log('  ✓ Snapshot loaded from go2rtc');
  } else {
    console.log('  ⚠ Snapshot may not have loaded, continuing anyway');
  }

  // Extra wait to ensure canvas is fully rendered with the snapshot
  await sleep(2000);

  const canvas = zoneEditorDialog.locator('canvas').first();
  let box = null;
  for (let i = 0; i < 20; i++) {
    box = await canvas.boundingBox();
    if (box && box.width > 0 && box.height > 0) break;
    await sleep(500);
  }
  
  if (box) {
    console.log('  Drawing zone around driveway/vehicle area...');
    // Draw a zone in the lower portion of the frame (where vehicles typically are)
    const points = [
      { x: box.width * 0.1, y: box.height * 0.5 },   // Left middle
      { x: box.width * 0.9, y: box.height * 0.5 },   // Right middle  
      { x: box.width * 0.9, y: box.height * 0.95 },  // Right bottom
      { x: box.width * 0.1, y: box.height * 0.95 },  // Left bottom
    ];
    
    for (const pt of points) {
      await canvas.click({ position: { x: pt.x, y: pt.y } });
      await sleep(300);
    }
    
    // Complete the zone
    const completeBtn = zoneEditorDialog.locator('button').filter({ hasText: /Complete Zone/i }).first();
    await completeBtn.click();
    await sleep(1000);
    console.log('  ✓ Zone drawn');
  }
  
  await sleep(1000);
  await page.screenshot({ path: path.join(config.outputDir, 'zone-editor-demo.png') });
  console.log('✓ Saved: zone-editor-demo.png');
  
  // Save and close
  const saveBtn = zoneEditorDialog.locator('button').filter({ hasText: /Save Zones/i }).first();
  await saveBtn.click({ force: true });
  await sleep(1500);
}

async function captureWebRTCWithDetection(page) {
  console.log('\n=== 2. WebRTC Live View with Detection Overlays ===');
  
  await page.goto(`${config.url}/index.html`, { waitUntil: 'networkidle', timeout: 15000 });
  await sleep(3000);
  
  // Wait for WebRTC streams to connect
  console.log('  Waiting for WebRTC streams to connect...');
  for (let i = 0; i < 30; i++) {
    const playing = await page.evaluate(() => {
      const videos = document.querySelectorAll('video');
      return Array.from(videos).filter(v => !v.paused && v.readyState >= 3).length;
    });
    if (playing >= 1) break;
    await sleep(1000);
  }
  
  // Wait a bit more for detection overlays to appear
  console.log('  Waiting for detection overlays...');
  await sleep(5000);
  
  await page.screenshot({ path: path.join(config.outputDir, 'detection-overlay.png') });
  console.log('✓ Saved: detection-overlay.png');
}

async function captureRecordingPlayback(page) {
  console.log('\n=== 3. Recording Playback ===');

  await page.goto(`${config.url}/recordings.html`, { waitUntil: 'networkidle', timeout: 15000 });
  await sleep(3000);

  // Look for the play button in the recordings list - it's typically a button with a play icon
  try {
    // Find all rows that have recordings
    const playButtons = page.locator('button[title="Play"]');
    const count = await playButtons.count();
    console.log(`  Found ${count} play buttons`);

    if (count > 0) {
      // Click the first play button
      await playButtons.first().click();
      console.log('  Clicked play button');
      await sleep(3000);

      // Wait for video modal to appear
      const videoModal = page.locator('div').filter({ has: page.locator('video') }).first();
      await videoModal.waitFor({ timeout: 10000 });
      console.log('  Video modal appeared');

      // Wait for video to load
      const video = page.locator('video').first();
      await video.waitFor({ timeout: 5000 });
      await sleep(2000);

      await page.screenshot({ path: path.join(config.outputDir, 'recording-playback.png') });
      console.log('✓ Saved: recording-playback.png');
      return;
    }
  } catch (e) {
    console.log(`  Video modal approach failed: ${e.message}`);
  }

  // Fallback: just capture the recordings page
  console.log('  Capturing recordings page instead');
  await page.screenshot({ path: path.join(config.outputDir, 'recording-playback.png') });
  console.log('✓ Saved: recording-playback.png (recordings list)');
}

(async () => {
  const browserPath = process.env.PLAYWRIGHT_BROWSER_PATH;
  console.log('Detection Demo Capture\n======================');
  console.log(`URL: ${config.url}`);
  console.log(`Stream: ${config.streamName}`);
  if (browserPath) console.log(`Browser: ${browserPath}`);
  
  const browser = await chromium.launch({
    headless: false,
    executablePath: browserPath || undefined,
    args: ['--no-sandbox', '--disable-setuid-sandbox', '--use-fake-ui-for-media-stream'],
  });
  
  const page = await browser.newPage({ viewport: { width: 1920, height: 1080 } });
  page.on('console', msg => console.log(`  [Browser] ${msg.text()}`));
  
  try {
    await login(page);
    await captureZoneEditorWithVehicleZone(page);
    await captureWebRTCWithDetection(page);
    await captureRecordingPlayback(page);
    console.log('\n✓ All detection demos captured!');
  } catch (err) {
    console.error('Error:', err.message);
  } finally {
    await browser.close();
  }
})();

