/**
 * LightNVR Web Interface JavaScript
 */

// Wait for DOM to be fully loaded
document.addEventListener('DOMContentLoaded', function() {
    // Initialize the application
    initApp();
});

/**
 * Initialize the application
 */
function initApp() {
    // Set up navigation
    setupNavigation();

    // Set up modals
    setupModals();

    // Set up event handlers
    setupEventHandlers();

    // Add stream styles
    addStreamStyles();

    // Load initial data
    loadInitialData();

    console.log('LightNVR Web Interface initialized');
}

/**
 * Set up navigation between pages
 */
function setupNavigation() {
    const navLinks = document.querySelectorAll('nav a');
    
    navLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            
            // Get the page to show
            const pageId = this.getAttribute('data-page');
            
            // Remove active class from all links
            navLinks.forEach(link => link.classList.remove('active'));
            
            // Add active class to clicked link
            this.classList.add('active');
            
            // Hide all pages
            document.querySelectorAll('.page').forEach(page => {
                page.classList.remove('active');
            });
            
            // Show the selected page
            document.getElementById(`${pageId}-page`).classList.add('active');
            
            // Refresh data when switching to certain pages
            if (pageId === 'streams') {
                loadStreams();
            } else if (pageId === 'recordings') {
                loadRecordings();
            } else if (pageId === 'system') {
                loadSystemInfo();
            }
        });
    });
    
    // Handle page links outside of navigation
    document.querySelectorAll('a[data-page]').forEach(link => {
        if (!link.closest('nav')) {
            link.addEventListener('click', function(e) {
                e.preventDefault();
                
                // Get the page to show
                const pageId = this.getAttribute('data-page');
                
                // Find the corresponding nav link and click it
                document.querySelector(`nav a[data-page="${pageId}"]`).click();
            });
        }
    });
}

/**
 * Set up modal dialogs
 */
function setupModals() {
    // Stream modal
    const streamModal = document.getElementById('stream-modal');
    const addStreamBtn = document.getElementById('add-stream-btn');
    const streamCancelBtn = document.getElementById('stream-cancel-btn');
    const streamCloseBtn = streamModal.querySelector('.close');
    
    // Show stream modal when add stream button is clicked
    if (addStreamBtn) {
        addStreamBtn.addEventListener('click', function() {
            // Reset form
            document.getElementById('stream-form').reset();
            // Show modal
            streamModal.style.display = 'block';
        });
    }
    
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
    
    // Video modal
    const videoModal = document.getElementById('video-modal');
    const videoCloseBtn = document.getElementById('video-close-btn');
    const videoModalCloseBtn = videoModal.querySelector('.close');
    
    // Hide video modal when close button is clicked
    if (videoCloseBtn) {
        videoCloseBtn.addEventListener('click', function() {
            // Stop video playback
            const videoPlayer = document.getElementById('video-player');
            if (videoPlayer) {
                videoPlayer.pause();
                videoPlayer.src = '';
            }
            
            videoModal.style.display = 'none';
        });
    }
    
    // Hide video modal when close button is clicked
    if (videoModalCloseBtn) {
        videoModalCloseBtn.addEventListener('click', function() {
            // Stop video playback
            const videoPlayer = document.getElementById('video-player');
            if (videoPlayer) {
                videoPlayer.pause();
                videoPlayer.src = '';
            }
            
            videoModal.style.display = 'none';
        });
    }
    
    // Close modals when clicking outside
    window.addEventListener('click', function(e) {
        if (e.target === streamModal) {
            streamModal.style.display = 'none';
        } else if (e.target === videoModal) {
            // Stop video playback
            const videoPlayer = document.getElementById('video-player');
            if (videoPlayer) {
                videoPlayer.pause();
                videoPlayer.src = '';
            }
            
            videoModal.style.display = 'none';
        }
    });
}

/**
 * Set up event handlers for various UI elements
 */
function setupEventHandlers() {
    // Save settings button
    const saveSettingsBtn = document.getElementById('save-settings-btn');
    if (saveSettingsBtn) {
        saveSettingsBtn.addEventListener('click', function() {
            saveSettings();
        });
    }
    
    // Stream save button
    const streamSaveBtn = document.getElementById('stream-save-btn');
    if (streamSaveBtn) {
        streamSaveBtn.addEventListener('click', function() {
            saveStream();
        });
    }
    
    // Stream test button
    const streamTestBtn = document.getElementById('stream-test-btn');
    if (streamTestBtn) {
        streamTestBtn.addEventListener('click', function() {
            testStream();
        });
    }
    
    // Refresh system button
    const refreshSystemBtn = document.getElementById('refresh-system-btn');
    if (refreshSystemBtn) {
        refreshSystemBtn.addEventListener('click', function() {
            loadSystemInfo();
        });
    }
    
    // Restart service button
    const restartBtn = document.getElementById('restart-btn');
    if (restartBtn) {
        restartBtn.addEventListener('click', function() {
            if (confirm('Are you sure you want to restart the LightNVR service?')) {
                restartService();
            }
        });
    }
    
    // Shutdown service button
    const shutdownBtn = document.getElementById('shutdown-btn');
    if (shutdownBtn) {
        shutdownBtn.addEventListener('click', function() {
            if (confirm('Are you sure you want to shutdown the LightNVR service?')) {
                shutdownService();
            }
        });
    }
    
    // Clear logs button
    const clearLogsBtn = document.getElementById('clear-logs-btn');
    if (clearLogsBtn) {
        clearLogsBtn.addEventListener('click', function() {
            if (confirm('Are you sure you want to clear the logs?')) {
                clearLogs();
            }
        });
    }
    
    // Backup configuration button
    const backupConfigBtn = document.getElementById('backup-config-btn');
    if (backupConfigBtn) {
        backupConfigBtn.addEventListener('click', function() {
            backupConfig();
        });
    }
    
    // Layout selector
    const layoutSelector = document.getElementById('layout-selector');
    if (layoutSelector) {
        layoutSelector.addEventListener('change', function() {
            updateVideoLayout(this.value);
        });
    }
    
    // Fullscreen button
    const fullscreenBtn = document.getElementById('fullscreen-btn');
    if (fullscreenBtn) {
        fullscreenBtn.addEventListener('click', function() {
            toggleFullscreen();
        });
    }
    
    // Logout button
    const logoutBtn = document.getElementById('logout');
    if (logoutBtn) {
        logoutBtn.addEventListener('click', function(e) {
            e.preventDefault();
            logout();
        });
    }
    
    // Refresh buttons
    const refreshStreamsBtn = document.querySelector('button#refresh-streams-btn');
    if (refreshStreamsBtn) {
        refreshStreamsBtn.addEventListener('click', function() {
            loadStreams();
        });
    }
    
    const refreshRecordingsBtn = document.getElementById('refresh-btn');
    if (refreshRecordingsBtn) {
        refreshRecordingsBtn.addEventListener('click', function() {
            loadRecordings();
        });
    }
}

/**
 * Load initial data for the application
 */
function loadInitialData() {
    // Load system information
    loadSystemInfo();
    
    // Load streams
    loadStreams();
    
    // Load recordings
    loadRecordings();
    
    // Load settings
    loadSettings();
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
 * Load system information
 */
function loadSystemInfo() {
    const systemContainer = document.querySelector('.system-container');
    if (!systemContainer) return;
    
    showLoading(systemContainer);
    
    // Fetch system information from API
    fetch('/api/system/info')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load system information');
            }
            return response.json();
        })
        .then(data => {
            // Update system information
            document.getElementById('system-version').textContent = data.version || '0.1.0';
            
            // Format uptime
            const uptime = data.uptime || 0;
            const days = Math.floor(uptime / 86400);
            const hours = Math.floor((uptime % 86400) / 3600);
            const minutes = Math.floor((uptime % 3600) / 60);
            document.getElementById('system-uptime').textContent = `${days}d ${hours}h ${minutes}m`;
            
            document.getElementById('system-cpu').textContent = `${data.cpu_usage || 0}%`;
            document.getElementById('system-memory').textContent = `${data.memory_usage || 0} MB / ${data.memory_total || 0} MB`;
            document.getElementById('system-storage').textContent = `${data.storage_usage || 0} GB / ${data.storage_total || 0} GB`;
            
            // Update stream statistics
            document.getElementById('system-active-streams').textContent = `${data.active_streams || 0} / ${data.max_streams || 0}`;
            document.getElementById('system-recording-streams').textContent = `${data.recording_streams || 0}`;
            document.getElementById('system-received').textContent = `${data.data_received || 0} MB`;
            document.getElementById('system-recorded').textContent = `${data.data_recorded || 0} MB`;
            
            // Load logs
            loadSystemLogs();
        })
        .catch(error => {
            console.error('Error loading system information:', error);
            // Use placeholder data
            document.getElementById('system-version').textContent = '0.1.0';
            document.getElementById('system-uptime').textContent = '0d 0h 0m';
            document.getElementById('system-cpu').textContent = '0%';
            document.getElementById('system-memory').textContent = '0 MB / 0 MB';
            document.getElementById('system-storage').textContent = '0 GB / 0 GB';
            document.getElementById('system-active-streams').textContent = '0 / 0';
            document.getElementById('system-recording-streams').textContent = '0';
            document.getElementById('system-received').textContent = '0 MB';
            document.getElementById('system-recorded').textContent = '0 MB';
        })
        .finally(() => {
            hideLoading(systemContainer);
        });
}

/**
 * Load system logs
 */
function loadSystemLogs() {
    const logsContainer = document.getElementById('system-logs');
    if (!logsContainer) return;
    
    // Fetch logs from API
    fetch('/api/system/logs')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load system logs');
            }
            return response.json();
        })
        .then(data => {
            if (data.logs && Array.isArray(data.logs)) {
                logsContainer.textContent = data.logs.join('\n');
            } else {
                logsContainer.textContent = 'No logs available';
            }
        })
        .catch(error => {
            console.error('Error loading system logs:', error);
            logsContainer.textContent = 'Error loading logs';
        });
}

/**
 * Load streams
 */
function loadStreams() {
    const streamsTable = document.getElementById('streams-table');
    if (!streamsTable) return;
    
    const tbody = streamsTable.querySelector('tbody');
    
    showLoading(streamsTable);
    
    // Clear existing rows
    tbody.innerHTML = '<tr><td colspan="7" class="empty-message">Loading streams...</td></tr>';
    
    // Fetch streams from API
    fetch('/api/streams')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load streams');
            }
            return response.json();
        })
        .then(streams => {
            tbody.innerHTML = '';
            
            if (!streams || streams.length === 0) {
                tbody.innerHTML = '<tr><td colspan="7" class="empty-message">No streams configured</td></tr>';
                return;
            }
            
            streams.forEach(stream => {
                const tr = document.createElement('tr');
                
                tr.innerHTML = `
                    <td>${stream.name}</td>
                    <td>${stream.url}</td>
                    <td>${stream.status || 'Unknown'}</td>
                    <td>${stream.width}x${stream.height}</td>
                    <td>${stream.fps}</td>
                    <td>${stream.record ? 'Yes' : 'No'}</td>
                    <td>
                        <button class="btn-primary edit-stream" data-name="${stream.name}">Edit</button>
                        <button class="btn-danger delete-stream" data-name="${stream.name}">Delete</button>
                    </td>
                `;
                
                tbody.appendChild(tr);
            });
            
            // Add event listeners for edit and delete buttons
            document.querySelectorAll('.edit-stream').forEach(button => {
                button.addEventListener('click', function() {
                    const streamName = this.getAttribute('data-name');
                    editStream(streamName);
                });
            });
            
            document.querySelectorAll('.delete-stream').forEach(button => {
                button.addEventListener('click', function() {
                    const streamName = this.getAttribute('data-name');
                    if (confirm(`Are you sure you want to delete the stream "${streamName}"?`)) {
                        deleteStream(streamName);
                    }
                });
            });
            
            // Update stream filter in recordings page
            updateStreamFilter(streams);
            
            // Update video grid
            updateVideoGrid(streams);
        })
        .catch(error => {
            console.error('Error loading streams:', error);
            tbody.innerHTML = '<tr><td colspan="7" class="empty-message">Error loading streams</td></tr>';
        })
        .finally(() => {
            hideLoading(streamsTable);
        });
}

/**
 * Update stream filter dropdown
 */
function updateStreamFilter(streams) {
    const streamFilter = document.getElementById('stream-filter');
    if (!streamFilter) return;
    
    // Clear existing options except "All Streams"
    while (streamFilter.options.length > 1) {
        streamFilter.remove(1);
    }
    
    // Add stream options
    if (streams && streams.length > 0) {
        streams.forEach(stream => {
            const option = document.createElement('option');
            option.value = stream.name;
            option.textContent = stream.name;
            streamFilter.appendChild(option);
        });
    }
}

/**
 * Update video grid with streams
 */
function updateVideoGrid(streams) {
    const videoGrid = document.getElementById('video-grid');
    if (!videoGrid) return;

    // Clear existing content
    videoGrid.innerHTML = '';

    if (!streams || streams.length === 0) {
        const placeholder = document.createElement('div');
        placeholder.className = 'placeholder';
        placeholder.innerHTML = `
            <p>No streams configured</p>
            <a href="#" data-page="streams" class="btn">Configure Streams</a>
        `;
        videoGrid.appendChild(placeholder);
        return;
    }

    // Get layout
    const layout = document.getElementById('layout-selector').value;

    // Set grid columns based on layout
    if (layout === '1') {
        videoGrid.style.gridTemplateColumns = '1fr';
    } else if (layout === '4') {
        videoGrid.style.gridTemplateColumns = 'repeat(2, 1fr)';
    } else if (layout === '9') {
        videoGrid.style.gridTemplateColumns = 'repeat(3, 1fr)';
    } else if (layout === '16') {
        videoGrid.style.gridTemplateColumns = 'repeat(4, 1fr)';
    }

    // Add video elements for each stream
    streams.forEach(stream => {
        const videoContainer = document.createElement('div');
        videoContainer.className = 'video-item';

        videoContainer.innerHTML = `
            <div class="video-header">
                <span>${stream.name}</span>
                <div class="video-controls">
                    <button class="btn-small snapshot" data-name="${stream.name}">Snapshot</button>
                    <button class="btn-small fullscreen" data-name="${stream.name}">Fullscreen</button>
                </div>
            </div>
            <div class="video-player" id="player-${stream.name.replace(/\s+/g, '-')}">
                <video id="video-${stream.name.replace(/\s+/g, '-')}" autoplay muted></video>
                <div class="loading-overlay">
                    <div class="spinner"></div>
                    <p>Connecting to stream...</p>
                </div>
            </div>
        `;

        videoGrid.appendChild(videoContainer);
    });

    // Initialize video players and add event listeners
    streams.forEach(stream => {
        initializeVideoPlayer(stream);

        // Add event listener for snapshot button
        const snapshotBtn = videoGrid.querySelector(`.snapshot[data-name="${stream.name}"]`);
        if (snapshotBtn) {
            snapshotBtn.addEventListener('click', () => {
                takeSnapshot(stream.name);
            });
        }

        // Add event listener for fullscreen button
        const fullscreenBtn = videoGrid.querySelector(`.fullscreen[data-name="${stream.name}"]`);
        if (fullscreenBtn) {
            fullscreenBtn.addEventListener('click', () => {
                toggleStreamFullscreen(stream.name);
            });
        }
    });
}

/**
 * Initialize video player for a stream
 /**
 * Initialize video player for a stream using HLS.js
 */
function initializeVideoPlayer(stream) {
    const videoElementId = `video-${stream.name.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const containerId = `player-${stream.name.replace(/\s+/g, '-')}`;
    const container = document.getElementById(containerId);

    if (!videoElement || !container) return;

    // Show loading state
    container.classList.add('loading');

    // Build the HLS stream URL - this would be generated by your backend
    // Your backend needs to convert RTSP to HLS using FFmpeg
    const hlsStreamUrl = `/api/streaming/${encodeURIComponent(stream.name)}/hls/index.m3u8`;

    // Check if HLS is supported natively
    if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
        // Native HLS support (Safari)
        videoElement.src = hlsStreamUrl;
        videoElement.addEventListener('loadedmetadata', function() {
            container.classList.remove('loading');
        });

        videoElement.addEventListener('error', function() {
            handleVideoError(stream.name);
        });
    }
    // Use HLS.js for browsers that don't support HLS natively
    else if (Hls.isSupported()) {
        const hls = new Hls({
            maxBufferLength: 30,
            maxMaxBufferLength: 60,
            liveSyncDurationCount: 3,
            enableWorker: true
        });

        hls.loadSource(hlsStreamUrl);
        hls.attachMedia(videoElement);

        hls.on(Hls.Events.MANIFEST_PARSED, function() {
            container.classList.remove('loading');
            videoElement.play().catch(error => {
                console.warn('Auto-play prevented:', error);
                // Add play button overlay for user interaction
                addPlayButtonOverlay(container, videoElement);
            });
        });

        hls.on(Hls.Events.ERROR, function(event, data) {
            if (data.fatal) {
                console.error('HLS error:', data);
                hls.destroy();
                handleVideoError(stream.name);
            }
        });

        // Store hls instance for cleanup
        container.hlsPlayer = hls;
    }
    // Fallback for unsupported browsers
    else {
        handleVideoError(stream.name, 'HLS not supported by your browser');
    }
}

/**
 * Add play button overlay for browsers that block autoplay
 */
function addPlayButtonOverlay(container, videoElement) {
    const playOverlay = document.createElement('div');
    playOverlay.className = 'play-overlay';
    playOverlay.innerHTML = '<div class="play-button"></div>';
    container.appendChild(playOverlay);

    playOverlay.addEventListener('click', function() {
        videoElement.play()
            .then(() => {
                playOverlay.remove();
            })
            .catch(error => {
                console.error('Play failed:', error);
            });
    });
}

/**
 * Handle video player errors
 */
function handleVideoError(streamName, message) {
    const containerId = `player-${streamName.replace(/\s+/g, '-')}`;
    const container = document.getElementById(containerId);

    if (!container) return;

    container.classList.remove('loading');
    container.classList.add('error');

    const errorMessage = document.createElement('div');
    errorMessage.className = 'error-message';
    errorMessage.innerHTML = `
        <div class="error-icon">!</div>
        <p>${message || 'Stream connection failed'}</p>
        <button class="retry-button">Retry</button>
    `;

    // Remove any existing error message
    const existingError = container.querySelector('.error-message');
    if (existingError) {
        existingError.remove();
    }

    container.appendChild(errorMessage);

    // Add retry button handler
    const retryButton = errorMessage.querySelector('.retry-button');
    if (retryButton) {
        retryButton.addEventListener('click', function() {
            // Fetch stream info again and reinitialize
            fetch(`/api/streams/${encodeURIComponent(streamName)}`)
                .then(response => response.json())
                .then(streamInfo => {
                    // Remove error message
                    errorMessage.remove();
                    container.classList.remove('error');

                    // Cleanup existing player if any
                    cleanupVideoPlayer(streamName);

                    // Reinitialize
                    initializeVideoPlayer(streamInfo);
                })
                .catch(error => {
                    console.error('Error fetching stream info:', error);
                    alert('Could not reconnect to stream: ' + error.message);
                });
        });
    }
}

/**
 * Cleanup video player resources when switching pages or streams
 */
function cleanupVideoPlayer(streamName) {
    const containerId = `player-${streamName.replace(/\s+/g, '-')}`;
    const container = document.getElementById(containerId);

    if (!container) return;

    // Destroy HLS instance if exists
    if (container.hlsPlayer) {
        container.hlsPlayer.destroy();
        delete container.hlsPlayer;
    }

    // Reset video element
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);

    if (videoElement) {
        videoElement.pause();
        videoElement.removeAttribute('src');
        videoElement.load();
    }

    // Reset container state
    container.classList.remove('loading', 'error');

    // Remove error message if any
    const errorMessage = container.querySelector('.error-message');
    if (errorMessage) {
        errorMessage.remove();
    }

    // Remove play overlay if any
    const playOverlay = container.querySelector('.play-overlay');
    if (playOverlay) {
        playOverlay.remove();
    }
}

/**
 * Take a snapshot of a stream
 */
function takeSnapshot(streamName) {
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const containerId = `player-${streamName.replace(/\s+/g, '-')}`;
    const container = document.getElementById(containerId);

    if (!videoElement || !container) {
        alert('Stream not found');
        return;
    }

    // Create a canvas to capture the image
    const canvas = document.createElement('canvas');

    // If using poster (in our simulation), use that size
    if (videoElement.poster) {
        const img = new Image();
        img.onload = function() {
            canvas.width = img.width;
            canvas.height = img.height;

            const ctx = canvas.getContext('2d');
            ctx.drawImage(img, 0, 0);

            // Add a timestamp
            const now = new Date();
            const timestamp = now.toLocaleString();

            ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
            ctx.fillRect(0, canvas.height - 30, canvas.width, 30);

            ctx.fillStyle = 'white';
            ctx.font = '14px Arial';
            ctx.textAlign = 'left';
            ctx.fillText(`${streamName} - ${timestamp}`, 10, canvas.height - 10);

            // Trigger download
            downloadSnapshot(canvas, streamName);
        };
        img.src = videoElement.poster;
    }
    // For real video, use the video dimensions
    else if (videoElement.videoWidth && videoElement.videoHeight) {
        canvas.width = videoElement.videoWidth;
        canvas.height = videoElement.videoHeight;

        const ctx = canvas.getContext('2d');
        ctx.drawImage(videoElement, 0, 0, canvas.width, canvas.height);

        // Add timestamp
        const now = new Date();
        const timestamp = now.toLocaleString();

        ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
        ctx.fillRect(0, canvas.height - 30, canvas.width, 30);

        ctx.fillStyle = 'white';
        ctx.font = '14px Arial';
        ctx.textAlign = 'left';
        ctx.fillText(`${streamName} - ${timestamp}`, 10, canvas.height - 10);

        // Trigger download
        downloadSnapshot(canvas, streamName);
    } else {
        alert('Cannot take snapshot: video not loaded');
    }
}

/**
 * Download a snapshot
 */
function downloadSnapshot(canvas, streamName) {
    try {
        // Create a formatted timestamp for the filename
        const now = new Date();
        const dateStr = now.toISOString().slice(0, 10).replace(/-/g, '');
        const timeStr = now.toTimeString().slice(0, 8).replace(/:/g, '');
        const filename = `${streamName.replace(/\s+/g, '_')}_${dateStr}_${timeStr}.jpg`;

        // Create a link and trigger download
        const link = document.createElement('a');
        link.download = filename;
        link.href = canvas.toDataURL('image/jpeg', 0.8);
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);

        // Show success message
        const statusElement = document.getElementById('status-text');
        if (statusElement) {
            const originalText = statusElement.textContent;
            statusElement.textContent = `Snapshot saved: ${filename}`;

            // Reset status after a delay
            setTimeout(() => {
                statusElement.textContent = originalText;
            }, 3000);
        }
    } catch (error) {
        console.error('Error saving snapshot:', error);
        alert('Error saving snapshot: ' + error.message);
    }
}

/**
 * Toggle fullscreen for a specific stream
 */
function toggleStreamFullscreen(streamName) {
    const containerId = `player-${streamName.replace(/\s+/g, '-')}`;
    const container = document.getElementById(containerId);

    if (!container) {
        alert('Stream not found');
        return;
    }

    if (!document.fullscreenElement) {
        container.requestFullscreen().catch(err => {
            console.error(`Error attempting to enable fullscreen: ${err.message}`);
            alert(`Could not enable fullscreen mode: ${err.message}`);
        });
    } else {
        document.exitFullscreen();
    }
}

/**
 * Add CSS for the video player
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
        .video-player {
            position: relative;
            width: 100%;
            height: 0;
            padding-bottom: 56.25%; /* 16:9 aspect ratio */
            background-color: #000;
            overflow: hidden;
        }
        
        .video-player video {
            position: absolute;
            top: 0;
            left: 0;
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
    `;

    document.head.appendChild(style);
}

/**
 * Update video layout
 */
function updateVideoLayout(layout) {
    // Get current streams
    const videoGrid = document.getElementById('video-grid');
    if (!videoGrid) return;
    
    // Set grid columns based on layout
    if (layout === '1') {
        videoGrid.style.gridTemplateColumns = '1fr';
    } else if (layout === '4') {
        videoGrid.style.gridTemplateColumns = 'repeat(2, 1fr)';
    } else if (layout === '9') {
        videoGrid.style.gridTemplateColumns = 'repeat(3, 1fr)';
    } else if (layout === '16') {
        videoGrid.style.gridTemplateColumns = 'repeat(4, 1fr)';
    }
}

/**
 * Toggle fullscreen mode
 */
function toggleFullscreen() {
    const videoGrid = document.getElementById('video-grid');
    if (!videoGrid) return;
    
    if (!document.fullscreenElement) {
        videoGrid.requestFullscreen().catch(err => {
            console.error(`Error attempting to enable fullscreen: ${err.message}`);
        });
    } else {
        document.exitFullscreen();
    }
}

/**
 * Load recordings
 */
function loadRecordings() {
    const recordingsTable = document.getElementById('recordings-table');
    if (!recordingsTable) return;
    
    const tbody = recordingsTable.querySelector('tbody');
    
    showLoading(recordingsTable);
    
    // Clear existing rows
    tbody.innerHTML = '<tr><td colspan="5" class="empty-message">Loading recordings...</td></tr>';
    
    // Get filter values
    const dateFilter = document.getElementById('date-picker').value;
    const streamFilter = document.getElementById('stream-filter').value;
    
    // Build query string
    let queryString = '';
    if (dateFilter) {
        queryString += `date=${encodeURIComponent(dateFilter)}`;
    }
    if (streamFilter && streamFilter !== 'all') {
        if (queryString) queryString += '&';
        queryString += `stream=${encodeURIComponent(streamFilter)}`;
    }
    if (queryString) {
        queryString = '?' + queryString;
    }
    
    // Fetch recordings from API
    // In a real implementation, this would be an API endpoint
    // For now, we'll use sample data
    
    // Simulate API call delay
    setTimeout(() => {
        // Sample recordings data
        const recordings = [
            {
                id: 1,
                stream: 'Front Door',
                start_time: '2025-03-06 10:00:00',
                duration: '15:00',
                size: '125 MB'
            },
            {
                id: 2,
                stream: 'Front Door',
                start_time: '2025-03-06 10:15:00',
                duration: '15:00',
                size: '130 MB'
            },
            {
                id: 3,
                stream: 'Back Yard',
                start_time: '2025-03-06 10:00:00',
                duration: '15:00',
                size: '110 MB'
            },
            {
                id: 4,
                stream: 'Back Yard',
                start_time: '2025-03-06 10:15:00',
                duration: '15:00',
                size: '115 MB'
            }
        ];
        
        // Filter recordings based on stream filter
        let filteredRecordings = recordings;
        if (streamFilter && streamFilter !== 'all') {
            filteredRecordings = recordings.filter(recording => recording.stream === streamFilter);
        }
        
        tbody.innerHTML = '';
        
        if (filteredRecordings.length === 0) {
            tbody.innerHTML = '<tr><td colspan="5" class="empty-message">No recordings found</td></tr>';
            hideLoading(recordingsTable);
            return;
        }
        
        filteredRecordings.forEach(recording => {
            const tr = document.createElement('tr');
            
            tr.innerHTML = `
                <td>${recording.stream}</td>
                <td>${recording.start_time}</td>
                <td>${recording.duration}</td>
                <td>${recording.size}</td>
                <td>
                    <button class="btn-primary play-recording" data-id="${recording.id}">Play</button>
                    <button class="btn download-recording" data-id="${recording.id}">Download</button>
                    <button class="btn-danger delete-recording" data-id="${recording.id}">Delete</button>
                </td>
            `;
            
            tbody.appendChild(tr);
        });
        
        // Add event listeners for play, download, and delete buttons
        document.querySelectorAll('.play-recording').forEach(button => {
            button.addEventListener('click', function() {
                const recordingId = this.getAttribute('data-id');
                playRecording(recordingId);
            });
        });
        
        document.querySelectorAll('.download-recording').forEach(button => {
            button.addEventListener('click', function() {
                const recordingId = this.getAttribute('data-id');
                downloadRecording(recordingId);
            });
        });
        
        document.querySelectorAll('.delete-recording').forEach(button => {
            button.addEventListener('click', function() {
                const recordingId = this.getAttribute('data-id');
                if (confirm('Are you sure you want to delete this recording?')) {
                    deleteRecording(recordingId);
                }
            });
        });
        
        hideLoading(recordingsTable);
    }, 500);
}

/**
 * Load settings
 */
function loadSettings() {
    const settingsContainer = document.querySelector('.settings-container');
    if (!settingsContainer) return;
    
    showLoading(settingsContainer);
    
    // Fetch settings from the server
    fetch('/api/settings')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load settings');
            }
            return response.json();
        })
        .then(settings => {
            // Update form fields with settings values
            document.getElementById('setting-log-level').value = settings.log_level;
            document.getElementById('setting-storage-path').value = settings.storage_path;
            document.getElementById('setting-max-storage').value = settings.max_storage;
            document.getElementById('setting-retention').value = settings.retention;
            document.getElementById('setting-auto-delete').checked = settings.auto_delete;
            document.getElementById('setting-web-port').value = settings.web_port;
            document.getElementById('setting-auth-enabled').checked = settings.auth_enabled;
            document.getElementById('setting-username').value = settings.username;
            document.getElementById('setting-password').value = settings.password;
            document.getElementById('setting-buffer-size').value = settings.buffer_size;
            document.getElementById('setting-use-swap').checked = settings.use_swap;
            document.getElementById('setting-swap-size').value = settings.swap_size;
        })
        .catch(error => {
            console.error('Error loading settings:', error);
            alert('Error loading settings. Please try again.');
        })
        .finally(() => {
            hideLoading(settingsContainer);
        });
}

/**
 * Save settings
 */
function saveSettings() {
    const settingsContainer = document.querySelector('.settings-container');
    if (!settingsContainer) return;
    
    showLoading(settingsContainer);
    
    // Collect all settings from the form
    const settings = {
        log_level: parseInt(document.getElementById('setting-log-level').value, 10),
        storage_path: document.getElementById('setting-storage-path').value,
        max_storage: parseInt(document.getElementById('setting-max-storage').value, 10),
        retention: parseInt(document.getElementById('setting-retention').value, 10),
        auto_delete: document.getElementById('setting-auto-delete').checked,
        web_port: parseInt(document.getElementById('setting-web-port').value, 10),
        auth_enabled: document.getElementById('setting-auth-enabled').checked,
        username: document.getElementById('setting-username').value,
        password: document.getElementById('setting-password').value,
        buffer_size: parseInt(document.getElementById('setting-buffer-size').value, 10),
        use_swap: document.getElementById('setting-use-swap').checked,
        swap_size: parseInt(document.getElementById('setting-swap-size').value, 10)
    };
    
    // Send settings to the server
    fetch('/api/settings', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(settings)
    })
    .then(response => {
        if (!response.ok) {
            throw new Error('Failed to save settings');
        }
        return response.json();
    })
    .then(data => {
        alert('Settings saved successfully');
    })
    .catch(error => {
        console.error('Error saving settings:', error);
        alert('Error saving settings: ' + error.message);
    })
    .finally(() => {
        hideLoading(settingsContainer);
    });
}

/**
 * Edit stream
 */
function editStream(streamName) {
    const streamModal = document.getElementById('stream-modal');
    if (!streamModal) return;
    
    showLoading(streamModal);
    
    // Fetch stream details from API
    fetch(`/api/streams/${encodeURIComponent(streamName)}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load stream details');
            }
            return response.json();
        })
        .then(stream => {
            // Fill the form with stream data
            document.getElementById('stream-name').value = stream.name;
            document.getElementById('stream-url').value = stream.url;
            document.getElementById('stream-enabled').checked = stream.enabled;
            document.getElementById('stream-width').value = stream.width;
            document.getElementById('stream-height').value = stream.height;
            document.getElementById('stream-fps').value = stream.fps;
            document.getElementById('stream-codec').value = stream.codec;
            document.getElementById('stream-priority').value = stream.priority;
            document.getElementById('stream-record').checked = stream.record;
            document.getElementById('stream-segment').value = stream.segment_duration;
            
            // Store original name for later comparison
            streamModal.dataset.originalName = stream.name;
            
            // Show the modal
            streamModal.style.display = 'block';
        })
        .catch(error => {
            console.error('Error loading stream details:', error);
            alert('Error loading stream details: ' + error.message);
        })
        .finally(() => {
            hideLoading(streamModal);
        });
}

/**
 * Save stream
 */
function saveStream() {
    const streamModal = document.getElementById('stream-modal');
    if (!streamModal) return;
    
    showLoading(streamModal);
    
    // Collect stream data from form
    const streamData = {
        name: document.getElementById('stream-name').value,
        url: document.getElementById('stream-url').value,
        enabled: document.getElementById('stream-enabled').checked,
        width: parseInt(document.getElementById('stream-width').value, 10),
        height: parseInt(document.getElementById('stream-height').value, 10),
        fps: parseInt(document.getElementById('stream-fps').value, 10),
        codec: document.getElementById('stream-codec').value,
        priority: parseInt(document.getElementById('stream-priority').value, 10),
        record: document.getElementById('stream-record').checked,
        segment_duration: parseInt(document.getElementById('stream-segment').value, 10)
    };
    
    // Validate required fields
    if (!streamData.name || !streamData.url) {
        alert('Name and URL are required');
        hideLoading(streamModal);
        return;
    }
    
    // Check if this is a new stream or an update
    const originalName = streamModal.dataset.originalName;
    let method = 'POST';
    let url = '/api/streams';
    
    if (originalName) {
        method = 'PUT';
        url = `/api/streams/${encodeURIComponent(originalName)}`;
    }
    
    // Send to API
    fetch(url, {
        method: method,
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(streamData)
    })
    .then(response => {
        if (!response.ok) {
            throw new Error('Failed to save stream');
        }
        return response.json();
    })
    .then(data => {
        // Close the modal
        streamModal.style.display = 'none';
        
        // Clear original name
        delete streamModal.dataset.originalName;
        
        // Reload streams
        loadStreams();
        
        alert('Stream saved successfully');
    })
    .catch(error => {
        console.error('Error saving stream:', error);
        alert('Error saving stream: ' + error.message);
    })
    .finally(() => {
        hideLoading(streamModal);
    });
}

/**
 * Delete stream
 */
function deleteStream(streamName) {
    if (!confirm(`Are you sure you want to delete stream "${streamName}"?`)) {
        return;
    }
    
    const streamsTable = document.getElementById('streams-table');
    if (streamsTable) {
        showLoading(streamsTable);
    }
    
    // Send delete request to API
    fetch(`/api/streams/${encodeURIComponent(streamName)}`, {
        method: 'DELETE'
    })
    .then(response => {
        if (!response.ok) {
            throw new Error('Failed to delete stream');
        }
        return response.json();
    })
    .then(data => {
        // Reload streams
        loadStreams();
        
        alert(`Stream "${streamName}" deleted successfully`);
    })
    .catch(error => {
        console.error('Error deleting stream:', error);
        alert('Error deleting stream: ' + error.message);
        
        if (streamsTable) {
            hideLoading(streamsTable);
        }
    });
}

/**
 * Test stream connection
 */
function testStream() {
    const streamModal = document.getElementById('stream-modal');
    if (!streamModal) return;
    
    // Get URL from form
    const url = document.getElementById('stream-url').value;
    if (!url) {
        alert('Please enter a stream URL');
        return;
    }
    
    showLoading(streamModal);
    
    // Send test request to API
    fetch('/api/streams/test', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ url: url })
    })
    .then(response => {
        if (!response.ok) {
            throw new Error('Failed to test stream connection');
        }
        return response.json();
    })
    .then(data => {
        if (data.success) {
            alert(`Stream connection test successful for URL: ${url}`);
            
            // Optionally auto-fill some form fields based on detected stream info
            if (data.details) {
                if (data.details.codec) {
                    document.getElementById('stream-codec').value = data.details.codec;
                }
                if (data.details.width && data.details.height) {
                    document.getElementById('stream-width').value = data.details.width;
                    document.getElementById('stream-height').value = data.details.height;
                }
                if (data.details.fps) {
                    document.getElementById('stream-fps').value = data.details.fps;
                }
            }
        } else {
            alert(`Stream connection test failed: ${data.error || 'Unknown error'}`);
        }
    })
    .catch(error => {
        console.error('Error testing stream connection:', error);
        alert('Error testing stream connection: ' + error.message);
    })
    .finally(() => {
        hideLoading(streamModal);
    });
}

/**
 * Play recording
 */
function playRecording(recordingId) {
    // In a real implementation, this would load the recording URL
    // For now, we'll just show the video modal with a placeholder message
    
    const videoModal = document.getElementById('video-modal');
    const videoPlayer = document.getElementById('video-player');
    const videoTitle = document.getElementById('video-modal-title');
    
    // Set video title
    videoTitle.textContent = `Recording Playback (ID: ${recordingId})`;
    
    // In a real implementation, we would set the video source
    // For now, we'll just show a message
    videoPlayer.innerHTML = `<div style="display:flex;justify-content:center;align-items:center;height:300px;background:#000;color:#fff;">
        <p>Video playback would be shown here (Recording ID: ${recordingId})</p>
    </div>`;
    
    // Show the modal
    videoModal.style.display = 'block';
}

/**
 * Download recording
 */
function downloadRecording(recordingId) {
    // In a real implementation, this would initiate a download
    // For demonstration purposes, we'll just show a message
    
    alert(`Download started for recording ID: ${recordingId}`);
    
    // In a real implementation, you would use something like:
    // window.location.href = `/api/recordings/${recordingId}/download`;
}

/**
 * Delete recording
 */
function deleteRecording(recordingId) {
    if (!confirm('Are you sure you want to delete this recording?')) {
        return;
    }
    
    const recordingsTable = document.getElementById('recordings-table');
    if (recordingsTable) {
        showLoading(recordingsTable);
    }
    
    // In a real implementation, this would make an API call to delete the recording
    // For demonstration purposes, we'll just simulate success
    
    setTimeout(() => {
        alert(`Recording ID: ${recordingId} deleted successfully`);
        loadRecordings();
    }, 500);
}

/**
 * Restart service
 */
function restartService() {
    if (!confirm('Are you sure you want to restart the LightNVR service?')) {
        return;
    }
    
    const systemContainer = document.querySelector('.system-container');
    if (systemContainer) {
        showLoading(systemContainer);
    }
    
    // In a real implementation, this would make an API call to restart the service
    // For demonstration purposes, we'll just simulate success
    
    setTimeout(() => {
        alert('Service restart initiated');
        
        if (systemContainer) {
            hideLoading(systemContainer);
        }
        
        // Update status
        document.getElementById('status-indicator').className = 'status-indicator status-warning';
        document.getElementById('status-text').textContent = 'System restarting...';
        
        // After a delay, reload everything
        setTimeout(() => {
            document.getElementById('status-indicator').className = 'status-indicator status-ok';
            document.getElementById('status-text').textContent = 'System running normally';
            loadInitialData();
        }, 3000);
    }, 1000);
}

/**
 * Shutdown service
 */
function shutdownService() {
    if (!confirm('Are you sure you want to shutdown the LightNVR service?')) {
        return;
    }
    
    const systemContainer = document.querySelector('.system-container');
    if (systemContainer) {
        showLoading(systemContainer);
    }
    
    // In a real implementation, this would make an API call to shutdown the service
    // For demonstration purposes, we'll just simulate success
    
    setTimeout(() => {
        alert('Service shutdown initiated');
        
        if (systemContainer) {
            hideLoading(systemContainer);
        }
        
        // Update status
        document.getElementById('status-indicator').className = 'status-indicator status-error';
        document.getElementById('status-text').textContent = 'System shutting down...';
    }, 1000);
}

/**
 * Clear logs
 */
function clearLogs() {
    if (!confirm('Are you sure you want to clear the logs?')) {
        return;
    }
    
    const logsContainer = document.getElementById('system-logs');
    if (!logsContainer) return;
    
    // In a real implementation, this would make an API call to clear the logs
    // For demonstration purposes, we'll just clear the logs display
    
    logsContainer.textContent = 'Logs cleared';
    alert('Logs cleared successfully');
}

/**
 * Backup configuration
 */
function backupConfig() {
    // In a real implementation, this would make an API call to backup the configuration
    // For demonstration purposes, we'll just show a success message
    
    alert('Configuration backup created successfully');
    
    // In a real implementation, you might initiate a download:
    // window.location.href = '/api/system/backup/download';
}

/**
 * Logout
 */
function logout() {
    if (!confirm('Are you sure you want to logout?')) {
        return;
    }
    
    // In a real implementation, this would make an API call to logout
    // For demonstration purposes, we'll just show a message and redirect
    
    alert('Logout successful');
    
    // Redirect to login page
    // window.location.href = '/login.html';
}


/**
 * Setup Snapshot Preview Modal
 */
function setupSnapshotModal() {
    // Get modal elements
    const modal = document.getElementById('snapshot-preview-modal');
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
    modal.style.display = 'flex';
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
 * Enhanced Take Snapshot function
 */
function takeSnapshot(streamName) {
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const containerId = `player-${streamName.replace(/\s+/g, '-')}`;
    const container = document.getElementById(containerId);

    if (!videoElement || !container) {
        alert('Stream not found');
        return;
    }

    // Show status message
    showStatusMessage('Taking snapshot...');

    // Create a canvas to capture the image
    const canvas = document.createElement('canvas');

    // If using poster (in our simulation), use that size
    if (videoElement.poster) {
        const img = new Image();
        img.onload = function() {
            canvas.width = img.width;
            canvas.height = img.height;

            const ctx = canvas.getContext('2d');
            ctx.drawImage(img, 0, 0);

            // Add a timestamp
            const now = new Date();
            const timestamp = now.toLocaleString();

            ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
            ctx.fillRect(0, canvas.height - 30, canvas.width, 30);

            ctx.fillStyle = 'white';
            ctx.font = '14px Arial';
            ctx.textAlign = 'left';
            ctx.fillText(`${streamName} - ${timestamp}`, 10, canvas.height - 10);

            // Show preview instead of direct download
            const imageData = canvas.toDataURL('image/jpeg', 0.9);
            showSnapshotPreview(imageData, streamName);
        };
        img.src = videoElement.poster;
    }
    // For real video, use the video dimensions
    else if (videoElement.videoWidth && videoElement.videoHeight) {
        canvas.width = videoElement.videoWidth;
        canvas.height = videoElement.videoHeight;

        const ctx = canvas.getContext('2d');
        ctx.drawImage(videoElement, 0, 0, canvas.width, canvas.height);

        // Add timestamp
        const now = new Date();
        const timestamp = now.toLocaleString();

        ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
        ctx.fillRect(0, canvas.height - 30, canvas.width, 30);

        ctx.fillStyle = 'white';
        ctx.font = '14px Arial';
        ctx.textAlign = 'left';
        ctx.fillText(`${streamName} - ${timestamp}`, 10, canvas.height - 10);

        // Show preview instead of direct download
        const imageData = canvas.toDataURL('image/jpeg', 0.9);
        showSnapshotPreview(imageData, streamName);
    } else {
        showStatusMessage('Cannot take snapshot: video not loaded', 3000);
    }
}