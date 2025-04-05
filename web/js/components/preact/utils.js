/**
 * LightNVR Web Interface Utility Functions
 * Shared utility functions for Preact components
 */

/**
 * Fetch system version from API with timeout and retry
 * @param {number} [timeout=3000] - Timeout in milliseconds
 * @param {number} [retries=1] - Number of retries if the request fails
 * @returns {Promise<string>} Promise that resolves to the system version
 */
export async function fetchSystemVersion(timeout = 3000, retries = 1) {
  // Create an AbortController for the timeout
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  try {
    // Try to fetch with a timeout
    const response = await fetch('/api/system', {
      signal: controller.signal
    });

    // Clear the timeout since we got a response
    clearTimeout(timeoutId);

    if (!response.ok) {
      throw new Error(`Failed to load system information: ${response.status}`);
    }

    const data = await response.json();
    return data.version || '';
  } catch (error) {
    // Clear the timeout if there was an error
    clearTimeout(timeoutId);

    console.error(`Error loading system version (attempts left: ${retries}):`, error);

    // If we have retries left, try again
    if (retries > 0) {
      console.log(`Retrying system version fetch (${retries} attempts left)...`);
      return fetchSystemVersion(timeout, retries - 1);
    }

    // If we're out of retries, return an empty string
    return '';
  }
}
