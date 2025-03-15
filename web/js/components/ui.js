/**
 * LightNVR Web Interface UI Components
 * Contains UI-related functionality like modals, status messages, and styles
 */

/**
 * Set up modal dialogs
 */
function setupModals() {
    // Stream modal
    const streamModal = document.getElementById('stream-modal');
    const streamCancelBtn = document.getElementById('stream-cancel-btn');
    const streamSaveBtn = document.getElementById('stream-save-btn');
    const streamTestBtn = document.getElementById('stream-test-btn');
    const streamCloseBtn = streamModal?.querySelector('.close');

    // Hide stream modal when cancel button is clicked
    if (streamCancelBtn) {
        streamCancelBtn.addEventListener('click', function() {
            streamModal.style.display = 'none';
        });
    }

    // Hide stream modal when close button is clicked
    if (streamCloseBtn) {
        streamCloseBtn.addEventListener('click', function() {
            streamModal.style.display = 'none';
        });
    }

    // Save stream when save button is clicked
    if (streamSaveBtn) {
        streamSaveBtn.addEventListener('click', function() {
            saveStream();
        });
    }

    // Test stream when test button is clicked
    if (streamTestBtn) {
        streamTestBtn.addEventListener('click', function() {
            testStream();
        });
    }

    // Video modal
    const videoModal = document.getElementById('video-modal');
    const videoCloseBtn = document.getElementById('video-close-btn');
    const videoModalCloseBtn = videoModal?.querySelector('.close');

    // Hide video modal when close button is clicked
    if (videoCloseBtn) {
        videoCloseBtn.addEventListener('click', function() {
            closeVideoModal();
        });
    }

    // Hide video modal when close button is clicked
    if (videoModalCloseBtn) {
        videoModalCloseBtn.addEventListener('click', function() {
            closeVideoModal();
        });
    }

    // Close modals when clicking outside
    window.addEventListener('click', function(e) {
        if (e.target === streamModal) {
            streamModal.style.display = 'none';
        } else if (e.target === videoModal) {
            closeVideoModal();
        }
    });
    
    // Close modals when Escape key is pressed
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape') {
            // Close any open modals
            if (streamModal && streamModal.style.display === 'block') {
                streamModal.style.display = 'none';
            }
            if (videoModal && videoModal.style.display === 'block') {
                closeVideoModal();
            }
            
            // Also close snapshot modal if it exists and is open
            const snapshotModal = document.getElementById('snapshot-preview-modal');
            if (snapshotModal && snapshotModal.style.display === 'block') {
                snapshotModal.style.display = 'none';
            }
        }
    });
}

/**
 * Close video modal and stop playback
 */
function closeVideoModal() {
    const videoModal = document.getElementById('video-modal');
    const videoPlayer = document.getElementById('video-player');

    if (videoPlayer) {
        const video = videoPlayer.querySelector('video');
        if (video) {
            video.pause();
            video.src = '';
        }
        videoPlayer.innerHTML = '';
    }

    if (videoModal) {
        videoModal.style.display = 'none';
    }
}

/**
 * Setup snapshot preview modal
 */
function setupSnapshotModal() {
    // Get modal elements
    const modal = document.getElementById('snapshot-preview-modal');
    if (!modal) return;

    const closeBtn = modal.querySelector('.close');
    const downloadBtn = document.getElementById('snapshot-download-btn');
    const closeModalBtn = document.getElementById('snapshot-close-btn');

    // Close modal when close button is clicked
    if (closeBtn) {
        closeBtn.addEventListener('click', function() {
            modal.style.display = 'none';
        });
    }

    // Close modal when close button is clicked
    if (closeModalBtn) {
        closeModalBtn.addEventListener('click', function() {
            modal.style.display = 'none';
        });
    }

    // Download image when download button is clicked
    if (downloadBtn) {
        downloadBtn.addEventListener('click', function() {
            const image = document.getElementById('snapshot-preview-image');
            const streamName = modal.dataset.streamName;

            if (image && image.src && streamName) {
                downloadSnapshotFromPreview(image.src, streamName);
            }
        });
    }

    // Close modal when clicking outside
    window.addEventListener('click', function(e) {
        if (e.target === modal) {
            modal.style.display = 'none';
        }
    });
    
    // Note: Escape key handling is now in setupModals() to handle all modals consistently
}

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
 * Download snapshot from preview
 */
function downloadSnapshotFromPreview(imageData, streamName) {
    try {
        // Create a formatted timestamp for the filename
        const now = new Date();
        const dateStr = now.toISOString().slice(0, 10).replace(/-/g, '');
        const timeStr = now.toTimeString().slice(0, 8).replace(/:/g, '');
        const filename = `${streamName.replace(/\s+/g, '_')}_${dateStr}_${timeStr}.jpg`;

        // Create a link and trigger download
        const link = document.createElement('a');
        link.download = filename;
        link.href = imageData;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);

        // Show success message
        showStatusMessage(`Snapshot saved: ${filename}`);
    } catch (error) {
        console.error('Error downloading snapshot:', error);
        alert('Error downloading snapshot: ' + error.message);
    }
}

/**
 * Show status message
 */
function showStatusMessage(message, duration = 3000) {
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
 * Add status message styles
 */
function addStatusMessageStyles() {
    // Check if styles already exist
    if (document.getElementById('status-message-styles')) {
        return;
    }

    // Create style element
    const style = document.createElement('style');
    style.id = 'status-message-styles';

    style.textContent = `
        .status-message {
            position: fixed;
            bottom: 20px;
            right: 20px;
            background-color: rgba(0, 0, 0, 0.8);
            color: white;
            padding: 10px 15px;
            border-radius: 4px;
            z-index: 1000;
            font-size: 14px;
            opacity: 0;
            transform: translateY(20px);
            transition: opacity 0.3s, transform 0.3s;
            max-width: 80%;
        }
        
        .status-message.visible {
            opacity: 1;
            transform: translateY(0);
        }
    `;

    document.head.appendChild(style);
}

/**
 * Add stream styles
 */
function addStreamStyles() {
    // Check if styles already exist
    if (document.getElementById('stream-styles')) {
        return;
    }

    // Create style element
    const style = document.createElement('style');
    style.id = 'stream-styles';

    style.textContent = `
        .video-grid {
            display: grid;
            gap: 1rem;
            height: calc(100vh - 200px);
            min-height: 400px;
        }
        
        .video-item {
            background-color: #000;
            border-radius: 4px;
            overflow: hidden;
            display: flex;
            flex-direction: column;
        }
        
        .video-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 5px 10px;
            background-color: rgba(0, 0, 0, 0.7);
            color: white;
        }
        
        .video-controls {
            display: flex;
            gap: 5px;
        }
        
        .btn-small {
            padding: 2px 5px;
            font-size: 12px;
            background-color: rgba(255, 255, 255, 0.2);
            border: none;
            border-radius: 3px;
            color: white;
            cursor: pointer;
        }
        
        .btn-small:hover {
            background-color: rgba(255, 255, 255, 0.3);
        }
        
        .video-player {
            position: relative;
            flex: 1;
            width: 100%;
            background-color: #000;
            overflow: hidden;
        }
        
        .video-player video {
            width: 100%;
            height: 100%;
            object-fit: contain;
        }
        
        .loading-overlay {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background-color: rgba(0, 0, 0, 0.7);
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            color: white;
            display: none;
        }
        
        .video-player.loading .loading-overlay {
            display: flex;
        }
        
        .spinner {
            width: 40px;
            height: 40px;
            border: 4px solid rgba(255, 255, 255, 0.3);
            border-radius: 50%;
            border-top-color: white;
            animation: spin 1s ease-in-out infinite;
            margin-bottom: 10px;
        }
        
        @keyframes spin {
            to { transform: rotate(360deg); }
        }
        
        .error-message {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            background-color: rgba(0, 0, 0, 0.8);
            color: white;
            text-align: center;
            padding: 20px;
        }
        
        .error-icon {
            width: 40px;
            height: 40px;
            border-radius: 50%;
            background-color: #f44336;
            color: white;
            display: flex;
            justify-content: center;
            align-items: center;
            font-size: 24px;
            font-weight: bold;
            margin-bottom: 10px;
        }
        
        .retry-button {
            margin-top: 15px;
            padding: 5px 15px;
            background-color: #1e88e5;
            color: white;
            border: none;
            border-radius: 4px;
            cursor: pointer;
        }
        
        .play-overlay {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background-color: rgba(0, 0, 0, 0.6);
            display: flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
            z-index: 10;
            backdrop-filter: blur(2px);
            -webkit-backdrop-filter: blur(2px);
            transition: background-color 0.2s ease;
        }
        
        .play-overlay:hover {
            background-color: rgba(0, 0, 0, 0.5);
        }
        
        .play-button {
            width: 70px;
            height: 70px;
            background-color: rgba(255, 255, 255, 0.2);
            border-radius: 50%;
            display: flex;
            justify-content: center;
            align-items: center;
            transition: transform 0.2s ease, background-color 0.2s ease;
        }
        
        .play-button::before {
            content: '';
            width: 0;
            height: 0;
            border-top: 15px solid transparent;
            border-bottom: 15px solid transparent;
            border-left: 25px solid white;
            margin-left: 5px;
        }
        
        .play-overlay:hover .play-button {
            transform: scale(1.1);
            background-color: rgba(255, 255, 255, 0.3);
        }
    `;

    document.head.appendChild(style);
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

/**
 * Create a progress indicator
 */
function createProgressIndicator(container, message) {
    const progressContainer = document.createElement('div');
    progressContainer.className = 'progress-container';
    
    const spinner = document.createElement('div');
    spinner.className = 'spinner';
    progressContainer.appendChild(spinner);
    
    const messageElement = document.createElement('p');
    messageElement.textContent = message || 'Loading...';
    progressContainer.appendChild(messageElement);
    
    const progressElement = document.createElement('div');
    progressElement.className = 'progress-bar';
    progressElement.innerHTML = '<div class="progress-fill" style="width: 0%"></div>';
    progressContainer.appendChild(progressElement);
    
    container.appendChild(progressContainer);
    return progressContainer;
}

/**
 * Update progress indicator
 */
function updateProgress(progressContainer, percent, message, isError = false) {
    if (!progressContainer) return;
    
    const progressFill = progressContainer.querySelector('.progress-fill');
    if (progressFill) {
        progressFill.style.width = `${percent}%`;
        
        if (isError) {
            progressFill.style.backgroundColor = '#f44336';
        }
    }
    
    const messageElement = progressContainer.querySelector('p');
    if (messageElement && message) {
        messageElement.textContent = message;
        
        if (isError) {
            messageElement.style.color = '#f44336';
        }
    }
}

/**
 * Remove progress indicator
 */
function removeProgressIndicator(progressContainer) {
    if (!progressContainer) return;
    
    // Add fade-out animation
    progressContainer.style.opacity = '0';
    progressContainer.style.transition = 'opacity 0.5s ease';
    
    // Remove after animation completes
    setTimeout(() => {
        if (progressContainer.parentNode) {
            progressContainer.parentNode.removeChild(progressContainer);
        }
    }, 500);
}

/**
 * Handle API errors
 */
function handleApiError(error, fallbackMessage) {
    console.error('API Error:', error);

    let errorMessage = fallbackMessage || 'An error occurred';

    if (error.response && error.response.json) {
        error.response.json().then(data => {
            if (data && data.error) {
                errorMessage = data.error;
            }
            alert(errorMessage);
        }).catch(() => {
            alert(errorMessage);
        });
    } else {
        alert(errorMessage);
    }
}
