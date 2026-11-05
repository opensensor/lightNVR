/**
 * LightNVR Web Interface Authentication Utilities
 * Helper functions for managing authentication state and session validation
 */

import { enhancedFetch } from '../fetch-utils.ts';
import type {} from '../types/global.d.ts';
import type { AuthVerifyResponse, SessionInfo, SessionUser } from '../types/core.ts';

const SESSION_TIMEOUT_MS = 10000;
let authGeneration = 0;
let stopPreviousValidation: (() => void) | undefined;

function isVerifyPayload(value: unknown): value is AuthVerifyResponse {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return false;
  const data = value as Record<string, unknown>;
  for (const key of ['authenticated', 'demo_mode', 'is_active', 'password_change_locked', 'must_change_password', 'auth_enabled']) {
    if (data[key] !== undefined && typeof data[key] !== 'boolean') return false;
  }
  for (const key of ['username', 'email']) {
    if (data[key] !== undefined && typeof data[key] !== 'string') return false;
  }
  for (const key of ['id', 'role_id']) {
    if (data[key] !== undefined && (typeof data[key] !== 'number' || !Number.isSafeInteger(data[key]))) return false;
  }
  return data.role === undefined || (typeof data.role === 'string' &&
    ['admin', 'user', 'viewer', 'api', 'unknown'].includes(data.role));
}

/**
 * Check if user has authentication credentials stored
 * @returns {boolean} - True if auth credentials exist
 */
export function hasAuthCredentials(): boolean {
  const auth = localStorage.getItem('auth');
  const sessionCookie = document.cookie.split('; ').find(row => row.startsWith('session='));
  return !!(auth || sessionCookie);
}

/**
 * Get authentication headers for API requests
 * @returns {Object} - Headers object with Authorization if available
 */
export function getAuthHeaders(): Record<string, string> {
  const auth = localStorage.getItem('auth');
  return auth ? { 'Authorization': 'Basic ' + auth } : {};
}

/**
 * Clear all authentication state
 */
export function clearAuthState(): void {
  authGeneration++;
  // Clear localStorage
  localStorage.removeItem('auth');

  // Clear non-HttpOnly cookies (session cookie is HttpOnly and can only be
  // cleared server-side, but we clear legacy/fallback cookies here)
  document.cookie = "auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Lax";
  document.cookie = "session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Lax";
  window._demoMode = false;
  window._authDisabled = false;
  window.dispatchEvent(new Event('lightnvr:auth-cleared'));
  // Cookie-only sessions may have no localStorage entry to trigger a storage
  // event. Notify other tabs without storing or transmitting credentials.
  if (typeof BroadcastChannel === 'function') {
    try {
      const channel = new BroadcastChannel('lightnvr-auth');
      try { channel.postMessage('cleared'); } finally { channel.close(); }
    } catch { /* Cross-tab messaging can be disabled by the browser. */ }
  }
}

/**
 * Validate current session by making a lightweight API call
 * @returns {Promise<{valid: boolean, id?: number, username?: string, email?: string, role?: string, role_id?: number, is_active?: boolean, password_change_locked?: boolean, must_change_password?: boolean, demo_mode?: boolean, authenticated?: boolean}>} - Session info
 */
export async function validateSession(options: {signal?: AbortSignal; verifyUrl?: string} = {}): Promise<SessionInfo> {
  const generation = authGeneration;
  const controller = new AbortController();
  const abort = () => controller.abort();
  options.signal?.addEventListener('abort', abort, {once: true});
  if (options.signal?.aborted) controller.abort();
  // Cover the response body too: enhancedFetch's own deadline ends at headers.
  const deadline = setTimeout(abort, SESSION_TIMEOUT_MS);
  try {
    // Use enhancedFetch with skipAuthRedirect to avoid triggering the global 401 handler
    // We want to handle the validation result ourselves
    const response = await enhancedFetch(options.verifyUrl ?? '/api/auth/verify', {
      method: 'GET',
      skipAuthRedirect: true, // Don't auto-redirect on 401
      signal: controller.signal,
      timeout: 0,
      retries: 0, // The caller controls retry; one bounded verification at a time.
      // Explorer identity checks must cross the HTTP boundary on each refresh.
      ...(options.verifyUrl !== undefined ? { cache: 'no-store' as const } : {}),
    });

    if (response.ok) {
      const data: unknown = await response.json();
      if (controller.signal.aborted || generation !== authGeneration || !isVerifyPayload(data)) {
        return {valid: false, status: 'unavailable'};
      }
      // In demo mode, the server returns authenticated: false but demo_mode: true
      // We treat this as a valid session with viewer-level access
      const isDemoMode = data.demo_mode === true;
      const isAuthenticated = data.authenticated === true;
      if (data.is_active === false || (data.authenticated === false && !isDemoMode)) {
        return {valid: false, status: 'invalid'};
      }
      if ((!isAuthenticated && !isDemoMode) || !data.role ||
          (isDemoMode && !isAuthenticated && data.role !== 'viewer')) {
        return {valid: false, status: 'unavailable'};
      }

      // Store demo mode state globally for UI components
      window._demoMode = isDemoMode;
      window._authDisabled = data.auth_enabled === false;

      return {
        valid: true,
        status: 'valid',
        authenticated: isAuthenticated,
        demo_mode: isDemoMode,
        id: data.id,
        username: data.username,
        email: data.email,
        role: data.role,
        role_id: data.role_id,
        is_active: data.is_active,
        password_change_locked: data.password_change_locked,
        must_change_password: data.must_change_password === true,
        auth_enabled: data.auth_enabled
      };
    }
    return {valid: false, status: response.status === 401 || response.status === 403 ? 'invalid' : 'unavailable'};
  } catch (error) {
    if (controller.signal.aborted || generation !== authGeneration) return {valid: false, status: 'unavailable'};
    const status = error && typeof error === 'object' && 'status' in error ? error.status : undefined;
    return {valid: false, status: status === 401 || status === 403 ? 'invalid' : 'unavailable'};
  } finally {
    clearTimeout(deadline);
    options.signal?.removeEventListener('abort', abort);
  }
}

/**
 * Check if demo mode is enabled
 * @returns {boolean} - True if in demo mode
 */
export function isDemoMode(): boolean {
  return window._demoMode === true;
}

/**
 * Get user info from session
 * @returns {Promise<{username: string, role: string, role_id: number}|null>} - User info or null
 */
export async function getUserInfo(): Promise<SessionUser | null> {
  const session = await validateSession();
  if (session.valid) {
    return {
      id: session.id,
      username: session.username,
      email: session.email,
      role: session.role,
      role_id: session.role_id,
      is_active: session.is_active,
      password_change_locked: session.password_change_locked,
      must_change_password: session.must_change_password
    };
  }
  return null;
}

/**
 * Check if user has admin role
 * @returns {Promise<boolean>}
 */
export async function isAdmin(): Promise<boolean> {
  const session = await validateSession();
  return session.valid && session.role === 'admin';
}

/**
 * Check if user has viewer role (most restrictive)
 * @returns {Promise<boolean>}
 */
export async function isViewer(): Promise<boolean> {
  const session = await validateSession();
  return session.valid && session.role === 'viewer';
}

/**
 * Check if we're currently on the login page
 * @returns {boolean} - True if on login page
 */
export function isOnLoginPage(): boolean {
  return window.location.pathname.includes('login.html');
}

/**
 * Redirect to login page with optional reason
 * @param {string} reason - Reason for redirect (optional)
 */
export function redirectToLogin(reason: string | null = null): void {
  if (isOnLoginPage()) {
    return; // Already on login page
  }
  
  const params = new URLSearchParams({ auth_required: 'true' });
  if (reason) {
    params.set('reason', reason);
  }
  
  // Store current page for redirect after login
  const currentPath = window.location.pathname + window.location.search;
  if (currentPath !== '/' && !currentPath.includes('login.html')) {
    params.set('redirect', currentPath);
  }
  
  window.location.href = `/login.html?${params.toString()}`;
}

/**
 * Setup periodic session validation
 * @param {number} intervalMs - Interval in milliseconds (default: 5 minutes)
 * @returns {number|null} - Interval ID that can be used to clear the interval, or null if not setup
 */
export function setupSessionValidation(intervalMs: number = 5 * 60 * 1000): ReturnType<typeof setInterval> | null {
  stopPreviousValidation?.();
  if (isOnLoginPage()) return null;

  let stopped = false;
  let paused = false;
  let inFlight: AbortController | null = null;
  const check = async () => {
    if (stopped || paused || inFlight || isOnLoginPage()) return;
    const controller = new AbortController();
    inFlight = controller;
    try {
      const session = await validateSession({signal: controller.signal});
      if (stopped || paused || controller.signal.aborted) return;
      if (session.status === 'invalid') {
        clearAuthState();
        redirectToLogin('session_expired');
      }
      // Unavailable neither erases credentials nor restores cached permissions.
      // HttpOnly session cookies are invisible to hasAuthCredentials(): always
      // ask the server, including after auth-disabled/demo configuration changes.
    } finally {
      if (inFlight === controller) inFlight = null;
    }
  };
  const intervalId = setInterval(() => { void check(); }, Math.max(1000, intervalMs));
  const pause = () => { paused = true; inFlight?.abort(); };
  const resume = () => { paused = false; void check(); };
  window.addEventListener('pagehide', pause);
  window.addEventListener('pageshow', resume);
  stopPreviousValidation = () => {
    stopped = true;
    clearInterval(intervalId);
    inFlight?.abort();
    window.removeEventListener('pagehide', pause);
    window.removeEventListener('pageshow', resume);
  };
  void check();
  return intervalId;
}
