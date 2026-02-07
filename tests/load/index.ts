#!/usr/bin/env node
/**
 * lightNVR Load Test Harness — Entry Point
 *
 * Usage:
 *   node --experimental-strip-types tests/load/index.ts [options]
 *
 * Run with --help for full usage information.
 */

import { parseArgs } from './config.ts';
import { ALL_SCENARIOS, filterScenarios } from './scenarios.ts';
import { runLoadTest } from './runner.ts';
import { printReport, writeCSV } from './reporter.ts';

async function main(): Promise<void> {
  const config = parseArgs(process.argv);

  // Filter scenarios by tags
  const scenarios = filterScenarios(ALL_SCENARIOS, config.tags);

  console.log('\n🔧 lightNVR Load Test Harness');
  console.log('─'.repeat(40));
  console.log(`  Target:       ${config.baseURL}`);
  console.log(`  Requests:     ${config.totalRequests}`);
  console.log(`  Concurrency:  ${config.concurrency}`);
  console.log(`  Ramp-up:      ${config.rampUpSeconds}s`);
  console.log(`  Think time:   ${config.thinkTimeMs}ms`);
  console.log(`  Timeout:      ${config.timeoutMs}ms`);
  console.log(`  Tags:         ${config.tags.length > 0 ? config.tags.join(', ') : 'all'}`);
  console.log(`  Scenarios:    ${scenarios.length} matched`);
  console.log('─'.repeat(40));

  if (scenarios.length === 0) {
    console.error('\n❌ No scenarios matched the given tags.');
    console.error('   Available tags: api, html, auth, system, streams, recordings,');
    console.error('                   settings, health, onvif, motion, timeline');
    process.exit(1);
  }

  // Connectivity pre-check
  console.log('\n⏳ Checking connectivity...');
  try {
    const authHeader =
      'Basic ' + Buffer.from(`${config.auth.username}:${config.auth.password}`).toString('base64');
    const res = await fetch(`${config.baseURL}/api/health`, {
      headers: { Authorization: authHeader },
      signal: AbortSignal.timeout(config.timeoutMs),
    });
    if (!res.ok && res.status !== 401) {
      console.warn(`   ⚠ Health check returned HTTP ${res.status}. Proceeding anyway.`);
    } else {
      console.log('   ✓ Server is reachable');
    }
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : String(err);
    console.error(`   ❌ Cannot reach ${config.baseURL}: ${msg}`);
    console.error('   Make sure lightNVR is running and the URL is correct.');
    process.exit(1);
  }

  // Run
  console.log(`\n🚀 Starting load test — ${config.totalRequests} requests across ${config.concurrency} workers...\n`);
  const startMs = performance.now();
  const results = await runLoadTest(scenarios, config);
  const durationMs = performance.now() - startMs;

  // Report
  printReport(results, config, durationMs);

  // CSV export
  if (config.csvPath) {
    writeCSV(results, config.csvPath);
  }

  // Exit code: non-zero if > 5 % failures
  const failRate = results.filter(r => !r.success).length / results.length;
  if (failRate > 0.05) {
    console.log(`⚠ Failure rate ${(failRate * 100).toFixed(1)}% exceeds 5% threshold — exiting with code 1`);
    process.exit(1);
  }
}

main().catch(err => {
  console.error('Fatal error:', err);
  process.exit(2);
});

