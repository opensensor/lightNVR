/**
 * SystemUtils
 * Utility functions for the system components
 */

/**
 * Format bytes to human-readable size
 * 
 * @param {number} bytes Number of bytes
 * @param {number} decimals Number of decimal places
 * @returns {string} Formatted size string
 */
export function formatBytes(bytes, decimals = 1) {
  if (bytes === 0) return '0 Bytes';
  
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
  
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

/**
 * Format uptime in seconds to a human-readable string
 * 
 * @param {number} seconds Uptime in seconds
 * @returns {string} Formatted uptime string
 */
export function formatUptime(seconds) {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  
  let result = '';
  if (days > 0) result += `${days}d `;
  if (hours > 0 || days > 0) result += `${hours}h `;
  if (minutes > 0 || hours > 0 || days > 0) result += `${minutes}m `;
  result += `${secs}s`;
  
  return result;
}

/**
 * Check if a log level meets the minimum required level
 * This is a JavaScript implementation of the same logic used in the backend
 * 
 * @param {string} logLevel The log level to check
 * @param {string} minLevel The minimum required level
 * @returns {boolean} True if the log level meets the minimum, false otherwise
 */
export function log_level_meets_minimum(logLevel, minLevel) {
  // Special case: if minLevel is debug, always return true
  // This ensures all log levels are shown when debug is selected
  if (String(minLevel || '').toLowerCase() === 'debug') {
    return true;
  }
  
  // Convert log levels to numeric values for comparison
  let levelValue = 2; // Default to INFO (2)
  let minValue = 2;   // Default to INFO (2)
  
  // Map log level strings to numeric values
  // ERROR = 0, WARNING = 1, INFO = 2, DEBUG = 3
  const logLevelLower = String(logLevel || '').toLowerCase();
  const minLevelLower = String(minLevel || '').toLowerCase();
  
  if (logLevelLower === 'error') {
    levelValue = 0;
  } else if (logLevelLower === 'warning' || logLevelLower === 'warn') {
    levelValue = 1;
  } else if (logLevelLower === 'info') {
    levelValue = 2;
  } else if (logLevelLower === 'debug') {
    levelValue = 3;
  }
  
  if (minLevelLower === 'error') {
    minValue = 0;
  } else if (minLevelLower === 'warning' || minLevelLower === 'warn') {
    minValue = 1;
  } else if (minLevelLower === 'info') {
    minValue = 2;
  } else if (minLevelLower === 'debug') {
    minValue = 3;
  }
  
  // Return true if the log level is less than or equal to the minimum level
  // Lower values are higher priority (ERROR = 0 is highest priority)
  // We want to include all levels with higher or equal priority to the minimum
  return levelValue <= minValue;
}
