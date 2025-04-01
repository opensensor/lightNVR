/**
 * LightNVR Web Interface Video Utilities
 * Contains utility functions for video components
 */

/**
 * Show a status message to the user
 * @param {string} message - The message to display
 * @param {number} duration - How long to show the message in milliseconds
 */
function showStatusMessage(message, duration = 3000) {
    // Check if the status message component exists
    let statusMessage = document.getElementById('status-message');
    
    // If it doesn't exist, create it
    if (!statusMessage) {
        statusMessage = document.createElement('div');
        statusMessage.id = 'status-message';
        statusMessage.className = 'status-message';
        document.body.appendChild(statusMessage);
    }
    
    // Set the message text
    statusMessage.textContent = message;
    
    // Add the visible class to show the message
    statusMessage.classList.add('visible');
    
    // Clear any existing timeout
    if (window.statusMessageTimeout) {
        clearTimeout(window.statusMessageTimeout);
    }
    
    // Set a timeout to hide the message
    window.statusMessageTimeout = setTimeout(() => {
        statusMessage.classList.remove('visible');
    }, duration);
}

/**
 * Show a loading indicator on an element
 * @param {HTMLElement} element - The element to show loading on
 */
function showLoading(element) {
    if (!element) return;
    
    // Add loading class to the element
    element.classList.add('loading');
    
    // Create loading overlay if it doesn't exist
    let loadingOverlay = element.querySelector('.loading-overlay');
    if (!loadingOverlay) {
        loadingOverlay = document.createElement('div');
        loadingOverlay.className = 'loading-overlay';
        loadingOverlay.innerHTML = `
            <div class="loading-spinner"></div>
            <div class="loading-text">Loading...</div>
        `;
        element.appendChild(loadingOverlay);
    }
    
    // Show the loading overlay
    loadingOverlay.style.display = 'flex';
}

/**
 * Hide the loading indicator on an element
 * @param {HTMLElement} element - The element to hide loading from
 */
function hideLoading(element) {
    if (!element) return;
    
    // Remove loading class from the element
    element.classList.remove('loading');
    
    // Hide the loading overlay
    const loadingOverlay = element.querySelector('.loading-overlay');
    if (loadingOverlay) {
        loadingOverlay.style.display = 'none';
    }
}

/**
 * Show a snapshot preview in a modal
 * @param {string} imageData - The image data URL
 * @param {string} streamName - The name of the stream
 */
function showSnapshotPreview(imageData, streamName) {
    // Check if the snapshot preview modal exists
    let snapshotModal = document.getElementById('snapshot-preview-modal');
    
    // If it doesn't exist, create it
    if (!snapshotModal) {
        snapshotModal = document.createElement('div');
        snapshotModal.id = 'snapshot-preview-modal';
        snapshotModal.className = 'modal';
        snapshotModal.innerHTML = `
            <div class="modal-content snapshot-preview-content">
                <div class="modal-header">
                    <h3 id="snapshot-preview-title" class="text-lg font-medium">Snapshot Preview</h3>
                    <span class="close">&times;</span>
                </div>
                <div class="modal-body snapshot-preview-body">
                    <img id="snapshot-preview-image" alt="Snapshot Preview">
                </div>
                <div class="modal-footer">
                    <button id="snapshot-download-btn" class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors">
                        Download
                    </button>
                    <button id="snapshot-close-btn" class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors">
                        Close
                    </button>
                </div>
            </div>
        `;
        document.body.appendChild(snapshotModal);
    }
    
    // Get modal elements
    const snapshotTitle = document.getElementById('snapshot-preview-title');
    const snapshotImage = document.getElementById('snapshot-preview-image');
    const snapshotDownloadBtn = document.getElementById('snapshot-download-btn');
    const snapshotCloseBtn = document.getElementById('snapshot-close-btn');
    const modalCloseBtn = snapshotModal.querySelector('.close');
    
    // Set the title and image
    snapshotTitle.textContent = `Snapshot: ${streamName}`;
    snapshotImage.src = imageData;
    
    // Define close modal function
    function closeModal() {
        snapshotModal.style.display = 'none';
    }
    
    // Set up close button event handlers
    if (snapshotCloseBtn) {
        snapshotCloseBtn.onclick = closeModal;
    }
    
    if (modalCloseBtn) {
        modalCloseBtn.onclick = closeModal;
    }
    
    // Close on click outside
    window.onclick = function(event) {
        if (event.target === snapshotModal) {
            closeModal();
        }
    };
    
    // Set up download button
    if (snapshotDownloadBtn) {
        snapshotDownloadBtn.onclick = function() {
            const link = document.createElement('a');
            link.href = imageData;
            link.download = `snapshot_${streamName}_${new Date().toISOString().replace(/[:.]/g, '-')}.jpg`;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
        };
    }
    
    // Show the modal
    snapshotModal.style.display = 'block';
}

// Export functions
window.showStatusMessage = showStatusMessage;
window.showLoading = showLoading;
window.hideLoading = hideLoading;
window.showSnapshotPreview = showSnapshotPreview;
