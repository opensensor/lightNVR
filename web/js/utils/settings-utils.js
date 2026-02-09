/**
 * LightNVR Settings Utilities
 * Provides cached access to server settings for frontend components
 */

import { fetchJSON } from '../fetch-utils.js';

// Cache for settings
let settingsCache = null;
let settingsCacheTime = 0;
const CACHE_TTL = 60000; // 1 minute cache TTL

/**
 * Fetch settings from the server with caching
 * @param {boolean} forceRefresh - Force refresh the cache
 * @returns {Promise<Object>} - Settings object
 */
export async function getSettings(forceRefresh = false) {
  const now = Date.now();
  
  // Return cached settings if still valid
  if (!forceRefresh && settingsCache && (now - settingsCacheTime) < CACHE_TTL) {
    return settingsCache;
  }
  
  try {
    const settings = await fetchJSON('/api/settings', {
      timeout: 10000,
      retries: 1,
      retryDelay: 500
    });
    
    // Update cache
    settingsCache = settings;
    settingsCacheTime = now;
    
    return settings;
  } catch (error) {
    console.error('Failed to fetch settings:', error);
    
    // Return cached settings if available, even if stale
    if (settingsCache) {
      console.warn('Using stale cached settings');
      return settingsCache;
    }
    
    // Return defaults if no cache available
    return getDefaultSettings();
  }
}

/**
 * Get the go2rtc API port from settings
 * @returns {Promise<number>} - go2rtc API port
 */
export async function getGo2rtcApiPort() {
  const settings = await getSettings();
  return settings.go2rtc_api_port || 1984; // Default to 1984 if not set
}

/**
 * Get the go2rtc base URL for API calls
 *
 * Over HTTPS: Uses origin + /go2rtc path prefix. The ingress/reverse proxy
 * should route /go2rtc/* directly to go2rtc service (which has base_path: /go2rtc/).
 * This avoids mixed content issues while hitting go2rtc directly (not through lightNVR).
 *
 * Over HTTP: Uses hostname:port to hit go2rtc directly.
 *
 * @returns {Promise<string>} - go2rtc base URL (e.g., "http://hostname:1984" or "https://hostname/go2rtc")
 */
export async function getGo2rtcBaseUrl() {
  // When served over HTTPS, use the /go2rtc path prefix.
  // The ingress routes /go2rtc/* directly to go2rtc service.
  if (window.location.protocol === 'https:') {
    return `${window.location.origin}/go2rtc`;
  }
  const port = await getGo2rtcApiPort();
  return `http://${window.location.hostname}:${port}`;
}

/**
 * Get default settings (used as fallback)
 * @returns {Object} - Default settings object
 */
function getDefaultSettings() {
  return {
    go2rtc_enabled: true,
    go2rtc_api_port: 1984,
    webrtc_disabled: false,
    web_port: 8080
  };
}

// Cache for go2rtc availability check
let go2rtcAvailableCache = null;
let go2rtcAvailableCacheTime = 0;
const GO2RTC_CACHE_TTL = 30000; // 30 second cache TTL for availability check

/**
 * Check if go2rtc is available and responding
 * Tries to reach go2rtc's API endpoint with a short timeout
 * @param {boolean} forceRefresh - Force refresh the cache
 * @returns {Promise<boolean>} - true if go2rtc is available
 */
export async function isGo2rtcAvailable(forceRefresh = false) {
  const now = Date.now();

  // Return cached result if still valid
  if (!forceRefresh && go2rtcAvailableCache !== null && (now - go2rtcAvailableCacheTime) < GO2RTC_CACHE_TTL) {
    return go2rtcAvailableCache;
  }

  try {
    const baseUrl = await getGo2rtcBaseUrl();
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 3000); // 3 second timeout

    const response = await fetch(`${baseUrl}/api/streams`, {
      method: 'GET',
      signal: controller.signal,
    });
    clearTimeout(timeoutId);

    const available = response.ok;
    go2rtcAvailableCache = available;
    go2rtcAvailableCacheTime = now;

    if (!available) {
      console.warn(`go2rtc API responded with status ${response.status}`);
    }
    return available;
  } catch (error) {
    console.warn('go2rtc is not available:', error.message);
    go2rtcAvailableCache = false;
    go2rtcAvailableCacheTime = now;
    return false;
  }
}

/**
 * Check if go2rtc is enabled in settings
 * This checks the configuration setting (not runtime availability)
 * @returns {Promise<boolean>} - true if go2rtc is enabled in settings
 */
export async function isGo2rtcEnabled() {
  const settings = await getSettings();
  // Default to true for backward compatibility
  return settings.go2rtc_enabled !== undefined ? settings.go2rtc_enabled : true;
}

/**
 * Clear the settings cache
 */
export function clearSettingsCache() {
  settingsCache = null;
  settingsCacheTime = 0;
  go2rtcAvailableCache = null;
  go2rtcAvailableCacheTime = 0;
}

