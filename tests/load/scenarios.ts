/**
 * Load Test Scenarios
 *
 * Defines every API endpoint and HTML page that the load test harness
 * exercises.  Each scenario carries tags so the runner can filter by
 * category (api, html, auth, system, streams, recordings, …).
 */

import type { Scenario, LoadTestConfig } from './config.ts';

// ---------------------------------------------------------------------------
// HTML Pages
// ---------------------------------------------------------------------------

const HTML_PAGES: Scenario[] = [
  { name: 'Login page',      method: 'GET', path: '/login.html',      tags: ['html', 'auth'],       weight: 2, expectedStatus: [200, 302] },
  { name: 'Dashboard',       method: 'GET', path: '/index.html',      tags: ['html'],               weight: 3, expectedStatus: [200, 302] },
  { name: 'Streams page',    method: 'GET', path: '/streams.html',    tags: ['html', 'streams'],    weight: 2, expectedStatus: [200, 302] },
  { name: 'Recordings page', method: 'GET', path: '/recordings.html', tags: ['html', 'recordings'], weight: 2, expectedStatus: [200, 302] },
  { name: 'Timeline page',   method: 'GET', path: '/timeline.html',   tags: ['html'],               weight: 1, expectedStatus: [200, 302] },
  { name: 'Settings page',   method: 'GET', path: '/settings.html',   tags: ['html', 'settings'],   weight: 1, expectedStatus: [200, 302] },
  { name: 'System page',     method: 'GET', path: '/system.html',     tags: ['html', 'system'],     weight: 1, expectedStatus: [200, 302] },
  { name: 'Users page',      method: 'GET', path: '/users.html',      tags: ['html'],               weight: 1, expectedStatus: [200, 302] },
  { name: 'HLS page',        method: 'GET', path: '/hls.html',        tags: ['html'],               weight: 1, expectedStatus: [200, 302] },
];

// ---------------------------------------------------------------------------
// API — Health / System
// ---------------------------------------------------------------------------

const HEALTH_API: Scenario[] = [
  { name: 'Health check',     method: 'GET', path: '/api/health',     tags: ['api', 'health'],  weight: 3, expectedStatus: [200, 401] },
  { name: 'HLS health',       method: 'GET', path: '/api/health/hls', tags: ['api', 'health'],  weight: 1, expectedStatus: [200, 401] },
  { name: 'System info',      method: 'GET', path: '/api/system',     tags: ['api', 'system'],  weight: 2, expectedStatus: [200, 401] },
  { name: 'System info (v2)', method: 'GET', path: '/api/system/info',tags: ['api', 'system'],  weight: 1, expectedStatus: [200, 401] },
  { name: 'System status',    method: 'GET', path: '/api/system/status', tags: ['api', 'system'], weight: 2, expectedStatus: [200, 401] },
  { name: 'System logs',      method: 'GET', path: '/api/system/logs',   tags: ['api', 'system'], weight: 1, expectedStatus: [200, 401] },
];

// ---------------------------------------------------------------------------
// API — Auth
// ---------------------------------------------------------------------------

const AUTH_API: Scenario[] = [
  { name: 'Auth verify',  method: 'GET',  path: '/api/auth/verify', tags: ['api', 'auth'], weight: 3, expectedStatus: [200, 401] },
  { name: 'Auth login',   method: 'POST', path: '/api/auth/login',
    body: { username: 'admin', password: 'admin' },
    headers: { 'Content-Type': 'application/json' },
    expectedStatus: [200, 302, 400],
    tags: ['api', 'auth'], weight: 2 },
  { name: 'Users list',   method: 'GET',  path: '/api/auth/users',  tags: ['api', 'auth'], weight: 1, expectedStatus: [200, 401] },
];

// ---------------------------------------------------------------------------
// API — Streams (read-only)
// ---------------------------------------------------------------------------

const STREAMS_API: Scenario[] = [
  { name: 'List streams',   method: 'GET', path: '/api/streams',   tags: ['api', 'streams'], weight: 3, expectedStatus: [200, 401] },
];

// ---------------------------------------------------------------------------
// API — Recordings (read-only)
// ---------------------------------------------------------------------------

const RECORDINGS_API: Scenario[] = [
  { name: 'List recordings',       method: 'GET', path: '/api/recordings',
    tags: ['api', 'recordings'], weight: 3, expectedStatus: [200, 401] },
  { name: 'List recordings (page 1, limit 10)', method: 'GET',
    path: '/api/recordings?page=1&limit=10',
    tags: ['api', 'recordings'], weight: 2, expectedStatus: [200, 401] },
  { name: 'Protected recordings',  method: 'GET', path: '/api/recordings/protected',
    tags: ['api', 'recordings'], weight: 1, expectedStatus: [200, 401] },
  { name: 'Retention policy',      method: 'GET', path: '/api/recordings/retention',
    tags: ['api', 'recordings'], weight: 1, expectedStatus: [200, 400, 401] },
];

// ---------------------------------------------------------------------------
// API — Settings (read-only)
// ---------------------------------------------------------------------------

const SETTINGS_API: Scenario[] = [
  { name: 'Get settings', method: 'GET', path: '/api/settings', tags: ['api', 'settings'], weight: 2, expectedStatus: [200, 401] },
];

// ---------------------------------------------------------------------------
// API — ONVIF (read-only)
// ---------------------------------------------------------------------------

const ONVIF_API: Scenario[] = [
  { name: 'ONVIF discovery status', method: 'GET', path: '/api/onvif/discovery/status',
    tags: ['api', 'onvif'], weight: 1, expectedStatus: [200, 401, 404] },
  { name: 'ONVIF devices',          method: 'GET', path: '/api/onvif/devices',
    tags: ['api', 'onvif'], weight: 1, expectedStatus: [200, 401, 404] },
];

// ---------------------------------------------------------------------------
// API — Motion (read-only)
// ---------------------------------------------------------------------------

const MOTION_API: Scenario[] = [
  { name: 'Motion storage', method: 'GET', path: '/api/motion/storage',
    tags: ['api', 'motion'], weight: 1, expectedStatus: [200, 401, 404] },
];

// ---------------------------------------------------------------------------
// API — Timeline (read-only)
// ---------------------------------------------------------------------------

const TIMELINE_API: Scenario[] = [
  { name: 'Timeline segments', method: 'GET', path: '/api/timeline/segments',
    tags: ['api', 'timeline'], weight: 1, expectedStatus: [200, 400, 401] },
];

// ---------------------------------------------------------------------------
// Combined export
// ---------------------------------------------------------------------------

/**
 * Build the full scenario list, injecting config credentials into the
 * Auth login scenario so the body matches the --user / --password flags.
 */
export function getAllScenarios(config: LoadTestConfig): Scenario[] {
  const authApi: Scenario[] = AUTH_API.map(s => {
    if (s.name === 'Auth login') {
      return { ...s, body: { username: config.auth.username, password: config.auth.password } };
    }
    return s;
  });

  return [
    ...HTML_PAGES,
    ...HEALTH_API,
    ...authApi,
    ...STREAMS_API,
    ...RECORDINGS_API,
    ...SETTINGS_API,
    ...ONVIF_API,
    ...MOTION_API,
    ...TIMELINE_API,
  ];
}

/**
 * Return scenarios filtered by the given tags.
 * If `tags` is empty every scenario is returned.
 */
export function filterScenarios(scenarios: Scenario[], tags: string[]): Scenario[] {
  if (tags.length === 0) return scenarios;
  return scenarios.filter(s => s.tags.some(t => tags.includes(t)));
}

