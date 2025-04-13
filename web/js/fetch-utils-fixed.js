/**
 * LightNVR Web Interface Fetch Utilities (Fixed Version)
 * Enhanced fetch API with timeout, cancellation, and retry capabilities
 * 
 * This version fixes potential race conditions with AbortController
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

  // Log the request details
  console.log(`enhancedFetch: ${fetchOptions.method || 'GET'} ${url}`);
  console.debug('enhancedFetch options:', {
    timeout,
    retries,
    retryDelay,
    ...fetchOptions
  });

  // Create a unique request ID for tracking
  const requestId = Date.now() + '-' + Math.random().toString(36).substr(2, 9);
  console.debug(`enhancedFetch request ID: ${requestId}`);

  let lastError;
  let attempt = 0;
  let aborted = false;

  // Function to check if the request has been aborted
  const checkAborted = () => {
    if (externalSignal && externalSignal.aborted) {
      aborted = true;
      console.warn(`enhancedFetch: Request ${requestId} was cancelled by external signal for ${url}`);
      throw new Error('Request was cancelled');
    }
    return aborted;
  };

  // Check if already aborted before starting
  checkAborted();

  while (attempt <= retries && !aborted) {
    // Create a new timeout controller for each attempt
    const timeoutController = new AbortController();
    let timeoutId;
    
    try {
      console.debug(`enhancedFetch: Attempt ${attempt + 1}/${retries + 1} for ${url} (request ID: ${requestId})`);
      
      // Set up timeout if specified
      if (timeout) {
        timeoutId = setTimeout(() => {
          console.warn(`enhancedFetch: Timeout reached for ${url} (request ID: ${requestId}), aborting request`);
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
      
      // Make the fetch request
      const response = await fetch(url, optionsWithSignal);
      
      // Clear the timeout
      if (timeoutId) {
        clearTimeout(timeoutId);
      }
      
      // Log the response
      console.log(`enhancedFetch response: ${response.status} ${response.statusText} for ${url} (request ID: ${requestId})`);
      
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
      console.error(`enhancedFetch error (attempt ${attempt + 1}/${retries + 1}, request ID: ${requestId}):`, error);
      
      // Check if the request was aborted
      if (error.name === 'AbortError') {
        // Check if it was aborted by the external signal
        if (checkAborted()) {
          throw new Error('Request was cancelled');
        } else {
          console.warn(`enhancedFetch: Request ${requestId} timed out for ${url}`);
          throw new Error('Request timed out');
        }
      }
      
      // If this was the last retry, throw the error
      if (attempt >= retries) {
        console.error(`enhancedFetch: All ${retries + 1} attempts failed for ${url} (request ID: ${requestId})`);
        break;
      }
      
      // Check if aborted before waiting
      checkAborted();
      
      // Wait before retrying
      console.log(`enhancedFetch: Waiting ${retryDelay}ms before retry ${attempt + 1} for ${url} (request ID: ${requestId})`);
      await new Promise(resolve => setTimeout(resolve, retryDelay));
      
      // Check if aborted after waiting
      checkAborted();
      
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
  console.log(`fetchJSON: ${options.method || 'GET'} ${url}`);
  try {
    const response = await enhancedFetch(url, options);
    console.log(`fetchJSON: Parsing JSON response from ${url}`);
    const data = await response.json();
    console.log(`fetchJSON: Successfully parsed JSON from ${url}`);
    return data;
  } catch (error) {
    console.error(`fetchJSON: Error fetching or parsing JSON from ${url}:`, error);
    throw error;
  }
}
