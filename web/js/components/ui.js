/**
 * LightNVR Web Interface UI Components
 * Contains UI-related functionality like modals, status messages, and styles
 */

/**
 * Show snapshot preview
 */
function showSnapshotPreview(imageData, streamName) {
    const modal = document.getElementById('snapshot-preview-modal');
    const image = document.getElementById('snapshot-preview-image');
    const title = document.getElementById('snapshot-preview-title');

    if (!modal || !image || !title) {
        console.error('Snapshot preview modal elements not found');
        return;
    }

    // Set image source
    image.src = imageData;

    // Set title
    title.textContent = `Snapshot: ${streamName}`;

    // Store stream name for download
    modal.dataset.streamName = streamName;

    // Show modal
    modal.style.display = 'block';
}

/**
 * Show status message (legacy function, redirects to toast system)
 */
function showStatusMessage(message, duration = 5000, isError = false) {
    // Use the new toast system if available
    if (typeof showSuccessToast === 'function' && typeof showErrorToast === 'function') {
        if (isError) {
            showErrorToast(message, duration);
        } else {
            showSuccessToast(message, duration);
        }
        return;
    }
    
    // Fallback to old implementation if toast.js is not loaded
    // Check if status message element exists
    let statusMessage = document.getElementById('status-message');

    // Create if it doesn't exist
    if (!statusMessage) {
        statusMessage = document.createElement('div');
        statusMessage.id = 'status-message';
        statusMessage.className = 'status-message';
        document.body.appendChild(statusMessage);
    }

    // Set message text
    statusMessage.textContent = message;

    // Show message
    statusMessage.classList.add('visible');

    // Hide after duration
    setTimeout(() => {
        statusMessage.classList.remove('visible');
    }, duration);
}

/**
 * Show loading state
 */
function showLoading(element) {
    if (!element) return;

    // Add loading class to element
    element.classList.add('loading');

    // Optionally add a loading spinner
    const spinner = document.createElement('div');
    spinner.className = 'spinner';
    element.appendChild(spinner);
}

/**
 * Hide loading state
 */
function hideLoading(element) {
    if (!element) return;

    // Remove loading class from element
    element.classList.remove('loading');

    // Remove loading spinner if exists
    const spinner = element.querySelector('.spinner');
    if (spinner) {
        spinner.remove();
    }
}
