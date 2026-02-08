/**
 * Load Test Reporter
 *
 * Aggregates raw request results into human-readable terminal output and
 * optionally writes a CSV file for further analysis.
 */

import { writeFileSync } from 'node:fs';
import type { LoadTestConfig, RequestResult } from './config.ts';

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function percentile(sorted: number[], p: number): number {
  if (sorted.length === 0) return 0;
  const idx = Math.ceil((p / 100) * sorted.length) - 1;
  return sorted[Math.max(0, idx)];
}

function fmt(ms: number): string {
  return ms.toFixed(1);
}

interface ScenarioStats {
  name: string;
  count: number;
  successes: number;
  failures: number;
  latencies: number[];
}

// ---------------------------------------------------------------------------
// CSV export
// ---------------------------------------------------------------------------

export function writeCSV(results: RequestResult[], filePath: string): void {
  const header = 'timestamp,scenario,method,path,status,latency_ms,success,bytes,error\n';
  const rows = results.map(r =>
    [
      new Date(r.timestamp).toISOString(),
      `"${r.scenario}"`,
      r.method,
      r.path,
      r.status,
      r.latencyMs.toFixed(2),
      r.success,
      r.byteLength,
      r.error ? `"${r.error.replace(/"/g, '""')}"` : '',
    ].join(','),
  );
  writeFileSync(filePath, header + rows.join('\n') + '\n');
  console.log(`\n📄 Detailed CSV written to ${filePath}`);
}

// ---------------------------------------------------------------------------
// Terminal report
// ---------------------------------------------------------------------------

export function printReport(results: RequestResult[], config: LoadTestConfig, durationMs: number): void {
  const total = results.length;
  const successes = results.filter(r => r.success).length;
  const failures = total - successes;
  const allLatencies = results.map(r => r.latencyMs).sort((a, b) => a - b);
  const totalBytes = results.reduce((s, r) => s + r.byteLength, 0);

  // Per-scenario breakdown
  const byScenario = new Map<string, ScenarioStats>();
  for (const r of results) {
    let s = byScenario.get(r.scenario);
    if (!s) {
      s = { name: r.scenario, count: 0, successes: 0, failures: 0, latencies: [] };
      byScenario.set(r.scenario, s);
    }
    s.count++;
    if (r.success) s.successes++; else s.failures++;
    s.latencies.push(r.latencyMs);
  }

  // Status code distribution
  const statusCounts = new Map<number, number>();
  for (const r of results) {
    statusCounts.set(r.status, (statusCounts.get(r.status) ?? 0) + 1);
  }

  // Header
  console.log('\n' + '='.repeat(72));
  console.log('  lightNVR Load Test Report');
  console.log('='.repeat(72));
  console.log(`  Target:       ${config.baseURL}`);
  console.log(`  Concurrency:  ${config.concurrency} workers`);
  console.log(`  Total Reqs:   ${total}`);
  console.log(`  Duration:     ${(durationMs / 1000).toFixed(1)}s`);
  console.log(`  Throughput:   ${(total / (durationMs / 1000)).toFixed(1)} req/s`);
  console.log(`  Data:         ${(totalBytes / 1024).toFixed(1)} KB transferred`);
  console.log('-'.repeat(72));

  // Overall latency
  console.log('  Latency (ms)  min     p50     p90     p95     p99     max');
  console.log(`  overall       ${fmt(allLatencies[0] ?? 0).padStart(6)}  ${fmt(percentile(allLatencies, 50)).padStart(6)}  ${fmt(percentile(allLatencies, 90)).padStart(6)}  ${fmt(percentile(allLatencies, 95)).padStart(6)}  ${fmt(percentile(allLatencies, 99)).padStart(6)}  ${fmt(allLatencies[allLatencies.length - 1] ?? 0).padStart(6)}`);
  console.log('-'.repeat(72));

  // Success / failure
  const pct = total > 0 ? ((successes / total) * 100).toFixed(1) : '0.0';
  console.log(`  Success:  ${successes}/${total} (${pct}%)`);
  console.log(`  Failures: ${failures}`);

  // Status distribution
  console.log('-'.repeat(72));
  console.log('  Status code distribution:');
  for (const [code, count] of [...statusCounts.entries()].sort((a, b) => a[0] - b[0])) {
    const label = code === 0 ? 'ERR' : String(code);
    console.log(`    ${label}: ${count}`);
  }

  // Per-scenario table
  console.log('-'.repeat(72));
  console.log('  Per-scenario breakdown:');
  console.log('  ' + 'Scenario'.padEnd(30) + 'Reqs'.padStart(6) + '  OK'.padStart(5) +
    '  Fail'.padStart(5) + '  p50ms'.padStart(8) + '  p99ms'.padStart(8));

  for (const s of [...byScenario.values()].sort((a, b) => b.count - a.count)) {
    const sorted = s.latencies.sort((a, b) => a - b);
    console.log('  ' +
      s.name.padEnd(30) +
      String(s.count).padStart(6) +
      String(s.successes).padStart(5) +
      String(s.failures).padStart(5) +
      fmt(percentile(sorted, 50)).padStart(8) +
      fmt(percentile(sorted, 99)).padStart(8));
  }

  // Errors
  const errors = results.filter(r => r.error);
  if (errors.length > 0) {
    console.log('-'.repeat(72));
    console.log(`  Errors (showing first 10 of ${errors.length}):`);
    for (const e of errors.slice(0, 10)) {
      console.log(`    ${e.method} ${e.path} — ${e.error}`);
    }
  }

  console.log('='.repeat(72) + '\n');
}

