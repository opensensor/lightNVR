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
