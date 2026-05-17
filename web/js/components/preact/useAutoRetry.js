/**
 * useAutoRetry — schedules an automatic retry while a player is in its
 * error state, so unattended monitoring dashboards (kiosk mode, wall
 * displays) recover from transient drops without manual intervention.
 *
 * Behavior:
 *   - When `error` becomes truthy, schedule `onRetry()` to fire after
 *     AUTO_RETRY_DELAY_MS + a small per-camera jitter.  The jitter desyncs
 *     simultaneous retries across a grid of cells so we don't slam the
 *     refresh API with N parallel calls at exactly the same instant.
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

const AUTO_RETRY_DELAY_MS = 5000;
const AUTO_RETRY_JITTER_MS = 1000;
const COUNTDOWN_TICK_MS = 250;

export function useAutoRetry(error, onRetry) {
  const [countdownSeconds, setCountdownSeconds] = useState(null);

  const onRetryRef = useRef(onRetry);
  onRetryRef.current = onRetry;

  useEffect(() => {
    if (!error) {
      setCountdownSeconds(null);
      return undefined;
    }

    const totalMs = AUTO_RETRY_DELAY_MS + Math.floor(Math.random() * AUTO_RETRY_JITTER_MS);
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
