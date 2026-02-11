/**
 * LightNVR URL Utilities
 * Helper functions for URL manipulation and credential obfuscation
 */

/**
 * Obfuscate credentials in a URL
 * Replaces username:password with ***:*** in URLs like rtsp://user:pass@host/path
 * 
 * @param {string} url - The URL that may contain credentials
 * @returns {string} - The URL with credentials obfuscated
 * 
 * @example
 * obfuscateUrlCredentials('rtsp://admin:password123@192.168.1.100:554/stream')
 * // Returns: 'rtsp://***:***@192.168.1.100:554/stream'
 * 
 * obfuscateUrlCredentials('rtsp://192.168.1.100:554/stream')
 * // Returns: 'rtsp://192.168.1.100:554/stream' (unchanged)
 */
export function obfuscateUrlCredentials(url) {
  if (!url || typeof url !== 'string') {
    return url;
  }

  // Pattern to match URLs with credentials: protocol://user:pass@rest
  // Captures: (protocol://) (user) (:) (password) (@rest)
  const credentialPattern = /^(\w+:\/\/)([^:@]+):([^@]+)@(.+)$/;
  
  const match = url.match(credentialPattern);
  if (match) {
    // Replace username and password with ***
    return `${match[1]}***:***@${match[4]}`;
  }
  
  // Also handle URLs with only username (no password): protocol://user@rest
  const usernameOnlyPattern = /^(\w+:\/\/)([^:@]+)@(.+)$/;
  const usernameMatch = url.match(usernameOnlyPattern);
  if (usernameMatch) {
    return `${usernameMatch[1]}***@${usernameMatch[3]}`;
  }
  
  // No credentials found, return as-is
  return url;
}

/**
 * Check if a URL contains credentials
 * 
 * @param {string} url - The URL to check
 * @returns {boolean} - True if the URL contains credentials
 */
export function urlHasCredentials(url) {
  if (!url || typeof url !== 'string') {
    return false;
  }
  
  // Check for pattern: protocol://something@rest
  const credentialPattern = /^\w+:\/\/[^@]+@.+$/;
  return credentialPattern.test(url);
}

/**
 * Check if credentials should be hidden based on user role and demo mode
 * 
 * @param {string} userRole - The user's role ('admin', 'user', 'viewer')
 * @param {boolean} isDemoMode - Whether demo mode is enabled
 * @returns {boolean} - True if credentials should be hidden
 */
export function shouldHideCredentials(userRole, isDemoMode) {
  // Hide credentials in demo mode or for viewer role
  return isDemoMode === true || userRole === 'viewer';
}

/**
 * Conditionally obfuscate URL credentials based on user role and demo mode
 * 
 * @param {string} url - The URL that may contain credentials
 * @param {string} userRole - The user's role ('admin', 'user', 'viewer')
 * @param {boolean} isDemoMode - Whether demo mode is enabled
 * @returns {string} - The URL, obfuscated if necessary
 */
export function conditionallyObfuscateUrl(url, userRole, isDemoMode) {
  if (shouldHideCredentials(userRole, isDemoMode)) {
    return obfuscateUrlCredentials(url);
  }
  return url;
}

