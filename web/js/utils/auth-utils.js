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
 * @returns {Promise<boolean>} - True if session is valid
 */
export async function validateSession() {
  try {
    // Use enhancedFetch with skipAuthRedirect to avoid triggering the global 401 handler
    // We want to handle the validation result ourselves
    const response = await enhancedFetch('/api/auth/verify', {
      method: 'GET',
      skipAuthRedirect: true, // Don't auto-redirect on 401
    });

    return response.ok;
  } catch (error) {
    // If we get a 401 or other error, session is invalid
    console.debug('Session validation failed:', error.message);
    return false;
  }
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

  // Don't validate if no credentials exist (user not logged in)
  if (!hasAuthCredentials()) {
    console.debug('Skipping session validation setup - no credentials found');
    return null;
  }

  console.log(`Setting up session validation with ${intervalMs}ms interval`);

  // Setup periodic validation only - don't validate immediately
  // The global 401 handler in fetch-utils.js will catch expired sessions
  // when the first API call is made
  const intervalId = setInterval(async () => {
    // Double-check credentials still exist before validating
    if (!hasAuthCredentials()) {
      console.debug('Session validation skipped - no credentials');
      return;
    }

    console.debug('Running periodic session validation');
    const isValid = await validateSession();

    if (!isValid) {
      console.warn('Periodic session validation failed, redirecting to login');
      clearAuthState();
      redirectToLogin('session_expired');
    } else {
      console.debug('Session validation passed');
    }
  }, intervalMs);

  return intervalId;
}

