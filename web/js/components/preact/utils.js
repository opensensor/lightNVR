/**
 * LightNVR Web Interface Utility Functions
 * Shared utility functions for Preact components
 */

/**
 * Fetch system version from API
 * @returns {Promise<string>} Promise that resolves to the system version
 */
export async function fetchSystemVersion() {
  try {
    const response = await fetch('/api/system');
    if (!response.ok) {
      throw new Error('Failed to load system information');
    }
    
    const data = await response.json();
    return data.version || '';
  } catch (error) {
    console.error('Error loading system version:', error);
    return '';
  }
}
