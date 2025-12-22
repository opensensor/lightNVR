/**
 * Global Teardown for LightNVR Integration Tests
 * 
 * Cleans up after all tests:
 * 1. Stops lightNVR (which stops go2rtc)
 * 2. Optionally cleans up test directories
 */

import { existsSync, readFileSync, rmSync } from 'fs';
import { execSync } from 'child_process';

const TEST_DIR = '/tmp/lightnvr-test';

async function stopLightNVR(): Promise<void> {
  console.log('Stopping lightNVR...');

  // Try our test PID file first, then fall back to lightNVR's PID file
  const testPidFile = `${TEST_DIR}/test-lightnvr.pid`;
  const lightnvrPidFile = `${TEST_DIR}/lightnvr.pid`;
  const pidFile = existsSync(testPidFile) ? testPidFile : lightnvrPidFile;

  if (existsSync(pidFile)) {
    try {
      const pid = parseInt(readFileSync(pidFile, 'utf-8').trim());

      // Try graceful shutdown first
      try {
        process.kill(pid, 'SIGTERM');
        console.log(`Sent SIGTERM to PID ${pid}`);

        // Wait a bit for graceful shutdown
        await new Promise(resolve => setTimeout(resolve, 2000));

        // Check if still running and force kill
        try {
          process.kill(pid, 0); // Check if process exists
          process.kill(pid, 'SIGKILL');
          console.log(`Sent SIGKILL to PID ${pid}`);
        } catch (e) {
          // Process already dead
        }
      } catch (e) {
        console.log(`Process ${pid} already stopped`);
      }

      // Remove PID files
      rmSync(testPidFile, { force: true });
      rmSync(lightnvrPidFile, { force: true });
    } catch (e) {
      console.log(`Error stopping lightNVR: ${e}`);
    }
  } else {
    console.log('No PID file found, lightNVR may not have been started by tests');
  }
}

async function cleanupTestDirectories(): Promise<void> {
  // Only cleanup if not in debug mode
  if (process.env.DEBUG || process.env.KEEP_TEST_DATA) {
    console.log('Keeping test directories (DEBUG or KEEP_TEST_DATA is set)');
    return;
  }
  
  console.log('Cleaning up test directories...');
  if (existsSync(TEST_DIR)) {
    try {
      rmSync(TEST_DIR, { recursive: true, force: true });
      console.log('Test directories removed');
    } catch (e) {
      console.log(`Error cleaning up: ${e}`);
    }
  }
}

async function globalTeardown(): Promise<void> {
  console.log('\n========================================');
  console.log('LightNVR Integration Test Teardown');
  console.log('========================================\n');
  
  await stopLightNVR();
  await cleanupTestDirectories();
  
  console.log('\n✓ Cleanup complete\n');
}

export default globalTeardown;

