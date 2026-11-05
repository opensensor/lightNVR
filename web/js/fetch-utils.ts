/**
 * LightNVR Web Interface Fetch Utilities
 * Enhanced fetch API with timeout, cancellation, and retry capabilities
 */

import type {} from './types/global.d.ts';
import type { ApiErrorBody } from './types/core.ts';

/** Options for enhancedFetch and fetchJSON: fetch's own plus retry behavior. */
export interface EnhancedFetchOptions extends RequestInit {
  /** Per-attempt timeout in milliseconds. A falsy value disables it. */
  timeout?: number;
  retries?: number;
  retryDelay?: number;
  /** Opt out of the redirect to the login page on a 401. */
  skipAuthRedirect?: boolean;
  /** Suppress URL, payload and error logging for sensitive API calls. */
  sensitive?: boolean;
}

/**
 * What the catch block in enhancedFetch reads from a caught value. Only
 * HTTPError has `status`; on other errors it is undefined at runtime, and the
 * comparisons below are then false.
 */
interface FetchFailure {
  status: number;
  message: string;
  name: string;
}

/**
 * Clear authentication state and redirect to login
 * @param {string} reason - Reason for redirect (optional)
 */
export function handleAuthenticationFailure(reason: string = 'Session expired'): void {
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
function isAuthenticationError(error: FetchFailure) {
  // Check if error status is 401 or message contains 401
  return error.status === 401 || (error.message && error.message.includes('401'));
}

/**
 * Custom HTTP error class with status code
 */
class HTTPError extends Error {
  declare status: number;
  declare statusText: string;

  constructor(status: number, statusText: string, message?: string) {
    super(message || `HTTP error ${status}: ${statusText}`);
    this.name = 'HTTPError';
    this.status = status;
    this.statusText = statusText;
  }
}

export type { HTTPError };

/**
 * Enhanced fetch function with timeout, retries and error handling
 * @param {string} url - URL to fetch
 * @param {Object} options - Fetch options
 * @returns {Promise<Response>} - Fetch response
 */
export async function enhancedFetch(url: string, options: EnhancedFetchOptions = {}): Promise<Response> {
  const {
    timeout = 30000,
    retries = 1,
    retryDelay = 1000,
    signal: externalSignal,
    skipAuthRedirect = false, // Allow callers to opt-out of auto-redirect (e.g., login page)
    sensitive = false,
    ...fetchOptions
  } = options;

  // Log the request details
  if (!sensitive) console.log(`enhancedFetch: ${fetchOptions.method || 'GET'} ${url}`);
  if (!sensitive) console.debug('enhancedFetch options:', {
    timeout,
    retries,
    retryDelay,
    skipAuthRedirect,
    ...fetchOptions
  });

  const method = (fetchOptions.method || 'GET').toUpperCase();
  const canRetryTimeout = method === 'GET' || method === 'HEAD';
  let lastError: unknown;

  for (let attempt = 0; attempt <= retries; attempt++) {
    if (externalSignal?.aborted) throw sensitive
      ? new DOMException('Request was cancelled', 'AbortError')
      : new Error('Request was cancelled');

    // Each attempt needs its own timeout and signal. Reusing an aborted
    // controller makes every subsequent attempt fail immediately.
    const timeoutController = new AbortController();
    const signal = externalSignal
      ? combineSignals(externalSignal, timeoutController.signal)
      : timeoutController.signal;
    let completed = false;
    const timeoutId = timeout ? setTimeout(() => {
      if (!sensitive) console.warn(`enhancedFetch: Timeout reached for ${url}, aborting request`);
      timeoutController.abort();
    }, timeout) : null;

    try {
      if (!sensitive) console.debug(`enhancedFetch: Attempt ${attempt + 1}/${retries + 1} for ${url}`);
      const response = await fetch(url, {
        credentials: 'same-origin',
        ...fetchOptions,
        signal
      });

      if (!sensitive) console.log(`enhancedFetch response: ${response.status} ${response.statusText} for ${url}`);

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
          const errorBody = (await response.json()) as ApiErrorBody | null;
          if (errorBody && errorBody.error) errorMessage = errorBody.error;
        } catch (parseError) {
          if (!sensitive) console.debug('Could not parse error response body:', parseError);
        }
        throw new HTTPError(response.status, response.statusText, errorMessage);
      }
      completed = true;
      return response;
    } catch (error) {
      if (!sensitive) console.error(`enhancedFetch error (attempt ${attempt + 1}/${retries + 1}):`, error);
      if (externalSignal?.aborted) {
        if (sensitive && typeof error === 'object' && error !== null && 'name' in error && error.name === 'AbortError') throw error;
        throw sensitive ? new DOMException('Request was cancelled', 'AbortError') : new Error('Request was cancelled');
      }
      if (isAuthenticationError(error as FetchFailure) ||
          ((error as FetchFailure).status >= 400 && (error as FetchFailure).status < 500)) {
        throw error;
      }
      if (timeoutController.signal.aborted) {
        lastError = new Error('Request timed out');
        // A mutation may have completed even when its response never arrived.
        if (!canRetryTimeout) throw lastError;
      } else if ((error as FetchFailure).name === 'AbortError') {
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
      if (!sensitive) console.log(`enhancedFetch: Waiting ${retryDelay}ms before retry ${attempt + 1} for ${url}`);
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
function combineSignals(...signals: AbortSignal[]): AbortSignal {
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
 * @returns {Promise<T>} - Parsed JSON data. T is the caller's claim about the
 *   response; it is not validated. The default of unknown forces narrowing.
 */
export async function fetchJSON<T = unknown>(url: string, options: EnhancedFetchOptions = {}): Promise<T> {
  try {
    const response = await enhancedFetch(url, options);
    if (!options.sensitive) console.log('fetchJSON: Parsing JSON response from', url);
    const data: T = await response.json();
    return data;
  } catch (error) {
    if (!options.sensitive) console.error('fetchJSON: Error fetching or parsing JSON from', url, ':', error);
    throw error;
  }
}
