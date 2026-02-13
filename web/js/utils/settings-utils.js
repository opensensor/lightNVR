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
 * Get the go2rtc base URL for HTTP API calls (HLS, WebRTC SDP, etc.)
 *
 * go2rtc is configured with base_path: /go2rtc/, so all its API endpoints
 * are served under the /go2rtc prefix (e.g., /go2rtc/api/streams).
 *
 * Over HTTPS: Routes through external reverse proxy at /go2rtc/* which forwards
 * to go2rtc. The proxy must handle both HTTP and WebSocket connections.
 *
 * Over HTTP (local dev): lightNVR's built-in /go2rtc/* proxy only handles HLS
 * and snapshots (not WebRTC). WebRTC must connect directly to go2rtc's port
 * for lower latency and because the curl-based proxy can't handle the SDP exchange.
 *
 * @returns {Promise<string>} - go2rtc base URL (e.g., "https://hostname/go2rtc" or "http://hostname:1984/go2rtc")
 */
export async function getGo2rtcBaseUrl() {
  // Over HTTPS, use the external reverse proxy (nginx, ingress, etc.)
  // which handles both HTTP and WebSocket to /go2rtc/*
  if (window.location.protocol === 'https:') {
    return `${window.location.origin}/go2rtc`;
  }

  // Over HTTP (local dev), connect directly to go2rtc's port
  // because lightNVR's proxy doesn't handle WebRTC endpoints
  const port = await getGo2rtcApiPort();
  return `http://${window.location.hostname}:${port}/go2rtc`;
}

/**
 * Get the go2rtc WebSocket URL for MSE streaming.
 *
 * lightNVR's reverse proxy at /go2rtc/* uses curl (HTTP-only) and CANNOT
 * handle WebSocket upgrade requests. MSE streaming requires a WebSocket
 * connection, so it must connect directly to go2rtc, bypassing the proxy.
 *
 * Over HTTP:  ws://hostname:go2rtc_port/go2rtc  (direct to go2rtc)
 * Over HTTPS: wss://hostname/go2rtc              (external reverse proxy handles WS)
 *
 * @returns {Promise<string>} - WebSocket-compatible go2rtc base URL
 */
export async function getGo2rtcWebSocketUrl() {
  // Over HTTPS, the external reverse proxy (nginx, HA ingress, etc.) handles
  // WebSocket upgrades natively and routes /go2rtc/* to go2rtc.
  if (window.location.protocol === 'https:') {
    return `wss://${window.location.host}/go2rtc`;
  }
  // Over HTTP, connect directly to go2rtc's port (bypass lightNVR's HTTP-only proxy)
  const port = await getGo2rtcApiPort();
  return `ws://${window.location.hostname}:${port}/go2rtc`;
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
    web_port: 8080,
    web_auth_enabled: true // Default to auth enabled for safety
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
 * Check if force native HLS is enabled in settings
 * When enabled, the web interface uses lightNVR's native FFmpeg-based HLS
 * instead of go2rtc's HLS, even when go2rtc is running.
 * @returns {Promise<boolean>} - true if force native HLS is enabled
 */
export async function isForceNativeHls() {
  const settings = await getSettings();
  return settings.go2rtc_force_native_hls === true;
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

