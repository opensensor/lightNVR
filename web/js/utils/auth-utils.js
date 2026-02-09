/**
 * LightNVR Web Interface Authentication Utilities
 * Helper functions for managing authentication state and session validation
 */

import { enhancedFetch } from '../fetch-utils.js';

/**
 * Check if user has authentication credentials stored
 * @returns {boolean} - True if auth credentials exist
 */
export function hasAuthCredentials() {
  const auth = localStorage.getItem('auth');
  const sessionCookie = document.cookie.split('; ').find(row => row.startsWith('session='));
  return !!(auth || sessionCookie);
}

/**
 * Get authentication headers for API requests
 * @returns {Object} - Headers object with Authorization if available
 */
export function getAuthHeaders() {
  const auth = localStorage.getItem('auth');
  return auth ? { 'Authorization': 'Basic ' + auth } : {};
}

/**
 * Clear all authentication state
 */
export function clearAuthState() {
  // Clear localStorage
  localStorage.removeItem('auth');
  
  // Clear cookies
  document.cookie = "auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict";
  document.cookie = "session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict";
}

/**
 * Validate current session by making a lightweight API call
 * @returns {Promise<{valid: boolean, username?: string, role?: string, role_id?: number}>} - Session info
 */
export async function validateSession() {
  try {
    // Use enhancedFetch with skipAuthRedirect to avoid triggering the global 401 handler
    // We want to handle the validation result ourselves
    const response = await enhancedFetch('/api/auth/verify', {
      method: 'GET',
      skipAuthRedirect: true, // Don't auto-redirect on 401
    });

    if (response.ok) {
      const data = await response.json();
      return {
        valid: true,
        username: data.username,
        role: data.role,
        role_id: data.role_id,
        auth_enabled: data.auth_enabled
      };
    }
    return { valid: false };
  } catch (error) {
    // If we get a 401 or other error, session is invalid
    console.debug('Session validation failed:', error.message);
    return { valid: false };
  }
}

/**
 * Get user info from session
 * @returns {Promise<{username: string, role: string, role_id: number}|null>} - User info or null
 */
export async function getUserInfo() {
  const session = await validateSession();
  if (session.valid) {
    return {
      username: session.username,
      role: session.role,
      role_id: session.role_id
    };
  }
  return null;
}

/**
 * Check if user has admin role
 * @returns {Promise<boolean>}
 */
export async function isAdmin() {
  const session = await validateSession();
  return session.valid && session.role === 'admin';
}

/**
 * Check if user has viewer role (most restrictive)
 * @returns {Promise<boolean>}
 */
export async function isViewer() {
  const session = await validateSession();
  return session.valid && session.role === 'viewer';
}

/**
 * Check if we're currently on the login page
 * @returns {boolean} - True if on login page
 */
export function isOnLoginPage() {
  return window.location.pathname.includes('login.html');
}

/**
 * Redirect to login page with optional reason
 * @param {string} reason - Reason for redirect (optional)
 */
export function redirectToLogin(reason = null) {
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
export function setupSessionValidation(intervalMs = 5 * 60 * 1000) {
  // Don't setup validation on login page
  if (isOnLoginPage()) {
    console.debug('Skipping session validation setup on login page');
    return null;
  }

  console.log(`Setting up session validation with ${intervalMs}ms interval`);

  // Validate immediately on page load by calling the server
  // This handles both auth-enabled and auth-disabled cases:
  // - If auth is disabled, /api/auth/verify returns success (no redirect needed)
  // - If auth is enabled and session is valid, returns success
  // - If auth is enabled and session is invalid, we redirect to login
  validateSession().then(session => {
    if (!session.valid) {
      console.warn('Initial session validation failed, redirecting to login');
      clearAuthState();
      redirectToLogin('session_expired');
    } else {
      console.debug('Initial session validation passed for user:', session.username);
      // If auth is disabled on the server, store that fact to avoid unnecessary checks
      if (session.auth_enabled === false) {
        console.debug('Authentication is disabled on the server, skipping periodic validation');
        window._authDisabled = true;
      }
    }
  });

  // Setup periodic validation
  const intervalId = setInterval(async () => {
    // Skip periodic validation if auth is disabled on the server
    if (window._authDisabled) {
      console.debug('Session validation skipped - auth disabled on server');
      return;
    }

    // Check credentials still exist before validating
    if (!hasAuthCredentials()) {
      console.debug('Session validation skipped - no credentials');
      return;
    }

    console.debug('Running periodic session validation');
    const session = await validateSession();

    if (!session.valid) {
      console.warn('Periodic session validation failed, redirecting to login');
      clearAuthState();
      redirectToLogin('session_expired');
    } else {
      console.debug('Session validation passed for user:', session.username);
    }
  }, intervalMs);

  return intervalId;
}

