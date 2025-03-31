/**
 * LightNVR Web Interface Fetch Utilities
 * Enhanced fetch API with timeout, cancellation, and retry capabilities
 */

/**
 * Enhanced fetch function with timeout, cancellation, and retry capabilities
 * @param {string} url - The URL to fetch
 * @param {Object} options - Fetch options
 * @param {number} [options.timeout=30000] - Timeout in milliseconds
 * @param {number} [options.retries=1] - Number of retry attempts
 * @param {number} [options.retryDelay=1000] - Delay between retries in milliseconds
 * @param {AbortSignal} [options.signal] - AbortSignal to cancel the request
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

  // Create a timeout controller if timeout is specified
  const timeoutController = new AbortController();
  let timeoutId;
  
  if (timeout) {
    timeoutId = setTimeout(() => {
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
      const response = await fetch(url, optionsWithSignal);
      
      // Clear the timeout
      if (timeoutId) {
        clearTimeout(timeoutId);
      }
      
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
      
      // If the request was aborted, don't retry
      if (error.name === 'AbortError') {
        if (externalSignal && externalSignal.aborted) {
          throw new Error('Request was cancelled');
        } else {
          throw new Error('Request timed out');
        }
      }
      
      // If this was the last retry, throw the error
      if (attempt >= retries) {
        break;
      }
      
      // Wait before retrying
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
  const response = await enhancedFetch(url, options);
  return response.json();
}
