/**
 * useAsyncAction — shared hook for wrapping async handlers with pending/error
 * state and an idempotency guard.
 *
 * Part of PRD UXD_01 §5.1 (task T1): gives every save/delete/retry in the app
 * a consistent primitive so rapid taps on the same button never fire more than
 * one server call (driving issue #399).
 *
 * Usage:
 *   const { run, pending, error, lastSuccess } = useAsyncAction(async (id) => {
 *     await api.delete(id);
 *   });
 *   ...
 *   <button onClick={() => run(streamId)} disabled={pending}>Delete</button>
 *
 * Semantics:
 *   - `run(...args)` calls `fn(...args)` and tracks its promise.
 *   - A second call to `run` while the previous is still pending is a silent
 *     no-op. It returns the same in-flight promise so callers that await
 *     `run(...)` still get a resolution.
 *   - `pending` is true from the moment `run` is invoked until the promise
 *     settles.
 *   - `error` holds the last rejection (or null on success / idle).
 *   - `lastSuccess` is the timestamp (ms since epoch) of the last successful
 *     resolution, or null if there hasn't been one.
 */

import { useCallback, useEffect, useRef, useState } from 'preact/hooks';

export interface AsyncAction<TArgs extends unknown[], TResult> {
  run: (...args: TArgs) => Promise<TResult>;
  pending: boolean;
  error: Error | null;
  lastSuccess: number | null;
  reset: () => void;
}

/**
 * @param fn - handler to wrap. Usually async; a synchronous return or throw is
 *   handled the same way as a resolved or rejected promise.
 */
export function useAsyncAction<TArgs extends unknown[], TResult>(
  fn: (...args: TArgs) => TResult | PromiseLike<TResult>,
): AsyncAction<TArgs, TResult> {
  const [pending, setPending] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const [lastSuccess, setLastSuccess] = useState<number | null>(null);

  // Keep the in-flight promise in a ref so concurrent `run` calls can share
  // it without triggering a re-render, and so the idempotency check isn't
  // fooled by batched state updates.
  const inFlightRef = useRef<Promise<TResult> | null>(null);
  // Track whether the component is still mounted before committing state.
  const mountedRef = useRef(true);
  // Always call the freshest `fn` captured by the latest render.
  const fnRef = useRef(fn);
  fnRef.current = fn;

  useEffect(() => {
    mountedRef.current = true;
    return () => {
      mountedRef.current = false;
    };
  }, []);

  const run = useCallback((...args: TArgs): Promise<TResult> => {
    // Idempotency: a second invocation while pending is a no-op that returns
    // the original promise so callers can still `await` it.
    if (inFlightRef.current) {
      return inFlightRef.current;
    }

    setPending(true);
    setError(null);

    let promise: Promise<TResult>;
    try {
      const result = fnRef.current(...args);
      promise = Promise.resolve(result) as Promise<TResult>;
    } catch (sync) {
      // Handle synchronous throws from non-async handlers.
      promise = Promise.reject(sync);
    }

    const tracked = promise.then(
      (value) => {
        if (mountedRef.current) {
          setPending(false);
          setLastSuccess(Date.now());
          setError(null);
        }
        inFlightRef.current = null;
        return value;
      },
      (err: unknown) => {
        const normalized = err instanceof Error ? err : new Error(String(err));
        if (mountedRef.current) {
          setPending(false);
          setError(normalized);
        }
        inFlightRef.current = null;
        throw normalized;
      }
    );

    // Assign after wrapping so the idempotency guard is reliable even for
    // synchronously-rejecting handlers.
    inFlightRef.current = tracked;

    // Return the tracked promise directly so callers may `await run(...)`
    // and observe success or failure. We attach a silent catch-handler on
    // a detached branch to prevent unhandledRejection noise when callers
    // fire-and-forget.
    tracked.catch(() => {});
    return tracked;
  }, []);

  const reset = useCallback(() => {
    setPending(false);
    setError(null);
    setLastSuccess(null);
  }, []);

  return { run, pending, error, lastSuccess, reset };
}
