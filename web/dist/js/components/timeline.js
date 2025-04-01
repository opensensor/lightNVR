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
let speedControlsInitialized = false;

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
    
    // Format dates for API in UTC
    const startTime = startDate.toISOString().split('.')[0] + 'Z'; // Ensure UTC timezone is explicit
    const endTime = endDate.toISOString().split('.')[0] + 'Z'; // Ensure UTC timezone is explicit
    
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
 * Set playback speed
 * @param {number} speed - Playback speed (0.25, 0.5, 1.0, 1.5, 2.0, 4.0)
 */
function setPlaybackSpeed(speed) {
    if (!videoPlayer) return;
    
    // Set playback speed
    videoPlayer.playbackRate = speed;
    
    // Update current speed indicator
    const currentSpeedIndicator = document.getElementById('current-speed-indicator');
    if (currentSpeedIndicator) {
        currentSpeedIndicator.textContent = `Current Speed: ${speed}× ${speed === 1.0 ? '(Normal)' : ''}`;
    }
    
    // Update button styles
    document.querySelectorAll('.speed-btn').forEach(btn => {
        // Remove active class from all buttons
        btn.classList.remove('bg-green-500', 'text-white');
        btn.classList.add('bg-gray-200', 'hover:bg-gray-300');
        
        // Add active class to the selected button
        if (parseFloat(btn.getAttribute('data-speed')) === speed) {
            btn.classList.remove('bg-gray-200', 'hover:bg-gray-300');
            btn.classList.add('bg-green-500', 'text-white');
        }
    });
    
    console.log(`Playback speed set to ${speed}x`);
    showStatusMessage(`Playback speed: ${speed}x`, 'info');
}

/**
 * Initialize or update speed controls
 */
function initSpeedControls() {
    // Check if speed controls already exist
    if (document.getElementById('timeline-speed-controls')) {
        return; // Speed controls already initialized
    }
    
    // Create speed controls container
    const speedControlsContainer = document.createElement('div');
    speedControlsContainer.id = 'timeline-speed-controls';
    speedControlsContainer.className = 'mt-6 mb-8 p-4 border-2 border-green-500 rounded-lg bg-white dark:bg-gray-800 shadow-md';
    
    // Create heading
    const heading = document.createElement('h3');
    heading.className = 'text-lg font-bold text-center mb-4 text-gray-800 dark:text-white';
    heading.textContent = 'PLAYBACK SPEED CONTROLS';
    speedControlsContainer.appendChild(heading);
    
    // Create speed buttons container
    const speedButtonsContainer = document.createElement('div');
    speedButtonsContainer.className = 'flex flex-wrap justify-center gap-2';
    
    // Create speed buttons
    const speeds = [0.25, 0.5, 1.0, 1.5, 2.0, 4.0];
    speeds.forEach(speed => {
        const button = document.createElement('button');
        button.textContent = speed === 1.0 ? '1× (Normal)' : `${speed}×`;
        button.className = speed === 1.0 
            ? 'speed-btn px-4 py-2 rounded-full bg-green-500 text-white font-bold transition-all transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50'
            : 'speed-btn px-4 py-2 rounded-full bg-gray-200 hover:bg-gray-300 font-bold transition-all transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50';
        button.setAttribute('data-speed', speed);
        
        // Add click event
        button.addEventListener('click', () => {
            setPlaybackSpeed(speed);
        });
        
        speedButtonsContainer.appendChild(button);
    });
    
    // Add buttons container to speed controls
    speedControlsContainer.appendChild(speedButtonsContainer);
    
    // Add current speed indicator
    const currentSpeedIndicator = document.createElement('div');
    currentSpeedIndicator.id = 'current-speed-indicator';
    currentSpeedIndicator.className = 'mt-4 text-center font-bold text-green-600 dark:text-green-400';
    currentSpeedIndicator.textContent = 'Current Speed: 1× (Normal)';
    speedControlsContainer.appendChild(currentSpeedIndicator);
    
    // Add speed controls to the page
    const playerContainer = document.getElementById('video-player');
    if (playerContainer) {
        // Insert after the player container
        playerContainer.parentNode.insertBefore(speedControlsContainer, playerContainer.nextSibling);
        speedControlsInitialized = true;
    }
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
    
    // Create a map to track hours with recordings
    const hourMap = new Map();
    
    // First pass: collect all segments by hour
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
        
        // Mark each hour that this segment spans
        const startFloorHour = Math.floor(startHour);
        const endCeilHour = Math.min(Math.ceil(endHour), 24);
        
        for (let h = startFloorHour; h < endCeilHour; h++) {
            if (h >= timelineStartHour && h <= timelineEndHour) {
                if (!hourMap.has(h)) {
                    hourMap.set(h, []);
                }
                hourMap.get(h).push(index);
            }
        }
    });
    
    // Second pass: add visible segments
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
        segmentElement.style.zIndex = '10'; // Ensure segments are above the background
        segmentElement.title = `${segment.start_time} - ${segment.end_time} (${segment.duration}s)`;
        segmentElement.dataset.index = index;
        
        // Add click event to play segment
        segmentElement.addEventListener('click', () => {
            playSegment(index);
        });
        
        segmentsContainer.appendChild(segmentElement);
    });
    
    // Third pass: fill in gaps with clickable areas that find the nearest segment
    for (let hour = Math.floor(timelineStartHour); hour < Math.ceil(timelineEndHour); hour++) {
        if (!hourMap.has(hour)) {
            // No segments in this hour, find nearest segment
            let nearestSegmentIndex = -1;
            let minDistance = Infinity;
            
            for (let i = 0; i < timelineSegments.length; i++) {
                const segment = timelineSegments[i];
                const segmentMidpoint = (segment.start_timestamp + segment.end_timestamp) / 2;
                const hourMidpoint = new Date(selectedDate);
                hourMidpoint.setHours(hour, 30, 0, 0);
                const hourTimestamp = hourMidpoint.getTime() / 1000;
                
                const distance = Math.abs(segmentMidpoint - hourTimestamp);
                if (distance < minDistance) {
                    minDistance = distance;
                    nearestSegmentIndex = i;
                }
            }
            
            if (nearestSegmentIndex >= 0) {
                // Create a clickable area for this hour
                const position = ((hour - timelineStartHour) / (timelineEndHour - timelineStartHour)) * 100;
                const width = 100 / (timelineEndHour - timelineStartHour);
                
                const clickableArea = document.createElement('div');
                clickableArea.className = 'timeline-clickable-area';
                clickableArea.style.position = 'absolute';
                clickableArea.style.left = `${position}%`;
                clickableArea.style.width = `${width}%`;
                clickableArea.style.height = '100%';
                clickableArea.style.zIndex = '5'; // Below segments but above background
                clickableArea.dataset.hour = hour;
                
                // Add click event to play nearest segment
                clickableArea.addEventListener('click', (e) => {
                    // Only handle if not clicking on a segment
                    if (!e.target.classList.contains('timeline-segment')) {
                        const hourDate = new Date(selectedDate);
                        hourDate.setHours(hour, 30, 0, 0); // Middle of the hour
                        const timestamp = hourDate.getTime() / 1000;
                        handleTimelineClick(e);
                    }
                });
                
                segmentsContainer.appendChild(clickableArea);
            }
        }
    }
    
    // Add click event to timeline container for seeking
    container.addEventListener('click', handleTimelineClick);
    
    // Add drag functionality for the timeline cursor
    container.addEventListener('mousedown', startDrag);
    document.addEventListener('mouseup', stopDrag);
    
    // Make the timeline draggable
    let isDragging = false;
    let lastX = 0;
    
    function startDrag(e) {
        isDragging = true;
        lastX = e.clientX;
        document.addEventListener('mousemove', drag);
    }
    
    function drag(e) {
        if (!isDragging) return;
        
        // Calculate time based on cursor position
        const container = document.getElementById('timeline-container');
        const rect = container.getBoundingClientRect();
        const clickX = e.clientX - rect.left;
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
        
        // Update cursor position
        currentTime = clickTimestamp;
        updateCursorPosition();
        updateTimeDisplay();
        
        // If video is playing, seek to the new position
        if (isPlaying && currentSegmentIndex >= 0) {
            const segment = timelineSegments[currentSegmentIndex];
            if (clickTimestamp >= segment.start_timestamp && clickTimestamp <= segment.end_timestamp) {
                // Within current segment, just seek
                const seekTime = clickTimestamp - segment.start_timestamp;
                videoPlayer.currentTime = seekTime;
            } else {
                // Find and play the appropriate segment
                for (let i = 0; i < timelineSegments.length; i++) {
                    const segment = timelineSegments[i];
                    if (clickTimestamp >= segment.start_timestamp && clickTimestamp <= segment.end_timestamp) {
                        playSegment(i, clickTimestamp);
                        break;
                    }
                }
            }
        }
        
        lastX = e.clientX;
    }
    
    function stopDrag() {
        isDragging = false;
        document.removeEventListener('mousemove', drag);
    }
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
    
    // Initialize video player if not already done
    initVideoPlayer();
    
    // Initialize speed controls if not already done
    initSpeedControls();
    
    // Use direct MP4 playback instead of HLS
    const recordingUrl = `/api/recordings/play/${segment.id}`;
    
    // Clean up any existing HLS player
    if (hlsPlayer) {
        hlsPlayer.destroy();
        hlsPlayer = null;
    }
    
    // Set the video source directly to the MP4 file
    videoPlayer.src = recordingUrl;
    videoPlayer.currentTime = seekTime;
    
    // Play the video
    videoPlayer.play().catch(error => {
        console.error('Error playing video:', error);
        showStatusMessage('Error playing video: ' + error.message, 'error');
    });
    
    isPlaying = true;
    updatePlayButton();
    
    // Start playback tracking
    startPlaybackTracking();
    
    console.log(`Playing segment ${index} (ID: ${segment.id}) from ${seekTime}s`);
}

/**
 * Start tracking playback progress
 */
function startPlaybackTracking() {
    // Clear any existing interval
    if (playbackInterval) {
        clearInterval(playbackInterval);
    }
    
    // Update more frequently (every 100ms) for smoother cursor movement
    playbackInterval = setInterval(() => {
        if (!isPlaying || currentSegmentIndex < 0 || !videoPlayer) {
            return;
        }
        
        const segment = timelineSegments[currentSegmentIndex];
        if (!segment) {
            console.error('Invalid segment at index', currentSegmentIndex);
            return;
        }
        
        // Calculate current timestamp based on video currentTime
        currentTime = segment.start_timestamp + videoPlayer.currentTime;
        
        // Update time display
        updateTimeDisplay();
        
        // Update cursor position
        updateCursorPosition();
        
        // Check if we've reached the end of the segment
        if (videoPlayer.currentTime >= segment.duration) {
            console.log('End of segment reached, trying to play next segment');
            // Try to play the next segment
            if (currentSegmentIndex < timelineSegments.length - 1) {
                playSegment(currentSegmentIndex + 1);
            } else {
                // End of all segments
                pausePlayback();
            }
        }
    }, 100);
    
    // Add timeupdate event listener for more accurate tracking
    videoPlayer.addEventListener('timeupdate', function() {
        if (currentSegmentIndex < 0 || !videoPlayer) {
            return;
        }
        
        const segment = timelineSegments[currentSegmentIndex];
        if (!segment) return;
        
        // Calculate current timestamp based on video currentTime
        currentTime = segment.start_timestamp + videoPlayer.currentTime;
        
        // Update time display
        updateTimeDisplay();
        
        // Update cursor position
        updateCursorPosition();
    });
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
    if (!playButton) return;
    
    const icon = playButton.querySelector('.icon');
    if (!icon) return;
    
    if (isPlaying) {
        icon.textContent = '⏸️';
        playButton.title = 'Pause';
    } else {
        icon.textContent = '▶️';
        playButton.title = 'Play';
    }
}

/**
 * Initialize video player
 */
function initVideoPlayer() {
    const playerContainer = document.getElementById('video-player');
    if (!playerContainer) return;
    
    // Clear any existing content
    playerContainer.innerHTML = '';
    
    // Create video container with Tailwind classes
    const videoContainer = document.createElement('div');
    videoContainer.className = 'relative w-full bg-black rounded-lg overflow-hidden shadow-lg';
    
    // Create video element
    videoPlayer = document.createElement('video');
    videoPlayer.className = 'w-full h-auto';
    videoPlayer.controls = true; // Enable native controls
    videoPlayer.autoplay = false;
    videoPlayer.muted = false;
    videoPlayer.playsInline = true;
    
    // Add video to container
    videoContainer.appendChild(videoPlayer);
    
    // Add container to player
    playerContainer.appendChild(videoContainer);
    
    // Add event listeners
    videoPlayer.addEventListener('error', function(e) {
        console.error('Video error:', e);
        showStatusMessage('Video playback error', 'error');
    });
    
    videoPlayer.addEventListener('ended', function() {
        console.log('Video ended event triggered');
        // Try to play the next segment
        if (currentSegmentIndex < timelineSegments.length - 1) {
            playSegment(currentSegmentIndex + 1);
        } else {
            // End of all segments
            pausePlayback();
        }
    });
    
    // Initialize speed controls
    initSpeedControls();
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
    if (!container) return;
    
    // Create message element with Tailwind classes
    const messageElement = document.createElement('div');
    messageElement.className = 'status-message rounded-lg px-4 py-3 shadow-md transition-all duration-300 opacity-0 transform translate-y-2';
    messageElement.textContent = message;
    
    // Add type-specific classes
    switch (type) {
        case 'success':
            messageElement.classList.add('bg-green-500', 'text-white');
            break;
        case 'warning':
            messageElement.classList.add('bg-yellow-500', 'text-white');
            break;
        case 'error':
            messageElement.classList.add('bg-red-500', 'text-white');
            break;
        default:
            messageElement.classList.add('bg-blue-500', 'text-white');
    }
    
    // Add to container
    container.appendChild(messageElement);
    
    // Animate in
    setTimeout(() => {
        messageElement.classList.remove('opacity-0', 'translate-y-2');
        messageElement.classList.add('opacity-100', 'translate-y-0');
    }, 10);
    
    // Remove after delay
    setTimeout(() => {
        messageElement.classList.add('opacity-0', 'translate-y-2');
        setTimeout(() => {
            if (container.contains(messageElement)) {
                container.removeChild(messageElement);
            }
        }, 300);
    }, 3000);
}
