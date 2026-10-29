/**
 * LightNVR Web Interface Fetch Utilities
 * Enhanced fetch API with timeout, cancellation, and retry capabilities
 */

/**
 * Clear authentication state and redirect to login
 * @param {string} reason - Reason for redirect (optional)
 */
export function handleAuthenticationFailure(reason = 'Session expired') {
  console.warn(`Authentication failure: ${reason}`);

  // Clear all auth-related storage
  localStorage.removeItem('auth');

  // Clear non-HttpOnly cookies (session cookie is HttpOnly and can only be
  // cleared server-side via Set-Cookie, but we clear legacy/fallback cookies here)
  document.cookie = "auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Lax";
  document.cookie = "session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Lax";

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

  const method = (fetchOptions.method || 'GET').toUpperCase();
  const canRetryTimeout = method === 'GET' || method === 'HEAD';
  let lastError;

  for (let attempt = 0; attempt <= retries; attempt++) {
    if (externalSignal?.aborted) throw new Error('Request was cancelled');

    // Each attempt needs its own timeout and signal. Reusing an aborted
    // controller makes every subsequent attempt fail immediately.
    const timeoutController = new AbortController();
    const signal = externalSignal
      ? combineSignals(externalSignal, timeoutController.signal)
      : timeoutController.signal;
    let completed = false;
    const timeoutId = timeout ? setTimeout(() => {
      console.warn(`enhancedFetch: Timeout reached for ${url}, aborting request`);
      timeoutController.abort();
    }, timeout) : null;

    try {
      console.debug(`enhancedFetch: Attempt ${attempt + 1}/${retries + 1} for ${url}`);
      const response = await fetch(url, {
        credentials: 'same-origin',
        ...fetchOptions,
        signal
      });

      console.log(`enhancedFetch response: ${response.status} ${response.statusText} for ${url}`);

      if (response.status === 401) {
        if (!skipAuthRedirect && !window._authDisabled && !window._demoMode) {
          handleAuthenticationFailure('Received 401 Unauthorized response');
        }
        throw new HTTPError(401, 'Unauthorized', 'Authentication required');
      }
      if (response.status === 403) {
        throw new HTTPError(403, 'Forbidden', 'Access denied - insufficient privileges');
      }
      if (!response.ok) {
        let errorMessage = response.statusText;
        try {
          const errorBody = await response.json();
          if (errorBody && errorBody.error) errorMessage = errorBody.error;
        } catch (parseError) {
          console.debug('Could not parse error response body:', parseError);
        }
        throw new HTTPError(response.status, response.statusText, errorMessage);
      }
      completed = true;
      return response;
    } catch (error) {
      console.error(`enhancedFetch error (attempt ${attempt + 1}/${retries + 1}):`, error);
      if (externalSignal?.aborted) throw new Error('Request was cancelled');
      if (isAuthenticationError(error) ||
          (error.status >= 400 && error.status < 500)) {
        throw error;
      }
      if (timeoutController.signal.aborted) {
        lastError = new Error('Request timed out');
        // A mutation may have completed even when its response never arrived.
        if (!canRetryTimeout) throw lastError;
      } else if (error.name === 'AbortError') {
        throw error;
      } else {
        lastError = error;
      }
    } finally {
      if (timeoutId !== null) clearTimeout(timeoutId);
      // Failed attempts release their signal listeners. Successful responses
      // still need external cancellation to reach their body readers.
      if (!completed) timeoutController.abort();
    }

    if (attempt < retries) {
      console.log(`enhancedFetch: Waiting ${retryDelay}ms before retry ${attempt + 1} for ${url}`);
      await new Promise(resolve => setTimeout(resolve, retryDelay));
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
  if (typeof AbortSignal.any === 'function') return AbortSignal.any(signals);

  const controller = new AbortController();
  const onAbort = () => {
    controller.abort();
    signals.forEach(signal => signal.removeEventListener('abort', onAbort));
  };
  for (const signal of signals) {
    if (signal.aborted) {
      onAbort();
      break;
    }
    signal.addEventListener('abort', onAbort, { once: true });
  }
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
    console.log('fetchJSON: Parsing JSON response from', url);
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('fetchJSON: Error fetching or parsing JSON from', url, ':', error);
    throw error;
  }
}