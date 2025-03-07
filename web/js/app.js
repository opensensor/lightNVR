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
 * Load system information
 */
function loadSystemInfo() {
    // In a real implementation, this would make an API call to get system info
    // For now, we'll just simulate it with placeholder data
    
    // Update system information
    document.getElementById('system-version').textContent = '0.1.0';
    document.getElementById('system-uptime').textContent = '1d 2h 34m';
    document.getElementById('system-cpu').textContent = '15%';
    document.getElementById('system-memory').textContent = '128 MB / 256 MB';
    document.getElementById('system-storage').textContent = '2.5 GB / 32 GB';
    
    // Update stream statistics
    document.getElementById('system-active-streams').textContent = '2 / 16';
    document.getElementById('system-recording-streams').textContent = '2';
    document.getElementById('system-received').textContent = '1.2 GB';
    document.getElementById('system-recorded').textContent = '850 MB';
    
    // Update logs
    document.getElementById('system-logs').textContent = 
        '[2025-03-06 22:30:15] [INFO] LightNVR started\n' +
        '[2025-03-06 22:30:16] [INFO] Loaded configuration from /etc/lightnvr/lightnvr.conf\n' +
        '[2025-03-06 22:30:17] [INFO] Initialized database\n' +
        '[2025-03-06 22:30:18] [INFO] Initialized storage manager\n' +
        '[2025-03-06 22:30:19] [INFO] Initialized stream manager\n' +
        '[2025-03-06 22:30:20] [INFO] Initialized web server on port 8080\n' +
        '[2025-03-06 22:30:21] [INFO] Added stream: Front Door\n' +
        '[2025-03-06 22:30:22] [INFO] Added stream: Back Yard\n' +
        '[2025-03-06 22:30:23] [INFO] Started recording: Front Door\n' +
        '[2025-03-06 22:30:24] [INFO] Started recording: Back Yard';
}

/**
 * Load streams
 */
function loadStreams() {
    // In a real implementation, this would make an API call to get streams
    // For now, we'll just simulate it with placeholder data
    
    const streamsTable = document.getElementById('streams-table');
    if (!streamsTable) return;
    
    const tbody = streamsTable.querySelector('tbody');
    tbody.innerHTML = '';
    
    // Add sample streams
    const streams = [
        {
            name: 'Front Door',
            url: 'rtsp://192.168.1.100:554/stream1',
            status: 'Running',
            resolution: '1920x1080',
            fps: 15,
            recording: true
        },
        {
            name: 'Back Yard',
            url: 'rtsp://192.168.1.101:554/stream1',
            status: 'Running',
            resolution: '1280x720',
            fps: 10,
            recording: true
        }
    ];
    
    if (streams.length === 0) {
        tbody.innerHTML = '<tr><td colspan="7" class="empty-message">No streams configured</td></tr>';
        return;
    }
    
    streams.forEach(stream => {
        const tr = document.createElement('tr');
        
        tr.innerHTML = `
            <td>${stream.name}</td>
            <td>${stream.url}</td>
            <td>${stream.status}</td>
            <td>${stream.resolution}</td>
            <td>${stream.fps}</td>
            <td>${stream.recording ? 'Yes' : 'No'}</td>
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
    const streamFilter = document.getElementById('stream-filter');
    if (streamFilter) {
        // Clear existing options except "All Streams"
        while (streamFilter.options.length > 1) {
            streamFilter.remove(1);
        }
        
        // Add stream options
        streams.forEach(stream => {
            const option = document.createElement('option');
            option.value = stream.name;
            option.textContent = stream.name;
            streamFilter.appendChild(option);
        });
    }
    
    // Update video grid
    updateVideoGrid(streams);
}

/**
 * Update video grid with streams
 */
function updateVideoGrid(streams) {
    const videoGrid = document.getElementById('video-grid');
    if (!videoGrid) return;
    
    // Clear existing content
    videoGrid.innerHTML = '';
    
    if (streams.length === 0) {
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
            <div class="video-placeholder">
                <p>Stream: ${stream.name}</p>
                <p>Resolution: ${stream.resolution}</p>
                <p>FPS: ${stream.fps}</p>
                <p>Status: ${stream.status}</p>
            </div>
        `;
        
        videoGrid.appendChild(videoContainer);
    });
    
    // In a real implementation, we would initialize video players here
}

/**
 * Update video layout
 */
function updateVideoLayout(layout) {
    // This would update the video grid layout
    // For now, we'll just reload the streams to update the grid
    loadStreams();
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
    // In a real implementation, this would make an API call to get recordings
    // For now, we'll just simulate it with placeholder data
    
    const recordingsTable = document.getElementById('recordings-table');
    if (!recordingsTable) return;
    
    const tbody = recordingsTable.querySelector('tbody');
    tbody.innerHTML = '';
    
    // Add sample recordings
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
    
    if (recordings.length === 0) {
        tbody.innerHTML = '<tr><td colspan="5" class="empty-message">No recordings found</td></tr>';
        return;
    }
    
    recordings.forEach(recording => {
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
}

/**
 * Load settings
 */
function loadSettings() {
    // In a real implementation, this would make an API call to get settings
    // For now, we'll just use the default values in the HTML
}

/**
 * Save settings
 */
function saveSettings() {
    // In a real implementation, this would make an API call to save settings
    // For now, we'll just show a success message
    
    alert('Settings saved successfully');
}

/**
 * Edit stream
 */
function editStream(streamName) {
    // In a real implementation, this would load the stream data
    // For now, we'll just show the modal with some placeholder data
    
    // Find the stream in our sample data
    const streams = [
        {
            name: 'Front Door',
            url: 'rtsp://192.168.1.100:554/stream1',
            enabled: true,
            width: 1920,
            height: 1080,
            fps: 15,
            codec: 'h264',
            priority: 10,
            record: true,
            segment_duration: 900
        },
        {
            name: 'Back Yard',
            url: 'rtsp://192.168.1.101:554/stream1',
            enabled: true,
            width: 1280,
            height: 720,
            fps: 10,
            codec: 'h264',
            priority: 5,
            record: true,
            segment_duration: 900
        }
    ];
    
    const stream = streams.find(s => s.name === streamName);
    if (!stream) return;
    
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
    
    // Show the modal
    document.getElementById('stream-modal').style.display = 'block';
}

/**
 * Save stream
 */
function saveStream() {
    // In a real implementation, this would make an API call to save the stream
    // For now, we'll just show a success message and reload the streams
    
    alert('Stream saved successfully');
    document.getElementById('stream-modal').style.display = 'none';
    loadStreams();
}

/**
 * Delete stream
 */
function deleteStream(streamName) {
    // In a real implementation, this would make an API call to delete the stream
    // For now, we'll just show a success message and reload the streams
    
    alert(`Stream "${streamName}" deleted successfully`);
    loadStreams();
}

/**
 * Test stream connection
 */
function testStream() {
    // In a real implementation, this would make an API call to test the stream
    // For now, we'll just show a success message
    
    const url = document.getElementById('stream-url').value;
    if (!url) {
        alert('Please enter a stream URL');
        return;
    }
    
    alert(`Stream connection test successful for URL: ${url}`);
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
    // For now, we'll just show a message
    
    alert(`Download started for recording ID: ${recordingId}`);
}

/**
 * Delete recording
 */
function deleteRecording(recordingId) {
    // In a real implementation, this would make an API call to delete the recording
    // For now, we'll just show a success message and reload the recordings
    
    alert(`Recording ID: ${recordingId} deleted successfully`);
    loadRecordings();
}

/**
 * Restart service
 */
function restartService() {
    // In a real implementation, this would make an API call to restart the service
    // For now, we'll just show a success message
    
    alert('Service restart initiated');
}

/**
 * Shutdown service
 */
function shutdownService() {
    // In a real implementation, this would make an API call to shutdown the service
    // For now, we'll just show a success message
    
    alert('Service shutdown initiated');
}

/**
 * Clear logs
 */
function clearLogs() {
    // In a real implementation, this would make an API call to clear the logs
    // For now, we'll just clear the logs display
    
    document.getElementById('system-logs').textContent = 'Logs cleared';
    alert('Logs cleared successfully');
}

/**
 * Backup configuration
 */
function backupConfig() {
    // In a real implementation, this would make an API call to backup the configuration
    // For now, we'll just show a success message
    
    alert('Configuration backup created successfully');
}

/**
 * Logout
 */
function logout() {
    // In a real implementation, this would make an API call to logout
    // For now, we'll just show a message
    
    alert('Logout successful');
}
