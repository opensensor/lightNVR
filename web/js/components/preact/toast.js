/**
 * LightNVR Toast Notification System
 * Simple, direct DOM-based toast notifications
 */

/**
 * Create a toast directly in the DOM
 * @param {string} message - Message to display
 * @param {string} type - Toast type ('success', 'error', 'warning', 'info')
 * @param {number} duration - Duration in milliseconds
 */
export function createDirectToast(message, type = 'info', duration = 4000) {
  // Ensure message is a string and handle undefined/null values
  const safeMessage = message !== undefined && message !== null ? String(message) : 'Operation completed';

  console.log('Creating direct DOM toast:', safeMessage);

  // Get or create container
  let container = document.getElementById('direct-toast-container');
  if (!container) {
    container = document.createElement('div');
    container.id = 'direct-toast-container';
    container.style.position = 'fixed';
    container.style.top = '20px';
    container.style.left = '50%';
    container.style.transform = 'translateX(-50%)';
    container.style.zIndex = '10000';
    container.style.display = 'flex';
    container.style.flexDirection = 'column';
    container.style.alignItems = 'center';
    document.body.appendChild(container);
    console.log('Created direct toast container');
  }

  // Create toast element
  const toast = document.createElement('div');
  toast.textContent = safeMessage;
  toast.style.padding = '10px 15px';
  toast.style.borderRadius = '4px';
  toast.style.marginBottom = '10px';
  toast.style.boxShadow = '0 2px 5px rgba(0,0,0,0.2)';
  toast.style.minWidth = '250px';
  toast.style.textAlign = 'center';
  toast.style.color = 'white';

  // Set color based on type
  switch (type) {
    case 'success':
      toast.style.backgroundColor = '#10b981'; // green-500
      break;
    case 'error':
      toast.style.backgroundColor = '#ef4444'; // red-500
      break;
    case 'warning':
      toast.style.backgroundColor = '#f59e0b'; // yellow-500
      break;
    default:
      toast.style.backgroundColor = '#3b82f6'; // blue-500
      break;
  }

  // Add to container
  container.appendChild(toast);
  console.log('Added toast to direct container');

  // Remove after specified duration
  setTimeout(() => {
    if (container.contains(toast)) {
      container.removeChild(toast);
      console.log('Removed direct toast');
    }

    // Remove container if empty
    if (container.children.length === 0) {
      document.body.removeChild(container);
      console.log('Removed direct toast container');
    }
  }, duration);
}

/**
 * Show a success toast notification
 * @param {string} message - Message to display
 * @param {number} duration - Duration in milliseconds (default: 4000)
 */
export function showSuccessToast(message, duration = 4000) {
  createDirectToast(message, 'success', duration);
}

/**
 * Show an error toast notification
 * @param {string} message - Message to display
 * @param {number} duration - Duration in milliseconds (default: 4000)
 */
export function showErrorToast(message, duration = 4000) {
  createDirectToast(message, 'error', duration);
}

/**
 * Show a warning toast notification
 * @param {string} message - Message to display
 * @param {number} duration - Duration in milliseconds (default: 4000)
 */
export function showWarningToast(message, duration = 4000) {
  createDirectToast(message, 'warning', duration);
}

/**
 * Show an info toast notification
 * @param {string} message - Message to display
 * @param {number} duration - Duration in milliseconds (default: 4000)
 */
export function showInfoToast(message, duration = 4000) {
  createDirectToast(message, 'info', duration);
}

/**
 * Generic function to show a toast notification
 * @param {string} message - Message to display
 * @param {string} type - Toast type ('success', 'error', 'warning', 'info')
 * @param {number} duration - Duration in milliseconds
 */
export function showToast(message, type = 'info', duration = 4000) {
  createDirectToast(message, type, duration);
}

/**
 * For backward compatibility with the old showStatusMessage function
 * @param {string} message - Message to display
 * @param {string} type - Message type ('success', 'error', 'warning', 'info')
 * @param {number} duration - Duration in milliseconds
 */
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

  createDirectToast(message, toastType, duration);
}

/**
 * Initialize the toast system
 * @param {boolean} showInitMessage - Whether to show an initialization message
 */
export function initToastContainer(showInitMessage = false) {
  console.log('Initializing simple toast system...');

  // Only show initialization message if explicitly requested
  if (showInitMessage) {
    setTimeout(() => {
      createDirectToast('Toast system initialized', 'info', 4000);
    }, 500);
  }
}

// Export for global use
if (typeof window !== 'undefined') {
  // Make toast functions available globally
  window.showSuccessToast = showSuccessToast;
  window.showErrorToast = showErrorToast;
  window.showWarningToast = showWarningToast;
  window.showInfoToast = showInfoToast;
  window.showToast = showToast;
  window.showStatusMessage = showStatusMessage;
  window.createDirectToast = createDirectToast;

  // Log for debugging
  console.log('Toast functions exported to window object');

  // Add a test function to the window object
  window.testToast = (type = 'info') => {
    const message = `Test ${type} toast at ${new Date().toLocaleTimeString()}`;
    console.log(`Triggering test toast: ${message}`);
    createDirectToast(message, type);
    console.log(`Test toast triggered: ${message}`);
    return 'Toast triggered';
  };
}
