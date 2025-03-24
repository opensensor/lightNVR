/**
 * LightNVR Timeline Playback Component
 * Handles timeline-based playback of recordings
 */

// Global variables
let streams = [];
let timelineSegments = [];
let selectedStream = null;
let selectedDate = null;
let videoPlayer = null;
let hlsPlayer = null;
let isPlaying = false;
let currentSegmentIndex = -1;
let zoomLevel = 1; // 1 = 1 hour, 2 = 30 minutes, 4 = 15 minutes
let timelineStartHour = 0;
let timelineEndHour = 24;
let currentTime = null;
let timelineCursor = null;
let playbackInterval = null;

/**
 * Initialize the timeline page
 */
function initTimelinePage() {
    console.log('Initializing timeline page');
    
    // Initialize date picker with today's date
    const today = new Date();
    const dateInput = document.getElementById('timeline-date');
    if (dateInput) {
        dateInput.value = formatDateForInput(today);
        selectedDate = dateInput.value;
        
        // Load streams for dropdown
        loadStreams();
        
        // Set up event listeners
        document.getElementById('apply-button').addEventListener('click', loadTimelineData);
        document.getElementById('play-button').addEventListener('click', togglePlayback);
        document.getElementById('zoom-in-button').addEventListener('click', zoomIn);
        document.getElementById('zoom-out-button').addEventListener('click', zoomOut);
        
        // Initialize video player
        initVideoPlayer();
        
        // Show instructions initially
        showStatusMessage('Select a stream and date, then click Apply to load recordings', 'info');
    } else {
        console.error('Timeline elements not found. Make sure the DOM is fully loaded.');
    }
}

/**
 * Format date for input element
 * @param {Date} date - Date to format
 * @returns {string} Formatted date string (YYYY-MM-DD)
 */
function formatDateForInput(date) {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
}

/**
 * Load streams for dropdown
 */
function loadStreams() {
    fetch('/api/streams')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load streams');
            }
            return response.json();
        })
        .then(data => {
            streams = data || [];
            populateStreamSelector(streams);
        })
        .catch(error => {
            console.error('Error loading streams:', error);
            showStatusMessage('Error loading streams: ' + error.message, 'error');
        });
}

/**
 * Populate stream selector dropdown
 * @param {Array} streams - Array of stream objects
 */
function populateStreamSelector(streams) {
    const selector = document.getElementById('stream-selector');
    
    // Clear existing options except the first one
    while (selector.options.length > 1) {
        selector.remove(1);
    }
    
    // Add streams to selector
    streams.forEach(stream => {
        // Add all streams, not just enabled ones
        const option = document.createElement('option');
        option.value = stream.name;
        option.textContent = stream.name;
        selector.appendChild(option);
    });
    
    // Enable selector
    selector.disabled = false;
}

/**
 * Load timeline data based on selected stream and date
 */
function loadTimelineData() {
    // Get selected stream and date
    const streamSelector = document.getElementById('stream-selector');
    const dateInput = document.getElementById('timeline-date');
    
    if (streamSelector.value === '') {
        showStatusMessage('Please select a stream', 'error');
        return;
    }
    
    selectedStream = streamSelector.value;
    selectedDate = dateInput.value;
    
    // Show loading message
    showStatusMessage('Loading timeline data...', 'info');
    
    // Calculate start and end times (full day)
    const startDate = new Date(selectedDate);
    startDate.setHours(0, 0, 0, 0);
    
    const endDate = new Date(selectedDate);
    endDate.setHours(23, 59, 59, 999);
    
    // Format dates for API
    const startTime = startDate.toISOString();
    const endTime = endDate.toISOString();
    
    // Fetch timeline segments
    fetch(`/api/timeline/segments?stream=${encodeURIComponent(selectedStream)}&start=${encodeURIComponent(startTime)}&end=${encodeURIComponent(endTime)}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load timeline data');
            }
            return response.json();
        })
        .then(data => {
            timelineSegments = data.segments || [];
            
            if (timelineSegments.length === 0) {
                showStatusMessage('No recordings found for the selected date', 'warning');
                clearTimeline();
                return;
            }
            
            // Render timeline
            renderTimeline();
            
            // Show success message
            showStatusMessage(`Loaded ${timelineSegments.length} recording segments`, 'success');
        })
        .catch(error => {
            console.error('Error loading timeline data:', error);
            showStatusMessage('Error loading timeline data: ' + error.message, 'error');
        });
}

/**
 * Render timeline with segments
 */
function renderTimeline() {
    const container = document.getElementById('timeline-container');
    
    // Clear existing timeline
    container.innerHTML = '';
    
    // Create ruler
    const ruler = document.createElement('div');
    ruler.className = 'timeline-ruler';
    container.appendChild(ruler);
    
    // Create segments container
    const segmentsContainer = document.createElement('div');
    segmentsContainer.className = 'timeline-segments';
    container.appendChild(segmentsContainer);
    
    // Create cursor
    timelineCursor = document.createElement('div');
    timelineCursor.className = 'timeline-cursor';
    timelineCursor.style.display = 'none';
    container.appendChild(timelineCursor);
    
    // Calculate time range based on zoom level
    const hoursPerView = 24 / zoomLevel;
    timelineStartHour = Math.max(0, Math.min(24 - hoursPerView, timelineStartHour));
    timelineEndHour = Math.min(24, timelineStartHour + hoursPerView);
    
    // Add hour markers and labels
    const hourWidth = 100 / (timelineEndHour - timelineStartHour);
    
    for (let hour = Math.floor(timelineStartHour); hour <= Math.ceil(timelineEndHour); hour++) {
        if (hour >= 0 && hour <= 24) {
            const position = (hour - timelineStartHour) * hourWidth;
            
            // Add hour marker
            const tick = document.createElement('div');
            tick.className = 'timeline-tick major';
            tick.style.left = `${position}%`;
            ruler.appendChild(tick);
            
            // Add hour label
            const label = document.createElement('div');
            label.className = 'timeline-label';
            label.textContent = `${hour}:00`;
            label.style.left = `${position}%`;
            ruler.appendChild(label);
            
            // Add half-hour marker if not the last hour
            if (hour < 24) {
                const halfHourPosition = position + (hourWidth / 2);
                const halfHourTick = document.createElement('div');
                halfHourTick.className = 'timeline-tick';
                halfHourTick.style.left = `${halfHourPosition}%`;
                ruler.appendChild(halfHourTick);
            }
        }
    }
    
    // Add segments
    timelineSegments.forEach((segment, index) => {
        // Convert timestamps to Date objects
        const startTime = new Date(segment.start_timestamp * 1000);
        const endTime = new Date(segment.end_timestamp * 1000);
        
        // Calculate position and width
        const startHour = startTime.getHours() + (startTime.getMinutes() / 60) + (startTime.getSeconds() / 3600);
        const endHour = endTime.getHours() + (endTime.getMinutes() / 60) + (endTime.getSeconds() / 3600);
        
        // Skip segments outside the visible range
        if (endHour < timelineStartHour || startHour > timelineEndHour) {
            return;
        }
        
        // Adjust start and end to fit within visible range
        const visibleStartHour = Math.max(startHour, timelineStartHour);
        const visibleEndHour = Math.min(endHour, timelineEndHour);
        
        // Calculate position and width as percentages
        const startPercent = ((visibleStartHour - timelineStartHour) / (timelineEndHour - timelineStartHour)) * 100;
        const widthPercent = ((visibleEndHour - visibleStartHour) / (timelineEndHour - timelineStartHour)) * 100;
        
        // Create segment element
        const segmentElement = document.createElement('div');
        segmentElement.className = 'timeline-segment';
        if (segment.has_detection) {
            segmentElement.classList.add('has-detection');
        }
        segmentElement.style.left = `${startPercent}%`;
        segmentElement.style.width = `${widthPercent}%`;
        segmentElement.title = `${segment.start_time} - ${segment.end_time} (${segment.duration}s)`;
        segmentElement.dataset.index = index;
        
        // Add click event to play segment
        segmentElement.addEventListener('click', () => {
            playSegment(index);
        });
        
        segmentsContainer.appendChild(segmentElement);
    });
    
    // Add click event to timeline container for seeking
    container.addEventListener('click', handleTimelineClick);
}

/**
 * Handle click on timeline for seeking
 * @param {Event} event - Click event
 */
function handleTimelineClick(event) {
    // Ignore clicks on segments (they have their own handlers)
    if (event.target.classList.contains('timeline-segment')) {
        return;
    }
    
    const container = document.getElementById('timeline-container');
    const rect = container.getBoundingClientRect();
    const clickX = event.clientX - rect.left;
    const containerWidth = rect.width;
    
    // Calculate time based on click position
    const clickPercent = clickX / containerWidth;
    const clickHour = timelineStartHour + (clickPercent * (timelineEndHour - timelineStartHour));
    
    // Find the segment that contains this time
    const clickDate = new Date(selectedDate);
    clickDate.setHours(Math.floor(clickHour));
    clickDate.setMinutes(Math.floor((clickHour % 1) * 60));
    clickDate.setSeconds(Math.floor(((clickHour % 1) * 60) % 1 * 60));
    
    const clickTimestamp = clickDate.getTime() / 1000;
    
    // Find segment that contains this timestamp
    let foundSegment = false;
    for (let i = 0; i < timelineSegments.length; i++) {
        const segment = timelineSegments[i];
        if (clickTimestamp >= segment.start_timestamp && clickTimestamp <= segment.end_timestamp) {
            // Play this segment starting at the clicked time
            playSegment(i, clickTimestamp);
            foundSegment = true;
            break;
        }
    }
    
    if (!foundSegment) {
        // Find the closest segment
        let closestSegment = -1;
        let minDistance = Infinity;
        
        for (let i = 0; i < timelineSegments.length; i++) {
            const segment = timelineSegments[i];
            const startDistance = Math.abs(segment.start_timestamp - clickTimestamp);
            const endDistance = Math.abs(segment.end_timestamp - clickTimestamp);
            const distance = Math.min(startDistance, endDistance);
            
            if (distance < minDistance) {
                minDistance = distance;
                closestSegment = i;
            }
        }
        
        if (closestSegment >= 0) {
            // Play the closest segment
            playSegment(closestSegment);
        }
    }
}

/**
 * Play a specific segment
 * @param {number} index - Index of segment in timelineSegments array
 * @param {number} [startTime] - Optional timestamp to start playback from
 */
function playSegment(index, startTime = null) {
    if (index < 0 || index >= timelineSegments.length) {
        return;
    }
    
    const segment = timelineSegments[index];
    currentSegmentIndex = index;
    
    // Calculate start time within the segment
    let seekTime = 0;
    if (startTime !== null) {
        // Calculate seconds from the start of the segment
        seekTime = startTime - segment.start_timestamp;
    }
    
    // Update current time
    currentTime = startTime !== null ? startTime : segment.start_timestamp;
    
    // Update time display
    updateTimeDisplay();
    
    // Update cursor position
    updateCursorPosition();
    
    // Create manifest URL with start time
    const startDate = new Date(segment.start_timestamp * 1000);
    const manifestUrl = `/api/timeline/manifest?stream=${encodeURIComponent(selectedStream)}&start=${encodeURIComponent(startDate.toISOString())}`;
    
    // Load and play the video
    if (hlsPlayer) {
        hlsPlayer.loadSource(manifestUrl);
        hlsPlayer.on(Hls.Events.MANIFEST_PARSED, function() {
            videoPlayer.currentTime = seekTime;
            videoPlayer.play().catch(error => {
                console.error('Error playing video:', error);
                showStatusMessage('Error playing video: ' + error.message, 'error');
            });
            isPlaying = true;
            updatePlayButton();
            
            // Start playback tracking
            startPlaybackTracking();
        });
    } else {
        // Fallback for browsers with native HLS support
        videoPlayer.src = manifestUrl;
        videoPlayer.currentTime = seekTime;
        videoPlayer.play().catch(error => {
            console.error('Error playing video:', error);
            showStatusMessage('Error playing video: ' + error.message, 'error');
        });
        isPlaying = true;
        updatePlayButton();
        
        // Start playback tracking
        startPlaybackTracking();
    }
}

/**
 * Start tracking playback progress
 */
function startPlaybackTracking() {
    // Clear any existing interval
    if (playbackInterval) {
        clearInterval(playbackInterval);
    }
    
    // Update every 500ms
    playbackInterval = setInterval(() => {
        if (!isPlaying || currentSegmentIndex < 0) {
            return;
        }
        
        const segment = timelineSegments[currentSegmentIndex];
        
        // Calculate current timestamp based on video currentTime
        currentTime = segment.start_timestamp + videoPlayer.currentTime;
        
        // Update time display
        updateTimeDisplay();
        
        // Update cursor position
        updateCursorPosition();
        
        // Check if we've reached the end of the segment
        if (videoPlayer.currentTime >= segment.duration) {
            // Try to play the next segment
            if (currentSegmentIndex < timelineSegments.length - 1) {
                playSegment(currentSegmentIndex + 1);
            } else {
                // End of all segments
                pausePlayback();
            }
        }
    }, 500);
}

/**
 * Update the time display
 */
function updateTimeDisplay() {
    if (currentTime === null) {
        return;
    }
    
    const timeDisplay = document.getElementById('time-display');
    const date = new Date(currentTime * 1000);
    
    // Format time as HH:MM:SS
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    const seconds = date.getSeconds().toString().padStart(2, '0');
    
    timeDisplay.textContent = `${hours}:${minutes}:${seconds}`;
}

/**
 * Update the cursor position on the timeline
 */
function updateCursorPosition() {
    if (currentTime === null || !timelineCursor) {
        return;
    }
    
    // Calculate cursor position
    const date = new Date(currentTime * 1000);
    const hour = date.getHours() + (date.getMinutes() / 60) + (date.getSeconds() / 3600);
    
    // Check if the current time is within the visible range
    if (hour < timelineStartHour || hour > timelineEndHour) {
        timelineCursor.style.display = 'none';
        return;
    }
    
    // Calculate position as percentage
    const position = ((hour - timelineStartHour) / (timelineEndHour - timelineStartHour)) * 100;
    
    // Update cursor position
    timelineCursor.style.left = `${position}%`;
    timelineCursor.style.display = 'block';
}

/**
 * Toggle playback (play/pause)
 */
function togglePlayback() {
    if (isPlaying) {
        pausePlayback();
    } else {
        resumePlayback();
    }
}

/**
 * Pause playback
 */
function pausePlayback() {
    if (videoPlayer) {
        videoPlayer.pause();
    }
    isPlaying = false;
    updatePlayButton();
}

/**
 * Resume playback
 */
function resumePlayback() {
    if (videoPlayer && currentSegmentIndex >= 0) {
        videoPlayer.play().catch(error => {
            console.error('Error playing video:', error);
            showStatusMessage('Error playing video: ' + error.message, 'error');
        });
        isPlaying = true;
        updatePlayButton();
    } else if (timelineSegments.length > 0) {
        // Start playing the first segment
        playSegment(0);
    }
}

/**
 * Update play/pause button appearance
 */
function updatePlayButton() {
    const playButton = document.getElementById('play-button');
    const icon = playButton.querySelector('.icon');
    
    if (isPlaying) {
        icon.textContent = '⏸️';
    } else {
        icon.textContent = '▶️';
    }
}

/**
 * Initialize video player
 */
function initVideoPlayer() {
    const playerContainer = document.getElementById('video-player');
    
    // Create video element
    videoPlayer = document.createElement('video');
    videoPlayer.controls = false;
    videoPlayer.autoplay = false;
    videoPlayer.muted = false;
    videoPlayer.playsInline = true;
    
    // Add to container
    playerContainer.appendChild(videoPlayer);
    
    // Initialize HLS.js if supported
    if (Hls.isSupported()) {
        // Configure HLS.js with optimized settings for MP4 playback
        hlsPlayer = new Hls({
            enableWorker: true,
            lowLatencyMode: false,
            backBufferLength: 90,
            maxBufferLength: 30,
            maxMaxBufferLength: 600,
            maxBufferSize: 60 * 1000 * 1000, // 60MB
            maxBufferHole: 0.5,
            highBufferWatchdogPeriod: 2,
            nudgeOffset: 0.2,
            nudgeMaxRetry: 5,
            maxFragLookUpTolerance: 0.25,
            liveSyncDurationCount: 3,
            liveMaxLatencyDurationCount: 10,
            enableCEA708Captions: false,
            stretchShortVideoTrack: false,
            maxAudioFramesDrift: 1,
            forceKeyFrameOnDiscontinuity: true,
            abrEwmaFastLive: 3.0,
            abrEwmaSlowLive: 9.0,
            abrEwmaFastVoD: 3.0,
            abrEwmaSlowVoD: 9.0,
            abrEwmaDefaultEstimate: 500000,
            abrBandWidthFactor: 0.95,
            abrBandWidthUpFactor: 0.7,
            fragLoadingTimeOut: 20000,
            fragLoadingMaxRetry: 6,
            fragLoadingRetryDelay: 1000,
            fragLoadingMaxRetryTimeout: 64000,
            startFragPrefetch: false,
            testBandwidth: true
        });
        
        hlsPlayer.attachMedia(videoPlayer);
        
        // Add detailed error handling
        hlsPlayer.on(Hls.Events.ERROR, function(event, data) {
            console.warn('HLS error:', data);
            
            if (data.fatal) {
                console.error('Fatal HLS error:', data);
                showStatusMessage('Video playback error: ' + data.details, 'error');
                
                switch(data.type) {
                    case Hls.ErrorTypes.NETWORK_ERROR:
                        // Try to recover network error
                        console.log('Attempting to recover from network error...');
                        hlsPlayer.startLoad();
                        break;
                    case Hls.ErrorTypes.MEDIA_ERROR:
                        // Try to recover media error
                        console.log('Attempting to recover from media error...');
                        hlsPlayer.recoverMediaError();
                        break;
                    default:
                        // Cannot recover
                        console.error('Cannot recover from error:', data.type);
                        break;
                }
            }
        });
        
        // Add more event listeners for debugging
        hlsPlayer.on(Hls.Events.MANIFEST_PARSED, function() {
            console.log('HLS manifest parsed successfully');
        });
        
        hlsPlayer.on(Hls.Events.LEVEL_LOADED, function() {
            console.log('HLS level loaded');
        });
        
        hlsPlayer.on(Hls.Events.FRAG_LOADED, function() {
            console.log('HLS fragment loaded');
        });
    } else if (videoPlayer.canPlayType('application/vnd.apple.mpegurl')) {
        // For browsers with native HLS support (Safari)
        console.log('Using native HLS support');
    } else {
        showStatusMessage('HLS playback is not supported in this browser', 'error');
    }
    
    // Add event listeners
    videoPlayer.addEventListener('error', function(e) {
        console.error('Video error:', e);
        showStatusMessage('Video playback error', 'error');
    });
    
    videoPlayer.addEventListener('ended', function() {
        // Try to play the next segment
        if (currentSegmentIndex < timelineSegments.length - 1) {
            playSegment(currentSegmentIndex + 1);
        } else {
            // End of all segments
            pausePlayback();
        }
    });
    
    // Add more video event listeners for debugging
    videoPlayer.addEventListener('canplay', function() {
        console.log('Video can play');
    });
    
    videoPlayer.addEventListener('waiting', function() {
        console.log('Video waiting for data');
    });
    
    videoPlayer.addEventListener('playing', function() {
        console.log('Video playing');
    });
}

/**
 * Zoom in on timeline
 */
function zoomIn() {
    if (zoomLevel < 8) {
        zoomLevel *= 2;
        renderTimeline();
        updateCursorPosition();
    }
}

/**
 * Zoom out on timeline
 */
function zoomOut() {
    if (zoomLevel > 1) {
        zoomLevel /= 2;
        renderTimeline();
        updateCursorPosition();
    }
}

/**
 * Clear timeline
 */
function clearTimeline() {
    const container = document.getElementById('timeline-container');
    container.innerHTML = '';
    
    // Reset video player
    if (videoPlayer) {
        videoPlayer.pause();
        videoPlayer.src = '';
    }
    
    // Reset state
    currentSegmentIndex = -1;
    currentTime = null;
    isPlaying = false;
    updatePlayButton();
    
    // Clear time display
    document.getElementById('time-display').textContent = '00:00:00';
    
    // Clear any tracking interval
    if (playbackInterval) {
        clearInterval(playbackInterval);
        playbackInterval = null;
    }
}

/**
 * Show status message
 * @param {string} message - Message to display
 * @param {string} type - Message type (info, success, warning, error)
 */
function showStatusMessage(message, type = 'info') {
    const container = document.getElementById('status-message-container');
    
    // Create message element
    const messageElement = document.createElement('div');
    messageElement.className = 'status-message';
    messageElement.textContent = message;
    
    // Add type-specific class
    switch (type) {
        case 'success':
            messageElement.style.backgroundColor = 'rgba(16, 185, 129, 0.9)'; // Green
            break;
        case 'warning':
            messageElement.style.backgroundColor = 'rgba(245, 158, 11, 0.9)'; // Yellow
            break;
        case 'error':
            messageElement.style.backgroundColor = 'rgba(239, 68, 68, 0.9)'; // Red
            break;
        default:
            messageElement.style.backgroundColor = 'rgba(59, 130, 246, 0.9)'; // Blue
    }
    
    // Add to container
    container.appendChild(messageElement);
    
    // Animate in
    setTimeout(() => {
        messageElement.classList.add('show');
    }, 10);
    
    // Remove after delay
    setTimeout(() => {
        messageElement.classList.remove('show');
        setTimeout(() => {
            container.removeChild(messageElement);
        }, 300);
    }, 3000);
}
