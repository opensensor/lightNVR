/**
 * LightNVR Web Interface Toast Container Component
 * Preact component for displaying toast notifications
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { createPortal } from 'preact/compat';

// Global toast state
let toastQueue = [];
let toastId = 0;
let setToastsFunction = null;

/**
 * Add a toast to the queue
 * @param {string} message - Message to display
 * @param {string} type - Toast type ('success', 'error', 'warning', 'info')
 * @param {number} duration - Duration in milliseconds
 */
export function addToast(message, type = 'info', duration = 4000) {
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
export function removeToast(id) {
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
  const containerRef = useRef(null);

  // Store the setter function in the global variable
  useEffect(() => {
    setToastsFunction = setToasts;

    // Clean up
    return () => {
      setToastsFunction = null;
    };
  }, []);

  // Create portal for the toast container
  return createPortal(
    <div
      ref={containerRef}
      className="fixed top-20 left-1/2 -translate-x-1/2 z-50 flex flex-col items-center gap-2.5 w-full max-w-[450px] pointer-events-none"
    >
      {toasts.map(toast => (
        <div
          key={toast.id}
          className={`py-3.5 px-4.5 rounded-lg shadow-lg w-full flex items-center pointer-events-auto font-medium
            ${toast.type === 'success' ? 'bg-green-500 text-white border-l-[6px] border-green-600' :
              toast.type === 'error' ? 'bg-red-500 text-white border-l-[6px] border-red-700' :
              toast.type === 'warning' ? 'bg-yellow-500 text-white border-l-[6px] border-yellow-600' :
              'bg-blue-500 text-white border-l-[6px] border-blue-600'}`}
          onClick={() => removeToast(toast.id)}
        >
          {toast.message}
        </div>
      ))}
    </div>,
    document.body
  );
}

// Export toast functions
export function showSuccessToast(message, duration = 4000) {
  addToast(message, 'success', duration);
}

export function showErrorToast(message, duration = 4000) {
  addToast(message, 'error', duration);
}

export function showWarningToast(message, duration = 4000) {
  addToast(message, 'warning', duration);
}

export function showInfoToast(message, duration = 4000) {
  addToast(message, 'info', duration);
}

export function showToast(message, type = 'info', duration = 4000) {
  addToast(message, type, duration);
}

// For backward compatibility
export function showStatusMessage(message, type = 'info', duration = 4000) {
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
    const message = `Test ${type} toast at ${new Date().toLocaleTimeString()}`;
    console.log(`Triggering test toast: ${message}`);
    addToast(message, type);
    console.log(`Test toast triggered: ${message}`);
    return 'Toast triggered';
  };
}
