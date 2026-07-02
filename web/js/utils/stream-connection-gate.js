/**
 * Stream Connection Gate — bounds concurrent stream connection attempts.
 *
 * Replaces the old fixed per-index init stagger (index * 200-300ms) in the
 * live view grids. The stagger only shaped the initial burst; it did nothing
 * once an offline camera's connection attempt started hanging. go2rtc holds
 * the HTTP request (WebRTC offer, stream.m3u8) open while it dials the
 * camera's RTSP source, so an unreachable camera pinned one of the browser's
 * ~6 per-host HTTP/1.1 connections (and a lightNVR proxy slot) for tens of
 * seconds — starving every other camera's requests.
 *
 * This gate enforces the burst ceiling structurally instead of by timing:
 *   - At most `maxConcurrent` connection attempts run at once, page-wide.
 *     Healthy cameras complete in well under a second, so slots recycle fast
 *     and the grid loads faster than the old stagger.
 *   - Every attempt gets a hard timeout. An offline camera can hold a slot
 *     for at most `attemptTimeoutMs`, then it fails fast and the slot frees.
 *   - Priority ordering lets cells whose stream the backend already reports
 *     as unhealthy queue behind cameras that are expected to connect.
 *
 * Usage:
 *   import { streamConnectionGate, GatePriority, isGateTimeout } from '.../stream-connection-gate.js';
 *
 *   const response = await streamConnectionGate.run(
 *     (signal) => fetch(url, { signal }),
 *     { priority: GatePriority.NORMAL, timeoutMs: 10000, signal: cellSignal }
 *   );
 */

export const GatePriority = {
  HIGH: 0,    // user-initiated (manual retry, single-stream view)
  NORMAL: 1,  // streams the backend reports healthy
  LOW: 2      // streams already known to be offline/reconnecting
};

/** Error name used when an attempt is killed by the gate's timeout. */
const GATE_TIMEOUT_NAME = 'GateTimeoutError';

/**
 * True when the given error came from the gate's per-attempt timeout
 * (as opposed to a component-unmount abort or a genuine network error).
 * @param {Error} err
 * @returns {boolean}
 */
export function isGateTimeout(err) {
  return !!err && err.name === GATE_TIMEOUT_NAME;
}

/**
 * True when the given error is an abort caused by the caller's own signal
 * (component unmounted / attempt superseded) — callers usually want to
 * silently bail out instead of surfacing an error state.
 * @param {Error} err
 * @returns {boolean}
 */
export function isGateAbort(err) {
  return !!err && err.name === 'AbortError';
}

function makeAbortError() {
  return new DOMException('Connection attempt aborted', 'AbortError');
}

export class StreamConnectionGate {
  constructor(maxConcurrent = 6, attemptTimeoutMs = 10000) {
    this.maxConcurrent = maxConcurrent;
    this.attemptTimeoutMs = attemptTimeoutMs;
    this.activeCount = 0;
    this.queue = [];
    this._seq = 0;
  }

  /**
   * Run `fn` when a slot is available. `fn` receives an AbortSignal that
   * fires on timeout or when the caller's own `signal` aborts — fn MUST
   * wire it into its fetch/WebSocket so the slot actually frees.
   *
   * @param {(signal: AbortSignal) => Promise<any>} fn
   * @param {Object} [opts]
   * @param {number} [opts.priority] - GatePriority level
   * @param {number} [opts.timeoutMs] - per-attempt timeout override
   * @param {AbortSignal} [opts.signal] - caller signal; aborting it cancels
   *   the attempt whether it is still queued or already running
   * @returns {Promise<any>} resolves/rejects with fn's result; rejects with
   *   a GateTimeoutError-named error on timeout, AbortError on caller abort
   */
  run(fn, { priority = GatePriority.NORMAL, timeoutMs = this.attemptTimeoutMs, signal } = {}) {
    return new Promise((resolve, reject) => {
      if (signal && signal.aborted) {
        reject(makeAbortError());
        return;
      }

      const item = {
        fn, priority, timeoutMs, signal, resolve, reject,
        seq: this._seq++,
        controller: null,
        timedOut: false,
        onAbort: null
      };

      if (signal) {
        item.onAbort = () => this._cancel(item);
        signal.addEventListener('abort', item.onAbort, { once: true });
      }

      // Insert by priority, FIFO within the same priority
      const insertIndex = this.queue.findIndex(q => q.priority > priority);
      if (insertIndex === -1) {
        this.queue.push(item);
      } else {
        this.queue.splice(insertIndex, 0, item);
      }

      this._pump();
    });
  }

  /** Cancel an item: drop it if queued, abort it if running. */
  _cancel(item) {
    const idx = this.queue.indexOf(item);
    if (idx !== -1) {
      this.queue.splice(idx, 1);
      item.reject(makeAbortError());
      return;
    }
    if (item.controller) {
      item.controller.abort();
    }
  }

  _pump() {
    while (this.activeCount < this.maxConcurrent && this.queue.length > 0) {
      const item = this.queue.shift();
      this.activeCount++;

      const controller = new AbortController();
      item.controller = controller;

      const timeoutTimer = setTimeout(() => {
        item.timedOut = true;
        controller.abort();
      }, item.timeoutMs);

      Promise.resolve()
        .then(() => item.fn(controller.signal))
        .then(
          result => item.resolve(result),
          err => {
            if (item.timedOut) {
              const timeoutErr = new Error(`Connection attempt timed out after ${item.timeoutMs}ms`);
              timeoutErr.name = GATE_TIMEOUT_NAME;
              item.reject(timeoutErr);
            } else {
              item.reject(err);
            }
          }
        )
        .finally(() => {
          clearTimeout(timeoutTimer);
          if (item.signal && item.onAbort) {
            item.signal.removeEventListener('abort', item.onAbort);
          }
          this.activeCount--;
          this._pump();
        });
    }
  }

  getStats() {
    return { active: this.activeCount, queued: this.queue.length };
  }
}

/**
 * Page-wide singleton shared by all live-view cells (WebRTC offers, HLS
 * manifest preflights, MSE WebSocket setup). 6 concurrent attempts keeps a
 * healthy 36-cell grid loading in a few seconds while never letting offline
 * cameras occupy more than a fraction of the browser's per-host connections.
 */
export const streamConnectionGate = new StreamConnectionGate(6, 10000);

/**
 * Map a backend stream status to a gate priority: streams the backend
 * already reports as unreachable queue behind streams expected to connect.
 * @param {string} status - stream.status from /api/streams ('Running', 'Error', ...)
 * @returns {number} GatePriority level
 */
export function priorityForStreamStatus(status) {
  if (status === 'Error' || status === 'Reconnecting') {
    return GatePriority.LOW;
  }
  return GatePriority.NORMAL;
}
