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

// go2rtc API is typically mounted under a base path (see include/video/go2rtc/go2rtc_api.h)
// In CI/lightNVR-managed mode we default to /go2rtc.
const GO2RTC_BASE_PATH = (process.env.GO2RTC_BASE_PATH ?? '/go2rtc').replace(/\/+$/, '');
const GO2RTC_HTTP_BASE = `http://localhost:${GO2RTC_API_PORT}${GO2RTC_BASE_PATH}`;

// go2rtc API uses the same basic auth as lightNVR's web config in the test ini.
// If go2rtc auth is disabled in some environments, providing the header is harmless.
const GO2RTC_AUTH_HEADER = 'Basic ' + Buffer.from('admin:admin').toString('base64');

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
  console.log(`lightNVR binary found at: ${LIGHTNVR_BIN}`);

  // Check if config exists
  if (!existsSync(LIGHTNVR_CONFIG)) {
    throw new Error(`lightNVR config not found at ${LIGHTNVR_CONFIG}`);
  }
  console.log(`lightNVR config found at: ${LIGHTNVR_CONFIG}`);

  // Check if go2rtc binary exists (lightNVR will try to start it)
  const go2rtcPath = path.join(PROJECT_ROOT, 'go2rtc', 'go2rtc');
  if (!existsSync(go2rtcPath)) {
    throw new Error(`go2rtc binary not found at ${go2rtcPath}. Build go2rtc first.`);
  }
  console.log(`go2rtc binary found at: ${go2rtcPath}`);

  // Check if web dist exists
  const webDistPath = path.join(PROJECT_ROOT, 'web', 'dist');
  if (!existsSync(webDistPath)) {
    throw new Error(`web/dist not found at ${webDistPath}. Build frontend first.`);
  }
  console.log(`web/dist found at: ${webDistPath}`);

  // Log test directories for verification
  console.log(`Test directories: ${TEST_DIR}`);
  console.log(`  - exists: ${existsSync(TEST_DIR)}`);
  console.log(`  - recordings: ${existsSync(TEST_DIR + '/recordings')}`);
  console.log(`  - go2rtc: ${existsSync(TEST_DIR + '/go2rtc')}`);

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

  // Log the exact command we're running
  console.log(`Starting lightNVR with command: ${LIGHTNVR_BIN} -c ${LIGHTNVR_CONFIG}`);
  console.log(`Working directory: ${PROJECT_ROOT}`);

  // Use 'inherit' for stdio - lightNVR writes to its own log file at /tmp/lightnvr-test/lightnvr.log
  // Using 'pipe' with unref() causes SIGPIPE issues when parent moves on
  lightnvrProcess = spawn(LIGHTNVR_BIN, ['-c', LIGHTNVR_CONFIG], {
    cwd: PROJECT_ROOT,
    stdio: ['ignore', 'inherit', 'inherit'],
    detached: true,
  });

  // Unref the process so the parent can exit without waiting for this child
  lightnvrProcess.unref();

  // Track if process exited during startup
  let processExitedDuringStartup = false;
  let exitCode: number | null = null;
  let exitSignal: string | null = null;

  // Handle process errors
  lightnvrProcess.on('error', (err) => {
    console.error(`lightNVR process error: ${err.message}`);
    console.error(`Error stack: ${err.stack}`);
  });

  // Log if the process exits unexpectedly during startup
  lightnvrProcess.on('exit', (code, signal) => {
    processExitedDuringStartup = true;
    exitCode = code;
    exitSignal = signal;
    if (code !== null) {
      console.error(`lightNVR process exited with code ${code}`);
    } else if (signal !== null) {
      console.error(`lightNVR process was killed with signal ${signal}`);
    }
  });

  // Store PID in a separate file for teardown (not the same as lightNVR's PID file)
  writeFileSync(`${TEST_DIR}/test-lightnvr.pid`, String(lightnvrProcess.pid));

  console.log(`lightNVR started with PID: ${lightnvrProcess.pid}`);

  // Give the process a moment to start before checking if it's still alive
  await sleep(2000);

  // Check if process exited immediately
  if (processExitedDuringStartup) {
    console.error(`lightNVR process exited immediately after spawn!`);
    console.error(`Exit code: ${exitCode}, Signal: ${exitSignal}`);

    // Try to read any logs that might have been written
    const logPath = `${TEST_DIR}/lightnvr.log`;
    if (existsSync(logPath)) {
      const logs = readFileSync(logPath, 'utf8');
      console.error('=== lightNVR log ===\n' + logs);
    } else {
      console.error('No log file was created at ' + logPath);
    }

    throw new Error(`lightNVR process exited immediately with code ${exitCode}`);
  }
  
  // Wait for lightNVR to be ready (with auth)
  const authHeader = 'Basic ' + Buffer.from('admin:admin').toString('base64');
  const ready = await waitForService(
    `http://localhost:${LIGHTNVR_PORT}/api/system`,
    45000,  // 45 seconds - increased to allow for go2rtc startup
    authHeader
  );

  if (!ready) {
    // Check if the process is still running
    const isRunning = lightnvrProcess && !lightnvrProcess.killed && lightnvrProcess.exitCode === null;
    console.error(`lightNVR process status: running=${isRunning}, killed=${lightnvrProcess?.killed}, exitCode=${lightnvrProcess?.exitCode}`);

    // Try to read main log file (lightNVR writes to this)
    const logPath = `${TEST_DIR}/lightnvr.log`;
    if (existsSync(logPath)) {
      const logs = readFileSync(logPath, 'utf8');
      const lastLines = logs.split('\n').slice(-50).join('\n');
      console.error('=== Last 50 lines of lightNVR log ===\n' + lastLines);
    } else {
      console.error('No log file found at ' + logPath);
    }

    // List what's in the test directory
    const { readdirSync, statSync } = require('fs');
    try {
      console.error('=== Contents of ' + TEST_DIR + ' ===');
      const files = readdirSync(TEST_DIR);
      for (const f of files) {
        const stat = statSync(`${TEST_DIR}/${f}`);
        console.error(`  ${f} (${stat.isDirectory() ? 'dir' : stat.size + ' bytes'})`);
      }
    } catch (e) {
      console.error('Failed to list test directory: ' + e);
    }

    throw new Error('lightNVR failed to start within 45 seconds');
  }
  console.log('lightNVR is ready');
}

async function waitForGo2rtc(): Promise<void> {
  console.log('Waiting for go2rtc on port ' + GO2RTC_API_PORT + '...');

  // LightNVR needs time to start go2rtc, so we use a longer timeout
  const ready = await waitForService(
    `${GO2RTC_HTTP_BASE}/api/streams`,
    60000,  // 60 seconds timeout - go2rtc startup can take a while
    GO2RTC_AUTH_HEADER
  );

  if (!ready) {
    // Try to get more diagnostic information
    console.error('go2rtc failed to start. Checking lightNVR logs...');
    const logPath = '/tmp/lightnvr-test/lightnvr.log';
    if (existsSync(logPath)) {
      const logs = readFileSync(logPath, 'utf8');
      const lastLines = logs.split('\n').slice(-50).join('\n');
      console.error('Last 50 lines of lightNVR log:\n' + lastLines);
    }

    // Check if go2rtc binary exists
    const go2rtcBinary = path.join(PROJECT_ROOT, 'go2rtc/go2rtc');
    console.error('go2rtc binary exists at ' + go2rtcBinary + ': ' + existsSync(go2rtcBinary));

    throw new Error('go2rtc failed to start within 60 seconds');
  }
  console.log('go2rtc is ready');
}



async function registerTestStreamsWithGo2rtc(): Promise<void> {
  console.log('Registering test streams with go2rtc...');

  for (const stream of TEST_STREAMS) {
    try {
      const encodedSource = encodeURIComponent(stream.source);
      const url = `${GO2RTC_HTTP_BASE}/api/streams?name=${stream.name}&src=${encodedSource}`;

      const response = await fetch(url, {
        method: 'PUT',
        headers: {
          'Authorization': GO2RTC_AUTH_HEADER,
        },
      });
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
      // Use go2rtc's RTSP endpoint - the streams are already registered with go2rtc
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

  // Register virtual streams with go2rtc first
  await registerTestStreamsWithGo2rtc();

  // Add streams to lightNVR using go2rtc RTSP endpoints
  await addStreamsToLightNVR();

  // Give streams time to initialize
  await sleep(3000);

  console.log('\n✓ Test environment ready\n');
}

export default globalSetup;

