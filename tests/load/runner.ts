/**
 * Load Test Runner
 *
 * Core engine that dispatches concurrent HTTP requests according to the
 * weighted scenario list, collects per-request metrics, and honours the
 * configured ramp-up / think-time / timeout settings.
 */

import type { LoadTestConfig, RequestResult, Scenario } from './config.ts';

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

/** Pick a random scenario respecting weights. */
function pickScenario(scenarios: Scenario[]): Scenario {
  const totalWeight = scenarios.reduce((s, sc) => s + (sc.weight ?? 1), 0);
  let r = Math.random() * totalWeight;
  for (const sc of scenarios) {
    r -= sc.weight ?? 1;
    if (r <= 0) return sc;
  }
  return scenarios[scenarios.length - 1];
}

// ---------------------------------------------------------------------------
// Single request executor
// ---------------------------------------------------------------------------

async function executeRequest(
  scenario: Scenario,
  config: LoadTestConfig,
  authHeader: string,
): Promise<RequestResult> {
  const url = `${config.baseURL}${scenario.path}`;
  const start = performance.now();
  const timestamp = Date.now();

  const headers: Record<string, string> = {
    'Authorization': authHeader,
    'Accept': scenario.tags.includes('html') ? 'text/html' : 'application/json',
    'Connection': 'close',  // Disable keep-alive — the libuv server has a race condition
    ...scenario.headers,
  };

  const init: RequestInit = {
    method: scenario.method,
    headers,
    redirect: 'manual',  // Don't follow redirects — measure the raw response
    signal: AbortSignal.timeout(config.timeoutMs),
  };

  if (scenario.body && (scenario.method === 'POST' || scenario.method === 'PUT')) {
    headers['Content-Type'] = 'application/json';
    init.body = JSON.stringify(scenario.body);
  }

  try {
    const res = await fetch(url, init);
    const buf = await res.arrayBuffer();
    const latencyMs = performance.now() - start;

    const expected = scenario.expectedStatus ?? [200];
    const success = expected.includes(res.status);

    return {
      scenario: scenario.name,
      method: scenario.method,
      path: scenario.path,
      status: res.status,
      latencyMs,
      success,
      timestamp,
      byteLength: buf.byteLength,
    };
  } catch (err: unknown) {
    const latencyMs = performance.now() - start;
    const message = err instanceof Error ? `${err.message} [${(err as any).cause ?? ''}]` : String(err);
    return {
      scenario: scenario.name,
      method: scenario.method,
      path: scenario.path,
      status: 0,
      latencyMs,
      success: false,
      error: message,
      timestamp,
      byteLength: 0,
    };
  }
}

// ---------------------------------------------------------------------------
// Worker loop
// ---------------------------------------------------------------------------

async function worker(
  id: number,
  scenarios: Scenario[],
  config: LoadTestConfig,
  authHeader: string,
  results: RequestResult[],
  remaining: { count: number },
): Promise<void> {
  while (true) {
    // Atomically claim a request slot
    if (remaining.count <= 0) break;
    remaining.count--;

    const scenario = pickScenario(scenarios);
    if (config.verbose) {
      console.log(`  [worker ${id}] ${scenario.method} ${scenario.path}`);
    }

    const result = await executeRequest(scenario, config, authHeader);
    results.push(result);

    if (config.thinkTimeMs > 0) {
      // Add ±30 % jitter to avoid thundering herd
      const jitter = config.thinkTimeMs * 0.3 * (Math.random() * 2 - 1);
      await sleep(Math.max(0, config.thinkTimeMs + jitter));
    }
  }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

export async function runLoadTest(
  scenarios: Scenario[],
  config: LoadTestConfig,
): Promise<RequestResult[]> {
  if (scenarios.length === 0) {
    throw new Error('No scenarios to run. Check your --tags filter.');
  }

  const authHeader =
    'Basic ' + Buffer.from(`${config.auth.username}:${config.auth.password}`).toString('base64');

  const results: RequestResult[] = [];
  const remaining = { count: config.totalRequests };

  // Ramp-up: start workers progressively
  const rampDelayMs =
    config.rampUpSeconds > 0 ? (config.rampUpSeconds * 1000) / config.concurrency : 0;

  const workerPromises: Promise<void>[] = [];

  for (let i = 0; i < config.concurrency; i++) {
    if (rampDelayMs > 0 && i > 0) await sleep(rampDelayMs);
    workerPromises.push(worker(i, scenarios, config, authHeader, results, remaining));
  }

  await Promise.all(workerPromises);

  return results;
}

