/**
 * LightNVR Web Interface JavaScript with URL-based Navigation
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
    // Initialize router for URL-based navigation
    initRouter();

    // Set up modals
    setupModals();

    // Set up snapshot modal
    setupSnapshotModal();

    // Add stream styles
    addStreamStyles();

    // Add status message styles
    addStatusMessageStyles();

    console.log('LightNVR Web Interface initialized');
}

/**
 * Initialize the router for URL-based navigation
 */
function initRouter() {
    // Define routes and corresponding handlers
    const router = {
        routes: {
            '/': loadLiveView,
            '/recordings': loadRecordingsView,
            '/streams': loadStreamsView,
            '/settings': loadSettingsView,
            '/system': loadSystemView,
            '/debug': loadDebugView,
            '/logout': handleLogout
        },

        init: function() {
            // Handle initial page load based on URL
            this.navigate(window.location.pathname || '/');

            // Handle navigation clicks
            document.querySelectorAll('nav a').forEach(link => {
                link.addEventListener('click', (e) => {
                    e.preventDefault();
                    const path = link.getAttribute('href');
                    this.navigate(path);
                });
            });

            // Handle browser back/forward buttons
            window.addEventListener('popstate', (e) => {
                this.navigate(window.location.pathname, false);
            });

            // Handle page links outside of navigation
            document.addEventListener('click', e => {
                const link = e.target.closest('a[href^="/"]');
                if (link && !link.hasAttribute('target') && !link.hasAttribute('download')) {
                    e.preventDefault();
                    this.navigate(link.getAttribute('href'));
                }
            });
        },

        navigate: function(path, addToHistory = true) {
            // Default to home if path is not recognized
            if (!this.routes[path]) {
                path = '/';
            }

            // Update URL in the address bar
            if (addToHistory) {
                history.pushState({path: path}, '', path);
            }

            // Update active navigation link
            document.querySelectorAll('nav a').forEach(a => {
                a.classList.remove('active');
            });

            const activeNav = document.querySelector(`nav a[href="${path}"]`);
            if (activeNav) {
                activeNav.classList.add('active');
            }

            // Load the requested page content
            this.routes[path]();
        }
    };

    // Initialize the router
    router.init();

    // Store router in window for global access
    window.appRouter = router;
}

/**
 * Load template into main content area
 */
function loadTemplate(templateId) {
    const mainContent = document.getElementById('main-content');
    const template = document.getElementById(templateId);

    if (template && mainContent) {
        mainContent.innerHTML = '';
        const clone = document.importNode(template.content, true);
        mainContent.appendChild(clone);
        return true;
    }
    return false;
}

/**
 * Page loader functions
 */
function loadLiveView() {
    if (loadTemplate('live-template')) {
        document.title = 'Live View - LightNVR';

        // Set up event handlers for Live View page
        setupLiveViewHandlers();

        // Load streams for video grid
        loadStreams(true);
    }
}

function loadRecordingsView() {
    if (loadTemplate('recordings-template')) {
        document.title = 'Recordings - LightNVR';

        // Set up event handlers for Recordings page
        setupRecordingsHandlers();

        // Load recordings data
        loadRecordings();
    }
}

function loadStreamsView() {
    if (loadTemplate('streams-template')) {
        document.title = 'Streams - LightNVR';

        // Set up event handlers for Streams page
        setupStreamsHandlers();

        // Load streams data
        loadStreams();
    }
}

function loadSettingsView() {
    if (loadTemplate('settings-template')) {
        document.title = 'Settings - LightNVR';

        // Set up event handlers for Settings page
        setupSettingsHandlers();

        // Load settings data
        loadSettings();
    }
}

function loadSystemView() {
    if (loadTemplate('system-template')) {
        document.title = 'System - LightNVR';

        // Set up event handlers for System page
        setupSystemHandlers();

        // Load system information
        loadSystemInfo();
    }
}

function loadDebugView() {
    if (loadTemplate('debug-template')) {
        document.title = 'Debug - LightNVR';

        // Set up event handlers for Debug page
        setupDebugHandlers();

        // Load debug information
        loadDebugInfo();
    }
}

function handleLogout() {
    if (confirm('Are you sure you want to logout?')) {
        // In a real implementation, this would make an API call to logout
        alert('Logout successful');
        // Redirect to login page (would be implemented in a real app)
        window.appRouter.navigate('/');
    } else {
        // If user cancels logout, go back to previous page
        window.history.back();
    }
}

/**
 * Page-specific event handlers
 */
function setupLiveViewHandlers() {
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
}

function setupRecordingsHandlers() {
    // Date picker
    const datePicker = document.getElementById('date-picker');
    if (datePicker) {
        // Set to today's date
        const today = new Date();
        const year = today.getFullYear();
        const month = String(today.getMonth() + 1).padStart(2, '0');
        const day = String(today.getDate()).padStart(2, '0');
        datePicker.value = `${year}-${month}-${day}`;
        
        datePicker.addEventListener('change', function() {
            console.log("Date changed to:", this.value);
            loadRecordings(1); // Reset to page 1 when filter changes
        });
    }

    // Stream filter
    const streamFilter = document.getElementById('stream-filter');
    if (streamFilter) {
        streamFilter.addEventListener('change', function() {
            loadRecordings(1); // Reset to page 1 when filter changes
        });
    }
    
    // Page size selector
    const pageSizeSelect = document.getElementById('page-size');
    if (pageSizeSelect) {
        pageSizeSelect.addEventListener('change', function() {
            loadRecordings(1); // Reset to page 1 when page size changes
        });
    }

    // Refresh button
    const refreshBtn = document.getElementById('refresh-btn');
    if (refreshBtn) {
        refreshBtn.addEventListener('click', function() {
            loadRecordings(1); // Reset to page 1 when refreshing
        });
    }
}

function setupStreamsHandlers() {
    // Add stream button
    const addStreamBtn = document.getElementById('add-stream-btn');
    if (addStreamBtn) {
        addStreamBtn.addEventListener('click', function() {
            const streamModal = document.getElementById('stream-modal');
            if (streamModal) {
                // Reset form
                document.getElementById('stream-form').reset();
                // Clear original name data attribute
                delete streamModal.dataset.originalName;
                // Show modal
                streamModal.style.display = 'block';
            }
        });
    }
}

function setupSettingsHandlers() {
    // Save settings button
    const saveSettingsBtn = document.getElementById('save-settings-btn');
    if (saveSettingsBtn) {
        saveSettingsBtn.addEventListener('click', function() {
            saveSettings();
        });
    }
}

function setupSystemHandlers() {
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
}

function setupDebugHandlers() {
    // Debug database button
    const debugDbBtn = document.getElementById('debug-db-btn');
    if (debugDbBtn) {
        debugDbBtn.addEventListener('click', function() {
            openDebugModal();
        });
    }
}

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

    // Debug modal
    const debugModal = document.getElementById('debug-modal');
    const debugCloseBtn = document.getElementById('debug-close-btn');
    const debugModalCloseBtn = debugModal?.querySelector('.close');
    const debugRefreshBtn = document.getElementById('debug-refresh-btn');

    // Close debug modal when close button is clicked
    if (debugCloseBtn) {
        debugCloseBtn.addEventListener('click', function() {
            debugModal.style.display = 'none';
        });
    }

    // Close debug modal when close button is clicked
    if (debugModalCloseBtn) {
        debugModalCloseBtn.addEventListener('click', function() {
            debugModal.style.display = 'none';
        });
    }

    // Refresh debug data when refresh button is clicked
    if (debugRefreshBtn) {
        debugRefreshBtn.addEventListener('click', function() {
            refreshDebugData();
        });
    }

    // Close modals when clicking outside
    window.addEventListener('click', function(e) {
        if (e.target === streamModal) {
            streamModal.style.display = 'none';
        } else if (e.target === videoModal) {
            closeVideoModal();
        } else if (e.target === debugModal) {
            debugModal.style.display = 'none';
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
 * Open debug modal and load data
 */
function openDebugModal() {
    const debugModal = document.getElementById('debug-modal');
    if (!debugModal) return;

    // Refresh debug data
    refreshDebugData();

    // Show modal
    debugModal.style.display = 'block';
}

/**
 * Refresh debug data
 */
function refreshDebugData() {
    const debugOutput = document.getElementById('debug-output');
    if (!debugOutput) return;

    debugOutput.textContent = 'Loading debug data...';

    // Simulate API call with delay
    setTimeout(() => {
        debugOutput.textContent = JSON.stringify({
            database: {
                version: '1.0.0',
                size: '1.2 MB',
                tables: ['streams', 'recordings', 'settings', 'logs'],
                records: {
                    streams: 2,
                    recordings: 24,
                    settings: 18,
                    logs: 156
                }
            }
        }, null, 2);
    }, 500);
}

/**
 * Load debug information
 */
function loadDebugInfo() {
    const debugInfo = document.getElementById('debug-info');
    if (!debugInfo) return;

    debugInfo.textContent = 'Loading debug information...';

    // Simulate API call with delay
    setTimeout(() => {
        debugInfo.textContent = `Environment: Development
Node.js Version: 18.15.0
Database: SQLite 3.36.0
FFmpeg Version: 4.4.2
OS: Linux 5.15.0-52-generic x86_64
Memory Usage: 78.4 MB
Uptime: 1d 4h 23m`;
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
 * Load streams - for Streams page or Live View page
 */
// Fix for streams page interactions
function loadStreams(forLiveView = false) {
    if (forLiveView) {
        // For Live View page, load streams for video grid
        const videoGrid = document.getElementById('video-grid');
        if (!videoGrid) return;

        showLoading(videoGrid);

        // Fetch streams from API
        fetch('/api/streams')
            .then(response => {
                if (!response.ok) {
                    throw new Error('Failed to load streams');
                }
                return response.json();
            })
            .then(streams => {
                hideLoading(videoGrid);
                updateVideoGrid(streams);
            })
            .catch(error => {
                console.error('Error loading streams for live view:', error);
                hideLoading(videoGrid);

                const placeholder = document.createElement('div');
                placeholder.className = 'placeholder';
                placeholder.innerHTML = `
                    <p>Error loading streams</p>
                    <a href="/streams" class="btn">Configure Streams</a>
                `;
                videoGrid.innerHTML = '';
                videoGrid.appendChild(placeholder);
            });
    } else {
        // For Streams page, load streams for table
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

                    // Ensure we have an ID for the stream (use name as fallback if needed)
                    const streamId = stream.id || stream.name;

                    tr.innerHTML = `
                        <td>${stream.name}</td>
                        <td>${stream.url}</td>
                        <td><span class="status-indicator ${stream.status}">${stream.status}</span></td>
                        <td>${stream.width}x${stream.height}</td>
                        <td>${stream.fps}</td>
                        <td>${stream.record ? 'Yes' : 'No'}</td>
                        <td>
                            <button class="btn-icon edit-btn" data-id="${streamId}" title="Edit"><span class="icon">✎</span></button>
                            <button class="btn-icon snapshot-btn" data-id="${streamId}" title="Snapshot"><span class="icon">📷</span></button>
                            <button class="btn-icon delete-btn" data-id="${streamId}" title="Delete"><span class="icon">×</span></button>
                        </td>
                    `;

                    tbody.appendChild(tr);
                });

                // Add event listeners for edit, snapshot, and delete buttons
                document.querySelectorAll('.edit-btn').forEach(btn => {
                    btn.addEventListener('click', function() {
                        const streamId = this.getAttribute('data-id');
                        console.log('Editing stream with ID:', streamId);
                        editStream(streamId);
                    });
                });

                document.querySelectorAll('.snapshot-btn').forEach(btn => {
                    btn.addEventListener('click', function() {
                        const streamId = this.getAttribute('data-id');
                        console.log('Taking snapshot of stream with ID:', streamId);
                        takeSnapshot(streamId);
                    });
                });

                document.querySelectorAll('.delete-btn').forEach(btn => {
                    btn.addEventListener('click', function() {
                        const streamId = this.getAttribute('data-id');
                        console.log('Deleting stream with ID:', streamId);
                        if (confirm(`Are you sure you want to delete this stream?`)) {
                            deleteStream(streamId);
                        }
                    });
                });

                // Update stream filter in recordings page
                updateStreamFilter(streams);
            })
            .catch(error => {
                console.error('Error loading streams:', error);
                tbody.innerHTML = '<tr><td colspan="7" class="empty-message">Error loading streams</td></tr>';
            })
            .finally(() => {
                hideLoading(streamsTable);
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
            <a href="/streams" class="btn">Configure Streams</a>
        `;
        videoGrid.appendChild(placeholder);
        return;
    }

    // Get layout
    const layout = document.getElementById('layout-selector').value;

    // Update video layout
    updateVideoLayout(layout);

    // Add video elements for each stream
    streams.forEach(stream => {
        // Ensure we have an ID for the stream (use name as fallback if needed)
        const streamId = stream.id || stream.name;

        const videoContainer = document.createElement('div');
        videoContainer.className = 'video-item';

        videoContainer.innerHTML = `
            <div class="video-header">
                <span>${stream.name}</span>
                <div class="video-controls">
                    <button class="btn-small snapshot" data-id="${streamId}" data-name="${stream.name}">Snapshot</button>
                    <button class="btn-small fullscreen" data-id="${streamId}" data-name="${stream.name}">Fullscreen</button>
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

        // Ensure we have an ID for the stream (use name as fallback if needed)
        const streamId = stream.id || stream.name;

        // Add event listener for snapshot button
        const snapshotBtn = videoGrid.querySelector(`.snapshot[data-id="${streamId}"]`);
        if (snapshotBtn) {
            snapshotBtn.addEventListener('click', () => {
                console.log('Taking snapshot of stream with ID:', streamId);
                takeSnapshot(streamId);
            });
        }

        // Add event listener for fullscreen button
        const fullscreenBtn = videoGrid.querySelector(`.fullscreen[data-id="${streamId}"]`);
        if (fullscreenBtn) {
            fullscreenBtn.addEventListener('click', () => {
                console.log('Toggling fullscreen for stream with ID:', streamId);
                toggleStreamFullscreen(stream.name);
            });
        }
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
            const streamId = stream.id || stream.name;
            const option = document.createElement('option');
            option.value = streamId;
            option.textContent = stream.name;
            streamFilter.appendChild(option);
        });
    }
}

/**
 * Take a snapshot of a stream
 */
function takeSnapshot(streamId) {
    console.log('Taking snapshot for stream ID:', streamId);

    // First get the stream name from ID
    fetch(`/api/streams/${encodeURIComponent(streamId)}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load stream details');
            }
            return response.json();
        })
        .then(stream => {
            // Now take the snapshot
            const videoElementId = `video-${stream.name.replace(/\s+/g, '-')}`;
            const videoElement = document.getElementById(videoElementId);
            const containerId = `player-${stream.name.replace(/\s+/g, '-')}`;
            const container = document.getElementById(containerId);

            if (!videoElement || !container) {
                // If we're on the streams page, not the live view page
                // Send a direct request to the API to take a snapshot
                showStatusMessage('Taking snapshot...');

                fetch(`/api/streams/${encodeURIComponent(streamId)}/snapshot`, {
                    method: 'POST'
                })
                    .then(response => {
                        if (!response.ok) {
                            throw new Error('Failed to take snapshot');
                        }
                        return response.blob();
                    })
                    .then(blob => {
                        const imageUrl = URL.createObjectURL(blob);
                        showSnapshotPreview(imageUrl, stream.name);
                    })
                    .catch(error => {
                        console.error('Error taking snapshot:', error);
                        showStatusMessage('Error taking snapshot: ' + error.message, 3000);
                    });

                return;
            }

            // Show status message
            showStatusMessage('Taking snapshot...');

            // Create a canvas to capture the image
            const canvas = document.createElement('canvas');

            // For real video, use the video dimensions
            if (videoElement.videoWidth && videoElement.videoHeight) {
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
                ctx.fillText(`${stream.name} - ${timestamp}`, 10, canvas.height - 10);

                // Show preview instead of direct download
                const imageData = canvas.toDataURL('image/jpeg', 0.9);
                showSnapshotPreview(imageData, stream.name);
            } else {
                showStatusMessage('Cannot take snapshot: video not loaded', 3000);
            }
        })
        .catch(error => {
            console.error('Error getting stream details:', error);
            showStatusMessage('Error taking snapshot: ' + error.message, 3000);
        });
}

/**
 * Update video layout
 */
function updateVideoLayout(layout) {
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
    else if (Hls && Hls.isSupported()) {
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
 * Load recordings with pagination
 */
function loadRecordings(page = 1) {
    const recordingsTable = document.getElementById('recordings-table');
    if (!recordingsTable) return;

    const tbody = recordingsTable.querySelector('tbody');

    showLoading(recordingsTable);

    // Clear existing rows
    tbody.innerHTML = '<tr><td colspan="5" class="empty-message">Loading recordings...</td></tr>';

    // Get filter values
    const dateFilter = document.getElementById('date-picker').value;
    const streamFilter = document.getElementById('stream-filter').value;
    const pageSizeSelect = document.getElementById('page-size');
    const pageSize = pageSizeSelect ? parseInt(pageSizeSelect.value, 10) : 20;

    // Build query string
    let queryParams = new URLSearchParams();
    
    if (dateFilter) {
        queryParams.append('date', dateFilter);
    }
    
    if (streamFilter && streamFilter !== 'all') {
        queryParams.append('stream', streamFilter);
    }
    
    // Add pagination parameters
    queryParams.append('page', page);
    queryParams.append('limit', pageSize);
    
    const queryString = queryParams.toString() ? `?${queryParams.toString()}` : '';

    // Fetch recordings from API
    fetch(`/api/recordings${queryString}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load recordings');
            }
            return response.json();
        })
        .then(data => {
            tbody.innerHTML = '';

            // Check if we have recordings
            if (!data.recordings || data.recordings.length === 0) {
                tbody.innerHTML = '<tr><td colspan="5" class="empty-message">No recordings found</td></tr>';
                
                // Update pagination info
                updatePaginationInfo(0, 0, 0, 1, 1);
                
                hideLoading(recordingsTable);
                return;
            }

            // Render recordings
            data.recordings.forEach(recording => {
                const tr = document.createElement('tr');

                tr.innerHTML = `
                    <td>${recording.stream}</td>
                    <td>${recording.start_time}</td>
                    <td>${recording.duration}</td>
                    <td>${recording.size}</td>
                    <td>
                        <button class="btn-icon play-btn" data-id="${recording.id}" title="Play"><span class="icon">▶</span></button>
                        <button class="btn-icon download-btn" data-id="${recording.id}" title="Download"><span class="icon">↓</span></button>
                        <button class="btn-icon delete-btn" data-id="${recording.id}" title="Delete"><span class="icon">×</span></button>
                    </td>
                `;

                tbody.appendChild(tr);
            });

            // Add event listeners for play, download, and delete buttons
            document.querySelectorAll('.play-btn').forEach(btn => {
                btn.addEventListener('click', function() {
                    const recordingId = this.getAttribute('data-id');
                    playRecording(recordingId);
                });
            });

            document.querySelectorAll('.download-btn').forEach(btn => {
                btn.addEventListener('click', function() {
                    const recordingId = this.getAttribute('data-id');
                    downloadRecording(recordingId);
                });
            });

            document.querySelectorAll('.delete-btn').forEach(btn => {
                btn.addEventListener('click', function() {
                    const recordingId = this.getAttribute('data-id');
                    if (confirm('Are you sure you want to delete this recording?')) {
                        deleteRecording(recordingId);
                    }
                });
            });

            // Update pagination info
            const { pagination } = data;
            const currentPage = pagination.page;
            const totalPages = pagination.pages;
            const totalRecords = pagination.total;
            const limit = pagination.limit;
            
            // Calculate the range of records being displayed
            const startRecord = (currentPage - 1) * limit + 1;
            const endRecord = Math.min(startRecord + data.recordings.length - 1, totalRecords);
            
            updatePaginationInfo(startRecord, endRecord, totalRecords, currentPage, totalPages);
            
            // Setup pagination buttons
            setupPaginationButtons(currentPage, totalPages);
        })
        .catch(error => {
            console.error('Error loading recordings:', error);
            tbody.innerHTML = '<tr><td colspan="5" class="empty-message">Error loading recordings</td></tr>';
            
            // Reset pagination info on error
            updatePaginationInfo(0, 0, 0, 1, 1);
        })
        .finally(() => {
            hideLoading(recordingsTable);
        });
}

/**
 * Update pagination information display
 */
function updatePaginationInfo(startRecord, endRecord, totalRecords, currentPage, totalPages) {
    const showingElement = document.getElementById('pagination-showing');
    const totalElement = document.getElementById('pagination-total');
    const currentElement = document.getElementById('pagination-current');
    
    if (showingElement) {
        showingElement.textContent = totalRecords > 0 ? `${startRecord}-${endRecord}` : '0-0';
    }
    
    if (totalElement) {
        totalElement.textContent = totalRecords;
    }
    
    if (currentElement) {
        currentElement.textContent = `Page ${currentPage} of ${totalPages}`;
    }
}

/**
 * Setup pagination button event handlers
 */
function setupPaginationButtons(currentPage, totalPages) {
    const firstBtn = document.getElementById('pagination-first');
    const prevBtn = document.getElementById('pagination-prev');
    const nextBtn = document.getElementById('pagination-next');
    const lastBtn = document.getElementById('pagination-last');
    
    // Disable first/prev buttons if on first page
    if (firstBtn) {
        firstBtn.disabled = currentPage <= 1;
        firstBtn.onclick = () => loadRecordings(1);
    }
    
    if (prevBtn) {
        prevBtn.disabled = currentPage <= 1;
        prevBtn.onclick = () => loadRecordings(currentPage - 1);
    }
    
    // Disable next/last buttons if on last page
    if (nextBtn) {
        nextBtn.disabled = currentPage >= totalPages;
        nextBtn.onclick = () => loadRecordings(currentPage + 1);
    }
    
    if (lastBtn) {
        lastBtn.disabled = currentPage >= totalPages;
        lastBtn.onclick = () => loadRecordings(totalPages);
    }
}

/**
 * Play recording
 */
function playRecording(recordingId) {
    const videoModal = document.getElementById('video-modal');
    const videoPlayer = document.getElementById('video-player');
    const videoTitle = document.getElementById('video-modal-title');

    if (!videoModal || !videoPlayer || !videoTitle) return;

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

            // Set video source
            videoPlayer.innerHTML = '';
            const videoElement = document.createElement('video');
            videoElement.controls = true;
            videoElement.autoplay = true;
            
            // Create a direct download URL
            const videoUrl = `/api/recordings/download/${recordingId}?direct=1`;
            console.log('Video URL:', videoUrl);
            
            // Set the source
            videoElement.src = videoUrl;
            
            // Add event listeners
            videoElement.addEventListener('loadeddata', () => {
                console.log('Video loaded successfully');
                videoModal.classList.remove('loading');
            });
            
            videoElement.addEventListener('canplay', () => {
                console.log('Video can play');
                videoModal.classList.remove('loading');
            });

            videoElement.addEventListener('error', (e) => {
                console.error('Video error:', e);
                videoModal.classList.remove('loading');
                videoPlayer.innerHTML = `
                    <div style="display:flex;justify-content:center;align-items:center;height:300px;background:#000;color:#fff;">
                        <p>Error loading video. The recording may be unavailable or in an unsupported format.</p>
                        <p>Error details: ${videoElement.error ? videoElement.error.message : 'Unknown error'}</p>
                    </div>
                `;
            });

            videoPlayer.appendChild(videoElement);

            // Set download button URL
            const downloadBtn = document.getElementById('video-download-btn');
            if (downloadBtn) {
                downloadBtn.onclick = () => {
                    window.location.href = `/api/recordings/download/${recordingId}`;
                };
            }
        })
        .catch(error => {
            console.error('Error loading recording:', error);
            videoModal.classList.remove('loading');
            videoPlayer.innerHTML = `
                <div style="display:flex;justify-content:center;align-items:center;height:300px;background:#000;color:#fff;">
                    <p>Error: ${error.message}</p>
                </div>
            `;
        });
}

/**
 * Download recording
 */
function downloadRecording(recordingId) {
    // Initiate download by redirecting to the download URL
    window.location.href = `/api/recordings/download/${recordingId}`;
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

    // Send delete request to API
    fetch(`/api/recordings/${recordingId}`, {
        method: 'DELETE'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to delete recording');
            }
            return response.json();
        })
        .then(data => {
            // Show success message
            alert('Recording deleted successfully');

            // Reload recordings
            loadRecordings();
        })
        .catch(error => {
            console.error('Error deleting recording:', error);
            alert('Error deleting recording: ' + error.message);

            if (recordingsTable) {
                hideLoading(recordingsTable);
            }
        });
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
function editStream(streamId) {
    console.log('Editing stream:', streamId);
    const streamModal = document.getElementById('stream-modal');
    if (!streamModal) return;

    showLoading(streamModal);

    // Fetch stream details from API
    fetch(`/api/streams/${encodeURIComponent(streamId)}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load stream details');
            }
            return response.json();
        })
        .then(stream => {
            console.log('Loaded stream details:', stream);
            // Fill the form with stream data
            document.getElementById('stream-name').value = stream.name;
            document.getElementById('stream-url').value = stream.url;
            document.getElementById('stream-enabled').checked = stream.enabled !== false; // Default to true if not specified
            document.getElementById('stream-width').value = stream.width || 1280;
            document.getElementById('stream-height').value = stream.height || 720;
            document.getElementById('stream-fps').value = stream.fps || 15;
            document.getElementById('stream-codec').value = stream.codec || 'h264';
            document.getElementById('stream-priority').value = stream.priority || 5;
            document.getElementById('stream-record').checked = stream.record !== false; // Default to true if not specified
            document.getElementById('stream-segment').value = stream.segment_duration || 900;

            // Store original stream ID for later comparison
            streamModal.dataset.streamId = streamId;

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
    const streamId = streamModal.dataset.streamId;
    let method = 'POST';
    let url = '/api/streams';

    if (streamId) {
        console.log('Updating existing stream:', streamId);
        method = 'PUT';
        url = `/api/streams/${encodeURIComponent(streamId)}`;
    } else {
        console.log('Creating new stream');
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
            console.log('Stream saved successfully:', data);
            // Close the modal
            streamModal.style.display = 'none';

            // Clear stream ID
            delete streamModal.dataset.streamId;

            // Reload streams
            if (document.getElementById('streams-table')) {
                loadStreams(); // For streams page
            } else if (document.getElementById('video-grid')) {
                loadStreams(true); // For live view page
            }

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
function deleteStream(streamId) {
    console.log('Deleting stream:', streamId);
    if (!confirm('Are you sure you want to delete this stream?')) {
        return;
    }

    const streamsTable = document.getElementById('streams-table');
    if (streamsTable) {
        showLoading(streamsTable);
    }

    // Send delete request to API
    fetch(`/api/streams/${encodeURIComponent(streamId)}`, {
        method: 'DELETE'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to delete stream');
            }
            return response.json();
        })
        .then(data => {
            console.log('Stream deleted successfully:', data);
            // Reload streams
            if (document.getElementById('streams-table')) {
                loadStreams(); // For streams page
            } else if (document.getElementById('video-grid')) {
                loadStreams(true); // For live view page
            }

            alert('Stream deleted successfully');
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

    // Send restart request to API
    fetch('/api/system/restart', {
        method: 'POST'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to restart service');
            }
            return response.json();
        })
        .then(data => {
            hideLoading(systemContainer);

            // Update status
            document.getElementById('status-indicator').className = 'status-warning';
            document.getElementById('status-text').textContent = 'System restarting...';

            // Show message
            alert('Service restart initiated. The system will be unavailable for a few moments.');

            // After a delay, check system status
            setTimeout(checkSystemStatus, 5000);
        })
        .catch(error => {
            console.error('Error restarting service:', error);
            alert('Error restarting service: ' + error.message);

            if (systemContainer) {
                hideLoading(systemContainer);
            }
        });
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

    // Send shutdown request to API
    fetch('/api/system/shutdown', {
        method: 'POST'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to shutdown service');
            }
            return response.json();
        })
        .then(data => {
            hideLoading(systemContainer);

            // Update status
            document.getElementById('status-indicator').className = 'status-danger';
            document.getElementById('status-text').textContent = 'System shutting down...';

            // Show message
            alert('Service shutdown initiated. The system will become unavailable shortly.');
        })
        .catch(error => {
            console.error('Error shutting down service:', error);
            alert('Error shutting down service: ' + error.message);

            if (systemContainer) {
                hideLoading(systemContainer);
            }
        });
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

    showLoading(logsContainer.parentElement);

    // Send clear logs request to API
    fetch('/api/system/logs/clear', {
        method: 'POST'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to clear logs');
            }
            return response.json();
        })
        .then(data => {
            // Clear logs display
            logsContainer.textContent = 'Logs cleared';

            // Show message
            alert('Logs cleared successfully');
        })
        .catch(error => {
            console.error('Error clearing logs:', error);
            alert('Error clearing logs: ' + error.message);
        })
        .finally(() => {
            hideLoading(logsContainer.parentElement);
        });
}

/**
 * Backup configuration
 */
function backupConfig() {
    const systemContainer = document.querySelector('.system-container');
    if (systemContainer) {
        showLoading(systemContainer);
    }

    // Send backup request to API
    fetch('/api/system/backup', {
        method: 'POST'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to create backup');
            }
            return response.json();
        })
        .then(data => {
            if (data.backupUrl) {
                // Create a link and trigger download
                const link = document.createElement('a');
                link.href = data.backupUrl;
                link.download = data.filename || 'lightnvr-backup.json';
                document.body.appendChild(link);
                link.click();
                document.body.removeChild(link);

                // Show message
                alert('Configuration backup created and downloaded successfully');
            } else {
                alert('Configuration backup created successfully');
            }
        })
        .catch(error => {
            console.error('Error creating backup:', error);
            alert('Error creating backup: ' + error.message);
        })
        .finally(() => {
            if (systemContainer) {
                hideLoading(systemContainer);
            }
        });
}

/**
 * Check system status after restart
 */
function checkSystemStatus() {
    fetch('/api/system/status')
        .then(response => {
            if (!response.ok) {
                // If server not responding yet, try again after delay
                setTimeout(checkSystemStatus, 2000);
                return;
            }
            return response.json();
        })
        .then(data => {
            if (data && data.status === 'ok') {
                // Service is back online
                document.getElementById('status-indicator').className = 'status-ok';
                document.getElementById('status-text').textContent = 'System running normally';

                // Reload system information
                loadSystemInfo();
            } else {
                // Not ready yet, try again
                setTimeout(checkSystemStatus, 2000);
            }
        })
        .catch(error => {
            // Error or server not responding, try again
            console.log('Waiting for system to restart...');
            setTimeout(checkSystemStatus, 2000);
        });
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
            background-color: rgba(0, 0, 0, 0.5);
            display: flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
        }
        
        .play-button {
            width: 0;
            height: 0;
            border-top: 20px solid transparent;
            border-bottom: 20px solid transparent;
            border-left: 30px solid white;
            margin-left: 10px;
        }
    `;

    document.head.appendChild(style);
}
