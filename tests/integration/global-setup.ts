/**
 * Global Setup for LightNVR Integration Tests
 * 
 * This runs once before all tests to start the test environment:
 * 1. Sets up test directories
 * 2. Starts lightNVR with test configuration
 * 3. Waits for services to be ready
 * 4. Registers test streams with go2rtc
 */

import { spawn, ChildProcess, execSync } from 'child_process';
import { existsSync, mkdirSync, writeFileSync, readFileSync, rmSync } from 'fs';
import path from 'path';

const PROJECT_ROOT = path.resolve(__dirname, '../..');
const TEST_DIR = '/tmp/lightnvr-test';
const TEST_RESULTS_DIR = path.join(PROJECT_ROOT, 'test-results');
const LIGHTNVR_BIN = path.join(PROJECT_ROOT, 'build/bin/lightnvr');
const LIGHTNVR_CONFIG = path.join(PROJECT_ROOT, 'config/lightnvr-test.ini');
const LIGHTNVR_PORT = 18080;
const GO2RTC_API_PORT = 11984;
const GO2RTC_RTSP_PORT = 18554;

// Test streams to register with go2rtc and lightNVR
const TEST_STREAMS = [
  { name: 'test_pattern', source: 'ffmpeg:virtual?video&size=720#video=h264', width: 1280, height: 720 },
  { name: 'test_colorbars', source: 'ffmpeg:virtual?video=smptebars&size=720#video=h264', width: 1280, height: 720 },
  { name: 'test_red', source: 'ffmpeg:virtual?video=color&color=red&size=640x480#video=h264', width: 640, height: 480 },
  { name: 'test_mandelbrot', source: 'ffmpeg:virtual?video=mandelbrot&size=640x480#video=h264', width: 640, height: 480 },
];

let lightnvrProcess: ChildProcess | null = null;

async function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function waitForService(url: string, timeoutMs: number = 30000, auth?: string): Promise<boolean> {
  const startTime = Date.now();
  const headers: Record<string, string> = {};
  if (auth) {
    headers['Authorization'] = auth;
  }

  while (Date.now() - startTime < timeoutMs) {
    try {
      const response = await fetch(url, { headers });
      // 200 = success, 401 = auth required but service is up
      if (response.ok || response.status === 401) {
        return true;
      }
    } catch (e) {
      // Service not ready yet
    }
    await sleep(1000);
  }
  return false;
}

async function setupTestDirectories(): Promise<void> {
  console.log('Setting up test directories...');

  // Clean up any existing test directory to ensure fresh state
  if (existsSync(TEST_DIR)) {
    console.log('Cleaning up existing test directory...');
    rmSync(TEST_DIR, { recursive: true, force: true });
  }

  // Ensure test-results directory exists for screenshots
  if (!existsSync(TEST_RESULTS_DIR)) {
    mkdirSync(TEST_RESULTS_DIR, { recursive: true });
    console.log('Created test-results directory');
  }

  const dirs = [
    TEST_DIR,
    `${TEST_DIR}/recordings`,
    `${TEST_DIR}/recordings/mp4`,
    `${TEST_DIR}/recordings/hls`,
    `${TEST_DIR}/models`,
    `${TEST_DIR}/go2rtc`,
  ];

  for (const dir of dirs) {
    mkdirSync(dir, { recursive: true });
  }
  console.log('Test directories created');
}

async function startLightNVR(): Promise<void> {
  console.log('Starting lightNVR...');
  
  // Check if binary exists
  if (!existsSync(LIGHTNVR_BIN)) {
    throw new Error(`lightNVR binary not found at ${LIGHTNVR_BIN}. Run ./scripts/build.sh first.`);
  }
  
  // Check if already running
  try {
    const response = await fetch(`http://localhost:${LIGHTNVR_PORT}/api/system`, {
      headers: { 'Authorization': 'Basic ' + Buffer.from('admin:admin').toString('base64') }
    });
    if (response.ok) {
      console.log('lightNVR already running');
      return;
    }
  } catch (e) {
    // Not running, start it
  }
  
  // Remove any stale PID file before starting
  const pidFile = `${TEST_DIR}/lightnvr.pid`;
  if (existsSync(pidFile)) {
    rmSync(pidFile);
  }

  lightnvrProcess = spawn(LIGHTNVR_BIN, ['-c', LIGHTNVR_CONFIG], {
    cwd: PROJECT_ROOT,
    stdio: ['ignore', 'pipe', 'pipe'],
    detached: true,
  });

  // Store PID in a separate file for teardown (not the same as lightNVR's PID file)
  writeFileSync(`${TEST_DIR}/test-lightnvr.pid`, String(lightnvrProcess.pid));
  
  // Log output for debugging
  lightnvrProcess.stdout?.on('data', (data) => {
    if (process.env.DEBUG) console.log(`[lightnvr] ${data}`);
  });
  lightnvrProcess.stderr?.on('data', (data) => {
    if (process.env.DEBUG) console.error(`[lightnvr] ${data}`);
  });
  
  console.log(`lightNVR started with PID: ${lightnvrProcess.pid}`);
  
  // Wait for lightNVR to be ready (with auth)
  const authHeader = 'Basic ' + Buffer.from('admin:admin').toString('base64');
  const ready = await waitForService(
    `http://localhost:${LIGHTNVR_PORT}/api/system`,
    30000,
    authHeader
  );
  
  if (!ready) {
    throw new Error('lightNVR failed to start within 30 seconds');
  }
  console.log('lightNVR is ready');
}

async function waitForGo2rtc(): Promise<void> {
  console.log('Waiting for go2rtc...');
  const ready = await waitForService(
    `http://localhost:${GO2RTC_API_PORT}/api/streams`,
    30000
  );
  
  if (!ready) {
    throw new Error('go2rtc failed to start within 30 seconds');
  }
  console.log('go2rtc is ready');
}

async function registerTestStreamsWithGo2rtc(): Promise<void> {
  console.log('Registering test streams with go2rtc...');

  for (const stream of TEST_STREAMS) {
    try {
      const encodedSource = encodeURIComponent(stream.source);
      const url = `http://localhost:${GO2RTC_API_PORT}/api/streams?name=${stream.name}&src=${encodedSource}`;

      const response = await fetch(url, { method: 'PUT' });
      if (response.ok) {
        console.log(`  ✓ Registered with go2rtc: ${stream.name}`);
      } else {
        console.log(`  ✗ Failed to register with go2rtc: ${stream.name} (${response.status})`);
      }
    } catch (e) {
      console.log(`  ✗ Error registering with go2rtc: ${stream.name} - ${e}`);
    }
  }
}

async function addStreamsToLightNVR(): Promise<void> {
  console.log('Adding test streams to lightNVR...');

  const authHeader = 'Basic ' + Buffer.from('admin:admin').toString('base64');

  for (const stream of TEST_STREAMS) {
    try {
      // Stream URL points to go2rtc RTSP endpoint
      const streamUrl = `rtsp://localhost:${GO2RTC_RTSP_PORT}/${stream.name}`;

      const streamConfig = {
        name: stream.name,
        url: streamUrl,
        enabled: true,
        streaming_enabled: true,
        width: stream.width,
        height: stream.height,
        fps: 15,
        codec: 'h264',
        priority: 5,
        record: false, // Don't record test streams by default
      };

      const response = await fetch(`http://localhost:${LIGHTNVR_PORT}/api/streams`, {
        method: 'POST',
        headers: {
          'Authorization': authHeader,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(streamConfig),
      });

      if (response.ok || response.status === 201) {
        console.log(`  ✓ Added to lightNVR: ${stream.name}`);
      } else if (response.status === 409) {
        console.log(`  ⚠ Already exists in lightNVR: ${stream.name}`);
      } else {
        const text = await response.text();
        console.log(`  ✗ Failed to add to lightNVR: ${stream.name} (${response.status}: ${text})`);
      }
    } catch (e) {
      console.log(`  ✗ Error adding to lightNVR: ${stream.name} - ${e}`);
    }
  }
}

async function globalSetup(): Promise<void> {
  console.log('\n========================================');
  console.log('LightNVR Integration Test Setup');
  console.log('========================================\n');

  // Create test-results directory for screenshots
  const testResultsDir = path.join(PROJECT_ROOT, 'test-results');
  if (!existsSync(testResultsDir)) {
    mkdirSync(testResultsDir, { recursive: true });
  }

  await setupTestDirectories();
  await startLightNVR();
  await waitForGo2rtc();
  await registerTestStreams();

  console.log('\n✓ Test environment ready\n');
}

export default globalSetup;

