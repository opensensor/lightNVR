/**
 * Request Queue - Limits concurrent requests to prevent server overload
 *
 * This prevents the frontend from overwhelming the backend with too many
 * simultaneous requests (e.g., thumbnail generation via ffmpeg).
 *
 * We control concurrency at the source so the frontend only sends as many
 * requests as the backend can handle at once. Transient capacity responses
 * are retried with backoff and visible content receives priority.
 *
 * Usage:
 *   import { queueThumbnailLoad, Priority } from './request-queue.js';
 *
 *   // High priority for visible content
 *   queueThumbnailLoad(url, Priority.HIGH)
 *     .then(() => console.log('Loaded!'))
 *     .catch(err => console.error('Failed:', err));
 *
 *   // Low priority for preloading
 *   queueThumbnailLoad(url, Priority.LOW).catch(() => {});
 */

import { nowMilliseconds } from './utils/date-utils.js';

/**
 * Priority levels for queued requests
 */
export const Priority = {
  HIGH: 0,    // Visible content, user-initiated
  NORMAL: 1,  // Default priority
  LOW: 2      // Preloading, background tasks
};

/**
 * Request Queue class
 */
export class RequestQueue {
  constructor(maxConcurrent = 4, startDelay = 0, debug = false) {
    this.maxConcurrent = maxConcurrent;
    this.activeCount = 0;
    this.queue = [];
    this.startDelay = startDelay; // Delay in ms between starting requests
    this.lastStartTime = 0;
    this.debug = debug;
    this.requestCounter = 0;
  }

  _log(...args) {
    if (this.debug) {
      console.log('[RequestQueue]', ...args);
    }
  }

  /**
   * Add a request to the queue
   * @param {Function} requestFn - Function that returns a Promise
   * @param {number} priority - Priority level (lower = higher priority)
   * @returns {Promise} - Promise that resolves when request completes
   */
  enqueue(requestFn, priority = Priority.NORMAL) {
    return new Promise((resolve, reject) => {
      const item = {
        requestFn,
        priority,
        resolve,
        reject,
        timestamp: nowMilliseconds()
      };

      // Insert into queue based on priority (and timestamp for same priority)
      const insertIndex = this.queue.findIndex(
        q => q.priority > priority || (q.priority === priority && q.timestamp > item.timestamp)
      );
      
      if (insertIndex === -1) {
        this.queue.push(item);
      } else {
        this.queue.splice(insertIndex, 0, item);
      }

      this._processQueue();
    });
  }

  /**
   * Process the queue - start requests up to maxConcurrent
   */
  _processQueue() {
    // Don't process if already at max concurrency or queue is empty
    if (this.activeCount >= this.maxConcurrent || this.queue.length === 0) {
      return;
    }

    // Apply start delay to prevent overwhelming the server
    if (this.startDelay > 0) {
      const now = nowMilliseconds();
      const timeSinceLastStart = now - this.lastStartTime;
      if (timeSinceLastStart < this.startDelay) {
        // Schedule processing after the delay
        const delay = this.startDelay - timeSinceLastStart;
        this._log(`Delaying next request by ${delay}ms (active: ${this.activeCount}, queued: ${this.queue.length})`);
        setTimeout(() => this._processQueue(), delay);
        return;
      }
      this.lastStartTime = nowMilliseconds();
    }

    // Start the next request
    const item = this.queue.shift();
    const requestId = ++this.requestCounter;
    this.activeCount++;

    this._log(`Starting request #${requestId} (active: ${this.activeCount}, queued: ${this.queue.length})`);

    // Execute the request
    item.requestFn()
      .then(result => {
        this._log(`Request #${requestId} completed successfully`);
        item.resolve(result);
      })
      .catch(error => {
        this._log(`Request #${requestId} failed:`, error.message);
        item.reject(error);
      })
      .finally(() => {
        this.activeCount--;
        this._log(`Request #${requestId} finished (active: ${this.activeCount}, queued: ${this.queue.length})`);
        this._processQueue();
      });

    // Try to start another request if we have capacity
    this._processQueue();
  }

  /**
   * Get queue statistics
   */
  getStats() {
    return {
      active: this.activeCount,
      queued: this.queue.length,
      total: this.activeCount + this.queue.length
    };
  }

  /**
   * Clear all pending requests
   */
  clear() {
    this.queue.forEach(item => {
      item.reject(new Error('Queue cleared'));
    });
    this.queue = [];
  }
}

/**
 * Global thumbnail request queue
 * Limits concurrent thumbnail generation to prevent overwhelming the server
 *
 * Configuration:
 * - maxConcurrent: 2 (stay below the backend's four-worker thumbnail limit
 *   so other clients retain generation capacity)
 * - startDelay: 100ms (stagger requests to avoid burst overload)
 * - debug: false (disable logging in production)
 */
export const thumbnailQueue = new RequestQueue(2, 100, false);

const loadedThumbnailUrls = new Set();
const pendingThumbnailLoads = new Map();
const MAX_REMEMBERED_THUMBNAILS = 512;
let thumbnailQueueGeneration = 0;

function rememberLoadedThumbnail(url) {
  loadedThumbnailUrls.delete(url);
  loadedThumbnailUrls.add(url);
  if (loadedThumbnailUrls.size > MAX_REMEMBERED_THUMBNAILS) {
    loadedThumbnailUrls.delete(loadedThumbnailUrls.values().next().value);
  }
}

/**
 * Load a thumbnail through fetch so HTTP status and Retry-After remain
 * observable. The successful response is stored in the browser HTTP cache;
 * rendering the card's img element then reuses that response.
 */
async function loadImage(url) {
  const response = await fetch(url, { credentials: 'same-origin' });
  if (!response.ok) {
    const error = new Error(`Failed to load thumbnail (${response.status}): ${url}`);
    error.status = response.status;
    error.retryAfter = response.headers?.get?.('Retry-After') || null;
    throw error;
  }
  await response.blob();
  return url;
}

export function parseRetryAfterMilliseconds(value, now = Date.now()) {
  if (!value) return null;
  const seconds = Number(value);
  if (Number.isFinite(seconds) && seconds >= 0) return seconds * 1000;

  const retryAt = Date.parse(value);
  if (!Number.isFinite(retryAt)) return null;
  return Math.max(0, retryAt - now);
}

export function shouldRetryThumbnailRequest(error) {
  if (!error?.status) return true;
  return [408, 429, 502, 503, 504].includes(error.status);
}

/**
 * Queue a thumbnail load with smart caching and automatic retry.
 *
 * On transient failures (server busy / 503, ffmpeg timeout, nginx gateway
 * timeout, etc.) the request is re-queued up to {@link maxRetries} times
 * with exponential back-off so that thumbnails generated by earlier
 * requests have time to land on disk and can be served from cache on the
 * next attempt.
 *
 * @param {string}  url        - Thumbnail URL
 * @param {number}  priority   - Priority level
 * @param {number}  maxRetries - Maximum number of retry attempts (default 3)
 * @returns {Promise<string>}  - Promise that resolves with the URL when loaded
 */
export function queueThumbnailLoad(url, priority = Priority.NORMAL, maxRetries = 3) {
  if (loadedThumbnailUrls.has(url)) {
    return Promise.resolve(url);
  }
  if (pendingThumbnailLoads.has(url)) {
    return pendingThumbnailLoads.get(url);
  }

  let attempt = 0;
  const generation = thumbnailQueueGeneration;

  const tryLoad = () => {
    if (generation !== thumbnailQueueGeneration) {
      return Promise.reject(new Error('Thumbnail queue cleared'));
    }
    return thumbnailQueue.enqueue(() => loadImage(url), priority);
  };

  const retryWithBackoff = (err) => {
    attempt++;
    if (generation !== thumbnailQueueGeneration ||
        attempt > maxRetries || !shouldRetryThumbnailRequest(err)) {
      return Promise.reject(err);
    }
    const serverDelay = parseRetryAfterMilliseconds(err.retryAfter);
    const delay = serverDelay ?? Math.min(2000 * Math.pow(2, attempt - 1), 10000);
    return new Promise((resolve) => setTimeout(resolve, delay)).then(tryLoad).catch(retryWithBackoff);
  };

  const pending = tryLoad()
    .catch(retryWithBackoff)
    .then((loadedUrl) => {
      rememberLoadedThumbnail(loadedUrl);
      return loadedUrl;
    })
    .finally(() => {
      if (pendingThumbnailLoads.get(url) === pending) {
        pendingThumbnailLoads.delete(url);
      }
    });
  pendingThumbnailLoads.set(url, pending);
  return pending;
}

/** Forget a successful preload after the rendered img reports a cache miss. */
export function invalidateThumbnailLoad(url) {
  loadedThumbnailUrls.delete(url);
}

/**
 * Clear the thumbnail queue (call when navigating away from recordings page)
 */
export function clearThumbnailQueue() {
  thumbnailQueueGeneration++;
  thumbnailQueue.clear();
  pendingThumbnailLoads.clear();
}
