/**
 * LightNVR Web Interface Video Player
 * Contains functionality for video playback and player management
 */

/**
 * Initialize video player for a stream using HLS.js
 */
function initializeVideoPlayer(stream) {
    const videoElementId = `video-${stream.name.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;

    if (!videoElement || !videoCell) return;
    
    // Add stream name overlay to the upper left corner if it doesn't exist
    let streamNameOverlay = videoCell.querySelector('.stream-name-overlay');
    if (!streamNameOverlay) {
        streamNameOverlay = document.createElement('div');
        streamNameOverlay.className = 'stream-name-overlay';
        streamNameOverlay.textContent = stream.name;
        streamNameOverlay.style.position = 'absolute';
        streamNameOverlay.style.top = '10px';
        streamNameOverlay.style.left = '10px';
        streamNameOverlay.style.padding = '5px 10px';
        streamNameOverlay.style.backgroundColor = 'rgba(0, 0, 0, 0.5)';
        streamNameOverlay.style.color = 'white';
        streamNameOverlay.style.borderRadius = '4px';
        streamNameOverlay.style.fontSize = '14px';
        streamNameOverlay.style.zIndex = '15';
        videoCell.appendChild(streamNameOverlay);
    }
    
    // Remove stream name from stream-info if it exists
    const streamInfo = videoCell.querySelector('.stream-info');
    if (streamInfo) {
        const streamNameElement = streamInfo.querySelector('span:first-child');
        if (streamNameElement && streamNameElement.textContent === stream.name) {
            streamNameElement.remove();
        }
    }
    
    // Create canvas overlay for detection bounding boxes
    const canvasId = `canvas-${stream.name.replace(/\s+/g, '-')}`;
    let canvasOverlay = document.getElementById(canvasId);
    
    if (!canvasOverlay) {
        canvasOverlay = document.createElement('canvas');
        canvasOverlay.id = canvasId;
        canvasOverlay.className = 'detection-overlay';
        canvasOverlay.style.position = 'absolute';
        canvasOverlay.style.top = '0';
        canvasOverlay.style.left = '0';
        canvasOverlay.style.width = '100%';
        canvasOverlay.style.height = '100%';
        canvasOverlay.style.pointerEvents = 'none'; // Allow clicks to pass through
        videoCell.appendChild(canvasOverlay);
    }
    
    // Start detection polling if detection is enabled for this stream
    console.log(`Stream ${stream.name} detection settings:`, {
        detection_based_recording: stream.detection_based_recording,
        detection_model: stream.detection_model,
        detection_threshold: stream.detection_threshold
    });
    
    if (stream.detection_based_recording && stream.detection_model) {
        console.log(`Starting detection polling for stream ${stream.name}`);
        startDetectionPolling(stream.name, canvasOverlay, videoElement);
    } else {
        console.log(`Detection not enabled for stream ${stream.name}`);
    }

    // Show loading state
    const loadingIndicator = videoCell.querySelector('.loading-indicator');
    if (loadingIndicator) {
        loadingIndicator.style.display = 'flex';
    }

    // Build the HLS stream URL with cache-busting timestamp to prevent stale data
    // Your backend needs to convert RTSP to HLS using FFmpeg
    const timestamp = Date.now();
    const hlsStreamUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8?_t=${timestamp}`;

    // Check if HLS is supported natively
    if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
        // Native HLS support (Safari)
        videoElement.src = hlsStreamUrl;
        videoElement.addEventListener('loadedmetadata', function() {
            if (loadingIndicator) {
                loadingIndicator.style.display = 'none';
            }
            
            // Show stream controls when stream is successfully loaded
            const streamControls = videoCell.querySelector('.stream-controls');
            if (streamControls) {
                streamControls.style.display = 'flex';
            }
        });

        videoElement.addEventListener('error', function() {
            handleVideoError(stream.name);
        });
    }
    // Use HLS.js for browsers that don't support HLS natively
    else if (Hls && Hls.isSupported()) {
        const hls = new Hls({
            maxBufferLength: 30,            // Increased from 10 to 30 seconds for better buffering on low-power devices
            maxMaxBufferLength: 60,         // Increased from 20 to 60 seconds for better buffering on low-power devices
            liveSyncDurationCount: 4,       // Increased from 3 to 4 segments for better stability
            liveMaxLatencyDurationCount: 10, // Increased from 5 to 10 segments for better stability on low-power devices
            liveDurationInfinity: false,    // Don't treat live streams as infinite duration
            lowLatencyMode: false,          // Disable low latency mode for better stability on low-power devices
            enableWorker: true,
            fragLoadingTimeOut: 30000,      // Increased from 20 to 30 seconds timeout for fragment loading
            manifestLoadingTimeOut: 20000,  // Increased from 15 to 20 seconds timeout for manifest loading
            levelLoadingTimeOut: 20000,     // Increased from 15 to 20 seconds timeout for level loading
            backBufferLength: 60,           // Add back buffer length to keep more segments in memory
            startLevel: -1,                 // Auto-select quality level based on network conditions
            abrEwmaDefaultEstimate: 500000, // Start with a lower bandwidth estimate (500kbps)
            abrBandWidthFactor: 0.7,        // Be more conservative with bandwidth estimates
            abrBandWidthUpFactor: 0.5,      // Be more conservative when increasing quality
            // fMP4 specific settings
            fLoader: undefined,             // Use default fragment loader
            pLoader: undefined,             // Use default playlist loader
            cmcd: undefined,                // No CMCD (Common Media Client Data) parameters
            enableDateRangeMetadataCues: true, // Enable date range metadata
            progressive: false,             // Not using progressive download
            lowLatencyMode: false           // Not using low latency mode
        });

        hls.loadSource(hlsStreamUrl);
        hls.attachMedia(videoElement);

        hls.on(Hls.Events.MANIFEST_PARSED, function() {
            if (loadingIndicator) {
                loadingIndicator.style.display = 'none';
            }
            
            // Show stream controls when stream is successfully loaded
            const streamControls = videoCell.querySelector('.stream-controls');
            if (streamControls) {
                streamControls.style.display = 'flex';
            }
            
            videoElement.play().catch(error => {
                console.warn('Auto-play prevented:', error);
                // Add play button overlay for user interaction
                addPlayButtonOverlay(videoCell, videoElement);
            });
        });

        hls.on(Hls.Events.ERROR, function(event, data) {
            if (data.fatal) {
                console.error('HLS error:', data);
                hls.destroy();
                
                // Check if the stream was recently enabled
                const videoCell = videoElement.closest('.video-cell');
                const loadingIndicator = videoCell.querySelector('.loading-indicator');
                
                // If the stream was recently enabled (indicated by the loading message),
                // automatically retry after a short delay
                if (loadingIndicator && 
                    loadingIndicator.querySelector('span').textContent === 'Starting stream...') {
                    
                    console.log(`Stream ${stream.name} failed to load after enabling, retrying in 2 seconds...`);
                    
                    // Show retry message
                    loadingIndicator.querySelector('span').textContent = 'Retrying connection...';
                    
                    // Retry after a delay
                    setTimeout(() => {
                        console.log(`Retrying stream ${stream.name} after failure`);
                        // Fetch updated stream info and reinitialize
                        fetch(`/api/streams/${encodeURIComponent(stream.name)}`)
                            .then(response => response.json())
                            .then(updatedStream => {
                                // Cleanup existing player
                                cleanupVideoPlayer(stream.name);
                                // Reinitialize with updated stream info
                                initializeVideoPlayer(updatedStream);
                            })
                            .catch(error => {
                                console.error(`Error fetching stream info for retry: ${error}`);
                                handleVideoError(stream.name, 'Failed to reconnect after enabling');
                            });
                    }, 2000);
                } else {
                    // Regular error handling for non-startup errors
                    handleVideoError(stream.name);
                }
            }
        });

        // Set up less frequent refresh to reduce load on low-power devices
        const refreshInterval = 60000; // 60 seconds (increased from 30 seconds)
        const refreshTimer = setInterval(() => {
            if (videoCell && videoCell.hlsPlayer) {
                console.log(`Refreshing HLS stream for ${stream.name}`);
                const newTimestamp = Date.now();
                const newUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8?_t=${newTimestamp}`;
                videoCell.hlsPlayer.loadSource(newUrl);
            } else {
                // Clear interval if video cell or player no longer exists
                clearInterval(refreshTimer);
            }
        }, refreshInterval);
        
        // Store hls instance and timer for cleanup
        videoCell.hlsPlayer = hls;
        videoCell.refreshTimer = refreshTimer;
    }
    // Fallback for unsupported browsers
    else {
        handleVideoError(stream.name, 'HLS not supported by your browser');
    }
}

/**
 * Add play button overlay for browsers that block autoplay
 */
function addPlayButtonOverlay(videoCell, videoElement) {
    const playOverlay = document.createElement('div');
    playOverlay.className = 'play-overlay';
    playOverlay.innerHTML = '<div class="play-button"></div>';
    videoCell.appendChild(playOverlay);

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
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;

    if (!videoCell) return;

    // Hide loading indicator
    const loadingIndicator = videoCell.querySelector('.loading-indicator');
    if (loadingIndicator) {
        loadingIndicator.style.display = 'none';
    }
    
    // Hide stream controls when there's an error
    const streamControls = videoCell.querySelector('.stream-controls');
    if (streamControls) {
        streamControls.style.display = 'none';
    }

    // Create error indicator if it doesn't exist
    let errorIndicator = videoCell.querySelector('.error-indicator');
    if (!errorIndicator) {
        errorIndicator = document.createElement('div');
        errorIndicator.className = 'error-indicator';
        videoCell.appendChild(errorIndicator);
    }

    errorIndicator.innerHTML = `
        <div class="error-icon">!</div>
        <p>${message || 'Stream connection failed'}</p>
        <button class="retry-button">Retry</button>
    `;

    // Add retry button handler
    const retryButton = errorIndicator.querySelector('.retry-button');
    if (retryButton) {
        retryButton.addEventListener('click', function() {
            // Show loading indicator again
            if (loadingIndicator) {
                loadingIndicator.style.display = 'flex';
            }
            
            // Hide error indicator
            errorIndicator.style.display = 'none';
            
            // Fetch stream info again and reinitialize
            fetch(`/api/streams/${encodeURIComponent(streamName)}`)
                .then(response => response.json())
                .then(streamInfo => {
                    // Cleanup existing player if any
                    cleanupVideoPlayer(streamName);

                    // Reinitialize
                    initializeVideoPlayer(streamInfo);
                })
                .catch(error => {
                    console.error('Error fetching stream info:', error);
                    
                    // Show error indicator again with new message
                    errorIndicator.style.display = 'flex';
                    const errorMsg = errorIndicator.querySelector('p');
                    if (errorMsg) {
                        errorMsg.textContent = 'Could not reconnect: ' + error.message;
                    }
                    
                    // Hide loading indicator
                    if (loadingIndicator) {
                        loadingIndicator.style.display = 'none';
                    }
                });
        });
    }
}

/**
 * Cleanup video player resources when switching pages or streams
 */
function cleanupVideoPlayer(streamName) {
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;

    if (!videoCell) return;

    // Destroy HLS instance and clear refresh timer if they exist
    if (videoCell.hlsPlayer) {
        videoCell.hlsPlayer.destroy();
        delete videoCell.hlsPlayer;
    }
    
    if (videoCell.refreshTimer) {
        clearInterval(videoCell.refreshTimer);
        delete videoCell.refreshTimer;
    }

    // Reset video element
    if (videoElement) {
        videoElement.pause();
        videoElement.removeAttribute('src');
        videoElement.load();
    }

    // Reset loading indicator
    const loadingIndicator = videoCell.querySelector('.loading-indicator');
    if (loadingIndicator) {
        loadingIndicator.style.display = 'none';
    }
    
    // Hide stream controls
    const streamControls = videoCell.querySelector('.stream-controls');
    if (streamControls) {
        streamControls.style.display = 'none';
    }

    // Remove error indicator if any
    const errorIndicator = videoCell.querySelector('.error-indicator');
    if (errorIndicator) {
        errorIndicator.remove();
    }

    // Remove play overlay if any
    const playOverlay = videoCell.querySelector('.play-overlay');
    if (playOverlay) {
        playOverlay.remove();
    }
    
    // Clean up detection polling if active
    const canvasId = `canvas-${streamName.replace(/\s+/g, '-')}`;
    const canvasOverlay = document.getElementById(canvasId);
    if (canvasOverlay && canvasOverlay.detectionInterval) {
        clearInterval(canvasOverlay.detectionInterval);
        delete canvasOverlay.detectionInterval;
    }
}

/**
 * Take a snapshot of the current video frame
 */
function takeSnapshot(streamId) {
    // Find the stream by ID or name
    const streamElement = document.querySelector(`.snapshot-btn[data-id="${streamId}"]`);
    if (!streamElement) {
        console.error('Stream element not found for ID:', streamId);
        return;
    }

    // Get the stream name from the data attribute
    const streamName = streamElement.getAttribute('data-name');
    if (!streamName) {
        console.error('Stream name not found for ID:', streamId);
        return;
    }

    // Find the video element
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    if (!videoElement) {
        console.error('Video element not found for stream:', streamName);
        return;
    }

    // Create a canvas element to capture the frame
    const canvas = document.createElement('canvas');
    canvas.width = videoElement.videoWidth;
    canvas.height = videoElement.videoHeight;

    // Check if we have valid dimensions
    if (canvas.width === 0 || canvas.height === 0) {
        console.error('Invalid video dimensions:', canvas.width, canvas.height);
        showStatusMessage('Cannot take snapshot: Video not loaded or has invalid dimensions', 5000);
        return;
    }

    // Draw the current frame to the canvas
    const ctx = canvas.getContext('2d');
    ctx.drawImage(videoElement, 0, 0, canvas.width, canvas.height);

    try {
        // Convert the canvas to a data URL (JPEG image)
        const imageData = canvas.toDataURL('image/jpeg', 0.95);
        
        // Show the snapshot in the preview modal
        showSnapshotPreview(imageData, streamName);
    } catch (error) {
        console.error('Error creating snapshot:', error);
        showStatusMessage('Failed to create snapshot: ' + error.message, 5000);
    }
}

// Export functions
window.initializeVideoPlayer = initializeVideoPlayer;
window.addPlayButtonOverlay = addPlayButtonOverlay;
window.handleVideoError = handleVideoError;
window.cleanupVideoPlayer = cleanupVideoPlayer;
window.takeSnapshot = takeSnapshot;
