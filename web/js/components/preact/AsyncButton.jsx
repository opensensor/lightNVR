/**
 * AsyncButton — button primitive that wraps any async handler with a
 * consistent pending/success/error UI.
 *
 * Implements PRD UXD_01 §5.1 (task T1): prevents the double-submit bug
 * tracked in #399 by disabling itself while its onClick promise is in
 * flight, replacing the label with a spinner, and surfacing errors inline.
 *
 * Props:
 *   - onClick: (evt) => Promise<any>
 *       Handler returning a promise. Required.
 *   - children: ReactNode
 *       The idle label. Used when `idleLabel` is not provided.
 *   - idleLabel?: ReactNode
 *       Optional explicit idle label; defaults to `children`.
 *   - confirmText?: string
 *       When set, the button becomes a two-step destructive control:
 *       first click swaps the label to `confirmText` and starts a 4 s
 *       commit window; second click within that window invokes onClick.
 *       Clicking anywhere outside the button during the window cancels.
 *   - successLabel?: ReactNode
 *       Label shown briefly (~1.5 s) after a successful resolution before
 *       returning to idle. Defaults to a check icon.
 *   - successDurationMs?: number (default 1500)
 *   - confirmWindowMs?: number (default 4000)
 *   - className, type, disabled, ...rest
 *       All standard <button> props pass through.
 *
 * Behavior:
 *   - While pending: disabled, spinner in place of label, second click
 *     silently dropped.
 *   - On resolve: brief check icon, then back to idle.
 *   - On reject: red border + inline error chip under the button with
 *     error.message (or String(error)).
 */

import { Fragment } from 'preact';
import { useCallback, useEffect, useRef, useState } from 'preact/hooks';
import { useAsyncAction } from '../../hooks/useAsyncAction.js';

/**
 * Inline 16x16 spinner icon, rotated via Tailwind's `animate-spin`.
 * Kept as an SVG so it inherits `currentColor` and doesn't pull a lib.
 */
function SpinnerIcon({ className = 'w-4 h-4' }) {
  return (
    <svg
      class={`${className} animate-spin`}
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      aria-hidden="true"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      ></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      ></path>
    </svg>
  );
}

/**
 * Inline 16x16 check icon for brief success confirmation.
 */
function CheckIcon({ className = 'w-4 h-4' }) {
  return (
    <svg
      class={className}
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      stroke-width="3"
      aria-hidden="true"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M5 13l4 4L19 7"
      />
    </svg>
  );
}

/**
 * Inline warning icon for the error chip.
 */
function WarningIcon({ className = 'w-4 h-4' }) {
  return (
    <svg
      class={className}
      xmlns="http://www.w3.org/2000/svg"
      fill="currentColor"
      viewBox="0 0 20 20"
      aria-hidden="true"
    >
      <path
        fill-rule="evenodd"
        d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
        clip-rule="evenodd"
      />
    </svg>
  );
}

/**
 * AsyncButton component.
 *
 * @param {Object} props
 * @param {(evt: MouseEvent) => Promise<any>} props.onClick
 */
export function AsyncButton({
  onClick,
  children,
  idleLabel,
  confirmText,
  successLabel,
  successDurationMs = 1500,
  confirmWindowMs = 4000,
  className = '',
  disabled = false,
  type = 'button',
  ...rest
}) {
  const { run, pending, error } = useAsyncAction(onClick || (() => Promise.resolve()));

  // Track the brief success flash.
  const [showSuccess, setShowSuccess] = useState(false);
  const successTimerRef = useRef(null);

  // Destructive-action two-step confirm state.
  const [armed, setArmed] = useState(false);
  const armedTimerRef = useRef(null);
  const buttonRef = useRef(null);

  const clearSuccessTimer = () => {
    if (successTimerRef.current) {
      clearTimeout(successTimerRef.current);
      successTimerRef.current = null;
    }
  };
  const clearArmedTimer = () => {
    if (armedTimerRef.current) {
      clearTimeout(armedTimerRef.current);
      armedTimerRef.current = null;
    }
  };

  // Cleanup timers on unmount.
  useEffect(() => () => {
    clearSuccessTimer();
    clearArmedTimer();
  }, []);

  // When armed for a destructive confirm, clicking outside cancels.
  useEffect(() => {
    if (!armed) return undefined;
    const onDocPointer = (evt) => {
      if (buttonRef.current && !buttonRef.current.contains(evt.target)) {
        setArmed(false);
        clearArmedTimer();
      }
    };
    // `mousedown` + `touchstart` covers desktop and mobile cancel paths.
    document.addEventListener('mousedown', onDocPointer, true);
    document.addEventListener('touchstart', onDocPointer, true);
    return () => {
      document.removeEventListener('mousedown', onDocPointer, true);
      document.removeEventListener('touchstart', onDocPointer, true);
    };
  }, [armed]);

  const executeRun = useCallback(async (evt) => {
    try {
      await run(evt);
      clearSuccessTimer();
      setShowSuccess(true);
      successTimerRef.current = setTimeout(() => {
        setShowSuccess(false);
        successTimerRef.current = null;
      }, successDurationMs);
    } catch (_e) {
      // Error is already captured by useAsyncAction; the render path below
      // surfaces it as a chip. Swallow here to avoid unhandled rejection.
      clearSuccessTimer();
      setShowSuccess(false);
    }
  }, [run, successDurationMs]);

  const handleClick = useCallback((evt) => {
    // The idempotency guard in useAsyncAction will silently drop this if
    // another call is already in flight, but an early bail keeps the
    // click-through to the DOM handler tidy.
    if (pending) {
      evt.preventDefault();
      return;
    }

    // Destructive-confirm flow: first click arms, second click commits.
    if (confirmText) {
      if (!armed) {
        setArmed(true);
        clearArmedTimer();
        armedTimerRef.current = setTimeout(() => {
          setArmed(false);
          armedTimerRef.current = null;
        }, confirmWindowMs);
        return;
      }
      // Second click within the window: disarm and commit.
      setArmed(false);
      clearArmedTimer();
    }

    executeRun(evt);
  }, [armed, confirmText, confirmWindowMs, executeRun, pending]);

  // Clear any stale success flash the moment a new pending starts.
  useEffect(() => {
    if (pending) {
      clearSuccessTimer();
      setShowSuccess(false);
    }
  }, [pending]);

  const label = idleLabel !== undefined ? idleLabel : children;
  const isDisabled = disabled || pending;

  // Build the visible content.
  let content;
  if (pending) {
    content = (
      <span class="inline-flex items-center justify-center gap-2">
        <SpinnerIcon />
        <span class="sr-only">Working…</span>
      </span>
    );
  } else if (showSuccess) {
    content = (
      <span class="inline-flex items-center justify-center gap-2">
        {successLabel !== undefined ? successLabel : <CheckIcon />}
      </span>
    );
  } else if (armed && confirmText) {
    content = confirmText;
  } else {
    content = label;
  }

  // Tack on an error-border class when there's a lingering error to surface.
  const errorBorderClass = error
    ? 'border border-[hsl(var(--danger))] ring-1 ring-[hsl(var(--danger))]'
    : '';

  return (
    <Fragment>
      <button
        {...rest}
        ref={buttonRef}
        type={type}
        class={`${className} ${errorBorderClass}`.trim()}
        onClick={handleClick}
        disabled={isDisabled}
        aria-busy={pending || undefined}
        data-pending={pending || undefined}
        data-armed={armed || undefined}
      >
        {content}
      </button>
      {error && (
        <div
          role="alert"
          class="mt-1 inline-flex items-center gap-1 text-xs text-[hsl(var(--danger))]"
          title={error.message || String(error)}
        >
          <WarningIcon className="w-3 h-3" />
          <span class="truncate max-w-xs">
            {error.message || String(error)}
          </span>
        </div>
      )}
    </Fragment>
  );
}

export default AsyncButton;
