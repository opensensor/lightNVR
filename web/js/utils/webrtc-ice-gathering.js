/**
 * Bounded ICE gathering helper for the go2rtc HTTP signaling flow.
 *
 * go2rtc's HTTP endpoint (`POST /api/webrtc?src=...`) has no trickle-ICE
 * channel: the SDP offer is the only place client candidates can travel.
 * Waiting indefinitely for `icegatheringstate === 'complete'` lets an
 * unreachable STUN/TURN server stall the connect for several seconds, so the
 * wait is bounded — the offer goes out with whatever candidates exist when
 * gathering completes or the timeout expires, whichever comes first.
 */

export const DEFAULT_ICE_GATHERING_TIMEOUT_MS = 1000;

export const ICE_GATHER_OUTCOME = {
  COMPLETE: 'complete',
  TIMEOUT: 'timeout',
  ABORTED: 'aborted',
};

/**
 * Wait for a peer connection's ICE gathering to complete, bounded by a
 * timeout.
 *
 * @param {RTCPeerConnection} pc - The peer connection to watch.
 * @param {number} [timeoutMs] - Maximum time to wait before resolving.
 * @param {AbortSignal} [abortSignal] - Optional signal; aborting ends the
 *   wait early (e.g. the cell unmounted or a retry superseded this attempt).
 * @returns {Promise<string>} Resolves with one of ICE_GATHER_OUTCOME values.
 */
export function waitForIceGathering(
  pc,
  timeoutMs = DEFAULT_ICE_GATHERING_TIMEOUT_MS,
  abortSignal = null
) {
  return new Promise((resolve) => {
    if (!pc || pc.iceGatheringState === 'complete') {
      resolve(ICE_GATHER_OUTCOME.COMPLETE);
      return;
    }
    if (timeoutMs <= 0) {
      resolve(ICE_GATHER_OUTCOME.TIMEOUT);
      return;
    }
    if (abortSignal && abortSignal.aborted) {
      resolve(ICE_GATHER_OUTCOME.ABORTED);
      return;
    }

    let settled = false;
    const finish = (outcome) => {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      pc.removeEventListener('icegatheringstatechange', onStateChange);
      if (abortSignal) {
        abortSignal.removeEventListener('abort', onAbort);
      }
      resolve(outcome);
    };
    const onStateChange = () => {
      if (pc.iceGatheringState === 'complete') {
        finish(ICE_GATHER_OUTCOME.COMPLETE);
      }
    };
    const onAbort = () => finish(ICE_GATHER_OUTCOME.ABORTED);

    const timer = setTimeout(() => finish(ICE_GATHER_OUTCOME.TIMEOUT), timeoutMs);
    // Under Node (unit tests) setTimeout returns a Timeout object; unref it so
    // a pending gather bound never keeps the test process alive.
    if (typeof timer === 'object' && typeof timer.unref === 'function') {
      timer.unref();
    }
    pc.addEventListener('icegatheringstatechange', onStateChange);
    if (abortSignal) {
      abortSignal.addEventListener('abort', onAbort);
    }
  });
}
