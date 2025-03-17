/**
 * LightNVR Web Interface Video Player
 * Contains functionality for video playback, layout, fullscreen
 */


/**
 * Load streams for live viewing, respecting their streaming_enabled state
 */
function enableAllStreams() {
    console.log('Loading streams for live view');
    
    // Fetch streams from API
    fetch('/api/streams')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load streams');
            }
            return response.json();
        })
        .then(streams => {
            if (!streams || streams.length === 0) {
                console.log('No streams to load');
                return;
            }
            
            // Process each stream based on its streaming_enabled state
            streams.forEach(stream => {
                const streamId = stream.id || stream.name;
                const isEnabled = stream.streaming_enabled !== false; // Default to true if not specified
                
                console.log(`Stream ${streamId} streaming_enabled state: ${isEnabled}`);
                
                // Only toggle if needed (if the current state doesn't match what we want)
                // This only affects the streaming_enabled flag, not the enabled flag
                fetch(`/api/streams/${encodeURIComponent(streamId)}/toggle_streaming`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ enabled: isEnabled })
                })
                .then(response => {
                    if (!response.ok) {
                        throw new Error('Failed to set stream state');
                    }
                    return response.json();
                })
                .then(data => {
                    console.log(`Stream ${streamId} state set successfully:`, data);
                })
                .catch(error => {
                    console.error(`Error setting stream ${streamId} state:`, error);
                });
            });
        })
        .catch(error => {
            console.error('Error loading streams for live view:', error);
        });
}

/**
 * Stop all streams when leaving the streams page
 */
function stopAllStreams() {
    console.log('Stopping all streams');
    
    // Fetch streams from API
    fetch('/api/streams')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load streams');
            }
            return response.json();
        })
        .then(streams => {
            if (!streams || streams.length === 0) {
                console.log('No streams to stop');
                return;
            }
            
            // Stop each stream
            streams.forEach(stream => {
                const streamId = stream.id || stream.name;
                console.log(`Stopping stream ${streamId}`);
                
                // Send toggle request to API to disable streaming (only affects streaming, not the enabled state)
                fetch(`/api/streams/${encodeURIComponent(streamId)}/toggle_streaming`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ enabled: false })
                })
                .then(response => {
                    if (!response.ok) {
                        throw new Error('Failed to stop stream');
                    }
                    return response.json();
                })
                .then(data => {
                    console.log(`Stream ${streamId} stopped successfully:`, data);
                })
                .catch(error => {
                    console.error(`Error stopping stream ${streamId}:`, error);
                });
            });
        })
        .catch(error => {
            console.error('Error loading streams to stop:', error);
        });
}

/**
 * Update video layout
 */
function updateVideoLayout(layout) {
    const videoGrid = document.getElementById('video-grid');
    const streamSelector = document.getElementById('stream-selector');
    if (!videoGrid) return;

    // Remove all layout classes
    videoGrid.classList.remove('layout-1', 'layout-4', 'layout-9', 'layout-16');
    
    // Add selected layout class
    videoGrid.classList.add(`layout-${layout}`);
    
    // Show/hide stream selector based on layout
    if (layout === '1' && streamSelector) {
        streamSelector.style.display = 'inline-block';
        
        // If we're switching to single view, show only the selected stream
        // or the first stream if none is selected
        const selectedStreamName = streamSelector.value;
        const videoCells = videoGrid.querySelectorAll('.video-cell');
        
        if (videoCells.length > 0) {
            videoCells.forEach(cell => {
                const streamName = cell.querySelector('.stream-info span').textContent;
                if (selectedStreamName && streamName !== selectedStreamName) {
                    cell.style.display = 'none';
                } else {
                    cell.style.display = 'flex';
                }
            });
            
            // If no stream is selected or the selected stream is not found,
            // show the first stream and update the selector
            if (!selectedStreamName || !Array.from(videoCells).some(cell => 
                cell.querySelector('.stream-info span').textContent === selectedStreamName && 
                cell.style.display !== 'none')) {
                
                const firstCell = videoCells[0];
                if (firstCell) {
                    firstCell.style.display = 'flex';
                    const firstStreamName = firstCell.querySelector('.stream-info span').textContent;
                    if (streamSelector.querySelector(`option[value="${firstStreamName}"]`)) {
                        streamSelector.value = firstStreamName;
                    }
                }
            }
        }
    } else if (streamSelector) {
        streamSelector.style.display = 'none';
        
        // Show all streams in grid view
        const videoCells = videoGrid.querySelectorAll('.video-cell');
        videoCells.forEach(cell => {
            cell.style.display = 'flex';
        });
    }
    
    // Adjust video cells if needed
    const videoCells = videoGrid.querySelectorAll('.video-cell');
    if (videoCells.length > 0) {
        // Force video elements to redraw to adjust to new layout
        videoCells.forEach(cell => {
            const video = cell.querySelector('video');
            if (video) {
                // Trigger a reflow
                video.style.display = 'none';
                setTimeout(() => {
                    video.style.display = 'block';
                }, 10);
            }
        });
    }
}

/**
 * Initialize video player for a stream using HLS.js
 */
function initializeVideoPlayer(stream) {
    const videoElementId = `video-${stream.name.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;

    if (!videoElement || !videoCell) return;
    
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
    const hlsStreamUrl = `/api/streaming/${encodeURIComponent(stream.name)}/hls/index.m3u8?_t=${timestamp}`;

    // Check if HLS is supported natively
    if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
        // Native HLS support (Safari)
        videoElement.src = hlsStreamUrl;
        videoElement.addEventListener('loadedmetadata', function() {
            if (loadingIndicator) {
                loadingIndicator.style.display = 'none';
            }
        });

        videoElement.addEventListener('error', function() {
            handleVideoError(stream.name);
        });
    }
    // Use HLS.js for browsers that don't support HLS natively
    else if (Hls && Hls.isSupported()) {
        const hls = new Hls({
            maxBufferLength: 10,            // Reduced from 60 to 10 seconds
            maxMaxBufferLength: 20,         // Reduced from 120 to 20 seconds
            liveSyncDurationCount: 3,       // Reduced from 5 to 3 segments
            liveMaxLatencyDurationCount: 5, // Maximum latency in segments
            liveDurationInfinity: false,    // Don't treat live streams as infinite duration
            lowLatencyMode: true,           // Enable low latency mode
            enableWorker: true,
            fragLoadingTimeOut: 20000,      // 20 seconds timeout for fragment loading (default is 8000ms)
            manifestLoadingTimeOut: 15000,  // 15 seconds timeout for manifest loading (default is 10000ms)
            levelLoadingTimeOut: 15000      // 15 seconds timeout for level loading (default is 10000ms)
        });

        hls.loadSource(hlsStreamUrl);
        hls.attachMedia(videoElement);

        hls.on(Hls.Events.MANIFEST_PARSED, function() {
            if (loadingIndicator) {
                loadingIndicator.style.display = 'none';
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
                handleVideoError(stream.name);
            }
        });

        // Set up more frequent refresh to prevent stale data
        const refreshInterval = 30000; // 30 seconds
        const refreshTimer = setInterval(() => {
            if (videoCell && videoCell.hlsPlayer) {
                console.log(`Refreshing HLS stream for ${stream.name}`);
                const newTimestamp = Date.now();
                const newUrl = `/api/streaming/${encodeURIComponent(stream.name)}/hls/index.m3u8?_t=${newTimestamp}`;
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
 * Toggle fullscreen for a specific stream
 */
function toggleStreamFullscreen(streamName) {
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;

    if (!videoCell) {
        console.error('Stream not found:', streamName);
        return;
    }

    if (!document.fullscreenElement) {
        videoCell.requestFullscreen().catch(err => {
            console.error(`Error attempting to enable fullscreen: ${err.message}`);
            alert(`Could not enable fullscreen mode: ${err.message}`);
        });
    } else {
        document.exitFullscreen();
    }
}

/**
 * Toggle fullscreen mode for the entire video grid
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
        alert('Cannot take snapshot: Video not loaded or has invalid dimensions');
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
        alert('Failed to create snapshot: ' + error.message);
    }
}

/**
 * Play recording - Ultra-simplified direct implementation
 */
function playRecording(recordingId) {
    const videoModal = document.getElementById('video-modal');
    const videoPlayer = document.getElementById('video-player');
    const videoTitle = document.getElementById('video-modal-title');
    const videoCloseBtn = document.getElementById('video-close-btn');
    const modalCloseBtn = videoModal?.querySelector('.close');

    if (!videoModal || !videoPlayer || !videoTitle) {
        console.error('Video modal elements not found');
        return;
    }

    // Set up close button event handlers
    if (videoCloseBtn) {
        videoCloseBtn.onclick = function() {
            closeVideoModal();
        };
    }
    
    if (modalCloseBtn) {
        modalCloseBtn.onclick = function() {
            closeVideoModal();
        };
    }

    // Show loading state
    videoModal.classList.add('loading');
    videoModal.style.display = 'block';

    // Fetch recording details
    fetch(`/api/recordings/${recordingId}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load recording details');
            }
            return response.json();
        })
        .then(recording => {
            console.log('Recording details:', recording);

            // Set video title
            videoTitle.textContent = `${recording.stream} - ${recording.start_time}`;

            // Base URLs for video
            const videoUrl = `/api/recordings/download/${recordingId}`;
            const downloadUrl = `${videoUrl}?download=1`;

            console.log('Video URL:', videoUrl);
            console.log('Download URL:', downloadUrl);

            // Reset video player
            videoPlayer.innerHTML = '';
            videoPlayer.controls = true;

            // Determine if HLS or MP4
            if (recording.path.endsWith('.m3u8')) {
                if (Hls.isSupported()) {
                    let hls = new Hls();
                    hls.loadSource(videoUrl);
                    hls.attachMedia(videoPlayer);
                    hls.on(Hls.Events.MANIFEST_PARSED, function () {
                        videoPlayer.play();
                    });
                } else if (videoPlayer.canPlayType('application/vnd.apple.mpegurl')) {
                    videoPlayer.src = videoUrl;
                    videoPlayer.play();
                } else {
                    console.error('HLS not supported');
                    videoPlayer.innerHTML = `
                        <p>HLS playback is not supported in this browser.</p>
                        <a href="${downloadUrl}" class="btn btn-primary" download>Download Video</a>
                    `;
                }
            } else {
                // Standard MP4 playback
                videoPlayer.src = videoUrl;
                videoPlayer.play();
            }

            // Update download button
            const downloadBtn = document.getElementById('video-download-btn');
            if (downloadBtn) {
                downloadBtn.onclick = function(e) {
                    e.preventDefault();
                    const link = document.createElement('a');
                    link.href = downloadUrl;
                    link.download = '';
                    document.body.appendChild(link);
                    link.click();
                    document.body.removeChild(link);
                    return false;
                };
            }

            // Remove loading state
            videoModal.classList.remove('loading');
        })
        .catch(error => {
            console.error('Error loading recording:', error);
            videoModal.classList.remove('loading');
            videoPlayer.innerHTML = `
                <div style="height:70vh;display:flex;flex-direction:column;justify-content:center;align-items:center;background:#000;color:#fff;padding:20px;text-align:center;">
                    <p style="font-size:18px;margin-bottom:10px;">Error: ${error.message}</p>
                    <p style="margin-bottom:20px;">Cannot load the recording.</p>
                    <a href="/api/recordings/download/${recordingId}?download=1" class="btn btn-primary" download>Download Video</a>
                </div>
            `;
        });
}

/**
 * Start polling for detection results and draw bounding boxes
 */
function startDetectionPolling(streamName, canvasOverlay, videoElement) {
    // Store the polling interval ID on the canvas element for cleanup
    if (canvasOverlay.detectionInterval) {
        clearInterval(canvasOverlay.detectionInterval);
    }
    
    // Function to draw bounding boxes
    function drawDetectionBoxes(detections) {
        const canvas = canvasOverlay;
        const ctx = canvas.getContext('2d');
        
        // Set canvas dimensions to match the displayed video element
        canvas.width = videoElement.clientWidth;
        canvas.height = videoElement.clientHeight;
        
        // Clear previous drawings
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        
        // No detections, just return
        if (!detections || detections.length === 0) {
            return;
        }
        
        // Get the actual video dimensions
        const videoWidth = videoElement.videoWidth;
        const videoHeight = videoElement.videoHeight;
        
        // If video dimensions aren't available yet, skip drawing
        if (!videoWidth || !videoHeight) {
            console.log('Video dimensions not available yet, skipping detection drawing');
            return;
        }
        
        // Calculate the scaling and positioning to maintain aspect ratio
        const videoAspect = videoWidth / videoHeight;
        const canvasAspect = canvas.width / canvas.height;
        
        let drawWidth, drawHeight, offsetX = 0, offsetY = 0;
        
        if (videoAspect > canvasAspect) {
            // Video is wider than canvas (letterboxing - black bars on top and bottom)
            drawWidth = canvas.width;
            drawHeight = canvas.width / videoAspect;
            offsetY = (canvas.height - drawHeight) / 2;
        } else {
            // Video is taller than canvas (pillarboxing - black bars on sides)
            drawHeight = canvas.height;
            drawWidth = canvas.height * videoAspect;
            offsetX = (canvas.width - drawWidth) / 2;
        }
        
        // Log dimensions for debugging
        console.log(`Canvas: ${canvas.width}x${canvas.height}, Video: ${videoWidth}x${videoHeight}`);
        console.log(`Draw area: ${drawWidth}x${drawHeight} with offset (${offsetX}, ${offsetY})`);
        
        // Draw each detection
        detections.forEach(detection => {
            // Calculate pixel coordinates based on normalized values (0-1)
            // and adjust for the actual display area
            const x = (detection.x * drawWidth) + offsetX;
            const y = (detection.y * drawHeight) + offsetY;
            const width = detection.width * drawWidth;
            const height = detection.height * drawHeight;
            
            // Draw bounding box
            ctx.strokeStyle = 'rgba(255, 0, 0, 0.8)';
            ctx.lineWidth = 3;
            ctx.strokeRect(x, y, width, height);
            
            // Draw label background
            const label = `${detection.label} (${Math.round(detection.confidence * 100)}%)`;
            ctx.font = '14px Arial';
            const textWidth = ctx.measureText(label).width;
            ctx.fillStyle = 'rgba(255, 0, 0, 0.7)';
            ctx.fillRect(x, y - 20, textWidth + 10, 20);
            
            // Draw label text
            ctx.fillStyle = 'white';
            ctx.fillText(label, x + 5, y - 5);
        });
    }
    
    // Poll for detection results every 500ms
    canvasOverlay.detectionInterval = setInterval(() => {
        if (!videoElement.videoWidth) {
            // Video not loaded yet, skip this cycle
            return;
        }
        
        // Log the API call we're about to make
        console.log(`Fetching detection results for stream ${streamName}`);
        
        // Fetch detection results from API
        fetch(`/api/detection/results/${encodeURIComponent(streamName)}`)
            .then(response => {
                console.log(`Detection API response status for ${streamName}:`, response.status);
                if (!response.ok) {
                    throw new Error(`Failed to fetch detection results: ${response.status}`);
                }
                return response.json();
            })
            .then(data => {
                console.log(`Detection results for ${streamName}:`, data);
                // Draw bounding boxes if we have detections
                if (data && data.detections) {
                    console.log(`Drawing ${data.detections.length} detection boxes for ${streamName}`);
                    drawDetectionBoxes(data.detections);
                } else {
                    console.log(`No detections found for ${streamName}`);
                }
            })
            .catch(error => {
                console.error(`Error fetching detection results for ${streamName}:`, error);
                // Clear canvas on error
                const ctx = canvasOverlay.getContext('2d');
                ctx.clearRect(0, 0, canvasOverlay.width, canvasOverlay.height);
            });
    }, 500);
}

/**
 * Update video grid with streams
 */
function updateVideoGrid(streams) {
    const videoGrid = document.getElementById('video-grid');
    const streamSelector = document.getElementById('stream-selector');
    if (!videoGrid) return;

    // Clear existing content
    videoGrid.innerHTML = '';

    if (!streams || streams.length === 0) {
        const placeholder = document.createElement('div');
        placeholder.className = 'placeholder';
        placeholder.innerHTML = `
            <p>No streams configured</p>
            <a href="streams.html" class="btn">Configure Streams</a>
        `;
        videoGrid.appendChild(placeholder);
        return;
    }

    // Get layout
    const layout = document.getElementById('layout-selector').value;

    // Update stream selector dropdown
    if (streamSelector) {
        // Clear existing options
        streamSelector.innerHTML = '';
        
        // Add options for each stream
        streams.forEach(stream => {
            const option = document.createElement('option');
            option.value = stream.name;
            option.textContent = stream.name;
            streamSelector.appendChild(option);
        });
        
        // Remove existing event listeners by cloning and replacing the element
        const newStreamSelector = streamSelector.cloneNode(true);
        streamSelector.parentNode.replaceChild(newStreamSelector, streamSelector);
        
        // Get the new reference to the stream selector
        const updatedStreamSelector = document.getElementById('stream-selector');
        
        // Add change event listener
        updatedStreamSelector.addEventListener('change', function() {
            // Get the current layout
            const currentLayout = document.getElementById('layout-selector').value;
            
            if (currentLayout === '1') {
                // Show only the selected stream in single view mode
                const selectedStreamName = this.value;
                const videoCells = videoGrid.querySelectorAll('.video-cell');
                
                videoCells.forEach(cell => {
                    const streamName = cell.querySelector('.stream-info span').textContent;
                    cell.style.display = (streamName === selectedStreamName) ? 'flex' : 'none';
                });
            }
        });
    }

    // Add video elements for each stream
    streams.forEach(stream => {
        // Ensure we have an ID for the stream (use name as fallback if needed)
        const streamId = stream.id || stream.name;

        const videoCell = document.createElement('div');
        videoCell.className = 'video-cell';

        videoCell.innerHTML = `
            <video id="video-${stream.name.replace(/\s+/g, '-')}" autoplay muted></video>
            <div class="stream-info">
                <span>${stream.name}</span>
                <span>${stream.width}x${stream.height} · ${stream.fps}fps</span>
                <div class="stream-controls">
                    <button class="snapshot-btn" data-id="${streamId}" data-name="${stream.name}">
                        <span>📷</span> Snapshot
                    </button>
                    <button class="fullscreen-btn" data-id="${streamId}" data-name="${stream.name}">
                        <span>⛶</span> Fullscreen
                    </button>
                    <label class="toggle-switch" title="Enable/Disable Stream">
                        <input type="checkbox" class="stream-toggle" data-id="${streamId}" ${stream.streaming_enabled !== false ? 'checked' : ''}>
                        <span class="toggle-slider"></span>
                    </label>
                </div>
            </div>
            <div class="loading-indicator">
                <div class="loading-spinner"></div>
                <span>Connecting...</span>
            </div>
        `;

        videoGrid.appendChild(videoCell);
    });

    // Initialize video players and add event listeners
    streams.forEach(stream => {
        initializeVideoPlayer(stream);

        // Ensure we have an ID for the stream (use name as fallback if needed)
        const streamId = stream.id || stream.name;

        // Add event listener for snapshot button
        const snapshotBtn = videoGrid.querySelector(`.snapshot-btn[data-id="${streamId}"]`);
        if (snapshotBtn) {
            snapshotBtn.addEventListener('click', () => {
                console.log('Taking snapshot of stream with ID:', streamId);
                
                // Exit fullscreen if active before taking snapshot
                if (document.fullscreenElement) {
                    document.exitFullscreen().then(() => {
                        setTimeout(() => takeSnapshot(streamId), 100);
                    }).catch(err => {
                        console.error(`Error exiting fullscreen: ${err.message}`);
                        takeSnapshot(streamId);
                    });
                } else {
                    takeSnapshot(streamId);
                }
            });
        }

        // Add event listener for fullscreen button
        const fullscreenBtn = videoGrid.querySelector(`.fullscreen-btn[data-id="${streamId}"]`);
        if (fullscreenBtn) {
            fullscreenBtn.addEventListener('click', () => {
                console.log('Toggling fullscreen for stream with ID:', streamId);
                toggleStreamFullscreen(stream.name);
            });
        }
        
        // Add event listener for stream toggle button
        const streamToggle = videoGrid.querySelector(`.stream-toggle[data-id="${streamId}"]`);
        if (streamToggle) {
            streamToggle.addEventListener('change', function() {
                const enabled = this.checked;
                console.log(`Toggling stream ${streamId} to ${enabled ? 'enabled' : 'disabled'}`);
                
                // Send toggle request to API (only affects streaming_enabled flag, not the enabled flag)
                fetch(`/api/streams/${encodeURIComponent(streamId)}/toggle_streaming`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ enabled: enabled })
                })
                .then(response => {
                    if (!response.ok) {
                        throw new Error('Failed to toggle stream');
                    }
                    return response.json();
                })
                .then(data => {
                    console.log('Stream toggled successfully:', data);
                    
                    // If disabled, show a message in the video cell
                    const videoCell = this.closest('.video-cell');
                    if (videoCell) {
                        if (!enabled) {
                            // Clean up the video player
                            cleanupVideoPlayer(stream.name);
                            
                            // Show disabled message
                            let disabledMsg = videoCell.querySelector('.disabled-message');
                            if (!disabledMsg) {
                                disabledMsg = document.createElement('div');
                                disabledMsg.className = 'disabled-message';
                                disabledMsg.style.position = 'absolute';
                                disabledMsg.style.top = '50%';
                                disabledMsg.style.left = '50%';
                                disabledMsg.style.transform = 'translate(-50%, -50%)';
                                disabledMsg.style.color = 'white';
                                disabledMsg.style.backgroundColor = 'rgba(0, 0, 0, 0.7)';
                                disabledMsg.style.padding = '10px';
                                disabledMsg.style.borderRadius = '5px';
                                disabledMsg.style.textAlign = 'center';
                                videoCell.appendChild(disabledMsg);
                            }
                            disabledMsg.textContent = 'Stream disabled';
                        } else {
                            // Remove disabled message if it exists
                            const disabledMsg = videoCell.querySelector('.disabled-message');
                            if (disabledMsg) {
                                disabledMsg.remove();
                            }
                            
                            // Reinitialize the video player
                            initializeVideoPlayer(stream);
                        }
                    }
                })
                .catch(error => {
                    console.error('Error toggling stream:', error);
                    alert('Error toggling stream: ' + error.message);
                    
                    // Revert the toggle state in the UI
                    this.checked = !enabled;
                });
            });
        }
    });
    
    // Update video layout
    updateVideoLayout(layout);
}
