/**
 * Request Queue - Limits concurrent requests to prevent server overload
 *
 * This prevents the frontend from overwhelming the backend with too many
 * simultaneous requests (e.g., thumbnail generation via ffmpeg).
 *
 * Instead of returning 503s and implementing retry logic, we control
 * concurrency at the source - the frontend only sends as many requests
 * as the backend can handle at once. Requests are queued with priority
 * levels so visible content loads first.
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
        timestamp: Date.now()
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
      const now = Date.now();
      const timeSinceLastStart = now - this.lastStartTime;
      if (timeSinceLastStart < this.startDelay) {
        // Schedule processing after the delay
        const delay = this.startDelay - timeSinceLastStart;
        this._log(`Delaying next request by ${delay}ms (active: ${this.activeCount}, queued: ${this.queue.length})`);
        setTimeout(() => this._processQueue(), delay);
        return;
      }
      this.lastStartTime = Date.now();
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
 * - maxConcurrent: 4 (conservative to avoid overwhelming ffmpeg)
 * - startDelay: 200ms (stagger request starts to give backend breathing room)
 * - debug: false (disable logging in production)
 */
export const thumbnailQueue = new RequestQueue(4, 200, false);

/**
 * Queue a thumbnail load
 * @param {string} url - Thumbnail URL
 * @param {number} priority - Priority level
 * @returns {Promise<HTMLImageElement>} - Promise that resolves with the loaded image element
 */
export function queueThumbnailLoad(url, priority = Priority.NORMAL) {
  return thumbnailQueue.enqueue(() => {
    return new Promise((resolve, reject) => {
      const img = new Image();

      img.onload = () => {
        // Resolve with the image element to keep it in memory
        // This ensures the browser has actually loaded and cached it
        resolve(img);
      };

      img.onerror = (error) => {
        reject(new Error(`Failed to load thumbnail: ${url}`));
      };

      img.src = url;
    });
  }, priority);
}

