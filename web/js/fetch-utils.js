/**
 * LightNVR Web Interface Fetch Utilities
 * Enhanced fetch API with timeout, cancellation, and retry capabilities
 */

/**
 * Enhanced fetch function with timeout, retries and error handling
 * @param {string} url - URL to fetch
 * @param {Object} options - Fetch options
 * @returns {Promise<Response>} - Fetch response
 */
export async function enhancedFetch(url, options = {}) {
  const {
    timeout = 30000,
    retries = 1,
    retryDelay = 1000,
    signal: externalSignal,
    ...fetchOptions
  } = options;

  // Log the request details
  console.log(`enhancedFetch: ${fetchOptions.method || 'GET'} ${url}`);
  console.debug('enhancedFetch options:', {
    timeout,
    retries,
    retryDelay,
    ...fetchOptions
  });

  // Create a timeout controller if timeout is specified
  const timeoutController = new AbortController();
  let timeoutId;

  if (timeout) {
    timeoutId = setTimeout(() => {
      console.warn(`enhancedFetch: Timeout reached for ${url}, aborting request`);
      timeoutController.abort();
    }, timeout);
  }

  // Create a combined signal if an external signal is provided
  const signal = externalSignal
    ? combineSignals(externalSignal, timeoutController.signal)
    : timeoutController.signal;

  // Add the signal to fetch options
  const optionsWithSignal = {
    ...fetchOptions,
    signal
  };

  let lastError;
  let attempt = 0;

  while (attempt <= retries) {
    try {
      console.debug(`enhancedFetch: Attempt ${attempt + 1}/${retries + 1} for ${url}`);
      const response = await fetch(url, optionsWithSignal);

      // Clear the timeout
      if (timeoutId) {
        clearTimeout(timeoutId);
      }

      // Log the response
      console.log(`enhancedFetch response: ${response.status} ${response.statusText} for ${url}`);

      // Check if the response is ok
      if (!response.ok) {
        throw new Error(`HTTP error ${response.status}: ${response.statusText}`);
      }

      return response;
    } catch (error) {
      lastError = error;

      // Clear the timeout
      if (timeoutId) {
        clearTimeout(timeoutId);
      }

      // Log the error
      console.error(`enhancedFetch error (attempt ${attempt + 1}/${retries + 1}):`, error);

      // If the request was aborted, don't retry
      if (error.name === 'AbortError') {
        if (externalSignal && externalSignal.aborted) {
          console.warn(`enhancedFetch: Request was cancelled by external signal for ${url}`);
          throw new Error('Request was cancelled');
        } else {
          console.warn(`enhancedFetch: Request timed out for ${url}`);
          throw new Error('Request timed out');
        }
      }

      // If this was the last retry, throw the error
      if (attempt >= retries) {
        console.error(`enhancedFetch: All ${retries + 1} attempts failed for ${url}`);
        break;
      }

      // Wait before retrying
      console.log(`enhancedFetch: Waiting ${retryDelay}ms before retry ${attempt + 1} for ${url}`);
      await new Promise(resolve => setTimeout(resolve, retryDelay));

      // Reset the timeout for the next attempt
      if (timeout) {
        timeoutController.abort(); // Abort the previous timeout
        const newTimeoutController = new AbortController();
        timeoutId = setTimeout(() => {
          newTimeoutController.abort();
        }, timeout);
      }

      attempt++;
    }
  }

  throw lastError;
}

/**
 * Combine multiple AbortSignals into one
 * @param {...AbortSignal} signals - Signals to combine
 * @returns {AbortSignal} - Combined signal
 */
function combineSignals(...signals) {
  const controller = new AbortController();
  
  const onAbort = () => {
    controller.abort();
    signals.forEach(signal => {
      signal.removeEventListener('abort', onAbort);
    });
  };
  
  signals.forEach(signal => {
    if (signal.aborted) {
      onAbort();
    } else {
      signal.addEventListener('abort', onAbort);
    }
  });
  
  return controller.signal;
}

/**
 * Create a request controller for managing fetch requests
 * @returns {Object} - Request controller object
 */
export function createRequestController() {
  const controller = new AbortController();
  
  return {
    signal: controller.signal,
    abort: () => controller.abort(),
    isAborted: () => controller.signal.aborted
  };
}

/**
 * Fetch JSON data with enhanced fetch
 * @param {string} url - The URL to fetch
 * @param {Object} options - Fetch options
 * @returns {Promise<any>} - Parsed JSON data
 */
export async function fetchJSON(url, options = {}) {
  try {
    const response = await enhancedFetch(url, options);
    console.log(`fetchJSON: Parsing JSON response from ${url}`);
    const data = await response.json();
    return data;
  } catch (error) {
    console.error(`fetchJSON: Error fetching or parsing JSON from ${url}:`, error);
    throw error;
  }
}