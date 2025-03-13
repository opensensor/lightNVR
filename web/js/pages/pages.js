/**
 * LightNVR Web Interface Page Handlers
 * Contains page-specific event handlers and setup functions
 */

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
    // Initialize from URL parameters if present
    initializeFromUrl();
    
    // Setup sortable column headers
    document.querySelectorAll('th.sortable').forEach(th => {
        th.addEventListener('click', function() {
            // Get current sort state
            const isAsc = this.classList.contains('sort-asc');
            const isDesc = this.classList.contains('sort-desc');
            
            // Remove sort classes from all headers
            document.querySelectorAll('th.sortable').forEach(header => {
                header.classList.remove('sort-asc', 'sort-desc');
                header.querySelector('.sort-icon').textContent = '';
            });
            
            // Toggle sort direction or set to desc by default
            if (isAsc) {
                this.classList.add('sort-desc');
                this.querySelector('.sort-icon').textContent = '▼';
            } else {
                this.classList.add('sort-asc');
                this.querySelector('.sort-icon').textContent = '▲';
            }
            
            // Reload with new sort
            loadRecordings(1);
        });
    });
    
    // Date range selector
    const dateRangeSelect = document.getElementById('date-range-select');
    if (dateRangeSelect) {
        // Set default date range
        if (!window.location.search) {
            dateRangeSelect.value = 'today';
        }
        
        // Toggle custom date range visibility
        toggleCustomDateRange();
        
        dateRangeSelect.addEventListener('change', function() {
            // If changing to custom, prefill with current filter values
            if (this.value === 'custom') {
                // Get current URL parameters
                const urlParams = new URLSearchParams(window.location.search);
                
                // If we have start/end params, use those
                if (urlParams.has('start') || urlParams.has('end')) {
                    // Already have URL params, they'll be used by initializeFromUrl
                    toggleCustomDateRange();
                    return;
                }
                
                // Otherwise, calculate based on current selection
                const now = new Date();
                let startDateTime, endDateTime;
                
                // Get previous selection
                const prevSelection = dateRangeSelect.getAttribute('data-prev-value') || 'today';
                
                // Calculate appropriate dates based on previous selection
                switch (prevSelection) {
                    case 'today':
                        startDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0);
                        endDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
                        break;
                    case 'yesterday':
                        const yesterday = new Date(now);
                        yesterday.setDate(yesterday.getDate() - 1);
                        startDateTime = new Date(yesterday.getFullYear(), yesterday.getMonth(), yesterday.getDate(), 0, 0, 0);
                        endDateTime = new Date(yesterday.getFullYear(), yesterday.getMonth(), yesterday.getDate(), 23, 59, 59);
                        break;
                    case 'last7days':
                        const sevenDaysAgo = new Date(now);
                        sevenDaysAgo.setDate(sevenDaysAgo.getDate() - 6);
                        startDateTime = new Date(sevenDaysAgo.getFullYear(), sevenDaysAgo.getMonth(), sevenDaysAgo.getDate(), 0, 0, 0);
                        endDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
                        break;
                    case 'last30days':
                        const thirtyDaysAgo = new Date(now);
                        thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 29);
                        startDateTime = new Date(thirtyDaysAgo.getFullYear(), thirtyDaysAgo.getMonth(), thirtyDaysAgo.getDate(), 0, 0, 0);
                        endDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
                        break;
                    default:
                        startDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0);
                        endDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
                }
                
                // Set the date inputs
                const startDate = document.getElementById('start-date');
                if (startDate) {
                    const year = startDateTime.getFullYear();
                    const month = String(startDateTime.getMonth() + 1).padStart(2, '0');
                    const day = String(startDateTime.getDate()).padStart(2, '0');
                    startDate.value = `${year}-${month}-${day}`;
                }
                
                const startTime = document.getElementById('start-time');
                if (startTime) {
                    const hours = String(startDateTime.getHours()).padStart(2, '0');
                    const minutes = String(startDateTime.getMinutes()).padStart(2, '0');
                    startTime.value = `${hours}:${minutes}`;
                }
                
                const endDate = document.getElementById('end-date');
                if (endDate) {
                    const year = endDateTime.getFullYear();
                    const month = String(endDateTime.getMonth() + 1).padStart(2, '0');
                    const day = String(endDateTime.getDate()).padStart(2, '0');
                    endDate.value = `${year}-${month}-${day}`;
                }
                
                const endTime = document.getElementById('end-time');
                if (endTime) {
                    const hours = String(endDateTime.getHours()).padStart(2, '0');
                    const minutes = String(endDateTime.getMinutes()).padStart(2, '0');
                    endTime.value = `${hours}:${minutes}`;
                }
            }
            
            // Store current value for next time
            dateRangeSelect.setAttribute('data-prev-value', this.value);
            
            // Toggle custom date range visibility
            toggleCustomDateRange();
        });
        
        // Initialize the previous value attribute
        dateRangeSelect.setAttribute('data-prev-value', dateRangeSelect.value);
    }
    
    // Set default dates for custom range
    const today = new Date();
    const startDate = document.getElementById('start-date');
    const endDate = document.getElementById('end-date');
    
    if (startDate && !startDate.value) {
        const year = today.getFullYear();
        const month = String(today.getMonth() + 1).padStart(2, '0');
        const day = String(today.getDate()).padStart(2, '0');
        startDate.value = `${year}-${month}-${day}`;
    }
    
    if (endDate && !endDate.value) {
        const year = today.getFullYear();
        const month = String(today.getMonth() + 1).padStart(2, '0');
        const day = String(today.getDate()).padStart(2, '0');
        endDate.value = `${year}-${month}-${day}`;
    }
    
    // Apply filters button
    const applyFiltersBtn = document.getElementById('apply-filters-btn');
    if (applyFiltersBtn) {
        applyFiltersBtn.addEventListener('click', function() {
            loadRecordings(1); // Reset to page 1 when applying filters
        });
    }
    
    // Reset filters button
    const resetFiltersBtn = document.getElementById('reset-filters-btn');
    if (resetFiltersBtn) {
        resetFiltersBtn.addEventListener('click', function() {
            resetFilters();
        });
    }
    
    // Toggle filters sidebar button (mobile)
    const toggleFiltersBtn = document.getElementById('toggle-filters-btn');
    if (toggleFiltersBtn) {
        toggleFiltersBtn.addEventListener('click', function() {
            toggleFiltersSidebar();
        });
        
        // Initialize sidebar state for mobile
        const sidebar = document.getElementById('filters-sidebar');
        if (sidebar && window.innerWidth <= 992) {
            sidebar.classList.add('collapsed');
        }
    }
    
    // Handle window resize for responsive sidebar
    window.addEventListener('resize', function() {
        const sidebar = document.getElementById('filters-sidebar');
        if (sidebar) {
            if (window.innerWidth <= 992) {
                sidebar.classList.add('collapsed');
            } else {
                sidebar.classList.remove('collapsed');
            }
        }
    });
    
    // Setup video modal close buttons
    const videoModal = document.getElementById('video-modal');
    const closeBtn = videoModal ? videoModal.querySelector('.close') : null;
    const closeVideoBtn = document.getElementById('video-close-btn');
    
    if (closeBtn) {
        closeBtn.addEventListener('click', function() {
            videoModal.style.display = 'none';
            const videoPlayer = document.getElementById('video-player');
            if (videoPlayer) {
                videoPlayer.pause();
                videoPlayer.src = '';
            }
        });
    }
    
    if (closeVideoBtn) {
        closeVideoBtn.addEventListener('click', function() {
            videoModal.style.display = 'none';
            const videoPlayer = document.getElementById('video-player');
            if (videoPlayer) {
                videoPlayer.pause();
                videoPlayer.src = '';
            }
        });
    }
    
    // Close modal when clicking outside
    window.addEventListener('click', function(event) {
        if (event.target === videoModal) {
            videoModal.style.display = 'none';
            const videoPlayer = document.getElementById('video-player');
            if (videoPlayer) {
                videoPlayer.pause();
                videoPlayer.src = '';
            }
        }
    });
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
