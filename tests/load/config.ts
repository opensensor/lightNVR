/**
 * Load Test Configuration and Types
 *
 * Defines all configuration, types, and CLI argument parsing for the
 * lightNVR load testing harness.
 */

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export interface Scenario {
  /** Human-readable name */
  name: string;
  /** HTTP method */
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
  /** URL path (relative to baseURL) */
  path: string;
  /** Optional JSON body for POST/PUT */
  body?: unknown;
  /** Expected successful status codes */
  expectedStatus?: number[];
  /** Additional headers */
  headers?: Record<string, string>;
  /** Weight for random selection (higher = more likely). Default 1 */
  weight?: number;
  /** Tags for filtering (e.g. 'api', 'html', 'auth') */
  tags: string[];
}

export interface RequestResult {
  scenario: string;
  method: string;
  path: string;
  status: number;
  latencyMs: number;
  success: boolean;
  error?: string;
  timestamp: number;
  byteLength: number;
}

export interface LoadTestConfig {
  /** Base URL of the lightNVR instance */
  baseURL: string;
  /** Total number of requests to send */
  totalRequests: number;
  /** Number of concurrent workers */
  concurrency: number;
  /** Ramp-up period in seconds (0 = all workers start immediately) */
  rampUpSeconds: number;
  /** Think time between requests per worker in ms (simulates user pauses) */
  thinkTimeMs: number;
  /** Request timeout in ms */
  timeoutMs: number;
  /** Basic auth credentials */
  auth: { username: string; password: string };
  /** Scenario tag filter (empty = all) */
  tags: string[];
  /** Output CSV path (empty = no CSV) */
  csvPath: string;
  /** Verbose logging */
  verbose: boolean;
}

// ---------------------------------------------------------------------------
// Defaults
// ---------------------------------------------------------------------------

export const DEFAULT_CONFIG: LoadTestConfig = {
  baseURL: process.env.LIGHTNVR_URL || 'http://localhost:8090',
  totalRequests: 200,
  concurrency: 10,
  rampUpSeconds: 2,
  thinkTimeMs: 100,
  timeoutMs: 10000,
  auth: {
    username: process.env.LIGHTNVR_USER || 'admin',
    password: process.env.LIGHTNVR_PASS || 'admin',
  },
  tags: [],
  csvPath: '',
  verbose: false,
};

// ---------------------------------------------------------------------------
// CLI Parsing
// ---------------------------------------------------------------------------

export function parseArgs(argv: string[]): LoadTestConfig {
  const config = { ...DEFAULT_CONFIG, auth: { ...DEFAULT_CONFIG.auth } };

  for (let i = 2; i < argv.length; i++) {
    const arg = argv[i];
    const next = (): string => {
      if (i + 1 >= argv.length) throw new Error(`Missing value for ${arg}`);
      return argv[++i];
    };

    switch (arg) {
      case '--url':          config.baseURL = next(); break;
      case '--requests':
      case '-n':             config.totalRequests = parseInt(next(), 10); break;
      case '--concurrency':
      case '-c':             config.concurrency = parseInt(next(), 10); break;
      case '--ramp-up':      config.rampUpSeconds = parseFloat(next()); break;
      case '--think-time':   config.thinkTimeMs = parseInt(next(), 10); break;
      case '--timeout':      config.timeoutMs = parseInt(next(), 10); break;
      case '--user':         config.auth.username = next(); break;
      case '--password':     config.auth.password = next(); break;
      case '--tags':         config.tags = next().split(',').map(t => t.trim()); break;
      case '--csv':          config.csvPath = next(); break;
      case '--verbose':
      case '-v':             config.verbose = true; break;
      case '--help':
      case '-h':             printUsage(); process.exit(0);
      default:
        console.error(`Unknown argument: ${arg}`);
        printUsage();
        process.exit(1);
    }
  }

  return config;
}

function printUsage(): void {
  console.log(`
lightNVR Load Test Harness

Usage: node --experimental-strip-types tests/load/index.ts [options]

Options:
  --url <url>           Base URL (default: $LIGHTNVR_URL or http://localhost:8090)
  -n, --requests <n>    Total requests to send (default: 200)
  -c, --concurrency <n> Concurrent workers (default: 10)
  --ramp-up <sec>       Ramp-up period in seconds (default: 2)
  --think-time <ms>     Think time between requests per worker (default: 100)
  --timeout <ms>        Request timeout in ms (default: 10000)
  --user <user>         Auth username (default: admin)
  --password <pass>     Auth password (default: admin)
  --tags <t1,t2,...>    Filter scenarios by tag: api, html, auth, system,
                        streams, recordings, settings, health (default: all)
  --csv <path>          Write detailed results to CSV file
  -v, --verbose         Verbose output
  -h, --help            Show this help

Examples:
  # Run all scenarios with 20 concurrent users, 500 total requests
  node --experimental-strip-types tests/load/index.ts -n 500 -c 20

  # Test only API endpoints
  node --experimental-strip-types tests/load/index.ts --tags api

  # Test only HTML pages with CSV output
  node --experimental-strip-types tests/load/index.ts --tags html --csv results.csv

  # Heavy load test against a remote server
  node --experimental-strip-types tests/load/index.ts --url http://192.168.1.50:8080 -n 2000 -c 50
`);
}

