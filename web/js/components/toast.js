/**
 * LightNVR Toast Notification System
 * Provides standardized success and error toast notifications
 */

// Toast container for managing multiple toasts
let toastContainer = null;

/**
 * Initialize the toast container
 */
function initToastContainer() {
    // Check if container already exists
    if (document.getElementById('toast-container')) {
        toastContainer = document.getElementById('toast-container');
        return;
    }

    // Create toast container with higher z-index to ensure visibility
    toastContainer = document.createElement('div');
    toastContainer.id = 'toast-container';
    toastContainer.className = 'toast-container';
    toastContainer.style.zIndex = '9999'; // Ensure it's above everything
    document.body.appendChild(toastContainer);
}

/**
 * Create a toast notification
 * @param {string} message - The message to display
 * @param {string} type - The type of toast ('success' or 'error')
 * @param {number} duration - Duration in milliseconds before auto-hiding
 * @returns {HTMLElement} - The created toast element
 */
function createToast(message, type = 'success', duration = 3000) {
    // Initialize container if not already done
    if (!toastContainer) {
        initToastContainer();
    }

    // Create toast element
    const toast = document.createElement('div');
    toast.className = `toast toast-${type}`;
    
    // Create icon based on type
    const iconSvg = type === 'success' 
        ? '<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/></svg>'
        : '<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/></svg>';
    
    // Create toast content
    toast.innerHTML = `
        <div class="toast-icon">${iconSvg}</div>
        <div class="toast-message">${message}</div>
        <div class="toast-close">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
        </div>
    `;
    
    // Add to container
    toastContainer.appendChild(toast);
    
    // Add close button functionality
    const closeBtn = toast.querySelector('.toast-close');
    if (closeBtn) {
        closeBtn.addEventListener('click', () => {
            removeToast(toast);
        });
    }
    
    // Show toast with animation
    setTimeout(() => {
        toast.classList.add('visible');
    }, 10);
    
    // Auto-hide after duration
    if (duration > 0) {
        setTimeout(() => {
            removeToast(toast);
        }, duration);
    }
    
    return toast;
}

/**
 * Remove a toast with animation
 * @param {HTMLElement} toast - The toast element to remove
 */
function removeToast(toast) {
    if (!toast) return;
    
    // Hide with animation
    toast.classList.remove('visible');
    
    // Remove after animation completes
    setTimeout(() => {
        if (toast.parentNode) {
            toast.parentNode.removeChild(toast);
        }
    }, 300);
}

/**
 * Show a success toast notification
 * @param {string} message - The success message to display
 * @param {number} duration - Duration in milliseconds before auto-hiding
 * @returns {HTMLElement} - The created toast element
 */
function showSuccessToast(message, duration = 5000) {
    return createToast(message, 'success', duration);
}

/**
 * Show an error toast notification
 * @param {string} message - The error message to display
 * @param {number} duration - Duration in milliseconds before auto-hiding
 * @returns {HTMLElement} - The created toast element
 */
function showErrorToast(message, duration = 8000) {
    return createToast(message, 'error', duration);
}

/**
 * Initialize Alpine.js integration for toast notifications
 */
function initAlpineToast() {
    if (typeof Alpine !== 'undefined') {
        document.addEventListener('alpine:init', () => {
            // Toast notification store
            Alpine.store('toast', {
                // Show success toast
                success(message, duration = 5000) {
                    showSuccessToast(message, duration);
                },
                
                // Show error toast
                error(message, duration = 8000) {
                    showErrorToast(message, duration);
                }
            });
            
            // Replace the statusMessage store with toast functionality
            Alpine.store('statusMessage', {
                show(message, duration = 5000, isError = false) {
                    if (isError) {
                        showErrorToast(message, duration);
                    } else {
                        showSuccessToast(message, duration);
                    }
                }
            });
        });
    }
}

// Initialize toast system
document.addEventListener('DOMContentLoaded', () => {
    initToastContainer();
    initAlpineToast();
});

// Handle API responses with standardized toasts
function handleApiResponse(response, successMessage = 'Operation successful') {
    if (!response.ok) {
        // Handle error response
        response.json()
            .then(data => {
                const errorMessage = data.error || 'An error occurred';
                showErrorToast(errorMessage);
            })
            .catch(() => {
                showErrorToast(`Error: ${response.status} ${response.statusText}`);
            });
        return false;
    } else {
        // Handle success response
        showSuccessToast(successMessage);
        return true;
    }
}

// Export functions for use in other modules
window.showSuccessToast = showSuccessToast;
window.showErrorToast = showErrorToast;
window.handleApiResponse = handleApiResponse;
