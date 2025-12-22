/**
 * LightNVR Web Interface Fetch Utilities
 * Enhanced fetch API with timeout, cancellation, and retry capabilities
 */

/**
 * Clear authentication state and redirect to login
 * @param {string} reason - Reason for redirect (optional)
 */
function handleAuthenticationFailure(reason = 'Session expired') {
  console.warn(`Authentication failure: ${reason}`);

  // Clear all auth-related storage
  localStorage.removeItem('auth');

  // Clear auth cookies
  document.cookie = "auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict";
  document.cookie = "session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict";

  // Only redirect if we're not already on the login page
  if (!window.location.pathname.includes('login.html')) {
    console.log('Redirecting to login page due to authentication failure');
    window.location.href = '/login.html?auth_required=true&reason=session_expired';
  }
}

/**
 * Check if an error is an authentication error that should trigger redirect
 * @param {Error} error - The error to check
 * @returns {boolean} - True if this is an auth error
 */
function isAuthenticationError(error) {
  // Check if error status is 401 or message contains 401
  return error.status === 401 || (error.message && error.message.includes('401'));
}

/**
 * Custom HTTP error class with status code
 */
class HTTPError extends Error {
  constructor(status, statusText, message) {
    super(message || `HTTP error ${status}: ${statusText}`);
    this.name = 'HTTPError';
    this.status = status;
    this.statusText = statusText;
  }
}

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
    skipAuthRedirect = false, // Allow callers to opt-out of auto-redirect (e.g., login page)
    ...fetchOptions
  } = options;

  // Log the request details
  console.log(`enhancedFetch: ${fetchOptions.method || 'GET'} ${url}`);
  console.debug('enhancedFetch options:', {
    timeout,
    retries,
    retryDelay,
    skipAuthRedirect,
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

  // Add the signal and credentials to fetch options
  const optionsWithSignal = {
    credentials: 'same-origin', // Include cookies in same-origin requests
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

      // Handle 401 Unauthorized - don't retry, just redirect
      if (response.status === 401) {
        if (!skipAuthRedirect) {
          handleAuthenticationFailure('Received 401 Unauthorized response');
        }
        throw new HTTPError(401, 'Unauthorized', 'Authentication required');
      }

      // Handle 403 Forbidden - access denied, don't redirect but throw with status
      if (response.status === 403) {
        throw new HTTPError(403, 'Forbidden', 'Access denied - insufficient privileges');
      }

      // Check if the response is ok - try to get error message from response body
      if (!response.ok) {
        let errorMessage = response.statusText;
        try {
          const errorBody = await response.json();
          if (errorBody && errorBody.error) {
            errorMessage = errorBody.error;
          }
        } catch (parseError) {
          // If we can't parse the response, just use the status text
          console.debug('Could not parse error response body:', parseError);
        }
        throw new HTTPError(response.status, response.statusText, errorMessage);
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

      // If this is an authentication error, don't retry - just fail immediately
      if (isAuthenticationError(error)) {
        console.warn(`enhancedFetch: Authentication error detected, not retrying`);
        throw error;
      }

      // If this is a client error (4xx), don't retry - it will fail the same way
      if (error.status && error.status >= 400 && error.status < 500) {
        console.warn(`enhancedFetch: Client error ${error.status} detected, not retrying`);
        throw error;
      }

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