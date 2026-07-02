/**
 * useAutoRetry — schedules an automatic retry while a player is in its
 * error state, so unattended monitoring dashboards (kiosk mode, wall
 * displays) recover from transient drops without manual intervention.
 *
 * Behavior:
 *   - When `error` becomes truthy, schedule `onRetry()` to fire after a
 *     backoff delay + a small per-camera jitter.  The jitter desyncs
 *     simultaneous retries across a grid of cells so we don't slam the
 *     refresh API with N parallel calls at exactly the same instant.
 *   - The delay escalates with consecutive failures (5s, 10s, 20s, 40s,
 *     capped at 60s) so a persistently offline camera settles into a slow
 *     poll instead of hammering go2rtc — and hogging shared connection-gate
 *     slots — every few seconds forever.  The ladder resets once the error
 *     stays cleared for AUTO_RETRY_RESET_AFTER_MS (i.e. the stream really
 *     recovered, not just a retry attempt clearing the error state).
 *   - While the timer is pending, return the seconds-remaining so the
 *     button can show "Retry in 5s..." countdown UX.
 *   - When `error` returns to falsy (manual retry, recovery, etc.),
 *     cancel everything and return null.
 *
 * This hook deliberately does NOT bound the number of retries — that's
 * the whole point.  Each player's own internal logic already attempts a
 * few in-band recoveries before raising `error`; by the time we get
 * here, the user-visible "click retry" gate is the only thing keeping
 * unattended displays from coming back on their own.
 */

import { useEffect, useRef, useState } from 'preact/hooks';

const AUTO_RETRY_BASE_DELAY_MS = 5000;
const AUTO_RETRY_MAX_DELAY_MS = 60000;
const AUTO_RETRY_JITTER_MS = 1000;
const AUTO_RETRY_RESET_AFTER_MS = 30000;
const COUNTDOWN_TICK_MS = 250;

export function useAutoRetry(error, onRetry) {
  const [countdownSeconds, setCountdownSeconds] = useState(null);

  const onRetryRef = useRef(onRetry);
  onRetryRef.current = onRetry;

  // Consecutive error episodes since the last sustained recovery
  const attemptRef = useRef(0);

  useEffect(() => {
    if (!error) {
      setCountdownSeconds(null);
      // Error cleared — if it stays cleared long enough, consider the
      // stream recovered and reset the backoff ladder. A retry attempt
      // that fails again re-raises `error` before this fires.
      const resetTimer = setTimeout(() => {
        attemptRef.current = 0;
      }, AUTO_RETRY_RESET_AFTER_MS);
      return () => clearTimeout(resetTimer);
    }

    const attempt = attemptRef.current;
    attemptRef.current = attempt + 1;

    const backoffMs = Math.min(
      AUTO_RETRY_BASE_DELAY_MS * Math.pow(2, attempt),
      AUTO_RETRY_MAX_DELAY_MS
    );
    const totalMs = backoffMs + Math.floor(Math.random() * AUTO_RETRY_JITTER_MS);
    const startedAt = Date.now();
    setCountdownSeconds(Math.ceil(totalMs / 1000));

    const tickInterval = setInterval(() => {
      const remainingMs = totalMs - (Date.now() - startedAt);
      if (remainingMs <= 0) {
        clearInterval(tickInterval);
        setCountdownSeconds(0);
      } else {
        setCountdownSeconds(Math.ceil(remainingMs / 1000));
      }
    }, COUNTDOWN_TICK_MS);

    const retryTimer = setTimeout(() => {
      const fn = onRetryRef.current;
      if (typeof fn === 'function') {
        try {
          fn();
        } catch (err) {
          console.error('useAutoRetry: onRetry threw', err);
        }
      }
    }, totalMs);

    return () => {
      clearInterval(tickInterval);
      clearTimeout(retryTimer);
    };
  }, [error]);

  return countdownSeconds;
}
