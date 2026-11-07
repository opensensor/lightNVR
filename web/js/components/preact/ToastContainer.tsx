/**
 * LightNVR Web Interface Toast Container Component
 * Preact component for displaying toast notifications
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { createPortal } from 'preact/compat';
import { formatLocalTime } from '../../utils/date-utils.ts';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

interface ToastEntry {
  id: number;
  message: string;
  type: string;
  duration: number;
}

// Toast helpers are also published on window, here and by Toast.tsx.
declare global {
  interface Window {
    showSuccessToast?: (message: string, duration?: number) => unknown;
    showErrorToast?: (message: string, duration?: number) => unknown;
    showWarningToast?: (message: string, duration?: number) => unknown;
    showInfoToast?: (message: string, duration?: number) => unknown;
    showToast?: (message: string, type?: string, duration?: number) => unknown;
    showStatusMessage?: (message: string, type?: string, duration?: number) => unknown;
    testToast?: (type?: string) => string;
  }
}

// Global toast state
let toastQueue: ToastEntry[] = [];
let toastId = 0;
let setToastsFunction: ((toasts: ToastEntry[]) => void) | null = null;

/**
 * Add a toast to the queue
 * @param {unknown} message - Message to display; stringified, with a fallback for null/undefined
 * @param {string} type - Toast type ('success', 'error', 'warning', 'info'); other values render as info
 * @param {number} duration - Duration in milliseconds
 */
export function addToast(message: unknown, type: string = 'info', duration: number = 4000) {
  // Ensure message is a string and handle undefined/null values
  const safeMessage = message !== undefined && message !== null ? String(message) : 'Operation completed';

  console.log('Adding toast to queue:', safeMessage);

  // Create a new toast
  const toast = {
    id: toastId++,
    message: safeMessage,
    type,
    duration
  };

  // Add to queue
  toastQueue = [...toastQueue, toast];

  // Update state if the setter function is available
  if (setToastsFunction) {
    setToastsFunction(toastQueue);
  }

  // Remove after duration
  setTimeout(() => {
    removeToast(toast.id);
  }, duration);
}

/**
 * Remove a toast from the queue
 * @param {number} id - Toast ID
 */
export function removeToast(id: number) {
  console.log('Removing toast from queue:', id);

  // Remove from queue
  toastQueue = toastQueue.filter(toast => toast.id !== id);

  // Update state if the setter function is available
  if (setToastsFunction) {
    setToastsFunction(toastQueue);
  }
}

/**
 * Toast Container Component
 * @returns {JSX.Element} Toast Container Component
 */
export function ToastContainer() {
  const [toasts, setToasts] = useState(toastQueue);
  const containerRef = useRef<HTMLDivElement>(null);

  // Store the setter function in the global variable
  useEffect(() => {
    setToastsFunction = setToasts;

    // Clean up
    return () => {
      setToastsFunction = null;
    };
  }, []);

  // Create portal for the toast container.
  // Positioning (PRD UXD_01 §5.3):
  //   - < 640px (mobile):   top-center — never overlaps the bottom nav.
  //   - ≥ 640px (desktop):  top-right.
  //   - pt-[env(safe-area-inset-top)] keeps toasts clear of the iOS notch.
  return createPortal(
    <div
      ref={containerRef}
      className="fixed top-4 left-1/2 -translate-x-1/2 sm:top-4 sm:inset-e-4 sm:inset-s-auto sm:translate-x-0 pt-[env(safe-area-inset-top)] app-toast-layer flex flex-col items-center sm:items-end gap-2.5 w-full max-w-[450px] px-4 sm:px-0 pointer-events-none"
    >
      {toasts.map(toast => {
        const getToastStyles = (type: string) => {
          const baseStyles = 'py-3.5 px-4.5 rounded-lg shadow-lg w-full flex items-center pointer-events-auto font-medium border-s-[6px]';
          const typeStyles = {
            success: { bg: 'hsl(var(--success))', fg: 'hsl(var(--success-foreground))', border: 'hsl(var(--success) / 0.8)' },
            error: { bg: 'hsl(var(--danger))', fg: 'hsl(var(--danger-foreground))', border: 'hsl(var(--danger) / 0.8)' },
            warning: { bg: 'hsl(var(--warning))', fg: 'hsl(var(--warning-foreground))', border: 'hsl(var(--warning) / 0.8)' },
            info: { bg: 'hsl(var(--info))', fg: 'hsl(var(--info-foreground))', border: 'hsl(var(--info) / 0.8)' }
          };
          return { baseStyles, ...typeStyles[type as ToastType] || typeStyles.info };
        };

        const styles = getToastStyles(toast.type);

        return (
          <div /* eslint-disable-line jsx-a11y/click-events-have-key-events, jsx-a11y/no-static-element-interactions -- pointer shortcut; toasts dismiss themselves */
            key={toast.id}
            className={styles.baseStyles}
            style={{
              backgroundColor: styles.bg,
              color: styles.fg,
              borderInlineStartColor: styles.border
            }}
            onClick={() => removeToast(toast.id)}
          >
            {toast.message}
          </div>
        );
      })}
    </div>,
    document.body
  );
}

// Export toast functions
export function showSuccessToast(message: unknown, duration: number = 4000) {
  addToast(message, 'success', duration);
}

export function showErrorToast(message: unknown, duration: number = 4000) {
  addToast(message, 'error', duration);
}

export function showWarningToast(message: unknown, duration: number = 4000) {
  addToast(message, 'warning', duration);
}

export function showInfoToast(message: unknown, duration: number = 4000) {
  addToast(message, 'info', duration);
}

export function showToast(message: unknown, type: string = 'info', duration: number = 4000) {
  addToast(message, type, duration);
}

// For backward compatibility
export function showStatusMessage(message: unknown, type: string = 'info', duration: number = 4000) {
  // Map type strings to the new format
  let toastType = 'info';

  if (type === 'success') {
    toastType = 'success';
  } else if (type === 'error') {
    toastType = 'error';
  } else if (type === 'warning') {
    toastType = 'warning';
  }

  addToast(message, toastType, duration);
}

// Initialize global functions
if (typeof window !== 'undefined') {
  window.showSuccessToast = showSuccessToast;
  window.showErrorToast = showErrorToast;
  window.showWarningToast = showWarningToast;
  window.showInfoToast = showInfoToast;
  window.showToast = showToast;
  window.showStatusMessage = showStatusMessage;

  // Add a test function
  window.testToast = (type = 'info') => {
    const message = `Test ${type} toast at ${formatLocalTime()}`;
    console.log(`Triggering test toast: ${message}`);
    addToast(message, type);
    console.log(`Test toast triggered: ${message}`);
    return 'Toast triggered';
  };
}
