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
 * @returns {Promise<string>} - go2rtc base URL (e.g., "http://hostname:1984")
 */
export async function getGo2rtcBaseUrl() {
  const port = await getGo2rtcApiPort();
  return `http://${window.location.hostname}:${port}`;
}

/**
 * Get default settings (used as fallback)
 * @returns {Object} - Default settings object
 */
function getDefaultSettings() {
  return {
    go2rtc_api_port: 1984,
    webrtc_disabled: false,
    web_port: 8080
  };
}

/**
 * Clear the settings cache
 */
export function clearSettingsCache() {
  settingsCache = null;
  settingsCacheTime = 0;
}

