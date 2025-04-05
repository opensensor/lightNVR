/**
 * LightNVR Web Interface Utility Functions
 * Shared utility functions for Preact components
 */

// Import the static version information
import { VERSION } from '../../version.js';

/**
 * Get system version from the static version file
 * This version is extracted from CMakeLists.txt during the build process
 * @returns {Promise<string>} Promise that resolves to the system version
 */
export async function fetchSystemVersion() {
  // Return the static version from the imported module
  return VERSION;
}
